// SPDX-License-Identifier: MIT

use gensense::engine::auditor::GenSenseAuditor;
use gensense::semantics::SymbolRegistry;
use gensense::semantics::data_flow::{TaintOrigin, TaintRegistry};
use gensense::{FileId, GenSenseContext, TaintCache};
use std::collections::HashMap;
use std::path::Path;

// Test path interning address stability
#[test]
fn test_path_interning_stability() {
    let path = Path::new("main.rs");
    let content = "fn main() { let x = 1; }";
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

    let mut file_trees = HashMap::new();
    file_trees.insert(
        "main.rs".to_string(),
        (tree.clone(), content.to_string(), ops.clone()),
    );

    let context = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        tree: &tree,
        symbols: &registry,
        semantic_ops: &ops,
        taint_cache: &taint_cache,
        file_trees: &file_trees,
    };

    let analyzer =
        gensense::semantics::data_flow::DataFlowAnalyzer::new(&context, tree.root_node());
    let taint_reg = TaintRegistry::default();

    // Resolve definition in a different file multiple times
    let res1 = analyzer.find_definition("main", &taint_reg);
    let res2 = analyzer.find_definition("main", &taint_reg);

    if let (Some((_, _, _, _, p1, _)), Some((_, _, _, _, p2, _))) = (res1, res2) {
        // Assert that the returned reference addresses are physically identical!
        assert_eq!(p1 as *const Path, p2 as *const Path);
    }
}

// Test parameter destructuring and parameter mapping
#[test]
fn test_parameter_destructuring_taint() {
    let content = r"
        function process({ body, headers }) {
            sink(body);
        }
    ";
    let path = Path::new("test.js");
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

    let context = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        tree: &tree,
        symbols: &registry,
        semantic_ops: &ops,
        taint_cache: &taint_cache,
        file_trees: &HashMap::new(),
    };

    let analyzer =
        gensense::semantics::data_flow::DataFlowAnalyzer::new(&context, tree.root_node());

    // Find the actual function definition node dynamically
    let mut def_node = None;
    let mut cursor = tree.root_node().walk();
    for child in tree.root_node().children(&mut cursor) {
        if child.kind() == "function_declaration" || child.kind() == "function" {
            def_node = Some(child);
            break;
        }
    }
    let def_node = def_node.unwrap_or_else(|| {
        panic!("Could not find function_declaration node");
    });

    // Map tainted arguments to destructured params
    let tainted_args = vec![(0, TaintOrigin::UserInput)];
    let mapped = analyzer
        .map_params(def_node, content, &tainted_args)
        .unwrap();

    // Verify that BOTH destructured properties are now correctly registered and tainted!
    assert_eq!(mapped.get_origin("body"), Some(TaintOrigin::UserInput));
    assert_eq!(mapped.get_origin("headers"), Some(TaintOrigin::UserInput));
}

// Test method chain taint propagation
#[test]
fn test_method_chain_taint_propagation() {
    let content = r"
        let raw = get_input();
        let cleaned = raw.trim().toLowerCase();
        sink(cleaned);
    ";
    let path = Path::new("test.js");
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

    let context = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        tree: &tree,
        symbols: &registry,
        semantic_ops: &ops,
        taint_cache: &taint_cache,
        file_trees: &HashMap::new(),
    };

    let analyzer =
        gensense::semantics::data_flow::DataFlowAnalyzer::new(&context, tree.root_node());
    let mut taint_reg = TaintRegistry::default();
    taint_reg.taint("raw", TaintOrigin::UserInput);

    let source_re = regex::Regex::new("get_input").unwrap();
    let sink_re = regex::Regex::new("sink").unwrap();

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
            observation: "Dummy".into(),
            impact: "None".into(),
            improvement: "None".into(),
            tags: vec![],
            category: "Test".into(),
            confidence: 0.5,
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
        "Method chains must successfully propagate taint from receiver to sink"
    );
}

// Test return value taint propagation
#[test]
fn test_return_value_taint_propagation() {
    let content = r"
        function identity(input) {
            return input;
        }
        let secret = get_input();
        let value = identity(secret);
        sink(value);
    ";
    let path = Path::new("test.js");
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

    let mut file_trees = HashMap::new();
    file_trees.insert(
        "test.js".to_string(),
        (tree.clone(), content.to_string(), ops.clone()),
    );

    let context = GenSenseContext {
        file_id: FileId(1),
        file_path: path,
        source_code: content,
        tree: &tree,
        symbols: &registry,
        semantic_ops: &ops,
        taint_cache: &taint_cache,
        file_trees: &file_trees,
    };

    let analyzer =
        gensense::semantics::data_flow::DataFlowAnalyzer::new(&context, tree.root_node());
    let mut taint_reg = TaintRegistry::default();
    taint_reg.taint("secret", TaintOrigin::UserInput);

    let source_re = regex::Regex::new("get_input").unwrap();
    let sink_re = regex::Regex::new("sink").unwrap();

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
            observation: "Dummy".into(),
            impact: "None".into(),
            improvement: "None".into(),
            tags: vec![],
            category: "Test".into(),
            confidence: 0.5,
        },
    };

    analyzer.discover_symbols(&mut taint_reg);

    let advisories = analyzer.analyze_block(
        tree.root_node(),
        &source_re,
        &sink_re,
        &rule,
        &mut taint_reg,
    );
    assert!(
        !advisories.is_empty(),
        "Function return values must correctly propagate taint from internal returns to sink"
    );
}
