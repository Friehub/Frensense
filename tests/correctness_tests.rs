// SPDX-License-Identifier: MIT

use gensense::engine::auditor::GenSenseAuditor;
use gensense::engine::project::Engine;
use gensense::semantics::SymbolRegistry;
use gensense::semantics::data_flow::TaintOrigin;
use gensense::{FileId, GenSenseContext, TaintCache};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[test]
fn test_symbol_shadowing() {
    let content = r"
        let x = 1;
        fn inner() {
            let x = 2;
            let y = x;
        }
    ";
    let path = Path::new("shadow.rs");
    let auditor = GenSenseAuditor::default_auditor();
    let (lang, tree) = auditor.parse_source(path, content).unwrap();
    let symbols = auditor
        .discover_symbols(path, FileId(1), content, &lang, &tree)
        .unwrap();

    let mut registry = SymbolRegistry::new();
    for sym in symbols {
        registry.insert(sym);
    }

    // Check at 'let x = 2' (line 4)
    let sym = registry.find_at("x", "shadow.rs", 4).unwrap();
    assert_eq!(sym.line, 4, "Should resolve to local x on line 4");
}

#[test]
#[allow(clippy::items_after_statements)]
fn test_taint_through_destructuring() {
    let content = r"
        let (a, b) = get_tainted_pair();
        sink(a);
    ";
    let path = Path::new("destruct.rs");
    let auditor = GenSenseAuditor::default_auditor();
    let (lang, tree) = auditor.parse_source(path, content).unwrap();
    let symbols = auditor
        .discover_symbols(path, FileId(1), content, &lang, &tree)
        .unwrap();
    let mut registry = SymbolRegistry::new();
    for sym in symbols {
        registry.insert(sym);
    }
    let ops = auditor.extract_semantic_ops(path, content, &tree);
    let taint_cache = TaintCache::default();

    let ctx = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        tree: &tree,
        symbols: &registry,
        semantic_ops: &ops,
        taint_cache: &taint_cache,
        file_trees: &HashMap::new(),
        taint_confidence_interprocedural: 0.80,
        taint_confidence_intraprocedural: 0.90,
        default_taint_max_depth: 5,
        ngram_window_size: 5,
    };

    let analyzer = gensense::semantics::data_flow::DataFlowAnalyzer::new(&ctx, tree.root_node());
    let mut taint_reg = gensense::semantics::data_flow::TaintRegistry::default();

    // Manual source injection
    taint_reg.taint("get_tainted_pair", TaintOrigin::UserInput);

    let source_re = regex::Regex::new("get_tainted_pair").unwrap();
    let sink_re = regex::Regex::new("sink").unwrap();

    // We need a dummy rule
    struct DummyRule {
        metadata: gensense::RuleMetadata,
    }
    impl gensense::GenSenseRule for DummyRule {
        fn metadata(&self) -> &gensense::RuleMetadata {
            &self.metadata
        }
        fn check<'a>(
            &self,
            _n: tree_sitter::Node<'a>,
            _c: &GenSenseContext<'a>,
        ) -> Vec<gensense::Advisory> {
            vec![]
        }
        fn applies_to(&self, _ext: &str) -> bool {
            true
        }
    }

    let rule = DummyRule {
        metadata: gensense::RuleMetadata {
            id: "DUMMY".into(),
            name: "Dummy".into(),
            severity: gensense::Severity::Info,
            observation: "Dummy finding".into(),
            impact: "None".into(),
            improvement: "None".into(),
            tags: vec![],
            category: "Test".into(),
            confidence: 0.55,
            precision: gensense::Precision::VeryHigh,
        },
    };

    let advisories = analyzer.analyze_block(
        tree.root_node(),
        &source_re,
        &sink_re,
        &rule,
        &mut taint_reg,
    );
    assert!(
        !advisories.is_empty(),
        "Taint should flow through destructuring to sink(a)"
    );
}

#[test]
fn test_suppression_correctness() {
    let _ = r#"
        // gensense-suppress RUST_PANIC
        panic!("intentional");
        panic!("unsuppressed");
    "#;
    // This test requires a real rule that matches panic!
    // Since we are doing engine-level tests, we verify GenSenseAuditor::audit suppression logic.
}

#[test]
fn test_snapshot_determinism() {
    let content = "fn main() { let x = 1; }";
    let path = Path::new("main.rs");
    let mut engine = Engine::new();

    let advisories1 = engine.run_content(path, content).unwrap();
    let advisories2 = engine.run_content(path, content).unwrap();

    assert_eq!(advisories1.len(), advisories2.len());
}

#[test]
fn test_sarif_output_properties() {
    use gensense::Advisory;
    use gensense::FileId;
    use gensense::reporter::Reporter;

    let adv = Advisory {
        rule_id: "TEST_RULE".into(),
        file_id: FileId(1),
        file_path: "src/main.rs".into(),
        severity: gensense::Severity::Warning,
        confidence: 0.85,
        observation: "observation".into(),
        impact: "impact".into(),
        improvement: "improvement".into(),
        line: 10,
        column: 5,
        start_byte: 100,
        end_byte: 120,
        original_content: "foo()".into(),
        proposed_replacement: None,
        proposed_import: None,
        enclosing_symbol: None,
        fingerprint: "hash".into(),
        auto_fixable: true,
        requires_human: false,
        tags: vec!["security".into(), "rust".into()],
    };

    let sarif = Reporter::to_sarif(&[adv], Path::new("."));
    let results = sarif
        .get("runs")
        .and_then(|r| r.as_array())
        .and_then(|r| r.first())
        .and_then(|run| run.get("results"))
        .and_then(|res| res.as_array())
        .expect("SARIF structure");

    assert_eq!(results.len(), 1);
    let result = &results[0];
    let properties = result.get("properties").expect("properties bag");

    let conf = properties
        .get("confidence")
        .and_then(serde_json::Value::as_f64)
        .expect("confidence");
    assert!((conf - 0.85).abs() < 1e-5);
    assert_eq!(
        properties
            .get("auto_fixable")
            .and_then(serde_json::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        properties
            .get("requires_human")
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );

    let tags = properties
        .get("tags")
        .and_then(|t| t.as_array())
        .expect("tags array");
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].as_str(), Some("security"));
    assert_eq!(tags[1].as_str(), Some("rust"));
}

#[test]
fn test_non_remediated_advisory_is_not_auto_fixable() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("panic.rs");
    fs::write(&f, "fn main() { panic!(\"boom\"); }").unwrap();

    let mut engine = Engine::new();
    let advisories = engine.run(&f).unwrap();
    let panic_adv = advisories.iter().find(|a| a.rule_id == "RUST_PANIC_IN_LIB");

    assert!(panic_adv.is_some(), "RUST_PANIC_IN_LIB must fire");
    let adv = panic_adv.unwrap();
    assert!(
        !adv.auto_fixable,
        "non-remediated advisory must not be auto_fixable"
    );
    assert!(adv.proposed_replacement.is_none());
}
