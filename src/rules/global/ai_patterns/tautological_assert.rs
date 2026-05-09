use crate::{Advisory, AuditContext, AuditorRule};
use tree_sitter::Node;

pub struct TautologicalAssert;

impl AuditorRule for TautologicalAssert {
    fn id(&self) -> &str {
        "AI_TAUTOLOGICAL_ASSERT"
    }
    fn description(&self) -> &str {
        "Tautological assertion detected (e.g. assert!(true))."
    }
    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }
    fn query(&self) -> Option<&str> {
        Some("(macro_invocation) @macro")
    }

    fn check(&self, node: Node, context: &AuditContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        if let Some(macro_name_node) = node.child(0) {
            let macro_name =
                &context.source_code[macro_name_node.start_byte()..macro_name_node.end_byte()];
            if matches!(macro_name, "assert" | "assert_eq" | "assert_ne") {
                let code = &context.source_code[node.start_byte()..node.end_byte()];
                if code.contains("assert!(true)") || code.contains("assert_eq!(1, 1)") {
                    advisories.push(self.new_advisory(
                        &node,
                        "We noticed a tautological assertion that is always true.".to_string(),
                        "Assertions that cannot fail do not contribute to the correctness of the system and may be artifacts of automated code generation.".to_string(),
                        "We recommend removing the tautology or replacing it with a meaningful check.".to_string(),
                    ));
                }
            }
        }
        advisories
    }
}
