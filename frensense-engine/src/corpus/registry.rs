// SPDX-License-Identifier: MIT

use crate::corpus::pattern::CorpusPattern;
use crate::corpus::source_sink::CorpusSourceSinkRegistry;
use crate::data_flow::taint_metrics::TaintMetrics;
use crate::data_flow::{TaintOrigin, TaintRegistry};
use crate::fingerprint::FunctionFingerprint;
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
    /// Auto-derived semantic filter suggestions (import + call exclusivity).
    pub api_idf_weights: rustc_hash::FxHashMap<u64, f32>,
    pub auto_filter_stats: Option<crate::auto_filter::AutoFilterStats>,
    source_sink: CorpusSourceSinkRegistry,
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
            api_idf_weights: rustc_hash::FxHashMap::default(),
            auto_filter_stats: None,
            source_sink: CorpusSourceSinkRegistry::default(),
        }
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

        // compute_api_idf skipped when weights came from the bundle
        if self.api_idf_weights.is_empty() {
            self.compute_api_idf();
        }
        self.build_index();
        Ok(count)
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
                label_index
                    .entry(hash_string(&format!("{:?}", sink_label)))
                    .or_default()
                    .push(i);
            }
            for origin in &pattern.required_origins {
                label_index
                    .entry(hash_string(&format!("{:?}", origin)))
                    .or_default()
                    .push(i);
            }
            for cat in &pattern.required_package_categories {
                label_index
                    .entry(hash_string(&format!("{:?}", cat)))
                    .or_default()
                    .push(i);
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
        // (Unused)

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
                            TaintMetrics::compute(&reg, fn_node, src, &fp.function_name),
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
                    fp,
                    func_node,
                    source,
                    ctx.file_path,
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
        fp: &FunctionFingerprint,
        func_node: Option<tree_sitter::Node<'a>>,
        source: Option<&'a str>,
        ctx_file_path: Option<&'a std::path::Path>,
        _actual_context: Option<&'a crate::context::FileContext>,
        taint_metrics: &Option<(TaintMetrics, TaintOrigin)>,
        spec: Option<&'a dyn frensense_lang::spec::LanguageSpec>,
    ) -> Option<PatternMatch> {
        // 1. SemanticFilter gate (hard)
        if let Some(ref filter) = pattern.semantic_filter {
            let file_path_str = ctx_file_path.and_then(|p| p.to_str());
            let passed = if let (Some(n), Some(s)) = (func_node, source) {
                filter.matches(n, s, file_path_str, None, spec)
            } else {
                true // assume pass if no AST context available
            };
            if !passed {
                return None;
            }
        }

        // 2. Flow hash gate (hard)
        if !pattern.discriminating_flow_hashes.is_empty()
            && fp.data_flow_path_hashes.is_empty()
            && fp.tainted_api_calls.is_empty()
        {
            return None;
        }

        // 3. Negative jaccard gate (hard)
        let compute_jaccard = |fp1: &FunctionFingerprint, fp2: &FunctionFingerprint| -> f64 {
            let mut intersection = 0.0;
            let mut union = 0.0;

            let mut all_tokens = std::collections::HashSet::new();
            for call in &fp1.api_calls {
                all_tokens.insert(*call);
            }
            for call in &fp1.api_call_segments {
                all_tokens.insert(*call);
            }
            for call in &fp2.api_calls {
                all_tokens.insert(*call);
            }
            for call in &fp2.api_call_segments {
                all_tokens.insert(*call);
            }

            for token in all_tokens {
                let weight = self.api_idf_weights.get(&token).copied().unwrap_or(1.0) as f64;
                let in_1 = fp1.api_calls.contains(&token) || fp1.api_call_segments.contains(&token);
                let in_2 = fp2.api_calls.contains(&token) || fp2.api_call_segments.contains(&token);

                if in_1 && in_2 {
                    intersection += weight;
                    union += weight;
                } else if in_1 || in_2 {
                    union += weight;
                }
            }
            if union > 0.0 {
                intersection / union
            } else {
                0.0
            }
        };

        let mut pos_jaccard = 0.0;
        for pos in &pattern.positives {
            let jaccard = compute_jaccard(fp, pos);
            if jaccard > pos_jaccard {
                pos_jaccard = jaccard;
            }
        }

        let mut neg_jaccard = 0.0_f64;
        for neg in &pattern.negatives {
            let neg_j = compute_jaccard(fp, neg);
            if neg_j > neg_jaccard {
                neg_jaccard = neg_j;
            }
        }

        if neg_jaccard >= pos_jaccard && neg_jaccard > 0.0 {
            return None;
        }

        let jaccard_margin = (pos_jaccard - neg_jaccard).max(0.0);
        let base_score = pos_jaccard * (0.5 + 0.5 * jaccard_margin);

        // 4. Minimum flow ratio (hard)
        let mut flow_ratio = 0.0;
        let mut candidate_flow_hashes: std::collections::HashSet<_> =
            fp.data_flow_path_hashes.iter().copied().collect();
        candidate_flow_hashes.extend(fp.tainted_api_calls.iter().copied());
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
                flow_ratio = matches / p_len.max(c_len);
            }

            if flow_ratio < 0.2 {
                return None;
            }
        } else {
            flow_ratio = if has_flow_match { 1.0 } else { 0.0 };
        }

        // Sink relevance
        let mut sink_relevance: f64 = 0.0;
        let mut derived_sink_labels = std::collections::HashSet::new();
        let mut derived_origins = std::collections::HashSet::new();

        if let Some(sp) = spec {
            for (sink_name, label) in sp.known_sink_names() {
                if fp.raw_call_names.iter().any(|c| c.contains(sink_name)) {
                    derived_sink_labels.insert(*label);
                }
            }
            for param in &fp.param_names {
                if let Some(o) = sp.classify_param_taint(Some(param), None) {
                    derived_origins.insert(o);
                }
            }
        }

        if pattern
            .sink_labels
            .iter()
            .any(|l| derived_sink_labels.contains(l))
        {
            sink_relevance += 0.5; // Scale to 1.0 total max
        }
        if pattern
            .required_origins
            .iter()
            .any(|o| derived_origins.contains(o))
        {
            sink_relevance += 0.5;
        }

        // 5. Compute base score
        let mut total_score = if !pattern.discriminating_flow_hashes.is_empty() {
            (flow_ratio * 0.55) + (base_score * 0.30) + (sink_relevance * 0.15)
        } else {
            (base_score * 0.60) + (sink_relevance * 0.40)
        };

        if total_score == 0.0 {
            return None;
        }

        // 6. Source confidence multiplier
        let source_confidence = if fp.has_confirmed_source { 1.0 } else { 0.65 };
        total_score *= source_confidence;

        // 7. Sanitizer mitigation (hard)
        let has_validation_name = taint_metrics
            .as_ref()
            .map_or(false, |(tm, _)| tm.has_validation_name);
        if has_validation_name && pattern.mitigating_sanitizer.is_some() {
            return None;
        }

        // 8. Threshold check
        let thres = self
            .threshold_overrides
            .get(&pattern.id)
            .copied()
            .unwrap_or(self.threshold);
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
                taint_branch_ratio: taint_metrics
                    .as_ref()
                    .map(|(tm, _)| tm.taint_branch_ratio as f64),
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
