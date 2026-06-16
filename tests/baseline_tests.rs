// SPDX-License-Identifier: MIT

use std::fs;
use tempfile::tempdir;

#[test]
fn test_sri_line_drift_resilience() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Taint flow: input() -> code -> eval(code)
    let ts_file = root.join("test.ts");
    let initial_content = "function handler() {\n    var code = input();\n    eval(code);\n}\n";
    fs::write(&ts_file, initial_content).unwrap();

    let mut engine = frensense::engine::project::Engine::new();
    let advisories = engine.run(root).unwrap();

    let taint_adv = advisories
        .iter()
        .find(|a| a.rule_id == "TAINT_INPUT_TO_EXEC");
    assert!(
        taint_adv.is_some(),
        "TAINT_INPUT_TO_EXEC must fire on eval(code) where code comes from input(). Got {} advisories",
        advisories.len()
    );
    let baseline_advisory = taint_adv.unwrap();
    let baseline_fuzzy = baseline_advisory.fuzzy_identity();

    // Shift the code down with comments
    let drifted_content = "// Comment\n// Comment\nfunction handler() {\n    var code = input();\n    eval(code);\n}\n";
    fs::write(&ts_file, drifted_content).unwrap();

    let new_advisories = engine.run(root).unwrap();
    let drifted_adv = new_advisories
        .iter()
        .find(|a| a.rule_id == "TAINT_INPUT_TO_EXEC")
        .unwrap();
    assert_eq!(
        drifted_adv.line, 5,
        "Violation should have shifted to line 5"
    );

    let drifted_fuzzy = drifted_adv.fuzzy_identity();
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
    fs::write(
        &ts_file,
        "function handler() { var code = input(); eval(code); }",
    )
    .unwrap();

    let mut engine = frensense::engine::project::Engine::new();
    let initial_advisories = engine.run(root).unwrap();
    let initial_adv = initial_advisories
        .iter()
        .find(|a| a.rule_id == "TAINT_INPUT_TO_EXEC")
        .unwrap();
    let initial_fuzzy = initial_adv.fuzzy_identity();

    // Change surrounding code but keep the sink call the same
    fs::write(
        &ts_file,
        "function handler() { var y = 1; var code = input(); eval(code); }",
    )
    .unwrap();

    let new_advisories = engine.run(root).unwrap();
    let new_adv = new_advisories
        .iter()
        .find(|a| a.rule_id == "TAINT_INPUT_TO_EXEC")
        .unwrap();
    let new_fuzzy = new_adv.fuzzy_identity();

    assert_eq!(
        initial_fuzzy, new_fuzzy,
        "Fuzzy identity should match if the flagged node content is the same"
    );
}
