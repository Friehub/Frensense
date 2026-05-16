use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TautologicalAssert;

impl GenSenseRule for TautologicalAssert {
    fn metadata(&self) -> &crate::RuleMetadata {
        static META: std::sync::LazyLock<crate::RuleMetadata> = std::sync::LazyLock::new(|| {
            crate::RuleMetadata {
                id: "AI_TAUTOLOGICAL_ASSERT".into(),
                name: "Tautological Assertion".into(),
                severity: crate::Severity::Warning,
                observation: "A tautological assertion was detected (e.g., assert!(true) or assert_eq!(x, x)).".into(),
                impact: "Assertions that cannot fail do not contribute to correctness and may be AI-generated artifacts.".into(),
                improvement: "Replace with a meaningful check or remove the redundant assertion.".into(),
                tags: vec!["ai-risk".into(), "correctness".into()],
                category: "Logic".into(),
            }
        });
        &META
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("(macro_invocation) @macro")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let Some(macro_name_node) = node.child(0) else {
            return advisories;
        };
        let macro_name =
            &context.source_code[macro_name_node.start_byte()..macro_name_node.end_byte()];

        if !matches!(macro_name, "assert" | "assert_eq" | "assert_ne") {
            return advisories;
        }

        // Find the token_tree (argument list) node
        let Some(token_tree) = node.child_by_field_name("token_tree").or_else(|| {
            node.children(&mut node.walk())
                .find(|c| c.kind() == "token_tree")
        }) else {
            return advisories;
        };

        // Collect non-punctuation argument nodes
        let args: Vec<Node> = token_tree
            .children(&mut token_tree.walk())
            .filter(|c| !matches!(c.kind(), "," | "!" | "(" | ")"))
            .collect();

        let is_tautology = match macro_name {
            "assert" => {
                // assert!(true) or assert!(false) — both are tautological
                args.first().is_some_and(|arg| {
                    let text = context.source_code[arg.start_byte()..arg.end_byte()].trim();
                    if text == "true" || text == "false" {
                        true
                    } else if arg.kind() == "binary_expression" {
                        let lhs_node = arg.child_by_field_name("left");
                        let rhs_node = arg.child_by_field_name("right");
                        if let (Some(l), Some(r)) = (lhs_node, rhs_node) {
                            let lhs_text = context.source_code[l.start_byte()..l.end_byte()].trim();
                            let rhs_text = context.source_code[r.start_byte()..r.end_byte()].trim();
                            lhs_text == rhs_text
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
            }
            "assert_eq" | "assert_ne" => {
                if args.len() >= 2 {
                    let lhs = context.source_code[args[0].start_byte()..args[0].end_byte()].trim();
                    let rhs = context.source_code[args[1].start_byte()..args[1].end_byte()].trim();
                    // Same text on both sides — assert_eq!(x, x)
                    lhs == rhs
                } else {
                    false
                }
            }
            _ => false,
        };

        if is_tautology {
            let advisory = self.new_advisory(
                &node,
                context,
                "Tautological assertion: both sides are identical or the condition is a literal."
                    .to_string(),
            );
            advisories.push(self.with_confidence(advisory, 0.85));
        } else {
            // Fallback: If AST isn't structured (common in macros), use regex/string comparison on the whole argument list
            let text = context.source_code[token_tree.start_byte()..token_tree.end_byte()].trim();
            // Match (x == x) or (x != x) or (true) etc.
            if let Some(caps) =
                regex::Regex::new(r"^\(\s*([^=!\s]+)\s*(?:==|!=)\s*([^=!\s]+)\s*\)$")
                    .ok()
                    .and_then(|re| re.captures(text))
            {
                if caps.get(1).map(|m| m.as_str()) == caps.get(2).map(|m| m.as_str()) {
                    let advisory = self.new_advisory(
                        &node,
                        context,
                        format!(
                            "Tautological assertion detected via text analysis: '{}'.",
                            caps.get(0).map_or(text, |m| m.as_str())
                        ),
                    );
                    advisories.push(self.with_confidence(advisory, 0.70)); // Lower confidence for text fallback
                }
            }
        }

        advisories
    }
}
