// SPDX-License-Identifier: MIT
#![allow(
    clippy::must_use_candidate,
    clippy::cast_precision_loss,
    clippy::collapsible_if,
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value
)]

pub mod atomic_section;
pub mod cfg;
pub mod data_flow;
pub mod fingerprint;
pub mod graph;
pub mod minhash;
pub mod parser;
pub mod pattern;
pub mod profile;
pub mod reachability;
pub mod secrets;
pub mod slice;
pub mod symbols;
pub mod temporal;

use std::collections::HashMap;
use std::path::Path;

/// Opaque identifier for a source file within a single analysis session.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u64);

/// Structured result of analyzing a single source file.
/// This is the primary output of the engine — no advisories, no rules.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub language: String,
    pub file_path: String,
    pub source: String,
    pub functions: Vec<fingerprint::FunctionFingerprint>,
    pub symbols: symbols::SymbolRegistry,
    pub graph: graph::SemanticGraph,
    pub temporal_events: Vec<graph::TemporalEvent>,
}

/// Structured result of analyzing a full project (multiple files).
#[derive(Debug, Clone)]
pub struct ProjectAnalysis {
    pub files: HashMap<String, AnalysisResult>,
    pub profile: Option<profile::ProjectProfile>,
}

#[derive(Debug)]
pub enum GenSenseError {
    ParseFailure(String),
    Config(String),
    Io(String),
    ParserError(String),
    Pattern(String),
    Engine(String),
}

impl std::fmt::Display for GenSenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFailure(msg) => write!(f, "Parse failure: {msg}"),
            Self::Config(msg) => write!(f, "Config error: {msg}"),
            Self::Io(msg) => write!(f, "IO error: {msg}"),
            Self::ParserError(msg) => write!(f, "Parser error: {msg}"),
            Self::Pattern(msg) => write!(f, "Pattern error: {msg}"),
            Self::Engine(msg) => write!(f, "Engine error: {msg}"),
        }
    }
}

impl std::error::Error for GenSenseError {}

impl From<std::io::Error> for GenSenseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<tree_sitter::LanguageError> for GenSenseError {
    fn from(e: tree_sitter::LanguageError) -> Self {
        Self::ParserError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, GenSenseError>;

/// Analyze a single source file. Returns structured analysis data.
///
/// # Errors
/// Returns an error if the language is unsupported or the source cannot be parsed.
pub fn analyze_file(source: &str, language: &str, file_path: &Path) -> Result<AnalysisResult> {
    let lang = parser::ParserRegistry::get_language_by_name(language)?;
    let mut ts_parser = tree_sitter::Parser::new();
    ts_parser
        .set_language(&lang)
        .map_err(|e| GenSenseError::ParserError(format!("Failed to set language: {e}")))?;
    let tree = ts_parser
        .parse(source, None)
        .ok_or_else(|| GenSenseError::ParseFailure("Failed to parse source".to_string()))?;

    let root = tree.root_node();

    let mut functions = Vec::new();
    let parser_registry = parser::ParserRegistry;
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    fingerprint::extract_fingerprints(root, source, file_path, &mut functions, 5);

    let mut symbols = symbols::SymbolRegistry::new();
    let file_id = FileId(0);
    if let Some(sym_query) = parser_registry.get_symbol_query_by_ext(ext) {
        symbols.extract_from_tree(&tree, source, file_path, file_id, sym_query);
    }
    if let Some(call_query) = parser_registry.get_call_query_by_ext(ext) {
        symbols.extract_edges_from_tree(&tree, source, file_path, call_query);
    }

    let graph = symbols.graph().clone();

    let temporal_events = graph::extract_temporal_events(root, source, file_path);

    Ok(AnalysisResult {
        language: language.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
        source: source.to_string(),
        functions,
        symbols,
        graph,
        temporal_events,
    })
}

/// Analyze multiple files in a project context. Produces a project profile
/// and per-file analysis results.
///
/// # Errors
/// Returns an error if any file fails to parse.
pub fn analyze_project(
    files: impl IntoIterator<Item = (String, String)>,
) -> Result<ProjectAnalysis> {
    let mut results = HashMap::new();
    let mut all_fingerprints = Vec::new();

    for (path_str, source) in files {
        let path = Path::new(&path_str);
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let language = match ext {
            "rs" => "rust",
            "ts" | "tsx" => "typescript",
            "js" | "jsx" => "javascript",
            "yml" | "yaml" => "yaml",
            _ => "unknown",
        };

        if language == "unknown" {
            continue;
        }

        let result = analyze_file(&source, language, path)?;
        all_fingerprints.extend(result.functions.clone());
        results.insert(path_str, result);
    }

    let profile = if all_fingerprints.is_empty() {
        None
    } else {
        Some(profile::ProjectProfile::learn(&all_fingerprints))
    };

    Ok(ProjectAnalysis { files: results, profile })
}
