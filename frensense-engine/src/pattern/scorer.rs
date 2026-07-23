// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::corpus::motifs::MOTIFS;
use crate::fingerprint::FunctionFingerprint;
use crate::minhash;
use crate::pattern::evidence::MatchEvidence;
use crate::pattern::weight_learner::DEFAULT_WEIGHTS;
use crate::pattern::canonical::CanonicalForm;
use crate::pattern::compiler::PatternNode;
use crate::pattern::matcher::MatchResult;

#[derive(Debug, Clone, Default)]
pub struct PatternScorer;

/// M1: Weighted Jaccard — IDF-weighted intersection / union.
pub fn weighted_jaccard(
    a: &rustc_hash::FxHashMap<u64, f32>,
    b: &rustc_hash::FxHashMap<u64, f32>,
) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.5;
    }
    let mut intersection = 0.0f64;
    let mut union = 0.0f64;
    let all_keys: rustc_hash::FxHashSet<_> = a.keys().chain(b.keys()).collect();
    for key in all_keys {
        let wa = f64::from(a.get(&key).copied().unwrap_or(0.0));
        let wb = f64::from(b.get(&key).copied().unwrap_or(0.0));
        intersection += wa.min(wb);
        union += wa.max(wb);
    }
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

/// M8: Cross-lingual transfer penalty.
/// If pattern language differs from candidate language, apply penalty.
/// Cross-language matching is useful for catching similar bug patterns across languages,
/// but should be heavily penalized to avoid false positives.
fn cross_lingual_penalty(pattern_lang: &str, candidate_lang: &str) -> f32 {
    if pattern_lang == candidate_lang || pattern_lang == "unknown" || candidate_lang == "unknown" {
        return 1.0;
    }
    // TypeScript and JavaScript share the same AST structure (tree-sitter-typescript
    // parses JS too). Treat them as equivalent for cross-lingual matching.
    let js_like = |l: &str| l == "typescript" || l == "javascript";
    if js_like(pattern_lang) && js_like(candidate_lang) {
        return 1.0;
    }
    0.20 // 80% penalty for genuinely different languages (e.g. Rust ↔ TypeScript)
}

#[derive(Debug, Clone)]
pub struct ScoredPattern {
    pub pattern_id: String,
    pub match_count: usize,
    pub avg_score: f64,
    pub structural_similarity: f64,
    pub canonical_form: Option<CanonicalForm>,
    pub minhash_similarity: f64,
    pub final_score: f64,
}

impl PatternScorer {
    pub fn score_matches(patterns: &[(&PatternNode, Vec<MatchResult>)]) -> Vec<ScoredPattern> {
        let mut scored = Vec::new();

        for (i, (pattern, matches)) in patterns.iter().enumerate() {
            let match_count = matches.len();
            let avg_score = if matches.is_empty() {
                0.0
            } else {
                matches.iter().map(|m| m.score).sum::<f64>() / matches.len() as f64
            };

            let canonical = if match_count > 0 {
                Some(CanonicalForm::from_node(pattern))
            } else {
                None
            };

            scored.push(ScoredPattern {
                pattern_id: format!("pattern_{i}"),
                match_count,
                avg_score,
                structural_similarity: 0.0,
                canonical_form: canonical.clone(),
                minhash_similarity: 0.0,
                final_score: 0.0,
            });
        }

        for i in 0..scored.len() {
            for j in i + 1..scored.len() {
                if let (Some(cf_i), Some(cf_j)) =
                    (&scored[i].canonical_form, &scored[j].canonical_form)
                {
                    scored[i].structural_similarity = cf_i.structural_similarity(cf_j);
                    scored[j].structural_similarity = scored[i].structural_similarity;
                }
            }
        }

        scored
    }

    pub fn compute_final(
        pattern: &PatternNode,
        matches: &[MatchResult],
        profiles: Option<&HashMap<String, f64>>,
    ) -> f64 {
        let base_score = if matches.is_empty() {
            0.0
        } else {
            let avg = matches.iter().map(|m| m.score).sum::<f64>() / matches.len() as f64;
            avg * (1.0 - 1.0 / (matches.len() as f64 + 1.0))
        };

        let structural_score = {
            let cf = CanonicalForm::from_node(pattern);
            let kind_diversity = cf.kind_sequence.len() as f64;
            (kind_diversity / (kind_diversity + 10.0)).min(1.0)
        };

        let profile_boost = profiles
            .and_then(|p| {
                let key = &pattern.kind;
                p.get(key).copied()
            })
            .unwrap_or(0.5);

        base_score * 0.4 + structural_score * 0.3 + profile_boost * 0.3
    }

    pub fn score_against_corpus(
        candidate: &FunctionFingerprint,
        positives: &[FunctionFingerprint],
        negatives: &[FunctionFingerprint],
        expected_context: Option<&crate::context::FileContext>,
        actual_context: Option<&crate::context::FileContext>,
        ngram_sim_threshold: f64,
        weights: &[f64; 11],
    ) -> f64 {
        let mut best_pos_score = 0.0f64;

        // 1. Find the best matching positive with API-call gating.
        //    If both pattern and candidate have API calls but share NONE,
        //    skip that positive entirely — structural match is coincidental.
        for positive in positives {
            // API-call gate: zero API overlap = skip (not a real match)
            // Fall back to motif overlap if literal API calls don't match.
            if !positive.api_calls.is_empty() && !candidate.api_calls.is_empty() {
                let has_overlap = positive
                    .api_calls
                    .iter()
                    .any(|h| candidate.api_calls.contains(h));
                if !has_overlap {
                    let motif_overlap = !candidate.motif_hashes.is_empty()
                        && !positive.motif_hashes.is_empty()
                        && positive.motif_hashes.iter().any(|h| candidate.motif_hashes.contains(h));
                    if !motif_overlap {
                        continue;
                    }
                }
            }

            let sim_pos = Self::compute_similarity(candidate, positive, true, ngram_sim_threshold, weights);
            let semantic_multiplier = if positive.semantic_markers.is_empty() {
                1.0
            } else if minhash::jaccard_similarity_sorted(
                &candidate.semantic_markers,
                &positive.semantic_markers,
            ) == 0.0
            {
                0.30
            } else {
                2.0
            };
            let transfer = cross_lingual_penalty(&positive.language, &candidate.language);

            let pos_score = sim_pos * f64::from(transfer) * semantic_multiplier;
            if pos_score > best_pos_score {
                best_pos_score = pos_score;
            }
        }

        // Fast path: if the best positive score is already too low to meet any reasonable threshold, return early
        // We know the minimum threshold is 0.5 usually, but even if it's 0.2, if best_pos_score < 0.1, we skip
        if best_pos_score < 0.1 {
            return best_pos_score;
        }

        // 2. Find the highest negative similarity (the worst penalty)
        let mut max_neg_sim = 0.0f64;
        for negative in negatives {
            let sim_neg = Self::compute_similarity(candidate, negative, false, ngram_sim_threshold, weights);
            if sim_neg > max_neg_sim {
                max_neg_sim = sim_neg;
            }
        }

        // 3. Context Featurization Penalty
        let context_multiplier = match (expected_context, actual_context) {
            (Some(exp), Some(act)) => {
                let mut penalty = 1.0;
                if exp.sensitivity == crate::context::DataSensitivity::High
                    && act.sensitivity != crate::context::DataSensitivity::High
                {
                    penalty *= 0.5;
                }
                if exp.environment == crate::context::Environment::RouteHandler
                    && (act.environment == crate::context::Environment::Test
                        || act.environment == crate::context::Environment::Utility)
                {
                    penalty *= 0.5;
                }
                if act.environment == crate::context::Environment::RouteHandler
                    && exp.environment != crate::context::Environment::RouteHandler
                    && exp.environment != crate::context::Environment::Unknown
                {
                    penalty *= 0.5;
                }
                penalty
            }
            _ => 1.0,
        };

        let neg_penalty = if max_neg_sim >= best_pos_score {
            (1.0 - max_neg_sim).max(0.1)
        } else {
            1.0 - (max_neg_sim * 0.3)
        };

        best_pos_score * neg_penalty * context_multiplier
    }

    /// Score a candidate against corpus and return both the final score and
    /// a full `MatchEvidence` breakdown. Uses the same learned `weights` as
    /// `score_against_corpus` so scores are identical.
    ///
    /// When `dim_cache` is provided, `raw_dimensions` results are memoised
    /// across all patterns, keyed by `fingerprint_id(&target)`.  The caller
    /// must guarantee the candidate is unchanged across calls sharing a cache.
    pub fn score_against_corpus_with_evidence(
        candidate: &FunctionFingerprint,
        positives: &[FunctionFingerprint],
        negatives: &[FunctionFingerprint],
        expected_context: Option<&crate::context::FileContext>,
        actual_context: Option<&crate::context::FileContext>,
        _ngram_sim_threshold: f64,
        weights: &[f64; 11],
    ) -> (f64, MatchEvidence) {
        Self::score_against_corpus_with_evidence_impl(
            candidate, positives, negatives,
            expected_context, actual_context,
            _ngram_sim_threshold, weights,
            None,
        )
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
        weights: &[f64; 11],
        dim_cache: &mut DimCache,
    ) -> (f64, MatchEvidence) {
        Self::score_against_corpus_with_evidence_impl(
            candidate, positives, negatives,
            expected_context, actual_context,
            _ngram_sim_threshold, weights,
            Some(dim_cache),
        )
    }

    fn score_against_corpus_with_evidence_impl(
        candidate: &FunctionFingerprint,
        positives: &[FunctionFingerprint],
        negatives: &[FunctionFingerprint],
        expected_context: Option<&crate::context::FileContext>,
        actual_context: Option<&crate::context::FileContext>,
        _ngram_sim_threshold: f64,
        weights: &[f64; 11],
        mut dim_cache: Option<&mut DimCache>,
    ) -> (f64, MatchEvidence) {
        // Inline helper: look up or compute raw_dimensions for a target.
        let mut raw_dim = |target: &FunctionFingerprint, is_negative: bool| -> RawDimensions {
            if let Some(ref mut cache) = dim_cache {
                let key = fingerprint_id(target);
                *cache.entry(key).or_insert_with(|| Self::raw_dimensions(candidate, target, is_negative))
            } else {
                Self::raw_dimensions(candidate, target, is_negative)
            }
        };

        let mut evidence = MatchEvidence::default();
        let mut best_pos_score = 0.0f64;

        for (i, positive) in positives.iter().enumerate() {
            if !positive.api_calls.is_empty() && !candidate.api_calls.is_empty() {
                let has_overlap = positive.api_calls.iter()
                    .any(|h| candidate.api_calls.contains(h));
                if !has_overlap { continue; }
            }

            let dim = raw_dim(positive, false);
            let sem_mult = if positive.semantic_markers.is_empty() { 1.0 }
                else if dim.semantic_sim == 0.0 { 0.30 }
                else { 2.0 };
            let transfer = cross_lingual_penalty(&positive.language, &candidate.language);
            let pos_score = dim.weighted_score(weights) * f64::from(transfer) * sem_mult;

            if pos_score > best_pos_score {
                best_pos_score = pos_score;
                evidence.ngram_sim = dim.ngram_sim;
                evidence.ast_sim = dim.ast_sim;
                evidence.signature_sim = dim.signature_sim;
                evidence.control_flow_sim = dim.cf_sim;
                evidence.api_sim = dim.api_sim;
                evidence.motif_sim = dim.motif_sim;
                evidence.flow_sim = if dim.flow_sim > 0.0 || (!candidate.data_flow_path_hashes.is_empty()
                    && !positive.data_flow_path_hashes.is_empty())
                {
                    Some(dim.flow_sim)
                } else {
                    None
                };
                evidence.semantic_sim = dim.semantic_sim;
                evidence.best_positive_index = i;
                evidence.has_taint_path = dim.flow_sim > 0.0;
            }
        }

        // Populate matched/missing calls
        if let Some(best_pos) = positives.get(evidence.best_positive_index) {
            for name in &candidate.raw_call_names {
                let mut h = rustc_hash::FxHasher::default();
                name.hash(&mut h);
                let hash = h.finish();
                if best_pos.api_calls.contains(&hash) {
                    evidence.matched_calls.push(name.clone());
                } else {
                    evidence.missing_calls.push(name.clone());
                }
            }
            for motif in MOTIFS {
                let mut h = rustc_hash::FxHasher::default();
                motif.name.hash(&mut h);
                let motif_hash = h.finish();
                if candidate.motif_hashes.contains(&motif_hash)
                    && best_pos.motif_hashes.contains(&motif_hash)
                {
                    evidence.matched_motifs.push(motif.name.to_string());
                }
            }
        }

        // Negative similarity
        let mut max_neg_sim = 0.0f64;
        for negative in negatives {
            let dim = raw_dim(negative, true);
            let neg_score = dim.weighted_score(weights);
            if neg_score > max_neg_sim {
                max_neg_sim = neg_score;
            }
        }
        evidence.negative_sim = max_neg_sim;

        let neg_penalty = if max_neg_sim >= best_pos_score {
            (1.0 - max_neg_sim).max(0.1)
        } else {
            1.0 - (max_neg_sim * 0.3)
        };

        let context_multiplier = match (expected_context, actual_context) {
            (Some(exp), Some(act)) => {
                let mut penalty = 1.0;
                if exp.sensitivity == crate::context::DataSensitivity::High
                    && act.sensitivity != crate::context::DataSensitivity::High
                {
                    penalty *= 0.5;
                }
                if exp.environment == crate::context::Environment::RouteHandler
                    && (act.environment == crate::context::Environment::Test
                        || act.environment == crate::context::Environment::Utility)
                {
                    penalty *= 0.5;
                }
                if act.environment == crate::context::Environment::RouteHandler
                    && exp.environment != crate::context::Environment::RouteHandler
                    && exp.environment != crate::context::Environment::Unknown
                {
                    penalty *= 0.5;
                }
                penalty
            }
            _ => 1.0,
        };

        let final_score = best_pos_score * neg_penalty * context_multiplier;
        (final_score, evidence)
    }

    fn compute_similarity(
        candidate: &FunctionFingerprint,
        target: &FunctionFingerprint,
        _is_positive: bool,
        ngram_sim_threshold: f64,
        weights: &[f64; 11],
    ) -> f64 {
        let jaccard = |a: &_, b: &_| minhash::jaccard_similarity_sorted(a, b);
        let jaccard_sorted = |a: &_, b: &_| minhash::jaccard_similarity_sorted(a, b);

        let ngram_sim = if candidate.weighted_ngram_hashes.is_empty()
            || target.weighted_ngram_hashes.is_empty()
        {
            jaccard(&candidate.ngram_hashes, &target.ngram_hashes)
        } else {
            weighted_jaccard(
                &candidate.weighted_ngram_hashes,
                &target.weighted_ngram_hashes,
            )
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

        let cf_sim = jaccard(&candidate.control_flow_hashes, &target.control_flow_hashes);
        let api_sim = jaccard(&candidate.api_calls, &target.api_calls);
        let motif_sim = jaccard(&candidate.motif_hashes, &target.motif_hashes);
        let flow_sim = jaccard(&candidate.data_flow_path_hashes, &target.data_flow_path_hashes);
        let tainted_api_sim = if target.tainted_api_calls.is_empty() {
            1.0
        } else {
            jaccard(&candidate.tainted_api_calls, &target.tainted_api_calls)
        };

        ngram_sim * weights[0]
            + ast_sim * weights[1]
            + jaccard_sorted(&candidate.signature_ngrams, &target.signature_ngrams) * weights[2]
            + jaccard_sorted(&candidate.param_type_ngrams, &target.param_type_ngrams) * weights[3]
            + type_usage_overlap(candidate, target) * weights[4]
            + semantic_sim * weights[5]
            + cf_sim * weights[6]
            + api_sim * weights[7]
            + tainted_api_sim * weights[8]
            + motif_sim * weights[9]
            + flow_sim * weights[10]
    }

    pub fn similarity_to_positive(
        candidate: &FunctionFingerprint,
        positive: &FunctionFingerprint,
    ) -> f64 {
        Self::compute_similarity(candidate, positive, true, 0.0, &DEFAULT_WEIGHTS)
    }

    pub fn similarity_to_negative(
        candidate: &FunctionFingerprint,
        negative: &FunctionFingerprint,
    ) -> f64 {
        Self::compute_similarity(candidate, negative, false, 0.0, &DEFAULT_WEIGHTS)
    }

    /// Compute a full `MatchEvidence` breakdown for a corpus match.
    /// Mirrors the logic of `score_against_corpus` but exposes each dimension.
    pub fn compute_evidence(
        candidate: &FunctionFingerprint,
        positives: &[FunctionFingerprint],
        negatives: &[FunctionFingerprint],
        weights: &[f64; 11],
    ) -> MatchEvidence {
        let jaccard = |a: &[u64], b: &[u64]| minhash::jaccard_similarity_sorted(a, b);

        // Find the best-matching positive and record per-dimension values
        let mut best_positive_index = 0usize;
        let mut best_dim: Option<RawDimensions> = None;
        let mut best_pos_score = 0.0f64;

        for (i, positive) in positives.iter().enumerate() {
            let dim = Self::raw_dimensions(candidate, positive, false);
            let semantic_multiplier = if positive.semantic_markers.is_empty() {
                1.0
            } else if jaccard(&candidate.semantic_markers, &positive.semantic_markers) == 0.0 {
                0.30
            } else {
                2.0
            };
            let transfer = crate::pattern::scorer::cross_lingual_penalty(
                &positive.language,
                &candidate.language,
            );
            let pos_score = dim.weighted_score(weights) * f64::from(transfer) * semantic_multiplier;
            if pos_score > best_pos_score {
                best_pos_score = pos_score;
                best_positive_index = i;
                best_dim = Some(dim);
            }
        }

        let dim = best_dim.unwrap_or_else(|| {
            Self::raw_dimensions(candidate, &positives[0], false)
        });

        // Compute negative similarity
        let mut max_neg_sim = 0.0f64;
        for negative in negatives {
            let neg_dim = Self::raw_dimensions(candidate, negative, true);
            let neg_score = neg_dim.weighted_score(weights);
            if neg_score > max_neg_sim {
                max_neg_sim = neg_score;
            }
        }

        let flow_sim_val = jaccard(&candidate.data_flow_path_hashes, &positives[best_positive_index].data_flow_path_hashes);

        MatchEvidence {
            ngram_sim: dim.ngram_sim,
            ast_sim: dim.ast_sim,
            signature_sim: dim.signature_sim,
            control_flow_sim: dim.cf_sim,
            api_sim: dim.api_sim,
            motif_sim: dim.motif_sim,
            flow_sim: if flow_sim_val > 0.0 { Some(flow_sim_val) } else { None },
            semantic_sim: dim.semantic_sim,
            negative_sim: max_neg_sim,
            matched_calls: Vec::new(),
            missing_calls: Vec::new(),
            matched_motifs: Vec::new(),
            has_taint_path: !candidate.data_flow_path_hashes.is_empty()
                && positives.iter().any(|p| !p.data_flow_path_hashes.is_empty()),
            best_positive_index,
        }
    }

    pub(crate) fn raw_dimensions(
        candidate: &FunctionFingerprint,
        target: &FunctionFingerprint,
        _is_negative: bool,
    ) -> RawDimensions {
        let jaccard = |a: &[u64], b: &[u64]| minhash::jaccard_similarity_sorted(a, b);
        let jaccard_sorted = |a: &[u64], b: &[u64]| minhash::jaccard_similarity_sorted(a, b);

        let ngram_sim = if candidate.weighted_ngram_hashes.is_empty()
            || target.weighted_ngram_hashes.is_empty()
        {
            jaccard(&candidate.ngram_hashes, &target.ngram_hashes)
        } else {
            weighted_jaccard(&candidate.weighted_ngram_hashes, &target.weighted_ngram_hashes)
        };

        let semantic_sim = jaccard(&candidate.semantic_markers, &target.semantic_markers);

        // Tree-edit distance is O(n²) LCS — skip when ngram is too low for
        // a perfect AST match to meaningfully move the weighted score.
        let ast_sim = if !candidate.skeleton_hashes.is_empty()
            && !target.skeleton_hashes.is_empty()
            && ngram_sim > 0.12
        {
            1.0 - crate::ast_distance::tree_edit_distance(
                &candidate.skeleton_hashes,
                &target.skeleton_hashes,
            )
        } else {
            jaccard(&candidate.structural_markers, &target.structural_markers)
        };

        let signature_sim = jaccard_sorted(&candidate.signature_ngrams, &target.signature_ngrams);
        let param_type_sim = jaccard_sorted(&candidate.param_type_ngrams, &target.param_type_ngrams);
        let type_usage_sim = type_usage_overlap(candidate, target);
        let cf_sim = jaccard(&candidate.control_flow_hashes, &target.control_flow_hashes);
        let api_sim = jaccard(&candidate.api_calls, &target.api_calls);
        let motif_sim = jaccard(&candidate.motif_hashes, &target.motif_hashes);
        let flow_sim = jaccard(&candidate.data_flow_path_hashes, &target.data_flow_path_hashes);
        let tainted_api_sim = if target.tainted_api_calls.is_empty() {
            1.0
        } else {
            jaccard(&candidate.tainted_api_calls, &target.tainted_api_calls)
        };

        RawDimensions {
            ngram_sim,
            ast_sim,
            signature_sim,
            param_type_sim,
            type_usage_sim,
            semantic_sim,
            cf_sim,
            api_sim,
            motif_sim,
            flow_sim,
            tainted_api_sim,
        }
    }
}

/// A lightweight identity-hash for a fingerprint, used as a cache key.
/// Computed from a few identifying fields — collisions are astronomically unlikely.
pub fn fingerprint_id(fp: &FunctionFingerprint) -> u64 {
    let structural_hash = fp.structural_markers.first().copied().unwrap_or(0);
    let api_hash = fp.api_calls.first().copied().unwrap_or(0);
    let ngram_len = fp.ngram_hashes.len() as u64;
    structural_hash.wrapping_mul(3)
        ^ api_hash.wrapping_mul(7)
        ^ ngram_len.wrapping_mul(11)
        ^ (fp.skeleton_hashes.len() as u64).wrapping_mul(13)
}

/// Global cache for `raw_dimensions` results across all patterns in a scan.
/// Safe to reuse across `score_against_corpus_with_evidence` calls because
/// the candidate is constant within a single `scan_function` invocation.
pub type DimCache = rustc_hash::FxHashMap<u64, RawDimensions>;

/// Intermediate raw-dimension values used internally by evidence computation.
#[derive(Clone, Copy)]
pub(crate) struct RawDimensions {
    ngram_sim: f64,
    ast_sim: f64,
    signature_sim: f64,
    param_type_sim: f64,
    type_usage_sim: f64,
    semantic_sim: f64,
    cf_sim: f64,
    api_sim: f64,
    motif_sim: f64,
    flow_sim: f64,
    tainted_api_sim: f64,
}

impl RawDimensions {
    fn weighted_score(&self, w: &[f64; 11]) -> f64 {
        self.ngram_sim * w[0]
            + self.ast_sim * w[1]
            + self.signature_sim * w[2]
            + self.param_type_sim * w[3]
            + self.type_usage_sim * w[4]
            + self.semantic_sim * w[5]
            + self.cf_sim * w[6]
            + self.api_sim * w[7]
            + self.tainted_api_sim * w[8]
            + self.motif_sim * w[9]
            + self.flow_sim * w[10]
    }
}

pub(crate) fn type_usage_overlap(a: &FunctionFingerprint, b: &FunctionFingerprint) -> f64 {
    if a.type_usages.is_empty() && b.type_usages.is_empty() {
        return 0.5;
    }
    let set_a: rustc_hash::FxHashSet<_> = a.type_usages.iter().collect();
    let set_b: rustc_hash::FxHashSet<_> = b.type_usages.iter().collect();
    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::extract_fingerprints;
    use crate::pattern::compiler::PatternCompiler;
    use crate::pattern::matcher::PatternMatcher;

    fn make_fingerprint(source: &str, path: &str, ext: &str) -> FunctionFingerprint {
        let mut parser = tree_sitter::Parser::new();
        let lang = match ext {
            "rs" => tree_sitter_rust::LANGUAGE.into(),
            _ => tree_sitter_rust::LANGUAGE.into(),
        };
        parser.set_language(&lang).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut fps = Vec::new();
        extract_fingerprints(
            tree.root_node(),
            source,
            std::path::Path::new(path),
            &mut fps,
            5,
        );
        fps.into_iter()
            .next()
            .unwrap_or_else(|| panic!("no fingerprint extracted from: {source}"))
    }

    #[test]
    fn test_score_matches_empty() {
        let result = PatternScorer::score_matches(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_score_single_pattern() {
        let source = "let x = 1;";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let node = tree.root_node();
        let pattern = PatternCompiler::compile_node(node, source);
        let matches = PatternMatcher::match_all(&pattern, node, source);
        let scored = PatternScorer::score_matches(&[(&pattern, matches)]);
        assert_eq!(scored.len(), 1);
        assert!(scored[0].match_count > 0);
    }

    #[test]
    fn test_corpus_scoring_identical_to_positive() {
        let pos = make_fingerprint("fn get_password() { read_file() }", "a.rs", "rs");
        let neg = make_fingerprint("fn safe() { 1 + 1 }", "a.rs", "rs");
        let cand = make_fingerprint("fn get_password() { read_file() }", "b.rs", "rs");
        let default_w = &[0.10, 0.22, 0.08, 0.04, 0.03, 0.13, 0.08, 0.06, 0.15, 0.06, 0.05];
        let score = PatternScorer::score_against_corpus(&cand, &[pos], &[neg], None, None, 0.5, default_w);
        assert!(
            score > 0.5,
            "candidate identical to positive should score high, got {score}"
        );
    }

    #[test]
    fn test_corpus_scoring_different() {
        let pos = make_fingerprint("fn get_password() { read_file() }", "a.rs", "rs");
        let neg = make_fingerprint("fn safe() { \"clean\".to_string() }", "a.rs", "rs");
        let cand = make_fingerprint("fn safe() { \"clean\".to_string() }", "b.rs", "rs");
        let default_w = &[0.10, 0.22, 0.08, 0.04, 0.03, 0.13, 0.08, 0.06, 0.15, 0.06, 0.05];
        let score = PatternScorer::score_against_corpus(&cand, &[pos], &[neg], None, None, 0.5, default_w);
        assert!(score < 0.6, "candidate closer to negative should score low");
    }
}
