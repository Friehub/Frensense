// SPDX-License-Identifier: MIT
#![warn(clippy::unwrap_used)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![allow(clippy::too_many_arguments, clippy::regex_creation_in_loops)]

use include_dir::{Dir, include_dir};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;
use tree_sitter::Node;

pub static EMBEDDED_RULES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/rules/definitions");
pub const GENSENSE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod engine;
pub mod parser;
pub mod patcher;
pub mod reporter;
pub mod rules;
pub mod semantics;

pub use crate::engine::Engine;
#[cfg(feature = "fingerprinting")]
pub use crate::engine::FunctionFingerprint;
pub use crate::engine::auditor::{GenSenseAuditor, ScanResult};

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
    pub observation: Cow<'static, str>,
    pub impact: Cow<'static, str>,
    pub improvement: Cow<'static, str>,
    pub tags: Vec<Cow<'static, str>>,
    #[serde(alias = "domain")]
    pub category: Cow<'static, str>,
    #[serde(default = "default_confidence")]
    pub confidence: f32,
}

const fn default_confidence() -> f32 {
    0.55
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Advisory {
    pub rule_id: String,
    pub file_id: FileId,
    pub file_path: String,
    pub severity: Severity,
    pub confidence: f32,
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
    /// The suggested import statement to inject, if any.
    pub proposed_import: Option<String>,
    pub enclosing_symbol: Option<String>,
    pub fingerprint: String,
    pub auto_fixable: bool,
    pub requires_human: bool,
    pub tags: Vec<String>,
}

impl Advisory {
    /// Returns a unique identity key for this advisory, used for baseline comparisons.
    #[must_use]
    pub fn identity(&self) -> (String, String, Option<String>, u32, u32) {
        (
            self.rule_id.clone(),
            self.file_path.clone(),
            self.enclosing_symbol.clone(),
            self.line,
            self.column,
        )
    }

    #[must_use]
    pub fn fuzzy_identity(&self) -> (String, String, Option<String>, String) {
        (
            self.rule_id.clone(),
            self.file_path.clone(),
            self.enclosing_symbol.clone(),
            self.original_content.clone(),
        )
    }
}

impl PartialEq for Advisory {
    fn eq(&self, other: &Self) -> bool {
        self.rule_id == other.rule_id
            && self.file_id == other.file_id
            && self.file_path == other.file_path
            && self.severity == other.severity
            && self.confidence.to_bits() == other.confidence.to_bits()
            && self.observation == other.observation
            && self.impact == other.impact
            && self.improvement == other.improvement
            && self.line == other.line
            && self.column == other.column
            && self.start_byte == other.start_byte
            && self.end_byte == other.end_byte
            && self.original_content == other.original_content
            && self.proposed_replacement == other.proposed_replacement
            && self.proposed_import == other.proposed_import
            && self.enclosing_symbol == other.enclosing_symbol
            && self.fingerprint == other.fingerprint
    }
}

impl Eq for Advisory {}

impl std::hash::Hash for Advisory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule_id.hash(state);
        self.file_id.hash(state);
        self.file_path.hash(state);
        self.severity.hash(state);
        self.confidence.to_bits().hash(state);
        self.observation.hash(state);
        self.impact.hash(state);
        self.improvement.hash(state);
        self.line.hash(state);
        self.column.hash(state);
        self.start_byte.hash(state);
        self.end_byte.hash(state);
        self.original_content.hash(state);
        self.proposed_replacement.hash(state);
        self.proposed_import.hash(state);
        self.enclosing_symbol.hash(state);
        self.fingerprint.hash(state);
    }
}

pub type TaintCache = RefCell<HashMap<(String, String, String, String, usize), Vec<Advisory>>>;

pub struct GenSenseContext<'a> {
    pub file_id: FileId,
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub symbols: &'a SymbolRegistry,
    pub semantic_ops: &'a [crate::semantics::data_flow::normalization::SemanticOp],
    pub taint_cache: &'a TaintCache,
    pub file_trees: &'a HashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
}

/// Core Trait: Represents a high-precision semantic `GenSense` rule.
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
        let file_path = context.file_path.to_string_lossy().to_string();
        let enclosing_symbol = context
            .symbols
            .find_function_at(&file_path, node.start_position().row + 1)
            .and_then(|idx| context.symbols.graph().get_symbol(idx))
            .map(|s| s.name.clone());

        Advisory {
            rule_id: meta.id.to_string(),
            file_id: context.file_id,
            file_path,
            severity: meta.severity,
            confidence: meta.confidence,
            observation,
            impact: meta.impact.to_string(),
            improvement: meta.improvement.to_string(),
            line: u32::try_from(node.start_position().row + 1).unwrap_or(u32::MAX),
            column: u32::try_from(node.start_position().column + 1).unwrap_or(u32::MAX),
            start_byte: u32::try_from(node.start_byte()).unwrap_or(u32::MAX),
            end_byte: u32::try_from(node.end_byte()).unwrap_or(u32::MAX),
            original_content: context.source_code[node.start_byte()..node.end_byte()].to_string(),
            proposed_replacement: None,
            proposed_import: None,
            enclosing_symbol,
            fingerprint: String::new(),
            auto_fixable: false,
            requires_human: false,
            tags: meta.tags.iter().map(ToString::to_string).collect(),
        }
    }

    fn new_remediated_advisory(
        &self,
        node: &Node,
        context: &GenSenseContext,
        observation: String,
        replacement: String,
        import: Option<String>,
    ) -> Advisory {
        let mut adv = self.new_advisory(node, context, observation);
        adv.proposed_replacement = Some(replacement);
        adv.proposed_import = import;
        adv
    }

    fn with_confidence(&self, mut advisory: Advisory, confidence: f32) -> Advisory {
        advisory.confidence = confidence;
        advisory
    }
}

pub use crate::engine::source::SourceRegistry;

/// A project-level rule that operates across all files simultaneously.
/// Receives the fully assembled, immutable `SymbolRegistry` and `SourceRegistry`.
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
    #[error("Engine error: {0}")]
    Engine(String),
}

pub type Result<T> = std::result::Result<T, GenSenseError>;

// Force cargo recompilation of embedded rules definitions
