// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TracingGuard;

impl GenSenseRule for TracingGuard {
    fn id(&self) -> &str {
        "RUST_MISSING_TRACING_SPAN"
    }

    fn description(&self) -> &str {
        "Async function lacks observability instrumentation (tracing span)."
    }

    fn category(&self) -> &str {
        "Observability"
    }
    fn tags(&self) -> Vec<&str> {
        vec!["beta"]
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        // Target async functions
        Some("(function_item) @fn")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // Only check async functions
        if !code.contains("async fn") {
            return advisories;
        }

        // Check for #[instrument] attribute or span! macro usage
        let mut has_instrument =
            code.contains("#[instrument") || code.contains("#[tracing::instrument");
        let has_span =
            code.contains("span!") || code.contains("info_span!") || code.contains("debug_span!");

        // If not found in text (which might not include outer attributes in some grammars),
        // check preceding siblings for attribute_item
        if !has_instrument {
            let mut current = node;
            while let Some(prev) = current.prev_sibling() {
                if prev.kind() == "attribute_item" {
                    let attr_code = &context.source_code[prev.start_byte()..prev.end_byte()];
                    if attr_code.contains("instrument") {
                        has_instrument = true;
                        break;
                    }
                } else if prev.kind() != "line_comment" && prev.kind() != "block_comment" {
                    break;
                }
                current = prev;
            }
        }

        if !has_instrument && !has_span {
            advisories.push(self.new_advisory(
                &node,
                "Async function missing observability instrumentation.".to_string(),
                "Production-grade protocol logic must be visible in telemetry. Missing spans make debugging distributed hangs impossible.".to_string(),
                "Add #[tracing::instrument] to the function or create an explicit span using 'span!'.".to_string(),
            ));
        }

        advisories
    }
}
