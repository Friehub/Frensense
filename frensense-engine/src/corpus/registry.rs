// SPDX-License-Identifier: MIT

use crate::corpus::pattern::CorpusPattern;
use crate::corpus::source_sink::CorpusSourceSinkRegistry;
use crate::data_flow::taint_metrics::TaintMetrics;
use crate::data_flow::{TaintOrigin, TaintRegistry};
use crate::fingerprint::{FunctionFingerprint, apply_idf_weights, compute_idf_weights};
use crate::pattern::scorer::ScorerConfig;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub score: f64,
    pub observation: Option<String>,
    pub impact: Option<String>,
    pub improvement: Option<String>,
    pub cwe: Option<String>,
    pub cvss: Option<f32>,
    pub owasp: Option<String>,
    pub severity: Option<String>,
    pub runtime_probe: Option<String>,
    /// Taint branch ratio from `TaintMetrics` - fraction of tainted accesses
    /// that are branched on. Propagated to the composition layer.
    pub taint_branch_ratio: Option<f64>,
    /// Whether the function name suggests a validator/sanitizer.
    pub has_validation_name: bool,
}

#[derive(Default, Clone)]
pub struct ScanContext<'a> {
    pub func_node: Option<tree_sitter::Node<'a>>,
    pub source: Option<&'a str>,
    pub actual_context: Option<&'a crate::context::FileContext>,
    pub spec: Option<&'a dyn frensense_lang::spec::LanguageSpec>,
}

pub struct PatternRegistry {
    patterns: Vec<CorpusPattern>,
    api_index: Option<FxHashMap<u64, Vec<usize>>>,
    flow_index: Option<FxHashMap<u64, Vec<usize>>>,
    threshold: f64,
    threshold_overrides: std::collections::HashMap<String, f64>,
    idf_weights: FxHashMap<u64, f32>,
    api_idf_weights: FxHashMap<u64, f32>,
    /// Auto-derived semantic filter suggestions (import + call exclusivity).
    pub auto_filter_stats: Option<crate::auto_filter::AutoFilterStats>,
    source_sink: CorpusSourceSinkRegistry,
    /// Configurable scoring parameters.
    pub scorer_config: ScorerConfig,
}

impl PatternRegistry {
    pub fn new(threshold: f64) -> Self {
        Self {
            patterns: Vec::new(),
            api_index: None,
            flow_index: None,
            threshold,
            threshold_overrides: std::collections::HashMap::new(),
            idf_weights: FxHashMap::default(),
            api_idf_weights: FxHashMap::default(),
            auto_filter_stats: None,
            source_sink: CorpusSourceSinkRegistry::default(),
            scorer_config: ScorerConfig::default(),
        }
    }

    /// Set custom scorer configuration.
    pub fn set_scorer_config(&mut self, config: ScorerConfig) {
        self.scorer_config = config;
    }

    /// Get a reference to the current scorer configuration.
    pub fn scorer_config(&self) -> &ScorerConfig {
        &self.scorer_config
    }

    /// Compute auto-derived semantic filter suggestions from corpus files.
    /// Only used as a fallback when the embedded bundle doesn't contain them.

    /// Get the corpus-learned source/sink registry.
    pub fn source_sink_registry(&self) -> &CorpusSourceSinkRegistry {
        &self.source_sink
    }

    #[cfg(feature = "serialize")]
    pub fn load_from_bundle(&mut self, bytes: &[u8]) -> crate::Result<usize> {
        let loaded =
            crate::corpus::bundle::load_bundle(bytes).map_err(crate::FrensenseError::Engine)?;
        let count = loaded.patterns.len();

        self.patterns = loaded
            .patterns
            .into_iter()
            .map(CorpusPattern::from)
            .collect();

        // Use pre-computed API IDF from bundle when available (avoids recomputation)
        if !loaded.api_idf_weights.is_empty() {
            self.api_idf_weights = loaded.api_idf_weights.into_iter().collect();
        }

        // Restore auto-derived filter suggestions from bundle
        // Bundle format: (pid, imports, calls, excl_calls, fn_re, excl_nodes, excl_fnames)
        if !loaded.auto_filter_stats.is_empty() {
            let mut contains_call_to = std::collections::HashMap::new();
            let mut must_not_contain_call_to = std::collections::HashMap::new();
            let mut contains_node_type = std::collections::HashMap::new();
            let mut must_not_contain_node_type = std::collections::HashMap::new();
            let mut must_not_match_function_name = std::collections::HashMap::new();
            for entry in loaded.auto_filter_stats {
                let pid = entry.pattern_id;
                let calls = entry.required_calls;
                let excl_calls = entry.forbidden_calls;
                let req_nodes = entry.required_node_types;
                let excl_nodes = entry.forbidden_node_types;
                let excl_fnames = entry.forbidden_fn_names;
                if !calls.is_empty() {
                    contains_call_to.insert(pid.clone(), calls.into_iter().collect());
                }
                if !excl_calls.is_empty() {
                    must_not_contain_call_to.insert(pid.clone(), excl_calls.into_iter().collect());
                }
                if !req_nodes.is_empty() {
                    contains_node_type.insert(pid.clone(), req_nodes.into_iter().collect());
                }
                if !excl_nodes.is_empty() {
                    must_not_contain_node_type
                        .insert(pid.clone(), excl_nodes.into_iter().collect());
                }
                if !excl_fnames.is_empty() {
                    must_not_match_function_name
                        .insert(pid.clone(), excl_fnames.into_iter().collect());
                }
            }
            self.auto_filter_stats = Some(crate::auto_filter::AutoFilterStats {
                contains_call_to,
                must_not_contain_call_to,
                contains_node_type,
                function_name_regex: std::collections::HashMap::new(),
                must_not_contain_node_type,
                must_not_match_function_name,
            });
        }

        self.apply_ngram_idf();
        // compute_api_idf skipped when weights came from the bundle
        if self.api_idf_weights.is_empty() {
            self.compute_api_idf();
        }
        self.build_index();
        Ok(count)
    }

    /// Compute and apply n-gram IDF weights to all corpus fingerprints.
    fn apply_ngram_idf(&mut self) {
        let all_positives: Vec<FunctionFingerprint> = self
            .patterns
            .iter()
            .flat_map(|p| p.positives.iter().cloned())
            .collect();

        if all_positives.is_empty() {
            return;
        }

        self.idf_weights = compute_idf_weights(&all_positives);

        for pattern in &mut self.patterns {
            for fp in &mut pattern.positives {
                apply_idf_weights(fp, &self.idf_weights);
            }
            for fp in &mut pattern.negatives {
                apply_idf_weights(fp, &self.idf_weights);
            }
        }
    }

    /// Compute API-call IDF weights from corpus patterns and store in `self.api_idf_weights`.
    fn compute_api_idf(&mut self) {
        let total = self.patterns.len() as f32;
        if total == 0.0 {
            return;
        }
        let mut api_doc_freq: FxHashMap<u64, f32> = FxHashMap::default();
        for pattern in &self.patterns {
            let mut seen_in_pattern: rustc_hash::FxHashSet<u64> = rustc_hash::FxHashSet::default();
            for fp in &pattern.positives {
                for &call in &fp.api_calls {
                    if seen_in_pattern.insert(call) {
                        *api_doc_freq.entry(call).or_insert(0.0) += 1.0;
                    }
                }
            }
        }
        self.api_idf_weights = api_doc_freq
            .into_iter()
            .map(|(call, df)| (call, (total / df).ln()))
            .collect();
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn set_threshold_override(&mut self, category: String, threshold: f64) {
        self.threshold_overrides.insert(category, threshold);
    }

    fn threshold_for_pattern(&self, pattern_id: &str) -> f64 {
        // Extract category from pattern naming convention: {lang}_{category}_{name}
        // e.g., "rust_sec_cmd_injection" → "sec", "ts_llm_console_log" → "llm"
        let category = pattern_id.split('_').nth(1).unwrap_or("");
        self.threshold_overrides
            .get(category)
            .copied()
            .unwrap_or(self.threshold)
    }

    fn build_index(&mut self) {
        if self.patterns.is_empty() {
            return;
        }

        let mut api_index: FxHashMap<u64, Vec<usize>> = FxHashMap::default();
        let mut flow_index: FxHashMap<u64, Vec<usize>> = FxHashMap::default();

        for (i, pattern) in self.patterns.iter().enumerate() {
            for fp in &pattern.positives {
                // Index exact API call segments
                for api_hash in &fp.api_call_segments {
                    api_index.entry(*api_hash).or_default().push(i);
                }
                for api_hash in &fp.api_calls {
                    api_index.entry(*api_hash).or_default().push(i);
                }

                // Index exact flow paths
                for flow_hash in &fp.data_flow_path_hashes {
                    flow_index.entry(*flow_hash).or_default().push(i);
                }
            }
        }
        self.api_index = Some(api_index);
        self.flow_index = Some(flow_index);
    }

    pub fn scan_function<'a>(
        &self,
        fp: &FunctionFingerprint,
        ctx: &ScanContext<'a>,
    ) -> Vec<PatternMatch> {
        let func_node = ctx.func_node;
        let source = ctx.source;
        let actual_context = ctx.actual_context;
        let spec = ctx.spec;

        // Exact match querying via Inverted Index
        let mut api_candidates: std::collections::HashSet<usize> = std::collections::HashSet::new();
        if let Some(ref api_idx) = self.api_index {
            for api_hash in fp.api_call_segments.iter().chain(fp.api_calls.iter()) {
                if let Some(matches) = api_idx.get(api_hash) {
                    for &id in matches {
                        api_candidates.insert(id);
                    }
                }
            }
        } else {
            // Fallback for empty corpus / tests
            api_candidates = (0..self.patterns.len()).collect();
        }

        let mut flow_candidates: std::collections::HashSet<usize> =
            std::collections::HashSet::new();
        if let Some(ref flow_idx) = self.flow_index {
            for flow_hash in &fp.data_flow_path_hashes {
                if let Some(matches) = flow_idx.get(flow_hash) {
                    for &id in matches {
                        flow_candidates.insert(id);
                    }
                }
            }
        }

        let all_candidates: Vec<(usize, bool)> = {
            let mut seen = std::collections::HashSet::new();
            let mut merged = Vec::new();
            // Merge: Flow candidates are primary. API candidates are fallback for patterns without flow.
            for &id in flow_candidates.iter().chain(api_candidates.iter()) {
                if seen.insert(id) {
                    merged.push((id, flow_candidates.contains(&id)));
                }
            }
            merged
        };
        let candidate_count = all_candidates.len();
        if candidate_count == 0 {
            return Vec::new();
        }

        // Apply IDF weights to candidate fingerprint for scoring
        let mut weighted_fp = fp.clone();
        if !self.idf_weights.is_empty() {
            apply_idf_weights(&mut weighted_fp, &self.idf_weights);
        }

        // Pre-compute TaintMetrics once per function (not per candidate).
        let taint_metrics: Option<(TaintMetrics, TaintOrigin)> = func_node.and_then(|fn_node| {
            let src = source?;
            let mut reg = TaintRegistry::default();
            let mut seen_origins: Vec<TaintOrigin> = Vec::new();
            let mut cursor = fn_node.walk();
            loop {
                let n = cursor.node();
                if n.kind() == "member_expression" || n.kind() == "subscript_expression" {
                    let text = &src[n.start_byte()..n.end_byte()];
                    let patterns = spec
                        .map(|s| s.known_source_patterns().to_vec())
                        .unwrap_or_else(|| vec![]);
                    for pattern in patterns {
                        if text.contains(pattern) {
                            let origin = crate::corpus::source_sink::taint_source_origin(pattern);
                            seen_origins.push(origin.clone());
                            if let Some(child) = n
                                .child_by_field_name("property")
                                .or_else(|| n.child(n.child_count().saturating_sub(1)))
                            {
                                let name = &src[child.start_byte()..child.end_byte()];
                                reg.taint(name, origin);
                            }
                            break;
                        }
                    }
                }
                if cursor.goto_first_child() {
                    continue;
                }
                loop {
                    if cursor.goto_next_sibling() {
                        break;
                    }
                    if !cursor.goto_parent() {
                        let dominant = seen_origins
                            .into_iter()
                            .find(|o| !matches!(o, TaintOrigin::UserInput));
                        return Some((
                            TaintMetrics::compute(&reg, fn_node, src, &weighted_fp.function_name),
                            dominant.unwrap_or(TaintOrigin::UserInput),
                        ));
                    }
                }
            }
        });

        // Parallel scoring: each candidate scored independently, then merged
        let mut matches: Vec<PatternMatch> = all_candidates
            .par_iter()
            .filter_map(|&(idx, has_flow_match)| {
                let pattern = &self.patterns[idx];
                self.score_candidate(
                    pattern,
                    idx,
                    has_flow_match,
                    &weighted_fp,
                    func_node,
                    source,
                    fp,
                    actual_context,
                    &taint_metrics,
                    spec,
                )
            })
            .collect();

        matches.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        matches
    }

    /// Score a single candidate pattern against the function fingerprint.
    /// Uses semantic filter + taint-based scoring (no structural similarity).
    fn score_candidate(
        &self,
        pattern: &CorpusPattern,
        _idx: usize,
        has_flow_match: bool,
        weighted_fp: &FunctionFingerprint,
        _func_node: Option<tree_sitter::Node>,
        _source: Option<&str>,
        _fp: &FunctionFingerprint,
        _actual_context: Option<&crate::context::FileContext>,
        taint_metrics: &Option<(TaintMetrics, TaintOrigin)>,
        spec: Option<&dyn frensense_lang::spec::LanguageSpec>,
    ) -> Option<PatternMatch> {
        // Semantic gate
        if weighted_fp.control_flow_hashes.is_empty() && weighted_fp.api_calls.is_empty() {
            return None;
        }

        let candidate_role =
            crate::function_role::classify_role_with_imports(weighted_fp, None, spec);
        if let Some(first_pos) = pattern.positives.first() {
            let pattern_role = crate::function_role::classify_role(first_pos);
            if crate::function_role::roles_are_incompatible(candidate_role, pattern_role) {
                return None;
            }
        }

        let pattern_has_flow = pattern
            .positives
            .iter()
            .any(|pos| !pos.data_flow_path_hashes.is_empty());
        if !has_flow_match && pattern_has_flow {
            return None;
        }

        let has_api_overlap = if !pattern.positives.is_empty() && !weighted_fp.api_calls.is_empty()
        {
            pattern.positives.iter().any(|pos| {
                pos.api_calls
                    .iter()
                    .any(|h| weighted_fp.api_calls.contains(h))
            })
        } else {
            false
        };

        if !has_api_overlap && !has_flow_match {
            return None;
        }

        let mut score: f64 = 0.0;
        let is_missing_call = pattern.id.contains("MISSING") || pattern.id.contains("CONFIG");
        let mut is_taint_confirmed = false;

        if let Some((tm, origin)) = taint_metrics {
            if tm.tainted_uses > 0 {
                is_taint_confirmed = true;
                score = 0.65;
                if tm.has_validation_name {
                    score *= 0.5;
                }
                if tm.taint_branch_ratio > 0.7 {
                    score *= 0.6;
                }
                if let Some(cat) = crate::corpus::source_sink::infer_sink_category(&pattern.id) {
                    let relevance = crate::corpus::source_sink::sink_taint_relevance(cat, origin);
                    score *= relevance;
                }
            }
        }

        if !is_taint_confirmed {
            if is_missing_call {
                score = 0.50;
            } else {
                score = 0.45; // High enough to pass L1 threshold (0.40), but will be dropped by composition (0.55) if taint doesn't confirm it
            }
        }

        let threshold = self.threshold_for_pattern(&pattern.id);

        if score >= threshold {
            Some(PatternMatch {
                pattern_id: pattern.id.clone(),
                score,
                observation: pattern.observation.clone(),
                impact: pattern.impact.clone(),
                improvement: pattern.improvement.clone(),
                cwe: pattern.cwe.clone(),
                cvss: pattern.cvss,
                owasp: pattern.owasp.clone(),
                severity: pattern.severity.clone(),
                runtime_probe: pattern.runtime_probe.clone(),
                taint_branch_ratio: match taint_metrics {
                    Some((tm, _)) if tm.tainted_uses > 0 => Some(tm.taint_branch_ratio as f64),
                    _ => None,
                },
                has_validation_name: taint_metrics
                    .as_ref()
                    .map(|(tm, _)| tm.has_validation_name)
                    .unwrap_or(false),
            })
        } else {
            None
        }
    }
}
