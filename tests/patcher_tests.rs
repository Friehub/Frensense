// SPDX-License-Identifier: MIT

use frensense::patcher::PatchManager;
use frensense::{Advisory, FileId, Severity};
use std::fs;

fn make_advisory(
    rule_id: &str,
    content: &str,
    replacement: &str,
    start_byte: u32,
    end_byte: u32,
    line: u32,
) -> Advisory {
    Advisory {
        rule_id: rule_id.to_string(),
        file_id: FileId(0),
        file_path: String::new(),
        severity: Severity::Warning,
        confidence: 0.8,
        observation: String::new(),
        impact: String::new(),
        improvement: String::new(),
        line,
        column: 0,
        start_byte,
        end_byte,
        original_content: content.to_string(),
        proposed_replacement: Some(replacement.to_string()),
        proposed_import: None,
        enclosing_symbol: None,
        fingerprint: String::new(),
        auto_fixable: true,
        requires_human: false,
        tags: vec![],
        taint_branch_ratio: None,
        matched_evidence: None,
    }
}

#[test]
fn test_apply_fix_removes_unused_variable() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.rs");
    let content = "fn main() {\n    let x = 1;\n    println!(\"hi\");\n}\n";
    fs::write(&file, content).unwrap();

    // "let x = 1;\n" starts at byte 12, ends at byte 23
    let advisory = make_advisory("UNUSED_VARIABLE", "    let x = 1;\n", "\n", 12, 27, 2);

    let patcher = PatchManager::new(dir.path());
    patcher.apply_fix(&advisory, &file).unwrap();

    let result = fs::read_to_string(&file).unwrap();
    assert!(
        !result.contains("let x = 1;"),
        "unused variable should be removed"
    );
    assert!(
        result.contains("println!(\"hi\");"),
        "other code should remain"
    );
}

#[test]
fn test_apply_fix_replaces_content() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.ts");
    let content = "function validate() {\n    return true;\n}\n";
    fs::write(&file, content).unwrap();

    // "    return true;" starts at byte 22, ends at byte 38
    let advisory = make_advisory(
        "TS_CSA_VALIDATE_UNCONDITIONAL",
        "    return true;",
        "    return input !== undefined && input !== null;",
        22,
        38,
        2,
    );

    let patcher = PatchManager::new(dir.path());
    patcher.apply_fix(&advisory, &file).unwrap();

    let result = fs::read_to_string(&file).unwrap();
    assert!(result.contains("return input !== undefined && input !== null;"));
    assert!(!result.contains("return true;"));
}

#[test]
fn test_apply_fixes_multiple_patches() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.rs");
    let content = "fn a() {\n    let x = 1;\n}\nfn b() {\n    let y = 2;\n}\n";
    fs::write(&file, content).unwrap();

    let a1 = make_advisory("UNUSED_VARIABLE", "    let x = 1;\n", "\n", 9, 24, 2);
    let a2 = make_advisory("UNUSED_VARIABLE", "    let y = 2;\n", "\n", 35, 50, 5);

    let patcher = PatchManager::new(dir.path());
    patcher.apply_fixes(&[&a1, &a2], &file).unwrap();

    let result = fs::read_to_string(&file).unwrap();
    assert!(!result.contains("let x = 1;"));
    assert!(!result.contains("let y = 2;"));
    assert!(result.contains("fn a()"));
    assert!(result.contains("fn b()"));
}

#[test]
fn test_context_mismatch_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.rs");
    fs::write(&file, "fn main() { let x = 1; }\n").unwrap();

    let advisory = make_advisory(
        "UNUSED_VARIABLE",
        "let z = 99;", // Wrong original content
        "\n",
        12,
        23,
        1,
    );

    let patcher = PatchManager::new(dir.path());
    let result = patcher.apply_fix(&advisory, &file);
    assert!(
        result.is_err(),
        "mismatched context should fail: {result:?}"
    );
}

#[test]
fn test_generate_diff_output() {
    let dir = tempfile::tempdir().unwrap();
    let advisory = make_advisory("TEST_RULE", "old line", "new line", 1, 9, 1);

    let patcher = PatchManager::new(dir.path());
    let diff = patcher
        .generate_diff(&advisory, std::path::Path::new("test.rs"))
        .unwrap();

    assert!(diff.contains("--- a/test.rs"));
    assert!(diff.contains("+++ b/test.rs"));
    assert!(diff.contains("-old line"));
    assert!(diff.contains("+new line"));
}

#[test]
fn test_apply_fix_empty_advisory_is_noop() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.rs");
    let original = "fn main() {}\n";
    fs::write(&file, original).unwrap();

    let patcher = PatchManager::new(dir.path());
    patcher.apply_fixes(&[], &file).unwrap();

    let result = fs::read_to_string(&file).unwrap();
    assert_eq!(result, original);
}
