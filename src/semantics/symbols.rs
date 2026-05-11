// SPDX-License-Identifier: MIT

use crate::semantics::graph::SemanticGraph;
use petgraph::graph::NodeIndex;
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
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
}

#[derive(Default)]
pub struct SymbolRegistry {
    pub graph: SemanticGraph,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, symbol: Symbol) -> NodeIndex {
        self.graph.add_symbol(symbol)
    }

    pub fn find(&self, name: &str) -> Vec<&Symbol> {
        self.graph
            .find_nodes(name)
            .into_iter()
            .filter_map(|idx| self.graph.get_symbol(idx))
            .collect()
    }

    pub fn find_at(&self, name: &str, file: &str, line: usize) -> Option<&Symbol> {
        self.find(name).into_iter().find(|s| {
            s.file_path == file && (s.line == line || (line >= s.line && line <= s.end_line))
        })
    }

    pub fn find_function_at(&self, file: &str, line: usize) -> Option<NodeIndex> {
        self.graph.find_node_at_line(file, line)
    }

    pub fn query_all(&self) -> Vec<&Symbol> {
        self.graph.all_symbols()
    }

    pub fn clear(&mut self) {
        self.graph = SemanticGraph::default();
    }

    pub fn add_call_edge(&mut self, file_path: &Path, caller: &str, callee: &str) {
        let file_str = file_path.display().to_string();
        let callers = self.find(caller);
        let callees = self.find(callee);

        let caller_idx = callers
            .iter()
            .find(|s| s.file_path == file_str)
            .and_then(|s| self.graph.find_node(&s.name, &s.file_path, s.line));

        let callee_idx = if callees.len() == 1 {
            self.graph
                .find_node(&callees[0].name, &callees[0].file_path, callees[0].line)
        } else {
            None
        };

        if let (Some(c1), Some(c2)) = (caller_idx, callee_idx) {
            self.graph
                .add_edge(c1, c2, crate::semantics::graph::EdgeKind::Calls);
        }
    }
}
