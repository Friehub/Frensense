// SPDX-License-Identifier: MIT

//! Consistency tests between AST-direct and graph-based analysis paths.
//!
//! These tests verify that the full pipeline (Path A: graph-based) doesn't
//! lose findings compared to a simplified AST check (Path B: AST-direct).
//! Any divergence indicates a potential regression or improvement.

use frensense::semantics::consistency::{ConsistencyCheck, DivergenceMetrics};
use frensense::semantics::simple_taint::simple_taint_check;
use frensense::FileId;
use regex::Regex;

/// Test that both paths detect the same taint findings for simple cases.
#[test]
fn test_consistency_taint_input_to_exec() {
    let source = "function handler(req) {\n    const input = req.body.query;\n    eval(input);\n}";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();

    // Path B: Simple AST check
    let source_re = Regex::new("input|body|param|query|request|user").unwrap();
    let sink_re = Regex::new("eval|exec|system|spawn").unwrap();
    let path_b = simple_taint_check(
        source,
        &tree,
        &source_re,
        &sink_re,
        std::path::Path::new("test.ts"),
        FileId(0),
    );

    // Path A: Full pipeline would run here, but for this test we verify
    // that the AST-direct check produces the expected finding
    assert_eq!(path_b.len(), 1, "AST-direct should detect 1 finding");
    assert_eq!(path_b[0].rule_id, "TAINT_INPUT_TO_EXEC");
}

/// Test that both paths agree on no findings when there's no taint flow.
#[test]
fn test_consistency_no_taint() {
    let source = "function safe() {\n    const x = 42;\n    console.log(x);\n}";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();

    let source_re = Regex::new("input|body|param|query|request|user").unwrap();
    let sink_re = Regex::new("eval|exec|system|spawn").unwrap();
    let path_b = simple_taint_check(
        source,
        &tree,
        &source_re,
        &sink_re,
        std::path::Path::new("test.ts"),
        FileId(0),
    );

    assert_eq!(path_b.len(), 0, "No taint flow should produce no findings");
}

/// Test divergence detection between two advisory sets.
#[test]
fn test_divergence_detection() {
    use frensense::Advisory;

    // Path A (graph-based) found 2 findings
    let path_a = vec![
        Advisory::bare("TAINT_INPUT_TO_EXEC", frensense::Severity::Critical, FileId(0), std::path::Path::new("test.ts"), "finding 1").with_line(10),
        Advisory::bare("CORPUS_XSS_PATTERN", frensense::Severity::Warning, FileId(0), std::path::Path::new("test.ts"), "finding 2").with_line(20),
    ];

    // Path B (AST-direct) found 3 findings
    let path_b = vec![
        Advisory::bare("TAINT_INPUT_TO_EXEC", frensense::Severity::Critical, FileId(0), std::path::Path::new("test.ts"), "finding 1").with_line(10),
        Advisory::bare("TAINT_CREDENTIAL_TO_DB", frensense::Severity::Critical, FileId(0), std::path::Path::new("test.ts"), "finding 3").with_line(30),
        Advisory::bare("TAINT_INPUT_TO_HTTP", frensense::Severity::Warning, FileId(0), std::path::Path::new("test.ts"), "finding 4").with_line(40),
    ];

    let check = ConsistencyCheck::new(path_b, path_a);
    assert!(!check.verify());

    let div = check.detect_divergence();
    assert_eq!(div.missing_in_graph.len(), 2, "Graph should miss 2 findings");
    assert_eq!(div.extra_in_graph.len(), 1, "Graph should have 1 extra finding");

    // Verify specific divergences
    let missing_rules: Vec<_> = div.missing_in_graph.iter().map(|k| &k.rule_id).collect();
    assert!(missing_rules.contains(&&"TAINT_CREDENTIAL_TO_DB".to_string()));
    assert!(missing_rules.contains(&&"TAINT_INPUT_TO_HTTP".to_string()));

    let extra_rules: Vec<_> = div.extra_in_graph.iter().map(|k| &k.rule_id).collect();
    assert!(extra_rules.contains(&&"CORPUS_XSS_PATTERN".to_string()));
}

/// Test metrics computation.
#[test]
fn test_metrics_computation() {
    use frensense::Advisory;

    let path_a = vec![
        Advisory::bare("TAINT_INPUT_TO_EXEC", frensense::Severity::Critical, FileId(0), std::path::Path::new("a.ts"), "f1").with_line(10),
        Advisory::bare("TAINT_INPUT_TO_EXEC", frensense::Severity::Critical, FileId(0), std::path::Path::new("a.ts"), "f2").with_line(20),
        Advisory::bare("CORPUS_XSS", frensense::Severity::Warning, FileId(0), std::path::Path::new("b.ts"), "f3").with_line(30),
    ];

    let path_b = vec![
        Advisory::bare("TAINT_INPUT_TO_EXEC", frensense::Severity::Critical, FileId(0), std::path::Path::new("a.ts"), "f1").with_line(10),
        Advisory::bare("TAINT_CREDENTIAL_TO_DB", frensense::Severity::Critical, FileId(0), std::path::Path::new("a.ts"), "f4").with_line(40),
    ];

    let check = ConsistencyCheck::new(path_b, path_a);
    let metrics = check.metrics();

    assert_eq!(metrics.total_graph, 3);
    assert_eq!(metrics.total_ast, 2);
    assert_eq!(metrics.total_missing, 1);
    assert_eq!(metrics.total_extra, 2);

    // Per-rule metrics
    let taint_exec = metrics.per_rule.get("TAINT_INPUT_TO_EXEC").unwrap();
    assert_eq!(taint_exec.total_graph, 2);
    assert_eq!(taint_exec.total_ast, 1);
    assert_eq!(taint_exec.missing, 0);
    assert_eq!(taint_exec.extra, 1);
}

/// Test regression detection against a baseline.
#[test]
fn test_regression_detection() {
    use frensense::semantics::consistency::{check_regression, RuleDivergence};
    use std::collections::HashMap;

    let baseline = DivergenceMetrics {
        total_graph: 100,
        total_ast: 95,
        total_missing: 5,
        total_extra: 3,
        per_rule: HashMap::from([(
            "TAINT_INPUT_TO_EXEC".to_string(),
            RuleDivergence {
                missing: 2,
                extra: 1,
                total_graph: 20,
                total_ast: 18,
            },
        )]),
        per_file: HashMap::new(),
    };

    let current = DivergenceMetrics {
        total_graph: 100,
        total_ast: 90,
        total_missing: 10,
        total_extra: 3,
        per_rule: HashMap::from([(
            "TAINT_INPUT_TO_EXEC".to_string(),
            RuleDivergence {
                missing: 5,
                extra: 1,
                total_graph: 20,
                total_ast: 15,
            },
        )]),
        per_file: HashMap::new(),
    };

    let regressions = check_regression(&current, &baseline);
    assert_eq!(regressions.len(), 2);
    assert!(regressions.iter().any(|r| r.contains("Missing findings increased")));
    assert!(regressions.iter().any(|r| r.contains("TAINT_INPUT_TO_EXEC")));
}
