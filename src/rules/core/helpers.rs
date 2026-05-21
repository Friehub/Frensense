// SPDX-License-Identifier: MIT

use super::CoreRule;
use tree_sitter::Node;

/// Compute the maximum nesting depth of a tree-sitter node.
#[must_use]
pub fn calculate_peak_depth(node: Node) -> usize {
    let mut max_child_depth = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let d = calculate_peak_depth(child);
        if d > max_child_depth {
            max_child_depth = d;
        }
    }

    let kind = node.kind();
    let increases_depth = matches!(
        kind,
        "if_statement"
            | "while_statement"
            | "for_statement"
            | "match_expression"
            | "if_expression"
            | "for_expression"
            | "while_expression"
            | "do_statement"
            | "try_statement"
            | "catch_clause"
            | "finally_clause"
            | "switch_statement"
            | "case_clause"
    );

    if increases_depth {
        max_child_depth + 1
    } else {
        max_child_depth
    }
}

impl CoreRule {
    #[must_use]
    pub fn check_parent_scope(&self, node: Node, scope_pattern: &str, source: &str) -> bool {
        let mut current = node;
        let scopes: Vec<&str> = scope_pattern.split('|').collect();

        while let Some(parent) = current.parent() {
            let kind = parent.kind();
            for scope in &scopes {
                if *scope == "async_fn" && kind == "function_item" {
                    let header = &source[parent.start_byte()
                        ..parent
                            .child_by_field_name("body")
                            .map_or(parent.end_byte(), |b| b.start_byte())];
                    if header.contains("async") {
                        return true;
                    }
                } else if kind == *scope {
                    return true;
                }
            }
            current = parent;
        }
        false
    }
}

pub mod serde_regex_opt {
    use regex::Regex;
    use serde::{Deserialize, Deserializer};

    /// # Errors
    ///
    /// Returns an error if the regex pattern is invalid.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        s.map_or_else(
            || Ok(None),
            |re_str| {
                Regex::new(&re_str)
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            },
        )
    }
}
