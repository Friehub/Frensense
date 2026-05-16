// SPDX-License-Identifier: MIT

use crate::semantics::graph::{SemanticGraph, SemanticNodeId};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub start_byte: usize,
    pub end_byte: usize,
    pub file_path: String,
    pub file_id: crate::FileId,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
}

#[derive(Default)]
pub struct SymbolRegistry {
    graph: SemanticGraph,
    file_index: HashMap<String, Vec<SemanticNodeId>>,
}

impl SymbolRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn graph(&self) -> &SemanticGraph {
        &self.graph
    }

    pub fn insert(&mut self, symbol: Symbol) -> SemanticNodeId {
        let file_path = symbol.file_path.clone();
        let id = self.graph.add_symbol(symbol);

        self.file_index.entry(file_path).or_default().push(id);
        id
    }

    #[must_use]
    pub fn find(&self, name: &str) -> Vec<&Symbol> {
        self.graph
            .find_nodes(name)
            .into_iter()
            .filter_map(|id| self.graph.get_symbol(id))
            .collect()
    }

    /// Finds the innermost symbol with a specific name at a specific location.
    /// This handles variable shadowing by selecting the smallest containing interval.
    #[must_use]
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

    /// Finds the innermost function containing the specified line.
    #[must_use]
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

    #[must_use]
    pub fn get_symbol(&self, id: SemanticNodeId) -> Option<&Symbol> {
        self.graph.get_symbol(id)
    }

    #[must_use]
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

        // 1. Resolve Caller: Must be in the current file
        let src_node_id = src_symbols
            .iter()
            .find(|s| s.file_path == file_str)
            .and_then(|s| self.graph.find_node(&s.name, &s.file_path, s.line));

        // 2. Resolve Callee: Priority is local file, then unambiguous global
        let target_node_id =
            if let Some(local) = target_symbols.iter().find(|s| s.file_path == file_str) {
                self.graph
                    .find_node(&local.name, &local.file_path, local.line)
            } else if target_symbols.len() == 1 {
                self.graph.find_node(
                    &target_symbols[0].name,
                    &target_symbols[0].file_path,
                    target_symbols[0].line,
                )
            } else {
                // Still ambiguous or external - in a full implementation, we'd use imports/usings
                None
            };

        if let (Some(s_id), Some(t_id)) = (src_node_id, target_node_id) {
            self.graph
                .add_edge(s_id, t_id, crate::semantics::graph::EdgeKind::Calls);
        }
    }

    #[must_use]
    pub fn find_by_regex(&self, re: &regex::Regex) -> Vec<&Symbol> {
        self.graph
            .all_symbols()
            .into_iter()
            .filter(|s| re.is_match(&s.name))
            .collect()
    }

    #[must_use]
    pub fn get_callees(&self, sym: &Symbol) -> Vec<&Symbol> {
        let Some(id) = self.graph.find_node(&sym.name, &sym.file_path, sym.line) else {
            return Vec::new();
        };

        self.graph
            .neighbors_of(id, crate::semantics::graph::EdgeKind::Calls)
            .into_iter()
            .filter_map(|i| self.graph.get_symbol(i))
            .collect()
    }

    #[must_use]
    pub fn get_callers(&self, sym: &Symbol) -> Vec<&Symbol> {
        let Some(id) = self.graph.find_node(&sym.name, &sym.file_path, sym.line) else {
            return Vec::new();
        };

        self.graph
            .incoming_neighbors_of(id, crate::semantics::graph::EdgeKind::Calls)
            .into_iter()
            .filter_map(|i| self.graph.get_symbol(i))
            .collect()
    }
    #[must_use]
    pub const fn graph_mut(&mut self) -> &mut SemanticGraph {
        &mut self.graph
    }

    #[must_use]
    pub fn find_callers(&self, name: &str) -> Vec<&Symbol> {
        let nodes = self.graph.find_nodes(name);
        let mut callers = Vec::new();
        for node in nodes {
            let caller_ids = self
                .graph
                .incoming_neighbors_of(node, crate::semantics::graph::EdgeKind::Calls);
            for id in caller_ids {
                if let Some(s) = self.graph.get_symbol(id) {
                    callers.push(s);
                }
            }
        }
        callers
    }
}
