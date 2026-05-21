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

// Test S-expression AST Query in rule compiler and engine
#[test]
fn test_sexpr_ast_query() {
    use gensense::engine::project::Engine;

    let yaml = r#"
rules:
  - id: "RUST_CSA_VALIDATE_UNCONDITIONAL_TEST"
    name: "Validator With No Rejection Path (Rust)"
    domain: "security"
    category: "Architecture"
    severity: Critical
    target_ext: "rs"
    on_node: '(function_item name: (identifier) @name (#match? @name ".*(validate|verify|check).*")) @node'
    body_must_contain: "return\\s+(false|None|Err)|panic!|Result::Err"
    body_may_delegate_via: "safeParse|validate|verify|check|assert"
    observation: "This validator function appears to have no rejection path."
    impact: "Validators that always succeed allow invalid data to propagate."
    improvement: "Ensure the function returns false, Option::None, or Result::Err for invalid input."
    tags: ["csa", "ai-risk", "security"]
    confidence: 0.85
"#;

    let mut engine = Engine::new();
    engine.set_isolate_rules(true);
    let rule_value: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
    let rules_list = rule_value["rules"].as_sequence().unwrap();
    let dsl_rule: gensense::rules::core::CoreRule =
        serde_yaml::from_value(rules_list[0].clone()).unwrap();
    let compiled_rule = gensense::rules::compiler::RuleCompiler::compile(dsl_rule).unwrap();
    engine.set_rules(vec![Box::new(compiled_rule)]);

    // Positive case: a validate function that returns nothing / has no rejection path
    let content_pos = r#"
        fn validate_user(user: &User) {
            let x = 1;
        }
    "#;
    let path_pos = Path::new("validate.rs");
    let advisories_pos = engine.run_content(path_pos, content_pos).unwrap();
    assert_eq!(
        advisories_pos.len(),
        1,
        "Should detect no rejection path in validate_user"
    );
    assert_eq!(
        advisories_pos[0].rule_id,
        "RUST_CSA_VALIDATE_UNCONDITIONAL_TEST"
    );

    // Negative case: a validate function that DOES return None / Err / panic
    let content_neg1 = r#"
        fn validate_user(user: &User) -> Result<(), Error> {
            if user.name.is_empty() {
                return Err(Error::Empty);
            }
            Ok(())
        }
    "#;
    let path_neg1 = Path::new("validate_safe.rs");
    let advisories_neg1 = engine.run_content(path_neg1, content_neg1).unwrap();
    assert_eq!(advisories_neg1.len(), 0, "Should pass since it returns Err");

    // Negative case: a function whose name does NOT match the validator prefix (e.g. main)
    let content_neg2 = r#"
        fn main() {
            let x = 1;
        }
    "#;
    let path_neg2 = Path::new("main.rs");
    let advisories_neg2 = engine.run_content(path_neg2, content_neg2).unwrap();
    assert_eq!(
        advisories_neg2.len(),
        0,
        "Should ignore main function because it's not a validator"
    );
}

#[test]
fn test_object_aliasing_field_taint_propagation() {
    let content = r"
        const pwd = input();
        const payload = { data: pwd };
        console.log(payload);
    ";
    let path = Path::new("test.ts");
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

    // Simulate: pwd is tainted by source_pattern, then payload = { data: pwd }
    // creates field-level taint payload.data, but payload itself is not directly tainted.
    taint_reg.taint("pwd", TaintOrigin::UserInput);
    taint_reg.taint_field("payload", "data", TaintOrigin::UserInput);

    let source_re = regex::Regex::new("input").unwrap();
    let sink_re = regex::Regex::new("console\\.log").unwrap();

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
            id: "TS_DATA_LEAK_TRACKER".into(),
            name: "Dummy".into(),
            severity: gensense::Severity::Warning,
            observation: "Dummy".into(),
            impact: "None".into(),
            improvement: "None".into(),
            tags: vec![],
            category: "Test".into(),
            confidence: 0.85,
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
        "Field-tainted object passed to sink must trigger advisory (payload.data -> payload -> console.log)"
    );
}
