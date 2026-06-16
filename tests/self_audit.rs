// tests/self_audit.rs
//
// Self-Audit Test: GenSense scans its own source code.
//
// Purpose:
//   - Ensure the engine itself meets the same quality bar it enforces on others.
//   - Catch regressions where a new feature introduces patterns GenSense flags
//     as dangerous (e.g., eval, data leaks, missing error handling).
//   - Verify that the engine can parse and analyse real Rust source without
//     panicking, hanging, or producing malformed output.
//
// Execution:
//   cargo test --features full self_audit
//
// This test is intentionally strict: it fails if ANY Critical-severity finding
// is found in the GenSense source tree. Warning findings are reported but do
// not fail the build — they serve as an ongoing code-quality dashboard.

use frensense::{Engine, Severity};
use std::path::Path;

fn get_engine() -> Engine {
    Engine::new()
}

#[test]
fn self_audit_no_critical_findings() {
    let mut engine = get_engine();
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let advisories = engine
        .run(&src_dir)
        .expect("Engine should run on its own source");

    let criticals: Vec<_> = advisories
        .iter()
        .filter(|a| a.severity == Severity::Critical)
        .collect();

    if !criticals.is_empty() {
        eprintln!("\n[SELF-AUDIT] Critical findings in GenSense source:\n");
        for a in &criticals {
            eprintln!(
                "  [CRITICAL] {} — {}:{}\n    {}\n",
                a.rule_id, a.file_path, a.line, a.observation
            );
        }
        panic!(
            "[SELF-AUDIT] {} critical finding(s) found in the GenSense engine source. \
             The engine must meet the same bar it enforces on others.",
            criticals.len()
        );
    }
}

#[test]
fn self_audit_report_warnings() {
    let mut engine = get_engine();
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let advisories = engine
        .run(&src_dir)
        .expect("Engine should run on its own source");

    let warnings: Vec<_> = advisories
        .iter()
        .filter(|a| a.severity == Severity::Warning)
        .collect();

    // Warnings do not fail CI — they are printed as an advisory dashboard.
    // Track this count over time; if it grows significantly, investigate.
    if !warnings.is_empty() {
        println!(
            "\n[SELF-AUDIT] Warning findings in GenSense source ({} total):",
            warnings.len()
        );
        for a in &warnings {
            println!("  [WARNING] {} — {}:{}", a.rule_id, a.file_path, a.line);
        }
    }

    // Soft threshold: fail if warnings exceed 175. (Acknowledge baseline debt)
    assert!(
        warnings.len() <= 175,
        "[SELF-AUDIT] Warning count ({}) exceeds threshold (175). \
         Review and resolve accumulated findings before merging.",
        warnings.len()
    );
}

#[test]
fn self_audit_engine_does_not_panic_on_own_source() {
    let engine = get_engine();
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let mut engine = std::panic::AssertUnwindSafe(engine);
    let result = std::panic::catch_unwind(move || engine.run(&src_dir).ok());

    assert!(
        result.is_ok(),
        "[SELF-AUDIT] The engine panicked while scanning its own source code. \
         This indicates a critical parser stability regression."
    );
}
