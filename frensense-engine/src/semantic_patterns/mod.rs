// SPDX-License-Identifier: MIT

pub mod check_then_act;
pub mod helpers;
pub mod registry;

pub use helpers::{has_ancestor_kind, node_text, find_parent_kind, is_inside_transaction, collect_calls_in_scope, AncestorIter};
pub use registry::{SemanticPattern, PatternRegistry, PatternRunner};

use tree_sitter::Node;

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
