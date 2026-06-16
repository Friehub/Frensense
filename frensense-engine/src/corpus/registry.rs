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
}

impl PatternRegistry {
    pub fn new(threshold: f64) -> Self {
        Self {
            patterns: Vec::new(),
            lsh_index: None,
            threshold,
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

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    fn build_lsh_index(&mut self) {
        if self.patterns.len() < 10 {
            return;
        }
        let num_hashes = 128;
        let num_bands = 16;
        let rows_per_band = num_hashes / num_bands;
        let mut index = LSHIndex::new(num_bands, rows_per_band);
        for (i, pattern) in self.patterns.iter().enumerate() {
            let sig = minhash_signature(&pattern.positive.ngram_hashes, num_hashes);
            index.insert(&sig, i as u64);
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
            let score =
                PatternScorer::score_against_corpus(fp, &pattern.positive, &pattern.negative);
            if score >= self.threshold {
                matches.push(PatternMatch {
                    pattern_id: pattern.id.clone(),
                    score,
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
