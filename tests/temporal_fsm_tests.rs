use gensense::{GenSenseAuditor, GenSenseContext, GenSenseRule};
use std::path::Path;
use tree_sitter::Parser;

#[test]
fn test_temporal_must_follow_violation() {
    let content = r#"
        fn main() {
            let lock = mutex.lock();
            // Missing unlock
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();

    let mut registry = gensense::semantics::SymbolRegistry::new();
    let auditor = GenSenseAuditor::default_auditor();
    let path = Path::new("test.rs");

    let syms = auditor.discover_symbols(path, content).unwrap();
    for sym in syms {
        registry.insert(sym);
    }
    auditor
        .discover_events(path, content, &mut registry)
        .unwrap();

    let context = GenSenseContext {
        file_path: path,
        source_code: content,
        symbols: &registry,
    };

    let analyzer = gensense::semantics::temporal::TemporalAnalyzer::new(&context);

    struct FakeRule;
    impl GenSenseRule for FakeRule {
        fn id(&self) -> &str {
            "TEST"
        }
        fn description(&self) -> &str {
            "test"
        }
        fn check(&self, _n: tree_sitter::Node, _c: &GenSenseContext) -> Vec<gensense::Advisory> {
            vec![]
        }
    }

    let advisories = analyzer.check_temporal(
        tree.root_node(),
        &["lock".to_string(), "unlock".to_string()],
        &gensense::rules::ir::TemporalBehavior::MustFollow,
        &FakeRule,
    );

    assert!(!advisories.is_empty(), "Should detect missing 'unlock'");
    assert!(advisories[0].observation.contains("incomplete"));
}

#[test]
fn test_temporal_forbidden_between_violation() {
    let content = r#"
        fn main() {
            let lock = mutex.lock();
            unsafe_operation();
            let _ = mutex.unlock();
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();

    let mut registry = gensense::semantics::SymbolRegistry::new();
    let auditor = GenSenseAuditor::default_auditor();
    let path = Path::new("test.rs");

    let syms = auditor.discover_symbols(path, content).unwrap();
    for sym in syms {
        registry.insert(sym);
    }
    auditor
        .discover_events(path, content, &mut registry)
        .unwrap();

    let context = GenSenseContext {
        file_path: path,
        source_code: content,
        symbols: &registry,
    };

    let analyzer = gensense::semantics::temporal::TemporalAnalyzer::new(&context);

    struct FakeRule;
    impl GenSenseRule for FakeRule {
        fn id(&self) -> &str {
            "TEST"
        }
        fn description(&self) -> &str {
            "test"
        }
        fn check(&self, _n: tree_sitter::Node, _c: &GenSenseContext) -> Vec<gensense::Advisory> {
            vec![]
        }
    }

    let advisories = analyzer.check_temporal(
        tree.root_node(),
        &["unsafe_operation".to_string()],
        &gensense::rules::ir::TemporalBehavior::ForbiddenBetween(
            "lock".to_string(),
            "unlock".to_string(),
        ),
        &FakeRule,
    );

    assert!(
        !advisories.is_empty(),
        "Should detect 'unsafe_operation' between lock/unlock"
    );
    assert!(advisories[0].observation.contains("forbidden"));
}
