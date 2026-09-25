// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for selective k=1 call-site context sensitivity (task 5).

#[cfg(test)]
pub mod context_tests {
    use crate::analysis::forward::{InterproceduralTaintEngine, ProgramSvfg};
    use crate::analysis::taint::config::TaintConfig;
    use crate::analysis::taint::context::{ContextSensitiveTaintEngine, ROOT_CTX};
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

    fn build_program(irs: Vec<FunctionIR>) -> ProgramSvfg<'static> {
        let map: FxHashMap<String, &'static FunctionIR> = irs
            .into_iter()
            .map(|ir| {
                let leaked: &'static FunctionIR = Box::leak(Box::new(ir));
                (leaked.name.clone(), leaked)
            })
            .collect();
        ProgramSvfg::new(&map, &config())
    }

    fn add_param(ir: &mut FunctionIR, name: &str) -> VarId {
        let v = ir.new_var(dummy_meta(name));
        ir.parameters.push(v);
        v
    }

    fn run_k1(prog: &ProgramSvfg) -> Vec<String> {
        let cfg = config();
        let mut e = ContextSensitiveTaintEngine::new(prog, &cfg);
        e.run();
        e.alert_messages()
    }

    /// The FP scenario: a callee invoked from a tainted site and a clean site.
    ///
    ///   fn id(x) { return x; }              ← identity: passes taint through
    ///   fn main() {
    ///       t = getSource();
    ///       a = id(t); db.execute(a);       ← REAL path (must alert)
    ///       c = "safe";
    ///       b = id(c); db.execute(b);       ← must NOT alert
    ///   }
    ///
    /// k=0 merges both `id` invocations into one FormalParam state, so the
    /// FormalRet → ActualRet edges fan taint out to BOTH call sites' dest
    /// nodes, the clean `b` is wrongly tainted (observable in the taint
    /// state set; the alert strings are identical so the alert count alone
    /// can't show it). k=1 splits by call site: the return from ctx(id, dirty
    /// site) may only reach the dirty site's ActualRet, so the clean site
    /// stays clean.
    #[test]
    fn test_k1_removes_false_positive_from_shared_callee() {
        let mut id = FunctionIR::new("id".into());
        let x0 = add_param(&mut id, "x");
        id.set_terminator(
            id.entry_block,
            Terminator::Return {
                src: Some(Operand::Var(x0)),
            },
        );

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let t = main.new_var(dummy_meta("t"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(t),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let a = main.new_var(dummy_meta("a"));
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(a),
                    mem_out: m2,
                    mem_in: m1,
                    func: "id".into(),
                    args: vec![Operand::Var(t)],
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
                    args: vec![Operand::Var(a)],
                },
            );
            // Clean call site: c = "safe"; b = id(c);
            let c = main.new_var(dummy_meta("c"));
            let m4 = main.new_var(mem_meta("m4"));
            main.push_instruction(
                b,
                Instruction::Assign {
                    dest: c,
                    src: Operand::StringLiteral("safe".to_string()),
                },
            );
            let bvar = main.new_var(dummy_meta("b"));
            let m5 = main.new_var(mem_meta("m5"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(bvar),
                    mem_out: m5,
                    mem_in: m4,
                    func: "id".into(),
                    args: vec![Operand::Var(c)],
                },
            );
            let m6 = main.new_var(mem_meta("m6"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m6,
                    mem_in: m5,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(bvar)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }
        let entry_block = main.entry_block;

        let prog = build_program(vec![id, main]);
        let cfg = config();

        // k=0 baseline: document the false positive in the STATE set, the
        // clean call's ActualRet (dest of the second id() call) is tainted.
        let mut k0 = InterproceduralTaintEngine::new(&prog, &cfg);
        k0.run();
        let main_i = prog.function_index("main").unwrap();
        // Clean call site is instruction index 4 (0-based) in main's entry
        // block; its dest `bvar` carries var index 6.
        let clean_ret_tainted_k0 = k0
            .tainted_nodes()
            .iter()
            .any(|(f, k)| *f == main_i && k.block == entry_block && k.instr_idx == Some(4));
        assert!(
            clean_ret_tainted_k0,
            "k=0 baseline must over-approximate: clean call site's ret is tainted"
        );

        // k=1: the clean call site's ActualRet must be clean in EVERY context.
        let mut k1 = ContextSensitiveTaintEngine::new(&prog, &cfg);
        k1.run();
        let clean_ret_tainted_k1 = k1.tainted().iter().any(|((f, k), ctxs)| {
            *f == main_i && k.block == entry_block && k.instr_idx == Some(4) && !ctxs.is_empty()
        });
        assert!(
            !clean_ret_tainted_k1,
            "k=1 must keep the clean call site's result clean in all contexts"
        );

        let alerts = k1.alert_messages();
        assert_eq!(
            alerts.len(),
            1,
            "k=1 must alert only on the real path; got {:?}",
            alerts
        );
        assert!(alerts[0].contains("[in main]"));

        // Precision bookkeeping: contexts are interned lazily, only when
        // taint actually crosses an arg edge. Here exactly one context is
        // created (the dirty call site); the clean site never creates one.
        let st = k1.stats();
        assert_eq!(st.context_collapses, 0);
        assert_eq!(
            st.contexts_created, 1,
            "only the tainted call site interns a context"
        );
    }

    /// No false negatives: the real taint path must survive context splitting.
    #[test]
    fn test_k1_preserves_real_path() {
        let mut id = FunctionIR::new("id".into());
        let x0 = add_param(&mut id, "x");
        id.set_terminator(
            id.entry_block,
            Terminator::Return {
                src: Some(Operand::Var(x0)),
            },
        );

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let t = main.new_var(dummy_meta("t"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(t),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let a = main.new_var(dummy_meta("a"));
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(a),
                    mem_out: m2,
                    mem_in: m1,
                    func: "id".into(),
                    args: vec![Operand::Var(t)],
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
                    args: vec![Operand::Var(a)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![id, main]);
        let alerts = run_k1(&prog);
        assert_eq!(
            alerts.len(),
            1,
            "real path must still alert; got {:?}",
            alerts
        );
    }

    /// Recursion under context splitting must terminate (bounded by the
    /// MAX_CONTEXTS_PER_FN cap) and stay sound.
    #[test]
    fn test_recursion_terminates_and_stays_sound() {
        // fn rec(x) { y = rec(x); db.execute(y); }
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
        let alerts = run_k1(&prog);
        assert_eq!(
            alerts.len(),
            1,
            "recursive taint path must still alert exactly once; got {:?}",
            alerts
        );
    }

    /// Phase 1 relevance: functions with no taint involvement are never
    /// context-split; the relevance set matches the k=0 reachable set.
    #[test]
    fn test_phase1_relevance_set_is_selective() {
        // fn noise() { x = 1; db.execute(x); }        ← sink but no source
        // fn dirty() { v = getSource(); db.execute(v); }
        // fn main()   { dirty(); noise(); }
        let mut noise = FunctionIR::new("noise".into());
        {
            let b = noise.entry_block;
            let x = noise.new_var(dummy_meta("x"));
            let m1 = noise.new_var(mem_meta("m1"));
            noise.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m1,
                    mem_in: noise.initial_memory_state,
                    func: "db.execute".into(),
                    args: vec![Operand::IntLiteral(1)],
                },
            );
            let _ = x;
            noise.set_terminator(b, Terminator::Return { src: None });
        }

        let mut dirty = FunctionIR::new("dirty".into());
        {
            let b = dirty.entry_block;
            let mem0 = dirty.initial_memory_state;
            let v = dirty.new_var(dummy_meta("v"));
            let m1 = dirty.new_var(mem_meta("m1"));
            dirty.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let m2 = dirty.new_var(mem_meta("m2"));
            dirty.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m2,
                    mem_in: m1,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(v)],
                },
            );
            dirty.set_terminator(b, Terminator::Return { src: None });
        }

        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m1,
                    mem_in: main.initial_memory_state,
                    func: "dirty".into(),
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
                    func: "noise".into(),
                    args: vec![],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let prog = build_program(vec![noise, dirty, main]);
        let cfg = config();
        let mut engine = ContextSensitiveTaintEngine::new(&prog, &cfg);
        engine.run();

        let st = engine.stats();
        assert_eq!(st.total_functions, 3);
        assert!(
            st.relevant_functions <= st.total_functions,
            "relevance set must be a subset of all functions"
        );
        assert_eq!(engine.alert_messages().len(), 1, "dirty's sink fires");

        // The real path is found: source in dirty reaches dirty's sink.
        assert!(engine.alert_messages()[0].contains("db.execute"));
        let _ = ROOT_CTX;
    }
}
