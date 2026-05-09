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
    let lines: Vec<&str> = source.lines().collect();
    let target = format!("gensense-ignore: {rule_id}");
    let target_all = "gensense-ignore: all";

    let search_start = start_row.saturating_sub(2);
    for i in search_start..start_row {
        if let Some(line) = lines.get(i) {
            if line.contains("//") && (line.contains(&target) || line.contains(target_all)) {
                return true;
            }
        }
    }
    if let Some(line) = lines.get(start_row) {
        if line.contains("//") && (line.contains(&target) || line.contains(target_all)) {
            return true;
        }
    }

    false
}
