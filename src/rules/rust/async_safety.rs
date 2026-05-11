// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct AsyncPanicSafety;

impl GenSenseRule for AsyncPanicSafety {
    fn id(&self) -> &str {
        "RUST_ASYNC_PANIC_PREVENTION"
    }

    fn description(&self) -> &str {
        "Unsafe error handling pattern (unwrap/expect/panic) in async scope."
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        // High-Precision Matcher: Finds .unwrap(), .expect(), and panic!() in one pass.
        Some("[ (call_expression) (macro_invocation) ] @node")
    }

    fn check<'a>(&self, node: Node<'a>, context: & GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // The engine finds the node, we verify the scope and context.
        if self.is_in_async_scope(node, context.source_code) {
            self.verify_node(node, context, &mut advisories);
        }

        advisories
    }
}

impl AsyncPanicSafety {
    fn is_in_async_scope(&self, node: Node, source: &str) -> bool {
        let mut current = node;
        while let Some(parent) = current.parent() {
            let kind = parent.kind();
            if kind == "async_block" {
                return true;
            }
            if kind == "function_item" {
                let header = &source[parent.start_byte()
                    ..parent
                        .child_by_field_name("body")
                        .map_or(parent.end_byte(), |b| b.start_byte())];
                return header.contains("async");
            }
            current = parent;
        }
        false
    }

    fn verify_node(&self, node: Node, context: &GenSenseContext, advisories: &mut Vec<Advisory>) {
        let kind = node.kind();
        if kind == "call_expression" {
            if let Some(func) = node.child_by_field_name("function") {
                let code = &context.source_code[func.start_byte()..func.end_byte()];
                if code.ends_with(".unwrap") || code.ends_with(".expect") {
                    self.push_advisory(node, advisories);
                }
            }
        } else if kind == "macro_invocation" {
            if let Some(macro_name) = node.child(0) {
                let code = &context.source_code[macro_name.start_byte()..macro_name.end_byte()];
                if code == "panic" {
                    self.push_advisory(node, advisories);
                }
            }
        }
    }

    fn push_advisory(&self, node: Node, advisories: &mut Vec<Advisory>) {
        advisories.push(
            self.new_advisory(
                &node,
                "Potential async panic safety violation.".to_string(),
                "Unwrapped Result or Option in async context can lead to unhandled task failure."
                    .to_string(),
                "Use '?' or handle the error gracefully to ensure protocol stability.".to_string(),
            ),
        );
    }
}
