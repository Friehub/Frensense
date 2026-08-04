// SPDX-License-Identifier: MIT

//! Layer signal composition for cross-layer confirmation.
//!
//! Replaces the coincidence-counter in `boost_overlap_confidence` with a real
//! AND-gate that checks whether layers are causally related, not just co-located.

use crate::Advisory;

// ─────────────────────────────────────────────────────────────────────────────
// Composition constants
//
// These thresholds and factors shape how layer signals combine into a final
// confidence. They are named and documented here so a single wrong value cannot
// silently change behaviour across the whole pipeline. The values are also
// exposed as fields on `CompositionConfig` (and on the engine) so they can be
// tuned per deployment.
// ─────────────────────────────────────────────────────────────────────────────

/// Multiplier applied to a corpus match that is NOT corroborated by confirmed
/// taint flow (L2). Corpus-only structural matches are down-weighted to ×0.6;
/// full corroboration (corpus + taint) keeps the base score unchanged.
pub const TAINT_UNCONFIRMED_PENALTY: f64 = 0.6;

/// Branch-ratio threshold for L3 validator suppression. Only a very high ratio
/// (>0.85) combined with a validator-style name suppresses the score, so real
/// vulnerabilities that branch on tainted input survive.
pub const HIGH_BRANCH_RATIO_THRESHOLD: f64 = 0.85;

/// Factor applied when suppressing a high-branch-ratio validator (×0.3).
pub const HIGH_BRANCH_RATIO_SUPPRESSION_FACTOR: f64 = 0.3;

/// Default per-duplicate (L4) lift factor: score × (1.0 + `boost_rate`).
pub const DEFAULT_BOOST_RATE: f64 = 0.10;

/// Default absolute ceiling on how much confidence L4 (near-duplicate) can add.
pub const DEFAULT_BOOST_MAX: f64 = 0.30;

/// Tunable parameters for `compose_confidence`. Every field has a documented
/// default (`CompositionConfig::default()`), and the engine exposes the same
/// knobs as configuration fields.
#[derive(Debug, Clone, Copy)]
pub struct CompositionConfig {
    /// Per-duplicate (L4) lift factor: score × (1.0 + `boost_rate`).
    pub boost_rate: f64,
    /// Absolute ceiling on how much confidence L4 can add.
    pub boost_max: f64,
    /// Multiplier for corpus-only matches with no taint confirmation (L2).
    pub taint_unconfirmed_penalty: f64,
    /// Branch-ratio threshold above which validator-named functions are suppressed (L3).
    pub high_branch_ratio_threshold: f64,
    /// Factor applied when suppressing a high-branch-ratio validator (L3).
    pub high_branch_ratio_suppression_factor: f64,
}

impl Default for CompositionConfig {
    fn default() -> Self {
        Self {
            boost_rate: DEFAULT_BOOST_RATE,
            boost_max: DEFAULT_BOOST_MAX,
            taint_unconfirmed_penalty: TAINT_UNCONFIRMED_PENALTY,
            high_branch_ratio_threshold: HIGH_BRANCH_RATIO_THRESHOLD,
            high_branch_ratio_suppression_factor: HIGH_BRANCH_RATIO_SUPPRESSION_FACTOR,
        }
    }
}

/// Signals from different analysis layers for a single function.
#[derive(Debug, Clone, Default)]
pub struct LayerSignals {
    /// Corpus pattern match (Layer 1)
    pub corpus_match: bool,
    /// Taint flow confirmed (Layer 2)
    pub taint_flow: bool,
    /// Taint branch ratio - high means function actually branches on input (Layer 3)
    pub taint_branch_ratio: Option<f64>,
    /// Near-duplicate inconsistency detected (Layer 4)
    pub near_duplicate: bool,
    /// Function name suggests a validator/sanitizer (from `TaintMetrics`).
    /// Guards the L3 suppression so ordinary vulnerabilities that branch on
    /// input aren't crushed.
    pub has_validation_name: bool,
}

/// Compose confidence from multiple layer signals.
///
/// `config` carries the tunable thresholds and factors for each layer (see
/// `CompositionConfig` for defaults). The engine forwards its own configuration
/// fields here so composition behaviour is fully configurable.
#[must_use]
pub fn compose_confidence(
    signals: &LayerSignals,
    base_score: f64,
    config: &CompositionConfig,
) -> f64 {
    let mut score = base_score;

    // L2 confirms L1: tainted data actually reaches a sink this function's shape implies
    if signals.corpus_match && signals.taint_flow {
        // Full corroboration - no penalty
    } else if signals.corpus_match && !signals.taint_flow {
        // Structural match with no confirmed dataflow - down-weight
        score *= config.taint_unconfirmed_penalty;
    }

    // L3 can SUPPRESS L1, but only for genuine validators. A high branch ratio
    // on input alone is NOT evidence of a validator — real vulnerabilities
    // (e.g. an IDOR handler that checks `user.role` then still passes
    // `req.params.id` to a DB query) branch on tainted input all the time.
    // Require both a very high ratio (>HIGH_BRANCH_RATIO_THRESHOLD) AND a
    // validator-name before suppressing, and log the decision so it is auditable.
    if let Some(ratio) = signals.taint_branch_ratio
        && ratio > config.high_branch_ratio_threshold
        && signals.has_validation_name
    {
        tracing::debug!(
            ratio,
            has_validation_name = signals.has_validation_name,
            base = base_score,
            suppressed_to = score * config.high_branch_ratio_suppression_factor,
            "composition: suppressing high-branch-ratio validator"
        );
        score *= config.high_branch_ratio_suppression_factor;
    }

    // L4 inconsistency can boost confidence using the configured rate and ceiling
    if signals.near_duplicate {
        let boosted = score * (1.0 + config.boost_rate);
        // Cap the absolute lift to boost_max (prevents runaway boosting on many duplicates)
        score = boosted.min(score + config.boost_max);
    }

    score.min(1.0)
}

/// Collect layer signals for a given advisory from all advisories on the same function.
#[must_use]
pub fn collect_signals(advisory: &Advisory, all_advisories: &[Advisory]) -> LayerSignals {
    let key = (advisory.file_id.0, advisory.line);
    let mut signals = LayerSignals::default();

    for adv in all_advisories {
        if (adv.file_id.0, adv.line) != key {
            continue;
        }

        // Check which layers fired
        if adv.rule_id.starts_with("CORPUS_") {
            signals.corpus_match = true;
            // A corpus finding with taint verification is equivalent to taint_flow
            if adv.tags.iter().any(|t| t == "taint-verified") {
                signals.taint_flow = true;
            }
            // Also accept has_taint_path from match_evidence as structural taint signal
            if !signals.taint_flow {
                if let Some(ref ev) = adv.match_evidence {
                    if ev.has_taint_path {
                        signals.taint_flow = true;
                    }
                }
            }
        }
        if adv.rule_id.starts_with("TAINT_") || adv.rule_id == "CROSS_FILE_TAINT" {
            signals.taint_flow = true;
        }
        if adv.rule_id == "NEAR_DUPLICATE_FUNCTION" {
            signals.near_duplicate = true;
        }
        // Use actual taint_branch_ratio / validation name from TaintMetrics if available
        if adv.taint_branch_ratio.is_some() {
            signals.taint_branch_ratio = adv.taint_branch_ratio;
        }
        if let Some(v) = adv.has_validation_name {
            signals.has_validation_name = v;
        }
    }

    signals
}

/// Apply real composition to advisories, replacing the coincidence counter.
///
/// `config` bundles every composition knob (L2 penalty, L3 suppression
/// threshold/factor, L4 boost rate/ceiling) and is forwarded from the engine's
/// configuration fields. See `CompositionConfig` for the defaults.
///
/// # Panics
/// May panic if internal assertions fail.
pub fn apply_composition(advisories: &mut [Advisory], config: &CompositionConfig) {
    // Clone advisories to read from while mutating
    let original: Vec<Advisory> = advisories.to_vec();

    for adv in advisories.iter_mut() {
        let signals = collect_signals(adv, &original);
        adv.confidence = compose_confidence(&signals, adv.confidence, config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_confidence_corpus_only() {
        let signals = LayerSignals {
            corpus_match: true,
            taint_flow: false,
            taint_branch_ratio: None,
            near_duplicate: false,
            has_validation_name: false,
        };
        let result = compose_confidence(&signals, 0.8, &CompositionConfig::default());
        // Corpus alone gets down-weighted to 0.6 → 0.48
        assert!((result - 0.48).abs() < 0.01);
    }

    #[test]
    fn test_compose_confidence_corpus_and_taint() {
        let signals = LayerSignals {
            corpus_match: true,
            taint_flow: true,
            taint_branch_ratio: None,
            near_duplicate: false,
            has_validation_name: false,
        };
        let result = compose_confidence(&signals, 0.8, &CompositionConfig::default());
        // Full corroboration - no penalty
        assert!((result - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_compose_confidence_high_branch_ratio_below_0_85_not_suppressed() {
        // Old threshold was 0.6 — a ratio of 0.8 would have crushed the score to 0.24.
        // New behavior: threshold raised to >0.85, so 0.8 no longer suppresses.
        let mut signals = LayerSignals {
            corpus_match: true,
            taint_flow: true,
            taint_branch_ratio: Some(0.8),
            near_duplicate: false,
            has_validation_name: true,
        };
        let result = compose_confidence(&signals, 0.8, &CompositionConfig::default());
        assert!((result - 0.8).abs() < 0.01, "0.8 ratio must not suppress, got {result}");

        signals.taint_branch_ratio = Some(0.6);
        let result = compose_confidence(&signals, 0.8, &CompositionConfig::default());
        assert!((result - 0.8).abs() < 0.01, "0.6 ratio must not suppress, got {result}");
    }

    #[test]
    fn test_compose_confidence_suppresses_only_validator_at_high_ratio() {
        // Validator name + ratio above 0.85 → genuine validator → suppress.
        let validator = LayerSignals {
            corpus_match: true,
            taint_flow: true,
            taint_branch_ratio: Some(0.95),
            near_duplicate: false,
            has_validation_name: true,
        };
        let result = compose_confidence(&validator, 0.8, &CompositionConfig::default());
        assert!((result - 0.24).abs() < 0.01, "validator suppressed, got {result}");

        // IDOR-style: branches on input but is NOT a validator (no name).
        // Same high ratio must NOT suppress, otherwise a real finding is buried.
        let idor = LayerSignals {
            has_validation_name: false,
            ..validator.clone()
        };
        let result = compose_confidence(&idor, 0.8, &CompositionConfig::default());
        assert!(
            (result - 0.8).abs() < 0.01,
            "non-validator high-ratio finding must NOT be suppressed, got {result}"
        );
    }

    #[test]
    fn test_collect_signals_uses_actual_taint_branch_ratio() {
        use crate::FileId;
        let mut adv = Advisory::bare(
            "TAINT_INPUT_TO_EXEC",
            crate::Severity::Critical,
            FileId(0),
            std::path::Path::new("test.rs"),
            "test",
        );
        adv.taint_branch_ratio = Some(0.4);
        adv.has_validation_name = Some(true);
        let all_advisories = vec![adv.clone()];
        let signals = collect_signals(&adv, &all_advisories);
        assert_eq!(signals.taint_branch_ratio, Some(0.4));
        assert!(signals.has_validation_name);
    }
}
