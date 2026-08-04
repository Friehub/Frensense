// SPDX-License-Identifier: MIT

pub mod builder;
pub mod cache;
pub mod config;
pub mod files;
pub mod helpers;
pub mod runner;

use crate::engine::auditor::FrensenseAuditor;
use crate::{FileId, FrensenseEnvironment, SourceRegistry};
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
    source_registry: SourceRegistry,
    min_confidence: f64,
    environment: FrensenseEnvironment,
    enabled_categories: HashSet<String>,
    enabled_tags: HashSet<String>,
    suite: crate::Suite,
    severity_filter: Option<crate::Severity>,
    language_filter: Option<Vec<&'static str>>,
    file_cache: cache::FileCache,
    cache_root: Option<PathBuf>,

    // Tunable parameters
    jaccard_threshold: f64,
    confidence_boost_rate: f64,
    confidence_boost_max: f64,
    taint_unconfirmed_penalty: f64,
    high_branch_ratio_threshold: f64,
    high_branch_ratio_suppression_factor: f64,
    max_source_lines: Option<usize>,
    ngram_window_size: usize,
    min_ngram_count: usize,
    taint_confidence_interprocedural: f64,
    taint_confidence_intraprocedural: f64,
    default_taint_max_depth: usize,

    // CLI-driven rule overrides (merged with config file)
    disabled_rule_ids: Vec<String>,
    severity_overrides: HashMap<String, crate::Severity>,

    #[cfg(feature = "fingerprinting")]
    profile: Option<frensense_engine::profile::ProjectProfile>,
    #[cfg(feature = "fingerprinting")]
    profile_threshold: f64,

    corpus_dir: Option<PathBuf>,
    corpus_threshold: f64,
    threshold_overrides: Vec<(String, f64)>,
    corpus_bundle: Option<&'static [u8]>,
    baseline_path: Option<PathBuf>,
    extra_taint_rule_dirs: Vec<PathBuf>,
    pub check_deps: bool,
    pub use_data_flow: bool,
    pub ngram_sim_threshold: f64,
    calibration: Option<crate::engine::confidence_calibration::CalibrationParams>,
    per_category_calibration:
        Option<crate::engine::per_category_calibration::PerCategoryCalibration>,
}

impl Engine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            auditor: FrensenseAuditor::new(Vec::new()),
            source_registry: SourceRegistry::new(),
            min_confidence: 0.1,
            environment: FrensenseEnvironment::Development,
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            suite: crate::Suite::All,
            severity_filter: None,
            language_filter: None,
            file_cache: cache::FileCache::default(),
            cache_root: None,
            jaccard_threshold: 0.8,
            confidence_boost_rate: 0.10,
            confidence_boost_max: 0.30,
            taint_unconfirmed_penalty: crate::engine::composition::TAINT_UNCONFIRMED_PENALTY,
            high_branch_ratio_threshold: crate::engine::composition::HIGH_BRANCH_RATIO_THRESHOLD,
            high_branch_ratio_suppression_factor:
                crate::engine::composition::HIGH_BRANCH_RATIO_SUPPRESSION_FACTOR,
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
            corpus_threshold: 0.40,
            threshold_overrides: Vec::new(),
            corpus_bundle: None,
            baseline_path: None,
            extra_taint_rule_dirs: Vec::new(),
            check_deps: false,
            use_data_flow: false,
            ngram_sim_threshold: 0.05,
            calibration: None,
            per_category_calibration: None,
        }
    }

    pub fn set_corpus_dir(&mut self, dir: std::path::PathBuf) {
        self.corpus_dir = Some(dir);
    }

    pub fn set_corpus_threshold(&mut self, threshold: f64) {
        self.corpus_threshold = threshold;
    }

    pub fn set_threshold_overrides(&mut self, overrides: Vec<(String, f64)>) {
        self.threshold_overrides = overrides;
    }

    pub fn set_corpus_bundle(&mut self, bundle: &'static [u8]) {
        self.corpus_bundle = Some(bundle);
    }

    /// blake3 hash of the corpus bundle, used for cache invalidation.
    #[must_use]
    pub fn corpus_bundle_hash(&self) -> Option<String> {
        self.corpus_bundle
            .map(|b| blake3::hash(b).to_hex().to_string())
    }

    pub fn set_baseline_path(&mut self, path: std::path::PathBuf) {
        self.baseline_path = Some(path);
    }

    pub fn set_check_deps(&mut self, check: bool) {
        self.check_deps = check;
    }

    pub fn set_extra_taint_rule_dirs(&mut self, dirs: Vec<PathBuf>) {
        self.extra_taint_rule_dirs = dirs;
    }

    pub fn set_use_data_flow(&mut self, enable: bool) {
        self.use_data_flow = enable;
    }

    pub fn set_ngram_sim_threshold(&mut self, threshold: f64) {
        self.ngram_sim_threshold = threshold;
    }

    pub fn load_calibration(&mut self) {
        use crate::engine::confidence_calibration::load_calibration;
        use crate::engine::per_category_calibration::load_per_category_calibration;
        use std::path::Path;

        // Try to load per-category calibration first
        let per_cat_paths = [
            Path::new("per_category_calibration.json"),
            Path::new(".frensense/per_category_calibration.json"),
        ];

        for path in &per_cat_paths {
            if let Some(params) = load_per_category_calibration(path)
                && params.global.n_samples > 0
                && params.global.accuracy > 0.0
            {
                self.per_category_calibration = Some(params);
                return;
            }
        }

        // Fall back to global calibration
        let paths = [
            Path::new("calibration.json"),
            Path::new(".frensense/calibration.json"),
        ];

        for path in &paths {
            if let Some(params) = load_calibration(path)
                && params.n_samples > 0
                && params.accuracy > 0.0
            {
                self.calibration = Some(params);
                return;
            }
        }
    }

    pub fn calibration(&self) -> Option<&crate::engine::confidence_calibration::CalibrationParams> {
        self.calibration.as_ref()
    }

}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
