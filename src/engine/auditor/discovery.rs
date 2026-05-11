// SPDX-License-Identifier: MIT

use super::GenSenseAuditor;
use crate::{parser::ParserRegistry, semantics::SymbolRegistry, GenSenseError, Result};
use std::path::Path;
use tree_sitter::{Node, Query, QueryCursor};

impl GenSenseAuditor {
    pub fn discover_symbols(
        &self,
        path: &Path,
        content: &str,
        language: &tree_sitter::Language,
        tree: &tree_sitter::Tree,
    ) -> Result<Vec<crate::semantics::Symbol>> {

        let query_str = match ParserRegistry::get_symbol_query(path) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };
        let query =
            Query::new(&language, query_str).map_err(|e| GenSenseError::Config(e.to_string()))?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());
        let capture_names = query.capture_names();
        let path_str = path.to_string_lossy().to_string();
        let mut symbols = Vec::new();

        for m in matches {
            for capture in m.captures {
                let name_idx = capture_names
                    .iter()
                    .position(|&n| n == "name")
                    .unwrap_or(999);
                if capture.index as usize != name_idx {
                    continue;
                }

                let node = capture.node;
                let name = match node.utf8_text(content.as_bytes()) {
                    Ok(s) if !s.is_empty() => s,
                    _ => continue,
                };

                let parent = match node.parent() {
                    Some(p) => p,
                    None => continue,
                };

                let kind = match parent.kind() {
                    "function_item" | "function_declaration" => {
                        crate::semantics::SymbolKind::Function
                    }
                    "parameter" => crate::semantics::SymbolKind::Parameter,
                    "let_declaration" | "variable_declarator" => {
                        crate::semantics::SymbolKind::Variable
                    }
                    "struct_item" => crate::semantics::SymbolKind::Struct,
                    "enum_item" => crate::semantics::SymbolKind::Enum,
                    "trait_item" => crate::semantics::SymbolKind::Interface,
                    "const_item" => crate::semantics::SymbolKind::Constant,
                    _ => crate::semantics::SymbolKind::Unknown,
                };

                if kind == crate::semantics::SymbolKind::Unknown {
                    continue;
                }

                symbols.push(crate::semantics::Symbol {
                    name: name.to_string(),
                    kind,
                    start_byte: parent.start_byte(),
                    end_byte: parent.end_byte(),
                    line: parent.start_position().row + 1,
                    end_line: parent.end_position().row + 1,
                    column: node.start_position().column + 1,
                    file_path: path_str.clone(),
                });
            }
        }
        symbols.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        Ok(symbols)
    }

    pub fn scan_for_edges(
        &self,
        path: &Path,
        content: &str,
        language: &tree_sitter::Language,
        tree: &tree_sitter::Tree,
    ) -> Result<Vec<(String, String)>> {

        let query_str = match ParserRegistry::get_call_query(path) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };
        let query =
            Query::new(&language, query_str).map_err(|e| GenSenseError::Config(e.to_string()))?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());
        let mut edges = Vec::new();

        for m in matches {
            for capture in m.captures {
                let call_node = capture.node;
                let function_node = call_node
                    .child_by_field_name("function")
                    .unwrap_or(call_node);
                let call_name = match function_node.utf8_text(content.as_bytes()) {
                    Ok(s) if !s.is_empty() => s,
                    _ => continue,
                };

                if let Some(func) = self.find_enclosing_function(call_node) {
                    if let Some(name_node) = func.child_by_field_name("name") {
                        let caller_name = match name_node.utf8_text(content.as_bytes()) {
                            Ok(s) if !s.is_empty() => s,
                            _ => continue,
                        };
                        edges.push((caller_name.to_string(), call_name.to_string()));
                    }
                }
            }
        }
        Ok(edges)
    }

    pub fn discover_events(
        &self,
        path: &Path,
        content: &str,
        tree: &tree_sitter::Tree,
        registry: &mut SymbolRegistry,
    ) -> Result<()> {
        self.link_symbols_to_scopes(tree.root_node(), path, content, registry);
        let mut cursor = tree.walk();
        self.traverse_for_events(tree.root_node(), &mut cursor, path, content, registry, None);
        Ok(())
    }

    pub fn link_symbols_to_scopes(
        &self,
        root: Node,
        path: &Path,
        content: &str,
        registry: &mut SymbolRegistry,
    ) {
        let mut cursor = root.walk();
        self.traverse_link_symbols(root, &mut cursor, path, content, registry);
    }

    fn traverse_link_symbols(
        &self,
        node: Node,
        cursor: &mut tree_sitter::TreeCursor,
        path: &Path,
        content: &str,
        registry: &mut SymbolRegistry,
    ) {
        let path_str = path.to_string_lossy();
        let name_node = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("pattern"));

        if let Some(name_node) = name_node {
            let name = match name_node.utf8_text(content.as_bytes()) {
                Ok(s) if !s.is_empty() => s,
                _ => return,
            };
            let line = name_node.start_position().row + 1;

            if let Some(sym) = registry.find_at(name, &path_str, line) {
                let idx = registry
                    .graph
                    .find_node(&sym.name, &sym.file_path, sym.line);
                if let (Some(idx), Some(func)) = (idx, self.find_enclosing_function(node)) {
                    if let Some(fname_node) = func.child_by_field_name("name") {
                        let fname = match fname_node.utf8_text(content.as_bytes()) {
                            Ok(s) if !s.is_empty() => s,
                            _ => return,
                        };
                        for &f_idx in &registry.graph.find_nodes(fname) {
                            if let Some(fsym) = registry.graph.get_symbol(f_idx) {
                                if fsym.file_path == path_str {
                                    let kind = match node.kind() {
                                        "parameter" => crate::semantics::graph::EdgeKind::Parameter,
                                        _ => crate::semantics::graph::EdgeKind::InScope,
                                    };
                                    registry.graph.add_edge(f_idx, idx, kind);
                                }
                            }
                        }
                    }
                }
            }
        }

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                let mut child_cursor = child.walk();
                self.traverse_link_symbols(child, &mut child_cursor, path, content, registry);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    pub fn find_enclosing_function<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        let mut current = node;
        while let Some(parent) = current.parent() {
            match parent.kind() {
                "function_item"
                | "function_declaration"
                | "arrow_function"
                | "method_definition" => return Some(parent),
                _ => current = parent,
            }
        }
        None
    }

    pub fn extract_semantic_ops(
        &self,
        path: &Path,
        content: &str,
        tree: &tree_sitter::Tree,
    ) -> Vec<crate::semantics::data_flow::normalization::SemanticOp> {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        crate::semantics::data_flow::normalization::SemanticExtractor::extract(
            tree.root_node(),
            content,
            ext,
        )
    }
}
