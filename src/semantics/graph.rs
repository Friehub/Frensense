// SPDX-License-Identifier: MIT

use crate::semantics::symbols::Symbol;
use petgraph::graph::{DiGraph, NodeIndex};
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
    SequentiallyFollows, // NEW: For temporal order
    InScope,             // NEW: Linking events to symbols
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    Acquire,
    Release,
    Await,
    Call,
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
/// This enables advanced reasoning like "who calls this?" or "is this variable tainted?".
#[derive(Default)]
pub struct SemanticGraph {
    pub graph: DiGraph<SemanticNode, EdgeKind>,
    pub name_index: HashMap<String, Vec<NodeIndex>>,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a symbol to the graph and updates the name index.
    pub fn add_symbol(&mut self, symbol: Symbol) -> NodeIndex {
        let name = symbol.name.clone();
        let idx = self.graph.add_node(SemanticNode::Declaration(symbol));
        self.name_index.entry(name).or_default().push(idx);
        idx
    }

    pub fn add_event(&mut self, event: TemporalEvent) -> NodeIndex {
        self.graph.add_node(SemanticNode::Event(event))
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

    /// Returns all node indices in the graph.
    pub fn all_nodes(&self) -> Vec<NodeIndex> {
        self.graph.node_indices().collect()
    }

    /// Returns neighbors of a node given a relationship kind.
    pub fn neighbors_of(&self, idx: NodeIndex, kind: EdgeKind) -> Vec<NodeIndex> {
        self.graph
            .edges(idx)
            .filter(|e| *e.weight() == kind)
            .map(|e| e.target())
            .collect()
    }

    /// Returns all events in a scope, ordered by their execution sequence.
    pub fn ordered_events_in_scope(&self, scope_idx: NodeIndex) -> Vec<TemporalEvent> {
        let mut events = Vec::new();
        let event_indices = self.neighbors_of(scope_idx, EdgeKind::InScope);

        // Find the start of the sequence within this scope
        let mut current = None;
        for &idx in &event_indices {
            let has_prev_in_scope = self
                .graph
                .edges_directed(idx, petgraph::Direction::Incoming)
                .any(|e| {
                    *e.weight() == EdgeKind::SequentiallyFollows
                        && event_indices.contains(&e.source())
                });

            if !has_prev_in_scope {
                current = Some(idx);
                break;
            }
        }

        // Traverse the sequence
        let mut visited = std::collections::HashSet::new();
        while let Some(idx) = current {
            if !visited.insert(idx) {
                break;
            } // Prevent cycles

            if let Some(SemanticNode::Event(ev)) = self.get_node(idx) {
                events.push(ev.clone());
            }

            current = self
                .graph
                .edges(idx)
                .find(|e| {
                    *e.weight() == EdgeKind::SequentiallyFollows
                        && event_indices.contains(&e.target())
                })
                .map(|e| e.target());
        }

        events
    }
}

use petgraph::visit::EdgeRef;
