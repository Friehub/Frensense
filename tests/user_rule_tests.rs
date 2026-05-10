// SPDX-License-Identifier: MIT

use gensense::{Engine, GenSenseAuditor};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_user_rule_loading() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let root = temp_dir.path();

    // Create the project structure
    let rules_dir = root.join(".gensense").join("rules");
    fs::create_dir_all(&rules_dir).expect("Failed to create rules dir");

    // Write a custom rule
    let rule_content = r#"
rules:
  - id: "TEST_CUSTOM_RULE"
    domain: "maintainability"
    target_ext: "rs"
    on_node: "macro_invocation"
    if_matches: "println!"
    observation: "Custom rule triggered."
    impact: "Testing user rules."
    improvement: "Fix it."
    severity: Critical
"#;
    fs::write(rules_dir.join("test_rule.yml"), rule_content).expect("Failed to write custom rule");

    // Write a source file that triggers the rule
    let src_dir = root.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src dir");
    fs::write(
        src_dir.join("main.rs"),
        "fn main() { println!(\"Hello\"); }",
    )
    .expect("Failed to write src file");

    let auditor = GenSenseAuditor::default_auditor();
    let mut engine = Engine::new(auditor);

    let advisories = engine.run(root).expect("Engine run failed");

    // Check if the custom rule was triggered
    let custom_finding = advisories.iter().find(|a| a.rule_id == "TEST_CUSTOM_RULE");
    assert!(
        custom_finding.is_some(),
        "Custom rule was not loaded or triggered"
    );
    let finding = custom_finding.unwrap();
    assert_eq!(finding.observation, "Custom rule triggered.");
    assert_eq!(finding.severity, gensense::Severity::Critical);
}

#[test]
fn test_user_rule_override() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let root = temp_dir.path();

    let rules_dir = root.join(".gensense").join("rules");
    fs::create_dir_all(&rules_dir).expect("Failed to create rules dir");

    // Write a custom rule that overrides an existing embedded rule.
    // Assuming RUST_STD_OUTPUT is embedded.
    let rule_content = r#"
rules:
  - id: "RUST_STD_OUTPUT"
    domain: "maintainability"
    target_ext: "rs"
    on_node: "macro_invocation"
    if_matches: "println!"
    observation: "OVERRIDDEN observation."
    impact: "OVERRIDDEN impact."
    improvement: "OVERRIDDEN improvement."
    severity: Critical
"#;
    fs::write(rules_dir.join("override_rule.yml"), rule_content)
        .expect("Failed to write override rule");

    fs::create_dir_all(root.join("src")).expect("Failed to create src dir");
    fs::write(
        root.join("src").join("main.rs"),
        "fn main() { println!(\"Hello\"); }",
    )
    .expect("Failed to write src file");

    let auditor = GenSenseAuditor::default_auditor();
    let mut engine = Engine::new(auditor);

    let advisories = engine.run(root).expect("Engine run failed");

    let findings: Vec<_> = advisories
        .iter()
        .filter(|a| a.rule_id == "RUST_STD_OUTPUT")
        .collect();
    assert_eq!(
        findings.len(),
        1,
        "Expected exactly one instance of the rule"
    );
    assert_eq!(findings[0].observation, "OVERRIDDEN observation.");
    assert_eq!(findings[0].severity, gensense::Severity::Critical);
}
