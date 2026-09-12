// SPDX-License-Identifier: MIT

//! Learn per-category feature weights from corpus positive/negative pairs.
//!
//! Each corpus pattern has positive (buggy) and negative (fixed) function
//! fingerprints.  The 15 scoring dimensions form a 15-d feature vector.
//! Positives are label 1, negatives label 0.
//!
//! We train a small logistic-regression model per category (CMDI, SSRF, …)
//! using gradient descent on binary cross-entropy.  The learned weight vector
//! replaces the hardcoded global constants in `compute_similarity`.
//!
//! # Design notes
//!
//! ## Feature centering (Bug 1 fix)
//! All features are Jaccard/containment similarities in [0, 1].  A raw dot
//! product w·x therefore also lives in [0, 1], and sigmoid([0,1]) only spans
//! [0.50, 0.73] - too narrow for meaningful gradients.  We centre each feature
//! around 0 and scale by 4 before the sigmoid:
//!
//!     logit = Σ wᵢ · (xᵢ - 0.5) · FEATURE_SCALE
//!
//! With FEATURE_SCALE = 4, x=0 → logit component -2, x=1 → +2, giving sigmoid
//! outputs in [0.12, 0.88] - enough range for the gradient to drive label=1
//! toward 1.0 and label=0 toward 0.0.
//!
//! ## L2 regularization anchored to DEFAULT_WEIGHTS (Bug 2 fix)
//! Rather than regularizing toward 0 (standard ridge), we regularize toward
//! DEFAULT_WEIGHTS.  This means the learned result is a data-weighted blend
//! of the hand-tuned prior and the corpus signal.  On small corpora the prior
//! dominates; as data grows the corpus signal takes over.
//!
//! ## Leave-one-out positive pairs (Bug 3 fix)
//! At inference, score_against_corpus takes max(candidate, positives). Training
//! label=1 pairs now mirror this: for each positive pᵢ, use its best similarity
//! to the remaining positives as the label=1 feature vector.

use std::collections::HashMap;

use frensense_engine::corpus::pattern::CorpusPattern;
use frensense_engine::fingerprint::FunctionFingerprint;

pub type FeatureVec = [f64; 15];

// Index mapping: 0=ngram, 1=ast, 2=signature, 3=param_type, 4=type_usage,
//                5=semantic, 6=cf, 7=api, 8=tainted_api, 9=motif, 10=flow,
//                11=config, 12=cf_order, 13=arg_type, 14=literal_concat
//
// flow_sim (0.16) makes data-flow path similarity the primary generalization
// signal - API-invariant (exec vs spawn vs Command::new all produce the
// same UserInputSource→CommandExecutionSink path). Eliminates need for
// M1-M15 mutation variants.
// See docs/SCORING_DIMENSIONS.md for analysis.
pub(crate) const DEFAULT_WEIGHTS: FeatureVec = [
    0.08, 0.10, 0.08, 0.04, 0.03, 0.10, 0.10, 0.10, 0.14, 0.12, 0.16, 0.03, 0.02, 0.04, 0.04,
];

/// Minimum number of total feature-vector pairs required to run gradient
/// descent for a given category or pattern.  A 15-parameter model trained on
/// fewer than 20 points is severely underdetermined and will overfit.
/// Bug 4 fix: raised from 5 to 20.
const MIN_TRAINING_PAIRS: usize = 20;

const LEARNING_RATE: f64 = 0.05;
const ITERATIONS: usize = 400;

/// Scale applied to centred features before the sigmoid.
/// Chosen so that a feature of 1.0 → logit contribution +2, 0.0 → -2.
const FEATURE_SCALE: f64 = 4.0;

/// L2 regularization strength.  Pulls each weight toward DEFAULT_WEIGHTS.
/// At 0.01 the prior dominates for small corpora; gradient signal takes over
/// as more training pairs accumulate.
const L2_LAMBDA: f64 = 0.01;

fn extract_category(pattern_id: &str) -> &str {
    pattern_id.split('_').nth(1).unwrap_or("default")
}

fn compute_features(candidate: &FunctionFingerprint, target: &FunctionFingerprint) -> FeatureVec {
    // Bug 6 fix: implicit return, no explicit `return` keyword on last expression.
    frensense_engine::pattern::similarity::compute_dimensions(candidate, target).as_array()
}

/// Predict P(label=1) for a feature vector given weight vector `w`.
///
/// Features are centred at 0.5 and scaled before the dot product so that the
/// sigmoid output spans [0.12, 0.88] even when all features live in [0, 1].
///
/// Bug 1 fix: previously used raw `sigmoid(w·x)` where both `w` and `x` are
/// in [0,1], producing outputs only in [0.50, 0.73]. The gradient
/// `(pred - label)` was therefore always ~0.5 in magnitude for both classes,
/// making it impossible for the model to discriminate positives from negatives.
fn predict(features: &FeatureVec, weights: &FeatureVec) -> f64 {
    let dot: f64 = features
        .iter()
        .zip(weights.iter())
        .map(|(&x, &w)| w * (x - 0.5) * FEATURE_SCALE)
        .sum();
    1.0 / (1.0 + (-dot).exp())
}

/// Train a 15-d weight vector from positive and negative feature vectors using
/// gradient descent on binary cross-entropy with L2 regularization.
fn train_weights(positives: &[FeatureVec], negatives: &[FeatureVec]) -> FeatureVec {
    // Initialise from the hand-tuned prior so that poor gradient signal
    // degrades gracefully back to DEFAULT_WEIGHTS rather than collapsing to
    // near-uniform weights after L1 normalization.
    let mut w = DEFAULT_WEIGHTS;

    let n_pos = positives.len();
    let n_neg = negatives.len();
    if n_pos == 0 && n_neg == 0 {
        return DEFAULT_WEIGHTS;
    }

    // Balanced class weights: each positive counts for 1/(2·n_pos),
    // each negative for 1/(2·n_neg). Prevents majority class from dominating
    // the gradient when positive and negative counts differ.
    let pos_weight = if n_pos > 0 { 0.5 / n_pos as f64 } else { 0.0 };
    let neg_weight = if n_neg > 0 { 0.5 / n_neg as f64 } else { 0.0 };

    for _ in 0..ITERATIONS {
        let mut grad = [0.0f64; 15];

        // Positive examples: label = 1
        for features in positives {
            let pred = predict(features, &w);
            let error = pred - 1.0;
            for i in 0..15 {
                // Gradient of cross-entropy w.r.t. wᵢ, accounting for feature scaling.
                grad[i] += pos_weight * error * (features[i] - 0.5) * FEATURE_SCALE;
            }
        }

        // Negative examples: label = 0
        for features in negatives {
            let pred = predict(features, &w);
            let error = pred; // pred - 0.0
            for i in 0..15 {
                grad[i] += neg_weight * error * (features[i] - 0.5) * FEATURE_SCALE;
            }
        }

        // Bug 2 fix: L2 regularization anchored to DEFAULT_WEIGHTS.
        // Pulls each weight toward the hand-tuned prior instead of toward 0.
        // This ensures the learned result is a blend of data signal and prior,
        // and prevents any single noisy example from dominating the final weights.
        for i in 0..15 {
            grad[i] += L2_LAMBDA * (w[i] - DEFAULT_WEIGHTS[i]);
        }

        for i in 0..15 {
            w[i] -= LEARNING_RATE * grad[i];
            // Weights represent importance, not direction: keep non-negative.
            w[i] = w[i].max(0.0);
        }
    }

    // L1-normalize so weights sum to 1, matching the convention in weighted_score().
    let sum: f64 = w.iter().sum();
    if sum > 1e-9 {
        for wi in &mut w {
            *wi /= sum;
        }
    } else {
        // Degenerate: all weights collapsed to 0. Fall back to prior.
        return DEFAULT_WEIGHTS;
    }
    w
}

pub fn learn_category_weights(patterns: &[CorpusPattern]) -> HashMap<String, FeatureVec> {
    // Bug 5 fix: removed the dead first `result` block + early return.
    // Previously two `let mut result = HashMap::new()` declarations existed,
    // the first was dead computation that only existed to support the bailout.

    let mut by_category: HashMap<String, (Vec<FeatureVec>, Vec<FeatureVec>)> = HashMap::new();
    let mut global_pos: Vec<FeatureVec> = Vec::new();
    let mut global_neg: Vec<FeatureVec> = Vec::new();

    for pattern in patterns {
        let cat = extract_category(&pattern.id).to_string();
        let pos_fps = &pattern.positives;
        let neg_fps = &pattern.negatives;

        if pos_fps.is_empty() {
            continue;
        }

        // Bug 3 fix: leave-one-out best-match for label=1 examples.
        //
        // Previously: every (posᵢ, posⱼ) pair was used as a label=1 example,
        // measuring intra-positive similarity. With 2 different shapes of the
        // same bug, these pairs have lower similarity than what the scorer sees
        // at inference time when it picks max(candidate, positives).
        //
        // Fix: for each positive pᵢ, find the highest-scoring similarity
        // against all other positives and use that feature vector as the
        // label=1 example. This mirrors the max-over-positives logic in
        // score_against_corpus exactly, making training pairs consistent with
        // inference.
        for i in 0..pos_fps.len() {
            let mut best_feats: Option<FeatureVec> = None;
            let mut best_score = f64::NEG_INFINITY;
            for j in 0..pos_fps.len() {
                if i == j {
                    continue;
                }
                let feats = compute_features(&pos_fps[i], &pos_fps[j]);
                // Use sum of all dimensions as a proxy for overall similarity.
                let score: f64 = feats.iter().sum();
                if score > best_score {
                    best_score = score;
                    best_feats = Some(feats);
                }
            }
            if let Some(feats) = best_feats {
                by_category.entry(cat.clone()).or_default().0.push(feats);
                by_category
                    .entry(pattern.id.clone())
                    .or_default()
                    .0
                    .push(feats);
                global_pos.push(feats);
            }
        }

        // Label=0 examples: positive vs. negative pairs (unchanged logic).
        for pos in pos_fps {
            for neg in neg_fps {
                let feats = compute_features(pos, neg);
                by_category.entry(cat.clone()).or_default().1.push(feats);
                by_category
                    .entry(pattern.id.clone())
                    .or_default()
                    .1
                    .push(feats);
                global_neg.push(feats);
            }
        }
    }

    // Train global weights on ALL data. Used as fallback for low-data categories.
    let global_weights = if !global_pos.is_empty() || !global_neg.is_empty() {
        train_weights(&global_pos, &global_neg)
    } else {
        DEFAULT_WEIGHTS
    };

    let mut result = HashMap::new();
    result.insert("_global".to_string(), global_weights);

    for (cat, (pos, neg)) in &by_category {
        let total = pos.len() + neg.len();
        if total >= MIN_TRAINING_PAIRS {
            let weights = train_weights(pos, neg);
            result.insert(cat.clone(), weights);
        } else {
            // Not enough data for this category/pattern: fall back to global
            // learned weights (which have more data) rather than DEFAULT_WEIGHTS.
            result.insert(cat.clone(), global_weights);
        }
    }

    result
}
