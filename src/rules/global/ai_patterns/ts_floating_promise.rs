use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TsFloatingPromiseDetector;

impl GenSenseRule for TsFloatingPromiseDetector {
    fn id(&self) -> &str {
        "TS_FLOATING_PROMISE"
    }
    fn description(&self) -> &str {
        "Unawaited promise detected (fetch/prisma/db)."
    }
    fn applies_to(&self, ext: &str) -> bool {
        ext == "ts" || ext == "tsx"
    }
    fn query(&self) -> Option<&str> {
        Some("(expression_statement) @stmt")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // Identify promise-returning calls
        let promise_sinks = ["fetch(", "prisma.", "axios.", "db.", "supabase."];
        if promise_sinks.iter().any(|&sink| code.contains(sink)) {
            // Check if it's awaited or returned
            let is_awaited = code.contains("await ");
            let is_returned = code.contains("return ");
            let is_assigned = code.contains("let ")
                || code.contains("const ")
                || code.contains("var ")
                || code.contains(" = ");

            if !is_awaited && !is_returned && !is_assigned {
                advisories.push(self.new_advisory(
                    &node,
                    "Floating Promise: A promise-returning call is not being awaited or handled.".to_string(),
                    "Floating promises can lead to unhandled rejections and out-of-order execution logic.".to_string(),
                    "Add 'await', 'return', or assign the promise to a variable.".to_string(),
                ));
            }
        }

        advisories
    }
}
