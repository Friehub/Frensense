// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the Sparse Value-Flow Graph (SVFG) module.
//!
//! Each test mirrors an existing `TaintEngine` test but uses `SvfgTaintEngine`
//! to demonstrate that the graph-traversal approach produces the same results
//! without a fixed-point loop.

#[cfg(test)]
pub mod svfg_tests {
    use crate::analysis::taint::config::TaintConfig;
    use crate::graph::heap::PointsToAnalysis;
    use crate::graph::svfg::{NodeKind, SvfgBuilder, SvfgTaintEngine};
    use crate::ir::function::*;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

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

    fn default_config_with(sources: &[&str], sinks: &[&str], sanitizers: &[&str]) -> TaintConfig {
        TaintConfig {
            sources: sources.iter().map(|s| s.to_string()).collect(),
            sinks: sinks.iter().map(|s| s.to_string()).collect(),
            sanitizers: sanitizers.iter().map(|s| s.to_string()).collect(),
        }
    }

    // -----------------------------------------------------------------------
    // Test 1: SVFG is built and edges are correct for a simple assignment chain
    //
    // IR:  v1 = getSource()   (def)
    //      v2 = v1            (use v1, def v2)
    // Expected SVFG edges: def(v1) → use(v2_instr), def(v2) exists
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_simple_assignment_edges() {
        let mut ir = FunctionIR::new("test_edges".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let v1 = ir.new_var(dummy_meta("v1"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let v2 = ir.new_var(dummy_meta("v2"));
        ir.push_instruction(
            b,
            Instruction::Assign {
                dest: v2,
                src: Operand::Var(v1),
            },
        );

        let (graph, def_site) = SvfgBuilder::new(&ir).build();

        let stats = graph.stats();
        println!("SVFG stats: {}", stats);

        // v1 must have a def node
        let v1_def_key = def_site.get(&v1).copied().expect("v1 must have a def site");
        let v1_node = graph.node(&v1_def_key).expect("v1 def node must exist");

        // v1's def node must have at least one successor (the Assign that uses v1)
        assert!(
            !v1_node.successors().is_empty(),
            "v1 def node should have successors (the Assign instruction uses it)"
        );

        // v2 must have a def node
        assert!(def_site.contains_key(&v2), "v2 must have a def site");
    }

    // -----------------------------------------------------------------------
    // Test 2: BFS taint reaches a sink through a chain
    //
    // IR:  v1 = getSource()
    //      v2 = v1
    //      db.execute(v2)
    // Expected: 1 alert on db.execute
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_taint_chain_to_sink() {
        let mut ir = FunctionIR::new("test_chain".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let v1 = ir.new_var(dummy_meta("v1"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let v2 = ir.new_var(dummy_meta("v2"));
        ir.push_instruction(
            b,
            Instruction::Assign {
                dest: v2,
                src: Operand::Var(v1),
            },
        );

        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_2,
                mem_in: mem_1,
                func: "db.execute".into(),
                args: vec![Operand::Var(v2)],
            },
        );

        let heap = PointsToAnalysis::new();
        let config = default_config_with(&["getSource"], &["db.execute"], &[]);
        let (graph, def_site) = SvfgBuilder::new(&ir).build();

        let mut engine = SvfgTaintEngine::new(&ir, &heap, &config, &graph, &def_site);
        engine.run();

        assert_eq!(
            engine.alerts.len(),
            1,
            "Expected 1 alert; got: {:?}",
            engine.alerts
        );
        assert!(engine.alerts[0].contains("db.execute"));
    }

    // -----------------------------------------------------------------------
    // Test 3: Sanitizer blocks taint propagation
    //
    // IR:  v1 = getSource()
    //      v2 = escapeHtml(v1)   ← sanitizer
    //      db.execute(v2)        ← should NOT alert
    //      db.execute(v1)        ← SHOULD alert (unsanitized path)
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_sanitizer_blocks_propagation() {
        let mut ir = FunctionIR::new("test_sanitizer".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let v1 = ir.new_var(dummy_meta("v1"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let v2 = ir.new_var(dummy_meta("v2"));
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v2),
                mem_out: mem_2,
                mem_in: mem_1,
                func: "escapeHtml".into(),
                args: vec![Operand::Var(v1)],
            },
        );

        // safe use of v2
        let mem_3 = ir.new_var(mem_meta("mem_3"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_3,
                mem_in: mem_2,
                func: "db.execute".into(),
                args: vec![Operand::Var(v2)],
            },
        );

        // unsafe use of v1
        let mem_4 = ir.new_var(mem_meta("mem_4"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_4,
                mem_in: mem_3,
                func: "db.execute".into(),
                args: vec![Operand::Var(v1)],
            },
        );

        let heap = PointsToAnalysis::new();
        let config = default_config_with(&["getSource"], &["db.execute"], &["escapeHtml"]);
        let (graph, def_site) = SvfgBuilder::new(&ir).build();

        let mut engine = SvfgTaintEngine::new(&ir, &heap, &config, &graph, &def_site);
        engine.run();

        // Only the direct v1 → db.execute path should fire.
        // The v1 → escapeHtml → v2 → db.execute path should be stopped at the sanitizer.
        assert_eq!(
            engine.alerts.len(),
            1,
            "Sanitizer should block taint to v2; only v1 path should alert. Got: {:?}",
            engine.alerts
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: No false positive when source and sink are disconnected
    //
    // IR:  v1 = getSource()
    //      v2 = "safe literal"
    //      db.execute(v2)
    // Expected: 0 alerts
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_no_false_positive_disconnected() {
        let mut ir = FunctionIR::new("test_disconnected".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let v1 = ir.new_var(dummy_meta("v1"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let v2 = ir.new_var(dummy_meta("v2"));
        ir.push_instruction(
            b,
            Instruction::Assign {
                dest: v2,
                src: Operand::StringLiteral("safe".into()),
            },
        );

        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_2,
                mem_in: mem_1,
                func: "db.execute".into(),
                args: vec![Operand::Var(v2)],
            },
        );

        let heap = PointsToAnalysis::new();
        let config = default_config_with(&["getSource"], &["db.execute"], &[]);
        let (graph, def_site) = SvfgBuilder::new(&ir).build();

        let stats = graph.stats();
        println!("Disconnected test SVFG: {}", stats);

        let mut engine = SvfgTaintEngine::new(&ir, &heap, &config, &graph, &def_site);
        engine.run();

        assert!(
            engine.alerts.is_empty(),
            "No taint should flow from a source to a disconnected sink. Got: {:?}",
            engine.alerts
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: SVFG correctly handles Phi nodes (branch-merge taint)
    //
    // IR (diamond CFG):
    //   entry: v_cond = ...
    //   true_branch:  v1 = getSource()
    //   false_branch: v1 = "safe"
    //   merge: v_phi = Phi(v1_true, v1_false)
    //          db.execute(v_phi)
    //
    // After SSA: v1_1 in true, v1_2 in false, v_phi merges them.
    // The tainted v1_1 should flow through the Phi to db.execute.
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_phi_node_taint_propagation() {
        let mut ir = FunctionIR::new("test_phi_taint".into());
        let b_entry = ir.entry_block;
        let b_true = ir.new_block();
        let b_false = ir.new_block();
        let b_merge = ir.new_block();

        ir.add_edge(b_entry, b_true);
        ir.add_edge(b_entry, b_false);
        ir.add_edge(b_true, b_merge);
        ir.add_edge(b_false, b_merge);

        let mem_0 = ir.initial_memory_state;

        // true branch: v_src = getSource()
        let v_src = ir.new_var(dummy_meta("v_src"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b_true,
            Instruction::CallStatic {
                dest: Some(v_src),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        // false branch: v_safe = "clean"
        let v_safe = ir.new_var(dummy_meta("v_safe"));
        ir.push_instruction(
            b_false,
            Instruction::Assign {
                dest: v_safe,
                src: Operand::StringLiteral("clean".into()),
            },
        );

        // merge: v_phi = Phi(v_src, v_safe)
        let v_phi = ir.new_var(dummy_meta("v_phi"));
        ir.push_phi(
            b_merge,
            Phi {
                dest: v_phi,
                incoming: vec![(b_true, v_src), (b_false, v_safe)],
            },
        );

        // sink in merge block
        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b_merge,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_2,
                mem_in: mem_0,
                func: "db.execute".into(),
                args: vec![Operand::Var(v_phi)],
            },
        );

        let heap = PointsToAnalysis::new();
        let config = default_config_with(&["getSource"], &["db.execute"], &[]);
        let (graph, def_site) = SvfgBuilder::new(&ir).build();

        let stats = graph.stats();
        println!("Phi propagation SVFG: {}", stats);

        // Confirm the Phi node exists in the graph
        let phi_node_key = def_site.get(&v_phi);
        assert!(
            phi_node_key.is_some(),
            "Phi node for v_phi must be in def_site"
        );
        let phi_node = graph
            .node(phi_node_key.unwrap())
            .expect("Phi node must exist");
        assert_eq!(phi_node.kind, NodeKind::Phi, "Node should be a Phi");

        let mut engine = SvfgTaintEngine::new(&ir, &heap, &config, &graph, &def_site);
        engine.run();

        assert_eq!(
            engine.alerts.len(),
            1,
            "Taint through Phi should reach sink. Got: {:?}",
            engine.alerts
        );
    }

    // -----------------------------------------------------------------------
    // Test 6: Graph statistics are non-trivial for a real program
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_stats() {
        let mut ir = FunctionIR::new("test_stats".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let v1 = ir.new_var(dummy_meta("v1"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let v2 = ir.new_var(dummy_meta("v2"));
        ir.push_instruction(
            b,
            Instruction::BinaryOp {
                dest: v2,
                op: "+".into(),
                lhs: Operand::Var(v1),
                rhs: Operand::IntLiteral(1),
            },
        );

        let v3 = ir.new_var(dummy_meta("v3"));
        ir.push_instruction(
            b,
            Instruction::Assign {
                dest: v3,
                src: Operand::Var(v2),
            },
        );

        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_2,
                mem_in: mem_1,
                func: "db.execute".into(),
                args: vec![Operand::Var(v3)],
            },
        );

        let (graph, _) = SvfgBuilder::new(&ir).build();
        let stats = graph.stats();

        println!("Stats test SVFG: {}", stats);

        assert!(stats.node_count > 0, "Graph must have nodes");
        assert!(stats.edge_count > 0, "Graph must have edges");
        // We defined: mem_0(param), v1(def), mem_1(def), v2(def), v3(def), mem_2(def)
        assert!(
            stats.param_count >= 1,
            "At least the initial memory state param"
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: BinaryOp propagates taint to destination
    //
    // IR:  v1 = getSource()
    //      v2 = v1 + 1
    //      db.execute(v2)
    // Expected: 1 alert
    // -----------------------------------------------------------------------
    #[test]
    fn test_svfg_binary_op_taint_propagation() {
        let mut ir = FunctionIR::new("test_binop".into());
        let b = ir.entry_block;
        let mem_0 = ir.initial_memory_state;

        let v1 = ir.new_var(dummy_meta("v1"));
        let mem_1 = ir.new_var(mem_meta("mem_1"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: Some(v1),
                mem_out: mem_1,
                mem_in: mem_0,
                func: "getSource".into(),
                args: vec![],
            },
        );

        let v2 = ir.new_var(dummy_meta("v2"));
        ir.push_instruction(
            b,
            Instruction::BinaryOp {
                dest: v2,
                op: "concat".into(),
                lhs: Operand::Var(v1),
                rhs: Operand::StringLiteral(" suffix".into()),
            },
        );

        let mem_2 = ir.new_var(mem_meta("mem_2"));
        ir.push_instruction(
            b,
            Instruction::CallStatic {
                dest: None,
                mem_out: mem_2,
                mem_in: mem_1,
                func: "db.execute".into(),
                args: vec![Operand::Var(v2)],
            },
        );

        let heap = PointsToAnalysis::new();
        let config = default_config_with(&["getSource"], &["db.execute"], &[]);
        let (graph, def_site) = SvfgBuilder::new(&ir).build();

        let mut engine = SvfgTaintEngine::new(&ir, &heap, &config, &graph, &def_site);
        engine.run();

        assert_eq!(
            engine.alerts.len(),
            1,
            "BinaryOp should propagate taint. Got: {:?}",
            engine.alerts
        );
    }
}
