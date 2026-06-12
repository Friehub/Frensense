// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use tree_sitter::Node;

use crate::data_flow::TaintRegistry;

#[derive(Debug, Clone, Default)]
pub struct TaintMetrics {
    pub tainted_uses: usize,
    pub taint_branched_on: usize,
    pub taint_branch_ratio: f32,
    pub has_validation_name: bool,
}

impl TaintMetrics {
    pub fn compute(
        registry: &TaintRegistry,
        root: Node,
        source: &str,
        function_name: &str,
    ) -> Self {
        let mut metrics = Self::default();
        metrics.has_validation_name = is_validation_name(function_name);

        let mut tainted_vars: HashSet<String> = HashSet::new();
        collect_tainted_vars(root, source, registry, &mut tainted_vars);

        metrics.tainted_uses = count_tainted_uses(root, source, &tainted_vars);

        let mut cursor = root.walk();
        loop {
            let node = cursor.node();
            let kind = node.kind();

            if is_conditional(kind) {
                if let Some(cond) = node.child_by_field_name("condition") {
                    let cond_text =
                        &source[cond.start_byte()..cond.end_byte()];
                    if tainted_vars.iter().any(|v| cond_text.contains(v.as_str())) {
                        metrics.taint_branched_on += 1;
                    }
                }
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    if metrics.tainted_uses > 0 {
                        metrics.taint_branch_ratio =
                            metrics.taint_branched_on as f32 / metrics.tainted_uses as f32;
                    }
                    return metrics;
                }
            }
        }
    }

    pub fn is_hollow_validator(&self) -> bool {
        self.has_validation_name && self.taint_branch_ratio < 0.2
    }
}

fn count_tainted_uses(node: Node, source: &str, tainted: &HashSet<String>) -> usize {
    let mut count = 0;
    let kind = node.kind();

    if kind == "identifier" {
        let name = &source[node.start_byte()..node.end_byte()];
        if tainted.contains(name) {
            count += 1;
        }
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            count += count_tainted_uses(cursor.node(), source, tainted);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    count
}

fn is_validation_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.starts_with("validate")
        || lower.starts_with("check_")
        || lower.starts_with("verify_")
        || lower.starts_with("ensure_")
        || lower.starts_with("sanitize_")
        || lower.starts_with("parse_")
        || lower.starts_with("guard_")
        || lower.contains("_validator")
        || lower.contains("_verifier")
}

fn is_conditional(kind: &str) -> bool {
    matches!(
        kind,
        "if_statement"
            | "if_expression"
            | "match_expression"
            | "match_statement"
            | "switch_statement"
            | "while_statement"
            | "while_expression"
            | "for_statement"
            | "for_expression"
            | "for_in_statement"
            | "loop_expression"
            | "conditional_expression"
            | "ternary_expression"
    )
}

fn collect_tainted_vars(
    node: Node,
    source: &str,
    registry: &TaintRegistry,
    vars: &mut HashSet<String>,
) {
    let kind = node.kind();

    if kind == "identifier" || kind == "variable_declarator" {
        let name = &source[node.start_byte()..node.end_byte()];
        if registry.is_tainted(name) {
            vars.insert(name.to_string());
        }
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_tainted_vars(cursor.node(), source, registry, vars);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_flow::TaintOrigin;

    #[test]
    fn test_is_validation_name() {
        assert!(is_validation_name("validate_user"));
        assert!(is_validation_name("check_password"));
        assert!(is_validation_name("verify_token"));
        assert!(is_validation_name("sanitize_input"));
        assert!(!is_validation_name("get_user"));
        assert!(!is_validation_name("compute_hash"));
    }

    #[test]
    fn test_hollow_validator_detection() {
        let source = r#"
fn validate_input(x: &str) -> bool {
    x.len() > 0
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let mut registry = TaintRegistry::default();
        registry.taint("x", TaintOrigin::UserInput);

        let metrics = TaintMetrics::compute(&registry, root, source, "validate_input");
        assert!(metrics.has_validation_name);
        assert!(metrics.taint_branch_ratio < 0.2);
        assert!(metrics.is_hollow_validator());
    }

    #[test]
    fn test_real_validator_not_hollow() {
        let source = r#"
fn validate_token(tok: &str) -> bool {
    if tok.len() > 100 { return false; }
    if tok.contains("DROP") { return false; }
    true
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let mut registry = TaintRegistry::default();
        registry.taint("tok", TaintOrigin::UserInput);

        let metrics = TaintMetrics::compute(&registry, root, source, "validate_token");
        assert!(metrics.has_validation_name);
        assert!(
            metrics.taint_branch_ratio > 0.2,
            "real validator should branch on tainted data"
        );
    }
}
