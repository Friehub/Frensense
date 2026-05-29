// SPDX-License-Identifier: MIT

use tree_sitter::Node;

use crate::pattern::compiler::{Pattern, PatternConstraint, PatternNode};

#[derive(Debug, Clone, Default)]
pub struct MatchResult {
    pub matched: bool,
    pub captures: Vec<(String, (usize, usize))>,
    pub score: f64,
}

#[derive(Debug, Default)]
pub struct PatternMatcher;

impl PatternMatcher {
    pub fn match_node(pattern: &PatternNode, node: Node, source: &str) -> MatchResult {
        let mut result = MatchResult::default();
        let mut captures = Vec::new();
        let matched = Self::match_recursive(pattern, node, source, &mut captures);
        result.matched = matched;
        result.captures = captures;
        result.score = if matched { 1.0 } else { 0.0 };
        result
    }

    pub fn match_pattern(pattern: &Pattern, node: Node, source: &str) -> MatchResult {
        match pattern {
            Pattern::Exact(pn) | Pattern::Structural(pn) => Self::match_node(pn, node, source),
            Pattern::Semantic(pn, constraints) => {
                let mut result = Self::match_node(pn, node, source);
                if result.matched {
                    for constraint in constraints {
                        result.matched = Self::check_constraint(constraint, &result.captures, node, source);
                        if !result.matched {
                            result.score = 0.0;
                            break;
                        }
                    }
                }
                result
            }
        }
    }

    fn match_recursive(
        pattern: &PatternNode,
        node: Node,
        source: &str,
        captures: &mut Vec<(String, (usize, usize))>,
    ) -> bool {
        if pattern.is_wildcard {
            if let Some(field) = &pattern.field_name {
                captures.push((field.clone(), (node.start_byte(), node.end_byte())));
            }
            return true;
        }

        if node.kind() != pattern.kind {
            return false;
        }

        if let Some(ref expected_text) = pattern.text {
            if node.child_count() == 0 {
                if let Ok(actual) = node.utf8_text(source.as_bytes()) {
                    if actual != expected_text {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        if let Some(field) = &pattern.field_name {
            captures.push((field.clone(), (node.start_byte(), node.end_byte())));
        }

        if pattern.children.is_empty() {
            return true;
        }

        let mut child_idx = 0;
        let mut cursor = node.walk();
        let mut matched_children = 0;

        loop {
            if !cursor.goto_first_child() {
                break;
            }

            loop {
                let child = cursor.node();
                if child_idx < pattern.children.len() {
                    if Self::match_recursive(&pattern.children[child_idx], child, source, captures) {
                        matched_children += 1;
                    }
                    child_idx += 1;
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            break;
        }

        matched_children > 0
    }

    fn check_constraint(
        constraint: &PatternConstraint,
        captures: &[(String, (usize, usize))],
        _node: Node,
        _source: &str,
    ) -> bool {
        if let Some(ref kind) = constraint.kind {
            if !captures.iter().any(|(_, _)| false) {
                if !captures.is_empty() {
                    return true;
                }
            }
            if !captures.iter().any(|(name, _)| name == kind) {
                return false;
            }
        }
        true
    }

    pub fn match_all(pattern: &PatternNode, root: Node, source: &str) -> Vec<MatchResult> {
        let mut results = Vec::new();
        let mut cursor = root.walk();

        loop {
            let node = cursor.node();
            let result = Self::match_node(pattern, node, source);
            if result.matched {
                results.push(result);
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return results;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::compiler::PatternCompiler;

    #[test]
    fn test_match_exact_expression() {
        let source = "let x = 1;";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let pattern = PatternCompiler::compile_node(root, source);
        let result = PatternMatcher::match_node(&pattern, root, source);
        assert!(result.matched);
    }

    #[test]
    fn test_no_match_different_kind() {
        let source_a = "let x = 1;";
        let source_b = "fn foo() {}";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        let tree_a = parser.parse(source_a, None).unwrap();
        let tree_a_root = tree_a.root_node();
        let tree_b = parser.parse(source_b, None).unwrap();
        let tree_b_root = tree_b.root_node();
        let pattern = PatternCompiler::compile_node(tree_a_root, source_a);
        let result = PatternMatcher::match_node(&pattern, tree_b_root, source_b);
        assert!(!result.matched);
    }
}
