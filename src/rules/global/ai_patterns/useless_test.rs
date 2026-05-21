use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct UselessTest;
static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for UselessTest {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("AI_USELESS_TEST"),
            name: Cow::Borrowed("Useless Test Detector"),
            severity: Severity::Warning,
            observation: Cow::Borrowed("Test function detected that logs output but lacks assertions."),
            impact: Cow::Borrowed("Tests without assertions do not verify program correctness and can pass even if the logic under test is failing."),
            improvement: Cow::Borrowed("Add assertions that validate the expected outcomes."),
            tags: vec![Cow::Borrowed("testing"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Testing"),
            confidence: 0.85,
        })
    }

    fn check<'a>(&self, node: Node<'a>, context: & GenSenseContext<'a>) -> Vec<Advisory> {
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
                        context,
                        "We noticed a test function that logs output but lacks assertions.".to_string(),
                    ));
                }
            }
        }
        advisories
    }
}
