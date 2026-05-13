// SPDX-License-Identifier: MIT

use gensense::engine::auditor::GenSenseAuditor;
use gensense::engine::project::Engine;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_e2e_user_yaml_rule_loaded() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Setup .gensense/rules/custom.yml
    let rules_dir = root.join(".gensense").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();

    let custom_rule = r#"
rules:
  - id: CUSTOM_TODO
    name: Custom Todo
    domain: quality
    severity: Info
    target_ext: "rs"
    on_node: "macro_invocation"
    if_matches: "todo!"
    observation: "Found a todo!"
    impact: "Impact"
    improvement: "Improvement"
    category: "Test"
    tags: []
"#;
    fs::write(rules_dir.join("custom.yml"), custom_rule).unwrap();

    // 2. Create a file with violation
    let rs_file = root.join("main.rs");
    fs::write(&rs_file, "fn main() { todo!(); }").unwrap();

    // 3. Run engine
    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    let advisories = engine.run(root).unwrap();

    let has_custom = advisories.iter().any(|a| a.rule_id == "CUSTOM_TODO");
    assert!(
        has_custom,
        "Custom YAML rule should have been loaded and fired. Found: {advisories:?}"
    );
}

#[test]
fn test_e2e_suppress_file_respected() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Create violation
    let rs_file = root.join("main.rs");
    fs::write(&rs_file, "fn main() { panic!(\"error\"); }").unwrap();

    // 2. Verify it fires without suppression
    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    let advisories = engine.run(root).unwrap();
    assert!(advisories.iter().any(|a| a.rule_id == "RUST_PANIC_IN_LIB"));

    // 3. Setup .gensense-suppress.yml
    let suppress_file = root.join(".gensense-suppress.yml");
    let suppress_content = r#"
suppressions:
  - rule_id: RUST_PANIC_IN_LIB
    path: "**/main.rs"
"#;
    fs::write(suppress_file, suppress_content).unwrap();

    // 4. Run again
    let mut engine2 = Engine::new(GenSenseAuditor::default_auditor());
    let advisories2 = engine2.run(root).unwrap();
    let has_panic = advisories2.iter().any(|a| a.rule_id == "RUST_PANIC_IN_LIB");
    assert!(
        !has_panic,
        "Violation should have been suppressed by .gensense-suppress.yml"
    );
}

#[test]
fn test_e2e_severity_override() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Setup .gensense/config.yml
    let config_dir = root.join(".gensense");
    fs::create_dir_all(&config_dir).unwrap();

    let config_content = r#"
severity_override:
  RUST_PANIC_IN_LIB: Critical
"#;
    fs::write(config_dir.join("config.yml"), config_content).unwrap();

    // 2. Create violation
    let rs_file = root.join("main.rs");
    fs::write(&rs_file, "fn main() { panic!(\"error\"); }").unwrap();

    // 3. Run engine
    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    let advisories = engine.run(root).unwrap();

    let panic_adv = advisories
        .iter()
        .find(|a| a.rule_id == "RUST_PANIC_IN_LIB")
        .expect("Rule should fire");
    assert_eq!(
        panic_adv.severity,
        gensense::Severity::Critical,
        "Severity should have been overridden to Critical"
    );
}
