// SPDX-License-Identifier: MIT

use std::hash::{Hash, Hasher};

use crate::fingerprint::FunctionFingerprint;
use crate::pattern::evidence::MatchEvidence;
use crate::pattern::similarity::RawDimensions;

// ─────────────────────────────────────────────────────────────────────────────
// Scoring configuration
//
// All thresholds and factors that affect scoring behaviour are collected here
// in a single struct. This makes them configurable via CLI flags or config
// files, and ensures a single source of truth for defaults.
// ─────────────────────────────────────────────────────────────────────────────

/// Configurable scoring parameters for the pattern matcher.
///
/// All fields have sensible defaults matching the original hardcoded constants.
/// Use `ScorerConfig::default()` to get the standard configuration, then
/// override individual fields as needed.
#[derive(Debug, Clone)]
pub struct ScorerConfig {
    /// Cross-lingual transfer penalty (0.0-1.0). Lower = harsher penalty.
    pub cross_lingual_penalty: f32,
    /// Penalty for zero semantic marker overlap (0.0-1.0).
    pub semantic_zero_penalty: f64,
    /// Boost for semantic marker match (1.0+ = boost, <1.0 = penalty).
    pub semantic_match_boost: f64,
    /// Noise gate: moderate dimension threshold.
    pub noise_gate_moderate_signal: f64,
    /// Noise gate: strong dimension threshold.
    pub noise_gate_strong_signal: f64,
    /// Noise gate: minimum moderate dims required.
    pub noise_gate_min_moderate_dims: usize,
    /// Per-factor context mismatch penalty.
    pub context_mismatch_penalty: f64,

    // --- Taint / Verification ---
    /// Confidence multiplier for taint-verified findings. Default: 1.2.
    pub taint_verified_boost: f64,
    /// Confidence multiplier for cross-file taint-verified findings. Default: 1.15.
    pub cross_file_taint_boost: f64,
    /// Maximum confidence after taint boost. Default: 0.95.
    pub taint_boost_cap: f64,
    /// Minimum score for untainted matches to be emitted. Default: 0.20.
    pub score_suppression_floor: f64,
}

impl Default for ScorerConfig {
    fn default() -> Self {
        Self {
            cross_lingual_penalty: 0.20,
            semantic_zero_penalty: 0.55,
            semantic_match_boost: 2.0,
            noise_gate_moderate_signal: 0.15,
            noise_gate_strong_signal: 0.4,
            noise_gate_min_moderate_dims: 3,
            context_mismatch_penalty: 0.5,

            taint_verified_boost: 1.2,
            cross_file_taint_boost: 1.15,
            taint_boost_cap: 0.95,
            score_suppression_floor: 0.25,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PatternScorer;

/// M1: Weighted Jaccard - IDF-weighted intersection / union.
pub fn weighted_jaccard(
    a: &rustc_hash::FxHashMap<u64, f32>,
    b: &rustc_hash::FxHashMap<u64, f32>,
) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let mut intersection = 0.0f64;
    let mut union = 0.0f64;
    let all_keys: rustc_hash::FxHashSet<_> = a.keys().chain(b.keys()).collect();
    for key in all_keys {
        let wa = f64::from(a.get(key).copied().unwrap_or(0.0));
        let wb = f64::from(b.get(key).copied().unwrap_or(0.0));
        intersection += wa.min(wb);
        union += wa.max(wb);
    }
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

/// If pattern language differs from candidate language, apply penalty.
/// Cross-language matching is useful for catching similar bug patterns across languages,
/// but should be heavily penalized to avoid false positives.
fn cross_lingual_penalty(pattern_lang: &str, candidate_lang: &str, config: &ScorerConfig) -> f32 {
    if pattern_lang == candidate_lang || pattern_lang == "unknown" || candidate_lang == "unknown" {
        return 1.0;
    }
    // TypeScript and JavaScript share the same AST structure (tree-sitter-typescript
    // parses JS too). Treat them as equivalent for cross-lingual matching.
    let js_like = |l: &str| l == "typescript" || l == "javascript";
    if js_like(pattern_lang) && js_like(candidate_lang) {
        return 1.0;
    }
    config.cross_lingual_penalty // 80% penalty for genuinely different languages (e.g. Rust ↔ TypeScript)
}

impl PatternScorer {
    fn compute_context_penalty(
        expected_context: Option<&crate::context::FileContext>,
        actual_context: Option<&crate::context::FileContext>,
        config: &ScorerConfig,
    ) -> f64 {
        match (expected_context, actual_context) {
            (Some(exp), Some(act)) => {
                let mut penalty = 1.0;
                if exp.sensitivity == crate::context::DataSensitivity::High
                    && act.sensitivity != crate::context::DataSensitivity::High
                {
                    penalty *= config.context_mismatch_penalty;
                }
                if exp.environment == crate::context::Environment::RouteHandler
                    && (act.environment == crate::context::Environment::Test
                        || act.environment == crate::context::Environment::Utility)
                {
                    penalty *= config.context_mismatch_penalty;
                }
                if act.environment == crate::context::Environment::RouteHandler
                    && exp.environment != crate::context::Environment::RouteHandler
                    && exp.environment != crate::context::Environment::Unknown
                {
                    penalty *= config.context_mismatch_penalty;
                }
                penalty
            }
            _ => 1.0,
        }
    }

    /// Like `score_against_corpus_with_evidence` but accepts a pre-computed
    /// `DimCache` for read-only lookup.  The caller must guarantee the cache
    /// contains `raw_dimensions(candidate, target)` for every target.
    pub fn score_against_corpus_with_evidence_cached(
        candidate: &FunctionFingerprint,
        positives: &[FunctionFingerprint],
        negatives: &[FunctionFingerprint],
        expected_context: Option<&crate::context::FileContext>,
        actual_context: Option<&crate::context::FileContext>,
        _ngram_sim_threshold: f64,
        weights: &[f64; 14],
        dim_cache: &DimCache,
        config: &ScorerConfig,
        min_evidence_dims: usize,
    ) -> (f64, MatchEvidence) {
        Self::score_against_corpus_with_evidence_impl(
            candidate,
            positives,
            negatives,
            expected_context,
            actual_context,
            _ngram_sim_threshold,
            weights,
            Some(dim_cache),
            config,
            min_evidence_dims,
        )
    }

    fn score_against_corpus_with_evidence_impl(
        candidate: &FunctionFingerprint,
        positives: &[FunctionFingerprint],
        negatives: &[FunctionFingerprint],
        expected_context: Option<&crate::context::FileContext>,
        actual_context: Option<&crate::context::FileContext>,
        _ngram_sim_threshold: f64,
        weights: &[f64; 14],
        dim_cache: Option<&DimCache>,
        config: &ScorerConfig,
        min_evidence_dims: usize,
    ) -> (f64, MatchEvidence) {
        // Inline helper: look up or compute raw_dimensions for a target.
        let raw_dim = |target: &FunctionFingerprint,
                       _is_negative: bool|
         -> crate::pattern::similarity::RawDimensions {
            if let Some(cache) = dim_cache {
                let key = fingerprint_id(target);
                if let Some(cached) = cache.get(&key) {
                    return *cached;
                }
            }
            crate::pattern::similarity::compute_dimensions(candidate, target)
        };

        let mut evidence = MatchEvidence::default();
        let mut best_pos_score = 0.0f64;
        let mut best_dim = RawDimensions::default();

        for (i, positive) in positives.iter().enumerate() {
            let has_flow_match = {
                candidate
                    .data_flow_path_hashes
                    .iter()
                    .any(|h| positive.data_flow_path_hashes.contains(h))
            };

            if !positive.api_calls.is_empty() && !candidate.api_calls.is_empty() {
                let has_overlap = positive
                    .api_calls
                    .iter()
                    .any(|h| candidate.api_calls.contains(h));
                if !has_overlap && !has_flow_match {
                    let motif_overlap = !candidate.motif_hashes.is_empty()
                        && !positive.motif_hashes.is_empty()
                        && positive
                            .motif_hashes
                            .iter()
                            .any(|h| candidate.motif_hashes.contains(h));
                    if !motif_overlap {
                        continue;
                    }
                }
            }

            let mut dim = raw_dim(positive, false);

            let has_flow_match = dim.flow_sim > 0.0 || {
                candidate
                    .data_flow_path_hashes
                    .iter()
                    .any(|h| positive.data_flow_path_hashes.contains(h))
            };

            if has_flow_match && dim.flow_sim == 0.0 {
                dim.flow_sim = 1.0;
            }

            // Early exit: if ngram similarity is very low, this positive can't produce
            // a high score. Skip the expensive weighted_score computation.
            if dim.ngram_sim < 0.05 && dim.api_sim < 0.1 && !has_flow_match {
                continue;
            }

            let sem_mult = if positive.semantic_markers.is_empty() {
                1.0
            } else if dim.semantic_sim == 0.0 {
                config.semantic_zero_penalty
            } else {
                config.semantic_match_boost
            };
            let transfer = cross_lingual_penalty(&positive.language, &candidate.language, config);
            let pos_score = dim.apply_semantic_override(dim.weighted_score(weights))
                * f64::from(transfer)
                * sem_mult;

            if pos_score > best_pos_score {
                best_pos_score = pos_score;
                best_dim = dim;
                evidence.ngram_sim = dim.ngram_sim;
                evidence.ast_sim = dim.ast_sim;
                evidence.signature_sim = dim.signature_sim;
                evidence.control_flow_sim = dim.cf_sim;
                evidence.api_sim = dim.api_sim;
                evidence.motif_sim = dim.motif_sim;
                evidence.flow_sim = if has_flow_match
                    || (!candidate.data_flow_path_hashes.is_empty()
                        && !positive.data_flow_path_hashes.is_empty())
                {
                    Some(dim.flow_sim.max(0.1))
                } else {
                    None
                };
                evidence.semantic_sim = dim.semantic_sim;
                evidence.best_positive_index = i;
                evidence.has_taint_path = has_flow_match;
            }
        }

        // Populate matched/missing calls
        if let Some(best_pos) = positives.get(evidence.best_positive_index) {
            for name in &candidate.raw_call_names {
                let mut h = rustc_hash::FxHasher::default();
                name.hash(&mut h);
                let hash = h.finish();
                if best_pos.api_calls.binary_search(&hash).is_ok() {
                    evidence.matched_calls.push(name.clone());
                } else {
                    evidence.missing_calls.push(name.clone());
                }
            }
            let mut seen_motifs = std::collections::HashSet::new();
            for spec in frensense_lang::registry::all_specs() {
                for &(_member, motif_name) in spec.known_motif_members() {
                    if !seen_motifs.insert(motif_name) {
                        continue;
                    }
                    let mut h = rustc_hash::FxHasher::default();
                    motif_name.hash(&mut h);
                    let motif_hash = h.finish();
                    if candidate.motif_hashes.contains(&motif_hash)
                        && best_pos.motif_hashes.contains(&motif_hash)
                    {
                        evidence.matched_motifs.push(motif_name.to_string());
                    }
                }
            }
        }

        // Per-dimension signal: for API calls, use intersection-size difference
        // (how many MORE calls does the candidate share with the positive than
        // with the negative?), normalized by the positive intersection count.
        // For other dimensions, use Jaccard-based difference (pos_sim - neg_sim).
        let mut worst_neg = RawDimensions::default();
        for negative in negatives {
            let dim = raw_dim(negative, true);
            if dim.ngram_sim > worst_neg.ngram_sim {
                worst_neg.ngram_sim = dim.ngram_sim;
            }
            if dim.ast_sim > worst_neg.ast_sim {
                worst_neg.ast_sim = dim.ast_sim;
            }
            if dim.signature_sim > worst_neg.signature_sim {
                worst_neg.signature_sim = dim.signature_sim;
            }
            if dim.param_type_sim > worst_neg.param_type_sim {
                worst_neg.param_type_sim = dim.param_type_sim;
            }
            if dim.type_usage_sim > worst_neg.type_usage_sim {
                worst_neg.type_usage_sim = dim.type_usage_sim;
            }
            if dim.semantic_sim > worst_neg.semantic_sim {
                worst_neg.semantic_sim = dim.semantic_sim;
            }
            if dim.cf_sim > worst_neg.cf_sim {
                worst_neg.cf_sim = dim.cf_sim;
            }
            if dim.api_sim > worst_neg.api_sim {
                worst_neg.api_sim = dim.api_sim;
            }
            if dim.motif_sim > worst_neg.motif_sim {
                worst_neg.motif_sim = dim.motif_sim;
            }
            if dim.flow_sim > worst_neg.flow_sim {
                worst_neg.flow_sim = dim.flow_sim;
            }
            if dim.tainted_api_sim > worst_neg.tainted_api_sim {
                worst_neg.tainted_api_sim = dim.tainted_api_sim;
            }
            if dim.cf_order_sim > worst_neg.cf_order_sim {
                worst_neg.cf_order_sim = dim.cf_order_sim;
            }
            if dim.arg_type_sim > worst_neg.arg_type_sim {
                worst_neg.arg_type_sim = dim.arg_type_sim;
            }
            if dim.literal_concat_sim > worst_neg.literal_concat_sim {
                worst_neg.literal_concat_sim = dim.literal_concat_sim;
            }
        }

        // API intersection-size signal
        let intersect_count = |a: &[u64], b: &[u64]| -> usize {
            if a.is_empty() || b.is_empty() {
                return 0;
            }
            let set_b: std::collections::HashSet<u64> = b.iter().copied().collect();
            a.iter().filter(|h| set_b.contains(h)).count()
        };
        let best_pos = &positives[evidence.best_positive_index];
        let first_neg = negatives
            .first()
            .map(|n| n.api_calls.as_slice())
            .unwrap_or(&[]);
        let pi = intersect_count(&candidate.api_calls, &best_pos.api_calls);
        let ni = intersect_count(&candidate.api_calls, first_neg);
        let signal_api = if pi > 0 {
            (pi as f64 - ni as f64) / pi as f64
        } else {
            0.0
        };

        let signal: [f64; 14] = [
            (best_dim.ngram_sim - worst_neg.ngram_sim).max(0.0),
            (best_dim.ast_sim - worst_neg.ast_sim).max(0.0),
            (best_dim.signature_sim - worst_neg.signature_sim).max(0.0),
            (best_dim.param_type_sim - worst_neg.param_type_sim).max(0.0),
            (best_dim.type_usage_sim - worst_neg.type_usage_sim).max(0.0),
            (best_dim.semantic_sim - worst_neg.semantic_sim).max(0.0),
            (best_dim.cf_sim - worst_neg.cf_sim).max(0.0),
            signal_api.max(0.0),
            (best_dim.tainted_api_sim - worst_neg.tainted_api_sim).max(0.0),
            (best_dim.motif_sim - worst_neg.motif_sim).max(0.0),
            (best_dim.flow_sim - worst_neg.flow_sim).max(0.0),
            (best_dim.cf_order_sim - worst_neg.cf_order_sim).max(0.0),
            (best_dim.arg_type_sim - worst_neg.arg_type_sim).max(0.0),
            (best_dim.literal_concat_sim - worst_neg.literal_concat_sim).max(0.0),
        ];

        let max_signal = signal.iter().cloned().fold(0.0f64, f64::max);
        evidence.negative_sim = max_signal;

        // Noise gate: require contrastive signal that the positive is more similar
        // than the negative. A single strong dimension or multiple moderate ones.
        let moderate_count = signal
            .iter()
            .filter(|&&s| s > config.noise_gate_moderate_signal)
            .count();
        let signal_sum: f64 = signal.iter().sum();

        let weighted_score = best_dim.apply_semantic_override(best_dim.weighted_score(weights));
        let gate = max_signal > config.noise_gate_strong_signal
            || (moderate_count >= min_evidence_dims && signal_sum > 0.30);

        let final_score = if gate { weighted_score } else { 0.0 };

        let context_multiplier =
            Self::compute_context_penalty(expected_context, actual_context, config);

        (final_score * context_multiplier, evidence)
    }

    // A lightweight identity-hash for a fingerprint, used as a cache key.
}

// Computed from a few identifying fields - collisions are astronomically unlikely.

pub fn fingerprint_id(fp: &FunctionFingerprint) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = rustc_hash::FxHasher::default();
    fp.file_path.hash(&mut hasher);
    fp.function_name.hash(&mut hasher);
    fp.line.hash(&mut hasher);
    fp.structural_markers.len().hash(&mut hasher);
    fp.api_calls.len().hash(&mut hasher);
    hasher.finish()
}

/// Global cache for `raw_dimensions` results across all patterns in a scan.
/// Safe to reuse across `score_against_corpus_with_evidence` calls because
/// the candidate is constant within a single `scan_function` invocation.
pub type DimCache = rustc_hash::FxHashMap<u64, crate::pattern::similarity::RawDimensions>;
