// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TodoGuard;

impl GenSenseRule for TodoGuard {
    fn id(&self) -> &str {
        "GLOBAL_TODO_PLACEHOLDER"
    }

    fn description(&self) -> &str {
        "Unresolved TODO or FIXME detected."
    }

    fn query(&self) -> Option<&str> {
        None // We traverse everything or look for comments
    }

    fn applies_to(&self, _ext: &str) -> bool {
        true
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        if node.kind().contains("comment") {
            let code = &context.source_code[node.start_byte()..node.end_byte()];
            if (code.contains("TODO") || code.contains("FIXME") || code.contains("HACK"))
                && !code.contains("AUDITED")
            {
                let proposed = code
                    .replace("TODO", "AUDITED(TODO)")
                    .replace("FIXME", "AUDITED(FIXME)")
                    .replace("HACK", "AUDITED(HACK)");

                advisories.push(self.new_remediated_advisory(
                    &node,
                    "We found an unresolved 'TODO', 'FIXME', or 'HACK' in the code.".to_string(),
                    "Unresolved placeholders in production code can indicate incomplete features or deferred debt that might be forgotten.".to_string(),
                    "We suggest resolving the task or tracking it in the project's issue management system.".to_string(),
                    code.to_string(),
                    proposed,
                ));
            }
        }

        advisories
    }
}
