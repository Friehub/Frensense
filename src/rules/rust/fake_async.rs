// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct FakeAsyncDetector;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for FakeAsyncDetector {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_FAKE_ASYNC"),
            name: Cow::Borrowed("Fake Async Detector"),
            severity: Severity::Info,
            observation: Cow::Borrowed("Async function detected that contains no await points."),
            impact: Cow::Borrowed("Async functions without awaits introduce state machine overhead and return a future unnecessarily without concurrency benefits."),
            improvement: Cow::Borrowed("Remove the 'async' keyword if the function doesn't need to be concurrent, or implement the intended await points."),
            tags: vec![Cow::Borrowed("optimization"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Performance"),
            confidence: 0.85,
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        crate::parser::ParserRegistry::ext_matches(ext, &["rs"])
    }

    fn query(&self) -> Option<&str> {
        Some("(function_item) @func")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let header_end = node
            .child_by_field_name("body")
            .map_or(node.end_byte(), |b| b.start_byte());
        let header = &context.source_code[node.start_byte()..header_end];

        if header.contains("async")
            && let Some(body) = node.child_by_field_name("body")
            && !has_await(body)
        {
            advisories.push(self.new_advisory(
                &node,
                context,
                "Async function contains no await points (Fake Async).".to_string(),
            ));
        }

        advisories
    }
}

fn has_await(node: Node) -> bool {
    let mut cursor = node.walk();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "await_expression" {
            return true;
        }

        cursor.reset(current);
        if cursor.goto_first_child() {
            loop {
                stack.push(cursor.node());
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
    false
}
