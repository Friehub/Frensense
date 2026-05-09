use std::fs;
use taas_auditor::{AstAuditor, Engine, Severity};
use tempfile::tempdir;

fn setup_engine() -> Engine {
    Engine::new(AstAuditor::default_auditor())
}

#[test]
fn test_false_positive_regression() {
    let mut engine = setup_engine();
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("clean.rs");

    fs::write(
        &file_path,
        r#"
        fn main() {
            let v = vec![1, 2, 3];
            for x in &v {
                println!("{}", x);
            }
        }

        #[tracing::instrument]
        async fn handle_request() -> Result<(), ()> {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            Ok(())
        }
    "#,
    )
    .unwrap();

    let advisories = engine.run(dir.path()).unwrap();

    // We expect 0 quality/security advisories.
    // (Ignoring institutional ones for now)
    let filtered: Vec<_> = advisories
        .into_iter()
        .filter(|a| !a.rule_id.starts_with("INSTITUTIONAL") && a.rule_id != "RUST_STD_OUTPUT")
        .collect();

    assert_eq!(
        filtered.len(),
        0,
        "Clean code should produce 0 quality advisories, but found: {filtered:?}"
    );
}

#[test]
fn test_rule_suppression() {
    let mut engine = setup_engine();
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("suppressed.rs");

    fs::write(
        &file_path,
        r#"
        fn main() {
            let v = vec![1, 2, 3];
            // taas-ignore: RUST_CLONE_IN_LOOP
            for x in v {
                let y = x.clone();
            }
        }
    "#,
    )
    .unwrap();

    let advisories = engine.run(dir.path()).unwrap();
    let filtered: Vec<_> = advisories
        .into_iter()
        .filter(|a| a.rule_id == "RUST_CLONE_IN_LOOP")
        .collect();

    assert_eq!(filtered.len(), 0, "Suppressed rule should not fire.");
}

#[test]
fn test_rule_isolation() {
    let mut engine = setup_engine();
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("bad.rs");

    fs::write(
        &file_path,
        r#"
        fn main() {
            let v = vec![1, 2, 3];
            for x in v {
                let y = x.clone(); // This should fire
            }
        }
    "#,
    )
    .unwrap();

    let advisories = engine.run(dir.path()).unwrap();
    let filtered: Vec<_> = advisories
        .into_iter()
        .filter(|a| a.rule_id == "RUST_CLONE_IN_LOOP")
        .collect();

    assert!(!filtered.is_empty(), "Clone in loop should be detected.");
    assert_snapshot("rule_isolation", &filtered);
}

#[test]
fn test_severity_tiers() {
    let mut engine = setup_engine();
    let dir = tempdir().unwrap();
    println!("DEBUG: temp dir path: {:?}", dir.path());
    let file_path = dir.path().join("secret_source.rs");

    fs::write(
        &file_path,
        r#"
        const SECRET = "0xabcdef1234567890abcdef1234567890abcdef1234567890";
    "#,
    )
    .unwrap();

    let advisories = engine.run(dir.path()).unwrap();
    let secret_adv = advisories
        .into_iter()
        .find(|a| a.rule_id == "SECRET_LEAK_DETECTION")
        .expect("Secret leak should be detected");

    // This should be Critical by default
    assert_eq!(secret_adv.severity, Severity::Critical);
}

#[test]
fn test_modular_features() {
    let mut engine = setup_engine();
    engine.enable_category("Security");

    // We can now verify that only Security rules fire
    // ...
}

/// INTEGRITY HELPER: Verifies that the audit findings match a known baseline.
/// Use this to prevent regression in diagnostic quality.
fn assert_snapshot(name: &str, advisories: &[taas_auditor::Advisory]) {
    let mut actual_ids: Vec<String> = advisories.iter().map(|a| a.rule_id.clone()).collect();
    actual_ids.sort();

    let snapshot_path = format!("tests/snapshots/{name}.json");

    if std::env::var("UPDATE_SNAPSHOTS").is_ok() {
        let json = serde_json::to_string_pretty(&actual_ids).unwrap();
        std::fs::write(&snapshot_path, json).unwrap();
        println!("✅ Updated snapshot: {snapshot_path}");
        return;
    }

    if !std::path::Path::new(&snapshot_path).exists() {
        panic!(
            "Snapshot {snapshot_path} does not exist. Run with UPDATE_SNAPSHOTS=1 to create it."
        );
    }

    let expected_json = std::fs::read_to_string(&snapshot_path).unwrap();
    let expected_ids: Vec<String> = serde_json::from_str(&expected_json).unwrap();

    assert_eq!(
        actual_ids, expected_ids,
        "Audit findings diverged from snapshot baseline '{name}'."
    );
}
