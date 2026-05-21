// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct TimeoutGuard;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for TimeoutGuard {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_ASYNC_MISSING_TIMEOUT"),
            name: Cow::Borrowed("Missing Timeout Guard"),
            severity: Severity::Critical,
            observation: Cow::Borrowed("Risky I/O operation detected without an explicit timeout guard."),
            impact: Cow::Borrowed("A network or I/O call that never responds can hang an entire worker task indefinitely, leading to protocol-level stalls or cascading failures."),
            improvement: Cow::Borrowed("Wrap this operation in 'tokio::time::timeout(Duration, ...).await' to ensure system liveness even if the remote peer stops responding."),
            tags: vec![Cow::Borrowed("liveness"), Cow::Borrowed("reliability"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Reliability"),
            confidence: 0.85,
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("(await_expression) @await")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // Heuristic: identify risky I/O operations
        let is_risky = code.contains("reqwest")
            || code.contains("TcpStream")
            || code.contains("connect")
            || code.contains("get(");

        if is_risky {
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

            if !wrapped_in_timeout {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    "Potential async livelock: risky I/O operation missing explicit timeout guard.".to_string(),
                ));
            }
        }

        advisories
    }
}
