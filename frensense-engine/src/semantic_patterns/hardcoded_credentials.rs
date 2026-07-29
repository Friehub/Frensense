use crate::data_flow::entropy::{calculate_shannon_entropy, is_secret_indicator, MIN_LENGTH_FOR_ENTROPY};
use crate::semantic_patterns::helpers::node_text;
use crate::semantic_patterns::registry::SemanticPattern;
use crate::semantic_patterns::PatternFinding;
use tree_sitter::Node;

pub struct HardcodedCredentials;

const ENTROPY_THRESHOLD: f64 = 4.5;

impl HardcodedCredentials {
    fn scan_node(node: Node, source: &str, findings: &mut Vec<PatternFinding>) {
        if node.kind() == "assignment_expression" || node.kind() == "variable_declarator" {
            let lhs = node.child_by_field_name("left")
                .or_else(|| node.child_by_field_name("name"));
            let rhs = node.child_by_field_name("right")
                .or_else(|| node.child_by_field_name("value"));

            if let (Some(lhs_node), Some(rhs_node)) = (lhs, rhs) {
                let lhs_text = node_text(lhs_node, source);

                if is_secret_indicator(lhs_text) {
                    if rhs_node.kind() == "string"
                        || rhs_node.kind() == "string_literal"
                        || rhs_node.kind() == "template_string"
                    {
                        let rhs_text = node_text(rhs_node, source);
                        let inner = rhs_text.trim_matches('"').trim_matches('\'').trim_matches('`');

                        if inner.len() >= MIN_LENGTH_FOR_ENTROPY {
                            let entropy = calculate_shannon_entropy(inner);
                            if entropy >= ENTROPY_THRESHOLD {
                                let line = source[..rhs_node.start_byte()].lines().count() + 1;
                                let col = source[..rhs_node.start_byte()]
                                    .rfind('\n').map_or(rhs_node.start_byte() + 1, |i| rhs_node.start_byte() - i);

                                findings.push(PatternFinding {
                                    pattern_id: "HARDCODED_CREDENTIALS".to_string(),
                                    severity: "Warning".to_string(),
                                    line,
                                    column: col,
                                    observation: format!("Hardcoded credential detected: variable `{lhs_text}` is assigned a high-entropy string literal (entropy: {entropy:.1}). This could be a secret, API key, or password embedded in source code."),
                                    impact: "Hardcoded secrets can be exposed through version control, compiled binaries, or error messages, leading to unauthorized access.".to_string(),
                                    improvement: "Store secrets in environment variables, a secrets manager (e.g. AWS Secrets Manager, HashiCorp Vault), or a secure config file excluded from version control.".to_string(),
                                    confidence: 0.78,
                                    tags: vec!["security".to_string(), "secrets".to_string(), "hardcoded".to_string()],
                                    enclosing_function: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                Self::scan_node(child, source, findings);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}

impl SemanticPattern for HardcodedCredentials {
    fn id(&self) -> &str { "HARDCODED_CREDENTIALS" }

    fn description(&self) -> &str {
        "Detects high-entropy string literals assigned to secret-indicator variable names"
    }

    fn severity(&self) -> &str { "Warning" }

    fn languages(&self) -> &[&str] { &["*"] }

    fn scan(&self, tree: Node, source: &str, _file_path: &str) -> Vec<PatternFinding> {
        let mut findings = Vec::new();
        Self::scan_node(tree, source, &mut findings);
        findings
    }
}