// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the demand-driven backward taint engine (task 4.4).

#[cfg(test)]
pub mod demand_tests {
    use crate::analysis::forward::ProgramSvfg;
    use crate::analysis::taint::config::TaintConfig;
    use crate::analysis::taint::engine::{BackwardTaintEngine, BackwardVerdict};
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

    /// Create a variable, register it as a formal parameter, and return it.
    fn add_param(ir: &mut FunctionIR, name: &str) -> VarId {
        let v = ir.new_var(dummy_meta(name));
        ir.parameters.push(v);
        v
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

    type RunOut = (
        Vec<crate::analysis::taint::engine::SinkFinding>,
        crate::analysis::taint::engine::BackwardStats,
    );

    fn run_backward(cfg: &TaintConfig, prog: &ProgramSvfg) -> RunOut {
        let mut engine = BackwardTaintEngine::new(prog, cfg);
        engine.run();
        (engine.findings, engine.stats)
    }

    // -----------------------------------------------------------------------
    // Test 1: backward walk through cross edges finds the source, a
    // sink in a leaf function whose taint comes from a caller.
    //
    //   fn main() { v = getSource(); helper(v); }
    //   fn helper(p) { db.execute(p); }
    //
    // Seeding at helper's sink and walking backward must cross
    // formal-param → actual-arg and reach getSource.
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_finds_source_through_arg_edge() {
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

        let cfg = config();
        let prog = build_program(vec![main, helper]);
        let (findings, stats) = run_backward(&cfg, &prog);

        let vulns: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable)
            .collect();
        assert_eq!(
            vulns.len(),
            1,
            "expected 1 vulnerable sink arg; got {:?}",
            findings
        );
        assert_eq!(vulns[0].function, "helper");
        assert_eq!(vulns[0].sink, "db.execute");
        assert!(vulns[0].alert.as_deref().unwrap().contains("[in helper]"));

        // Demand-driven cost: helper must be visited, main must be entered
        // only via the backward walk from its own sink... main has no sink,
        // so total functions visited should be small (helper only).
        assert!(
            stats.sink_args_explored >= 1,
            "at least one sink arg explored"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: backward through a return edge.
    //
    //   fn get() { return getSource(); }
    //   fn main() { v = get(); db.execute(v); }
    //
    // Walking back from main's sink must cross actual-ret → formal-ret.
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_finds_source_through_ret_edge() {
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

        let cfg = config();
        let prog = build_program(vec![get, main]);
        let (findings, _stats) = run_backward(&cfg, &prog);

        let vulns: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable)
            .collect();
        assert_eq!(vulns.len(), 1, "expected 1 vulnerable; got {:?}", findings);
        assert_eq!(vulns[0].function, "main");
    }

    // -----------------------------------------------------------------------
    // Test 3: sanitizer-cut path → Sanitized, not Vulnerable.
    //
    //   fn main() { v = getSource(); w = escapeHtml(v); db.execute(w); }
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_sanitized_path() {
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
                    func: "escapeHtml".into(),
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

        let cfg = config();
        let prog = build_program(vec![main]);
        let (findings, _stats) = run_backward(&cfg, &prog);

        assert_eq!(findings.len(), 1);
        assert_eq!(
            findings[0].verdict,
            BackwardVerdict::Sanitized,
            "sanitizer-cut path must be Sanitized; got {:?}",
            findings
        );
        assert!(findings[0].alert.is_none());
    }

    // -----------------------------------------------------------------------
    // Test 4: pure-literal argument → Clean.
    //
    //   fn main() { db.execute("SELECT 1"); }
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_clean_literal() {
        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m1,
                    mem_in: mem0,
                    func: "db.execute".into(),
                    args: vec![Operand::StringLiteral("SELECT 1".into())],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }

        let cfg = config();
        let prog = build_program(vec![main]);
        let (findings, stats) = run_backward(&cfg, &prog);

        // Literal args are not Var operands → nothing to explore.
        assert_eq!(stats.sink_args_explored, 0);
        assert!(findings.is_empty());
    }

    // -----------------------------------------------------------------------
    // Test 5: unresolvable param → Unknown (callee never analysed).
    //
    //   fn main() { v = LoadGlobal("input"); db.execute(v); }, no source,
    //   and the value comes from an external/unresolvable root.
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_unknown_root() {
        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let v = main.new_var(dummy_meta("v"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::LoadGlobal {
                    dest: v,
                    mem_in: mem0,
                    name: "userInput".into(),
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

        let cfg = config();
        let prog = build_program(vec![main]);
        let (findings, _stats) = run_backward(&cfg, &prog);

        assert_eq!(findings.len(), 1);
        assert_eq!(
            findings[0].verdict,
            BackwardVerdict::Unknown,
            "externally-defined value must classify as Unknown; got {:?}",
            findings
        );
    }

    // -----------------------------------------------------------------------
    // Test 6: demand-driven cost model, functions NOT on any sink's backward
    // path are never visited. 10 functions; only main/helper touch the sink;
    // the 8 noise functions contain sources but no sinks.
    // -----------------------------------------------------------------------
    #[test]
    fn test_demand_driven_visits_only_relevant_functions() {
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

        let mut irs = vec![main, helper];
        for i in 0..8 {
            let mut noise = FunctionIR::new(format!("noise_{}", i));
            let b = noise.entry_block;
            let mem0 = noise.initial_memory_state;
            let v = noise.new_var(dummy_meta("n"));
            let m1 = noise.new_var(mem_meta("m1"));
            // Sources everywhere, forward engines seed and traverse these;
            // the demand-driven engine must never touch them.
            noise.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            noise.set_terminator(b, Terminator::Return { src: None });
            irs.push(noise);
        }

        let cfg = config();
        let prog = build_program(irs);
        let (findings, stats) = run_backward(&cfg, &prog);

        assert_eq!(prog.functions.len(), 10);
        // Vulnerable path found despite noise.
        let vulns: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable)
            .collect();
        assert_eq!(vulns.len(), 1);

        // Backward exploration touched at most main+helper, never the noise.
        assert!(
            stats.functions_visited <= 2,
            "demand-driven exploration must skip noise functions; visited {}",
            stats.functions_visited
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: default mode honours summary suppression, so a sanitizing
    // callee produces Sanitized, matching the forward engine. The opt-out
    // over-approximating mode (`without_suppression`) leaks through the local
    // pass-through edge and reports Vulnerable, documenting the difference.
    //
    //   fn clean(x) { return escapeHtml(x); }
    //   fn main() { v = getSource(); w = clean(v); db.execute(w); }
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_matches_forward_on_sanitizing_callee() {
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

        let cfg = config();
        let prog = build_program(vec![clean, main]);

        // Default mode: suppression honoured → Sanitized, no alert.
        let (f1, _) = run_backward(&cfg, &prog);
        assert_eq!(f1.len(), 1, "one sink arg explored; got {:?}", f1);
        assert!(
            f1.iter().all(|f| f.alert.is_none()),
            "no alert in default mode; got {:?}",
            f1
        );
        assert_eq!(
            f1[0].verdict,
            BackwardVerdict::Sanitized,
            "default (suppression-honouring) mode must classify as Sanitized; got {:?}",
            f1
        );

        // Over-approximating mode: leaks past the sanitizer via the local
        // pass-through edge → Vulnerable. Documented unsound-by-nature mode.
        let mut e2 = BackwardTaintEngine::new(&prog, &cfg).without_suppression();
        e2.run();
        assert_eq!(
            e2.findings[0].verdict,
            BackwardVerdict::Vulnerable,
            "over-approximating mode leaks past the sanitizing callee; got {:?}",
            e2.findings
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: backward agrees with forward on a plain vulnerable chain.
    // -----------------------------------------------------------------------
    #[test]
    fn test_backward_agrees_with_forward() {
        use crate::analysis::forward::InterproceduralTaintEngine;

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

        // Forward.
        let cfg = config();
        let mut fwd = InterproceduralTaintEngine::new(&prog, &cfg);
        fwd.run();

        // Backward.
        let cfg = config();
        let (bwd, _stats) = run_backward(&cfg, &prog);
        let bwd_alerts: Vec<String> = bwd.iter().filter_map(|f| f.alert.clone()).collect();

        assert_eq!(fwd.alerts.len(), 1);
        assert_eq!(bwd_alerts.len(), 1);
        // Same alert text modulo the message wording (both mention the sink).
        assert!(fwd.alerts[0].contains("db.execute"));
        assert!(bwd_alerts[0].contains("db.execute"));
    }
}
