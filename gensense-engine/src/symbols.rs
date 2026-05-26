// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;

use crate::graph::{EdgeKind, SemanticGraph, SemanticNodeId};
use crate::FileId;

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Function,
    Struct,
    Class,
    Interface,
    Enum,
    Constant,
    Module,
    Variable,
    Parameter,
    Unknown,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub start_byte: usize,
    pub end_byte: usize,
    pub file_path: String,
    pub file_id: FileId,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolRegistry {
    graph: SemanticGraph,
    file_index: HashMap<String, Vec<SemanticNodeId>>,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn graph(&self) -> &SemanticGraph {
        &self.graph
    }

    pub fn insert(&mut self, symbol: Symbol) -> SemanticNodeId {
        let file_path = symbol.file_path.clone();
        let id = self.graph.add_symbol(symbol);
        self.file_index.entry(file_path).or_default().push(id);
        id
    }

    pub fn find(&self, name: &str) -> Vec<&Symbol> {
        self.graph
            .find_nodes(name)
            .into_iter()
            .filter_map(|id| self.graph.get_symbol(id))
            .collect()
    }

    pub fn find_at(&self, name: &str, file: &str, line: usize) -> Option<&Symbol> {
        self.file_index
            .get(file)?
            .iter()
            .filter_map(|&id| {
                let s = self.graph.get_symbol(id)?;
                if s.name == name && line >= s.line && line <= s.end_line {
                    Some(s)
                } else {
                    None
                }
            })
            .min_by_key(|s| s.end_line - s.line)
    }

    pub fn find_function_at(&self, file: &str, line: usize) -> Option<SemanticNodeId> {
        self.file_index
            .get(file)?
            .iter()
            .filter(|&&id| {
                self.graph.get_symbol(id).is_some_and(|s| {
                    s.kind == SymbolKind::Function && line >= s.line && line <= s.end_line
                })
            })
            .min_by_key(|&&id| {
                self.graph
                    .get_symbol(id)
                    .map_or(usize::MAX, |s| s.end_line - s.line)
            })
            .copied()
    }

    pub fn get_symbol(&self, id: SemanticNodeId) -> Option<&Symbol> {
        self.graph.get_symbol(id)
    }

    pub fn query_all(&self) -> Vec<&Symbol> {
        self.graph.all_symbols()
    }

    pub fn clear(&mut self) {
        self.graph = SemanticGraph::default();
        self.file_index.clear();
    }

    pub fn add_call_edge(&mut self, file_path: &Path, src_name: &str, target_name: &str) {
        let file_str = file_path.display().to_string();
        let src_symbols = self.find(src_name);
        let target_symbols = self.find(target_name);

        let src_node_id = src_symbols
            .iter()
            .find(|s| s.file_path == file_str)
            .and_then(|s| self.graph.find_node(&s.name, &s.file_path, s.line));

        let target_node_id = if let Some(local) = target_symbols.iter().find(|s| s.file_path == file_str) {
            self.graph.find_node(&local.name, &local.file_path, local.line)
        } else if target_symbols.len() == 1 {
            self.graph.find_node(&target_symbols[0].name, &target_symbols[0].file_path, target_symbols[0].line)
        } else {
            None
        };

        if let (Some(s_id), Some(t_id)) = (src_node_id, target_node_id) {
            self.graph.add_edge(s_id, t_id, EdgeKind::Calls);
        }
    }

    pub fn get_callees(&self, sym: &Symbol) -> Vec<&Symbol> {
        let Some(id) = self.graph.find_node(&sym.name, &sym.file_path, sym.line) else {
            return Vec::new();
        };
        self.graph
            .neighbors_of(id, EdgeKind::Calls)
            .into_iter()
            .filter_map(|i| self.graph.get_symbol(i))
            .collect()
    }

    pub fn get_callers(&self, sym: &Symbol) -> Vec<&Symbol> {
        let Some(id) = self.graph.find_node(&sym.name, &sym.file_path, sym.line) else {
            return Vec::new();
        };
        self.graph
            .incoming_neighbors_of(id, EdgeKind::Calls)
            .into_iter()
            .filter_map(|i| self.graph.get_symbol(i))
            .collect()
    }

    pub fn graph_mut(&mut self) -> &mut SemanticGraph {
        &mut self.graph
    }

    pub fn find_callers(&self, name: &str) -> Vec<&Symbol> {
        let nodes = self.graph.find_nodes(name);
        let mut callers = Vec::new();
        for node in nodes {
            let caller_ids = self.graph.incoming_neighbors_of(node, EdgeKind::Calls);
            for id in caller_ids {
                if let Some(s) = self.graph.get_symbol(id) {
                    callers.push(s);
                }
            }
        }
        callers
    }

    /// Extract symbols from a tree-sitter tree using the given symbol query.
    pub fn extract_from_tree(
        &mut self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
        file_id: FileId,
        query_str: &str,
    ) {
        use tree_sitter::Query;
        let lang = crate::parser::ParserRegistry::get_language(file_path).ok();
        let Some(lang) = lang else { return };
        let Ok(query) = Query::new(&lang, query_str) else { return };
        let mut cursor = tree_sitter::QueryCursor::new();
        let file_str = file_path.to_string_lossy().to_string();

        for m in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "name" {
                    let node = capture.node;
                    let name = &source[node.start_byte()..node.end_byte()];
                    let symbol = Symbol {
                        name: name.to_string(),
                        kind: SymbolKind::Function,
                        start_byte: node.start_byte(),
                        end_byte: node.end_byte(),
                        file_path: file_str.clone(),
                        file_id,
                        line: node.start_position().row + 1,
                        column: node.start_position().column + 1,
                        end_line: node.end_position().row + 1,
                    };
                    self.insert(symbol);
                }
            }
        }
    }

    /// Extract call edges from a tree-sitter tree using the given call query.
    pub fn extract_edges_from_tree(
        &mut self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
        query_str: &str,
    ) {
        use tree_sitter::Query;
        let lang = crate::parser::ParserRegistry::get_language(file_path).ok();
        let Some(lang) = lang else { return };
        let Ok(query) = Query::new(&lang, query_str) else { return };
        let mut cursor = tree_sitter::QueryCursor::new();

        for m in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "call" {
                    let node = capture.node;
                    let name = &source[node.start_byte()..node.end_byte()];
                    self.add_call_edge(file_path, &name, &name);
                }
            }
        }
    }
}
