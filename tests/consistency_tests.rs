// SPDX-License-Identifier: MIT

use gensense::engine::auditor::GenSenseAuditor;
use gensense::semantics::SymbolRegistry;
use gensense::{Advisory, FileId, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::Parser;

#[allow(clippy::too_many_lines)]
#[test]
fn test_temporal_consistency_rust_deadlock() {
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

    let content = r"
        async fn dangerous_op() {
            let _lock = my_mutex.lock();
            do_something().await;
        }
    ";

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let _tree = parser.parse(content, None).unwrap();
    let mut registry = SymbolRegistry::new();
    let auditor = GenSenseAuditor::new(vec![]);
    let path = Path::new("test.rs");

    let (language, tree) = auditor.parse_source(path, content).unwrap();
    let syms = auditor
        .discover_symbols(path, FileId(1), content, &language, &tree)
        .unwrap();
    for s in syms {
        registry.insert(s);
    }
    auditor
        .discover_events(path, content, &tree, &mut registry)
        .unwrap();

    let tc = RefCell::new(HashMap::new());
    let ops = auditor.extract_semantic_ops(path, content, &tree);

    let ctx = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        tree: &tree,
        symbols: &registry,
        semantic_ops: &ops,
        taint_cache: &tc,
        file_trees: &HashMap::new(),
    };

    let ast_analyzer = gensense::semantics::temporal::TemporalAnalyzer::new(&ctx);
    let sequence = vec![
        regex::Regex::new("lock").unwrap(),
        regex::Regex::new(r"\.await").unwrap(),
    ];

    let rule = MockRule {
        metadata: RuleMetadata {
            id: "DEADLOCK".into(),
            name: "Deadlock".into(),
            severity: Severity::Critical,
            observation: "Potential deadlock detected".into(),
            impact: "Deadlock".into(),
            improvement: "Unlock before await".into(),
            tags: vec![],
            category: "Security".into(),
            confidence: 0.55,
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
            .graph()
            .find_node(&sym.name, &sym.file_path, sym.line)
            .unwrap();
        let events = registry.graph().ordered_events_in_scope(idx);
        let mut has_lock = false;
        for ev in events {
            if ev.label == "lock" {
                has_lock = true;
            }
            if has_lock && ev.label == ".await" {
                advisories_graph.push(rule.new_advisory(
                    &function_node,
                    &ctx,
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
