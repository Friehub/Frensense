// SPDX-License-Identifier: MIT
#![allow(clippy::all)]

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
#[cfg(feature = "fingerprinting")]
pub use engine::FunctionFingerprint;
pub use engine::{Engine, GenSenseAuditor};
pub use rules::core::CoreRule;
pub use semantics::{DataFlowAnalyzer, Symbol, SymbolKind, SymbolRegistry, TaintRegistry};

/// Current core version of the GenSense engine.
#[cfg(feature = "node")]
pub mod js;

pub const GENSENSE_VERSION: &str = "0.1.7";

/// Static embed of standardized modular safety rules to ensure out-of-the-box functionality.
pub static EMBEDDED_RULES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/rules");

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenSenseEnvironment {
    Production,
    Staging,
    Development,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
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

pub struct GenSenseContext<'a> {
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub symbols: &'a SymbolRegistry,
}

/// Core Trait: Represents a high-precision semantic GenSense rule.
pub trait GenSenseRule: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> &str {
        "General"
    }
    fn tags(&self) -> Vec<&str> {
        vec![]
    }
    fn impact(&self) -> &str {
        ""
    }
    fn improvement(&self) -> &str {
        ""
    }
    fn query(&self) -> Option<&str> {
        None
    }

    /// Verified: Called when a node matches the query, or during manual traversal.
    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory>;

    /// Check: Does this rule apply to the given extension?
    fn applies_to(&self, _ext: &str) -> bool {
        true
    }

    /// DRY Helper: Checks if the file being audited matches the expected extension.
    fn matches_ext(&self, context: &GenSenseContext, expected: &str) -> bool {
        context.file_path.extension().and_then(|s| s.to_str()) == Some(expected)
    }

    /// DRY Helper: Create a new advisory for this rule.
    fn new_advisory(
        &self,
        node: &Node,
        observation: String,
        impact: String,
        improvement: String,
    ) -> Advisory {
        Advisory {
            rule_id: self.id().to_string(),
            severity: self.severity(),
            observation,
            impact,
            improvement,
            line: node.start_position().row + 1,
            column: node.start_position().column + 1,
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
pub enum GenSenseError {
    #[error("Tree-sitter parse failure for {0}")]
    ParseFailure(String),
    #[error("Tree-sitter language error: {0}")]
    LanguageError(#[from] LanguageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, GenSenseError>;
