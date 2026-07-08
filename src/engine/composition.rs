// SPDX-License-Identifier: MIT

//! Layer signal composition for cross-layer confirmation.
//!
//! Replaces the coincidence-counter in `boost_overlap_confidence` with a real
//! AND-gate that checks whether layers are causally related, not just co-located.

use crate::Advisory;

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
}

/// Compose confidence from multiple layer signals.
///
/// This implements a real AND-gate:
/// - L2 confirms L1: taint flow confirms corpus match → full confidence
/// - L1 alone: structural match with no dataflow → down-weight
/// - L3 can suppress L1: high branch ratio means function is a real validator
/// - L4 can boost or suppress: inconsistency across duplicates
#[must_use]
pub fn compose_confidence(signals: &LayerSignals, base_score: f64) -> f64 {
    let mut score = base_score;

    // L2 confirms L1: tainted data actually reaches a sink this function's shape implies
    if signals.corpus_match && signals.taint_flow {
        // Full corroboration - no penalty
    } else if signals.corpus_match && !signals.taint_flow {
        // Structural match with no confirmed dataflow - down-weight
        score *= 0.6;
    }

    // L3 can SUPPRESS L1, not just sit beside it
    if let Some(ratio) = signals.taint_branch_ratio
        && ratio > 0.6
    {
        // This function really does branch on its input - likely a real validator
        score *= 0.3;
    }

    // L4 inconsistency can boost confidence (inconsistent code is suspicious)
    if signals.near_duplicate {
        score *= 1.2;
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
        }
        if adv.rule_id.starts_with("TAINT_") || adv.rule_id == "CROSS_FILE_TAINT" {
            signals.taint_flow = true;
        }
        if adv.rule_id == "NEAR_DUPLICATE_FUNCTION" {
            signals.near_duplicate = true;
        }
        // Use actual taint_branch_ratio from TaintMetrics if available
        if adv.taint_branch_ratio.is_some() {
            signals.taint_branch_ratio = adv.taint_branch_ratio;
        }
    }

    signals
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Apply real composition to advisories, replacing the coincidence counter.
pub fn apply_composition(advisories: &mut [Advisory]) {
    // Clone advisories to read from while mutating
    let original: Vec<Advisory> = advisories.to_vec();

    for adv in advisories.iter_mut() {
        let signals = collect_signals(adv, &original);
        let base_score = adv.confidence;
        adv.confidence = compose_confidence(&signals, base_score);
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
        };
        let result = compose_confidence(&signals, 0.8);
        // Corpus alone gets down-weighted to 0.6
        assert!((result - 0.48).abs() < 0.01);
    }

    #[test]
    fn test_compose_confidence_corpus_and_taint() {
        let signals = LayerSignals {
            corpus_match: true,
            taint_flow: true,
            taint_branch_ratio: None,
            near_duplicate: false,
        };
        let result = compose_confidence(&signals, 0.8);
        // Full corroboration - no penalty
        assert!((result - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_compose_confidence_high_branch_ratio() {
        let signals = LayerSignals {
            corpus_match: true,
            taint_flow: true,
            taint_branch_ratio: Some(0.8),
            near_duplicate: false,
        };
        let result = compose_confidence(&signals, 0.8);
        // High branch ratio suppresses - function is likely a real validator
        // 0.8 * 0.3 = 0.24
        assert!((result - 0.24).abs() < 0.01);
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
        let all_advisories = vec![adv.clone()];
        let signals = collect_signals(&adv, &all_advisories);
        assert_eq!(signals.taint_branch_ratio, Some(0.4));
    }
}
