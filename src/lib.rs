// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use include_dir::{include_dir, Dir};
use std::path::Path;
use thiserror::Error;
use tree_sitter::{LanguageError, Node};

// --- Module Declarations ---
pub mod engine;
pub mod parser;
pub mod patcher;
pub mod reporter;
pub mod rules;
pub mod semantics;

// --- Re-exports for Public API ---
pub use engine::{AstAuditor, Engine, FunctionFingerprint};
pub use rules::core::CoreRule;
pub use semantics::{DataFlowAnalyzer, Symbol, SymbolKind, SymbolRegistry, TaintRegistry};

/// Static embed of institutional modular redline rules to ensure out-of-the-box functionality.
pub static EMBEDDED_RULES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/rules");

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AuditorEnvironment {
    Production,
    Staging,
    Development,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Advisory {
    pub rule_id: String,
    pub severity: Severity,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub line: usize,
    pub column: usize,
    pub file_path: String,
    /// The original text that triggered the finding (used for verification during patching).
    pub original_content: String,
    /// The suggested replacement code, if any.
    pub proposed_replacement: Option<String>,
}

pub struct AuditContext<'a> {
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub symbols: &'a SymbolRegistry,
}

/// Core Trait: Represents a high-precision semantic audit rule.
pub trait AuditorRule: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> &str {
        "General"
    }
    fn tags(&self) -> Vec<&str> {
        vec![]
    }
    fn query(&self) -> Option<&str> {
        None
    }

    /// Verified: Called when a node matches the query, or during manual traversal.
    fn check(&self, node: Node, context: &AuditContext) -> Vec<Advisory>;

    /// Check: Does this rule apply to the given extension?
    fn applies_to(&self, _ext: &str) -> bool {
        true
    }

    /// DRY Helper: Checks if the file being audited matches the expected extension.
    fn matches_ext(&self, context: &AuditContext, expected: &str) -> bool {
        context.file_path.extension().and_then(|s| s.to_str()) == Some(expected)
    }

    /// DRY Helper: Creates a standard narrative advisory.
    fn new_advisory(
        &self,
        node: &Node,
        observation: String,
        impact: String,
        improvement: String,
    ) -> Advisory {
        let pos = node.start_position();
        Advisory {
            rule_id: self.id().to_string(),
            severity: self.severity(),
            observation,
            impact,
            improvement,
            line: pos.row + 1,
            column: pos.column + 1,
            file_path: String::new(),
            original_content: String::new(),
            proposed_replacement: None,
        }
    }

    fn new_critical_advisory(
        &self,
        node: &Node,
        observation: String,
        impact: String,
        improvement: String,
    ) -> Advisory {
        let mut adv = self.new_advisory(node, observation, impact, improvement);
        adv.severity = Severity::Critical;
        adv
    }

    /// Helper to create a remediated advisory.
    fn new_remediated_advisory(
        &self,
        node: &Node,
        observation: String,
        impact: String,
        improvement: String,
        original_content: String,
        proposed_replacement: String,
    ) -> Advisory {
        let mut advisory = self.new_advisory(node, observation, impact, improvement);
        advisory.original_content = original_content;
        advisory.proposed_replacement = Some(proposed_replacement);
        advisory
    }
}

#[derive(Error, Debug)]
pub enum AuditorError {
    #[error("Tree-sitter parse failure for {0}")]
    ParseFailure(String),
    #[error("Tree-sitter language error: {0}")]
    LanguageError(#[from] LanguageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, AuditorError>;
