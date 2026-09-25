// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

#[cfg(test)]
pub mod tests {
    use crate::analysis::taint::config::{TaintConfig, TaintEngine};
    use crate::graph::heap::PointsToAnalysis;
    use crate::ir::function::*;
    use crate::ir::ssa::SSABuilder;
    use rustc_hash::FxHashSet;

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

    #[test]
    fn test_ssa_phi_generation() {
        let mut ir = FunctionIR::new("test_func".into());
        let x = ir.new_var(dummy_meta("x"));
        let y = ir.new_var(dummy_meta("y"));

        let b_entry = ir.entry_block;
        let b_true = ir.new_block();
        let b_false = ir.new_block();
        let b_merge = ir.new_block();

        // Build Diamond CFG
        ir.add_edge(b_entry, b_true);
        ir.add_edge(b_entry, b_false);
        ir.add_edge(b_true, b_merge);
        ir.add_edge(b_false, b_merge);

        // Branch 1: x = 1
        ir.push_instruction(
            b_true,
            Instruction::Assign {
                dest: x,
                src: Operand::IntLiteral(1),
            },
        );
        // Branch 2: x = 2
        ir.push_instruction(
            b_false,
            Instruction::Assign {
                dest: x,
                src: Operand::IntLiteral(2),
            },
        );

        // Merge: y = x (Reads the merged x)
        ir.push_instruction(
            b_merge,
            Instruction::Assign {
                dest: y,
                src: Operand::Var(x),
            },
        );

        let ssa_ir = SSABuilder::new(ir).build();

        let merge_block = &ssa_ir.blocks[&b_merge];

        // 1. Assert Phi node was generated for 'x'
        assert_eq!(
            merge_block.phis.len(),
            1,
            "Exactly one Phi node should be generated at the merge block"
        );

        let phi = &merge_block.phis[0];
        assert_eq!(
            phi.incoming.len(),
            2,
            "Phi node should have 2 incoming edges"
        );

        // 2. Assert the y = x instruction was updated to read the Phi destination
        if let Instruction::Assign {
            src: Operand::Var(read_var),
            ..
        } = merge_block.instructions[0]
        {
            assert_eq!(
                read_var, phi.dest,
                "The read instruction should point to the newly generated Phi destination"
            );
        } else {
            panic!("Expected Assign instruction");
        }
    }

    #[test]
    fn test_strong_update_memory_ssa() {
        let mut ir = FunctionIR::new("test_strong_update".into());
        let b_entry = ir.entry_block;

        let mem_0 = ir.initial_memory_state;

        // 1. Get malicious data (Source)
        let malicious = ir.new_var(dummy_meta("malicious"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b_entry,
            Instruction::CallStatic {
                dest: Some(malicious),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        // 2. Allocate object
        let obj = ir.new_var(dummy_meta("obj"));
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b_entry,
            Instruction::Allocate {
                dest: obj,
                mem_out: mem_2,
                mem_in: mem_1,
                kind: AllocationKind::Object,
            },
        );

        // 3. Write malicious data to object: obj.data = malicious
        let mem_3 = ir.new_var(mem_meta("mem_3"));
        ir.push_instruction(
            b_entry,
            Instruction::StoreField {
                mem_out: mem_3,
                mem_in: mem_2,
                base: obj,
                field: "data".into(),
                src: Operand::Var(malicious),
            },
        );

        // 4. Overwrite with safe data: obj.data = "safe" (STRONG UPDATE)
        let mem_4 = ir.new_var(mem_meta("mem_4"));
        ir.push_instruction(
            b_entry,
            Instruction::StoreField {
                mem_out: mem_4,
                mem_in: mem_3,
                base: obj,
                field: "data".into(),
                src: Operand::StringLiteral("safe".into()),
            },
        );

        // 5. Read back: let val = obj.data
        let val = ir.new_var(dummy_meta("val"));
        ir.push_instruction(
            b_entry,
            Instruction::LoadField {
                dest: val,
                mem_in: mem_4,
                base: obj,
                field: "data".into(),
            },
        );

        // 6. Sink: db.execute(val)
        let mem_5 = ir.new_var(mem_meta("mem_5"));
        ir.push_instruction(
            b_entry,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_5,
                mem_in: mem_4,
                func: "db.execute".into(),
                args: vec![Operand::Var(val)],
            },
        );

        // Run Heap Analysis
        let mut heap = PointsToAnalysis::new();
        heap.analyze(&ir);

        // Run Taint Analysis
        let mut sources = FxHashSet::default();
        sources.insert("getSource".into());
        let mut sinks = FxHashSet::default();
        sinks.insert("db.execute".into());

        let config = TaintConfig {
            sources,
            sinks,
            sanitizers: FxHashSet::default(),
        };

        let mut taint_engine = TaintEngine::new(&ir, &heap, &config);
        taint_engine.run();

        assert!(
            taint_engine.alerts.is_empty(),
            "Strong update should have cleared the taint, but an alert was thrown: {:?}",
            taint_engine.alerts
        );
    }

    #[test]
    fn test_juiceshop_loop_convergence() {
        // Simulates:
        // let data = "clean";
        // while (cond) {
        //    db.execute(data); // 1st pass: clean. 2nd pass: TAINTED!
        //    data = req.body.malicious;
        // }

        let mut ir = FunctionIR::new("test_loop".into());
        let b_entry = ir.entry_block;
        let b_loop_header = ir.new_block();
        let b_loop_body = ir.new_block();
        let b_exit = ir.new_block();

        ir.add_edge(b_entry, b_loop_header);
        ir.add_edge(b_loop_header, b_loop_body);
        ir.add_edge(b_loop_body, b_loop_header); // Back-edge!
        ir.add_edge(b_loop_header, b_exit);

        // Entry: data_0 = "clean"
        let data_0 = ir.new_var(dummy_meta("data_0"));
        ir.push_instruction(
            b_entry,
            Instruction::Assign {
                dest: data_0,
                src: Operand::StringLiteral("clean".into()),
            },
        );

        // Loop Header: data_1 = Phi(data_0, data_2)
        // (We don't run SSABuilder here to keep it isolated, we manually construct the Phi)
        let data_1 = ir.new_var(dummy_meta("data_1"));
        let data_2 = ir.new_var(dummy_meta("data_2"));
        ir.push_phi(
            b_loop_header,
            Phi {
                dest: data_1,
                incoming: vec![(b_entry, data_0), (b_loop_body, data_2)],
            },
        );

        let mem_0 = ir.initial_memory_state;

        // Loop Body: db.execute(data_1)
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b_loop_body,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_1,
                mem_in: mem_0,
                func: "db.execute".into(),
                args: vec![Operand::Var(data_1)],
            },
        );

        // Loop Body: data_2 = getSource()
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b_loop_body,
            Instruction::CallStatic {
                dest: Some(data_2),
                mem_out: mem_2,
                mem_in: mem_1,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let heap = PointsToAnalysis::new();
        let mut sources = FxHashSet::default();
        sources.insert("getSource".into());
        let mut sinks = FxHashSet::default();
        sinks.insert("db.execute".into());

        let config = TaintConfig {
            sources,
            sinks,
            sanitizers: FxHashSet::default(),
        };
        let mut engine = TaintEngine::new(&ir, &heap, &config);

        // This is the true test of Fixed-Point Iteration.
        // It must run twice because taint flows BACKWARDS up the CFG back-edge.
        engine.run();

        assert_eq!(
            engine.alerts.len(),
            1,
            "Failed to catch vulnerability in loop back-edge! Engine stopped iterating too early."
        );
        assert!(
            engine.alerts[0].contains("db.execute"),
            "Alert should trigger on db.execute"
        );
    }

    #[test]
    fn test_juiceshop_deep_aliasing_and_wildcards() {
        // Simulates:
        // let req = getSource();
        // let body = req.body;
        // let arr = [];
        // arr[0] = body;
        // let alias = arr[dynamic_index];
        // db.execute(alias.email);

        let mut ir = FunctionIR::new("test_deep_alias".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        // req = getSource()
        let req = ir.new_var(dummy_meta("req"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(req),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        // req points to Loc_1 (Simulate parameter injection)
        let mut heap = PointsToAnalysis::new();
        let loc_req = heap.alloc();
        heap.pts.entry(req).or_default().insert(loc_req);

        // req.body = TAINTED
        // Note: For objects flowing from sources, their fields are implicitly tainted.
        // In our manual test, we simulate extracting the tainted field.
        let body = ir.new_var(dummy_meta("body"));
        ir.push_instruction(
            b,
            Instruction::Assign {
                dest: body,
                src: Operand::Var(req),
            },
        ); // Direct alias for simplicity of source

        // arr = Allocate
        let arr = ir.new_var(dummy_meta("arr"));
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::Allocate {
                dest: arr,
                mem_out: mem_2,
                mem_in: mem_1,
                kind: AllocationKind::Array,
            },
        );

        // arr[0] = body (StoreElement)
        let mem_3 = ir.new_var(mem_meta("mem_3"));
        ir.push_instruction(
            b,
            Instruction::StoreElement {
                mem_out: mem_3,
                mem_in: mem_2,
                base: arr,
                index: Operand::IntLiteral(0),
                src: Operand::Var(body),
            },
        );

        // alias = arr[dynamic] (LoadElement wildcard)
        let alias = ir.new_var(dummy_meta("alias"));
        let dyn_idx = ir.new_var(dummy_meta("dynamic_index"));
        ir.push_instruction(
            b,
            Instruction::LoadElement {
                dest: alias,
                mem_in: mem_3,
                base: arr,
                index: Operand::Var(dyn_idx),
            },
        );

        // db.execute(alias.email) -> Sink!
        let email = ir.new_var(dummy_meta("email"));
        ir.push_instruction(
            b,
            Instruction::LoadField {
                dest: email,
                mem_in: mem_3,
                base: alias,
                field: "email".into(),
            },
        );

        let mem_4 = ir.new_var(mem_meta("mem_4"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_4,
                mem_in: mem_3,
                func: "db.execute".into(),
                args: vec![Operand::Var(alias)],
            },
        );

        heap.analyze(&ir);

        let mut sources = FxHashSet::default();
        sources.insert("getSource".into());
        let mut sinks = FxHashSet::default();
        sinks.insert("db.execute".into());
        let config = TaintConfig {
            sources,
            sinks,
            sanitizers: FxHashSet::default(),
        };
        let mut engine = TaintEngine::new(&ir, &heap, &config);

        engine.run();

        assert_eq!(
            engine.alerts.len(),
            1,
            "Failed to catch wildcard array element alias taint!"
        );
    }

    #[test]
    fn test_sanitizers_and_weak_updates() {
        // Simulates:
        // let arr = [];
        // arr[0] = getSource();        // arr[0] is tainted
        // arr[dynamic] = "clean";      // Should NOT clean arr[0] (Weak Update)
        // let val = arr[0];            // Still tainted
        // let safe = escapeHtml(val);  // Sanitized!
        // db.execute(val);             // ALERT
        // db.execute(safe);            // NO ALERT

        let mut ir = FunctionIR::new("test_sanitizers".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        // arr = Allocate
        let arr = ir.new_var(dummy_meta("arr"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::Allocate {
                dest: arr,
                mem_out: mem_1,
                mem_in: mem_0,
                kind: AllocationKind::Array,
            },
        );

        // malicious = getSource()
        let malicious = ir.new_var(dummy_meta("malicious"));
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(malicious),
                mem_out: mem_2,
                mem_in: mem_1,
                func: "getSource".into(),
                args: vec![],
            },
        );

        // arr[0] = malicious
        let mem_3 = ir.new_var(mem_meta("mem_3"));
        ir.push_instruction(
            b,
            Instruction::StoreElement {
                mem_out: mem_3,
                mem_in: mem_2,
                base: arr,
                index: Operand::IntLiteral(0),
                src: Operand::Var(malicious),
            },
        );

        // arr[dynamic] = "clean" (Weak Update - should NOT erase arr[0] taint)
        let dyn_idx = ir.new_var(dummy_meta("dynamic_index"));
        let mem_4 = ir.new_var(mem_meta("mem_4"));
        ir.push_instruction(
            b,
            Instruction::StoreElement {
                mem_out: mem_4,
                mem_in: mem_3,
                base: arr,
                index: Operand::Var(dyn_idx),
                src: Operand::StringLiteral("clean".into()),
            },
        );

        // val = arr[0]
        let val = ir.new_var(dummy_meta("val"));
        ir.push_instruction(
            b,
            Instruction::LoadElement {
                dest: val,
                mem_in: mem_4,
                base: arr,
                index: Operand::IntLiteral(0),
            },
        );

        // safe = escapeHtml(val)
        let safe = ir.new_var(dummy_meta("safe"));
        let mem_5 = ir.new_var(mem_meta("mem_5"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(safe),
                mem_out: mem_5,
                mem_in: mem_4,
                func: "escapeHtml".into(),
                args: vec![Operand::Var(val)],
            },
        );

        // db.execute(val) -> ALERTS
        let mem_6 = ir.new_var(mem_meta("mem_6"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_6,
                mem_in: mem_5,
                func: "db.execute".into(),
                args: vec![Operand::Var(val)],
            },
        );

        // db.execute(safe) -> NO ALERT
        let mem_7 = ir.new_var(mem_meta("mem_7"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_7,
                mem_in: mem_6,
                func: "db.execute".into(),
                args: vec![Operand::Var(safe)],
            },
        );

        let mut heap = PointsToAnalysis::new();
        heap.analyze(&ir);

        let mut sources = FxHashSet::default();
        sources.insert("getSource".into());
        let mut sinks = FxHashSet::default();
        sinks.insert("db.execute".into());
        let mut sanitizers = FxHashSet::default();
        sanitizers.insert("escapeHtml".into());

        let config = TaintConfig {
            sources,
            sinks,
            sanitizers,
        };
        let mut engine = TaintEngine::new(&ir, &heap, &config);

        engine.run();

        assert_eq!(
            engine.alerts.len(),
            1,
            "Expected exactly 1 alert. Weak update shouldn't clean the array, and sanitizer should protect the second call."
        );
        assert!(
            engine.alerts[0].contains("db.execute"),
            "Alert should be on the unsanitized val."
        );
    }

    #[test]
    fn test_nested_control_flow_ssa() {
        // Simulates:
        // let x = 0;
        // if (cond1) {
        //     while (cond2) {
        //         x = 1;
        //     }
        // }
        // let y = x;
        // Proves SSABuilder Dominance Frontiers can handle nested loops and branches perfectly.

        let mut ir = FunctionIR::new("test_nested".into());
        let b_entry = ir.entry_block;
        let b_if_true = ir.new_block();
        let b_while_header = ir.new_block();
        let b_while_body = ir.new_block();
        let b_while_exit = ir.new_block();
        let b_merge = ir.new_block();

        ir.add_edge(b_entry, b_if_true);
        ir.add_edge(b_entry, b_merge); // if false, jump straight to merge

        ir.add_edge(b_if_true, b_while_header);
        ir.add_edge(b_while_header, b_while_body);
        ir.add_edge(b_while_body, b_while_header); // loop back
        ir.add_edge(b_while_header, b_while_exit);
        ir.add_edge(b_while_exit, b_merge);

        // x_0 = 0
        let x = ir.new_var(dummy_meta("x"));
        ir.push_instruction(
            b_entry,
            Instruction::Assign {
                dest: x,
                src: Operand::IntLiteral(0),
            },
        );

        // Loop body: x_1 = 1
        ir.push_instruction(
            b_while_body,
            Instruction::Assign {
                dest: x,
                src: Operand::IntLiteral(1),
            },
        );

        // Merge: y = x
        let y = ir.new_var(dummy_meta("y"));
        ir.push_instruction(
            b_merge,
            Instruction::Assign {
                dest: y,
                src: Operand::Var(x),
            },
        );

        let ssa_ir = SSABuilder::new(ir).build();

        // There should be a Phi node in b_while_header to merge x_0 (from entry) and x_1 (from loop body)
        let while_header_phis = &ssa_ir.blocks[&b_while_header].phis;
        assert_eq!(
            while_header_phis.len(),
            1,
            "Expected Phi node in while header"
        );

        // There should be a Phi node in b_merge to merge x_0 (from entry/if false) and the x from the while loop
        let merge_phis = &ssa_ir.blocks[&b_merge].phis;
        assert_eq!(merge_phis.len(), 1, "Expected Phi node in if-merge block");
    }
}
