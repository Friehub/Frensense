// SPDX-License-Identifier: MIT

use rustc_hash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

const DEFAULT_NUM_HASHES: usize = 128;
const DEFAULT_BANDS: usize = 32;
const DEFAULT_ROWS_PER_BAND: usize = 4;

fn sha1_hash(value: u64, seed: u64) -> u64 {
    let mut hasher = FxHasher::default();
    value.hash(&mut hasher);
    seed.hash(&mut hasher);
    hasher.finish()
}

pub fn minhash_signature(hashes: &[u64], num_hashes: usize) -> Vec<u64> {
    if hashes.is_empty() {
        return vec![0u64; num_hashes];
    }

    let mut signature = Vec::with_capacity(num_hashes);

    for i in 0..num_hashes {
        let seed = i as u64;
        let min_val = hashes
            .iter()
            .map(|&h| sha1_hash(h, seed))
            .min()
            .unwrap_or(0);
        signature.push(min_val);
    }

    signature
}

pub fn jaccard_similarity_sorted(a: &[u64], b: &[u64]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let mut i = 0;
    let mut j = 0;
    let mut intersection = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            intersection += 1;
            i += 1;
            j += 1;
        }
    }

    let union = a.len() + b.len() - intersection;
    intersection as f64 / union as f64
}

pub fn overlap_coefficient_sorted(a: &[u64], b: &[u64]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let mut i = 0;
    let mut j = 0;
    let mut intersection = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            intersection += 1;
            i += 1;
            j += 1;
        }
    }

    let min_len = std::cmp::min(a.len(), b.len());
    intersection as f64 / min_len as f64
}

#[allow(clippy::implicit_hasher)]
pub fn jaccard_similarity(a: &FxHashSet<u64>, b: &FxHashSet<u64>) -> f64 {
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 {
        return 1.0;
    }
    intersection as f64 / union as f64
}

pub fn signature_similarity(a: &[u64], b: &[u64]) -> f64 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let matches = a.iter().zip(b.iter()).filter(|(x, y)| x == y).count();
    matches as f64 / a.len() as f64
}

pub struct LSHIndex {
    bands: Vec<FxHashSet<u64>>,
    num_bands: usize,
    rows_per_band: usize,
}

impl Default for LSHIndex {
    fn default() -> Self {
        Self {
            bands: vec![FxHashSet::default(); DEFAULT_BANDS],
            num_bands: DEFAULT_BANDS,
            rows_per_band: DEFAULT_ROWS_PER_BAND,
        }
    }
}

impl LSHIndex {
    pub fn new(num_bands: usize, rows_per_band: usize) -> Self {
        Self {
            bands: vec![FxHashSet::default(); num_bands],
            num_bands,
            rows_per_band,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn insert(&mut self, signature: &[u64], item_id: u64) {
        for band in 0..self.num_bands {
            let start = band * self.rows_per_band;
            if start >= signature.len() {
                break;
            }
            let end = (start + self.rows_per_band).min(signature.len());
            let mut hasher = FxHasher::default();
            for &val in &signature[start..end] {
                val.hash(&mut hasher);
            }
            let bucket = hasher.finish() % self.num_bands as u64;
            self.bands[bucket as usize].insert(item_id);
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn query(&self, signature: &[u64]) -> Vec<u64> {
        let mut candidates = FxHashSet::default();
        for band in 0..self.num_bands {
            let start = band * self.rows_per_band;
            if start >= signature.len() {
                break;
            }
            let end = (start + self.rows_per_band).min(signature.len());
            let mut hasher = FxHasher::default();
            for &val in &signature[start..end] {
                val.hash(&mut hasher);
            }
            let bucket = hasher.finish() % self.num_bands as u64;
            for fp in &self.bands[bucket as usize] {
                candidates.insert(*fp);
            }
        }
        candidates.into_iter().collect()
    }

    pub fn buckets(&self) -> &[FxHashSet<u64>] {
        &self.bands
    }
}

pub fn similarity_score(hashes_a: &[u64], hashes_b: &[u64]) -> f64 {
    let sig_a = minhash_signature(hashes_a, DEFAULT_NUM_HASHES);
    let sig_b = minhash_signature(hashes_b, DEFAULT_NUM_HASHES);
    signature_similarity(&sig_a, &sig_b)
}

pub fn approximate_jaccard(hashes_a: &[u64], hashes_b: &[u64]) -> f64 {
    similarity_score(hashes_a, hashes_b)
}

pub fn hash_ngrams(tokens: &[String], window_size: usize) -> Vec<u64> {
    if tokens.len() < window_size {
        return Vec::new();
    }
    let mut hashes = FxHashSet::default();
    for i in 0..=(tokens.len().saturating_sub(window_size)) {
        let mut state = FxHasher::default();
        for token in &tokens[i..i + window_size] {
            token.hash(&mut state);
        }
        hashes.insert(state.finish());
    }
    let mut vec: Vec<u64> = hashes.into_iter().collect();
    vec.sort_unstable();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minhash_identical_sets() {
        let set: Vec<u64> = vec![42, 99];
        let sig = minhash_signature(&set, DEFAULT_NUM_HASHES);
        assert_eq!(sig.len(), DEFAULT_NUM_HASHES);
    }

    #[test]
    fn test_jaccard_identical() {
        let mut a = FxHashSet::default();
        a.insert(1);
        a.insert(2);
        let b = a.clone();
        assert!((jaccard_similarity(&a, &b) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_disjoint() {
        let mut a = FxHashSet::default();
        a.insert(1);
        let mut b = FxHashSet::default();
        b.insert(2);
        assert!((jaccard_similarity(&a, &b) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_signature_similarity() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 5, 6];
        let sim = signature_similarity(&a, &b);
        assert!((sim - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_lsh_index() {
        let mut index = LSHIndex::new(4, 2);
        let sig = minhash_signature(&[], 8);
        index.insert(&sig, 42);
        let candidates = index.query(&sig);
        assert!(candidates.contains(&42));
    }

    #[test]
    fn test_approximate_jaccard_empty() {
        let sim = approximate_jaccard(&[], &[]);
        assert!((sim - 1.0).abs() < 1e-10);
    }
}
