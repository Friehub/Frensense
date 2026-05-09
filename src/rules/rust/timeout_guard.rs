// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TimeoutGuard;

impl GenSenseRule for TimeoutGuard {
    fn id(&self) -> &str {
        "RUST_MISSING_TIMEOUT"
    }

    fn description(&self) -> &str {
        "Async I/O operation missing explicit timeout protection."
    }

    fn severity(&self) -> crate::Severity {
        crate::Severity::Critical
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        // Target await expressions
        Some("(await_expression) @await")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // Identify risky I/O operations (network, file, etc.)
        let is_network = code.contains("reqwest")
            || code.contains("TcpStream")
            || code.contains("connect")
            || code.contains("get(");

        // Check if it's wrapped in a timeout
        // This is a heuristic: check parent nodes or surrounding text for 'timeout'
        let mut wrapped_in_timeout = false;
        let mut current = node;
        while let Some(parent) = current.parent() {
            let parent_code = &context.source_code[parent.start_byte()..parent.end_byte()];
            if parent_code.contains("timeout(") {
                wrapped_in_timeout = true;
                break;
            }
            if parent.kind() == "function_item" || parent.kind() == "block" {
                break;
            }
            current = parent;
        }

        if is_network && !wrapped_in_timeout {
            advisories.push(self.new_advisory(
                &node,
                "Potential async livelock: risky I/O operation missing timeout guard.".to_string(),
                "A network call that never responds can hang an entire worker task indefinitely, leading to protocol-level stall.".to_string(),
                "Wrap this operation in 'tokio::time::timeout(Duration, ...).await' to ensure system liveness.".to_string(),
            ));
        }

        advisories
    }
}
