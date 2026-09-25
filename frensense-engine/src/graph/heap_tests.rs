// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for field-sensitive heap flows (task 4.3/8.2):
//! local store→load edges and cross-function heap edges.

#[cfg(test)]
pub mod heap_flow_tests {
    use crate::analysis::forward::ProgramSvfg;
    use crate::analysis::taint::config::TaintConfig;
    use crate::analysis::taint::engine::BackwardTaintEngine;
    use crate::ir::function::*;
    use crate::ir::ssa::SSABuilder;
    use rustc_hash::FxHashMap;

    fn meta(name: &str) -> VarMetadata {
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
            sources: ["getSource", "req.body"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            sinks: ["db.execute", "sink"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            sanitizers: ["escapeHtml"].iter().map(|s| s.to_string()).collect(),
        }
    }

    /// One function: v1 = getSource(); obj.data = v1; v2 = obj.data; sink(v2)
    #[test]
    fn test_heap_flow_local_store_then_load() {
        let mut ir = FunctionIR::new("local_heap".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        // obj = Allocate
        let obj = ir.new_var(meta("obj"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::Allocate {
                dest: obj,
                mem_out: mem_1,
                mem_in: mem_0,
                kind: AllocationKind::Object,
            },
        );

        // v1 = getSource()
        let v1 = ir.new_var(meta("v1"));
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_2,
                mem_in: mem_1,
                func: "getSource".into(),
                args: vec![],
            },
        );

        // obj.data = v1
        let mem_3 = ir.new_var(mem_meta("mem_3"));
        ir.push_instruction(
            b,
            Instruction::StoreField {
                mem_out: mem_3,
                mem_in: mem_2,
                base: obj,
                field: "data".into(),
                src: Operand::Var(v1),
            },
        );

        // v2 = obj.data
        let v2 = ir.new_var(meta("v2"));
        ir.push_instruction(
            b,
            Instruction::LoadField {
                dest: v2,
                mem_in: mem_3,
                base: obj,
                field: "data".into(),
            },
        );

        // sink(v2)
        let mem_4 = ir.new_var(mem_meta("mem_4"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_4,
                mem_in: mem_3,
                func: "sink".into(),
                args: vec![Operand::Var(v2)],
            },
        );

        let ssa = SSABuilder::new(ir).build();
        let mut irs: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        let ir_ref: &'static FunctionIR = Box::leak(Box::new(ssa));
        irs.insert("local_heap".into(), ir_ref);

        let cfg = config();
        let prog = ProgramSvfg::new(&irs, &cfg);
        let mut engine = BackwardTaintEngine::new(&prog, &cfg);
        engine.run();

        let vulns: Vec<_> = engine
            .findings
            .iter()
            .filter(|f| f.verdict == crate::analysis::taint::engine::BackwardVerdict::Vulnerable)
            .collect();
        assert_eq!(
            vulns.len(),
            1,
            "obj.data = source(); sink(obj.data) must alert; findings: {:?}",
            engine.findings
        );
    }

    /// Field sensitivity: writing obj.a must NOT taint a read of obj.b.
    #[test]
    fn test_heap_flow_field_sensitive_no_false_positive() {
        let mut ir = FunctionIR::new("field_sensitive".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let obj = ir.new_var(meta("obj"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::Allocate {
                dest: obj,
                mem_out: mem_1,
                mem_in: mem_0,
                kind: AllocationKind::Object,
            },
        );

        let v1 = ir.new_var(meta("v1"));
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_2,
                mem_in: mem_1,
                func: "getSource".into(),
                args: vec![],
            },
        );

        // obj.a = v1  (tainted field)
        let mem_3 = ir.new_var(mem_meta("mem_3"));
        ir.push_instruction(
            b,
            Instruction::StoreField {
                mem_out: mem_3,
                mem_in: mem_2,
                base: obj,
                field: "a".into(),
                src: Operand::Var(v1),
            },
        );

        // obj.b = "clean"
        let mem_4 = ir.new_var(mem_meta("mem_4"));
        ir.push_instruction(
            b,
            Instruction::StoreField {
                mem_out: mem_4,
                mem_in: mem_3,
                base: obj,
                field: "b".into(),
                src: Operand::StringLiteral("clean".into()),
            },
        );

        // v2 = obj.b  (must stay clean)
        let v2 = ir.new_var(meta("v2"));
        ir.push_instruction(
            b,
            Instruction::LoadField {
                dest: v2,
                mem_in: mem_4,
                base: obj,
                field: "b".into(),
            },
        );

        // sink(v2)
        let mem_5 = ir.new_var(mem_meta("mem_5"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_5,
                mem_in: mem_4,
                func: "sink".into(),
                args: vec![Operand::Var(v2)],
            },
        );

        let ssa = SSABuilder::new(ir).build();
        let mut irs: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        let ir_ref: &'static FunctionIR = Box::leak(Box::new(ssa));
        irs.insert("field_sensitive".into(), ir_ref);

        let cfg = config();
        let prog = ProgramSvfg::new(&irs, &cfg);
        let mut engine = BackwardTaintEngine::new(&prog, &cfg);
        engine.run();

        let vulns: Vec<_> = engine
            .findings
            .iter()
            .filter(|f| f.verdict == crate::analysis::taint::engine::BackwardVerdict::Vulnerable)
            .collect();
        assert!(
            vulns.is_empty(),
            "Different fields must not leak taint; got {:?}",
            vulns
        );
    }

    /// Cross-function: caller stores into an object, callee loads from it.
    ///
    /// caller:  obj = {}; obj.data = getSource(); helper(obj)
    /// callee:  function helper(p) { v = p.data; sink(v); }
    #[test]
    fn test_heap_flow_cross_function_param_object() {
        let mut callee = FunctionIR::new("helper".into());
        {
            let b = callee.entry_block;
            let mem_0 = callee.initial_memory_state;
            // Formal param p is callee.parameters[0], create it.
            let p = callee.new_var(meta("p"));
            callee.parameters.push(p);
            // v = p.data
            let v = callee.new_var(meta("v"));
            callee.push_instruction(
                b,
                Instruction::LoadField {
                    dest: v,
                    mem_in: mem_0,
                    base: p,
                    field: "data".into(),
                },
            );
            // sink(v)
            let mem_1 = callee.new_var(mem_meta("mem_1"));
            callee.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem_1,
                    mem_in: mem_0,
                    func: "sink".into(),
                    args: vec![Operand::Var(v)],
                },
            );
        }

        let mut caller = FunctionIR::new("caller".into());
        {
            let b = caller.entry_block;
            let mem_0 = caller.initial_memory_state;

            let obj = caller.new_var(meta("obj"));
            let mem_1 = caller.new_var(mem_meta("mem_1"));
            caller.push_instruction(
                b,
                Instruction::Allocate {
                    dest: obj,
                    mem_out: mem_1,
                    mem_in: mem_0,
                    kind: AllocationKind::Object,
                },
            );

            let v1 = caller.new_var(meta("v1"));
            let mem_2 = caller.new_var(mem_meta("mem_2"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v1),
                    mem_out: mem_2,
                    mem_in: mem_1,
                    func: "getSource".into(),
                    args: vec![],
                },
            );

            // obj.data = v1
            let mem_3 = caller.new_var(mem_meta("mem_3"));
            caller.push_instruction(
                b,
                Instruction::StoreField {
                    mem_out: mem_3,
                    mem_in: mem_2,
                    base: obj,
                    field: "data".into(),
                    src: Operand::Var(v1),
                },
            );

            // helper(obj)
            let mem_4 = caller.new_var(mem_meta("mem_4"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem_4,
                    mem_in: mem_3,
                    func: "helper".into(),
                    args: vec![Operand::Var(obj)],
                },
            );
        }

        let callee_ssa = SSABuilder::new(callee).build();
        let caller_ssa = SSABuilder::new(caller).build();
        let mut irs: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        let callee_ref: &'static FunctionIR = Box::leak(Box::new(callee_ssa));
        let caller_ref: &'static FunctionIR = Box::leak(Box::new(caller_ssa));
        irs.insert("helper".into(), callee_ref);
        irs.insert("caller".into(), caller_ref);

        let cfg = config();
        let prog = ProgramSvfg::new(&irs, &cfg);
        let mut engine = BackwardTaintEngine::new(&prog, &cfg);
        engine.run();

        let vulns: Vec<_> = engine
            .findings
            .iter()
            .filter(|f| f.verdict == crate::analysis::taint::engine::BackwardVerdict::Vulnerable)
            .collect();
        assert_eq!(
            vulns.len(),
            1,
            "Caller store → callee load through a parameter object must alert; findings: {:?}",
            engine.findings
        );
        assert_eq!(vulns[0].function, "helper");
    }

    /// Reverse direction: callee mutates its parameter object, caller reads.
    ///
    /// callee: function fill(p) { p.data = getSource(); }
    /// caller: obj = {}; fill(obj); v = obj.data; sink(v);
    #[test]
    fn test_heap_flow_callee_mutates_param_object() {
        let mut callee = FunctionIR::new("fill".into());
        {
            let b = callee.entry_block;
            let mem_0 = callee.initial_memory_state;
            let p = callee.new_var(meta("p"));
            callee.parameters.push(p);

            let v1 = callee.new_var(meta("v1"));
            let mem_1 = callee.new_var(mem_meta("mem_1"));
            callee.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v1),
                    mem_out: mem_1,
                    mem_in: mem_0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            // p.data = v1
            let mem_2 = callee.new_var(mem_meta("mem_2"));
            callee.push_instruction(
                b,
                Instruction::StoreField {
                    mem_out: mem_2,
                    mem_in: mem_1,
                    base: p,
                    field: "data".into(),
                    src: Operand::Var(v1),
                },
            );
        }

        let mut caller = FunctionIR::new("caller2".into());
        {
            let b = caller.entry_block;
            let mem_0 = caller.initial_memory_state;

            let obj = caller.new_var(meta("obj"));
            let mem_1 = caller.new_var(mem_meta("mem_1"));
            caller.push_instruction(
                b,
                Instruction::Allocate {
                    dest: obj,
                    mem_out: mem_1,
                    mem_in: mem_0,
                    kind: AllocationKind::Object,
                },
            );

            // fill(obj)
            let mem_2 = caller.new_var(mem_meta("mem_2"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem_2,
                    mem_in: mem_1,
                    func: "fill".into(),
                    args: vec![Operand::Var(obj)],
                },
            );

            // v = obj.data
            let v = caller.new_var(meta("v"));
            caller.push_instruction(
                b,
                Instruction::LoadField {
                    dest: v,
                    mem_in: mem_2,
                    base: obj,
                    field: "data".into(),
                },
            );

            // sink(v)
            let mem_3 = caller.new_var(mem_meta("mem_3"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem_3,
                    mem_in: mem_2,
                    func: "sink".into(),
                    args: vec![Operand::Var(v)],
                },
            );
        }

        let callee_ssa = SSABuilder::new(callee).build();
        let caller_ssa = SSABuilder::new(caller).build();
        let mut irs: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        let callee_ref: &'static FunctionIR = Box::leak(Box::new(callee_ssa));
        let caller_ref: &'static FunctionIR = Box::leak(Box::new(caller_ssa));
        irs.insert("fill".into(), callee_ref);
        irs.insert("caller2".into(), caller_ref);

        let cfg = config();
        let prog = ProgramSvfg::new(&irs, &cfg);
        let mut engine = BackwardTaintEngine::new(&prog, &cfg);
        engine.run();

        let vulns: Vec<_> = engine
            .findings
            .iter()
            .filter(|f| f.verdict == crate::analysis::taint::engine::BackwardVerdict::Vulnerable)
            .collect();
        assert_eq!(
            vulns.len(),
            1,
            "Callee mutation of a caller object must alert; findings: {:?}",
            engine.findings
        );
        assert_eq!(vulns[0].function, "caller2");
    }

    /// Negative: cross-function heap flow between *different fields* stays clean.
    #[test]
    fn test_heap_flow_cross_function_field_mismatch() {
        let mut callee = FunctionIR::new("helper_fs".into());
        {
            let b = callee.entry_block;
            let mem_0 = callee.initial_memory_state;
            let p = callee.new_var(meta("p"));
            callee.parameters.push(p);
            // v = p.other  (different field than what the caller stored)
            let v = callee.new_var(meta("v"));
            callee.push_instruction(
                b,
                Instruction::LoadField {
                    dest: v,
                    mem_in: mem_0,
                    base: p,
                    field: "other".into(),
                },
            );
            let mem_1 = callee.new_var(mem_meta("mem_1"));
            callee.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem_1,
                    mem_in: mem_0,
                    func: "sink".into(),
                    args: vec![Operand::Var(v)],
                },
            );
        }

        let mut caller = FunctionIR::new("caller_fs".into());
        {
            let b = caller.entry_block;
            let mem_0 = caller.initial_memory_state;

            let obj = caller.new_var(meta("obj"));
            let mem_1 = caller.new_var(mem_meta("mem_1"));
            caller.push_instruction(
                b,
                Instruction::Allocate {
                    dest: obj,
                    mem_out: mem_1,
                    mem_in: mem_0,
                    kind: AllocationKind::Object,
                },
            );

            let v1 = caller.new_var(meta("v1"));
            let mem_2 = caller.new_var(mem_meta("mem_2"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(v1),
                    mem_out: mem_2,
                    mem_in: mem_1,
                    func: "getSource".into(),
                    args: vec![],
                },
            );

            // obj.data = v1, stores "data", callee reads "other"
            let mem_3 = caller.new_var(mem_meta("mem_3"));
            caller.push_instruction(
                b,
                Instruction::StoreField {
                    mem_out: mem_3,
                    mem_in: mem_2,
                    base: obj,
                    field: "data".into(),
                    src: Operand::Var(v1),
                },
            );

            let mem_4 = caller.new_var(mem_meta("mem_4"));
            caller.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: mem_4,
                    mem_in: mem_3,
                    func: "helper_fs".into(),
                    args: vec![Operand::Var(obj)],
                },
            );
        }

        let callee_ssa = SSABuilder::new(callee).build();
        let caller_ssa = SSABuilder::new(caller).build();
        let mut irs: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        let callee_ref: &'static FunctionIR = Box::leak(Box::new(callee_ssa));
        let caller_ref: &'static FunctionIR = Box::leak(Box::new(caller_ssa));
        irs.insert("helper_fs".into(), callee_ref);
        irs.insert("caller_fs".into(), caller_ref);

        let cfg = config();
        let prog = ProgramSvfg::new(&irs, &cfg);
        let mut engine = BackwardTaintEngine::new(&prog, &cfg);
        engine.run();

        let vulns: Vec<_> = engine
            .findings
            .iter()
            .filter(|f| f.verdict == crate::analysis::taint::engine::BackwardVerdict::Vulnerable)
            .collect();
        assert!(
            vulns.is_empty(),
            "Field-mismatched cross-function heap flow must stay clean; got {:?}",
            vulns
        );
    }
}
