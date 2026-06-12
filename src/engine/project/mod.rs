// SPDX-License-Identifier: MIT

pub mod builder;
pub mod cache;
pub mod config;
pub mod files;
pub mod helpers;
pub mod runner;

use crate::engine::auditor::FrensenseAuditor;
use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId, FrensenseEnvironment, ProjectRule, SourceRegistry};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Detailed internal snapshot of a file's analysis artifacts.
pub struct FileSnapshot {
    pub id: FileId,
    pub path: PathBuf,
    pub content: String,
    pub tree: tree_sitter::Tree,
    pub symbols: Vec<crate::semantics::symbols::Symbol>,
    pub edges: Vec<(String, String)>,
    pub semantic_ops: Vec<crate::semantics::data_flow::normalization::SemanticOp>,
}

pub struct Engine {
    auditor: FrensenseAuditor,
    project_rules: Vec<Box<dyn ProjectRule>>,
    source_registry: SourceRegistry,
    min_confidence: f32,
    environment: FrensenseEnvironment,
    enabled_categories: HashSet<String>,
    enabled_tags: HashSet<String>,
    suite: crate::Suite,
    extra_rule_dirs: Vec<PathBuf>,
    no_builtin_rules: bool,
    isolate_rules: bool,
    severity_filter: Option<crate::Severity>,
    language_filter: Option<Vec<&'static str>>,
    file_cache: cache::FileCache,
    cache_root: Option<PathBuf>,

    // Tunable parameters
    jaccard_threshold: f64,
    confidence_boost_rate: f32,
    confidence_boost_max: f32,
    max_source_lines: Option<usize>,
    ngram_window_size: usize,
    min_ngram_count: usize,
    taint_confidence_interprocedural: f32,
    taint_confidence_intraprocedural: f32,
    default_taint_max_depth: usize,

    // CLI-driven rule overrides (merged with config file)
    disabled_rule_ids: Vec<String>,
    severity_overrides: HashMap<String, crate::Severity>,

    #[cfg(feature = "fingerprinting")]
    profile: Option<crate::engine::profile::ProjectProfile>,
    #[cfg(feature = "fingerprinting")]
    profile_threshold: f64,

    corpus_dir: Option<PathBuf>,
}

impl Engine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            auditor: FrensenseAuditor::new(Vec::new()),
            project_rules: Vec::new(),
            source_registry: SourceRegistry::new(),
            min_confidence: 0.1,
            environment: FrensenseEnvironment::Development,
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            suite: crate::Suite::All,
            extra_rule_dirs: Vec::new(),
            no_builtin_rules: false,
            isolate_rules: false,
            severity_filter: None,
            language_filter: None,
            file_cache: cache::FileCache::default(),
            cache_root: None,
            jaccard_threshold: 0.8,
            confidence_boost_rate: 0.10,
            confidence_boost_max: 0.30,
            max_source_lines: None,
            ngram_window_size: 5,
            min_ngram_count: 3,
            taint_confidence_interprocedural: 0.80,
            taint_confidence_intraprocedural: 0.90,
            default_taint_max_depth: 5,
            disabled_rule_ids: Vec::new(),
            severity_overrides: HashMap::new(),
            #[cfg(feature = "fingerprinting")]
            profile: None,
            #[cfg(feature = "fingerprinting")]
            profile_threshold: 0.7,
            corpus_dir: None,
        }
    }
}

pub struct ProjectAuditor {
    rules: Vec<Box<dyn ProjectRule>>,
}

impl ProjectAuditor {
    #[must_use]
    pub fn new(rules: Vec<Box<dyn ProjectRule>>) -> Self {
        Self { rules }
    }

    #[must_use]
    pub fn run(
        &self,
        symbols: &SymbolRegistry,
        sources: &SourceRegistry,
        _env: FrensenseEnvironment,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        for rule in &self.rules {
            advisories.extend(rule.check_project(symbols, sources));
        }
        advisories
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
