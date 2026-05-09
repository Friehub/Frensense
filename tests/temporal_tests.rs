use gensense::rules::ir::TemporalBehavior;
use gensense::semantics::temporal::TemporalAnalyzer;
use gensense::GenSenseContext;
use std::path::Path;
use tree_sitter::Parser;

#[test]
fn test_temporal_deadlock_detection() {
    let content = r#"
        async fn dangerous_op() {
            let _lock = my_mutex.lock();
            do_something().await;
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();

    let mut registry = gensense::semantics::SymbolRegistry::new();
    let auditor = gensense::GenSenseAuditor::new(vec![]);
    let file_path = Path::new("test.rs");

    // Pass 1: Symbol Discovery
    let syms = auditor.discover_symbols(file_path, content).unwrap();
    for s in syms {
        registry.insert(s);
    }

    // Pass 2: Event Discovery (this populates the graph)
    auditor
        .discover_events(file_path, content, &mut registry)
        .unwrap();

    let context = GenSenseContext {
        file_path,
        source_code: content,
        symbols: &registry,
    };

    let analyzer = TemporalAnalyzer::new(&context);
    let sequence = vec!["lock".to_string(), r"\.await".to_string()];

    // We need a mock rule to get advisories
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
    // Ensure we actually got the function_item
    let mut scope_node = function_node;
    while scope_node.kind() != "function_item" && scope_node.parent().is_some() {
        scope_node = scope_node.parent().unwrap();
    }

    let advisories = analyzer.check_temporal(
        scope_node,
        &sequence,
        &TemporalBehavior::MustNotFollow,
        &MockRule,
    );

    assert!(
        !advisories.is_empty(),
        "Should have detected a temporal violation"
    );
    assert!(
        advisories[0].observation.contains("must NOT follow"),
        "Error message should mention the violation"
    );
}
