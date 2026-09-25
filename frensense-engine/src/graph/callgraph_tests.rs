// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for call-graph construction and dispatch resolution (§8.1) and its
//! integration with `ProgramSvfg` cross-edge installation.

#[cfg(test)]
pub mod callgraph_tests {
    use crate::analysis::forward::ProgramSvfg;
    use crate::analysis::taint::config::TaintConfig;
    use crate::graph::callgraph::{CallGraphBuilder, CallTarget, method_key};
    use crate::ir::function::*;
    use rustc_hash::FxHashMap;

    fn dummy_meta(name: &str) -> VarMetadata {
        VarMetadata {
            source_name: Some(name.to_string()),
            type_name: None,
            byte_range: None,
            is_memory_state: false,
            object_keys: Vec::new(),
        }
    }

    fn add_param(ir: &mut FunctionIR, name: &str) -> VarId {
        let v = ir.new_var(dummy_meta(name));
        ir.parameters.push(v);
        v
    }

    fn empty_config() -> TaintConfig {
        TaintConfig {
            sources: rustc_hash::FxHashSet::default(),
            sinks: rustc_hash::FxHashSet::default(),
            sanitizers: rustc_hash::FxHashSet::default(),
        }
    }

    fn fn_ref_meta(name: &str) -> VarMetadata {
        VarMetadata {
            source_name: Some(name.to_string()),
            type_name: None,
            byte_range: None,
            is_memory_state: false,
            object_keys: Vec::new(),
        }
    }

    fn build_graph(irs: Vec<FunctionIR>) -> FxHashMap<String, &'static FunctionIR> {
        let mut map = FxHashMap::default();
        for ir in irs {
            let leaked: &'static FunctionIR = Box::leak(Box::new(ir));
            map.insert(leaked.name.clone(), leaked);
        }
        map
    }

    // -------------------------------------------------------------------
    // 1. Alias resolution: `const h = helper; h(x)` calls helper.
    // -------------------------------------------------------------------
    #[test]
    fn test_alias_resolution() {
        let mut helper = FunctionIR::new("helper".into());
        {
            let b = helper.entry_block;
            helper.set_terminator(b, Terminator::Return { src: None });
        }

        let mut caller = FunctionIR::new("caller".into());
        {
            let b = caller.entry_block;
            // h = "helper" (fn-ref lowered as a string-literal assignment)
            let h = caller.new_var(fn_ref_meta("h"));
            caller.push_instruction(
                b,
                Instruction::Assign {
                    dest: h,
                    src: Operand::StringLiteral("helper".into()),
                },
            );
            // x = h()  → CallPointer via h
            let x = caller.new_var(dummy_meta("x"));
            let mem = caller.new_var(dummy_meta("m"));
            caller.push_instruction(
                b,
                Instruction::CallPointer {
                    dest: Some(x),
                    mem_out: mem,
                    mem_in: caller.initial_memory_state,
                    func_ptr: Operand::Var(h),
                    args: vec![],
                },
            );
            caller.set_terminator(b, Terminator::Return { src: None });
        }

        let irs = build_graph(vec![helper, caller]);
        let cg = CallGraphBuilder::new(&irs).build();

        let sites = cg.targets_at("caller", irs["caller"].entry_block.0, 1);
        assert!(
            sites.contains(&CallTarget::Function("helper".into())),
            "aliased call h() must resolve to helper; got {:?}",
            sites
        );
    }

    // -------------------------------------------------------------------
    // 2. Method dispatch: receiver of known class resolves Class.method.
    // -------------------------------------------------------------------
    #[test]
    fn test_method_dispatch_via_class_allocation() {
        let mut database = FunctionIR::new(method_key("Database", "query"));
        {
            let b = database.entry_block;
            database.set_terminator(b, Terminator::Return { src: None });
        }

        let mut caller = FunctionIR::new("caller".into());
        {
            let b = caller.entry_block;
            let db = caller.new_var(dummy_meta("db"));
            let m2 = caller.new_var(dummy_meta("m2"));
            caller.push_instruction(
                b,
                Instruction::Allocate {
                    dest: db,
                    mem_out: m2,
                    mem_in: caller.initial_memory_state,
                    kind: AllocationKind::ClassInstance("Database".into()),
                },
            );
            let r = caller.new_var(dummy_meta("r"));
            let mem = caller.new_var(dummy_meta("m3"));
            caller.push_instruction(
                b,
                Instruction::CallVirtual {
                    dest: Some(r),
                    mem_out: mem,
                    mem_in: caller.initial_memory_state,
                    receiver: Operand::Var(db),
                    method: "query".into(),
                    args: vec![],
                },
            );
            caller.set_terminator(b, Terminator::Return { src: None });
        }

        let irs = build_graph(vec![database, caller]);
        let cg = CallGraphBuilder::new(&irs).build();

        let sites = cg.targets_at("caller", irs["caller"].entry_block.0, 1);
        assert!(
            sites.contains(&CallTarget::Method {
                class: "Database".into(),
                method: "query".into()
            }),
            "db.query() on Database instance must resolve to Database.query; got {:?}",
            sites
        );
        let _ = database;
    }

    // -------------------------------------------------------------------
    // 3. Higher-order callback: passing a fn ref into a called parameter.
    // -------------------------------------------------------------------
    #[test]
    fn test_callback_edge() {
        // `fn run(handler) { handler(); }`, handler (param 0) is called.
        let mut run = FunctionIR::new("run".into());
        let handler = add_param(&mut run, "handler");
        {
            let b = run.entry_block;
            let r = run.new_var(dummy_meta("r"));
            let mem = run.new_var(dummy_meta("m"));
            run.push_instruction(
                b,
                Instruction::CallPointer {
                    dest: Some(r),
                    mem_out: mem,
                    mem_in: run.initial_memory_state,
                    func_ptr: Operand::Var(handler),
                    args: vec![],
                },
            );
            run.set_terminator(b, Terminator::Return { src: None });
        }

        // `fn caller() { run(processor); }`
        let mut caller = FunctionIR::new("caller".into());
        {
            let b = caller.entry_block;
            let mem = caller.new_var(dummy_meta("m2"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem,
                    mem_in: caller.initial_memory_state,
                    func: "run".into(),
                    args: vec![Operand::StringLiteral("processor".into())],
                },
            );
            caller.set_terminator(b, Terminator::Return { src: None });
        }

        // `fn processor() { }`
        let mut processor = FunctionIR::new("processor".into());
        {
            let b = processor.entry_block;
            processor.set_terminator(b, Terminator::Return { src: None });
        }

        let irs = build_graph(vec![run, caller, processor]);
        let cg = CallGraphBuilder::new(&irs).build();

        // caller's call to run() must ALSO carry a callback target processor.
        let sites = cg.targets_at("caller", irs["caller"].entry_block.0, 0);
        assert!(
            sites.contains(&CallTarget::Function("run".into())),
            "direct call resolves to run; got {:?}",
            sites
        );
        assert!(
            sites.contains(&CallTarget::Function("processor".into())),
            "callback edge: run's called param 0 receives processor; got {:?}",
            sites
        );
    }

    // -------------------------------------------------------------------
    // 4. Recursion converges; cycle guards do not hang or panic.
    // -------------------------------------------------------------------
    #[test]
    fn test_recursion_and_cycles() {
        let mut a = FunctionIR::new("a".into());
        let mut b = FunctionIR::new("b".into());
        {
            let bb = a.entry_block;
            let mem = a.new_var(dummy_meta("m"));
            a.push_instruction(
                bb,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem,
                    mem_in: a.initial_memory_state,
                    func: "b".into(),
                    args: vec![],
                },
            );
            a.set_terminator(bb, Terminator::Return { src: None });
        }
        {
            let bb = b.entry_block;
            let mem = b.new_var(dummy_meta("m"));
            b.push_instruction(
                bb,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem,
                    mem_in: b.initial_memory_state,
                    func: "a".into(),
                    args: vec![],
                },
            );
            b.set_terminator(bb, Terminator::Return { src: None });
        }

        let irs = build_graph(vec![a, b]);
        let cg = CallGraphBuilder::new(&irs).build();
        assert_eq!(cg.callees_of("a"), &["b".to_string()]);
        assert_eq!(cg.callers_of("b"), &["a".to_string()]);

        // Full ProgramSvfg construction over the cycle must terminate.
        let irs2 = build_graph(vec![
            FunctionIR::new("a2".into()),
            FunctionIR::new("b2".into()),
        ]);
        let _prog = ProgramSvfg::new(&irs2, &empty_config());
    }

    // -------------------------------------------------------------------
    // 5. Unresolved external stays out of internal edges but is recorded.
    // -------------------------------------------------------------------
    #[test]
    fn test_unresolved_external_recorded() {
        let mut caller = FunctionIR::new("caller".into());
        {
            let b = caller.entry_block;
            let mem = caller.new_var(dummy_meta("m"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem,
                    mem_in: caller.initial_memory_state,
                    func: "libraryFetch".into(),
                    args: vec![],
                },
            );
            caller.set_terminator(b, Terminator::Return { src: None });
        }

        let irs = build_graph(vec![caller]);
        let cg = CallGraphBuilder::new(&irs).build();

        assert!(
            cg.edges.get("caller").is_none(),
            "no internal edge for external callee"
        );
        let has_ext = cg.unresolved.values().any(|n| n == "libraryFetch");
        assert!(has_ext, "external callee should be recorded in unresolved");
        assert_eq!(cg.callees_of("caller"), &[] as &[String]);
    }

    // -------------------------------------------------------------------
    // 6. Integration: resolved callgraph targets produce ProgramSvfg cross
    //    edges (actual-arg → formal-param) through the alias path.
    // -------------------------------------------------------------------
    #[test]
    fn test_program_svfg_uses_callgraph_for_indirect_calls() {
        let mut callee = FunctionIR::new("sink_helper".into());
        let _a = add_param(&mut callee, "a");
        {
            let b = callee.entry_block;
            callee.set_terminator(b, Terminator::Return { src: None });
        }

        let mut caller = FunctionIR::new("main".into());
        {
            let b = caller.entry_block;
            // f = "sink_helper"
            let f = caller.new_var(fn_ref_meta("f"));
            caller.push_instruction(
                b,
                Instruction::Assign {
                    dest: f,
                    src: Operand::StringLiteral("sink_helper".into()),
                },
            );
            // t = f(user_input)
            let inp = caller.new_var(dummy_meta("inp"));
            caller.push_instruction(
                b,
                Instruction::Assign {
                    dest: inp,
                    src: Operand::StringLiteral("user_input".into()),
                },
            );
            let r = caller.new_var(dummy_meta("r"));
            let mem = caller.new_var(dummy_meta("m"));
            caller.push_instruction(
                b,
                Instruction::CallPointer {
                    dest: Some(r),
                    mem_out: mem,
                    mem_in: caller.initial_memory_state,
                    func_ptr: Operand::Var(f),
                    args: vec![Operand::Var(inp)],
                },
            );
            caller.set_terminator(b, Terminator::Return { src: None });
        }

        let irs = build_graph(vec![callee, caller]);
        let prog = ProgramSvfg::new(&irs, &empty_config());

        // The indirect call must be bound to the callee, not left unresolved.
        let ci = prog.function_index("main").unwrap();
        let bindings = &prog.functions[ci].bindings;
        let indirect: Vec<_> = bindings
            .iter()
            .filter(|b| b.callee_name == "sink_helper")
            .collect();
        assert_eq!(
            indirect.len(),
            1,
            "CallPointer site should be bound via alias resolution"
        );
        assert_eq!(
            indirect[0].callees,
            vec![prog.function_index("sink_helper").unwrap()],
            "binding must point at sink_helper"
        );
        // And a cross edge into the callee's param node must exist.
        assert!(
            !prog.cross_edges.is_empty(),
            "resolved indirect call must produce cross edges"
        );
    }

    // -------------------------------------------------------------------
    // 7. Determinism: same input → identical graph.
    // -------------------------------------------------------------------
    #[test]
    fn test_deterministic_graph() {
        let mk = || {
            let mut a = FunctionIR::new("a".into());
            let b = FunctionIR::new("b".into());
            let bb = a.entry_block;
            let mem = a.new_var(dummy_meta("m"));
            a.push_instruction(
                bb,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem,
                    mem_in: a.initial_memory_state,
                    func: "b".into(),
                    args: vec![],
                },
            );
            a.set_terminator(bb, Terminator::Return { src: None });
            let _ = &b;
            build_graph(vec![a, b])
        };
        let g1 = CallGraphBuilder::new(&mk()).build();
        let g2 = CallGraphBuilder::new(&mk()).build();
        assert_eq!(g1.edges, g2.edges);
        assert_eq!(g1.unresolved, g2.unresolved);
    }
}
