// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::corpus::loader::{CorpusPattern, load_corpus};
use crate::corpus::source_sink::{CorpusSourceSinkRegistry, build_registry_from_dir};
use crate::fingerprint::{FunctionFingerprint, apply_idf_weights, compute_idf_weights};
use crate::minhash::{LSHIndex, minhash_signature};
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
}

#[derive(Default)]
pub struct PatternRegistry {
    patterns: Vec<CorpusPattern>,
    lsh_index: Option<LSHIndex>,
    threshold: f64,
    ngram_sim_threshold: f64,
    threshold_overrides: std::collections::HashMap<String, f64>,
    idf_weights: FxHashMap<u64, f32>,
    source_sink: CorpusSourceSinkRegistry,
}

impl PatternRegistry {
    pub fn new(threshold: f64, ngram_sim_threshold: f64) -> Self {
        Self {
            patterns: Vec::new(),
            lsh_index: None,
            threshold,
            ngram_sim_threshold,
            threshold_overrides: std::collections::HashMap::new(),
            idf_weights: FxHashMap::default(),
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
        let bundle_patterns = crate::corpus::bundle::load_bundle(bytes)?;
        let count = bundle_patterns.len();

        self.patterns = bundle_patterns
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
        self.compute_and_apply_idf();
        self.build_lsh_index();
        Ok(count)
    }

    /// Compute IDF weights from corpus fingerprints and apply to all patterns.
    fn compute_and_apply_idf(&mut self) {
        let all_positives: Vec<FunctionFingerprint> = self
            .patterns
            .iter()
            .flat_map(|p| p.positives.iter().cloned())
            .collect();

        if all_positives.is_empty() {
            return;
        }

        self.idf_weights = compute_idf_weights(&all_positives);

        // Apply IDF weights to all corpus fingerprints
        for pattern in &mut self.patterns {
            for fp in &mut pattern.positives {
                apply_idf_weights(fp, &self.idf_weights);
            }
            for fp in &mut pattern.negatives {
                apply_idf_weights(fp, &self.idf_weights);
            }
        }
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
        let num_hashes = 128;
        // Maximize recall for sub-graph containment by setting bands=128, rows=1
        let num_bands = 128;
        let rows_per_band = num_hashes / num_bands;
        let mut index = LSHIndex::new(num_bands, rows_per_band);
        for (i, pattern) in self.patterns.iter().enumerate() {
            if let Some(first_pos) = pattern.positives.first() {
                // Use structural_markers (AST types) for robust Locality Sensitive Hashing instead of exact lexemes
                let sig = minhash_signature(&first_pos.structural_markers, num_hashes);
                index.insert(&sig, i as u64);
            }
        }
        self.lsh_index = Some(index);
    }

    pub fn scan_function(
        &self,
        fp: &FunctionFingerprint,
        func_node: Option<tree_sitter::Node<'_>>,
        source: Option<&str>,
        actual_context: Option<&crate::context::FileContext>,
    ) -> Vec<PatternMatch> {
        let candidates: Vec<usize> = if let Some(ref lsh) = self.lsh_index {
            let sig = minhash_signature(&fp.structural_markers, 128);
            lsh.query(&sig)
                .iter()
                .map(|&id| id as usize)
                .filter(|&id| id < self.patterns.len())
                .collect()
        } else {
            (0..self.patterns.len()).collect()
        };

        // Apply IDF weights to candidate fingerprint for scoring
        let mut weighted_fp = fp.clone();
        if !self.idf_weights.is_empty() {
            apply_idf_weights(&mut weighted_fp, &self.idf_weights);
        }

        let mut extracted_flows: Option<std::collections::HashSet<(String, String)>> = None;

        // eprintln!("DEBUG: fp.name = {:?}, api = {}, candidates = {}", fp.function_name, fp.api_calls.len(), candidates.len());

        let mut matches = Vec::new();
        for &idx in &candidates {
            let pattern = &self.patterns[idx];
            // eprintln!("DEBUG: checking pattern {}", pattern.id);
            if pattern.id.contains("blocking_io") {
                // eprintln!("DEBUG: found pattern {} is candidate.", pattern.id);
            }

            // Apply semantic filter if present
            if let (Some(filter), Some(node), Some(src)) =
                (&pattern.semantic_filter, func_node, source)
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

            // Fast early exit: if the candidate shares almost no structural markers with the primary positive example,
            // it's highly unlikely to be a match. We can skip the expensive full scoring.
            if let Some(first_pos) = pattern.positives.first() {
                let struct_sim = crate::minhash::overlap_coefficient_sorted(
                    &weighted_fp.structural_markers,
                    &first_pos.structural_markers,
                );

                if struct_sim < self.ngram_sim_threshold {
                    continue; // Prune this candidate early
                }

                // API-call gate: if both candidate and positive have API calls but share NONE,
                // skip — the structural similarity is coincidental.
                // Only gates when BOTH sides have non-empty API calls.
                // Functions with empty api_calls (e.g., anonymous arrow functions)
                // bypass this gate and fall through to structural scoring.
                if !first_pos.api_calls.is_empty() && !weighted_fp.api_calls.is_empty() {
                    let api_overlap = first_pos
                        .api_calls
                        .iter()
                        .any(|h| weighted_fp.api_calls.contains(h));
                    if !api_overlap {
                        continue;
                    }
                }
            }

            let best_score = PatternScorer::score_against_corpus(
                &weighted_fp,
                &pattern.positives,
                &pattern.negatives,
                pattern.expected_context.as_ref(),
                actual_context,
                self.ngram_sim_threshold,
            );
            let threshold = self.threshold_for_pattern(&pattern.id);
            if pattern.id == "rust_async_blocking_io" || pattern.id.contains("async") {
                // eprintln!("DEBUG: pattern {}, best_score {}, threshold {}", pattern.id, best_score, threshold);
            }
            if best_score >= threshold {
                matches.push(PatternMatch {
                    pattern_id: pattern.id.clone(),
                    score: best_score,
                    positive_similarity: 0.0,
                    negative_similarity: 0.0,
                    observation: pattern.observation.clone(),
                    impact: pattern.impact.clone(),
                    improvement: pattern.improvement.clone(),
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
