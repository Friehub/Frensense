// SPDX-License-Identifier: MIT

//! Per-pattern confidence calibration via logistic regression.
//!
//! Each pattern gets its own sigmoid:  P(tp | score) = 1 / (1 + exp(-(A·score + B)))
//! Trained at bundle build time by holding out 20% of positive/negative pairs,
//! scoring them against the pattern, and fitting (A, B) via gradient descent.
//!
//! Falls back to per-category Platt scaling when a pattern has fewer than
//! MIN_EXAMPLES training pairs.

use std::collections::HashMap;

use crate::corpus::loader::CorpusPattern;
use crate::fingerprint::FunctionFingerprint;
use crate::minhash;
use crate::pattern::scorer::type_usage_overlap;

/// Minimum number of scored examples required to fit a per-pattern sigmoid.
const MIN_EXAMPLES: usize = 10;

/// Sigmoid parameters:  P(tp | score) = 1 / (1 + exp(-(A * score + B)))
pub type CalibrationParams = (f32, f32);

/// Compute the 8-d feature vector for calibration scoring.
fn compute_calibration_features(
    candidate: &FunctionFingerprint,
    target: &FunctionFingerprint,
) -> f64 {
    let jaccard = |a: &[u64], b: &[u64]| minhash::jaccard_similarity_sorted(a, b);

    let ngram_sim =
        if candidate.weighted_ngram_hashes.is_empty() || target.weighted_ngram_hashes.is_empty() {
            jaccard(&candidate.ngram_hashes, &target.ngram_hashes)
        } else {
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
            if union_sum == 0.0 {
                0.0
            } else {
                intersection / union_sum
            }
        };
    let semantic_sim = jaccard(&candidate.semantic_markers, &target.semantic_markers);
    let ast_sim = if !candidate.skeleton_hashes.is_empty() && !target.skeleton_hashes.is_empty() {
        1.0 - crate::ast_distance::tree_edit_distance(
            &candidate.skeleton_hashes,
            &target.skeleton_hashes,
        )
    } else {
        jaccard(&candidate.structural_markers, &target.structural_markers)
    };
    let cf_sim = jaccard(&candidate.control_flow_hashes, &target.control_flow_hashes);
    // API sim: max of full-name and segment Jaccard (mirrors scorer)
    let api_sim_full = jaccard(&candidate.api_calls, &target.api_calls);
    let api_sim_seg =
        if !candidate.api_call_segments.is_empty() && !target.api_call_segments.is_empty() {
            jaccard(&candidate.api_call_segments, &target.api_call_segments)
        } else {
            0.0
        };
    let api_sim = api_sim_full.max(api_sim_seg);
    let tainted_api_sim = jaccard(&candidate.tainted_api_calls, &target.tainted_api_calls);

    let arg_type_sim =
        if !candidate.argument_call_types.is_empty() && !target.argument_call_types.is_empty() {
            jaccard(&candidate.argument_call_types, &target.argument_call_types)
        } else {
            0.0
        };
    let literal_concat_sim = if !candidate.literal_pattern_hashes.is_empty()
        && !target.literal_pattern_hashes.is_empty()
    {
        jaccard(
            &candidate.literal_pattern_hashes,
            &target.literal_pattern_hashes,
        )
    } else {
        0.0
    };

    // Use hardcoded fallback weights for calibration (avoids circular dependency)
    ngram_sim * 0.12
        + ast_sim * 0.20
        + jaccard(&candidate.signature_ngrams, &target.signature_ngrams) * 0.08
        + jaccard(&candidate.param_type_ngrams, &target.param_type_ngrams) * 0.04
        + type_usage_overlap(candidate, target) * 0.03
        + semantic_sim * 0.12
        + cf_sim * 0.12
        + api_sim * 0.12
        + tainted_api_sim * 0.17
        + arg_type_sim * 0.04
        + literal_concat_sim * 0.04
}

/// Train per-pattern calibration parameters from corpus patterns.
///
/// For each pattern with enough examples:
/// 1. Split positives into train (80%) and validate (20%)
/// 2. Score validate set against train positives
/// 3. Score negatives against train positives
/// 4. Fit sigmoid (A, B) via gradient descent on binary cross-entropy
///
/// Returns a map of pattern_id → (A, B).
pub fn train_per_pattern_calibration(
    patterns: &[CorpusPattern],
) -> HashMap<String, CalibrationParams> {
    let mut result = HashMap::new();

    for pattern in patterns {
        let pos = &pattern.positives;
        let neg = &pattern.negatives;

        // Need enough examples for meaningful split
        if pos.len() < MIN_EXAMPLES || neg.len() < MIN_EXAMPLES {
            continue;
        }

        // Simple 80/20 split: use first 80% as train, last 20% as validate
        let split = (pos.len() as f64 * 0.8).ceil() as usize;
        if split == 0 || split >= pos.len() {
            continue;
        }

        let train_pos: Vec<&FunctionFingerprint> = pos.iter().take(split).collect();
        let val_pos: Vec<&FunctionFingerprint> = pos.iter().skip(split).collect();
        let train_pos_owned: Vec<FunctionFingerprint> =
            train_pos.iter().map(|f| (*f).clone()).collect();
        let val_fps: Vec<&FunctionFingerprint> =
            val_pos.iter().copied().chain(neg.iter()).collect();
        let labels: Vec<f64> = val_pos
            .iter()
            .map(|_| 1.0)
            .chain(neg.iter().map(|_| 0.0))
            .collect();

        // Score each validation fingerprint against the training positives
        let mut scores: Vec<(f64, f64)> = Vec::new();
        for (i, fp) in val_fps.iter().enumerate() {
            // Compute raw feature score
            let mut best = 0.0f64;
            for train in &train_pos_owned {
                let sim = compute_calibration_features(fp, train);
                if sim > best {
                    best = sim;
                }
            }
            scores.push((best, labels[i]));
        }

        if scores.len() < MIN_EXAMPLES {
            continue;
        }

        // Fit sigmoid: P(tp) = 1 / (1 + exp(-(A * score + B)))
        // Gradient descent on binary cross-entropy
        let mut a = 1.0f64;
        let mut b = 0.0f64;
        let lr = 0.1;
        let iterations = 500;

        for _ in 0..iterations {
            let mut grad_a = 0.0f64;
            let mut grad_b = 0.0f64;
            for (score, label) in &scores {
                let z = a * score + b;
                let p = 1.0 / (1.0 + (-z).exp());
                let error = p - label;
                grad_a += error * score;
                grad_b += error;
            }
            let n = scores.len() as f64;
            a -= lr * grad_a / n;
            b -= lr * grad_b / n;
        }

        result.insert(pattern.id.clone(), (a as f32, b as f32));
    }

    result
}

/// Apply per-pattern calibration to a raw score.
///
/// If the pattern has learned parameters, apply them.
/// Otherwise return the raw score unchanged (caller should fall back
/// to per-category Platt scaling).
pub fn calibrate(raw_score: f64, params: Option<&(f32, f32)>) -> f64 {
    let (a, b) = match params {
        Some(&(a, b)) => (a as f64, b as f64),
        None => (8.0, -3.2), // Fallback Platt scaling: centers probability 0.5 at raw_score 0.4
    };
    let z = a * raw_score + b;
    // Clamp to avoid overflow
    let z = z.clamp(-20.0, 20.0);
    1.0 / (1.0 + (-z).exp())
}
