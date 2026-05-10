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
    ) -> Result<Vec<crate::semantics::Symbol>> {
        let (language, tree) = match self.parse_source(path, content) {
            Ok(res) => res,
            Err(_) => return Ok(Vec::new()),
        };

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
                // SAFETY: avoid panic on malformed capture indices
                let capture_name = capture_names
                    .get(capture.index as usize)
                    .map(|s| &**s)
                    .unwrap_or("");

                // only process symbol name captures
                if capture_name != "name" {
                    continue;
                }

                let node = capture.node;

                // SAFETY: avoid invalid UTF8 byte slicing
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

                // optional:
                // keep Unknown symbols for future analysis
                // remove this continue if desired
                if kind == crate::semantics::SymbolKind::Unknown {
                    continue;
                }

                symbols.push(crate::semantics::Symbol {
                    name: name.to_string(),
                    kind,
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

    pub fn scan_for_edges(&self, path: &Path, content: &str) -> Result<Vec<(String, String)>> {
        let (language, tree) = match self.parse_source(path, content) {
            Ok(res) => res,
            Err(_) => return Ok(Vec::new()),
        };

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

                // FIX:
                // extract only the callable identifier
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
        registry: &mut SymbolRegistry,
    ) -> Result<()> {
        let (_, tree) = match self.parse_source(path, content) {
            Ok(res) => res,
            Err(_) => return Ok(()),
        };

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

            if let Some(idx) = registry.find_node_at(name, &path_str, line) {
                if let Some(func) = self.find_enclosing_function(node) {
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

        // SAFER TREE TRAVERSAL
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();

                // create isolated cursor per recursion branch
                let mut child_cursor = child.walk();

                self.traverse_link_symbols(child, &mut child_cursor, path, content, registry);

                if !cursor.goto_next_sibling() {
                    break;
                }
            }

            cursor.goto_parent();
        }
    }
}
