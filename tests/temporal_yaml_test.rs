// SPDX-License-Identifier: MIT

#[test]
fn test_yaml_temporal_rule_must_not_follow() {
    use gensense::rules::compiler::RuleCompiler;
    use gensense::rules::core::CoreRule;
    use gensense::{GenSenseAuditor, GenSenseContext, GenSenseRule};
    use std::path::Path;

    let yaml = r#"
id: "RUST_MUTEX_AWAIT_YAML"
domain: "reliability"
target_ext: "rs"
on_node: "function_item"
observation: "Mutex held across await point (YAML Rule)"
impact: "Potential deadlock in async code"
improvement: "Ensure mutex is dropped before await"
severity: Critical
temporal:
  sequence: ["lock", "\\.await"]
  behavior: "must_not_follow"
"#;

    let dsl: CoreRule = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    let rule = RuleCompiler::compile(dsl);

    let content = r#"
        async fn bad_function() {
            let guard = mutex.lock();
            do_work().await;
        }

        async fn good_function() {
            let guard = mutex.lock();
            do_sync_work();
            drop(guard);
            do_async_work().await;
        }
    "#;

    let mut registry = gensense::semantics::SymbolRegistry::new();
    let auditor = GenSenseAuditor::new(vec![]);
    let path = Path::new("test.rs");

    // 1. Discovery
    let syms = auditor
        .discover_symbols(path, content)
        .expect("Symbol discovery failed");
    for s in syms {
        registry.insert(s);
    }
    auditor
        .discover_events(path, content, &mut registry)
        .expect("Event discovery failed");

    let context = GenSenseContext {
        file_path: path,
        source_code: content,
        symbols: &registry,
    };

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();

    // 2. Execution
    let mut cursor = tree.root_node().walk();
    let mut bad_found = false;
    let mut good_found = false;

    for node in tree.root_node().children(&mut cursor) {
        if node.kind() == "function_item" {
            let code = &content[node.start_byte()..node.end_byte()];
            let advisories = rule.check(node, &context);

            if code.contains("fn bad_function") {
                assert!(!advisories.is_empty(), "bad_function should be flagged");
                assert_eq!(advisories[0].rule_id, "RUST_MUTEX_AWAIT_YAML");
                bad_found = true;
            } else if code.contains("fn good_function") {
                assert!(advisories.is_empty(), "good_function should NOT be flagged");
                good_found = true;
            }
        }
    }

    assert!(bad_found, "bad_function node not found in AST");
    assert!(good_found, "good_function node not found in AST");
}
