// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct DeadlockGuard;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for DeadlockGuard {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_ASYNC_MUTEX_DEADLOCK"),
            name: Cow::Borrowed("Async Mutex Deadlock Detector"),
            severity: Severity::Critical,
            observation: Cow::Borrowed("Potential async deadlock detected: Mutex guard held across .await point."),
            impact: Cow::Borrowed("Holding a standard Mutex guard across an await point can block the entire executor thread."),
            improvement: Cow::Borrowed("Use tokio::sync::Mutex or ensure the guard is dropped before the await."),
            tags: vec![Cow::Borrowed("reliability"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
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

        if let Some(parent_fn) = Self::find_parent_function(node)
            && Self::has_mutex_lock(parent_fn, node, context.source_code)
        {
            advisories.push(
                self.new_advisory(
                    &node,
                    context,
                    "Potential async deadlock detected: Mutex guard held across .await point."
                        .to_string(),
                ),
            );
        }

        advisories
    }
}

impl DeadlockGuard {
    fn find_parent_function(node: Node<'_>) -> Option<Node<'_>> {
        let mut current = node;
        while let Some(parent) = current.parent() {
            match parent.kind() {
                "function_item" | "closure_expression" | "block" => return Some(parent),
                _ => current = parent,
            }
        }
        None
    }

    fn has_mutex_lock(scope: Node, await_node: Node, source: &str) -> bool {
        let await_start = await_node.start_byte();
        scan_for_lock(scope, await_start, source)
    }
}

fn scan_for_lock(node: Node, before_byte: usize, source: &str) -> bool {
    if node.kind() == "call_expression"
        && let Some(f) = node.child_by_field_name("function")
    {
        let code = &source[f.start_byte()..f.end_byte()];
        if code.contains(".lock") && f.start_byte() < before_byte {
            return true;
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.start_byte() < before_byte && scan_for_lock(child, before_byte, source) {
            return true;
        }
    }
    false
}
