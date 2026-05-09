// SPDX-License-Identifier: MIT

use super::DataFlowAnalyzer;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a> {
    pub fn find_definition(&self, name: &str) -> Option<Node<'a>> {
        let mut current = self.root;
        while let Some(parent) = current.parent() {
            current = parent;
        }

        let query_str = format!(
            "[(function_item name: (identifier) @name)
              (function_declaration name: (identifier) @name)
              (variable_declarator name: (identifier) @name)
              (let_declaration pattern: (identifier) @name)
              (lexical_declaration (variable_declarator name: (identifier) @name))
             ] (#eq? @name \"{name}\")"
        );

        let language = crate::parser::ParserRegistry::get_language(self.context.file_path).ok()?;
        let query = tree_sitter::Query::new(&language, &query_str).ok()?;
        let mut cursor = tree_sitter::QueryCursor::new();
        let matches = cursor.matches(&query, current, self.context.source_code.as_bytes());

        for m in matches {
            if let Some(capture) = m.captures.iter().next() {
                return Some(capture.node.parent().unwrap_or(capture.node));
            }
        }
        None
    }

    pub fn map_params(
        &self,
        def_node: Node<'a>,
        tainted_args: &[usize],
    ) -> Option<super::TaintRegistry> {
        let params_node = def_node.child_by_field_name("parameters")?;
        let mut registry = super::TaintRegistry::default();
        let mut cursor = params_node.walk();
        let mut p_idx = 0;

        for param in params_node.children(&mut cursor) {
            if matches!(param.kind(), "(" | ")" | ",") {
                continue;
            }
            if tainted_args.contains(&p_idx) {
                let p_name = &self.context.source_code[param.start_byte()..param.end_byte()];
                registry.taint(p_name.to_string(), "parameter".to_string());
            }
            p_idx += 1;
        }
        Some(registry)
    }
}
