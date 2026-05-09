use crate::{Advisory, AuditContext, AuditorRule};
use tree_sitter::Node;

pub struct UselessTest;

impl AuditorRule for UselessTest {
    fn id(&self) -> &str {
        "AI_USELESS_TEST"
    }
    fn description(&self) -> &str {
        "Test function detected that logs output but lacks assertions."
    }
    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }
    fn query(&self) -> Option<&str> {
        Some("(function_item) @func")
    }

    fn check(&self, node: Node, context: &AuditContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];
        if code.contains("#[test]") || code.contains("#[tokio::test]") {
            if let Some(body) = node.child_by_field_name("body") {
                let body_code = &context.source_code[body.start_byte()..body.end_byte()];
                let has_assert = body_code.contains("assert!");
                let only_logs = body_code.contains("info!")
                    || body_code.contains("debug!")
                    || body_code.contains("println!");

                if only_logs && !has_assert {
                    advisories.push(self.new_advisory(
                        &node,
                        "We noticed a test function that logs output but lacks assertions.".to_string(),
                        "Tests without assertions do not verify program correctness and can pass even if the logic under test is failing.".to_string(),
                        "We recommend adding assertions that validate the expected outcomes.".to_string(),
                    ));
                }
            }
        }
        advisories
    }
}
