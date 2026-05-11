// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct TypeScriptUnsafeCast;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for TypeScriptUnsafeCast {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("TS_UNSAFE_TYPE_ASSERTION"),
            name: Cow::Borrowed("Unsafe Type Assertion"),
            severity: Severity::Warning,
            impact: Cow::Borrowed("Unsafe type assertion (as any) bypasses TypeScript's type safety guarantees, potentially leading to runtime crashes or silent data corruption."),
            improvement: Cow::Borrowed("Use type guards (is), proper interface definitions, or 'unknown' with validation instead of 'any'."),
            tags: vec![Cow::Borrowed("safety"), Cow::Borrowed("typescript")],
            category: Cow::Borrowed("Safety"),
        })
    }

    fn query(&self) -> Option<&str> {
        Some("(as_expression) @as")
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "ts" || ext == "tsx"
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        if let Some(type_node) = node.child_by_field_name("type") {
            let type_code = &context.source_code[type_node.start_byte()..type_node.end_byte()];
            if type_code == "any" {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    "Unsafe TypeScript type assertion (as any) detected.".to_string(),
                ));
            }
        }

        advisories
    }
}
