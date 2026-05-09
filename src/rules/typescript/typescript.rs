// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TypeScriptUnsafeCast;

impl GenSenseRule for TypeScriptUnsafeCast {
    fn id(&self) -> &str {
        "TS_UNSAFE_TYPE_ASSERTION"
    }

    fn description(&self) -> &str {
        "Unsafe type assertion (as any) detected. This bypasses TypeScript's type safety guarantees."
    }

    fn query(&self) -> Option<&str> {
        Some("as_expression")
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "ts" || ext == "tsx"
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // Check if the target type is 'any'
        if let Some(type_node) = node.child_by_field_name("type") {
            let type_code = &context.source_code[type_node.start_byte()..type_node.end_byte()];
            if type_code == "any" {
                let _pos = node.start_position();
                advisories.push(self.new_advisory(
                    &node,
                    "Unsafe TypeScript type assertion (as) detected.".to_string(),
                    "Force-casting types with 'as' can hide underlying type mismatches and lead to runtime errors.".to_string(),
                    "Use type guards or proper interface definitions to ensure type safety.".to_string(),
                ));
            }
        }

        advisories
    }
}
