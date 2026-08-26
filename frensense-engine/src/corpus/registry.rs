// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::corpus::loader::{CorpusPattern, load_corpus};
use crate::corpus::source_sink::{CorpusSourceSinkRegistry, build_registry_from_dir};
use crate::data_flow::taint_metrics::TaintMetrics;
use crate::data_flow::{TaintOrigin, TaintRegistry};
use crate::fingerprint::{FunctionFingerprint, apply_idf_weights, compute_idf_weights};
use crate::minhash::{LSHIndex, minhash_signature};
use crate::pattern::evidence::MatchEvidence;
use crate::pattern::scorer::PatternScorer;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub score: f64,
    pub positive_similarity: f64,
    pub negative_similarity: f64,
    pub observation: Option<String>,
    pub impact: Option<String>,
    pub improvement: Option<String>,
    /// Detailed per-dimension breakdown of why this match scored as it did.
    /// Always `Some` for corpus matches; `None` for rule-based matches.
    pub matched_evidence: Option<MatchEvidence>,
    pub cwe: Option<String>,
    pub cvss: Option<f32>,
    pub owasp: Option<String>,
    pub severity: Option<String>,
    pub runtime_probe: Option<String>,
    /// Taint branch ratio from `TaintMetrics` — fraction of tainted accesses
    /// that are branched on. Propagated to the composition layer.
    pub taint_branch_ratio: Option<f64>,
    /// Whether the function name suggests a validator/sanitizer.
    pub has_validation_name: bool,
}

#[derive(Default)]
pub struct PatternRegistry {
    patterns: Vec<CorpusPattern>,
    lsh_index: Option<LSHIndex>,
    lsh_index_api: Option<LSHIndex>,
    threshold: f64,
    ngram_sim_threshold: f64,
    struct_overlap_threshold: f64,
    threshold_overrides: std::collections::HashMap<String, f64>,
    idf_weights: FxHashMap<u64, f32>,
    api_idf_weights: FxHashMap<u64, f32>,
    /// Per-category learned feature weights (trained at build time or loaded from bundle).
    pub category_weights: std::collections::HashMap<String, [f64; 15]>,
    /// Auto-derived semantic filter suggestions (import + call exclusivity).
    pub auto_filter_stats: Option<crate::auto_filter::AutoFilterStats>,
    /// Per-pattern sigmoid calibration (A, B) parameters, keyed by pattern id.
    pub pattern_calibration: std::collections::HashMap<String, (f32, f32)>,
    /// Learned semantic markers: maps API call name → semantic category.
    /// Built by discovering which API calls appear in which pattern categories
    /// across the corpus. Merged with hardcoded markers during scanning.
    pub learned_semantic_markers: std::collections::HashMap<String, String>,
    source_sink: CorpusSourceSinkRegistry,
}

impl PatternRegistry {
    pub fn new(threshold: f64, ngram_sim_threshold: f64, struct_overlap_threshold: f64) -> Self {
        Self {
            patterns: Vec::new(),
            lsh_index: None,
            lsh_index_api: None,
            threshold,
            ngram_sim_threshold,
            struct_overlap_threshold,
            threshold_overrides: std::collections::HashMap::new(),
            idf_weights: FxHashMap::default(),
            api_idf_weights: FxHashMap::default(),
            category_weights: std::collections::HashMap::new(),
            auto_filter_stats: None,
            pattern_calibration: std::collections::HashMap::new(),
            learned_semantic_markers: std::collections::HashMap::new(),
            source_sink: CorpusSourceSinkRegistry::default(),
        }
    }

    pub fn load_corpus(&mut self, corpus_dir: &Path) -> crate::Result<usize> {
        let patterns = load_corpus(corpus_dir)
            .map_err(|e| crate::FrensenseError::Engine(e))?
            .0;
        let count = patterns.len();
        self.source_sink = build_registry_from_dir(corpus_dir);
        self.patterns = patterns;
        self.compute_and_apply_idf();
        self.build_lsh_index();
        Ok(count)
    }

    pub fn load_corpus_dirs(&mut self, dirs: &[&Path]) -> crate::Result<usize> {
        let mut all_patterns = Vec::new();
        for dir in dirs {
            match load_corpus(dir) {
                Ok((patterns, _warnings)) => all_patterns.extend(patterns),
                Err(ref e) => eprintln!("Corpus warning: skipping {}: {e}", dir.display()),
            }
        }
        // Build source/sink registry from the first corpus dir (primary)
        if let Some(&dir) = dirs.first() {
            self.source_sink = build_registry_from_dir(dir);
        }
        let count = all_patterns.len();
        self.patterns = all_patterns;
        self.compute_and_apply_idf();
        self.build_lsh_index();
        // Compute auto-filter stats from loaded patterns (fallback when bundle unavailable)
        if self.auto_filter_stats.is_none() {
            self.compute_auto_filter_stats(dirs);
        }
        Ok(count)
    }

    /// Compute auto-derived semantic filter suggestions from corpus files.
    /// Only used as a fallback when the embedded bundle doesn't contain them.
    fn compute_auto_filter_stats(&mut self, dirs: &[&Path]) {
        use std::collections::HashMap;
        let mut source_texts = HashMap::new();
        for dir in dirs {
            collect_source_texts(dir, &mut source_texts);
        }
        if source_texts.is_empty() {
            return;
        }
        // Convert patterns to BundlePattern format for the auto-filter function
        let bundle_patterns: Vec<crate::corpus::bundle::BundlePattern> = self
            .patterns
            .iter()
            .map(|p| crate::corpus::bundle::BundlePattern {
                id: p.id.clone(),
                positives: p.positives.clone(),
                negatives: p.negatives.clone(),
                semantic_filter: p.semantic_filter.clone(),
                observation: p.observation.clone(),
                impact: p.impact.clone(),
                improvement: p.improvement.clone(),
                expected_context: p.expected_context.clone(),
                cwe: p.cwe.clone(),
                cvss: p.cvss,
                owasp: p.owasp.clone(),
                severity: p.severity.clone(),
                runtime_probe: p.runtime_probe.clone(),
            })
            .collect();
        let stats = crate::auto_filter::compute_auto_filters(&bundle_patterns, &source_texts);
        self.auto_filter_stats = Some(stats);
    }

    /// Get the corpus-learned source/sink registry.
    pub fn source_sink_registry(&self) -> &CorpusSourceSinkRegistry {
        &self.source_sink
    }

    #[cfg(feature = "serialize")]
    pub fn load_from_bundle(&mut self, bytes: &[u8]) -> crate::Result<usize> {
        let loaded = crate::corpus::bundle::load_bundle(bytes)
            .map_err(|e| crate::FrensenseError::Engine(e))?;
        let count = loaded.patterns.len();

        self.patterns = loaded
            .patterns
            .into_iter()
            .map(|bp| CorpusPattern {
                id: bp.id.clone(),
                positives: bp.positives,
                negatives: bp.negatives,
                semantic_filter: bp.semantic_filter,
                observation: bp.observation,
                impact: bp.impact,
                improvement: bp.improvement,
                expected_context: bp.expected_context,
                cwe: bp.cwe.clone(),
                cvss: bp.cvss,
                owasp: bp.owasp.clone(),
                severity: bp.severity.clone(),
                runtime_probe: bp.runtime_probe.clone(),
            })
            .collect();

        // Use pre-computed API IDF from bundle when available (avoids recomputation)
        if !loaded.api_idf_weights.is_empty() {
            self.api_idf_weights = loaded.api_idf_weights.into_iter().collect();
        }

        // Restore per-category feature weights from bundle
        if !loaded.category_weights.is_empty() {
            self.category_weights = loaded.category_weights.into_iter().collect();
        }

        // Restore auto-derived filter suggestions from bundle
        // Bundle format: (pid, imports, calls, excl_calls, fn_re, excl_nodes, excl_fnames)
        if !loaded.auto_filter_stats.is_empty() {
            let mut contains_call_to = std::collections::HashMap::new();
            let mut excludes_call = std::collections::HashMap::new();
            let mut excludes_node_type = std::collections::HashMap::new();
            let mut excludes_function_name = std::collections::HashMap::new();
            for entry in loaded.auto_filter_stats {
                let pid = entry.0;
                let calls = entry.2;
                let excl_calls = entry.3;
                let excl_nodes = entry.5;
                let excl_fnames = entry.6;
                if !calls.is_empty() {
                    contains_call_to.insert(pid.clone(), calls);
                }
                if !excl_calls.is_empty() {
                    excludes_call.insert(pid.clone(), excl_calls);
                }
                if !excl_nodes.is_empty() {
                    excludes_node_type.insert(pid.clone(), excl_nodes);
                }
                if !excl_fnames.is_empty() {
                    excludes_function_name.insert(pid.clone(), excl_fnames);
                }
            }
            self.auto_filter_stats = Some(crate::auto_filter::AutoFilterStats {
                contains_call_to,
                excludes_call,
                function_name_regex: std::collections::HashMap::new(),
                excludes_node_type,
                excludes_function_name,
            });
        }

        self.apply_ngram_idf();
        // compute_api_idf skipped when weights came from the bundle
        if self.api_idf_weights.is_empty() {
            self.compute_api_idf();
        }
        self.build_lsh_index();
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

    /// Learn per-category feature weights and per-pattern calibration from corpus positive/negative pairs.
    fn compute_category_weights(&mut self) {
        // Only compute if not already loaded from bundle
        if self.category_weights.is_empty() {
            self.category_weights =
                crate::pattern::weight_learner::learn_category_weights(&self.patterns);
        }
        if self.pattern_calibration.is_empty() {
            self.pattern_calibration =
                crate::per_pattern_calibration::train_per_pattern_calibration(&self.patterns);
        }
    }

    /// Learn semantic markers from corpus patterns.
    fn compute_learned_semantic_markers(&mut self) {
        if self.learned_semantic_markers.is_empty() {
            self.learned_semantic_markers = learn_semantic_markers(&self.patterns);
        }
    }

    /// Run both IDF passes, learn category weights, and learn semantic markers.
    /// Called after `load_corpus` / `load_corpus_dirs`.
    fn compute_and_apply_idf(&mut self) {
        self.apply_ngram_idf();
        self.compute_api_idf();
        self.compute_category_weights();
        self.compute_learned_semantic_markers();
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn set_threshold_override(&mut self, category: String, threshold: f64) {
        self.threshold_overrides.insert(category, threshold);
    }

    /// Override per-category feature weights. Used to calibrate detection for
    /// specific vulnerability classes without retraining the full corpus.
    pub fn set_category_weights(&mut self, category: &str, weights: [f64; 15]) {
        self.category_weights.insert(category.to_string(), weights);
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

    fn build_lsh_index(&mut self) {
        if self.patterns.len() < 10 {
            return;
        }
        let num_hashes = 120;
        // Standard LSH: bands=40, rows=3.  Gives ~95%+ recall for J≥0.4
        // while filtering out most J<0.2 false positives.
        let num_bands = 40;
        let rows_per_band = 3;

        // Structural LSH (existing)
        let mut struct_index = LSHIndex::new(num_bands, rows_per_band);
        // API-call LSH (new — helps distinguish patterns by what they call)
        let mut api_index = LSHIndex::new(num_bands, rows_per_band);

        for (i, pattern) in self.patterns.iter().enumerate() {
            // Issue 6 fix: index ALL positives, not just the first.
            // Multi-function corpus patterns have a module-scope fingerprint AND
            // a callback fingerprint. Only indexing first() means the callback is
            // invisible to LSH queries, so pattern never matches the candidate callback.
            for fp in &pattern.positives {
                // Structural signature
                let sig_s = minhash_signature(&fp.structural_markers, num_hashes);
                struct_index.insert(&sig_s, i as u64);

                // API-call signature: use segments-only for recall on chained calls
                // where full call text differs (e.g. .then chain includes different query args).
                // Segments capture just the method name (query, then, catch, json, ...).
                let sig_a = if !fp.api_call_segments.is_empty() {
                    minhash_signature(&fp.api_call_segments, num_hashes)
                } else if !fp.api_calls.is_empty() {
                    minhash_signature(&fp.api_calls, num_hashes)
                } else {
                    minhash_signature(&fp.structural_markers, num_hashes)
                };
                api_index.insert(&sig_a, i as u64);
            }
        }
        self.lsh_index = Some(struct_index);
        self.lsh_index_api = Some(api_index);
    }

    pub fn scan_function(
        &self,
        fp: &FunctionFingerprint,
        func_node: Option<tree_sitter::Node<'_>>,
        source: Option<&str>,
        actual_context: Option<&crate::context::FileContext>,
    ) -> Vec<PatternMatch> {
        // Query both LSH tables (structural + API-call)
        let struct_candidates: std::collections::HashSet<usize> =
            if let Some(ref lsh) = self.lsh_index {
                let sig = minhash_signature(&fp.structural_markers, 128);
                lsh.query(&sig)
                    .iter()
                    .map(|&id| id as usize)
                    .filter(|&id| id < self.patterns.len())
                    .collect()
            } else {
                (0..self.patterns.len()).collect()
            };
        let api_candidates: std::collections::HashSet<usize> =
            if let Some(ref lsh) = self.lsh_index_api {
                let sig = if !fp.api_call_segments.is_empty() {
                    minhash_signature(&fp.api_call_segments, 128)
                } else if !fp.api_calls.is_empty() {
                    minhash_signature(&fp.api_calls, 128)
                } else {
                    minhash_signature(&fp.structural_markers, 128)
                };
                lsh.query(&sig)
                    .iter()
                    .map(|&id| id as usize)
                    .filter(|&id| id < self.patterns.len())
                    .collect()
            } else {
                struct_candidates.clone()
            };

        // Merge: a candidate passes if it's in EITHER table (preserve recall).
        // Track which table(s) it passed through for penalty application.
        let all_candidates: Vec<(usize, bool)> = {
            let mut seen = std::collections::HashSet::new();
            let mut merged = Vec::new();
            for &id in struct_candidates.iter().chain(api_candidates.iter()) {
                if seen.insert(id) {
                    let hit_both = struct_candidates.contains(&id) && api_candidates.contains(&id);
                    merged.push((id, hit_both));
                }
            }
            merged
        };

        // Apply IDF weights to candidate fingerprint for scoring
        let mut weighted_fp = fp.clone();
        if !self.idf_weights.is_empty() {
            apply_idf_weights(&mut weighted_fp, &self.idf_weights);
        }

        // Pre-compute data flows once (shared across all candidates, cheap AST walk)
        let precomputed_flows = func_node.and_then(|node| {
            let src = source?;
            let needs_flows = self.patterns.iter().any(|p| {
                p.semantic_filter
                    .as_ref()
                    .map_or(false, |f| !f.required_taint_flows.is_empty())
            });
            if needs_flows {
                Some(crate::corpus::data_flow_extractor::extract_data_flows(
                    node, src,
                ))
            } else {
                None
            }
        });

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
                    for &pattern in crate::corpus::loader::TAINT_SOURCE_PATTERNS {
                        if text.contains(pattern) {
                            let origin = crate::corpus::loader::taint_source_origin(pattern);
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
            .filter_map(|&(idx, hit_both)| {
                let pattern = &self.patterns[idx];
                self.score_candidate(
                    pattern,
                    idx,
                    hit_both,
                    &weighted_fp,
                    func_node,
                    source,
                    fp,
                    actual_context,
                    &taint_metrics,
                    precomputed_flows.as_ref(),
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
    /// Extracted from the scan loop for rayon parallelism.
    fn score_candidate(
        &self,
        pattern: &CorpusPattern,
        _idx: usize,
        hit_both: bool,
        weighted_fp: &FunctionFingerprint,
        func_node: Option<tree_sitter::Node>,
        source: Option<&str>,
        fp: &FunctionFingerprint,
        actual_context: Option<&crate::context::FileContext>,
        taint_metrics: &Option<(TaintMetrics, TaintOrigin)>,
        precomputed_flows: Option<&std::collections::HashSet<(String, String)>>,
    ) -> Option<PatternMatch> {
        // Merge hand-authored semantic filter with auto-derived suggestions
        let merged_filter = match (&pattern.semantic_filter, &self.auto_filter_stats) {
            (Some(hand), Some(auto)) => Some(crate::auto_filter::merge_filters(
                hand,
                Some(auto),
                &pattern.id,
            )),
            (Some(hand), None) => Some(hand.clone()),
            (None, Some(auto)) => Some(crate::auto_filter::merge_filters(
                &Default::default(),
                Some(auto),
                &pattern.id,
            )),
            (None, None) => None,
        };

        // Apply semantic filter if present
        if let (Some(filter), Some(node), Some(src)) = (merged_filter.as_ref(), func_node, source) {
            if !filter.matches(node, src, Some(fp.file_path.as_str()), precomputed_flows) {
                return None;
            }
        }

        // Semantic gate: skip trivially small functions
        if weighted_fp.structural_markers.len() < 3
            || (weighted_fp.control_flow_hashes.is_empty() && weighted_fp.api_calls.is_empty())
        {
            return None;
        }

        // Function role classifier
        let candidate_role = crate::function_role::classify_role(weighted_fp);
        if let Some(first_pos) = pattern.positives.first() {
            let pattern_role = crate::function_role::classify_role(first_pos);
            if crate::function_role::roles_are_incompatible(candidate_role, pattern_role) {
                return None;
            }
        }

        // Structural overlap gate
        if !pattern.positives.is_empty() {
            let struct_sim = pattern
                .positives
                .iter()
                .map(|p| {
                    crate::minhash::overlap_coefficient_sorted(
                        &weighted_fp.structural_markers,
                        &p.structural_markers,
                    )
                })
                .fold(0.0f64, f64::max);
            if struct_sim < self.struct_overlap_threshold {
                return None;
            }
        }

        // API-call gate
        let gate_pos = pattern
            .positives
            .iter()
            .filter(|p| !p.api_calls.is_empty())
            .max_by_key(|p| {
                p.api_calls
                    .iter()
                    .filter(|h| weighted_fp.api_calls.contains(h))
                    .count()
            });
        if let Some(gate_pos) = gate_pos {
            let api_overlap = !weighted_fp.api_calls.is_empty()
                && gate_pos
                    .api_calls
                    .iter()
                    .any(|h| weighted_fp.api_calls.contains(h));
            let motif_overlap = !weighted_fp.motif_hashes.is_empty()
                && !gate_pos.motif_hashes.is_empty()
                && gate_pos
                    .motif_hashes
                    .iter()
                    .any(|h| weighted_fp.motif_hashes.contains(h));
            if !api_overlap && !motif_overlap {
                if !weighted_fp.api_calls.is_empty() && !self.api_idf_weights.is_empty() {
                    let top_calls: Vec<u64> = {
                        let mut scored: Vec<(u64, f32)> = gate_pos
                            .api_calls
                            .iter()
                            .filter_map(|h| self.api_idf_weights.get(h).map(|idf| (*h, *idf)))
                            .collect();
                        scored.sort_by(|a, b| {
                            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        scored.into_iter().take(3).map(|(h, _)| h).collect()
                    };
                    if top_calls.iter().all(|h| !weighted_fp.api_calls.contains(h)) {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }

        let mut dim_cache = FxHashMap::default();
        let pat_weights =
            crate::pattern::weight_learner::category_weights(&pattern.id, &self.category_weights);
        let (best_score, evidence) = PatternScorer::score_against_corpus_with_evidence_cached(
            weighted_fp,
            &pattern.positives,
            &pattern.negatives,
            pattern.expected_context.as_ref(),
            actual_context,
            self.ngram_sim_threshold,
            pat_weights,
            &mut dim_cache,
        );

        let best_score = if !hit_both {
            best_score * 0.85
        } else {
            best_score
        };

        let best_score = crate::per_pattern_calibration::calibrate(
            best_score,
            self.pattern_calibration.get(&pattern.id),
        );

        let best_score = if let Some((tm, origin)) = taint_metrics {
            let mut multiplier: f64 = 1.0;
            if tm.is_hollow_validator() {
                multiplier = 0.5;
            } else if tm.taint_branch_ratio > 0.5 {
                multiplier = 0.8;
            }
            if let Some(cat) = crate::corpus::source_sink::infer_sink_category(&pattern.id) {
                let relevance: f64 = crate::corpus::source_sink::sink_taint_relevance(cat, origin);
                multiplier = multiplier.min(relevance);
            }
            best_score * multiplier
        } else {
            best_score
        };

        let threshold = self.threshold_for_pattern(&pattern.id);
        let has_taint = evidence.has_taint_path;
        let effective_threshold = if has_taint {
            threshold.min(0.15)
        } else {
            threshold
        };
        if best_score >= effective_threshold {
            Some(PatternMatch {
                pattern_id: pattern.id.clone(),
                score: best_score,
                positive_similarity: evidence.api_sim,
                negative_similarity: evidence.negative_sim,
                observation: pattern.observation.clone(),
                impact: pattern.impact.clone(),
                improvement: pattern.improvement.clone(),
                matched_evidence: Some(evidence),
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

/// Learn semantic category markers from corpus patterns.
///
/// Walks all positive fingerprints' raw call names and groups them by
/// pattern category (second segment of pattern ID: "sec", "csa", etc.).
/// Any API call name appearing in ≥2 distinct patterns of the same category
/// is promoted to a learned semantic marker that maps `api_name → category`.
///
/// During scanning, these are merged into fingerprint semantic_markers so
/// that matching code produces the same semantic-sim signal as the hardcoded
/// markers would.
pub fn learn_semantic_markers(
    patterns: &[CorpusPattern],
) -> std::collections::HashMap<String, String> {
    use std::collections::HashMap;
    // API call name → set of (category, pattern_count)
    let mut api_to_cats: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for pattern in patterns {
        let cat = pattern
            .id
            .split('_')
            .nth(1)
            .unwrap_or("unknown")
            .to_string();
        for fp in &pattern.positives {
            for call in &fp.raw_call_names {
                // Use the last segment (method name) for generalization
                let seg = call
                    .rsplit(|c: char| c == '.' || c == ':')
                    .next()
                    .unwrap_or(call)
                    .to_string();
                let entry = api_to_cats.entry(seg).or_default();
                *entry.entry(cat.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut result = HashMap::new();
    for (api_name, category_counts) in &api_to_cats {
        // Skip very common method names that would be noise
        const NOISE_NAMES: &[&str] = &[
            "then", "catch", "json", "next", "toString", "map", "filter", "forEach", "find",
            "sort", "join", "split", "trim", "log", "error", "send", "status",
        ];
        if NOISE_NAMES.contains(&api_name.as_str()) {
            continue;
        }
        for (cat, count) in category_counts {
            if *count >= 2 {
                result.insert(api_name.clone(), cat.clone());
            }
        }
    }
    result
}

/// Recursively collect source texts from a corpus directory for auto-filter computation.
fn collect_source_texts(
    dir: &std::path::Path,
    out: &mut std::collections::HashMap<String, String>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_source_texts(&path, out);
        } else if path.is_file() {
            let fname = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            // Only collect positive and negative files
            if fname.contains("_positive") || fname.contains("_negative") {
                if let Ok(src) = std::fs::read_to_string(&path) {
                    out.insert(fname.to_string(), src);
                }
            }
        }
    }
}
