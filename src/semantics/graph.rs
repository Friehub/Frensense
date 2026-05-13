// SPDX-License-Identifier: MIT

use crate::semantics::symbols::Symbol;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

/// The type of relationship between two semantic symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Calls,
    RefersTo,
    OwnedBy,
    Inherits,
    Overrides,
    FlowsFrom,
    SequentiallyFollows,
    InScope,
    Parameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    Acquire,
    Release,
    Await,
    Call,
    Assignment,
    Return,
}

#[derive(Debug, Clone)]
pub struct TemporalEvent {
    pub event_type: EventType,
    pub label: String,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum SemanticNode {
    Declaration(Symbol),
    Event(TemporalEvent),
}

/// A directed graph representing the semantic structure of the codebase.
#[derive(Default)]
pub struct SemanticGraph {
    pub graph: DiGraph<SemanticNode, EdgeKind>,
    pub name_index: HashMap<String, Vec<NodeIndex>>,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> NodeIndex {
        let name = symbol.name.clone();
        let idx = self.graph.add_node(SemanticNode::Declaration(symbol));
        self.name_index.entry(name).or_default().push(idx);
        idx
    }

    pub fn add_event(&mut self, event: TemporalEvent) -> NodeIndex {
        let label = event.label.clone();
        let idx = self.graph.add_node(SemanticNode::Event(event));
        self.name_index.entry(label).or_default().push(idx);
        idx
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, kind: EdgeKind) {
        self.graph.add_edge(from, to, kind);
    }

    pub fn find_nodes(&self, name: &str) -> Vec<NodeIndex> {
        self.name_index.get(name).cloned().unwrap_or_default()
    }

    pub fn get_node(&self, idx: NodeIndex) -> Option<&SemanticNode> {
        self.graph.node_weight(idx)
    }

    pub fn get_symbol(&self, idx: NodeIndex) -> Option<&Symbol> {
        match self.graph.node_weight(idx) {
            Some(SemanticNode::Declaration(s)) => Some(s),
            _ => None,
        }
    }

    pub fn all_symbols(&self) -> Vec<&Symbol> {
        self.graph
            .node_weights()
            .filter_map(|n| {
                if let SemanticNode::Declaration(s) = n {
                    Some(s)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_node(&self, name: &str, file: &str, line: usize) -> Option<NodeIndex> {
        self.find_nodes(name).into_iter().find(|&idx| {
            if let Some(SemanticNode::Declaration(s)) = self.get_node(idx) {
                s.file_path == file && s.line == line
            } else {
                false
            }
        })
    }

    pub fn neighbors_of(&self, idx: NodeIndex, kind: EdgeKind) -> Vec<NodeIndex> {
        self.graph
            .edges(idx)
            .filter(|e| *e.weight() == kind)
            .map(|e| e.target())
            .collect()
    }

    pub fn ordered_events_in_scope(&self, scope_idx: NodeIndex) -> Vec<TemporalEvent> {
        let mut events = Vec::new();
        let event_indices: Vec<NodeIndex> = self
            .neighbors_of(scope_idx, EdgeKind::InScope)
            .into_iter()
            .filter(|&idx| matches!(self.get_node(idx), Some(SemanticNode::Event(_))))
            .collect();

        let event_set: std::collections::HashSet<NodeIndex> =
            event_indices.iter().copied().collect();

        let mut starts: Vec<_> = event_set
            .iter()
            .filter(|&&idx| {
                !self
                    .graph
                    .edges_directed(idx, petgraph::Direction::Incoming)
                    .any(|e| {
                        *e.weight() == EdgeKind::SequentiallyFollows
                            && event_set.contains(&e.source())
                    })
            })
            .copied()
            .collect();

        if starts.len() > 1 {
            starts.sort_by(|a, b| {
                let node_a = self.get_node(*a);
                let node_b = self.get_node(*b);
                match (node_a, node_b) {
                    (Some(SemanticNode::Event(ea)), Some(SemanticNode::Event(eb))) => {
                        ea.line.cmp(&eb.line).then(ea.column.cmp(&eb.column))
                    }
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }
        let mut current = starts.first().copied();
        let mut visited = std::collections::HashSet::new();
        while let Some(idx) = current {
            if !visited.insert(idx) {
                break;
            }
            if let Some(SemanticNode::Event(ev)) = self.get_node(idx) {
                events.push(ev.clone());
            }
            let mut next_edges: Vec<_> = self
                .graph
                .edges(idx)
                .filter(|e| {
                    *e.weight() == EdgeKind::SequentiallyFollows && event_set.contains(&e.target())
                })
                .collect();
            if next_edges.len() > 1 {
                next_edges.sort_by(|a, b| {
                    let node_a = self.get_node(a.target());
                    let node_b = self.get_node(b.target());
                    match (node_a, node_b) {
                        (Some(SemanticNode::Event(ea)), Some(SemanticNode::Event(eb))) => {
                            ea.line.cmp(&eb.line).then(ea.column.cmp(&eb.column))
                        }
                        _ => std::cmp::Ordering::Equal,
                    }
                });
            }
            current = next_edges.first().map(|e| e.target());
        }
        events
    }
}
