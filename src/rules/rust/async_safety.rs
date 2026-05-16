// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct AsyncPanicSafety;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for AsyncPanicSafety {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_ASYNC_PANIC_PREVENTION"),
            name: Cow::Borrowed("Async Panic Prevention"),
            severity: Severity::Warning,
            observation: Cow::Borrowed("Potential panic point detected in an async scope."),
            impact: Cow::Borrowed("Unwrapped Result or Option in async context can lead to unhandled task failures and cascading system instability."),
            improvement: Cow::Borrowed("Use '?' (try operator) or handle the error gracefully to ensure the async task remains stable."),
            tags: vec![Cow::Borrowed("safety"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Safety"),
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("[ (call_expression) (macro_invocation) ] @node")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        if Self::is_in_async_scope(node, context.source_code) {
            let kind = node.kind();
            if kind == "call_expression" {
                if let Some(func) = node.child_by_field_name("function") {
                    let code = &context.source_code[func.start_byte()..func.end_byte()];
                    if code.ends_with(".unwrap") || code.ends_with(".expect") {
                        advisories.push(
                            self.new_advisory(
                                &node,
                                context,
                                "Unsafe error handling pattern (.unwrap/.expect) in async scope."
                                    .to_string(),
                            ),
                        );
                    }
                }
            } else if kind == "macro_invocation" {
                if let Some(macro_name) = node.child(0) {
                    let code = &context.source_code[macro_name.start_byte()..macro_name.end_byte()];
                    if code == "panic" {
                        advisories.push(self.new_advisory(
                            &node,
                            context,
                            "Unsafe macro (panic!) used in async scope.".to_string(),
                        ));
                    }
                }
            }
        }

        advisories
    }
}

impl AsyncPanicSafety {
    fn is_in_async_scope(node: Node, source: &str) -> bool {
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
}
