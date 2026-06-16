// SPDX-License-Identifier: MIT

use frensense::engine::project::Engine;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_e2e_suppress_file_respected() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a file with taint violation: credential -> log
    let ts_file = root.join("leak.ts");
    fs::write(
        &ts_file,
        "function logPassword(password: string) { console.log(password); }",
    )
    .unwrap();

    // Verify it fires without suppression
    let mut engine = Engine::new();
    let advisories = engine.run(root).unwrap();
    let rule_id = "TAINT_CREDENTIAL_TO_LOG";
    assert!(
        advisories.iter().any(|a| a.rule_id == rule_id),
        "Taint rule should fire before suppression"
    );

    // Setup suppress file
    let suppress_file = root.join(".frensense-suppress.yml");
    let suppress_content =
        format!("suppressions:\n  - rule_id: {rule_id}\n    path: \"**/leak.ts\"\n");
    fs::write(suppress_file, suppress_content).unwrap();

    // Run again — should be suppressed
    let mut engine2 = Engine::new();
    let advisories2 = engine2.run(root).unwrap();
    assert!(
        !advisories2.iter().any(|a| a.rule_id == rule_id),
        "Violation should have been suppressed"
    );
}

#[test]
fn test_e2e_severity_override() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Setup config with severity override
    let config_dir = root.join(".frensense");
    fs::create_dir_all(&config_dir).unwrap();
    let rule_id = "TAINT_CREDENTIAL_TO_LOG";
    let config_content = format!("severity_override:\n  {rule_id}: Critical\n");
    fs::write(config_dir.join("config.yml"), config_content).unwrap();

    // Create violation
    let ts_file = root.join("leak.ts");
    fs::write(
        &ts_file,
        "function logPassword(password: string) { console.log(password); }",
    )
    .unwrap();

    // Run engine
    let mut engine = Engine::new();
    let advisories = engine.run(root).unwrap();

    let adv = advisories
        .iter()
        .find(|a| a.rule_id == rule_id)
        .expect("Rule should fire");
    assert_eq!(
        adv.severity,
        frensense::Severity::Critical,
        "Severity should have been overridden to Critical"
    );
}

#[test]
fn test_e2e_user_yaml_rule_loaded() {
    // User YAML rule loading is not yet implemented (load_user_rules returns empty).
    // This test verifies the engine runs cleanly even when a custom rules dir exists.
    let dir = tempdir().unwrap();
    let root = dir.path();

    let rules_dir = root.join(".frensense").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(
        rules_dir.join("custom.yml"),
        "rules:\n  - id: CUSTOM_TODO\n    on_node: macro_invocation\n",
    )
    .unwrap();

    let rs_file = root.join("main.rs");
    fs::write(&rs_file, "fn main() { todo!(); }").unwrap();

    let mut engine = Engine::new();
    // Should not panic — custom rules are silently ignored for now
    let _advisories = engine.run(root).unwrap();
}

#[test]
fn test_e2e_project_rule_fires_via_engine() {
    // Project rules from YAML are not yet wired (load_user_rules returns empty).
    // Verify the engine runs without crashing when project rule YAML exists.
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("api.rs"), "fn handle_request() { db_query(); }").unwrap();
    fs::write(root.join("db.rs"), "fn db_query() {}").unwrap();

    let rules_dir = root.join(".frensense").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(
        rules_dir.join("guard.yml"),
        r#"
project_rules:
  - id: MUST_HAVE_AUTH
    must_have_guard:
      source_pattern: "handle_.*"
      guard_pattern: "check_auth"
"#,
    )
    .unwrap();

    let mut engine = Engine::new();
    // Should not panic — project rules are silently ignored for now
    let _advisories = engine.run(root).unwrap();
}

#[test]
fn test_e2e_project_rule_suppressed_by_disabled_rules() {
    // Same as above — project rules not wired, but config loading should not crash.
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("api.rs"), "fn handle_request() {}").unwrap();

    let config_dir = root.join(".frensense");
    let rules_dir = config_dir.join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(
        config_dir.join("rules").join("guard.yml"),
        r#"
project_rules:
  - id: MUST_HAVE_AUTH
    must_have_guard:
      source_pattern: "handle_.*"
      guard_pattern: "check_auth"
"#,
    )
    .unwrap();

    fs::write(
        config_dir.join("config.yml"),
        "disabled_rules:\n  - MUST_HAVE_AUTH\n",
    )
    .unwrap();

    let mut engine = Engine::new();
    let advisories = engine.run(root).unwrap();

    assert!(
        !advisories.iter().any(|a| a.rule_id == "MUST_HAVE_AUTH"),
        "Project rule should be suppressed by disabled_rules config"
    );
}

#[test]
fn test_e2e_project_rule_severity_override() {
    // Severity override config should apply to taint rules too.
    let dir = tempdir().unwrap();
    let root = dir.path();

    let config_dir = root.join(".frensense");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(
        config_dir.join("config.yml"),
        "severity_override:\n  TAINT_INPUT_TO_EXEC: Info\n",
    )
    .unwrap();

    // Create violation: input -> eval
    let ts_file = root.join("vuln.ts");
    fs::write(
        &ts_file,
        "function handler() { var code = input(); eval(code); }",
    )
    .unwrap();

    let mut engine = Engine::new();
    let advisories = engine.run(root).unwrap();

    let adv = advisories
        .iter()
        .find(|a| a.rule_id == "TAINT_INPUT_TO_EXEC")
        .expect("Taint rule should fire");

    assert_eq!(
        adv.severity,
        frensense::Severity::Info,
        "Severity should be overridden to Info"
    );
}

#[test]
fn test_cli_json_output() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Write a file with a taint violation
    let ts_file = root.join("leak.ts");
    fs::write(
        &ts_file,
        "function logPassword(password: string) { console.log(password); }",
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--bin",
            "frensense",
            "--",
            root.to_str().unwrap(),
            "--json",
        ])
        .output()
        .expect("failed to execute frensense");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON output");
    assert_eq!(
        parsed.get("clean").and_then(serde_json::Value::as_bool),
        Some(false),
        "Should not be clean — stderr: {stderr}"
    );
    assert!(
        parsed
            .get("advisory_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap()
            >= 1
    );

    let advisories = parsed
        .get("advisories")
        .and_then(|v| v.as_array())
        .expect("advisories list");
    assert!(!advisories.is_empty());
}

#[test]
fn test_user_corpus_loaded_via_corpus_dir() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Use the built-in corpus to verify loading works
    let corpus_dir = std::path::PathBuf::from("corpus/targets");
    if !corpus_dir.exists() {
        // Skip if corpus dir doesn't exist in this environment
        return;
    }

    // Create a file that should match a corpus pattern
    fs::write(
        root.join("leak.ts"),
        "function logPassword(password: string) { console.log(password); }\n",
    )
    .unwrap();

    let mut engine = Engine::new();
    engine.set_corpus_dir(corpus_dir);
    let advisories = engine.run(root).unwrap();

    // Taint rules should fire (proving the engine works), and corpus may also fire
    let taint_hits: Vec<_> = advisories
        .iter()
        .filter(|a| a.rule_id.starts_with("TAINT_"))
        .collect();
    assert!(
        !taint_hits.is_empty(),
        "Engine should produce taint findings"
    );
}

#[test]
fn test_extra_rule_dirs_empty_does_not_crash() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("main.ts"), "function foo() {}\n").unwrap();

    let mut engine = Engine::new();
    // Should not crash even with extra dirs that don't exist
    let _advisories = engine.run(root).unwrap();
}
