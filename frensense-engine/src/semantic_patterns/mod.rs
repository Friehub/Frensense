// SPDX-License-Identifier: MIT

pub mod auth_guard_dominator;
pub mod csrf_missing_token;
pub mod hardcoded_credentials;
pub mod helpers;
pub mod idor_missing_ownership;
pub mod registry;

pub use auth_guard_dominator::MissingAuthGuard;
pub use csrf_missing_token::CsrfMissingToken;
pub use hardcoded_credentials::HardcodedCredentials;
pub use helpers::{
    AncestorIter, collect_calls_in_scope, find_parent_kind, has_ancestor_kind,
    is_inside_transaction, node_text,
};
pub use idor_missing_ownership::IdorMissingOwnershipCheck;
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
