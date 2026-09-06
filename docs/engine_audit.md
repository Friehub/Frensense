# Frensense Engine: Production Audit

## Bad Fallbacks

### F1 — `weighted_jaccard` returns `0.5` when both maps are empty
**File:** `scorer.rs:23-24`
```rust
if a.is_empty() && b.is_empty() {
    return 0.5;
}
```
**Problem:** When neither fingerprint has IDF-weighted n-gram hashes (e.g. a
corpus pattern built before IDF weighting was introduced), both maps are empty
and the function returns `0.5` — a neutral positive score. This fabricates
similarity where there is none. The downstream weighted sum treats it as a
real signal and inflates the final score. The correct value is `0.0` (no
shared data → no similarity).

**Fix:** Return `0.0`.

---

### F2 — `type_usage_overlap` returns `0.5` when both sets are empty
**File:** `scorer.rs:667-669`
```rust
if a.type_usages.is_empty() && b.type_usages.is_empty() {
    return 0.5;
}
```
**Problem:** Same as F1. Two functions with no type annotations are given a
free 0.5 similarity contribution, weighted by `w[4] = 0.03` in
`DEFAULT_WEIGHTS`. Small but consistent — it shifts every pair's score upward
by `0.015`, pushing borderline false positives over the threshold.

**Fix:** Return `0.0`.

---

### F3 — `compute_final` profile boost defaults to `0.5`
**File:** `scorer.rs:131-136`
```rust
let profile_boost = profiles
    .and_then(|p| p.get(key).copied())
    .unwrap_or(0.5);
```
**Problem:** `compute_final` is the *rule-based* scorer path (not the corpus
path). When no profile exists for a pattern kind, the boost defaults to 0.5
instead of a neutral 1.0 multiplier or 0.0 additive. It then contributes `0.5
× 0.3 = 0.15` to the final score unconditionally, artificially inflating every
rule-match when no profile data is loaded. This function is called with
`profiles = None` in most real scan paths.

**Fix:** Default to `0.0` so missing profile data contributes nothing.

---

### F4 — `FileId` silently wraps to `u32::MAX` on overflow
**File:** `lib.rs:179`
```rust
let file_id = FileId(u32::try_from(idx).unwrap_or(u32::MAX));
```
**Problem:** If a project has more than ~4 billion files (impossible in
practice) this silently wraps. More realistically, `u32::MAX` as a sentinel
collides with a real file ID and corrupts symbol graph edges. Should be a hard
error since `idx` is a `usize` enumerator — it will never exceed `u32::MAX` on
any realistic system, so the `try_from` will never fail. The `unwrap_or` gives
a false sense that this is handled.

**Fix:** Use `u32::try_from(idx).expect("file count exceeds u32 limit")` — the
panic communicates intent. In production, replace with a proper `Result`
return.

---

### F5 — `loader.rs` silently skips patterns with parse errors
**File:** `corpus/loader.rs:58`
```rust
eprintln!("Corpus warning: skipping ...");
```
**Problem:** When a corpus TOML fails to parse, the pattern is silently
skipped with a warning to stderr. In a production scan pipeline, stderr is
often redirected or suppressed. A missing pattern is a silent coverage gap —
the engine reports no findings for that vulnerability class with no indication
that coverage was reduced. This is a data-integrity failure that should be
surfaced as a hard error or at minimum a structured diagnostic in the output.

**Fix:** Accumulate load errors and surface them in the `PatternRegistry`
return type so callers can decide to abort or warn.

---

### F6 — Leftover `eprintln!("[DEBUG]")` in production semantic filter
**File:** `corpus/semantic.rs:228`, `corpus/semantic.rs:568`
```rust
eprintln!("DEBUG: semantic rejected, missing taint flow {:?}", req_flow);
eprintln!("[DEBUG] Learned data flows: {:?}", c.required_taint_flows);
```
**Problem:** These are diagnostic traces written unconditionally to stderr
during every scan. In production this floods the output of any tool calling
Frensense, pollutes CI logs, and in tight scan loops adds string formatting
overhead on every filtered candidate. The `required_taint_flows` debug line
fires during corpus load time, not just scan time, making it even noisier.

**Fix:** Remove both. Diagnostic tracing should go through a proper
feature-gated tracing crate (`tracing` with `#[cfg(debug_assertions)]`), not
naked `eprintln!`.

---

### F7 — `semantic.rs` `required_taint_flows` silently rejects all if flows not extracted
**File:** `corpus/semantic.rs:235-237`
```rust
} else {
    return false;
}
```
**Problem:** If `extracted_flows` is `None` (which happens when the caller
does not pass an AST node), the filter rejects the candidate silently. The
call site in `registry.rs` only extracts flows when `!filter.required_taint_flows.is_empty()`,
but it only passes `func_node` when scanning with an AST node — which is not
always the case for the bundle-based scan path. This creates a silent FN
(false negative) when `required_taint_flows` is set but `func_node` is absent.

**Fix:** When `extracted_flows` is `None` but `required_taint_flows` is set,
pass through rather than reject. The flow check is a precision hint, not a
correctness gate — if we can't extract flows, we should let scoring decide.

---

## Dead / Unwired Code

### D1 — `graph.rs` and `temporal.rs` are compiled but unused in the scan path
**File:** `lib.rs:28,37`, `lib.rs:61`
```rust
pub mod graph;
pub mod temporal;
// ...
pub temporal_events: Vec<graph::TemporalEvent>,
```
`AnalysisResult` stores `temporal_events` and `graph`, but neither field is
read by `scan_function`, `PatternRegistry`, or any scoring code. They are
populated in `analyze_file` (which itself is not called by the CLI scanner —
the CLI calls `load_corpus` + `scan_function` directly on its own parsed
fingerprints). The graph and temporal modules are dead weight in the actual
detection pipeline.

**Action:** Gate behind a `#[cfg(feature = "full-analysis")]` feature flag so
they compile only when actually needed.

---

### D2 — `per_pattern_calibration.rs` trains but its output is never applied in the main scan path
**File:** `per_pattern_calibration.rs`, `corpus/registry.rs`

`train_per_pattern_calibration` trains per-pattern sigmoid parameters, and
`pattern_calibration` is stored on `PatternRegistry`. But `scan_function`
never calls `calibrate()` — it reads the raw `best_score` directly and
compares to threshold. The entire calibration training and storage pipeline is
dead in production.

**Action:** Wire the calibration into `scan_function` after scoring, or remove
the module.

---

### D3 — `profile.rs` `ProjectProfile::learn` is computed but never used for scoring
**File:** `lib.rs:185-189`, `scorer.rs:131-136`

`ProjectProfile` is learned from all fingerprints and stored in
`ProjectAnalysis.profile`. `compute_final` accepts it via `profiles:
Option<&HashMap<String, f64>>`, but `compute_final` is the rule-based path —
not the corpus path. The corpus path (`score_against_corpus`) never takes a
profile. The profile boost contributes 0.3 weight to an unused code path, and
the 0.5 fallback (F3) applies unconditionally in tests that do use it.

**Action:** Either wire `ProjectProfile` into the corpus scoring path or
remove the `profile_boost` from `compute_final` entirely.

---

### D4 — `semantic_patterns/` module is declared but call path is unclear
**File:** `lib.rs:35`
```rust
pub mod semantic_patterns;
```
Grep shows no call sites using `semantic_patterns::` in the corpus scan path.

**Action:** Verify whether this feeds into the scan pipeline or is a legacy
module. If legacy, gate behind a feature flag.

---

### D5 — `auto_filter.rs` `contains_import` map is always empty
After removing the category-level loop (this session), `contains_import` is
initialized but never populated — the per-pattern loop only populates
`contains_call_to`. The field still exists in `AutoFilterStats` and is merged
in `merge_filters`, doing nothing but allocating an empty `HashMap`.

**Action:** Remove the `contains_import` field from `AutoFilterStats` and the
corresponding merge path, or document that it is reserved for future
hand-authored filter use.

---

### D6 — `graph.rs` builds a `SemanticGraph` inside `symbols.rs` but it is just cloned out
**File:** `lib.rs:144`
```rust
let graph = symbols.graph().clone();
```
The graph is cloned from the symbol registry and stored in `AnalysisResult`.
But the symbol registry *keeps its own copy*, so there are now two live copies
of every graph. Since neither is used downstream, both are waste.

---

## Summary Table

| ID | Location | Severity | Type | Status |
|----|---|---|---|---|
| F1 | `scorer.rs:23` | High | Bad fallback (inflates score) | Fix below |
| F2 | `scorer.rs:668` | Medium | Bad fallback (inflates score) | Fix below |
| F3 | `scorer.rs:136` | Medium | Bad fallback (inflates score) | Fix below |
| F4 | `lib.rs:179` | Low | Silent overflow sentinel | Fix below |
| F5 | `loader.rs:58` | High | Silent coverage gap | Fix below |
| F6 | `semantic.rs:228,568` | Medium | Debug output in production | Fix below |
| F7 | `semantic.rs:235` | Medium | Silent FN on missing AST node | Fix below |
| D1 | `graph.rs`, `temporal.rs` | Medium | Dead modules in scan path | Needs feature flag |
| D2 | `per_pattern_calibration.rs` | High | Trained but never applied | Needs wiring |
| D3 | `profile.rs` | Low | Computed but scoring unused | Needs wiring or removal |
| D4 | `semantic_patterns/` | Low | Call path unclear | Needs verification |
| D5 | `auto_filter.rs` | Low | Empty map allocated every run | Remove field |
| D6 | `lib.rs:144` | Low | Duplicate graph clone | Remove clone |
