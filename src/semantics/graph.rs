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
    Parameter,           // NEW: Linking function to its parameters
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
    ///
    /// FIX (Bug 1 — two changes):
    ///
    /// 1. Start-node search: the original code used `break` after the first
    ///    node with no incoming in-scope predecessor. If the graph has
    ///    out-of-order insertions (which nested blocks produce), this can pick
    ///    a mid-sequence node as the "start", silently truncating the chain.
    ///    Fixed by scanning ALL nodes with `Iterator::find` — no early break.
    ///
    /// 2. Walk loop: the original code called `event_indices.contains()` where
    ///    `event_indices` is a Vec, making each check O(n) and the full
    ///    traversal O(n²). Fixed by using `event_set` (HashSet) which is
    ///    already built above, giving O(1) per check.
    pub fn ordered_events_in_scope(&self, scope_idx: NodeIndex) -> Vec<TemporalEvent> {
        let mut events = Vec::new();
        let event_indices: Vec<NodeIndex> = self
            .neighbors_of(scope_idx, EdgeKind::InScope)
            .into_iter()
            .filter(|&idx| matches!(self.get_node(idx), Some(SemanticNode::Event(_))))
            .collect();

        // Build a HashSet for O(1) membership checks throughout this function.
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

        // Traverse the sequence from start to end.
        let mut visited = std::collections::HashSet::new();
        while let Some(idx) = current {
            if !visited.insert(idx) {
                break; // Cycle guard
            }

            if let Some(SemanticNode::Event(ev)) = self.get_node(idx) {
                events.push(ev.clone());
            }

            // FIX 2: Use event_set (HashSet, O(1)) instead of
            // event_indices (Vec, O(n)) for the membership check.
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

use petgraph::visit::EdgeRef;
