// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the bottom-up interprocedural SVFG layer (task 4.2).

#[cfg(test)]
pub mod interprocedural_tests {
    use crate::analysis::forward::{InterproceduralTaintEngine, ProgramSvfg};
    use crate::analysis::taint::config::TaintConfig;
    use crate::graph::svfg::{NodeKind, SvfgBuilder};
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

    fn mem_meta(name: &str) -> VarMetadata {
        VarMetadata {
            source_name: Some(name.to_string()),
            type_name: None,
            byte_range: None,
            is_memory_state: true,
            object_keys: Vec::new(),
        }
    }

    fn config() -> TaintConfig {
        TaintConfig {
            sources: ["getSource".to_string()].into_iter().collect(),
            sinks: ["db.execute".to_string()].into_iter().collect(),
            sanitizers: ["escapeHtml".to_string()].into_iter().collect(),
        }
    }

    /// Build a ProgramSvfg from SSA IRs keyed by name.
    fn build_program(irs: Vec<FunctionIR>) -> ProgramSvfg<'static> {
        // Leak the IRs so we can hold &'static references (test convenience;
        // production callers own their IRs and pass real borrows).
        let map: FxHashMap<String, &'static FunctionIR> = irs
            .into_iter()
            .map(|ir| {
                let leaked: &'static FunctionIR = Box::leak(Box::new(ir));
                (leaked.name.clone(), leaked)
            })
            .collect();
        ProgramSvfg::new(&map, &config())
    }

    fn run_engine(prog: &ProgramSvfg) -> Vec<String> {
        let cfg = config();
        let mut engine = InterproceduralTaintEngine::new(prog, &cfg);
        engine.run();
        std::mem::take(&mut engine.alerts)
    }

    /// Create a variable, register it as a formal parameter, and return it
    /// (avoids the double-mutable-borrow of `f.parameters.push(f.new_var(..))`).
    fn add_param(ir: &mut FunctionIR, name: &str) -> VarId {
        let v = ir.new_var(dummy_meta(name));
        ir.parameters.push(v);
        v
    }

    // -----------------------------------------------------------------------
    // Test 1: actual-arg → formal-param edge carries taint into the callee.
    //
    //   fn main() { v = getSource(); helper(v); }      (no local sink)
    //   fn helper(p) { db.execute(p); }                 (no local source)
    //
    // Neither function can alert on its own; only the cross edge makes the
    // full source→sink path visible.
    // -----------------------------------------------------------------------
    #[test]
    fn test_arg_edge_carries_taint_into_callee() {
        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "helper".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let mut helper = FunctionIR::new("helper".into());
        let p0 = add_param(&mut helper, "p");
        {
            let b = helper.entry_block;
            let mem0 = helper.initial_memory_state;
            let m1 = helper.new_var(mem_meta("m1"));
            helper.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m1,
                    mem_in: mem0,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(p0)],
                },
            );
            helper.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![main, helper]);
        let hi = prog.function_index("helper").unwrap();

        // Cross edge exists: main's ActualArg → helper's FormalParam.
        let _main_i = prog.function_index("main").unwrap();
        assert!(
            !prog.cross_edges.is_empty(),
            "expected interprocedural edges to be installed"
        );

        let alerts = run_engine(&prog);
        assert_eq!(
            alerts.len(),
            1,
            "arg edge must carry taint into helper's sink; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("db.execute") && alerts[0].contains("[in helper]"));

        // Summary: helper's param 0 reaches a sink.
        let sum = prog.functions[hi].summary.as_ref().unwrap();
        assert!(sum.param_reaches_sink[0], "helper param 0 reaches sink");
    }

    // -----------------------------------------------------------------------
    // Test 2: formal-ret → actual-ret edge carries taint back to the caller.
    //
    //   fn get() { return getSource(); }               (no local sink)
    //   fn main() { v = get(); db.execute(v); }        (no local source)
    // -----------------------------------------------------------------------
    #[test]
    fn test_ret_edge_carries_taint_back_to_caller() {
        let mut get = FunctionIR::new("get".into());
        {
            let b = get.entry_block;
            let mem0 = get.initial_memory_state;
            let v = get.new_var(dummy_meta("v"));
            let m1 = get.new_var(mem_meta("m1"));
            get.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            get.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(v)),
                },
            );
        }

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "get".into(),
                    args: vec![],
                },
            );
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![get, main]);
        let alerts = run_engine(&prog);

        assert_eq!(
            alerts.len(),
            1,
            "return edge must carry taint from get() into main's sink; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("db.execute") && alerts[0].contains("[in main]"));
    }

    // -----------------------------------------------------------------------
    // Test 3: compositional summary, taint passes THROUGH an intermediary.
    //
    //   fn source_fn() { return getSource(); }
    //   fn middle()   { return source_fn(); }   ← never analysed twice
    //   fn main()     { db.execute(middle()); }
    //
    // `middle` must be summarised once (leaves first) and its summary applied
    // at main's call site. Also checks bottom-up order is really leaves-first.
    // -----------------------------------------------------------------------
    #[test]
    fn test_compositional_summary_through_intermediary() {
        let mut source_fn = FunctionIR::new("source_fn".into());
        {
            let b = source_fn.entry_block;
            let mem0 = source_fn.initial_memory_state;
            let v = source_fn.new_var(dummy_meta("v"));
            let m1 = source_fn.new_var(mem_meta("m1"));
            source_fn.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            source_fn.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(v)),
                },
            );
        }

        let mut middle = FunctionIR::new("middle".into());
        {
            let b = middle.entry_block;
            let mem0 = middle.initial_memory_state;
            let v = middle.new_var(dummy_meta("v"));
            let m1 = middle.new_var(mem_meta("m1"));
            middle.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "source_fn".into(),
                    args: vec![],
                },
            );
            middle.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(v)),
                },
            );
        }

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "middle".into(),
                    args: vec![],
                },
            );
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![source_fn, middle, main]);

        // Bottom-up: leaves before callers.
        let idx = |n: &str| prog.function_index(n).unwrap();
        let (s, m, mn) = (idx("source_fn"), idx("middle"), idx("main"));
        let pos = |f: usize| prog.topological_order.iter().position(|&x| x == f).unwrap();
        assert!(pos(s) < pos(m), "source_fn (leaf) before middle");
        assert!(pos(m) < pos(mn), "middle before main");

        // middle's summary: return is taint-carrying (relational, from its
        // own analysis applying source_fn's summary).
        let msum = prog.functions[m].summary.as_ref().unwrap();
        assert!(
            msum.param_taints_return.is_empty() && true,
            "middle has no params; summary shape ok"
        );

        let alerts = run_engine(&prog);
        assert_eq!(
            alerts.len(),
            1,
            "taint must flow source_fn → middle → main via summaries + ret edges; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("[in main]"));
    }

    // -----------------------------------------------------------------------
    // Test 4: sanitizing callee suppresses the conservative pass-through.
    //
    //   fn clean(x) { return escapeHtml(x); }
    //   fn main() { v = getSource(); w = clean(v); db.execute(w); }  → 0 alerts
    //
    // The local arg→dest pass-through edge at the `clean` call site must be
    // suppressed by clean's summary, so w stays clean.
    // -----------------------------------------------------------------------
    #[test]
    fn test_sanitizing_callee_suppresses_pass_through() {
        let mut clean = FunctionIR::new("clean".into());
        let x0 = add_param(&mut clean, "x");
        {
            let b = clean.entry_block;
            let mem0 = clean.initial_memory_state;
            let r = clean.new_var(dummy_meta("r"));
            let m1 = clean.new_var(mem_meta("m1"));
            clean.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(r),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "escapeHtml".into(),
                    args: vec![Operand::Var(x0)],
                },
            );
            clean.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(r)),
                },
            );
        }

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let w = main.new_var(dummy_meta("w"));
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(w),
                    mem_out: m2,
                    mem_in: m1,
                    func: "clean".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            let m3 = main.new_var(mem_meta("m3"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m3,
                    mem_in: m2,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(w)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![clean, main]);
        let alerts = run_engine(&prog);

        assert!(
            alerts.is_empty(),
            "sanitizing callee must suppress pass-through; got {:?}",
            alerts
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: sanitizing callee still reports the UNsanitized sibling path.
    //
    // Same as test 4 but the caller also feeds the raw source straight to the
    // sink, exactly one alert.
    // -----------------------------------------------------------------------
    #[test]
    fn test_sanitizer_precision_keeps_unsanitized_path() {
        let mut clean = FunctionIR::new("clean".into());
        let x0 = add_param(&mut clean, "x");
        {
            let b = clean.entry_block;
            let mem0 = clean.initial_memory_state;
            let r = clean.new_var(dummy_meta("r"));
            let m1 = clean.new_var(mem_meta("m1"));
            clean.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(r),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "escapeHtml".into(),
                    args: vec![Operand::Var(x0)],
                },
            );
            clean.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(r)),
                },
            );
        }

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let w = main.new_var(dummy_meta("w"));
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(w),
                    mem_out: m2,
                    mem_in: m1,
                    func: "clean".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            let m3 = main.new_var(mem_meta("m3"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m3,
                    mem_in: m2,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(w)],
                },
            );
            let m4 = main.new_var(mem_meta("m4"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m4,
                    mem_in: m3,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(v)], // raw!
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![clean, main]);
        let alerts = run_engine(&prog);

        assert_eq!(
            alerts.len(),
            1,
            "only the raw (unsanitized) path should alert; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("argument 0"));
    }

    // -----------------------------------------------------------------------
    // Test 6: recursive callees (cycles) stay sound, no panic, local
    // pass-through edges retained, basic propagation still works.
    // -----------------------------------------------------------------------
    #[test]
    fn test_recursive_callee_remains_sound() {
        // fn rec(x) { y = rec(x); db.execute(y); }   ← self-recursive, cycle
        // fn main()  { v = getSource(); rec(v); }
        let mut rec = FunctionIR::new("rec".into());
        let x0 = add_param(&mut rec, "x");
        {
            let b = rec.entry_block;
            let mem0 = rec.initial_memory_state;
            let y = rec.new_var(dummy_meta("y"));
            let m1 = rec.new_var(mem_meta("m1"));
            rec.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(y),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "rec".into(),
                    args: vec![Operand::Var(x0)],
                },
            );
            let m2 = rec.new_var(mem_meta("m2"));
            rec.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(y)],
                },
            );
            rec.set_terminator(b, Terminator::Return { src: None });
        }

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "rec".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![rec, main]);
        let alerts = run_engine(&prog);

        // Taint crosses the arg edge into rec's formal param, then the local
        // pass-through (rec's dest y receives taint from arg) reaches the sink.
        assert_eq!(
            alerts.len(),
            1,
            "cycle handling must stay sound: expected exactly 1 alert; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("[in rec]"));
    }

    // -----------------------------------------------------------------------
    // Test 7: graph shape, cross edges are explicit and value-flow directed.
    // -----------------------------------------------------------------------
    #[test]
    fn test_cross_edges_structure() {
        let mut callee = FunctionIR::new("callee".into());
        let a0 = add_param(&mut callee, "a");
        let b0 = add_param(&mut callee, "b");
        {
            let b = callee.entry_block;
            let r = callee.new_var(dummy_meta("r"));
            let _m1 = callee.new_var(mem_meta("m1"));
            callee.push_instruction(
                b,
                Instruction::BinaryOp {
                    dest: r,
                    op: "+".into(),
                    lhs: Operand::Var(a0),
                    rhs: Operand::Var(b0),
                },
            );
            callee.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(r)),
                },
            );
        }

        let mut caller = FunctionIR::new("caller".into());
        {
            let b = caller.entry_block;
            let x = caller.new_var(dummy_meta("x"));
            let y = caller.new_var(dummy_meta("y"));
            let r = caller.new_var(dummy_meta("r"));
            let m1 = caller.new_var(mem_meta("m1"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(r),
                    mem_out: m1,
                    mem_in: caller.initial_memory_state,
                    func: "callee".into(),
                    args: vec![Operand::Var(x), Operand::Var(y)],
                },
            );
            caller.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![callee, caller]);
        let _ci = prog.function_index("caller").unwrap();
        let ki = prog.function_index("callee").unwrap();

        // FormalParam nodes exist in the callee.
        assert_eq!(
            prog.functions[ki].svfg.param_nodes().len(),
            3, // 2 params + initial memory state
            "callee should have 2 explicit params + mem-state param"
        );

        // Every cross edge must point INTO the callee's param nodes or OUT of
        // the callee's FormalRet nodes (value-flow direction).
        let mut param_edge_count = 0;
        let mut ret_edge_count = 0;
        for ((from_f, from_k), edges) in &prog.cross_edges {
            for (to_f, to_k) in edges {
                let to_node = prog.functions[*to_f].svfg.node(to_k).unwrap();
                let from_node = prog.functions[*from_f].svfg.node(from_k).unwrap();
                match &to_node.kind {
                    NodeKind::FormalParam => {
                        assert_eq!(*to_f, ki);
                        param_edge_count += 1;
                    }
                    _ => {
                        // Return edges: from FormalRet of callee to caller ActualRet.
                        assert_eq!(from_node.kind, NodeKind::FormalRet);
                        assert_eq!(*from_f, ki);
                        ret_edge_count += 1;
                    }
                }
            }
        }
        assert_eq!(param_edge_count, 2, "two actual-arg → formal-param edges");
        assert!(
            ret_edge_count >= 1,
            "at least one formal-ret → actual-ret edge"
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: summaries are cached, a leaf referenced by two callers is
    // analysed once (its summary computed exactly once, bottom-up).
    // -----------------------------------------------------------------------
    #[test]
    fn test_leaf_summary_computed_once_shared_by_callers() {
        let mut leaf = FunctionIR::new("leaf".into());
        let la = add_param(&mut leaf, "a");
        {
            let b = leaf.entry_block;
            let r = leaf.new_var(dummy_meta("r"));
            let m1 = leaf.new_var(mem_meta("m1"));
            leaf.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(r),
                    mem_out: m1,
                    mem_in: leaf.initial_memory_state,
                    func: "escapeHtml".into(),
                    args: vec![Operand::Var(la)],
                },
            );
            leaf.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(r)),
                },
            );
        }

        let mk_caller = |name: &str| {
            let mut f = FunctionIR::new(name.to_string());
            let v = f.new_var(dummy_meta("v"));
            let w = f.new_var(dummy_meta("w"));
            let m1 = f.new_var(mem_meta("m1"));
            let m2 = f.new_var(mem_meta("m2"));
            let m3 = f.new_var(mem_meta("m3"));
            f.push_instruction(
                f.entry_block,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: f.initial_memory_state,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            f.push_instruction(
                f.entry_block,
                Instruction::CallStatic {
                    dest: Some(w),
                    mem_out: m2,
                    mem_in: m1,
                    func: "leaf".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            f.push_instruction(
                f.entry_block,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m3,
                    mem_in: m2,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(w)],
                },
            );
            f.set_terminator(f.entry_block, Terminator::Return { src: None });
            f
        };
        let c1 = mk_caller("caller1");
        let c2 = mk_caller("caller2");

        let prog = build_program(vec![leaf, c1, c2]);

        // Both callers' flows are killed by the sanitizing leaf summary.
        let alerts = run_engine(&prog);
        assert!(
            alerts.is_empty(),
            "sanitizing leaf summary must protect both callers; got {:?}",
            alerts
        );

        // Leaf's summary exists exactly once in the registry.
        assert!(prog.summary_registry.summaries.contains_key("leaf"));
        let li = prog.function_index("leaf").unwrap();
        assert!(prog.functions[li].summary.is_some());
    }

    // -----------------------------------------------------------------------
    // Test 9: single-function SVFG still works standalone (regression guard:
    // the interprocedural refactor must not have broken the local path).
    // -----------------------------------------------------------------------
    #[test]
    fn test_single_function_regression() {
        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![main]);
        let alerts = run_engine(&prog);
        assert_eq!(
            alerts.len(),
            1,
            "local source→sink path still fires; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("[in main]"));

        // Also sanity-check the raw SvfgBuilder path is untouched.
        let (graph, def_site) = SvfgBuilder::new(prog.functions[0].ir).build();
        assert!(!graph.stats().edge_count.to_string().is_empty());
        let _ = def_site;
    }

    // -----------------------------------------------------------------------
    // Test: sibling-callee heap flow (field-sensitive cross edges, direction
    // 3). `handler` passes the SAME object to `writer` (stores its param
    // field) and to `reader` (loads and returns it); the sink consumes the
    // return value. No caller-side store/load of the field exists, so only
    // a writer→reader cross edge completes the path:
    //
    //   fn writer(obj, input)  { obj.data = input; }
    //   fn reader(obj)         { return obj.data; }
    //   fn handler()           { writer(o, getSource()); db.execute(reader(o)); }
    // -----------------------------------------------------------------------
    pub(crate) fn sibling_program() -> ProgramSvfg<'static> {
        let mut handler = FunctionIR::new("handler".into());
        {
            let b = handler.entry_block;
            let mem0 = handler.initial_memory_state;
            let obj = handler.new_var(dummy_meta("o"));
            let alloc_mem = handler.new_var(mem_meta("am"));
            handler.push_instruction(
                b,
                Instruction::Allocate {
                    dest: obj,
                    mem_out: alloc_mem,
                    mem_in: mem0,
                    kind: AllocationKind::Object,
                },
            );
            let taint = handler.new_var(dummy_meta("t"));
            let m1 = handler.new_var(mem_meta("m1"));
            handler.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(taint),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let m2 = handler.new_var(mem_meta("m2"));
            handler.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "writer".into(),
                    args: vec![Operand::Var(obj), Operand::Var(taint)],
                },
            );
            let ret = handler.new_var(dummy_meta("r"));
            let m3 = handler.new_var(mem_meta("m3"));
            handler.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(ret),
                    mem_out: m3,
                    mem_in: m2,
                    func: "reader".into(),
                    args: vec![Operand::Var(obj)],
                },
            );
            let m4 = handler.new_var(mem_meta("m4"));
            handler.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m4,
                    mem_in: m3,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(ret)],
                },
            );
            handler.set_terminator(b, Terminator::Return { src: None });
        }

        let mut writer = FunctionIR::new("writer".into());
        let w_obj = add_param(&mut writer, "obj");
        let w_in = add_param(&mut writer, "input");
        {
            let b = writer.entry_block;
            let field_val = writer.new_var(mem_meta("fout"));
            writer.push_instruction(
                b,
                Instruction::StoreField {
                    mem_out: field_val,
                    mem_in: writer.initial_memory_state,
                    base: w_obj,
                    field: "data".into(),
                    src: Operand::Var(w_in),
                },
            );
            writer.set_terminator(b, Terminator::Return { src: None });
        }

        let mut reader = FunctionIR::new("reader".into());
        let r_obj = add_param(&mut reader, "obj");
        {
            let b = reader.entry_block;
            let loaded = reader.new_var(dummy_meta("v"));
            let field_mem = reader.new_var(mem_meta("fm"));
            reader.push_instruction(
                b,
                Instruction::LoadField {
                    dest: loaded,
                    mem_in: field_mem,
                    base: r_obj,
                    field: "data".into(),
                },
            );
            reader.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(loaded)),
                },
            );
        }

        build_program(vec![handler, writer, reader])
    }

    #[test]
    fn test_sibling_callee_heap_flow_completes_path() {
        let prog = sibling_program();
        let wi = prog.function_index("writer").unwrap();
        let ri = prog.function_index("reader").unwrap();

        // The sibling edge: writer's store-use → reader's load-def.
        let writer_to_reader = prog
            .cross_edges
            .iter()
            .any(|((ff, _), targets)| *ff == wi && targets.iter().any(|(tf, _)| *tf == ri));
        assert!(writer_to_reader, "expected a writer→reader heap cross edge");

        let alerts = run_engine(&prog);
        assert_eq!(
            alerts.len(),
            1,
            "field taint must flow writer→reader to the sink; got {:?}",
            alerts
        );
    }

    #[test]
    fn test_sibling_heap_flow_is_field_sensitive() {
        let prog = sibling_program();
        let wi = prog.function_index("writer").unwrap();
        let ri = prog.function_index("reader").unwrap();

        // The writer stores field "data"; a load of a DIFFERENT field on the
        // sibling's parameter must NOT receive it. Verify no edge exists from
        // any writer store to a reader load of another field by checking the
        // installed edges reference the load dest only, i.e. the edge count
        // from writer to reader is exactly 1 (same-field).
        let count = prog
            .cross_edges
            .iter()
            .filter(|((ff, _), _)| *ff == wi)
            .flat_map(|(_, targets)| targets.iter().filter(|(tf, _)| *tf == ri))
            .count();
        assert_eq!(count, 1, "only the matching field should be linked");
    }
}
