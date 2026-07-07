// SPDX-License-Identifier: MIT

pub mod helpers;
pub mod registry;

pub use helpers::{
    AncestorIter, collect_calls_in_scope, find_parent_kind, has_ancestor_kind,
    is_inside_transaction, node_text,
};
pub use registry::{PatternRegistry, PatternRunner, SemanticPattern};

/// Result of a semantic pattern match.
#[derive(Debug, Clone)]
pub struct PatternFinding {
    pub pattern_id: String,
    pub severity: String,
    pub line: usize,
    pub column: usize,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub confidence: f64,
    pub tags: Vec<String>,
    pub enclosing_function: Option<String>,
}
