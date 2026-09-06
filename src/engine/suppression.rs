// SPDX-License-Identifier: MIT

use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tree_sitter::Node;

#[derive(Debug, Serialize, Deserialize)]
pub struct SuppressConfig {
    pub suppressions: Vec<Suppression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suppression {
    pub rule_id: String,
    pub path: String,
}

#[must_use]
pub fn is_suppressed(
    suppressions: &[(String, Pattern)],
    node: Node,
    rule_id: &str,
    source: &str,
    path: &Path,
) -> bool {
    // 1. Config-level suppression
    for (sid, pattern) in suppressions {
        if (sid == rule_id || sid == "all") && pattern.matches_path(path) {
            return true;
        }
    }

    // 2. Inline suppression
    let start_row = node.start_position().row;
    let target = format!("frensense-ignore: {rule_id}");
    let target_all = "frensense-ignore: all";

    let search_start = start_row.saturating_sub(2);
    let mut current_row = 0;
    for line in source.lines() {
        if current_row >= search_start
            && current_row <= start_row
            && (line.contains("//") || line.contains("/*") || line.contains('#'))
            && (line.contains(&target) || line.contains(target_all))
        {
            return true;
        }
        current_row += 1;
        if current_row > start_row {
            break;
        }
    }

    false
}
