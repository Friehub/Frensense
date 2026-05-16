// SPDX-License-Identifier: MIT

use crate::semantics::symbols::Symbol;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticNodeId(pub(crate) NodeIndex);

/// A directed graph representing the semantic structure of the codebase.
#[derive(Default)]
pub struct SemanticGraph {
    graph: DiGraph<SemanticNode, EdgeKind>,
    name_index: HashMap<String, Vec<NodeIndex>>,
}

impl SemanticGraph {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> SemanticNodeId {
        let name = symbol.name.clone();
        let idx = self.graph.add_node(SemanticNode::Declaration(symbol));
        self.name_index.entry(name).or_default().push(idx);
        SemanticNodeId(idx)
    }

    pub fn add_event(&mut self, event: TemporalEvent) -> SemanticNodeId {
        let label = event.label.clone();
        let idx = self.graph.add_node(SemanticNode::Event(event));
        self.name_index.entry(label).or_default().push(idx);
        SemanticNodeId(idx)
    }

    pub fn add_edge(&mut self, from: SemanticNodeId, to: SemanticNodeId, kind: EdgeKind) {
        self.graph.add_edge(from.0, to.0, kind);
    }

    #[must_use]
    pub fn find_nodes(&self, name: &str) -> Vec<SemanticNodeId> {
        self.name_index
            .get(name)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(SemanticNodeId)
            .collect()
    }

    #[must_use]
    pub fn get_node(&self, id: SemanticNodeId) -> Option<&SemanticNode> {
        self.graph.node_weight(id.0)
    }

    #[must_use]
    pub fn get_symbol(&self, id: SemanticNodeId) -> Option<&Symbol> {
        match self.graph.node_weight(id.0) {
            Some(SemanticNode::Declaration(s)) => Some(s),
            _ => None,
        }
    }

    #[must_use]
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

    #[must_use]
    pub fn find_node(&self, name: &str, file: &str, line: usize) -> Option<SemanticNodeId> {
        self.name_index
            .get(name)
            .and_then(|indices| {
                indices.iter().find(|&&idx| {
                    if let Some(SemanticNode::Declaration(s)) = self.get_node(SemanticNodeId(idx)) {
                        s.file_path == file && s.line == line
                    } else {
                        false
                    }
                })
            })
            .copied()
            .map(SemanticNodeId)
    }

    #[must_use]
    pub fn neighbors_of(&self, id: SemanticNodeId, kind: EdgeKind) -> Vec<SemanticNodeId> {
        self.graph
            .edges(id.0)
            .filter(|e| *e.weight() == kind)
            .map(|e| SemanticNodeId(e.target()))
            .collect()
    }

    #[must_use]
    pub fn incoming_neighbors_of(&self, id: SemanticNodeId, kind: EdgeKind) -> Vec<SemanticNodeId> {
        self.graph
            .edges_directed(id.0, petgraph::Direction::Incoming)
            .filter(|e| *e.weight() == kind)
            .map(|e| SemanticNodeId(e.source()))
            .collect()
    }

    #[must_use]
    pub fn ordered_events_in_scope(&self, scope_id: SemanticNodeId) -> Vec<TemporalEvent> {
        let mut events = Vec::new();
        let event_ids: Vec<SemanticNodeId> = self
            .neighbors_of(scope_id, EdgeKind::InScope)
            .into_iter()
            .filter(|&id| matches!(self.get_node(id), Some(SemanticNode::Event(_))))
            .collect();

        let event_set: std::collections::HashSet<NodeIndex> =
            event_ids.iter().map(|id| id.0).collect();

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
                let node_a = self.get_node(SemanticNodeId(*a));
                let node_b = self.get_node(SemanticNodeId(*b));
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
            if let Some(SemanticNode::Event(ev)) = self.get_node(SemanticNodeId(idx)) {
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
                    let node_a = self.get_node(SemanticNodeId(a.target()));
                    let node_b = self.get_node(SemanticNodeId(b.target()));
                    match (node_a, node_b) {
                        (Some(SemanticNode::Event(ea)), Some(SemanticNode::Event(eb))) => {
                            ea.line.cmp(&eb.line).then(ea.column.cmp(&eb.column))
                        }
                        _ => std::cmp::Ordering::Equal,
                    }
                });
            }
            current = next_edges.first().map(petgraph::visit::EdgeRef::target);
        }
        events
    }

    #[must_use]
    pub fn has_call_path(&self, from_name: &str, to_name: &str) -> bool {
        let from_nodes = self.find_nodes(from_name);
        let to_nodes: HashSet<_> = self
            .find_nodes(to_name)
            .into_iter()
            .map(|id| id.0)
            .collect();

        for from in from_nodes {
            let mut visited = HashSet::new();
            let mut stack = vec![from.0];
            visited.insert(from.0);

            while let Some(current) = stack.pop() {
                if to_nodes.contains(&current) {
                    return true;
                }

                for edge in self.graph.edges(current) {
                    if *edge.weight() == EdgeKind::Calls && visited.insert(edge.target()) {
                        stack.push(edge.target());
                    }
                }
            }
        }
        false
    }
}
