// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::corpus::loader::{CorpusPattern, load_corpus};
use crate::fingerprint::FunctionFingerprint;
use crate::minhash::{LSHIndex, minhash_signature};
use crate::pattern::scorer::PatternScorer;

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub score: f64,
    pub positive_similarity: f64,
    pub negative_similarity: f64,
}

#[derive(Default)]
pub struct PatternRegistry {
    patterns: Vec<CorpusPattern>,
    lsh_index: Option<LSHIndex>,
    threshold: f64,
    threshold_overrides: std::collections::HashMap<String, f64>,
}

impl PatternRegistry {
    pub fn new(threshold: f64) -> Self {
        Self {
            patterns: Vec::new(),
            lsh_index: None,
            threshold,
            threshold_overrides: std::collections::HashMap::new(),
        }
    }

    pub fn load_corpus(&mut self, corpus_dir: &Path) -> Result<usize, String> {
        let patterns = load_corpus(corpus_dir)?;
        let count = patterns.len();
        self.patterns = patterns;
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
        let count = all_patterns.len();
        self.patterns = all_patterns;
        self.build_lsh_index();
        Ok(count)
    }

    #[cfg(feature = "serialize")]
    pub fn load_from_bundle(&mut self, bytes: &[u8]) -> Result<usize, String> {
        let bundle_patterns = crate::corpus::bundle::load_bundle(bytes)?;
        let count = bundle_patterns.len();
        self.patterns = bundle_patterns
            .into_iter()
            .map(|bp| CorpusPattern {
                id: bp.id,
                positives: bp.positives,
                negatives: bp.negatives,
            })
            .collect();
        self.build_lsh_index();
        Ok(count)
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
        // Scale bands with pattern count: more bands = better recall at high pattern counts
        let num_bands = if self.patterns.len() > 1000 { 32 } else { 16 };
        let rows_per_band = num_hashes / num_bands;
        let mut index = LSHIndex::new(num_bands, rows_per_band);
        for (i, pattern) in self.patterns.iter().enumerate() {
            if let Some(first_pos) = pattern.positives.first() {
                let sig = minhash_signature(&first_pos.ngram_hashes, num_hashes);
                index.insert(&sig, i as u64);
            }
        }
        self.lsh_index = Some(index);
    }

    pub fn scan_function(&self, fp: &FunctionFingerprint) -> Vec<PatternMatch> {
        let candidates: Vec<usize> = if let Some(ref lsh) = self.lsh_index {
            let sig = minhash_signature(&fp.ngram_hashes, 128);
            lsh.query(&sig)
                .iter()
                .map(|&id| id as usize)
                .filter(|&id| id < self.patterns.len())
                .collect()
        } else {
            (0..self.patterns.len()).collect()
        };

        let mut matches = Vec::new();
        for &idx in &candidates {
            let pattern = &self.patterns[idx];
            let best_score = pattern
                .positives
                .iter()
                .flat_map(|pos| {
                    pattern
                        .negatives
                        .iter()
                        .map(move |neg| PatternScorer::score_against_corpus(fp, pos, neg))
                })
                .fold(0.0f64, f64::max);
            let threshold = self.threshold_for_pattern(&pattern.id);
            if best_score >= threshold {
                matches.push(PatternMatch {
                    pattern_id: pattern.id.clone(),
                    score: best_score,
                    positive_similarity: 0.0,
                    negative_similarity: 0.0,
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
