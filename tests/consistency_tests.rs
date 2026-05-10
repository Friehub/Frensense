#[test]
fn test_temporal_consistency_rust_deadlock() {
    use gensense::{GenSenseAuditor, GenSenseContext};
    use std::path::Path;
    use tree_sitter::Parser;

    let content = r#"
        async fn dangerous_op() {
            let _lock = my_mutex.lock();
            do_something().await;
        }
    "#;

    // Path B: AST -> TemporalAnalyzer
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();
    let mut registry = gensense::semantics::SymbolRegistry::new();
    let auditor = GenSenseAuditor::new(vec![]);

    // Build the graph (used by both paths)
    let syms = auditor
        .discover_symbols(Path::new("test.rs"), content)
        .unwrap();
    for s in syms {
        println!(
            "DEBUG: Discovered symbol: {} at {}:{}",
            s.name, s.file_path, s.line
        );
        registry.insert(s);
    }
    auditor
        .discover_events(Path::new("test.rs"), content, &mut registry)
        .unwrap();

    let context = GenSenseContext {
        file_path: Path::new("test.rs"),
        source_code: content,
        symbols: &registry,
    };

    let ast_analyzer = gensense::semantics::temporal::TemporalAnalyzer::new(&context);
    let sequence = vec!["lock".to_string(), r"\.await".to_string()];

    struct MockRule;
    impl gensense::GenSenseRule for MockRule {
        fn id(&self) -> &str {
            "DEADLOCK"
        }
        fn description(&self) -> &str {
            "Potential Deadlock"
        }
        fn severity(&self) -> gensense::Severity {
            gensense::Severity::Critical
        }
        fn impact(&self) -> &str {
            "Deadlock"
        }
        fn improvement(&self) -> &str {
            "Unlock before await"
        }
        fn check(
            &self,
            _node: tree_sitter::Node,
            _ctx: &GenSenseContext,
        ) -> Vec<gensense::Advisory> {
            vec![]
        }
    }

    // Find the function node
    let function_node = tree
        .root_node()
        .descendant_for_point_range(
            tree_sitter::Point::new(1, 0),
            tree_sitter::Point::new(1, 30),
        )
        .unwrap();
    let mut scope_node = function_node;
    while scope_node.kind() != "function_item" && scope_node.parent().is_some() {
        scope_node = scope_node.parent().unwrap();
    }
    println!(
        "DEBUG: scope_node kind: {}, line: {}",
        scope_node.kind(),
        scope_node.start_position().row + 1
    );

    let advisories_ast = ast_analyzer.check_temporal(
        scope_node,
        &sequence,
        &gensense::rules::ir::TemporalBehavior::MustNotFollow,
        &MockRule,
    );

    // Path A: Native Graph Check
    let advisories_graph = registry.check_graph_deadlock();

    println!("AST Advisories: {:?}", advisories_ast.len());
    println!("Graph Advisories: {:?}", advisories_graph.len());

    if advisories_ast.is_empty() {
        println!("DEBUG: AST path failed. Dumping graph events for scope...");
        // We need to find the scope_idx manually to debug
        let file_path = "test.rs";
        let line = scope_node.start_position().row + 1;
        if let Some(idx) = registry.find_function_at(file_path, line) {
            let events = registry.graph.ordered_events_in_scope(idx);
            println!("DEBUG: Events in scope: {:?}", events.len());
            for (i, ev) in events.iter().enumerate() {
                println!("  {}: {} ({:?})", i, ev.label, ev.event_type);
            }
        } else {
            println!("DEBUG: Could not find function in registry at {file_path}:{line}");
        }
    }

    assert!(
        !advisories_ast.is_empty(),
        "AST path should find the deadlock"
    );
    assert!(
        !advisories_graph.is_empty(),
        "Graph path should find the deadlock"
    );

    assert_eq!(advisories_ast[0].line, advisories_graph[0].line);
}
