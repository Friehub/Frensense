// SPDX-License-Identifier: MIT

use rustc_hash::{FxHashSet, FxHasher};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::path::Path;
use tree_sitter::Node;

#[cfg(feature = "fingerprinting")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub ngram_hashes: FxHashSet<u64>,
}

#[cfg(not(feature = "fingerprinting"))]
pub type FunctionFingerprint = ();

#[cfg(feature = "fingerprinting")]
pub fn extract_fingerprints(
    node: Node,
    source_code: &str,
    path: &Path,
    fingerprints: &mut Vec<FunctionFingerprint>,
) {
    let kind = node.kind();
    if matches!(
        kind,
        "function_item" | "function_declaration" | "method_definition" | "arrow_function"
    ) {
        if let Some(body) = node.child_by_field_name("body") {
            let mut function_name = "anonymous".to_string();
            if let Some(name_node) = node.child_by_field_name("name") {
                function_name =
                    source_code[name_node.start_byte()..name_node.end_byte()].to_string();
            } else if kind == "arrow_function" {
                if let Some(parent) = node.parent() {
                    if parent.kind() == "variable_declarator" {
                        if let Some(name_node) = parent.child_by_field_name("name") {
                            function_name = source_code
                                [name_node.start_byte()..name_node.end_byte()]
                                .to_string();
                        }
                    }
                }
            }

            let body_code = &source_code[body.start_byte()..body.end_byte()];
            let tokens: Vec<&str> = body_code
                .split_whitespace()
                .filter(|t| !t.is_empty() && !t.starts_with("//"))
                .collect();

            if tokens.len() >= 5 {
                let mut ngram_hashes = FxHashSet::default();
                for i in 0..=(tokens.len().saturating_sub(5)) {
                    let mut hasher = FxHasher::default();
                    tokens[i..i + 5].hash(&mut hasher);
                    ngram_hashes.insert(hasher.finish());
                }

                let pos = node.start_position();
                fingerprints.push(FunctionFingerprint {
                    file_path: path.to_string_lossy().to_string(),
                    function_name,
                    line: pos.row + 1,
                    ngram_hashes,
                });
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_fingerprints(child, source_code, path, fingerprints);
    }
}
