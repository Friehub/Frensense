// SPDX-License-Identifier: MIT

use crate::semantics::graph::SemanticGraph;
use crate::Advisory;
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
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
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

    /// Helper to find a specific symbol at a location.
    pub fn find_at(&self, name: &str, file: &str, line: usize) -> Option<&Symbol> {
        self.find(name)
            .into_iter()
            .find(|s| s.file_path == file && s.line == line)
    }

    pub fn find_function_at(&self, file: &str, line: usize) -> Option<NodeIndex> {
        for idx in self.graph.all_nodes() {
            if let Some(sym) = self.graph.get_symbol(idx) {
                if sym.kind == SymbolKind::Function && sym.file_path == file && sym.line == line {
                    return Some(idx);
                }
            }
        }
        None
    }
    pub fn add_call_edge(&mut self, file_path: &Path, caller: &str, callee: &str) {
        let path_str = file_path.to_string_lossy();
        let callers = self.graph.find_nodes(caller);
        let callees = self.graph.find_nodes(callee);

        for &caller_idx in &callers {
            if let Some(sym) = self.graph.get_symbol(caller_idx) {
                if sym.file_path == path_str {
                    for &callee_idx in &callees {
                        self.graph.add_edge(
                            caller_idx,
                            callee_idx,
                            crate::semantics::EdgeKind::Calls,
                        );
                    }
                }
            }
        }
    }
    pub fn check_graph_deadlock(&self) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // Find all functions in the graph
        for idx in self.graph.all_nodes() {
            if let Some(crate::semantics::graph::SemanticNode::Declaration(sym)) =
                self.graph.get_node(idx)
            {
                if sym.kind == crate::semantics::SymbolKind::Function {
                    // Find all events in this function's scope (ORDERED)
                    let events = self.graph.ordered_events_in_scope(idx);

                    // Check for Acquire -> Await sequence
                    let mut lock_held = false;
                    for ev in events {
                        match ev.event_type {
                            crate::semantics::graph::EventType::Acquire => lock_held = true,
                            crate::semantics::graph::EventType::Release => lock_held = false,
                            crate::semantics::graph::EventType::Await if lock_held => {
                                advisories.push(Advisory {
                                    rule_id: "GRAPH_DEADLOCK".to_string(),
                                    severity: crate::Severity::Critical,
                                    observation: format!("Potential Deadlock: Lock '{}' held across await point at {}:{}.", ev.label, ev.line, ev.column),
                                    impact: "May cause permanent thread/task starvation.".to_string(),
                                    improvement: "Release the lock before awaiting or use an async-aware mutex.".to_string(),
                                    line: ev.line,
                                    column: ev.column,
                                    file_path: ev.file_path.clone(),
                                    original_content: "".to_string(),
                                    proposed_replacement: None,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        advisories
    }
}
