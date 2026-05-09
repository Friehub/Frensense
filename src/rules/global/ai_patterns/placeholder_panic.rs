use crate::{Advisory, AuditContext, AuditorRule};
use tree_sitter::Node;

pub struct PlaceholderPanic;

impl AuditorRule for PlaceholderPanic {
    fn id(&self) -> &str {
        "AI_PLACEHOLDER_PANIC"
    }
    fn description(&self) -> &str {
        "Unimplemented placeholder panic detected."
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
            if matches!(macro_name, "todo" | "unimplemented") {
                let code = &context.source_code[node.start_byte()..node.end_byte()];
                if code.contains("TODO")
                    || code.contains("implement")
                    || code.contains("placeholder")
                {
                    advisories.push(self.new_advisory(
                        &node,
                        "We observed a 'todo!' or 'unimplemented!' placeholder macro.".to_string(),
                        "Placeholders left in production paths can lead to unexpected runtime panics and suggest incomplete implementation logic.".to_string(),
                        "Consider providing a basic implementation or returning a 'Result::Err' that the caller can handle gracefully.".to_string(),
                    ));
                }
            }
        }
        advisories
    }
}
