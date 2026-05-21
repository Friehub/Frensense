// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct TracingGuard;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for TracingGuard {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_MISSING_TRACING_SPAN"),
            name: Cow::Borrowed("Missing Observability Span"),
            severity: Severity::Info,
            observation: Cow::Borrowed("Async function detected without observability instrumentation (tracing span)."),
            impact: Cow::Borrowed("Production-grade protocol logic must be visible in telemetry. Missing spans make debugging distributed hangs or latency spikes extremely difficult."),
            improvement: Cow::Borrowed("Add #[tracing::instrument] to the function or create an explicit span using 'info_span!' or 'debug_span!'."),
            tags: vec![Cow::Borrowed("observability"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Observability"),
            confidence: 0.85,
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("(function_item) @fn")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let header_end = node
            .child_by_field_name("body")
            .map_or(node.end_byte(), |b| b.start_byte());
        let header = &context.source_code[node.start_byte()..header_end];

        if header.contains("async") {
            let code = &context.source_code[node.start_byte()..node.end_byte()];

            // Check for #[instrument] or manual span!
            let mut has_instrument =
                code.contains("#[instrument") || code.contains("#[tracing::instrument");
            let has_span = code.contains("span!")
                || code.contains("info_span!")
                || code.contains("debug_span!");

            if !has_instrument {
                // Check preceding siblings for attributes (some grammars separate them)
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
                advisories.push(
                    self.new_advisory(
                        &node,
                        context,
                        "Async function missing observability instrumentation (tracing span)."
                            .to_string(),
                    ),
                );
            }
        }

        advisories
    }
}
