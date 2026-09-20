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
    pub file_path: Option<&'a std::path::Path>,
    pub actual_context: Option<&'a crate::context::FileContext>,
    pub spec: Option<&'a dyn frensense_lang::spec::LanguageSpec>,
}

pub struct PatternRegistry {
    patterns: Vec<CorpusPattern>,
    api_index: Option<FxHashMap<u64, Vec<usize>>>,
    flow_index: Option<FxHashMap<u64, Vec<usize>>>,
    label_index: Option<FxHashMap<u64, Vec<usize>>>,
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
            label_index: None,
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
        let mut label_index: FxHashMap<u64, Vec<usize>> = FxHashMap::default();

        for (i, pattern) in self.patterns.iter().enumerate() {
            // Index semantic labels
            for sink_label in &pattern.sink_labels {
                label_index.entry(hash_string(&format!("{:?}", sink_label))).or_default().push(i);
            }
            for origin in &pattern.required_origins {
                label_index.entry(hash_string(&format!("{:?}", origin))).or_default().push(i);
            }
            for cat in &pattern.required_package_categories {
                label_index.entry(hash_string(&format!("{:?}", cat))).or_default().push(i);
            }
        
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
        self.label_index = Some(label_index);
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
        
        // Label candidates disconnected from gating
        // They are preserved in the pattern struct for contextual scoring later,
        // but no longer force unrelated functions into the candidate pool.

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
            // Merge: Flow candidates > API candidates (Labels disconnected from initial gating)
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
                    ctx.file_path,
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
    fn score_candidate<'a>(
        &self,
        pattern: &CorpusPattern,
        _pattern_idx: usize,
        has_flow_match: bool,
        weighted_fp: &FunctionFingerprint,
        func_node: Option<tree_sitter::Node<'a>>,
        source: Option<&'a str>,
        ctx_file_path: Option<&'a std::path::Path>,
        _fp: &FunctionFingerprint,
        actual_context: Option<&'a crate::context::FileContext>,
        taint_metrics: &Option<(TaintMetrics, TaintOrigin)>,
        spec: Option<&'a dyn frensense_lang::spec::LanguageSpec>,
    ) -> Option<PatternMatch> {
        let mut base_score: f64 = 0.0;
        
        // 1. Base score (up to 0.4): Fingerprint Jaccard similarity (api_calls + api_call_segments)
        // weighted_fp already has IDF weights applied. We need to compare it to the pattern's positives.
        let mut best_jaccard = 0.0;
        for pos in &pattern.positives {
            let mut intersection = 0.0;
            let mut union = 0.0;
            
            // Collect all unique tokens from both
            let mut all_tokens = std::collections::HashSet::new();
            for call in &weighted_fp.api_calls { all_tokens.insert(*call); }
            for call in &weighted_fp.api_call_segments { all_tokens.insert(*call); }
            for call in &pos.api_calls { all_tokens.insert(*call); }
            for call in &pos.api_call_segments { all_tokens.insert(*call); }
            
            for token in all_tokens {
                let weight = self.idf_weights.get(&token).copied().unwrap_or(1.0) as f64;
                let in_candidate = weighted_fp.api_calls.contains(&token) || weighted_fp.api_call_segments.contains(&token);
                let in_pos = pos.api_calls.contains(&token) || pos.api_call_segments.contains(&token);
                
                if in_candidate && in_pos {
                    intersection += weight;
                    union += weight;
                } else if in_candidate || in_pos {
                    union += weight;
                }
            }
            
            let jaccard = if union > 0.0 { intersection / union } else { 0.0 };
            if jaccard > best_jaccard {
                best_jaccard = jaccard;
            }
        }
        // base_score is kept as pure jaccard here
        base_score = best_jaccard;
        
        // 2. Semantic match
        let mut semantic_score: f64 = 0.0;
        let mut derived_sink_labels = std::collections::HashSet::new();
        let mut derived_origins = std::collections::HashSet::new();
        
        if let Some(sp) = spec {
            for (sink_name, label) in sp.known_sink_names() {
                if weighted_fp.raw_call_names.iter().any(|c| c.contains(sink_name)) {
                    derived_sink_labels.insert(*label);
                }
            }
            for param in &weighted_fp.param_names {
                if let Some(o) = sp.classify_param_taint(Some(param), None) {
                    derived_origins.insert(o);
                }
            }
        }
        
        if pattern.sink_labels.iter().any(|l| derived_sink_labels.contains(l)) {
            semantic_score += 0.3;
        }
        
        if pattern.required_origins.iter().any(|o| derived_origins.contains(o)) {
            semantic_score += 0.3;
        }
        
        // Bonus for SemanticFilter passing
        if let Some(ref filter) = pattern.semantic_filter {
            let passed = if let (Some(n), Some(s)) = (func_node, source) {
                filter.matches(n, s, None, None, spec)
            } else {
                true // assume pass if no AST context available
            };
            if passed {
                semantic_score += 0.2;
            }
        }
        
        // Calculate flow_score based on discriminating_flow_hashes
        let mut best_flow_ratio = 0.0;
        let candidate_flow_hashes: std::collections::HashSet<_> = weighted_fp.data_flow_path_hashes.iter().copied().collect();
        let c_len = candidate_flow_hashes.len() as f64;
        
        if !pattern.discriminating_flow_hashes.is_empty() {
            let p_len = pattern.discriminating_flow_hashes.len() as f64;
            let mut matches = 0.0;
            for hash in &pattern.discriminating_flow_hashes {
                if candidate_flow_hashes.contains(hash) {
                    matches += 1.0;
                }
            }
            if p_len > 0.0 {
                best_flow_ratio = matches / p_len.max(c_len);
            }
        } else if has_flow_match {
            // Fallback for patterns that matched the flow index but have no discriminating hashes (e.g. empty negs)
            best_flow_ratio = 1.0;
        }
        
        let flow_score = best_flow_ratio;
        
        // Final score weighting
        // If there's no flow score, but we have strong semantic/base matches, allow it to pass for structural patterns
        // But flow_score strongly dominates.
        let mut total_score = if !pattern.discriminating_flow_hashes.is_empty() || has_flow_match {
            (flow_score * 0.6) + (base_score * 0.25) + (semantic_score * 0.15)
        } else {
            (base_score * 0.6) + (semantic_score * 0.4) // Fallback for purely structural/config patterns with no data flow
        };
        
        // Reject if there is no evidence signal at all.
        if total_score == 0.0 {
            return None;
        }
        
        // Wire TaintConfidenceAdjuster - only run if score is reasonably high to avoid costly PDG rebuilding
        if total_score >= 0.5 {
            if let Some(s) = source {
                let file_path = ctx_file_path.unwrap_or_else(|| std::path::Path::new("unknown"));
                let line = func_node.map(|n| n.start_position().row as u32).unwrap_or(0);
                
                // Extract the specific sink call for line_content if possible, 
                // otherwise just use the function name
                let line_content = weighted_fp.api_calls.iter().next().map(|&h| h.to_string()).unwrap_or_default();
                
                total_score = crate::data_flow::confidence::TaintConfidenceAdjuster::adjust_confidence(
                    s,
                    file_path,
                    line,
                    &line_content,
                    total_score as f32,
                    &self.source_sink,
                    None // local_tainted_vars
                ) as f64;
            }
        }

        // 3. Sanitizer mitigation
        let has_validation_name = taint_metrics.as_ref().map_or(false, |(tm, _)| tm.has_validation_name);
        if has_validation_name && pattern.mitigating_sanitizer.is_some() {
            total_score = total_score.min(0.5); // Cap at 0.5 to prevent match
        }
        
        let thres = self.threshold_overrides.get(&pattern.id).copied().unwrap_or(self.threshold);
        if total_score >= thres {
            Some(PatternMatch {
                pattern_id: pattern.id.clone(),
                score: total_score,
                observation: pattern.observation.clone(),
                impact: pattern.impact.clone(),
                improvement: pattern.improvement.clone(),
                cwe: pattern.cwe.clone(),
                cvss: pattern.cvss,
                owasp: pattern.owasp.clone(),
                severity: pattern.severity.clone(),
                runtime_probe: pattern.runtime_probe.clone(),
                taint_branch_ratio: taint_metrics.as_ref().map(|(tm, _)| tm.taint_branch_ratio as f64),
                has_validation_name,
            })
        } else {
            None
        }
    }
}


fn hash_string(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = rustc_hash::FxHasher::default();
    s.hash(&mut hasher);
    hasher.finish()
}
