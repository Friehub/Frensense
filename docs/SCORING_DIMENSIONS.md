# Scoring Dimensions & Default Weights

## The 11 Similarity Dimensions

Every function pair (candidate vs pattern positive/negative) is compared across
11 dimensions. The weighted sum produces a similarity score in [0, 1].

| Index | Dimension | Value | What it measures |
|-------|-----------|-------|------------------|
| 0 | `ngram_sim` | `[0, 1]` | Variable/function name tokens match (bag-of-words) |
| 1 | `ast_sim` | `[0, 1]` | AST skeleton structure (tree edit distance complement) |
| 2 | `signature_sim` | `[0, 1]` | Function signature n-grams (parameter names + types) |
| 3 | `param_type_sim` | `[0, 1]` | Parameter type annotations overlap |
| 4 | `type_usage_sim` | `[0, 1]` | Type references overlap (e.g. both use `Request`, `Response`) |
| 5 | `semantic_sim` | `[0, 1]` | Semantic marker overlap (API categories, abstract kinds) |
| 6 | `cf_sim` | `[0, 1]` | Control flow structure matched (branch/loop/return sequences) |
| 7 | `api_sim` | `[0, 1]` | Literal API call overlap (e.g. `exec` matches `exec`) |
| 8 | `tainted_api_sim` | `[0, 1]` | API calls with user-parameter arguments overlap |
| 9 | `motif_sim` | `[0, 1]` | Motif group overlap (e.g. `exec` ↔ `spawn` ↔ `Command::new`) |
| 10 | **`flow_sim`** | `[0, 1]` | **Data-flow path overlap (source→sink chains)** |

## Default Weights (Fallback)

When per-category learned weights are unavailable (<20 training examples), the
engine falls back to `DEFAULT_WEIGHTS` in `weight_learner.rs:25`:

```rust
pub(crate) const DEFAULT_WEIGHTS: FeatureVec =
    [0.12, 0.20, 0.08, 0.04, 0.03, 0.12, 0.10, 0.10, 0.15, 0.04, 0.02];
```

| Dim | Weight | % of total |
|-----|--------|-----------|
| ngram_sim | 0.12 | 12% |
| ast_sim | **0.20** | **20%** — largest |
| signature_sim | 0.08 | 8% |
| param_type_sim | 0.04 | 4% |
| type_usage_sim | 0.03 | 3% |
| semantic_sim | 0.12 | 12% |
| cf_sim | 0.10 | 10% |
| api_sim | 0.10 | 10% |
| tainted_api_sim | **0.15** | **15%** |
| motif_sim | 0.04 | 4% |
| **flow_sim** | **0.02** | **2% — underweighted** |

## The flow_sim Problem (Critical for Generalization)

`flow_sim` is the most GENERALIZABLE dimension but has the LOWEST weight (0.02).

### What flow_sim measures

`data_flow_path_hashes` captures abstract source→sink chains like:
```
UserInputSource → taint_flow → CommandExecutionSink
UserInputSource → taint_flow → HttpOutboundSink
UserInputSource → taint_flow → SqlSink
```

These paths are **invariant to**:
- Variable renaming (`cmd` → `userInput` → `command`)
- API version (`exec` vs `spawn` vs `Command::new`)
- Helper extraction (inline vs helper function)
- Control flow (if/else vs try/catch)
- Async/await vs callbacks

### Why it's underweighted

The weight (0.02) was set conservatively because `data_flow_path_hashes` is empty
for patterns without explicit taint sources (e.g. security header checks, crypto).
If `flow_sim` were higher, patterns without flow paths would score lower across
the board — even when they're valid (non-taint) patterns.

### The fix

Raise `flow_sim` to **0.10–0.12** and correspondingly reduce `api_sim` and
`ngram_sim`. This shifts reliance from literal API matching (requires exact
`exec` call) to data-flow structure (any user-input-to-sink chain).

**Before**: `api_sim=0.10` → `request(url)` matches, `http.get(url)` doesn't
**After**: `flow_sim=0.10` → both produce same `UserInputSource → HttpOutboundSink` path → match

This eliminates the need for M1–M15 mutation variants. One positive per
vulnerability class suffices.

### Other dimensions that need tuning

| Dimension | Current | Issue | Suggested |
|-----------|---------|-------|-----------|
| `ngram_sim` | 0.12 | Too high — penalizes variable renames | 0.08 |
| `ast_sim` | 0.20 | Good — structural match is reliable | Keep |
| `api_sim` | 0.10 | Too high — locks to specific APIs | 0.06 |
| `flow_sim` | **0.02** | Way too low — prevents generalization | **0.10** |

## Per-Category Learned Weights

For categories with ≥20 patterns (48 categories currently), the weight learner
trains custom weights via logistic regression. These override DEFAULT_WEIGHTS.

The learner (`weight_learner.rs:94-125`) uses balanced gradient descent:
- Equal weight per-class (positives count as 1/2N each, negatives as 1/2N each)
- 200 iterations with learning rate 0.1
- L1-normalized so weights sum to 1.0

## How Scoring Works (Complete Flow)

```
Candidate function  +  Pattern positive  +  Pattern negative
         │                      │                    │
         ▼                      ▼                    ▼
    11-dim raw_dimensions    raw_dimensions     raw_dimensions
         │                      │                    │
         ▼                      ▼                    ▼
    weighted_score(weights)  weighted_score      weighted_score
         │                      │                    │
         ▼                      ▼                    ▼
    × cross_lingual_penalty  × cross_lingual    × cross_lingual
    × semantic_multiplier    × semantic_mult    × semantic_mult
         │                      │                    │
         ▼                      ▼                    ▼
    best_pos_score      (max across positives)   max_neg_sim
         │                                         │
         └──────────────┬──────────────────────────┘
                        ▼
            neg_penalty = if max_neg_sim ≥ best_pos_score
                          then (1 - max_neg_sim).max(0.1)
                          else 1 - (max_neg_sim × 0.3)
                        ▼
            final_score = best_pos_score × neg_penalty × context_multiplier
```

Where:
- `cross_lingual_penalty` = 1.0 if same language (TS↔JS = same), else 0.20
- `semantic_multiplier` = 0.30 if semantic markers don't overlap, 2.0 if they do
- `context_multiplier` = 0.5 if route handler vs utility mismatch
