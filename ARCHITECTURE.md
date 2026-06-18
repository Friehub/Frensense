# Frensense: Architecture — Accurate State (2026-06-17)

> Written from direct code reads of `frensense-engine/src/pattern/scorer.rs`,
> `frensense-engine/src/corpus/registry.rs`, `src/engine/project/runner.rs`,
> `src/reporter.rs`, and `frensense-engine/src/fingerprint.rs`.
> Every claim in this document is grounded in a specific file and line number.

---

## What the Engine Actually Does

### Fingerprinting — 7 Dimensions Per Function

`frensense-engine/src/fingerprint.rs` — `extract_fingerprints()`

For every function the scanner finds, it extracts:

| Field | What it captures |
|---|---|
| `ngram_hashes` | Positional n-gram hashes of body tokens (position-encoded: `return` at line 5 ≠ line 50) |
| `weighted_ngram_hashes` | Same hashes with IDF weights — rare tokens score higher than `let x =` |
| `signature_ngrams` | N-grams of the function signature text (parameter names + types) |
| `param_type_ngrams` | N-grams of parameter type names only |
| `name_segments` | camelCase/snake_case split function name (`validateUser` → `["validate", "User"]`) |
| `structural_markers` | Abstract AST node kind hashes (language-normalized) |
| `type_usages` | All type identifier names used in the body |
| `comment_density` | Fraction of bytes that are comments |

### Corpus Scoring — 5-Dimensional Contrastive Score

`frensense-engine/src/pattern/scorer.rs` — `score_against_corpus()` (line 130)

The scorer computes similarity to both the positive and negative example across 5 dimensions:

```
sim_to_positive =
    weighted_jaccard(ngram_hashes)          × 0.35
  + jaccard(structural_markers)             × 0.30
  + jaccard(signature_ngrams)              × 0.20
  + jaccard(param_type_ngrams)             × 0.10
  + type_usage_overlap()                   × 0.05

sim_to_negative = same formula against the negative example

final_score = sim_to_positive × (1.0 - sim_to_negative) × cross_lingual_penalty
```

The `(1.0 - sim_to_negative)` term is the contrastive signal: a function that looks
like both the positive and negative scores low. A function that looks like the positive
and unlike the negative scores high. This is already built.

Cross-lingual penalty: 25% reduction when pattern language ≠ candidate language (M8).

### Candidate Retrieval — MinHash LSH

`frensense-engine/src/minhash.rs` — `LSHIndex` with 16 bands × 8 rows

At scan time, the scanned function's 128-hash MinHash signature is compared against the LSH
index. Only candidate patterns that share at least one band bucket are scored. This avoids
comparing against all 89+ patterns exhaustively and scales to thousands of patterns without
linear slowdown.

### Output — How a Corpus Match Becomes a Finding

`src/engine/project/runner.rs` — lines 583–608

When `registry.scan_function(fp)` returns a `PatternMatch`, the runner builds an `Advisory`:

```rust
Advisory {
    rule_id: format!("CORPUS_{}", m.pattern_id.to_uppercase()),
    observation: format!(
        "Function '{}' matches corpus pattern '{}' (score: {:.2})",
        fp.function_name, m.pattern_id, m.score
    ),
    impact: "Function shape matches a known violation pattern.",
    improvement: "Review against corpus example.",
    ...
}
```

**This is the confirmed gap.** The observation, impact, and improvement are hardcoded generic
strings. The pattern ID is in the output but the actual advisory text explaining what the
pattern catches, why it matters, and how to fix it is not stored anywhere — not in the FRC
bundle, not in a sidecar file, not in the corpus metadata.

Compare this to taint findings (runner.rs line 172–173):
```rust
impact: rule.impact.clone(),
improvement: rule.improvement.clone(),
```
Taint findings use the advisory text from `taint_rules.toml`. Corpus findings do not.

---

## The Two Real Gaps (Confirmed From Code)

### Gap 1: Corpus Findings Have No Pattern-Specific Advisory Text

**Location:** `src/engine/project/runner.rs` lines 590–596

Every corpus match produces the same three strings:
- `"Function shape matches a known violation pattern."`
- `"Review against corpus example."`

These tell the developer nothing about what the pattern is, why it's dangerous, or how to
fix it. A developer seeing `CORPUS_TS_SEC_SQL_INJECTION_3` with no explanation has to go
find the corpus file themselves to understand what fired.

**The fix:** Add a sidecar advisory TOML per corpus pattern. At bundle build time, embed the
advisory. At scan time, use it.

Sidecar file: `corpus/targets/ts_sec_sql_injection_3.advisory.toml`
```toml
severity = "critical"
cwe = "CWE-89"
category = "sec"
observation = "Template literal or string concatenation used to build a SQL query with user-controlled input."
impact = "Attacker can alter query logic, bypass authentication, or exfiltrate data."
improvement = "Use parameterized queries: db.query('SELECT * FROM users WHERE id = $1', [userId])"
```

Bundle format change in `frensense-engine/src/corpus/bundle.rs`:
```rust
pub struct BundlePattern {
    pub id: String,
    pub positives: Vec<FunctionFingerprint>,
    pub negatives: Vec<FunctionFingerprint>,
    pub advisory: Option<PatternAdvisory>,  // new
}

pub struct PatternAdvisory {
    pub severity: String,
    pub cwe: Option<String>,
    pub category: String,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
}
```

Runner change: replace hardcoded strings with `pattern.advisory.observation` etc., falling
back to the generic strings if no advisory is present.

**Also fix:** `PatternMatch::positive_similarity` and `PatternMatch::negative_similarity` are
both set to `0.0` in `registry.rs` line 121–122. The scorer computes `sim_to_positive` and
`sim_to_negative` but they are discarded. Populate them before returning so callers and the
reporter can include them in the finding for debugging:

```rust
// registry.rs — inside scan_function, after best_score is computed:
let (best_pos_sim, best_neg_sim) = ... // capture alongside best_score
matches.push(PatternMatch {
    pattern_id: pattern.id.clone(),
    score: best_score,
    positive_similarity: best_pos_sim,
    negative_similarity: best_neg_sim,
});
```

This exposes the breakdown (how positive vs how negative) in the JSON output, which is
useful for threshold tuning.

### Gap 2: Taint Source Seeding Is Regex-Based

**Location:** `src/semantics/data_flow/resolve.rs` — `COMBINED_SOURCE_RE`

The data flow analyzer seeds taint by matching identifier names against a regex built from
all taint rules' `source` patterns. A variable named `url`, `input`, or `data` gets tainted
— not because it actually received user-controlled input, but because its name matches.

This is why `TAINT_INPUT_TO_HTTP` has 520 false positives on axum. Variables named `url`
that are constructed internally (e.g., `let url = format!("https://internal-api/{}", id)`)
match the source regex and taint propagates.

**The fix:** Replace name-based seeding with type-based entry point detection.

For Rust (Axum): seed taint on function parameters whose type contains `Path<`, `Query<`,
`Json<`, `Form<`, or `Bytes`. These types can only hold user-controlled input — that's their
contract in the Axum framework.

For TypeScript (Express/Fastify): seed taint on member accesses `req.params.*`,
`req.query.*`, `req.body.*`, `req.headers.*`. These are the established HTTP input surfaces.

The source regex stays as a fallback for languages/frameworks not yet covered.

---

## What the Scorer Already Has (Correct Summary)

These things are **already working** and do not need to be built:

| Feature | Where | Status |
|---|---|---|
| IDF-weighted n-gram scoring (M1) | `scorer.rs:weighted_jaccard` | Done |
| 5-dimensional scoring (n-grams + structure + signature + params + types) | `scorer.rs:score_against_corpus` | Done |
| Contrastive score via `sim_to_positive × (1.0 - sim_to_negative)` | `scorer.rs` line ~175 | Done |
| Cross-lingual penalty 25% (M8) | `scorer.rs:cross_lingual_penalty` | Done |
| Positional n-gram hashing (M9) | `fingerprint.rs:token_ngrams_positional` | Done |
| MinHash LSH candidate retrieval | `minhash.rs:LSHIndex` | Done |
| Multi-example per pattern (max score across all pairs) | `registry.rs:scan_function` | Done |
| Bundle embed + versioning (S3, S4) | `corpus/bundle.rs` | Done |
| Configurable per-engine threshold | `PatternRegistry::new(threshold)` | Done |

---

## Build Order (Only the Real Remaining Items)

```
Item 1: Advisory TOML sidecar + bundle embedding
  Files to change:
    frensense-engine/src/corpus/bundle.rs        — add PatternAdvisory to BundlePattern
    src/bin/build-corpus-bundle.rs               — read .advisory.toml during build
    src/engine/project/runner.rs lines 590–596   — use advisory text in Advisory struct

  Files to create (89 advisory TOMLs):
    corpus/targets/{pattern_name}.advisory.toml  — one per existing pattern

  Output: corpus findings show the same quality of advisory text as taint findings.

Item 2: Populate positive_similarity / negative_similarity in PatternMatch
  File: frensense-engine/src/corpus/registry.rs — lines 83–124
  Change: compute and store pos/neg sim breakdown alongside best_score
  Output: JSON findings include breakdown; useful for threshold tuning.

Item 3: AST type-based taint source seeding
  File: src/semantics/data_flow/resolve.rs
  Change: replace/complement COMBINED_SOURCE_RE with framework type detection
  Output: TAINT_INPUT_TO_HTTP FP rate drops from 100% toward baseline.

Item 4: Sanitizer propagation stop
  File: src/semantics/data_flow/resolve.rs
  Change: when sanitizer function (from taint_rules.toml) is called on tainted value,
          stop taint propagation on the return value
  Output: tainted values that pass through validated sanitizers no longer reach sinks.
```

Items 1 and 2 are corpus/reporting work — no risk of breaking existing detection.
Items 3 and 4 are taint engine work — run `compute_metrics.py --by-rule` after each to
verify FP rate direction.
