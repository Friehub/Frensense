# Scoring Dimensions & Calibration

## The 12 Similarity Dimensions

Every function pair (candidate vs pattern positive/negative) is compared across
12 independent dimensions. The `RawDimensions` struct holds Jaccard-similarity
values for each dimension, computed from fingerprint hashes.

| Index | Dimension | Fingerprint field | What it measures |
|-------|-----------|-------------------|------------------|
| 0 | `ngram_sim` | `ngram_hashes` / `weighted_ngram_hashes` | Token n-gram overlap |
| 1 | `ast_sim` | `skeleton_hashes` or `structural_markers` | AST skeleton (tree-edit distance complement) |
| 2 | `signature_sim` | `signature_ngrams` | Parameter names + types |
| 3 | `param_type_sim` | `param_type_ngrams` | Type annotation overlap |
| 4 | `type_usage_sim` | `type_usages` | Type references in body |
| 5 | `semantic_sim` | `semantic_markers` | API category overlap |
| 6 | `cf_sim` | `control_flow_hashes` | Control flow structure (branch/loop/return) |
| 7 | `api_sim` | `api_calls` | Literal API call overlap |
| 8 | `tainted_api_sim` | `tainted_api_calls` | Tainted (user-param) call overlap |
| 9 | `motif_sim` | `motif_hashes` | Motif group (e.g. exec↔spawn) |
| 10 | `flow_sim` | `data_flow_path_hashes` | Data-flow path (source→sink) |
| 11 | `config_sim` | `config_literal_hashes` | Config literal values (false/true) |

**Important:** `tainted_api_sim` returns 0.0 when `target.tainted_api_calls` is
empty (no tainted calls → no similarity, not "unknown"). The old default of 1.0
added a false 0.17 boost to every score. Similarly, `jaccard_similarity_sorted`
returns 0.0 when both inputs are empty (was 1.0). These fixes eliminated
~44% of false positives in the NodeGoat benchmark.

## Per-Dimension Signal

For each dimension `d`, the **signal** is how much more the candidate resembles
the buggy positive than the fixed negative:

```
signal[d] = max(0.0, pos_sim[d] − neg_sim[d])
```

Signal is in [0, 1]. A value of 0 means the dimension does not distinguish
buggy from fixed code for this candidate. 1.0 means perfect separation.

For the `api_sim` dimension specifically, signal uses an intersection-size
difference (how many MORE API calls the candidate shares with the positive
than with the negative), normalized by the positive intersection count:

```
signal_api = max(0, intersect(candidate, pos) − intersect(candidate, neg))
              ────────────────────────────────────────────────────────
              max(intersect(candidate, pos), 1)
```

This amplifies the distinguishing call (e.g. `eval` vs `parseInt`) instead of
diluting it across shared calls (`res.render`, `next`, `isNaN`).

## Noise Gate

A single dimensional coincidence (e.g. `api_sim = 0.45` by chance) should
not produce a match. The noise gate rejects candidates unless:

```
max(signal) > 0.4          # one strong dimension
  OR
count(signal[d] > 0.15) ≥ 2  # ≥2 moderate dimensions
```

This prevents structurally coincidental matches from patterns whose positives
and negatives are not discriminative for the specific candidate.

## Weighted Sum (The Final Score)

The final score is the **11-dimension weighted sum** of the best-matching
positive's `RawDimensions`, using per-category learned weights (or fallback
`DEFAULT_WEIGHTS`). This is the SAME type of score that the Platt scaling
calibration was trained on.

```rust
weighted_score = Σ(weights[d] × raw_dimensions[d])  // d in 0..11
```

DEFAULT_WEIGHTS (fallback when fewer than 20 training pairs exist):
```
ngram=0.10  ast=0.20  signature=0.08  param_type=0.04  type_usage=0.03
semantic=0.10  cf=0.08  api=0.06  tainted_api=0.12  motif=0.06  flow=0.10
```

## Calibration (Platt Scaling)

Per-pattern sigmoid parameters (A, B) are trained at bundle-build time by
`train_per_pattern_calibration` in `per_pattern_calibration.rs`. For each
pattern, 80% of positives are held out, scored against the remaining 20%,
and a sigmoid is fit via gradient descent:

```
P(true_positive | score) = 1 / (1 + exp(−(A × score + B)))
```

The calibration is applied in `scan_function` (registry.rs) after scoring
and before the threshold comparison:

```rust
let calibrated = calibrate(raw_score, self.pattern_calibration.get(&pattern.id));
// calibrate returns raw_score unchanged if no params exist
```

**Critical:** The calibration is trained on WEIGHTED SUMS (range ~0.3–0.6).
The runtime score MUST be the same weighted sum, NOT `max_signal` or a
blended formula. Using `max_signal` (which can be 0.8–1.0) causes the sigmoid
to extrapolate outside its training range, squashing everything to ~0.999
and producing guaranteed false positives.

## Scoring Pipeline (ASCII Flow)

```
candidate fingerprint
        │
        ▼
raw_dimensions(candidate, positive[d])  for each positive[d]
raw_dimensions(candidate, negative[d])  for each negative[d]
        │
        ▼
Select best positive (highest weighted_score with semantic/cross-lingual multipliers)
        │
        ▼
Compute per-dimension worst_negative (max across all negatives per dimension)
        │
        ▼
signal[d] = max(0, pos_sim[d] − neg_sim[d])
        │
        ▼
Noise gate: max(signal) > 0.4 OR ≥2 dims with signal > 0.15?
   NO  → score = 0.0
   YES →
        │
        ▼
weighted_score = Σ(weights[d] × best_positive.raw_dimensions[d])
        │
        ▼
calibrated = 1 / (1 + exp(−(A × weighted_score + B)))  // per-pattern sigmoid
        │
        ▼
calibrated ≥ threshold?  →  PatternMatch { score: calibrated }
```
