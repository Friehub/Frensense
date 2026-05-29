// SPDX-License-Identifier: MIT

use tree_sitter::Node;



#[derive(Debug, Clone)]
pub struct PatternNode {
    pub kind: String,
    pub field_name: Option<String>,
    pub text: Option<String>,
    pub is_wildcard: bool,
    pub children: Vec<PatternNode>,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone)]
pub struct PatternConstraint {
    pub kind: Option<String>,
    pub field: Option<String>,
    pub text: Option<String>,
    pub wildcard: bool,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Exact(PatternNode),
    Structural(PatternNode),
    Semantic(PatternNode, Vec<PatternConstraint>),
}

#[derive(Debug, Default)]
pub struct PatternCompiler;

impl PatternCompiler {
    pub fn compile_node(node: Node, source: &str) -> PatternNode {
        Self::compile_node_inner(node, source, false)
    }

    fn compile_node_inner(node: Node, source: &str, wildcard: bool) -> PatternNode {
        let mut children = Vec::new();
        let mut cursor = node.walk();
        loop {
            if cursor.goto_first_child() {
                let child = cursor.node();
                children.push(Self::compile_node_inner(child, source, false));
                continue;
            }
            if cursor.goto_next_sibling() {
                let sibling = cursor.node();
                children.push(Self::compile_node_inner(sibling, source, false));
                continue;
            }
            break;
        }
        while cursor.goto_parent() {
            break;
        }

        let text = if children.is_empty() {
            Some(source[node.start_byte()..node.end_byte()].to_string())
        } else {
            None
        };

        PatternNode {
            kind: node.kind().to_string(),
            field_name: None,
            text,
            is_wildcard: wildcard,
            children,
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
        }
    }

    pub fn compile_with_wildcards(node: Node, source: &str, wildcard_kinds: &[&str]) -> Pattern {
        let pattern = Self::compile_node_inner(node, source, false);
        Self::apply_wildcards(pattern, wildcard_kinds)
    }

    fn apply_wildcards(pattern: PatternNode, wildcard_kinds: &[&str]) -> Pattern {
        let mut p = pattern;
        if wildcard_kinds.contains(&p.kind.as_str()) {
            p.is_wildcard = true;
        }
        for child in &mut p.children {
            if wildcard_kinds.contains(&child.kind.as_str()) {
                child.is_wildcard = true;
            }
        }
        Pattern::Exact(p)
    }

    pub fn compile_from_source(source: &str, language: &str) -> Result<Pattern, String> {
        use crate::parser::ParserRegistry;

        let lang = ParserRegistry::get_language_by_name(language).map_err(|e| e.to_string())?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).map_err(|e| e.to_string())?;
        let tree = parser.parse(source, None).ok_or("Failed to parse pattern source")?;
        let root = tree.root_node();
        Ok(Pattern::Exact(Self::compile_node(root, source)))
    }
}

pub fn compile_pattern(node: Node, source: &str) -> PatternNode {
    PatternCompiler::compile_node(node, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_expr() {
        let source = "let x = 1;";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let pattern = PatternCompiler::compile_node(root, source);
        assert_eq!(pattern.kind, "source_file");
        assert!(!pattern.children.is_empty());
    }
}
