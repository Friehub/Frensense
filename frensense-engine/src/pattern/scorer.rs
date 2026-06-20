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
fn weighted_jaccard(
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
fn cross_lingual_penalty(pattern_lang: &str, candidate_lang: &str) -> f32 {
    if pattern_lang == candidate_lang || pattern_lang == "unknown" || candidate_lang == "unknown" {
        1.0
    } else {
        0.75 // 25% penalty for cross-language matching
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
        positive: &FunctionFingerprint,
        negative: &FunctionFingerprint,
    ) -> f64 {
        let jaccard = |a: &_, b: &_| minhash::jaccard_similarity(a, b);

        // M1: Use weighted Jaccard for n-grams when weights are available
        let ngram_sim_pos = if candidate.weighted_ngram_hashes.is_empty()
            || positive.weighted_ngram_hashes.is_empty()
        {
            jaccard(&candidate.ngram_hashes, &positive.ngram_hashes)
        } else {
            weighted_jaccard(
                &candidate.weighted_ngram_hashes,
                &positive.weighted_ngram_hashes,
            )
        };
        let ngram_sim_neg = if candidate.weighted_ngram_hashes.is_empty()
            || negative.weighted_ngram_hashes.is_empty()
        {
            jaccard(&candidate.ngram_hashes, &negative.ngram_hashes)
        } else {
            weighted_jaccard(
                &candidate.weighted_ngram_hashes,
                &negative.weighted_ngram_hashes,
            )
        };

        // Semantic similarity: how many API categories overlap
        let semantic_sim_pos = jaccard(&candidate.semantic_markers, &positive.semantic_markers);
        let semantic_sim_neg = jaccard(&candidate.semantic_markers, &negative.semantic_markers);

        // M2: AST edit distance (structural similarity)
        let ast_sim_pos = if !candidate.skeleton.is_empty() && !positive.skeleton.is_empty() {
            1.0 - crate::ast_distance::tree_edit_distance(&candidate.skeleton, &positive.skeleton)
        } else {
            jaccard(&candidate.structural_markers, &positive.structural_markers)
        };
        let ast_sim_neg = if !candidate.skeleton.is_empty() && !negative.skeleton.is_empty() {
            1.0 - crate::ast_distance::tree_edit_distance(&candidate.skeleton, &negative.skeleton)
        } else {
            jaccard(&candidate.structural_markers, &negative.structural_markers)
        };

        // Weighted blend: AST distance gets 40% weight for structural discrimination
        let sim_to_positive = ngram_sim_pos * 0.20
            + ast_sim_pos * 0.40
            + jaccard(&candidate.signature_ngrams, &positive.signature_ngrams) * 0.10
            + jaccard(&candidate.param_type_ngrams, &positive.param_type_ngrams) * 0.05
            + type_usage_overlap(candidate, positive) * 0.05
            + semantic_sim_pos * 0.20;

        let sim_to_negative = ngram_sim_neg * 0.20
            + ast_sim_neg * 0.40
            + jaccard(&candidate.signature_ngrams, &negative.signature_ngrams) * 0.10
            + jaccard(&candidate.param_type_ngrams, &negative.param_type_ngrams) * 0.05
            + type_usage_overlap(candidate, negative) * 0.05
            + semantic_sim_neg * 0.20;

        // M8: Apply cross-lingual transfer penalty
        let transfer = cross_lingual_penalty(&positive.language, &candidate.language);

        sim_to_positive * (1.0 - sim_to_negative) * f64::from(transfer)
    }

    pub fn similarity_to_positive(
        candidate: &FunctionFingerprint,
        positive: &FunctionFingerprint,
    ) -> f64 {
        let jaccard = |a: &_, b: &_| minhash::jaccard_similarity(a, b);
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
        ngram_sim * 0.35
            + jaccard(&candidate.structural_markers, &positive.structural_markers) * 0.30
            + jaccard(&candidate.signature_ngrams, &positive.signature_ngrams) * 0.20
            + jaccard(&candidate.param_type_ngrams, &positive.param_type_ngrams) * 0.10
            + type_usage_overlap(candidate, positive) * 0.05
    }

    pub fn similarity_to_negative(
        candidate: &FunctionFingerprint,
        negative: &FunctionFingerprint,
    ) -> f64 {
        let jaccard = |a: &_, b: &_| minhash::jaccard_similarity(a, b);
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
        ngram_sim * 0.35
            + jaccard(&candidate.structural_markers, &negative.structural_markers) * 0.30
            + jaccard(&candidate.signature_ngrams, &negative.signature_ngrams) * 0.20
            + jaccard(&candidate.param_type_ngrams, &negative.param_type_ngrams) * 0.10
            + type_usage_overlap(candidate, negative) * 0.05
    }
}

fn type_usage_overlap(a: &FunctionFingerprint, b: &FunctionFingerprint) -> f64 {
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
        let score = PatternScorer::score_against_corpus(&cand, &pos, &neg);
        assert!(
            score > 0.6,
            "candidate identical to positive should score high, got {score}"
        );
    }

    #[test]
    fn test_corpus_scoring_different() {
        let pos = make_fingerprint("fn get_password() { read_file() }", "a.rs", "rs");
        let neg = make_fingerprint("fn safe() { \"clean\".to_string() }", "a.rs", "rs");
        let cand = make_fingerprint("fn safe() { \"clean\".to_string() }", "b.rs", "rs");
        let score = PatternScorer::score_against_corpus(&cand, &pos, &neg);
        assert!(score < 0.6, "candidate closer to negative should score low");
    }
}
