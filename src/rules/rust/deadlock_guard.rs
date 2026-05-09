// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct DeadlockGuard;

impl GenSenseRule for DeadlockGuard {
    fn id(&self) -> &str {
        "RUST_ASYNC_MUTEX_DEADLOCK"
    }

    fn description(&self) -> &str {
        "Potential deadlock: async lock guard held across .await point."
    }

    fn category(&self) -> &str {
        "Reliability"
    }
    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        // Find all await points - we check for locks in their parent scopes
        Some("await_expression")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // The engine found an 'await_expression'
        if let Some(parent_fn) = self.find_parent_function(node) {
            if self.has_mutex_lock(parent_fn, node, context.source_code) {
                let _pos = node.start_position();
                advisories.push(self.new_advisory(
                    &node,
                    "Potential async deadlock detected.".to_string(),
                    "Holding a standard Mutex guard across an await point can block the entire executor thread.".to_string(),
                    "Use tokio::sync::Mutex or ensure the guard is dropped before the await.".to_string(),
                ));
            }
        }

        advisories
    }
}

impl DeadlockGuard {
    /// Internal Helper: Finds the nearest parent function or closure scope.
    fn find_parent_function<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        let mut current = node;
        while let Some(parent) = current.parent() {
            match parent.kind() {
                "function_item" | "closure_expression" | "block" => return Some(parent),
                _ => current = parent,
            }
        }
        None
    }

    /// Internal Helper: Scans a scope for 'lock()' calls on std::sync::Mutex.
    /// This is an MVP heuristic: looks for 'lock(' before the await point in the same block.
    fn has_mutex_lock(&self, scope: Node, await_node: Node, source: &str) -> bool {
        let _cursor = scope.walk();
        let await_start = await_node.start_byte();

        // Strategy: Look for '.lock()' calls that occur BEFORE the await point in this scope
        self.scan_for_lock(scope, await_start, source)
    }

    fn scan_for_lock(&self, node: Node, before_byte: usize, source: &str) -> bool {
        // High-precision check: Look for method calls named 'lock'
        if node.kind() == "call_expression" {
            let func = node.child_by_field_name("function");
            if let Some(f) = func {
                let code = &source[f.start_byte()..f.end_byte()];
                if code.contains(".lock") && f.start_byte() < before_byte {
                    return true;
                }
            }
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                if cursor.node().start_byte() < before_byte
                    && self.scan_for_lock(cursor.node(), before_byte, source)
                {
                    return true;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        false
    }
}
