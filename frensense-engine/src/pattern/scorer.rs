// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::fingerprint::FunctionFingerprint;
use crate::minhash;
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
    let all_keys: std::collections::HashSet<_> = a.keys().chain(b.keys()).collect();
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

/// M8: Cross-lingual transfer penalty.
/// If pattern language differs from candidate language, apply penalty.
/// Cross-language matching is useful for catching similar bug patterns across languages,
/// but should be heavily penalized to avoid false positives.
fn cross_lingual_penalty(pattern_lang: &str, candidate_lang: &str) -> f32 {
    if pattern_lang == candidate_lang || pattern_lang == "unknown" || candidate_lang == "unknown" {
        1.0
    } else {
        0.20 // 80% penalty for cross-language matching
    }
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
        weights: &[f64; 10],
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

    fn compute_similarity(
        candidate: &FunctionFingerprint,
        target: &FunctionFingerprint,
        _is_positive: bool,
        ngram_sim_threshold: f64,
        weights: &[f64; 10],
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
        // Motif similarity: two functions calling exec() and Command::new() respectively
        // score 1.0 on motif_sim even though api_sim ≈ 0.
        let motif_sim = jaccard(&candidate.motif_hashes, &target.motif_hashes);
        // Tainted API sim: if the pattern's positive has no tainted calls, skip this dim.
        // Otherwise, functions where input clearly does NOT flow into the sink get penalized.
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
    }

    pub fn similarity_to_positive(
        candidate: &FunctionFingerprint,
        positive: &FunctionFingerprint,
    ) -> f64 {
        let jaccard = |a: &_, b: &_| minhash::jaccard_similarity_sorted(a, b);
        let jaccard_sorted = |a: &_, b: &_| minhash::jaccard_similarity_sorted(a, b);
        let ngram_sim = if candidate.weighted_ngram_hashes.is_empty()
            || positive.weighted_ngram_hashes.is_empty()
        {
            jaccard(&candidate.ngram_hashes, &positive.ngram_hashes)
        } else {
            weighted_jaccard(
                &candidate.weighted_ngram_hashes,
                &positive.weighted_ngram_hashes,
            )
        };
        let cf_sim = jaccard(
            &candidate.control_flow_hashes,
            &positive.control_flow_hashes,
        );
        let api_sim = jaccard(&candidate.api_calls, &positive.api_calls);
        let motif_sim = jaccard(&candidate.motif_hashes, &positive.motif_hashes);
        ngram_sim * 0.20
            + jaccard(&candidate.structural_markers, &positive.structural_markers) * 0.25
            + jaccard_sorted(&candidate.signature_ngrams, &positive.signature_ngrams) * 0.15
            + jaccard_sorted(&candidate.param_type_ngrams, &positive.param_type_ngrams) * 0.05
            + type_usage_overlap(candidate, positive) * 0.05
            + cf_sim * 0.10
            + api_sim * 0.10
            + motif_sim * 0.10
    }

    pub fn similarity_to_negative(
        candidate: &FunctionFingerprint,
        negative: &FunctionFingerprint,
    ) -> f64 {
        let jaccard = |a: &_, b: &_| minhash::jaccard_similarity_sorted(a, b);
        let jaccard_sorted = |a: &_, b: &_| minhash::jaccard_similarity_sorted(a, b);
        let ngram_sim = if candidate.weighted_ngram_hashes.is_empty()
            || negative.weighted_ngram_hashes.is_empty()
        {
            jaccard(&candidate.ngram_hashes, &negative.ngram_hashes)
        } else {
            weighted_jaccard(
                &candidate.weighted_ngram_hashes,
                &negative.weighted_ngram_hashes,
            )
        };
        let cf_sim = jaccard(
            &candidate.control_flow_hashes,
            &negative.control_flow_hashes,
        );
        let api_sim = jaccard(&candidate.api_calls, &negative.api_calls);
        let motif_sim = jaccard(&candidate.motif_hashes, &negative.motif_hashes);
        ngram_sim * 0.20
            + jaccard(&candidate.structural_markers, &negative.structural_markers) * 0.25
            + jaccard_sorted(&candidate.signature_ngrams, &negative.signature_ngrams) * 0.15
            + jaccard_sorted(&candidate.param_type_ngrams, &negative.param_type_ngrams) * 0.05
            + type_usage_overlap(candidate, negative) * 0.05
            + cf_sim * 0.10
            + api_sim * 0.10
            + motif_sim * 0.10
    }
}

pub(crate) fn type_usage_overlap(a: &FunctionFingerprint, b: &FunctionFingerprint) -> f64 {
    if a.type_usages.is_empty() && b.type_usages.is_empty() {
        return 0.5;
    }
    let set_a: std::collections::HashSet<_> = a.type_usages.iter().collect();
    let set_b: std::collections::HashSet<_> = b.type_usages.iter().collect();
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
        let default_w = &[0.12, 0.20, 0.08, 0.04, 0.03, 0.12, 0.10, 0.10, 0.15, 0.06];
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
        let default_w = &[0.12, 0.20, 0.08, 0.04, 0.03, 0.12, 0.10, 0.10, 0.15, 0.06];
        let score = PatternScorer::score_against_corpus(&cand, &[pos], &[neg], None, None, 0.5, default_w);
        assert!(score < 0.6, "candidate closer to negative should score low");
    }
}
