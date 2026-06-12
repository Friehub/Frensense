// SPDX-License-Identifier: MIT

use std::hash::Hash;

use crate::pattern::compiler::PatternNode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalForm {
    pub structure_hash: u64,
    pub kind_sequence: Vec<String>,
    pub depth: usize,
    pub normalized_text: Option<String>,
}

impl CanonicalForm {
    pub fn from_node(node: &PatternNode) -> Self {
        let structure_hash = compute_structure_hash(node);
        let kind_sequence = collect_kind_sequence(node);
        let depth = compute_depth(node);
        let normalized_text = normalize_identifiers(node);

        Self {
            structure_hash,
            kind_sequence,
            depth,
            normalized_text,
        }
    }

    pub fn structural_similarity(&self, other: &CanonicalForm) -> f64 {
        if self.structure_hash == other.structure_hash {
            return 1.0;
        }
        let max_len = self.kind_sequence.len().max(other.kind_sequence.len());
        if max_len == 0 {
            return 0.0;
        }
        let common_prefix = self
            .kind_sequence
            .iter()
            .zip(other.kind_sequence.iter())
            .take_while(|(a, b)| a == b)
            .count();
        common_prefix as f64 / max_len as f64
    }
}

fn compute_structure_hash(node: &PatternNode) -> u64 {
    use std::hash::Hasher;
    let mut hasher = rustc_hash::FxHasher::default();
    node.kind.hash(&mut hasher);
    node.is_wildcard.hash(&mut hasher);
    node.children.len().hash(&mut hasher);
    for child in &node.children {
        child.kind.hash(&mut hasher);
        child.is_wildcard.hash(&mut hasher);
        child.children.len().hash(&mut hasher);
    }
    hasher.finish()
}

fn collect_kind_sequence(node: &PatternNode) -> Vec<String> {
    let mut seq = Vec::new();
    seq.push(node.kind.clone());
    for child in &node.children {
        seq.extend(collect_kind_sequence(child));
    }
    seq
}

fn compute_depth(node: &PatternNode) -> usize {
    if node.children.is_empty() {
        return 1;
    }
    1 + node.children.iter().map(compute_depth).max().unwrap_or(0)
}

fn normalize_identifiers(node: &PatternNode) -> Option<String> {
    if node.text.is_some() && !node.children.is_empty() {
        return None;
    }
    let text = node.text.as_deref()?;
    if text.len() <= 3 || text.chars().all(|c| c.is_ascii_digit()) {
        return Some(text.to_string());
    }
    let normalized = text
        .chars()
        .map(|c| {
            if c.is_uppercase() || c.is_lowercase() {
                'x'
            } else if c.is_ascii_digit() {
                '0'
            } else {
                c
            }
        })
        .collect();
    Some(normalized)
}

pub fn make_canonical(pattern: &PatternNode) -> CanonicalForm {
    CanonicalForm::from_node(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::compiler::PatternCompiler;

    #[test]
    fn test_canonical_form() {
        let source = "let x = 1;";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let pattern = PatternCompiler::compile_node(root, source);
        let cf = CanonicalForm::from_node(&pattern);
        assert!(cf.structure_hash != 0);
        assert!(!cf.kind_sequence.is_empty());
    }

    #[test]
    fn test_similar_siblings() {
        let source_a = "let x = 1;";
        let source_b = "let y = 2;";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        let tree_a = parser.parse(source_a, None).unwrap();
        let tree_b = parser.parse(source_b, None).unwrap();
        let pattern_a = PatternCompiler::compile_node(tree_a.root_node(), source_a);
        let pattern_b = PatternCompiler::compile_node(tree_b.root_node(), source_b);
        let cf_a = CanonicalForm::from_node(&pattern_a);
        let cf_b = CanonicalForm::from_node(&pattern_b);
        let sim = cf_a.structural_similarity(&cf_b);
        assert!(sim > 0.5, "similar let bindings should have high structural similarity");
    }
}
