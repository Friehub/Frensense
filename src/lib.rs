// SPDX-License-Identifier: MIT
#![allow(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::return_self_not_must_use,
    clippy::missing_panics_doc,
    clippy::module_inception
)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![allow(
    clippy::too_many_arguments,
    clippy::regex_creation_in_loops,
    clippy::must_use_candidate,
    clippy::cast_precision_loss
)]

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use thiserror::Error;
use tree_sitter::Node;

pub const FRENSENSE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod cli;
pub mod engine;
pub mod mcp;
pub mod parser;
pub mod patcher;
pub mod reporter;
pub mod semantics;
#[cfg(feature = "temporal")]
pub mod temporal;

pub use crate::engine::Engine;
#[cfg(feature = "fingerprinting")]
pub use crate::engine::FunctionFingerprint;
pub use crate::engine::auditor::{FrensenseAuditor, ScanResult};

use crate::semantics::SymbolRegistry;

pub use frensense_engine::{FileId, ScopeId};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

impl Severity {
    #[must_use]
    pub fn meets_threshold(&self, threshold: Severity) -> bool {
        match (self, threshold) {
            (Severity::Critical, _)
            | (Severity::Info, Severity::Info)
            | (Severity::Warning, Severity::Warning | Severity::Info) => true,
            (Severity::Warning, Severity::Critical) | (Severity::Info, _) => false,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrensenseEnvironment {
    Production,
    Staging,
    Development,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Precision {
    VeryHigh,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suite {
    Default,
    Extended,
    All,
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
    #[serde(default = "default_precision")]
    pub precision: Precision,
}

const fn default_confidence() -> f32 {
    0.55
}

const fn default_precision() -> Precision {
    Precision::Low
}

impl RuleMetadata {
    #[must_use]
    pub fn meets_suite(&self, suite: Suite) -> bool {
        match suite {
            Suite::Default => self.precision == Precision::VeryHigh,
            Suite::Extended => matches!(self.precision, Precision::VeryHigh | Precision::High),
            Suite::All => true,
        }
    }
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

/// Lossless usize → u32, saturating at `u32::MAX`.
#[inline]
#[must_use]
pub fn to_u32(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

impl Advisory {
    /// Create an advisory with common defaults pre-filled.
    /// Only set the fields that differ per finding.
    #[must_use]
    pub fn bare(
        rule_id: impl Into<String>,
        severity: Severity,
        file_id: FileId,
        file_path: &std::path::Path,
        observation: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            file_id,
            file_path: file_path.display().to_string(),
            severity,
            observation: observation.into(),
            confidence: 0.5,
            impact: String::new(),
            improvement: String::new(),
            line: 0,
            column: 0,
            start_byte: 0,
            end_byte: 0,
            original_content: String::new(),
            proposed_replacement: None,
            proposed_import: None,
            enclosing_symbol: None,
            fingerprint: String::new(),
            auto_fixable: false,
            requires_human: true,
            tags: Vec::new(),
        }
    }

    pub fn with_confidence(mut self, v: f32) -> Self {
        self.confidence = v;
        self
    }
    pub fn with_line(mut self, v: u32) -> Self {
        self.line = v;
        self
    }
    pub fn with_column(mut self, v: u32) -> Self {
        self.column = v;
        self
    }
    pub fn with_bytes(mut self, start: u32, end: u32) -> Self {
        self.start_byte = start;
        self.end_byte = end;
        self
    }
    pub fn with_content(mut self, v: impl Into<String>) -> Self {
        self.original_content = v.into();
        self
    }
    pub fn with_impact(mut self, v: impl Into<String>) -> Self {
        self.impact = v.into();
        self
    }
    pub fn with_improvement(mut self, v: impl Into<String>) -> Self {
        self.improvement = v.into();
        self
    }
    pub fn with_enclosing_symbol(mut self, v: impl Into<String>) -> Self {
        self.enclosing_symbol = Some(v.into());
        self
    }
    pub fn with_tags<const N: usize>(mut self, tags: [&str; N]) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn with_replacement(mut self, v: impl Into<String>) -> Self {
        self.proposed_replacement = Some(v.into());
        self
    }

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

type TaintCacheKey = (String, String, String, String, usize);
type TaintCacheMap = HashMap<TaintCacheKey, Vec<Advisory>>;

const TAINT_CACHE_MAX: usize = 1024;

pub struct TaintCache {
    inner: RefCell<TaintCacheMap>,
    order: RefCell<VecDeque<TaintCacheKey>>,
}

impl TaintCache {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(HashMap::new()),
            order: RefCell::new(VecDeque::new()),
        }
    }

    pub fn get(&self, key: &TaintCacheKey) -> Option<Vec<Advisory>> {
        self.inner.borrow().get(key).cloned()
    }

    pub fn insert(&self, key: TaintCacheKey, value: Vec<Advisory>) {
        let mut inner = self.inner.borrow_mut();
        let mut order = self.order.borrow_mut();
        if inner.len() >= TAINT_CACHE_MAX
            && let Some(oldest) = order.pop_front()
        {
            inner.remove(&oldest);
        }
        order.push_back(key.clone());
        inner.insert(key, value);
    }
}

impl Default for TaintCache {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FrensenseContext<'a> {
    pub file_id: FileId,
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub symbols: &'a SymbolRegistry,
    pub graph: &'a crate::semantics::graph::SemanticGraph,
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
    pub taint_confidence_interprocedural: f32,
    pub taint_confidence_intraprocedural: f32,
    pub default_taint_max_depth: usize,
    pub ngram_window_size: usize,
}

/// Core Trait: Represents a high-precision semantic `Frensense` rule.
pub trait FrensenseRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;

    /// The core logic for verifying a finding.
    fn check<'a>(&self, node: Node<'a>, context: &FrensenseContext<'a>) -> Vec<Advisory>;

    /// Helper to get rule ID
    fn id(&self) -> &str {
        self.metadata().id.as_ref()
    }

    /// Helper to check file applicability
    fn applies_to(&self, extension: &str) -> bool;

    /// File-level check (not per-node). Fires once per file.
    /// Default no-op — override for rules like file-length limits.
    fn file_check(&self, _context: &FrensenseContext<'_>) -> Vec<Advisory> {
        Vec::new()
    }

    /// Helper for query-based matching
    fn query(&self) -> Option<&str> {
        None
    }

    /// DRY Helper: Create a new advisory for this rule.
    fn new_advisory(
        &self,
        node: &Node,
        context: &FrensenseContext,
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
        context: &FrensenseContext,
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

    fn is_enabled_in(&self, env: FrensenseEnvironment) -> bool {
        let meta = self.metadata();
        if env == FrensenseEnvironment::Production {
            return !meta.tags.iter().any(|t| t == "beta");
        }
        true
    }
}

#[derive(Error, Debug)]
pub enum FrensenseError {
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

pub type Result<T> = std::result::Result<T, FrensenseError>;

// Force cargo recompilation of embedded rules definitions
