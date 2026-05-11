// SPDX-License-Identifier: MIT

use gensense::engine::auditor::GenSenseAuditor;
use gensense::semantics::SymbolRegistry;
use gensense::{Advisory, FileId, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::Parser;

#[test]
fn test_temporal_consistency_rust_deadlock() {
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
    let mut registry = SymbolRegistry::new();
    let auditor = GenSenseAuditor::new(vec![]);
    let path = Path::new("test.rs");

    let syms = auditor.discover_symbols(path, content).unwrap();
    for s in syms {
        registry.insert(s);
    }
    auditor
        .discover_events(path, content, &mut registry)
        .unwrap();

    let sc = RefCell::new(HashMap::new());
    let tc = RefCell::new(HashMap::new());

    let context = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        symbols: &registry,
        semantic_cache: &sc,
        taint_cache: &tc,
    };

    let ast_analyzer = gensense::semantics::temporal::TemporalAnalyzer::new(&context);
    let sequence = vec!["lock".to_string(), r"\.await".to_string()];

    struct MockRule {
        metadata: RuleMetadata,
    }
    impl GenSenseRule for MockRule {
        fn metadata(&self) -> &RuleMetadata {
            &self.metadata
        }
        fn check(&self, _n: tree_sitter::Node, _c: &GenSenseContext) -> Vec<Advisory> {
            vec![]
        }
        fn applies_to(&self, _ext: &str) -> bool {
            true
        }
    }

    let rule = MockRule {
        metadata: RuleMetadata {
            id: "DEADLOCK".into(),
            name: "Deadlock".into(),
            severity: Severity::Critical,
            impact: "Deadlock".into(),
            improvement: "Unlock before await".into(),
            tags: vec![],
            category: "Security".into(),
        },
    };

    let function_node = tree.root_node().child(0).unwrap();

    let advisories_ast = ast_analyzer.check_temporal(
        function_node,
        &sequence,
        &gensense::rules::ir::TemporalBehavior::MustNotFollow,
        &rule,
    );

    let mut advisories_graph = Vec::new();
    if let Some(sym) = registry.find_at("dangerous_op", "test.rs", 2) {
        let idx = registry
            .graph
            .find_node(&sym.name, &sym.file_path, sym.line)
            .unwrap();
        let events = registry.graph.ordered_events_in_scope(idx);
        let mut has_lock = false;
        for ev in events {
            if ev.label == "lock" {
                has_lock = true;
            }
            if has_lock && ev.label == ".await" {
                advisories_graph.push(rule.new_advisory(
                    &function_node,
                    &context,
                    "Graph-detected deadlock".to_string(),
                ));
                break;
            }
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
}
