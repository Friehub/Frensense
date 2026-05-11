// SPDX-License-Identifier: MIT

use include_dir::{include_dir, Dir};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;
use tree_sitter::Node;

pub static EMBEDDED_RULES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/rules");
pub const GENSENSE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod engine;
pub mod parser;
pub mod patcher;
pub mod reporter;
pub mod rules;
pub mod semantics;

pub use crate::engine::auditor::GenSenseAuditor;
pub use crate::engine::Engine;

use crate::semantics::SymbolRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FileId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ScopeId(pub u64);

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuleMetadata {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub severity: Severity,
    pub impact: Cow<'static, str>,
    pub improvement: Cow<'static, str>,
    pub tags: Vec<Cow<'static, str>>,
    pub category: Cow<'static, str>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Advisory {
    pub rule_id: String,
    pub file_id: FileId,
    pub file_path: String,
    pub severity: Severity,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub line: u32,
    pub column: u32,
    pub start_byte: u32,
    pub end_byte: u32,
    pub original_content: String,
    /// The suggested replacement code, if any.
    pub proposed_replacement: Option<String>,
}

pub type TaintCache = RefCell<HashMap<(String, String, usize), Vec<Advisory>>>;

pub struct GenSenseContext<'a> {
    pub file_id: FileId,
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub symbols: &'a SymbolRegistry,
    pub semantic_ops: &'a [crate::semantics::data_flow::normalization::SemanticOp],
    pub taint_cache: &'a TaintCache,
}

/// Core Trait: Represents a high-precision semantic GenSense rule.
pub trait GenSenseRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;

    /// The core logic for verifying a finding.
    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory>;

    /// Helper to get rule ID
    fn id(&self) -> &str {
        self.metadata().id.as_ref()
    }

    /// Helper to check file applicability
    fn applies_to(&self, extension: &str) -> bool;

    /// Helper for query-based matching
    fn query(&self) -> Option<&str> {
        None
    }

    /// DRY Helper: Create a new advisory for this rule.
    fn new_advisory(
        &self,
        node: &Node,
        context: &GenSenseContext,
        observation: String,
    ) -> Advisory {
        let meta = self.metadata();
        Advisory {
            rule_id: meta.id.to_string(),
            file_id: context.file_id,
            file_path: context.file_path.to_string_lossy().to_string(),
            severity: meta.severity,
            observation,
            impact: meta.impact.to_string(),
            improvement: meta.improvement.to_string(),
            line: (node.start_position().row + 1) as u32,
            column: (node.start_position().column + 1) as u32,
            start_byte: node.start_byte() as u32,
            end_byte: node.end_byte() as u32,
            original_content: context.source_code[node.start_byte()..node.end_byte()].to_string(),
            proposed_replacement: None,
        }
    }

    fn new_remediated_advisory(
        &self,
        node: &Node,
        context: &GenSenseContext,
        observation: String,
        replacement: String,
    ) -> Advisory {
        let mut adv = self.new_advisory(node, context, observation);
        adv.proposed_replacement = Some(replacement);
        adv
    }
}

pub use crate::engine::source::SourceRegistry;

/// A project-level rule that operates across all files simultaneously.
/// Receives the fully assembled, immutable SymbolRegistry and SourceRegistry.
pub trait ProjectRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;

    /// The core logic. Receives the complete project graph (read-only).
    fn check_project(&self, symbols: &SymbolRegistry, sources: &SourceRegistry) -> Vec<Advisory>;

    fn id(&self) -> &str {
        self.metadata().id.as_ref()
    }

    fn is_enabled_in(&self, env: GenSenseEnvironment) -> bool {
        let meta = self.metadata();
        if env == GenSenseEnvironment::Production {
            return !meta.tags.iter().any(|t| t == "beta");
        }
        true
    }
}

#[derive(Error, Debug)]
pub enum GenSenseError {
    #[error("Parse failure: {0}")]
    ParseFailure(String),
    #[error("Rule configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Parser(#[from] tree_sitter::LanguageError),
    #[error("Pattern error: {0}")]
    Pattern(String),
}

pub type Result<T> = std::result::Result<T, GenSenseError>;
