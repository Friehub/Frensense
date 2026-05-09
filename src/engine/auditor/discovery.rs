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
        let (_, tree) = self.parse_source(path, content)?;
        let query_str = match ParserRegistry::get_symbol_query(path) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };

        let language = ParserRegistry::get_language(path)?;
        let query =
            Query::new(&language, query_str).map_err(|e| GenSenseError::Config(e.to_string()))?;
        let mut cursor = QueryCursor::new();
        let mut symbols = Vec::new();
        let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        let capture_names = query.capture_names();
        for m in matches {
            for capture in m.captures {
                let capture_name = &capture_names[capture.index as usize];
                let kind = match *capture_name {
                    "name" => {
                        let parent = capture.node.parent();
                        match parent.map(|p| p.kind()) {
                            Some("function_item") | Some("function_declaration") => {
                                crate::semantics::SymbolKind::Function
                            }
                            Some("parameter") => crate::semantics::SymbolKind::Parameter,
                            Some("let_declaration") | Some("variable_declarator") => {
                                crate::semantics::SymbolKind::Variable
                            }
                            Some("struct_item") => crate::semantics::SymbolKind::Struct,
                            Some("enum_item") => crate::semantics::SymbolKind::Enum,
                            Some("trait_item") => crate::semantics::SymbolKind::Interface,
                            Some("const_item") => crate::semantics::SymbolKind::Constant,
                            _ => crate::semantics::SymbolKind::Unknown,
                        }
                    }
                    _ => crate::semantics::SymbolKind::Unknown,
                };

                if kind == crate::semantics::SymbolKind::Unknown {
                    continue;
                }

                let name = &content[capture.node.start_byte()..capture.node.end_byte()];
                symbols.push(crate::semantics::Symbol {
                    name: name.to_string(),
                    kind,
                    line: capture.node.start_position().row + 1,
                    column: capture.node.start_position().column + 1,
                    file_path: path.to_string_lossy().to_string(),
                });
            }
        }
        Ok(symbols)
    }

    pub fn scan_for_edges(&self, path: &Path, content: &str) -> Result<Vec<(String, String)>> {
        let (language, tree) = self.parse_source(path, content)?;
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
                let call_name = &content[call_node.start_byte()..call_node.end_byte()];

                if let Some(p) = self.find_enclosing_function(call_node) {
                    if let Some(name_node) = p.child_by_field_name("name") {
                        let caller_name = &content[name_node.start_byte()..name_node.end_byte()];
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
        let (_, tree) = self.parse_source(path, content)?;
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
        let name_node = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("pattern"));

        if let Some(name_node) = name_node {
            let name = &content[name_node.start_byte()..name_node.end_byte()];
            let line = name_node.start_position().row + 1;

            if let Some(idx) = registry.find_node_at(name, &path.to_string_lossy(), line) {
                if let Some(func) = self.find_enclosing_function(node) {
                    if let Some(fname_node) = func.child_by_field_name("name") {
                        let fname = &content[fname_node.start_byte()..fname_node.end_byte()];
                        for &f_idx in &registry.graph.find_nodes(fname) {
                            if let Some(fsym) = registry.graph.get_symbol(f_idx) {
                                if fsym.file_path == path.to_string_lossy() {
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
                self.traverse_link_symbols(cursor.node(), cursor, path, content, registry);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }
}
