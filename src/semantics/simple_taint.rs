// SPDX-License-Identifier: MIT

//! Simple AST-direct taint checker for consistency validation.
//!
//! This is Path B in the consistency check — a simplified analysis that
//! detects source→sink patterns without interprocedural tracking or
//! semantic graph traversal. Used to verify that the full pipeline
//! (Path A) doesn't lose findings.

use crate::Advisory;
use crate::FileId;
use regex::Regex;
use tree_sitter::Node;

/// Simple finding from AST-direct analysis.
#[derive(Debug)]
pub struct SimpleFinding {
    pub rule_id: String,
    pub line: u32,
    pub source_match: String,
    pub sink_match: String,
}

/// Run a simplified AST-direct taint check on source code.
///
/// This performs single-function analysis: within each function body,
/// it checks if any source pattern appears and any sink pattern appears.
/// No interprocedural tracking, no graph traversal.
pub fn simple_taint_check(
    source: &str,
    tree: &tree_sitter::Tree,
    source_re: &Regex,
    sink_re: &Regex,
    file_path: &std::path::Path,
    file_id: FileId,
) -> Vec<Advisory> {
    let mut findings = Vec::new();
    let root = tree.root_node();

    // Collect all function bodies
    let functions = collect_functions(root);

    for func in functions {
        let body = match func.child_by_field_name("body") {
            Some(b) => b,
            None => continue,
        };

        let body_text = &source[body.start_byte()..body.end_byte()];
        let body_line = body.start_position().row as u32 + 1;

        // Check for source patterns in function body
        let has_source = source_re.is_match(body_text);
        // Check for sink patterns in function body
        let has_sink = sink_re.is_match(body_text);

        // If both source and sink exist in the same function, emit finding
        if has_source && has_sink {
            let source_match = find_first_match(body_text, source_re);
            let sink_match = find_first_match(body_text, sink_re);

            // Determine rule ID based on the sink pattern
            let rule_id = infer_rule_id(&sink_match);

            findings.push(
                Advisory::bare(
                    &rule_id,
                    crate::Severity::Warning,
                    file_id,
                    file_path,
                    format!(
                        "Source '{}' and sink '{}' found in same function (AST-direct check).",
                        source_match, sink_match
                    ),
                )
                .with_line(body_line)
                .with_content(source_match.clone())
                .with_impact("Untrusted input may reach a dangerous sink.")
                .with_improvement("Validate or sanitize input before reaching sink.")
                .with_tags(["taint", "ast-direct"]),
            );
        }
    }

    findings
}

/// Collect all function/method nodes from the AST.
fn collect_functions(root: Node) -> Vec<Node> {
    let mut functions = Vec::new();
    let mut cursor = root.walk();

    loop {
        let node = cursor.node();
        let kind = node.kind();

        if matches!(
            kind,
            "function_item"
                | "function_declaration"
                | "method_definition"
                | "arrow_function"
                | "function"
        ) {
            functions.push(node);
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return functions;
            }
        }
    }
}

/// Find the first match of a regex in text.
fn find_first_match(text: &str, re: &Regex) -> String {
    re.find(text)
        .map(|m| m.as_str().to_string())
        .unwrap_or_default()
}

/// Infer a rule ID based on the sink pattern.
fn infer_rule_id(sink: &str) -> String {
    let lower = sink.to_lowercase();
    if lower.contains("eval") || lower.contains("exec") || lower.contains("system") {
        "TAINT_INPUT_TO_EXEC".to_string()
    } else if lower.contains("query") || lower.contains("execute") || lower.contains("sql") {
        "TAINT_INPUT_TO_SQL".to_string()
    } else if lower.contains("fetch") || lower.contains("http") || lower.contains("request") {
        "TAINT_INPUT_TO_HTTP".to_string()
    } else if lower.contains("write") || lower.contains("create") || lower.contains("open") {
        "TAINT_INPUT_TO_FS".to_string()
    } else if lower.contains("innerhtml") || lower.contains("outerhtml") || lower.contains("document.write") {
        "TAINT_INPUT_TO_DOM_XSS".to_string()
    } else if lower.contains("log") || lower.contains("print") || lower.contains("console") {
        "TAINT_CREDENTIAL_TO_LOG".to_string()
    } else {
        "TAINT_INPUT_TO_DANGEROUS_SINK".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_taint_check_detects_eval() {
        let source = "function handler(req) {\n    const input = req.body.query;\n    eval(input);\n}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();

        let source_re = Regex::new("input|body|param|query|request|user").unwrap();
        let sink_re = Regex::new("eval|exec|system").unwrap();

        let findings = simple_taint_check(
            source,
            &tree,
            &source_re,
            &sink_re,
            std::path::Path::new("test.ts"),
            FileId(0),
        );

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "TAINT_INPUT_TO_EXEC");
    }

    #[test]
    fn test_simple_taint_check_no_source() {
        let source = "function handler() {\n    eval(\"safe\");\n}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();

        let source_re = Regex::new("input|body|param|query").unwrap();
        let sink_re = Regex::new("eval|exec").unwrap();

        let findings = simple_taint_check(
            source,
            &tree,
            &source_re,
            &sink_re,
            std::path::Path::new("test.ts"),
            FileId(0),
        );

        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_simple_taint_check_no_sink() {
        let source = "function handler(input) {\n    console.log(input);\n}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();

        let source_re = Regex::new("input|body|param|query").unwrap();
        let sink_re = Regex::new("eval|exec").unwrap();

        let findings = simple_taint_check(
            source,
            &tree,
            &source_re,
            &sink_re,
            std::path::Path::new("test.ts"),
            FileId(0),
        );

        assert_eq!(findings.len(), 0);
    }
}
