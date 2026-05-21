// SPDX-License-Identifier: MIT

use std::fs;
use tempfile::tempdir;

#[test]
fn test_sri_line_drift_resilience() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Create a file with a violation
    let ts_file = root.join("test.ts");
    let initial_content = "function test_any() {\n    eval(\"1\");\n}\n";
    fs::write(&ts_file, initial_content).unwrap();

    // 2. Generate initial baseline
    let mut engine = gensense::engine::project::Engine::new();
    let advisories = engine.run(root).unwrap();

    println!("DEBUG: Advisories len={}", advisories.len());
    assert!(
        !advisories.is_empty(),
        "Should have found an 'eval' violation"
    );
    for a in &advisories {
        println!(
            "DEBUG: Advisory rule_id={}, enclosing_symbol={:?}",
            a.rule_id, a.enclosing_symbol
        );
    }
    let baseline_advisory = &advisories[0];
    assert_eq!(
        baseline_advisory.enclosing_symbol,
        Some("test_any".to_string())
    );
    assert_eq!(baseline_advisory.line, 2);

    let baseline_fuzzy = baseline_advisory.fuzzy_identity();

    // 3. Shift the code down with comments
    let drifted_content = "// Comment\n// Comment\nfunction test_any() {\n    eval(\"1\");\n}\n";
    fs::write(&ts_file, drifted_content).unwrap();

    // 4. Run engine again
    let new_advisories = engine.run(root).unwrap();
    assert!(!new_advisories.is_empty());
    let drifted_advisory = &new_advisories[0];
    assert_eq!(
        drifted_advisory.line, 4,
        "Violation should have shifted to line 4"
    );

    // 5. Verify fuzzy identity matches
    let drifted_fuzzy = drifted_advisory.fuzzy_identity();
    assert_eq!(
        baseline_fuzzy, drifted_fuzzy,
        "Fuzzy identity must be stable across line shifts"
    );
}

#[test]
fn test_sri_content_change_detection() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let ts_file = root.join("test.ts");
    fs::write(&ts_file, "function test() { eval(\"1\"); }").unwrap();

    let mut engine = gensense::engine::project::Engine::new();
    let initial_advisories = engine.run(root).unwrap();
    let initial_fuzzy = initial_advisories[0].fuzzy_identity();

    // Change surrounding code (e.g. adding a variable) but keep evaluation of "1" the same
    fs::write(&ts_file, "function test() { let y = 1; eval(\"1\"); }").unwrap();

    let new_advisories = engine.run(root).unwrap();
    let new_fuzzy = new_advisories[0].fuzzy_identity();

    assert_eq!(
        initial_fuzzy, new_fuzzy,
        "Fuzzy identity should match if the flagged node content ('eval(\"1\")') is the same"
    );
}
