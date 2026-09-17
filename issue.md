# Scoring Issue: Real Vulnerabilities Score Lower Than Noise

## The Problem

Real NodeGoat vulnerabilities score **0.50** while generic noise patterns score **0.55**.
The scoring layer penalizes genuine findings and boosts false positives.

## Evidence

### Real findings at 0.50 (ground truth matches)
```
A5-HELMET_MISSING     | server.js:40
A5-X_POWERED_BY       | server.js:41
A5-NOSNIFF            | server.js:64
A5-COOKIE_NAME        | server.js:89
A8-CSRF_MIDDLEWARE     | server.js:106
A1-LOG_INJECTION      | session.js:64
A10-SSRF              | research.js:16
A2-WEAK_PW            | user-dao.js:17
A4-IDOR_PARAM         | allocations.js:18
REDOS                 | profile.js:59
... (21 total)
```

### Noise at 0.55 (generic patterns)
```
CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL      | 4 findings
CORPUS_TS_EXPRESS5_ASYNC_ERROR_UNHANDLED | 4 findings
CORPUS_TS_EXPRESS_ASYNC_HANDLER_MISSING  | 4 findings
CORPUS_TS_CJS_ERROR_STACK_LEAK           | 4 findings
CORPUS_TS_SSE_DATA_INJECTION             | 4 findings
... (128 total)
```

## Root Cause: `sem_mult` in scorer.rs

```rust
// Current code (line 453-459)
let sem_mult = if positive.semantic_markers.is_empty() {
    1.0
} else if dim.semantic_sim == 0.0 {
    SEMANTIC_ZERO_PENALTY   // 0.30x ← REAL FINDINGS HIT THIS
} else {
    SEMANTIC_MATCH_BOOST    // 2.0x  ← NOISE HIT THIS
};
```

### Why this is wrong

1. Real findings match via ngram_sim, ast_sim, api_sim — strong structural evidence
2. But they have NO semantic_sim match (the patterns use different semantic markers)
3. So they get penalized with SEMANTIC_ZERO_PENALTY (0.30x multiplier)
4. Noise patterns match semantic_sim and get SEMANTIC_MATCH_BOOST (2.0x multiplier)
5. Result: real findings score LOWER than noise

### The semantic_sim gap

- Real NodeGoat patterns (ts_ns_helmet_missing, etc.) have semantic markers in comments
- But the actual code patterns don't produce semantic_sim matches with the target code
- The semantic_sim is comparing comment-based markers, not code behavior

## Impact on Benchmarks

### NodeGoat (default settings, no min-confidence filter)
- Total: 163 findings
- At 0.50: 21 findings — ALL real ground truth vulnerabilities
- At 0.55: 128 findings — ALL generic noise patterns
- At >=0.80: 12 findings — ALL dependency checks (VULN_NPM_*)

### Juice Shop (same issue)
- Real SQLi findings at 0.50
- Generic SSRF/IDOR noise at 0.55
- Cannot filter without losing real findings

### Benchmark distortion
- Benchmarks run with `--min-confidence 0.0` (show everything)
- Real users run with default or `--confidence medium` (0.60+)
- At 0.60+: only 2 findings (both dependency checks)
- **No usable middle ground** — can't separate real from noise

## Score Distribution

```
NodeGoat:
  0.43:    2 ##
  0.50:   21 #####################   ← REAL VULNS
  0.55:  128 ################################... ← NOISE
  >=0.80:  12 ############           ← DEPENDENCY CHECKS

Juice Shop:
  0.35-0.55:  50 findings
  0.55-0.60: 587 findings             ← NOISE CLUSTER
  >=0.80:      2 findings             ← DEPENDENCY CHECKS
```

## Previous Fix Attempt (commit 6a5b6c1)

Added `has_flow_match` to the sem_mult condition:
```rust
} else if has_flow_match || dim.semantic_sim > 0.0 {
    SEMANTIC_MATCH_BOOST
} else {
    SEMANTIC_ZERO_PENALTY
};
```

**Did not help** — real findings don't have flow_match either.

## What Needs to Happen

1. **Trace similarity computation** for a real finding (e.g., A5-HELMET_MISSING)
   - What is ngram_sim, ast_sim, api_sim, semantic_sim?
   - Why is semantic_sim = 0?
   - What is the weighted_score before sem_mult?

2. **Trace similarity computation** for a noise finding (e.g., CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL)
   - What is semantic_sim > 0?
   - What makes it score higher?

3. **Fix sem_mult logic** to not penalize strong structural matches
   - Option A: Remove SEMANTIC_ZERO_PENALTY entirely
   - Option B: Only apply penalty when all dimensions are weak
   - Option C: Use weighted_score threshold to skip penalty

4. **Re-benchmark** after fix to verify real findings score higher than noise

## Traced Evidence

### Noise finding: CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL (0.55)
```json
{
  "ngram_sim": 0.0,
  "ast_sim": 0.57,
  "signature_sim": 0.0,
  "control_flow_sim": 0.0,
  "api_sim": 0.2,
  "motif_sim": 1.0,        ← MATCHED
  "flow_sim": 1.0,         ← MATCHED (taint path)
  "semantic_sim": 0.25,    ← MATCHED (semantic markers)
  "has_taint_path": true
}
```
**Why it scores 0.55:** flow_sim=1.0 + motif_sim=1.0 + semantic_sim=0.25
→ sem_mult = SEMANTIC_MATCH_BOOST (2.0x) because semantic_sim > 0

### Real finding: A5-HELMET_MISSING (0.50)
```json
{
  "rule_id": "A5-HELMET_MISSING",
  "confidence": 0.5,
  "tags": [],              ← NO "corpus" tag = MISSING_CALL detection
  // No match_evidence — different detection mechanism
}
```
**Why it scores 0.50:** MISSING_CALL detection (helmet() not called)
→ sem_mult = SEMANTIC_ZERO_PENALTY (0.30x) because semantic_sim = 0

### The fundamental mismatch

Two different detection mechanisms are being compared:
1. **Flow-based** (noise): traces data from source → sink, computes similarity
2. **Missing-call** (real): checks if a function is NOT called

Missing-call findings don't have match_evidence (ngram_sim, semantic_sim, etc.)
They get penalized by sem_mult because they can't produce semantic_sim > 0

## Recommended Fix

The `sem_mult` penalty should NOT apply to MISSING_CALL findings because:
1. They don't have match_evidence (no semantic_sim to compare)
2. They use a different detection mechanism (checking absence, not similarity)
3. Penalizing them for missing semantic_sim is measuring the wrong thing

**Fix:** Skip sem_mult when `positive.tags` doesn't contain "corpus" or when the finding has no match_evidence.

```rust
let sem_mult = if positive.semantic_markers.is_empty() {
    1.0
} else if !positive.tags.contains(&"corpus".to_string()) {
    // MISSING_CALL / CONFIG findings — different detection mechanism
    1.0
} else if has_flow_match || dim.semantic_sim > 0.0 {
    SEMANTIC_MATCH_BOOST
} else {
    SEMANTIC_ZERO_PENALTY
};
```

## Files Involved

- `frensense-engine/src/pattern/scorer.rs` — sem_mult logic (line 453-459)
- `frensense-engine/src/pattern/similarity.rs` — dimension computation
- `corpus/targets/legacy/nodegoat/ts_ns_*.js` — NodeGoat patterns
- `scripts/benchmark_nodegoat.py` — benchmark script (uses --min-confidence 0.0)

---

## Issue 2: Two Separate Detection Systems with Different Scoring

### The Problem

Real vulnerabilities from the middleware audit system score **0.50** (hardcoded default), while corpus noise scores **0.55** (from the scoring pipeline). These two systems never share scoring logic.

### System A: Middleware Audit (`src/engine/findings/middleware_audit.rs`)

18 hardcoded pattern checks that look for specific patterns in specific files:
- A5-HELMET_MISSING, A5-X_POWERED_BY, A5-NOSNIFF, A5-COOKIE_NAME, A8-CSRF_MIDDLEWARE
- A2-USER_ENUM, A2-WEAK_PW, A2-NO_SESSION_REGENERATE, A7-NO_ADMIN_CHECK
- A1-LOG_INJECTION, A3-WRONG_ENCODING, A4-IDOR_PARAM, A10-SSRF, REDOS, etc.

**How it works**: Simple text pattern matching (e.g., "is `helmet(` in active code? is it in comments?"). No fingerprinting, no similarity computation.

**Confidence**: Hardcoded to `0.5` via `Advisory::bare()` at `src/lib.rs:193`. Never adjusted by the scoring pipeline.

### System B: Corpus Scoring (`frensense-engine/src/pattern/scorer.rs`)

Fingerprint similarity-based matching against 611 compiled patterns. Uses `weighted_score()` → `compute_score_with_negatives()` → composition.

**Confidence**: Computed from `weighted_score()` in `similarity.rs:23-53`. Noise findings land at 0.55 because the `gate` computation (`identity_gate * 2.5 + 0.1`) produces ~0.55 for generic matches.

### Why They're Incompatible

```
Middleware audit (REAL findings):  0.50  ← hardcoded default
Corpus noise (FALSE POSITIVES):    0.55  ← from weighted_score computation
```

The 0.05 gap means real vulnerabilities score **lower** than noise. No threshold can separate them. The middleware audit bypasses the entire scoring pipeline — it never computes ngram_sim, api_sim, semantic_sim, or flow_sim.

### The Pipeline

```
run_corpus_scan()           → CORPUS_* findings (score: 0.43-0.90)
run_findings_modules()      → middleware audit findings (score: 0.50)
check_vulnerable_deps()     → VULN_NPM_* findings (score: 0.90)
apply_composition()         → adjusts confidence based on cross-layer signals
```

The middleware audit findings enter the pipeline at `runner.rs:1501` via `run_findings_modules()`, but their confidence is already locked at 0.50.

---

## Issue 3: Slow Scoring (5-15 seconds per function)

### The Problem

Some functions take 5-15 seconds to score:
```
slow scoring function=configureApp  file=server.ts         ms=15851
slow scoring function=handleYamlUpload file=fileUpload.ts  ms=10625
slow scoring function=observeMetrics file=metrics.ts       ms=7706
slow scoring function=updateProductReviews                 ms=7013
```

### Root Cause

The bottleneck is in `registry.rs:scan_function` (line 395):

1. **LSH query** (lines 406-446): Queries 3 indexes (structural, API-call, flow). Union of all matches → `all_candidates`
2. **DimCache pre-compute** (lines 567-586): For every candidate × every positive/negative target, computes **15 similarity dimensions** via `compute_dimensions()` — including `tree_edit_distance()` on AST skeleton hashes (similarity.rs:254), which is O(n²)
3. **score_candidate** (line 593): For each surviving candidate, runs gates + weighted scoring

The slow functions are large — they have many API calls and structural markers, so the LSH returns a huge candidate set. The DimCache then computes dimensions for `N_candidates × (N_positives + N_negatives)` pairs, and `tree_edit_distance` is the expensive part.

### Potential Fix

Add early termination: if a function has >N structural markers or >M API calls, skip the expensive AST distance computation and fall back to Jaccard on structural_markers (which `compute_dimensions` already does when `ngram_sim <= 0.25`).

---

## Issue 4: "Anonymous" Function Names

### The Problem

Some functions show as "anonymous" in scoring logs:
```
slow scoring function=anonymous file=datacreator.ts  ms=2031
slow scoring function=anonymous file=server.ts       ms=2727
```

### Root Cause

Two naming systems exist:

| Path | Location | Fallback | Handles `const foo = () => {}`? |
|------|----------|----------|---------------------------------|
| Fingerprint extraction | `extraction.rs:134` | `"anonymous"` | Yes — via `infer_function_name()` |
| Cross-file taint pass | `runner.rs:237` | `"anon_{row}_{col}"` | Yes — checks `variable_declarator` parent |
| Taint summary cache | `runner.rs:1226` | `"_anonymous"` | **No** — only checks node's own `name` field |
| Taint verification | `runner.rs:939` | `"_anonymous"` | **No** — only checks node's own `name` field |

The `anonymous` in the slow scoring log comes from `extraction.rs:134`, which defaults to `"anonymous"` when `infer_function_name()` fails. This happens for:
- Callbacks passed as arguments: `app.use((req, res) => {...})`
- IIFE callbacks: `.forEach((item) => {...})`
- Functions where the parent node is a `call_expression`, not a `variable_declarator`

The `infer_function_name()` function at `route_registry.rs:356-399` handles 5 parent cases (assignment, variable_declarator, pair, method_definition, function_declaration) but NOT call_expression arguments.

---

## Issue 5: frensense-lang Not Fully Migrated (Three Overlapping Systems)

### The Problem

The `LanguageSpec` trait in `frensense-lang` was designed to replace all hardcoded language knowledge, but the engine was never fully migrated. Three overlapping systems exist that never fully connect.

### The Three Systems

#### System A: LanguageSpec (per-language, trait-based)
- **File**: `frensense-lang/src/spec.rs`
- **Implementations**: 6 providers (TypeScript, JavaScript, Go, Python, Rust, C)
- Provides: `known_sink_names()`, `known_source_patterns()`, `request_param_names()`, `response_method_names()`, `db_api_method_names()`, `shell_api_method_names()`, `route_registration_patterns()`, `propagator_rules()`, etc.

#### System B: Hardcoded source/sink in source_sink.rs
- **File**: `frensense-engine/src/corpus/source_sink.rs`
- `ALWAYS_REGISTER_SINKS` (lines 32-227): 195-entry monolithic sink list mixing all languages
- `always_register_source_patterns()` (lines 366-394): 23-entry JS-centric source pattern list
- Used as the `Default` for `CorpusSourceSinkRegistry`

#### System C: Hardcoded motifs in motifs.rs
- **File**: `frensense-engine/src/corpus/motifs.rs`
- `MOTIFS` (lines 21-420): 19 motif groups with ~400 hardcoded member call names
- Duplicates what LanguageSpec providers already define

### How They Overlap

| What | Hardcoded (source_sink.rs) | LanguageSpec | Overlap? |
|------|---------------------------|--------------|----------|
| Source patterns | 23 JS-only entries | Go:31, Py:28, Rust:11, JS:52, C:2 | Yes — JS tripled |
| Sink names | 195 mixed-language entries | Go:~120, Py:~100, Rust:~90, JS:~160, C:~20 | Yes — all in flat list |
| HTTP methods | `function_role.rs:30-64` | `TypeScriptSpec.response_method_names()` | Yes |
| Request params | `function_role.rs:67` | `TypeScriptSpec.request_param_names()` | Yes |
| Route registrations | `function_role.rs:71-100` | `TypeScriptSpec.route_registration_patterns()` | Yes |
| DB API names | `function_role.rs:103-146` | `TypeScriptSpec.db_api_method_names()` | Yes |
| Shell API names | `function_role.rs:148-174` | `TypeScriptSpec.shell_api_method_names()` | Yes |

### How frensense-lang Fails

**1. The fallback pattern makes LanguageSpec optional everywhere.**

Every engine subsystem passes the spec as `Option<&dyn LanguageSpec>` and falls back to hardcoded lists:
- `semantic.rs:484-501`: `known_sink_names()` and `known_source_patterns()` try spec first, fall back to `always_register_sinks_with_categories()` / `always_register_source_patterns()`
- `function_role.rs:248-350`: `classify_role_with_imports()` tries spec methods, then falls back to `HTTP_METHODS`, `REQUEST_PARAM_NAMES`, etc.
- `frensense-bundler/src/loader/features.rs:78-82`: falls back to `always_register_source_patterns()`
- `fingerprint/extraction.rs:86-105`: falls back to hardcoded node kinds

**2. The hardcoded fallbacks are strictly worse than the spec.**

- `always_register_source_patterns()` has only JS/Express patterns. Go, Python, Rust source patterns are lost.
- `ALWAYS_REGISTER_SINKS` mixes all languages into one flat list with no category discrimination.

**3. Massive duplication across systems.**

- `req.body` appears in: `always_register_source_patterns()`, `JS_SOURCE_PATTERNS`, and `MOTIFS[0].members`
- `eval` appears in: `ALWAYS_REGISTER_SINKS`, `GoSpec.known_sink_names`, `PythonSpec.known_sink_names`, `JS_SINK_NAMES`, and `MOTIFS[8].members`
- `Command::new` appears in: `ALWAYS_REGISTER_SINKS`, `RustSpec.known_sink_names`, and `MOTIFS[1].members`

**4. Some engine subsystems never use the spec at all.**

Hardcoded node-kind strings remain in files that should use `spec.classify()`:
- `symbols.rs:53-61`: hardcoded `"function_declaration"`, `"method_definition"`, `"variable_declarator"`
- `export_matcher.rs:128-201`: hardcoded `"method_definition"`, `"variable_declarator"`
- `route_registry.rs:164-448`: many hardcoded node kinds
- `data_flow/confidence.rs:300-339`: hardcoded node kinds
- `data_flow/pdg.rs:164-165`: hardcoded `"variable_declarator"`, `"assignment_expression"`

### What Should Be Corpus-Driven But Is Not

1. **Source patterns** should come from `LanguageSpec.known_source_patterns()`, not `always_register_source_patterns()`
2. **Sink registration** should use `LanguageSpec.known_sink_names()`, not `ALWAYS_REGISTER_SINKS`
3. **Motif members** should be derived from spec data, not static arrays in `motifs.rs`
4. **`function_role.rs` fallback arrays** should be removed — they duplicate what every spec provides
5. **The `SemanticProvider`** should be initialized with spec data at construction time, not have a runtime fallback to hardcoded lists

### Root Cause

LanguageSpec was designed to replace all hardcoded lists, and the trait is fully implemented across all 6 providers. However, the engine was never fully migrated — the hardcoded fallbacks were kept as safety nets and became the de facto primary path in many code paths. The spec is consulted in ~22 call sites via `spec_for_ext()`, but the hardcoded `ALWAYS_REGISTER_SINKS` and `always_register_source_patterns()` are used in 11+ direct call sites and serve as the `Default` for `CorpusSourceSinkRegistry`.

---

## Issue 6: Full Engine Migration Audit

### A. Hardcoded Node Kinds (~370 occurrences across 31 files)

Every file below has hardcoded tree-sitter node kind strings (e.g., `"function_declaration"`, `"call_expression"`) that should use `LanguageSpec.classify()`.

**Files with NO spec access (19 files, ~200 occurrences):**

| File | Occurrences | Impact |
|------|-------------|--------|
| `src/semantics/data_flow/cross_file.rs` | 31 | Critical taint flow path |
| `src/semantics/data_flow/interprocedural.rs` | 29 | Critical taint flow path |
| `src/engine/project/runner.rs` | 30 | Main audit orchestrator |
| `src/semantics/data_flow/normalization.rs` | 19 | Semantic extraction |
| `frensense-engine/src/route_registry.rs` | 18 | Route handler detection |
| `frensense-engine/src/data_flow/taint_metrics.rs` | 16 | Taint metrics |
| `src/engine/auditor/discovery.rs` | 14 | Rule discovery |
| `src/engine/auditor/events.rs` | 12 | Event extraction |
| `frensense-engine/src/symbols.rs` | 10 | Symbol table |
| `frensense-engine/src/data_flow/resolver.rs` | 12 | Parameter bindings |
| `frensense-engine/src/corpus/semantic.rs` | 10 | Semantic filters |
| `frensense-engine/src/export_matcher.rs` | 5 | Export matching |
| `frensense-engine/src/decorator.rs` | 3 | Decorator detection |
| `src/engine/ast_diff.rs` | 8 | AST diffing |
| `src/semantics/simple_taint.rs` | 5 | Simple taint |
| `src/semantics/data_flow/handlers.rs` | 5 | Handler detection |
| `src/semantics/data_flow/corpus_seeder.rs` | 6 | Corpus seeding |
| `frensense-engine/src/data_flow/normalization.rs` | 1 | Normalization |

**Files with spec but hardcoded fallbacks (12 files, ~170 occurrences):**

| File | Occurrences | Issue |
|------|-------------|-------|
| `frensense-engine/src/fingerprint/ast_walkers.rs` | 25 | Fallback branches when spec=None |
| `frensense-engine/src/cfg/def_use.rs` | 23 | Helper fns lack spec access |
| `frensense-engine/src/data_flow/confidence.rs` | 20 | Fallback branches |
| `frensense-engine/src/corpus/flow_fingerprint.rs` | 20 | Fallback branches |
| `frensense-engine/src/corpus/data_flow_extractor.rs` | 14 | Fallback branches |
| `frensense-engine/src/corpus/source_sink.rs` | 10 | Fallback branches |
| `frensense-engine/src/graph.rs` | 7 | Fallback branches |
| `frensense-engine/src/cfg/mod.rs` | 12 | Fallback branches |
| `frensense-engine/src/data_flow/pdg.rs` | 4 | Direct hardcoded |
| `frensense-engine/src/fingerprint/extraction.rs` | 3 | Fallback when spec=None |
| `frensense-engine/src/corpus/registry.rs` | 1 | Indirect |
| `frensense-bundler/src/loader/features.rs` | 10 | Spec accepted but unused |

### B. Hardcoded Source/Sink/Motif Lists (~1,040 duplicated entries)

| Hardcoded Array | File | Entries | Languages | LanguageSpec Provides Same? |
|----------------|------|---------|-----------|---------------------------|
| `ALWAYS_REGISTER_SINKS` | `source_sink.rs:32-227` | 196 | Mixed | YES — all 6 providers |
| `ALWAYS_REGISTER_SINKS_COMPILER_AWARE` | `source_sink.rs:236-345` | 109 | Mixed | YES — subset |
| `always_register_source_patterns()` | `source_sink.rs:366-394` | 26 | JS-focused | YES — all providers |
| `SANITIZER_FRAGMENTS` | `source_sink.rs:683-695` | 11 | Agnostic | YES — `classify_sanitizer()` |
| `MOTIFS` (18 groups) | `motifs.rs:21-420` | 276 | Multi-lang | YES — 12/18 groups |
| `HTTP_METHODS` | `function_role.rs:30-64` | 24 | Multi-lang | YES — `response_method_names()` |
| `REQUEST_PARAM_NAMES` | `function_role.rs:67` | 6 | JS/Go | YES — `request_param_names()` |
| `ROUTE_REGISTRATIONS` | `function_role.rs:71-100` | 29 | JS only | YES — `route_registration_patterns()` |
| `DB_API` | `function_role.rs:103-146` | 43 | Multi-lang | YES — `db_api_method_names()` |
| `SHELL_API` | `function_role.rs:149-174` | 23 | Multi-lang | YES — `shell_api_method_names()` |
| `HTTP_FRAMEWORK_PACKAGES` | `semantic.rs:209-211` | 11 | JS only | YES — `package_category()` |
| `PACKAGE_SINK_CATEGORIES` | `semantic.rs:218-254` | 35 | JS only | YES — `package_category()` |
| `ROUTE_ENV_KEYWORDS` | `context/mod.rs:33-84` | 50 | Multi-lang | YES — `route_context_hints()` |
| `TEST_ENV_KEYWORDS` | `context/mod.rs:6-28` | 16 | Multi-lang | YES — `test_context_hints()` |

### C. LanguageSpec Usage Gaps (7 bugs)

| # | File | Line | Bug | Severity |
|---|------|------|-----|----------|
| 1 | `corpus/registry.rs` | 673 | `classify_role(weighted_fp)` called WITHOUT spec in `score_candidate`, despite spec being available in parameters. Function role gate is always spec-blind. | HIGH |
| 2 | `corpus/semantic.rs` | 409, 432 | `extract_data_flows(*node, source, None)` — auto-filter stats builder is spec-blind. | MEDIUM |
| 3 | `bundler/loader/mod.rs` | 103 | `FileContext::extract()` called without spec, even though `spec_for_ext(ext)` was resolved 14 lines earlier. | MEDIUM |
| 4 | `bundler/loader/features.rs` | 29-99 | `collect_function_features` accepts spec but never uses it — hardcoded `"call_expression"` matching. | MEDIUM |
| 5 | `semantic.rs` | 373 | `ImportMapProvider::new()` sets `spec: None`. The `with_spec` constructor exists but is never called — all spec-aware code paths in ImportMapProvider are dead. | MEDIUM |
| 6 | `data_flow/mod.rs` | 148 | `classify_param_name_in_context` convenience function hardcodes `None` for spec. | LOW |
| 7 | `context/mod.rs` | 126 | `FileContext::extract` convenience method hardcodes `None` for spec. | LOW |

### D. Provider System Gaps

| # | Issue | Severity |
|---|-------|----------|
| 1 | **No OxcProvider caching** — each `per_file_provider()` call re-parses the entire TS/JS file with Oxc. A file can be parsed 2+ times during corpus scan. | HIGH |
| 2 | **SemanticExtractor misses Python/Go/C** — `normalization.rs:47-51` only handles `rs`, `ts`, `js`, `tsx`, `jsx`. Zero semantic ops for Go/Python/C, disabling alias tracking. | HIGH |
| 3 | **Hardcoded function kinds in runner.rs** (lines 208, 919, 1215) — misses Go `method_declaration`, Python `async_function_definition`, Rust `closure_expression`. | HIGH |
| 4 | **ImportMapProvider spec integration is dead** — `with_spec()` constructor never called. All `self.spec.as_deref()` paths unreachable. | MEDIUM |
| 5 | **No compiler providers for Python/Go/C** — silently degrades to heuristics under `--use-compiler`. | MEDIUM |
| 6 | **CargoDepsProvider is a stub** — always returns empty. | LOW |
| 7 | **Consumer crate parser duplication** — `src/parser.rs` has its own hardcoded extension match, misses Go/C. | LOW |
| 8 | **`spec_for_path()` is dead code** — defined but never called. | LOW |

### E. Dead Code from Incomplete Migration

| Item | Location | Why Dead |
|------|----------|----------|
| `ImportMapProvider::with_spec()` | `semantic.rs:382-394` | Never called |
| `ImportMapProvider.spec` field | `semantic.rs:368` | Always `None` |
| `package_sink_category_from_spec()` | `semantic.rs:278-298` | Always receives `None` spec |
| `is_http_framework_package()` spec branch | `semantic.rs:303-315` | Always receives `None` spec |
| `InterproceduralTaintVerifier::with_spec()` | `interprocedural.rs:90-110` | Never called |
| `CargoDepsProvider` | `deps_provider.rs:80-90` | Always returns empty |
| `Language::Html` variant | `lang/mod.rs:14-15` | Marked `#[allow(dead_code)]` |
| `spec_for_path()` | `registry.rs:130` | Never called |

### F. What Should Be Corpus-Driven But Is Not

1. **Source patterns**: Should come from `LanguageSpec.known_source_patterns()`, not `always_register_source_patterns()` (26 JS-only entries)
2. **Sink registration**: Should use `LanguageSpec.known_sink_names()`, not `ALWAYS_REGISTER_SINKS` (196 mixed entries)
3. **Motif members**: Should be derived from spec data, not 276 hardcoded entries in `motifs.rs`
4. **Function role arrays**: `HTTP_METHODS`, `REQUEST_PARAM_NAMES`, `ROUTE_REGISTRATIONS`, `DB_API`, `SHELL_API` should be removed — they duplicate what every spec provides
5. **Node kind classification**: All ~370 hardcoded node kind strings should use `LanguageSpec.classify()`
6. **Context hints**: `ROUTE_ENV_KEYWORDS` and `TEST_ENV_KEYWORDS` should come from spec, not 66 hardcoded entries

### G. Migration Priority

**Phase 1 — Fix bugs where spec IS available but not wired (immediate):**
1. Wire spec through `score_candidate` in `registry.rs:673`
2. Wire spec through `bundler/loader/mod.rs:103` and `features.rs:29-99`
3. Wire spec through `corpus/semantic.rs:409,432`

**Phase 2 — Eliminate dead code paths:**
4. Remove `ImportMapProvider.spec` field and dead `with_spec()` constructor
5. Remove `InterproceduralTaintVerifier::with_spec()` dead code
6. Remove `spec_for_path()` dead code

**Phase 3 — Wire spec into files that lack it (high-impact first):**
7. `cross_file.rs` (31 occurrences) — add spec parameter
8. `interprocedural.rs` (29 occurrences) — add spec parameter
9. `runner.rs` (30 occurrences) — replace hardcoded function kind lists with spec calls
10. `normalization.rs` (19 occurrences) — add spec parameter
11. `route_registry.rs` (18 occurrences) — add spec parameter
12. `symbols.rs` (10 occurrences) — add spec parameter

**Phase 4 — Remove hardcoded lists:**
13. Remove `ALWAYS_REGISTER_SINKS` and `always_register_source_patterns()` — replace with spec-aggregated defaults
14. Remove `MOTIFS` static array — derive from spec data
15. Remove `function_role.rs` fallback arrays — use spec exclusively
16. Remove `HTTP_FRAMEWORK_PACKAGES` and `PACKAGE_SINK_CATEGORIES` — use spec
17. Remove `ROUTE_ENV_KEYWORDS` and `TEST_ENV_KEYWORDS` — use spec

**Phase 5 — Provider system improvements:**
18. Cache OxcProvider per-file
19. Add `SemanticExtractor` support for Python/Go/C
20. Wire `ImportMapProvider::with_spec()` or remove the dead code
21. Add `--use-compiler` warning for unsupported languages
