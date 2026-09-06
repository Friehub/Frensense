# Match Evidence — Explainability System

Every corpus finding includes a `MatchEvidence` object that explains **why** the
match scored the way it did. This is the equivalent of a compiler telling you
which variable has a type error — each dimension shows its contribution.

## Structure (JSON output)

```json
{
  "rule_id": "CORPUS_TS_OPEN_REDIRECT",
  "confidence": 0.819,
  "match_evidence": {
    "ngram_sim": 0.45,
    "ast_sim": 0.86,
    "signature_sim": 0.32,
    "control_flow_sim": 1.0,
    "api_sim": 0.28,
    "motif_sim": 1.0,
    "flow_sim": null,
    "semantic_sim": 0.71,
    "negative_sim": 0.42,
    "matched_calls": ["res.redirect"],
    "missing_calls": ["security.isRedirectAllowed", "challengeUtils.solveIf"],
    "matched_motifs": ["HttpResponseSink"],
    "has_taint_path": true,
    "best_positive_index": 0
  }
}
```

## What Each Field Means

### Similarity Dimensions (all [0, 1])

| Field | What it measures | High = | Low = |
|-------|-----------------|--------|-------|
| `ngram_sim` | Variable/function name tokens match | Same variable names | Renamed variables |
| `ast_sim` | AST skeleton structure | Same code structure | Different structure |
| `signature_sim` | Parameter names + types | Same function signature | Different params |
| `control_flow_sim` | Branch/loop/return sequences | Same control flow | Different flow |
| `api_sim` | Literal API call overlap | Calls same APIs | Calls different APIs |
| `motif_sim` | Sink/source motif family | Same vulnerability class | Different class |
| `flow_sim` | Data-flow source→sink path | `null` if no taint path found | — |
| `semantic_sim` | Semantic category markers | Same bug category | Different category |
| `negative_sim` | Similarity to FIXED examples | **Too high = likely FP** | Low = distinct from safe |

### Diagnostic Lists

| Field | Example | Purpose |
|-------|---------|---------|
| `matched_calls` | `["res.redirect"]` | These pattern calls were FOUND in your function |
| `missing_calls` | `["security.isRedirectAllowed"]` | These pattern calls were NOT found (negative evidence) |
| `matched_motifs` | `["HttpResponseSink"]` | Motif families detected |
| `has_taint_path` | `true` | User input → sink chain confirmed |

## How to Read a Finding

```
[WARNING] CORPUS_TS_OPEN_REDIRECT  conf=0.819  routes/auth.ts:47
  Matched:
    v res.redirect()              ← matched_calls
    v HttpResponseSink motif      ← matched_motifs
    v user-input -> sink taint path ← has_taint_path (data flow verified)
    v control flow (100% match)   ← control_flow_sim
    v AST structure (86% match)   ← ast_sim
  Differed:
    x 42% similar to safe (negative) example  ← negative_sim (acceptable, <50%)
```

**If a finding has high `negative_sim` (>0.5)**, the function is structurally
similar to a FIXED (safe) version. This is suspicious — the function likely
has guard/validation code that the vulnerable pattern doesn't. Lower the
threshold or add a more specific corpus negative.

**If a finding has `flow_sim: null`**, the pattern expects a taint chain
(user input → sink) but none was found. The match is purely structural.
This is acceptable for non-taint patterns (security headers, crypto checks)
but suspicious for injection/SSRF patterns.

**If `matched_calls` is empty** but the finding still fired, the match is
purely structural (AST shape + motif). This is the weakest form of match
and often indicates a false positive.

## CLI Reporter

The evidence block is displayed automatically for all corpus findings.
Run with `--json` to get machine-readable evidence, or default format
for human-readable output.

## Evidence Computation

`PatternScorer::compute_evidence()` in `scorer.rs` mirrors the scoring
pipeline but returns each dimension individually instead of a weighted sum.
The evidence is populated in `registry.rs:478` when a match exceeds threshold.

```rust
let evidence = PatternScorer::compute_evidence(
    &weighted_fp,
    &pattern.positives,
    &pattern.negatives,
    pat_weights,
);
```

No post-hoc analysis. No attention weights. No gradients. The evidence is
a **byproduct of the scoring computation** — every dimension was already
computed to produce the final score.
