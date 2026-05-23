// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct BlockingIoDetector;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for BlockingIoDetector {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_ASYNC_BLOCKING_IO"),
            name: Cow::Borrowed("Sync Blocking IO in Async"),
            severity: Severity::Warning,
            observation: Cow::Borrowed("Synchronous blocking call detected in an async context."),
            impact: Cow::Borrowed("Calling synchronous blocking functions inside an async task blocks the entire executor thread, stalling all other tasks on that thread."),
            improvement: Cow::Borrowed("Use asynchronous alternatives (e.g., tokio::time::sleep, tokio::fs, or tokio::net). If no async version exists, use spawn_blocking."),
            tags: vec![Cow::Borrowed("performance"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Performance"),
            confidence: 0.85,
            precision: crate::Precision::VeryHigh,
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        crate::parser::ParserRegistry::ext_matches(ext, &["rs"])
    }

    fn query(&self) -> Option<&str> {
        Some("(call_expression) @call")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        if super::is_excluded_test_scope(node, context) {
            return Vec::new();
        }

        // Skip if this call expression is wrapped in .await (correctly async)
        if let Some(parent) = node.parent()
            && parent.kind() == "await_expression"
        {
            return Vec::new();
        }

        // Skip if this call is .unwrap()/.expect() on an await_expression
        if let Some(func) = node.child_by_field_name("function")
            && func.kind() == "field_expression"
            && let Some(value) = func.child_by_field_name("value")
            && value.kind() == "await_expression"
        {
            return Vec::new();
        }

        if super::is_in_async_scope(node, context.source_code)
            && let Some(func) = node.child_by_field_name("function")
        {
            let code = &context.source_code[func.start_byte()..func.end_byte()];

            let blocking_patterns = [
                "std::thread::sleep",
                "thread::sleep",
                "std::fs",
                "fs::",
                "std::net",
                "TcpStream::connect",
                "TcpListener::bind",
            ];

            if blocking_patterns.iter().any(|p| code.contains(p)) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Potentially blocking call '{code}' detected in async context."),
                ));
            }
        }

        advisories
    }
}
