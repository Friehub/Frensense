// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct FakeAsyncDetector;

impl GenSenseRule for FakeAsyncDetector {
    fn id(&self) -> &str {
        "RUST_FAKE_ASYNC"
    }

    fn description(&self) -> &str {
        "Function marked as async but contains no .await points (Fake Async)."
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        // Match all function items - we will filter for 'async' in check()
        Some("(function_item) @func")
    }

    fn check<'a>(&self, node: Node<'a>, context: & GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // 1. Verify it's an async function
        let header_end = node
            .child_by_field_name("body")
            .map_or(node.end_byte(), |b| b.start_byte());
        let header = &context.source_code[node.start_byte()..header_end];

        if header.contains("async") {
            // 2. Scan body for await points
            if let Some(body) = node.child_by_field_name("body") {
                if !has_await(body) {
                    advisories.push(self.new_advisory(
                        &node,
                        "Async function without await points (Fake Async).".to_string(),
                        "Async functions without awaits introduce state machine overhead without concurrency benefits.".to_string(),
                        "Remove the 'async' keyword or implement intended concurrency.".to_string(),
                    ));
                }
            }
        }

        advisories
    }
}

/// Recursive AST walk: checks if any child is an await_expression.
fn has_await(node: Node) -> bool {
    if node.kind() == "await_expression" {
        return true;
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if has_await(cursor.node()) {
                return true;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    false
}
