// SPDX-License-Identifier: MIT

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
            score_suppression_floor: 0.50,
        }
    }
}
