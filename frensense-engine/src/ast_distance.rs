// SPDX-License-Identifier: MIT

//! AST Edit Distance (M2)
//!
//! Computes tree edit distance between structural skeletons of functions.
//! This catches structural differences that n-gram bag-of-words misses.

use tree_sitter::Node;

/// Extract the structural skeleton from an AST node.
/// Returns a list of node kinds (identifiers and literals removed).
pub fn extract_skeleton(root: Node, _source: &str) -> Vec<String> {
    let mut skeleton = Vec::new();
    extract_skeleton_recursive(root, &mut skeleton);
    skeleton
}

/// Recursively extract node kinds, skipping identifiers and literals.
fn extract_skeleton_recursive(node: Node, skeleton: &mut Vec<String>) {
    if skeleton.len() > 256 {
        return;
    }

    let kind = node.kind();

    // Skip leaf nodes that are identifiers or literals
    if node.child_count() == 0 {
        if kind == "identifier"
            || kind == "string"
            || kind == "number"
            || kind == "true"
            || kind == "false"
            || kind == "null"
            || kind == "undefined"
            || kind == "shorthand_property_identifier"
        {
            return;
        }
    }

    skeleton.push(kind.to_string());

    // Recurse into children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_skeleton_recursive(child, skeleton);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Compute tree edit distance between two skeletons using a simplified algorithm.
/// Returns a normalized score between 0.0 (identical) and 1.0 (completely different).
///
/// This uses a greedy approximation of the Zhang-Shasha algorithm for efficiency.
pub fn tree_edit_distance(skeleton_a: &[u64], skeleton_b: &[u64]) -> f64 {
    if skeleton_a.is_empty() && skeleton_b.is_empty() {
        return 0.0;
    }
    if skeleton_a.is_empty() || skeleton_b.is_empty() {
        return 1.0;
    }

    // LCS-based edit distance (simplified but effective for our use case)
    let lcs_len = longest_common_subsequence(skeleton_a, skeleton_b);
    let max_len = skeleton_a.len().max(skeleton_b.len());

    // Normalize to 0-1 where 0 = identical, 1 = completely different
    1.0 - (lcs_len as f64 / max_len as f64)
}

/// Compute longest common subsequence length.
fn longest_common_subsequence(a: &[u64], b: &[u64]) -> usize {
    let m = a.len();
    let n = b.len();

    // Use space-optimized DP
    let mut prev = vec![0usize; n + 1];
    let mut curr = vec![0usize; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }
        std::mem::swap(&mut prev, &mut curr);
        curr.fill(0);
    }

    prev[n]
}

/// Compute AST edit distance between two function nodes.
/// Returns a normalized score between 0.0 (identical structure) and 1.0 (completely different).
pub fn compute_ast_distance(node_a: Node, node_b: Node, source_a: &str, source_b: &str) -> f64 {
    let skeleton_a = extract_skeleton(node_a, source_a);
    let skeleton_b = extract_skeleton(node_b, source_b);
    let mut hash_a = Vec::with_capacity(skeleton_a.len());
    for s in &skeleton_a {
        let mut hasher = rustc_hash::FxHasher::default();
        std::hash::Hash::hash(s, &mut hasher);
        hash_a.push(std::hash::Hasher::finish(&hasher));
    }
    let mut hash_b = Vec::with_capacity(skeleton_b.len());
    for s in &skeleton_b {
        let mut hasher = rustc_hash::FxHasher::default();
        std::hash::Hash::hash(s, &mut hasher);
        hash_b.push(std::hash::Hasher::finish(&hasher));
    }
    tree_edit_distance(&hash_a, &hash_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_skeleton() {
        let source = "function foo(x) { return x + 1; }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let skeleton = extract_skeleton(root, source);
        assert!(!skeleton.is_empty());
        // Should not contain "foo" or "x" (identifiers)
        assert!(!skeleton.contains(&"identifier".to_string()));
    }

    fn hash(s: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = rustc_hash::FxHasher::default();
        s.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_tree_edit_distance_identical() {
        let a = vec![hash("function"), hash("block"), hash("return")];
        let b = vec![hash("function"), hash("block"), hash("return")];
        assert!((tree_edit_distance(&a, &b) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_tree_edit_distance_different() {
        let a = vec![hash("function"), hash("block"), hash("return")];
        let b = vec![hash("if"), hash("block"), hash("while")];
        let dist = tree_edit_distance(&a, &b);
        assert!(dist > 0.5);
    }

    #[test]
    fn test_tree_edit_distance_empty() {
        let a: Vec<u64> = vec![];
        let b = vec![hash("function")];
        assert!((tree_edit_distance(&a, &b) - 1.0).abs() < 0.001);
    }
}
