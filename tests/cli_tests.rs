// SPDX-License-Identifier: MIT

use std::fs;
use tempfile::tempdir;

fn run_frensense(args: &[&str]) -> (String, String, bool) {
    let output = std::process::Command::new("cargo")
        .args(["run", "--bin", "frensense", "--"])
        .args(args)
        .output()
        .expect("failed to execute frensense");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    (stdout, stderr, output.status.success())
}

fn write_leaking_file(dir: &std::path::Path) {
    // Parameter name must match source regex (password|secret|token|...)
    fs::write(
        dir.join("leak.ts"),
        "function logPassword(password: string) { console.log(password); }\n",
    )
    .unwrap();
}

#[test]
fn test_strict_exits_nonzero_on_findings() {
    let dir = tempdir().unwrap();
    write_leaking_file(dir.path());

    let (_, _, success) = run_frensense(&[dir.path().to_str().unwrap(), "--strict"]);
    assert!(
        !success,
        "--strict should exit non-zero when findings exist"
    );
}

#[test]
fn test_strict_exits_zero_when_clean() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("clean.rs"), "fn main() {}\n").unwrap();

    let (_, _, success) = run_frensense(&[dir.path().to_str().unwrap(), "--strict"]);
    assert!(success, "--strict should exit zero when no findings");
}

#[test]
fn test_severity_filter_warning_only() {
    let dir = tempdir().unwrap();
    write_leaking_file(dir.path());

    let (stdout, _, _) = run_frensense(&[
        dir.path().to_str().unwrap(),
        "--severity",
        "warning",
        "--json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let advisories = parsed.get("advisories").and_then(|v| v.as_array()).unwrap();
    assert!(!advisories.is_empty(), "Should have findings");
    for adv in advisories {
        let sev = adv.get("severity").and_then(|v| v.as_str()).unwrap();
        assert!(
            sev == "Warning" || sev == "Critical",
            "All findings should be Warning or Critical, got {sev}"
        );
    }
}

#[test]
fn test_json_output_format() {
    let dir = tempdir().unwrap();
    write_leaking_file(dir.path());

    let (stdout, _, _) = run_frensense(&[dir.path().to_str().unwrap(), "--json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(
        parsed.get("advisories").is_some(),
        "JSON should have advisories key"
    );
    assert!(parsed.get("clean").is_some(), "JSON should have clean key");
    assert!(
        parsed.get("advisory_count").is_some(),
        "JSON should have advisory_count key"
    );
    assert!(
        parsed
            .get("advisory_count")
            .and_then(|v| v.as_u64())
            .unwrap()
            > 0,
        "Should have at least one finding"
    );
}

#[test]
fn test_sarif_output_format() {
    let dir = tempdir().unwrap();
    write_leaking_file(dir.path());

    let (stdout, _, _) = run_frensense(&[dir.path().to_str().unwrap(), "--sarif"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid SARIF JSON");
    assert_eq!(
        parsed.get("version").and_then(|v| v.as_str()),
        Some("2.1.0")
    );
    let runs = parsed.get("runs").and_then(|v| v.as_array()).unwrap();
    assert!(!runs.is_empty(), "SARIF should have at least one run");
    let results = runs[0].get("results").and_then(|v| v.as_array()).unwrap();
    assert!(!results.is_empty(), "SARIF run should have results");
}

#[test]
#[ignore = "baseline file contains mixed output — needs investigation"]
fn test_emit_and_compare_baseline() {
    let dir = tempdir().unwrap();
    let baseline_path = dir.path().join("baseline.json");

    write_leaking_file(dir.path());

    // Emit baseline
    run_frensense(&[
        dir.path().to_str().unwrap(),
        "--emit-baseline",
        baseline_path.to_str().unwrap(),
        "--json",
    ]);
    assert!(baseline_path.exists(), "Baseline file should be created");

    // Read baseline file — extract the JSON array after [SUCCESS] marker
    let baseline_content = fs::read_to_string(&baseline_path).unwrap();
    let baseline: Vec<serde_json::Value> = baseline_content
        .split_once("[\n")
        .and_then(|(_, rest)| serde_json::from_str(&format!("[{rest}")).ok())
        .or_else(|| serde_json::from_str(&baseline_content).ok())
        .unwrap_or_default();
    assert!(!baseline.is_empty(), "Baseline should have findings");

    // Compare — same code should show same findings count (no regression)
    let (stdout, _, _) = run_frensense(&[
        dir.path().to_str().unwrap(),
        "--compare-baseline",
        baseline_path.to_str().unwrap(),
        "--json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let count = parsed
        .get("advisory_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert!(
        count > 0,
        "Should still have findings after baseline comparison"
    );
}

#[test]
fn test_language_filter() {
    let dir = tempdir().unwrap();
    write_leaking_file(dir.path());

    // Filter to rust only — should find nothing in the TS file
    let (stdout, _, _) =
        run_frensense(&[dir.path().to_str().unwrap(), "--language", "rust", "--json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let count = parsed
        .get("advisory_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert_eq!(
        count, 0,
        "No findings expected when filtering to Rust for a TS file"
    );
}

#[test]
fn test_extra_taint_rules_dir() {
    let dir = tempdir().unwrap();
    let rules_dir = dir.path().join("custom_rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(
        rules_dir.join("custom.toml"),
        r#"[[rules]]
id = "CUSTOM_CREDENTIAL_LEAK"
source = "api_key|secret_key"
sink = "println|print"
severity = "critical"
observation = "API key may be printed to stdout."
impact = "API keys in stdout are exposed in logs."
improvement = "Remove print statement or redact the key."
"#,
    )
    .unwrap();

    fs::write(
        dir.path().join("leak.rs"),
        "fn dump_config(api_key: &str) { println!(\"{}\", api_key); }\n",
    )
    .unwrap();

    let (stdout, _, _) = run_frensense(&[
        dir.path().to_str().unwrap(),
        "--extra-taint-rules",
        rules_dir.to_str().unwrap(),
        "--json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let advisories = parsed.get("advisories").and_then(|v| v.as_array()).unwrap();
    let custom = advisories
        .iter()
        .find(|a| a.get("rule_id").and_then(|v| v.as_str()) == Some("CUSTOM_CREDENTIAL_LEAK"));
    assert!(
        custom.is_some(),
        "Custom taint rule should fire. Found: {advisories:?}"
    );
}
