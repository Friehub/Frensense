// SPDX-License-Identifier: MIT

use crate::semantics::graph::SemanticGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
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
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
}

#[derive(Default)]
pub struct SymbolRegistry {
    pub graph: SemanticGraph,
    pub file_index: HashMap<String, Vec<NodeIndex>>,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, symbol: Symbol) -> NodeIndex {
        let file_path = symbol.file_path.clone();
        let idx = self.graph.add_symbol(symbol);

        self.file_index.entry(file_path).or_default().push(idx);
        idx
    }

    pub fn find(&self, name: &str) -> Vec<&Symbol> {
        self.graph
            .find_nodes(name)
            .into_iter()
            .filter_map(|idx| self.graph.get_symbol(idx))
            .collect()
    }

    /// Finds the innermost symbol with a specific name at a specific location.
    /// This handles variable shadowing by selecting the smallest containing interval.
    pub fn find_at(&self, name: &str, file: &str, line: usize) -> Option<&Symbol> {
        self.file_index
            .get(file)?
            .iter()
            .filter_map(|&idx| {
                let s = self.graph.get_symbol(idx)?;
                if s.name == name && line >= s.line && line <= s.end_line {
                    Some(s)
                } else {
                    None
                }
            })
            .min_by_key(|s| s.end_line - s.line)
    }

    /// Finds the innermost function containing the specified line.
    pub fn find_function_at(&self, file: &str, line: usize) -> Option<NodeIndex> {
        self.file_index
            .get(file)?
            .iter()
            .filter(|&&idx| {
                if let Some(s) = self.graph.get_symbol(idx) {
                    s.kind == SymbolKind::Function && line >= s.line && line <= s.end_line
                } else {
                    false
                }
            })
            .min_by_key(|&&idx| {
                let s = self
                    .graph
                    .get_symbol(idx)
                    .expect("Symbol missing after filter");
                s.end_line - s.line
            })
            .copied()
    }

    pub fn query_all(&self) -> Vec<&Symbol> {
        self.graph.all_symbols()
    }

    pub fn clear(&mut self) {
        self.graph = SemanticGraph::default();
        self.file_index.clear();
    }

    pub fn add_call_edge(&mut self, file_path: &Path, caller: &str, callee: &str) {
        let file_str = file_path.display().to_string();
        let callers = self.find(caller);
        let callees = self.find(callee);

        // 1. Resolve Caller: Must be in the current file
        let caller_idx = callers
            .iter()
            .find(|s| s.file_path == file_str)
            .and_then(|s| self.graph.find_node(&s.name, &s.file_path, s.line));

        // 2. Resolve Callee: Priority is local file, then unambiguous global
        let callee_idx = if let Some(local) = callees.iter().find(|s| s.file_path == file_str) {
            self.graph
                .find_node(&local.name, &local.file_path, local.line)
        } else if callees.len() == 1 {
            self.graph
                .find_node(&callees[0].name, &callees[0].file_path, callees[0].line)
        } else {
            // Still ambiguous or external - in a full implementation, we'd use imports/usings
            None
        };

        if let (Some(c1), Some(c2)) = (caller_idx, callee_idx) {
            self.graph
                .add_edge(c1, c2, crate::semantics::graph::EdgeKind::Calls);
        }
    }

    pub fn find_by_regex(&self, re: &regex::Regex) -> Vec<&Symbol> {
        self.graph
            .all_symbols()
            .into_iter()
            .filter(|s| re.is_match(&s.name))
            .collect()
    }

    pub fn get_callees(&self, sym: &Symbol) -> Vec<&Symbol> {
        let idx = match self.graph.find_node(&sym.name, &sym.file_path, sym.line) {
            Some(i) => i,
            None => return Vec::new(),
        };

        self.graph
            .neighbors_of(idx, crate::semantics::graph::EdgeKind::Calls)
            .into_iter()
            .filter_map(|i| self.graph.get_symbol(i))
            .collect()
    }

    pub fn get_callers(&self, sym: &Symbol) -> Vec<&Symbol> {
        let idx = match self.graph.find_node(&sym.name, &sym.file_path, sym.line) {
            Some(i) => i,
            None => return Vec::new(),
        };

        self.graph
            .graph
            .edges_directed(idx, petgraph::Direction::Incoming)
            .filter(|e| *e.weight() == crate::semantics::graph::EdgeKind::Calls)
            .filter_map(|e| self.graph.get_symbol(e.source()))
            .collect()
    }
}
