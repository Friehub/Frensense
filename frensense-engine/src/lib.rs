#![allow(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::stable_sort_primitive,
    clippy::cast_possible_truncation,
    clippy::implicit_hasher,
    clippy::field_reassign_with_default,
    clippy::match_same_arms
)]
// SPDX-License-Identifier: MIT
#![allow(
    clippy::must_use_candidate,
    clippy::cast_precision_loss,
    clippy::collapsible_if,
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value
)]

pub mod ast_distance;
pub mod auto_filter;
pub mod cfg;
pub mod context;
pub mod corpus;
pub mod data_flow;
pub mod decorator;
pub mod deps;
pub mod export_matcher;
pub mod fingerprint;
pub mod function_role;
#[cfg(feature = "full-analysis")]
pub mod graph;
pub mod import_resolver;
pub mod lang;
pub mod minhash;
#[cfg(feature = "oxc")]
pub mod oxc_provider;
pub mod parser;
pub mod pattern;
pub mod per_pattern_calibration;
#[cfg(feature = "full-analysis")]
pub mod profile;
pub mod route_registry;
#[cfg(feature = "rust-hir")]
pub mod rust_hir_provider;
pub mod semantic;
pub mod symbols;
#[cfg(feature = "full-analysis")]
pub mod temporal;

use rustc_hash::FxHashMap;
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
    pub import_map: import_resolver::ImportMap,
    pub route_registry: route_registry::HandlerRegistry,
    #[cfg(feature = "full-analysis")]
    pub graph: graph::SemanticGraph,
    #[cfg(feature = "full-analysis")]
    pub temporal_events: Vec<graph::TemporalEvent>,
}

/// Structured result of analyzing a full project (multiple files).
#[derive(Debug, Clone)]
pub struct ProjectAnalysis {
    pub files: FxHashMap<String, AnalysisResult>,
    pub project_registry: route_registry::HandlerRegistry,
    #[cfg(feature = "full-analysis")]
    pub profile: Option<profile::ProjectProfile>,
}

#[derive(Debug, thiserror::Error)]
pub enum FrensenseError {
    #[error("Parse failure: {0}")]
    ParseFailure(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parser error: {0}")]
    ParserError(String),
    #[error("Pattern error: {0}")]
    Pattern(String),
    #[error("Engine error: {0}")]
    Engine(String),
}

impl From<std::io::Error> for FrensenseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<tree_sitter::LanguageError> for FrensenseError {
    fn from(e: tree_sitter::LanguageError) -> Self {
        Self::ParserError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, FrensenseError>;

/// Analyze a single source file. Returns structured analysis data.
///
/// # Errors
/// Returns an error if the language is unsupported or the source cannot be parsed.
pub fn analyze_file(
    source: &str,
    language: &str,
    file_path: &Path,
    file_id: FileId,
) -> Result<AnalysisResult> {
    let lang = parser::ParserRegistry::get_language_by_name(language)?;
    let mut ts_parser = tree_sitter::Parser::new();
    ts_parser
        .set_language(&lang)
        .map_err(|e| FrensenseError::ParserError(format!("Failed to set language: {e}")))?;
    let tree = ts_parser
        .parse(source, None)
        .ok_or_else(|| FrensenseError::ParseFailure("Failed to parse source".to_string()))?;

    let root = tree.root_node();
    let import_map = import_resolver::ImportMap::build_from_tree(source, root);
    let route_registry =
        route_registry::build_handler_registry(root, source, &file_path.to_string_lossy());

    let mut functions = Vec::new();
    let parser_registry = parser::ParserRegistry;
    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    fingerprint::extract_fingerprints(root, source, file_path, &mut functions, 5);

    let mut symbols = symbols::SymbolRegistry::new();
    if let Some(sym_query) = parser_registry.get_symbol_query_by_ext(ext) {
        symbols.extract_from_tree(&tree, source, file_path, file_id, sym_query);
    }
    if let Some(call_query) = parser_registry.get_call_query_by_ext(ext) {
        symbols.extract_edges_from_tree(&tree, source, file_path, call_query);
    }

    #[cfg(feature = "full-analysis")]
    let graph = symbols.graph().clone();

    #[cfg(feature = "full-analysis")]
    let temporal_events = graph::extract_temporal_events(root, source, file_path, None);

    Ok(AnalysisResult {
        language: language.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
        source: source.to_string(),
        functions,
        symbols,
        import_map,
        route_registry,
        #[cfg(feature = "full-analysis")]
        graph,
        #[cfg(feature = "full-analysis")]
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
    let mut results = FxHashMap::default();
    let mut project_registry = route_registry::HandlerRegistry::default();
    #[cfg(feature = "full-analysis")]
    let mut all_fingerprints = Vec::new();

    for (idx, (path_str, source)) in files.into_iter().enumerate() {
        let path = Path::new(&path_str);
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let language = crate::parser::ext_to_language(ext);

        if language == "unknown" {
            continue;
        }

        // FileId is a u32. Wrapping to u32::MAX silently corrupts symbol graph
        // edges since MAX is indistinguishable from a real ID. In practice
        // idx never exceeds u32::MAX (4 billion files) so this is a hard invariant.
        let file_id = FileId(u32::try_from(idx).expect("project file count exceeded u32::MAX"));

        let result = analyze_file(&source, language, path, file_id)?;
        project_registry.merge(result.route_registry.clone());
        results.insert(path_str, result);
    }

    #[cfg(feature = "full-analysis")]
    {
        // 1. Build a global semantic graph
        let mut global_graph = crate::graph::SemanticGraph::new();
        for res in results.values() {
            global_graph.merge(res.graph.clone());
        }

        // 2. Build the cross-file taint resolver
        let all_symbols = global_graph.all_symbols();
        let mut resolver =
            crate::data_flow::cross_file::build_resolver(&all_symbols, &global_graph);

        // 3. Register exposed taint sources (e.g. HTTP handlers)
        for res in results.values() {
            for func in &res.functions {
                let role =
                    crate::function_role::classify_role_with_imports(func, Some(&res.import_map));
                if role == crate::function_role::FunctionRole::HttpHandler {
                    let key = format!("{}:{}", res.file_path, func.function_name);
                    resolver.register_exposed_taint(
                        &key,
                        &res.file_path,
                        crate::data_flow::TaintOrigin::UserInput,
                    );
                }
            }
        }

        // 3b. Propagate taint through the call graph so intermediate non-HttpHandler
        //     functions (DataTransformer, etc.) called by seeded sources are also
        //     treated as taint sources.  Without this, multi-hop chains like
        //     HttpHandler → service → repository → DB fail to resolve.
        resolver.propagate_taint();

        // 4. Resolve taint for sinks (e.g. DbQuery, ShellExecutor) and update fingerprints
        for res in results.values_mut() {
            for func in &mut res.functions {
                let role =
                    crate::function_role::classify_role_with_imports(func, Some(&res.import_map));
                if matches!(
                    role,
                    crate::function_role::FunctionRole::DbQuery
                        | crate::function_role::FunctionRole::ShellExecutor
                ) {
                    let taints = resolver.resolve_taint(&func.function_name, &res.file_path, 10);
                    if !taints.is_empty() {
                        // If cross-file taint reached this sink, hash the api call names to simulate the structural taint
                        for api in &func.raw_call_names {
                            let mut hasher = std::collections::hash_map::DefaultHasher::new();
                            std::hash::Hash::hash(api, &mut hasher);
                            func.tainted_api_calls
                                .push(std::hash::Hasher::finish(&hasher));
                        }
                    }
                }
            }
            all_fingerprints.extend(res.functions.clone());
        }
    }

    // Second pass: update `is_registered_handler` for all fingerprints
    // using the merged project-level registry (cross-file route registrations).
    for result in results.values_mut() {
        for fp in &mut result.functions {
            if !fp.is_registered_handler {
                fp.is_registered_handler =
                    project_registry.is_registered_handler(&fp.function_name);
            }
        }
    }

    Ok(ProjectAnalysis {
        files: results,
        project_registry,
        #[cfg(feature = "full-analysis")]
        profile: if all_fingerprints.is_empty() {
            None
        } else {
            Some(profile::ProjectProfile::learn(&all_fingerprints))
        },
    })
}
