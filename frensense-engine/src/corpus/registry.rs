// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::corpus::loader::{CorpusPattern, load_corpus};
use crate::corpus::source_sink::{CorpusSourceSinkRegistry, build_registry_from_dir};
use crate::fingerprint::{FunctionFingerprint, apply_idf_weights, compute_idf_weights};
use crate::minhash::{LSHIndex, minhash_signature};
use crate::pattern::evidence::MatchEvidence;
use crate::pattern::scorer::PatternScorer;
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
    pub category_weights: std::collections::HashMap<String, [f64; 11]>,
    /// Auto-derived semantic filter suggestions (import + call exclusivity).
    pub auto_filter_stats: Option<crate::auto_filter::AutoFilterStats>,
    /// Per-pattern sigmoid calibration (A, B) parameters, keyed by pattern id.
    pub pattern_calibration: std::collections::HashMap<String, (f32, f32)>,
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
            source_sink: CorpusSourceSinkRegistry::default(),
        }
    }

    pub fn load_corpus(&mut self, corpus_dir: &Path) -> Result<usize, String> {
        let patterns = load_corpus(corpus_dir)?;
        let count = patterns.len();
        self.source_sink = build_registry_from_dir(corpus_dir);
        self.patterns = patterns;
        self.compute_and_apply_idf();
        self.build_lsh_index();
        Ok(count)
    }

    pub fn load_corpus_dirs(&mut self, dirs: &[&Path]) -> Result<usize, String> {
        let mut all_patterns = Vec::new();
        for dir in dirs {
            match load_corpus(dir) {
                Ok(patterns) => all_patterns.extend(patterns),
                Err(e) => eprintln!("Corpus warning: skipping {}: {e}", dir.display()),
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
        Ok(count)
    }

    /// Get the corpus-learned source/sink registry.
    pub fn source_sink_registry(&self) -> &CorpusSourceSinkRegistry {
        &self.source_sink
    }

    #[cfg(feature = "serialize")]
    pub fn load_from_bundle(&mut self, bytes: &[u8]) -> Result<usize, String> {
        let loaded = crate::corpus::bundle::load_bundle(bytes)?;
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
            })
            .collect();

        // Use pre-computed API IDF from bundle when available (avoids recomputation)
        if !loaded.api_idf_weights.is_empty() {
            self.api_idf_weights = loaded
                .api_idf_weights
                .into_iter()
                .collect();
        }

        // Restore per-category feature weights from bundle
        if !loaded.category_weights.is_empty() {
            self.category_weights = loaded
                .category_weights
                .into_iter()
                .collect();
        }

        // Restore auto-derived filter suggestions from bundle
        if !loaded.auto_filter_stats.is_empty() {
            let mut contains_import = std::collections::HashMap::new();
            let mut contains_call_to = std::collections::HashMap::new();
            for (pid, imports, calls) in loaded.auto_filter_stats {
                if !imports.is_empty() {
                    contains_import.insert(pid.clone(), imports);
                }
                if !calls.is_empty() {
                    contains_call_to.insert(pid, calls);
                }
            }
            self.auto_filter_stats = Some(crate::auto_filter::AutoFilterStats {
                contains_import,
                contains_call_to,
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

    /// Learn per-category feature weights from corpus positive/negative pairs.
    fn compute_category_weights(&mut self) {
        // Only compute if not already loaded from bundle
        if self.category_weights.is_empty() {
            self.category_weights =
                crate::pattern::weight_learner::learn_category_weights(&self.patterns);
        }
    }

    /// Run both IDF passes and learn category weights.
    /// Called after `load_corpus` / `load_corpus_dirs`.
    fn compute_and_apply_idf(&mut self) {
        self.apply_ngram_idf();
        self.compute_api_idf();
        self.compute_category_weights();
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
            if let Some(first_pos) = pattern.positives.first() {
                // Structural signature
                let sig_s = minhash_signature(&first_pos.structural_markers, num_hashes);
                struct_index.insert(&sig_s, i as u64);

                // API-call signature (use api_calls, fall back to empty vec if none)
                let sig_a = if !first_pos.api_calls.is_empty() {
                    minhash_signature(&first_pos.api_calls, num_hashes)
                } else {
                    minhash_signature(&first_pos.structural_markers, num_hashes)
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
        let struct_candidates: std::collections::HashSet<usize> = if let Some(ref lsh) = self.lsh_index
        {
            let sig = minhash_signature(&fp.structural_markers, 128);
            lsh.query(&sig).iter().map(|&id| id as usize).filter(|&id| id < self.patterns.len()).collect()
        } else {
            (0..self.patterns.len()).collect()
        };
        let api_candidates: std::collections::HashSet<usize> = if let Some(ref lsh) = self.lsh_index_api
        {
            let sig = if !fp.api_calls.is_empty() {
                minhash_signature(&fp.api_calls, 128)
            } else {
                minhash_signature(&fp.structural_markers, 128)
            };
            lsh.query(&sig).iter().map(|&id| id as usize).filter(|&id| id < self.patterns.len()).collect()
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

        let mut extracted_flows: Option<std::collections::HashSet<(String, String)>> = None;

        // eprintln!("DEBUG: fp.name = {:?}, api = {}, candidates = {}", fp.function_name, fp.api_calls.len(), candidates.len());

        let mut matches = Vec::new();
        for &(idx, hit_both) in &all_candidates {
            let pattern = &self.patterns[idx];
            // eprintln!("DEBUG: checking pattern {}", pattern.id);
            if pattern.id.contains("blocking_io") {
                // eprintln!("DEBUG: found pattern {} is candidate.", pattern.id);
            }

            // Merge hand-authored semantic filter with auto-derived suggestions
            let merged_filter = match (&pattern.semantic_filter, &self.auto_filter_stats) {
                (Some(hand), Some(auto)) => Some(crate::auto_filter::merge_filters(
                    hand, Some(auto), &pattern.id,
                )),
                (Some(hand), None) => Some(hand.clone()),
                (None, Some(auto)) => Some(crate::auto_filter::merge_filters(
                    &Default::default(), Some(auto), &pattern.id,
                )),
                (None, None) => None,
            };

            // Apply semantic filter if present
            if let (Some(filter), Some(node), Some(src)) =
                (merged_filter.as_ref(), func_node, source)
            {
                if !filter.required_taint_flows.is_empty() && extracted_flows.is_none() {
                    extracted_flows = Some(crate::corpus::data_flow_extractor::extract_data_flows(
                        node, src,
                    ));
                }

                if !filter.matches(
                    node,
                    src,
                    Some(fp.file_path.as_str()),
                    extracted_flows.as_ref(),
                ) {
                    continue;
                }
            }

            // Semantic gate: skip trivially small functions that can't match complex patterns.
            // Uses structural marker count as a proxy for function complexity.
            // A function with < 3 structural markers is too simple to be a real vulnerability.
            // Prevents fn main() {} from matching patterns that require actual logic.
            if weighted_fp.structural_markers.len() < 3 {
                continue;
            }

            // Skip functions with no control flow AND no API calls — trivial getters/setters
            // But allow functions that have API calls (like eval()) even without control flow
            if weighted_fp.control_flow_hashes.is_empty() && weighted_fp.api_calls.is_empty() {
                continue;
            }

            // Function role classifier: if the candidate's role is incompatible with the
            // pattern's role, skip.  An HttpHandler cannot be a ShellExecutor or DbQuery.
            let candidate_role = crate::function_role::classify_role(&weighted_fp);
            if let Some(first_pos) = pattern.positives.first() {
                let pattern_role = crate::function_role::classify_role(first_pos);
                if crate::function_role::roles_are_incompatible(candidate_role, pattern_role) {
                    continue;
                }
            }

            // Fast early exit: if the candidate shares almost no structural markers with the primary positive example,
            // it's highly unlikely to be a match. We can skip the expensive full scoring.
            if let Some(first_pos) = pattern.positives.first() {
                let struct_sim = crate::minhash::overlap_coefficient_sorted(
                    &weighted_fp.structural_markers,
                    &first_pos.structural_markers,
                );

                if struct_sim < self.struct_overlap_threshold {
                    continue; // Prune this candidate early
                }

                // API-call gate: find the first positive with non-empty api_calls.
                // Helper functions (like getCommand) often appear first but have no calls.
                // The actual vulnerability typically has a distinctive API call.
                let gate_pos = pattern.positives.iter().find(|p| !p.api_calls.is_empty());
                if let Some(gate_pos) = gate_pos {
                    let api_overlap = if !weighted_fp.api_calls.is_empty() {
                        gate_pos.api_calls.iter().any(|h| weighted_fp.api_calls.contains(h))
                    } else {
                        false
                    };
                    let motif_overlap = if !weighted_fp.motif_hashes.is_empty() && !gate_pos.motif_hashes.is_empty() {
                        gate_pos.motif_hashes.iter().any(|h| weighted_fp.motif_hashes.contains(h))
                    } else {
                        false
                    };
                    if !api_overlap && !motif_overlap {
                        if !weighted_fp.api_calls.is_empty() && !self.api_idf_weights.is_empty() {
                            // Top-3 IDF gate: require at least 1 of the top-3 IDF-weighted
                            // calls from the pattern to appear in the candidate.
                            let top_calls: Vec<u64> = {
                                let mut scored: Vec<(u64, f32)> = gate_pos
                                    .api_calls
                                    .iter()
                                    .filter_map(|h| self.api_idf_weights.get(h).map(|idf| (*h, *idf)))
                                    .collect();
                                scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                                scored.into_iter().take(3).map(|(h, _)| h).collect()
                            };
                            let hit_count = top_calls.iter()
                                .filter(|&&h| weighted_fp.api_calls.contains(&h))
                                .count();
                            if hit_count == 0 {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }

            let pat_weights = crate::pattern::weight_learner::category_weights(
                &pattern.id,
                &self.category_weights,
            );
            let (best_score, evidence) = PatternScorer::score_against_corpus_with_evidence(
                &weighted_fp,
                &pattern.positives,
                &pattern.negatives,
                pattern.expected_context.as_ref(),
                actual_context,
                self.ngram_sim_threshold,
                pat_weights,
            );

            // LSH multi-table penalty: if candidate only hit the structural table
            // but NOT the API table, it's likely a structural FP. Reduce confidence.
            let best_score = if !hit_both {
                best_score * 0.85
            } else {
                best_score
            };

            let threshold = self.threshold_for_pattern(&pattern.id);
            if pattern.id == "rust_async_blocking_io" || pattern.id.contains("async") {
                // eprintln!("DEBUG: pattern {}, best_score {}, threshold {}", pattern.id, best_score, threshold);
            }
            if best_score >= threshold {
                matches.push(PatternMatch {
                    pattern_id: pattern.id.clone(),
                    score: best_score,
                    positive_similarity: evidence.api_sim,
                    negative_similarity: evidence.negative_sim,
                    observation: pattern.observation.clone(),
                    impact: pattern.impact.clone(),
                    improvement: pattern.improvement.clone(),
                    matched_evidence: Some(evidence),
                });
            }
        }

        matches.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        matches
    }
}
