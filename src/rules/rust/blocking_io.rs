// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct BlockingIoDetector;

impl GenSenseRule for BlockingIoDetector {
    fn id(&self) -> &str {
        "RUST_ASYNC_BLOCKING"
    }

    fn description(&self) -> &str {
        "Potentially blocking call (sleep/fs/net) detected in async context."
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("(call_expression) @call")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        if self.is_in_async_scope(node, context.source_code) {
            if let Some(func) = node.child_by_field_name("function") {
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
                        "Potentially blocking call detected in async context.".to_string(),
                        "Calling synchronous blocking functions inside an async task blocks the entire executor thread, stalling all other tasks.".to_string(),
                        "Use the asynchronous alternatives provided by tokio (e.g., tokio::time::sleep or tokio::fs).".to_string(),
                    ));
                }
            }
        }

        advisories
    }
}

impl BlockingIoDetector {
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
}
