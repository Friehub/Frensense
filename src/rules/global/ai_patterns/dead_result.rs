use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct DeadResult;

impl GenSenseRule for DeadResult {
    fn id(&self) -> &str {
        "AI_DEAD_RESULT_DISCARD"
    }
    fn description(&self) -> &str {
        "Silent result discard (let _ = ...) detected."
    }
    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }
    fn query(&self) -> Option<&str> {
        Some("(let_declaration) @let")
    }

    fn check<'a>(&self, node: Node<'a>, context: & GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];
        if code.starts_with("let _ =") && code.contains("(") {
            advisories.push(self.new_advisory(
                &node,
                "We observed a result being silently discarded using 'let _ ='.".to_string(),
                "Discarding results can hide silent failures and make debugging significantly more difficult.".to_string(),
                "Consider handling the result explicitly or using '.expect()' / '.unwrap()'.".to_string(),
            ));
        }
        advisories
    }
}
