// SPDX-License-Identifier: MIT

//! Learn per-category feature weights from corpus positive/negative pairs.
//!
//! Each corpus pattern has positive (buggy) and negative (fixed) function
//! fingerprints.  The 8 scoring dimensions form an 8-d feature vector.
//! Positives are label 1, negatives label 0.
//!
//! We train a small logistic-regression model per category (CMDI, SSRF, …)
//! using gradient descent on binary cross-entropy.  The learned weight vector
//! replaces the hardcoded global constants in `compute_similarity`.

use std::collections::HashMap;

use crate::corpus::loader::CorpusPattern;
use crate::fingerprint::FunctionFingerprint;
use crate::minhash;
use crate::pattern::scorer::type_usage_overlap;

pub type FeatureVec = [f64; 8];

/// Hardcoded fallback weights used when there are fewer than `MIN_TRAINING_PAIRS`
/// examples for a category.
const DEFAULT_WEIGHTS: FeatureVec = [0.12, 0.28, 0.08, 0.04, 0.03, 0.15, 0.15, 0.15];

/// Minimum number of positive + negative pairs required to train a per-category
/// weight vector.  Below this threshold the fallback is returned.
const MIN_TRAINING_PAIRS: usize = 20;

const LEARNING_RATE: f64 = 0.1;
const ITERATIONS: usize = 200;

/// Extract the category string from a pattern id (second underscore-delimited segment).
/// E.g. `"ts_cmdi_exec_direct"` → `"cmdi"`.
fn extract_category(pattern_id: &str) -> &str {
    pattern_id.split('_').nth(1).unwrap_or("default")
}

/// Compute the 8-d feature vector for a `(candidate, target)` fingerprint pair.
fn compute_features(candidate: &FunctionFingerprint, target: &FunctionFingerprint) -> FeatureVec {
    let jaccard = |a: &[u64], b: &[u64]| minhash::jaccard_similarity_sorted(a, b);

    let ngram_sim = if candidate.weighted_ngram_hashes.is_empty()
        || target.weighted_ngram_hashes.is_empty()
    {
        jaccard(&candidate.ngram_hashes, &target.ngram_hashes)
    } else {
        // Weighted jaccard uses the IDF-weighted hashes
        let mut intersection = 0.0f64;
        let mut union_sum = 0.0f64;
        for (h, w) in &candidate.weighted_ngram_hashes {
            union_sum += *w as f64;
            if target.weighted_ngram_hashes.contains_key(h) {
                intersection += *w as f64;
            }
        }
        for w in target.weighted_ngram_hashes.values() {
            union_sum += *w as f64;
        }
        if union_sum == 0.0 { 0.0 } else { intersection / union_sum }
    };

    let semantic_sim = jaccard(&candidate.semantic_markers, &target.semantic_markers);

    let ast_sim = if !candidate.skeleton_hashes.is_empty() && !target.skeleton_hashes.is_empty()
    {
        1.0 - crate::ast_distance::tree_edit_distance(
            &candidate.skeleton_hashes,
            &target.skeleton_hashes,
        )
    } else {
        jaccard(&candidate.structural_markers, &target.structural_markers)
    };

    let sig_sim = jaccard(&candidate.signature_ngrams, &target.signature_ngrams);
    let type_sim = jaccard(&candidate.param_type_ngrams, &target.param_type_ngrams);
    let type_usage_sim = type_usage_overlap(candidate, target);
    let cf_sim = jaccard(&candidate.control_flow_hashes, &target.control_flow_hashes);
    let api_sim = jaccard(&candidate.api_calls, &target.api_calls);

    [ngram_sim, ast_sim, sig_sim, type_sim, type_usage_sim, semantic_sim, cf_sim, api_sim]
}

/// Logistic regression prediction: σ(w · x)
fn predict(features: &FeatureVec, weights: &FeatureVec) -> f64 {
    let dot: f64 = features.iter().zip(weights.iter()).map(|(x, w)| x * w).sum();
    1.0 / (1.0 + (-dot).exp())
}

/// Train weights for a single category using gradient descent.
fn train_weights(
    positives: &[FeatureVec],
    negatives: &[FeatureVec],
) -> FeatureVec {
    let mut w = [0.5f64; 8];
    let mut all = Vec::new();
    for f in positives { all.push((f, 1.0)); }
    for f in negatives { all.push((f, 0.0)); }

    for _ in 0..ITERATIONS {
        let mut grad = [0.0f64; 8];
        for (features, label) in &all {
            let pred = predict(features, &w);
            let error = pred - label;
            for i in 0..8 {
                grad[i] += error * features[i];
            }
        }
        for i in 0..8 {
            w[i] -= LEARNING_RATE * grad[i] / all.len() as f64;
            w[i] = w[i].clamp(0.0, 1.0);
        }
    }

    // L1-normalize so weights sum to 1 (matching the hardcoded convention)
    let sum: f64 = w.iter().sum();
    if sum > 0.0 {
        for wi in &mut w { *wi /= sum; }
    }
    w
}

/// Learn per-category weight vectors from all corpus patterns.
///
/// For each category, collect every positive-as-positive pair (label 1)
/// and every negative-as-positive pair (label 0 — candidate matches the
/// positive fingerprint but is from a negative file, so it *shouldn't* match).
/// Train logistic regression, store the resulting weights.
///
/// Categories with fewer than `MIN_TRAINING_PAIRS` pairs get the global defaults.
pub fn learn_category_weights(patterns: &[CorpusPattern]) -> HashMap<String, FeatureVec> {
    let mut by_category: HashMap<&str, (Vec<FeatureVec>, Vec<FeatureVec>)> = HashMap::new();

    for pattern in patterns {
        let cat = extract_category(&pattern.id);
        let pos_fps = &pattern.positives;
        let neg_fps = &pattern.negatives;

        if pos_fps.is_empty() { continue; }

        // Every positive-vs-positive pair is a training example with label 1
        for i in 0..pos_fps.len() {
            for j in i + 1..pos_fps.len() {
                let feats = compute_features(&pos_fps[i], &pos_fps[j]);
                by_category.entry(cat).or_default().0.push(feats);
            }
        }

        // Every positive-vs-negative pair is a training example with label 0
        for pos in pos_fps {
            for neg in neg_fps {
                let feats = compute_features(pos, neg);
                by_category.entry(cat).or_default().1.push(feats);
            }
        }
    }

    let mut result = HashMap::new();
    for (cat, (pos, neg)) in &by_category {
        let total = pos.len() + neg.len();
        if total >= MIN_TRAINING_PAIRS {
            let weights = train_weights(pos, neg);
            result.insert(cat.to_string(), weights);
        }
    }
    result
}

/// Look up learned weights for a category, falling back to defaults.
pub fn category_weights<'a>(
    pattern_id: &str,
    learned: &'a HashMap<String, FeatureVec>,
) -> &'a FeatureVec {
    let cat = extract_category(pattern_id);
    learned.get(cat).unwrap_or(&DEFAULT_WEIGHTS)
}
