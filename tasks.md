# Frensense v0.4.0 — Task Tracker

Sources: `frensense_audit_v0.4.0.md`, `CORPUS_CURATION_GUIDE.md`, `CORPUS_BAKING_STRATEGY.md`

---

## Bugs

### B1 — Taint Advisory Over-Flags Clean Arguments
- **Status:** Open
- **Priority:** High
- **File:** `src/analysis/taint/resolve.rs` → `analyze_call`
- **Problem:** `db::execute(conn, tainted_data)` fires advisories on ALL args including clean `conn`. Should only flag the tainted argument.
- **Fix:** Per-argument taint attribution at call sites instead of per-call-site.

### B2 — Corpus Loader Warns on Malformed TypeScript Files
- **Status:** Open
- **Priority:** Medium
- **Problem:** `ts_hardcoded_secret_negative.ts` has no parseable function body. Loader emits a warning instead of silently skipping.
- **Fix:** Silent skip with optional `--verbose` diagnostic.

### B3 — ts_open_redirect_positive is Empty
- **Status:** Open
- **Priority:** High
- **File:** `corpus/ts_open_redirect_positive.ts`
- **Problem:** File exists but contains no code. Open redirect detection has no positive example and cannot fire.
- **Fix:** Add content like `res.redirect(req.query.url)` — user-controlled URL passed directly to redirect.

### B4 — Dependency Resolver Silently Disabled for Rust
- **Status:** Open
- **Priority:** Medium
- **Problem:** `deps` module works for TS/JS via `package.json` but is disabled for Rust. Users have no indication this capability is inactive.
- **Fix:** Add `--check-deps` opt-in flag that requires `cargo metadata` to be available.

### B5 — RulesWrapper Dead Code in CLI
- **Status:** Open
- **Priority:** Medium
- **File:** `src/cli/commands.rs`
- **Problem:** `RulesWrapper` struct is leftover from YAML rule era. Serves no purpose in corpus-driven architecture.
- **Fix:** Remove entirely.

### B6 — Stale GenSense Name References
- **Status:** Open
- **Priority:** Medium
- **Files:** `debug_registry.txt`, `clippy_errors.txt`, `Makefile`, `SKILLS.md`, `BENCHMARK.md`
- **Problem:** Multiple files still reference "GenSense" instead of "Frensense".
- **Fix:** Grep all files for `gensense`, `GenSense`, `GENSENSE` and replace with `frensense`, `Frensense`, `Frensense`.

### B7 — L3 Taint Entropy Wired to Nothing
- **Status:** Open
- **Priority:** High
- **Problem:** `TaintMetrics.taint_branch_ratio` is computed but discarded. Hollow validators generate L1+L2 findings even though L3 would suppress or downgrade them.
- **Fix:** Wire `taint_branch_ratio` into the confidence pipeline. If a function named like a validator has ratio < 0.2, reduce confidence of corpus matches against validator patterns.

### B8 — YAML Dependencies Still in Cargo.toml
- **Status:** Open
- **Priority:** Medium
- **File:** `Cargo.toml`
- **Problem:** `serde_yaml` and `tree-sitter-yaml` remain in dependencies after YAML DSL was deleted. Unused weight in compile time and binary size.
- **Fix:** Audit for any remaining usage. Remove `serde_yaml` and make `tree-sitter-yaml` optional if YAML scanning is not a current feature.

---

## L3 Wiring (Architectural)

### A1 — Wire TaintMetrics into Confidence Pipeline
- **Status:** Open
- **Priority:** High
- **Depends on:** B7
- **Problem:** L3 taint entropy computation runs but result is thrown away. The AND gate is effectively 3 layers, not 4.
- **Fix:** Read `taint_branch_ratio` when deciding whether to emit or adjust confidence. Hollow validator functions (named `validate_*`, ratio < 0.2) should have confidence reduced.

---

## Taint Rules Externalization

### E1 — Externalize Taint Rules to TOML
- **Status:** Open
- **Priority:** High
- **Problem:** Six taint rules are hardcoded in Rust source. Teams with custom ORMs, HTTP clients, or logging frameworks cannot extend without forking.
- **Fix:** Move `TaintRule` structs to `taint_rules.toml`. Document the format. Add example custom rules in docs.

### E2 — Document Custom Taint Rule Format
- **Status:** Open
- **Priority:** Medium
- **Depends on:** E1
- **Fix:** Write docs showing how to add a new taint source, sink, or sanitizer in `taint_rules.toml`.

---

## Consumer-Layer Tests

### T1 — CLI Flag Integration Tests
- **Status:** Open
- **Priority:** High
- **Problem:** Zero tests on consumer layer after YAML rule tests were deleted. CLI flags, output format, SARIF serialization, baseline suppression all untested.
- **Fix:** Add integration tests for: `--strict`, `--severity`, `--baseline`, `--diff-only`, `--json`, `--sarif`.

### T2 — Corpus Loader Edge Case Tests
- **Status:** Open
- **Priority:** Medium
- **Depends on:** B2
- **Fix:** Test that malformed corpus files (empty, no function body, bad syntax) are handled gracefully without warnings or crashes.

### T3 — Patcher Output Tests
- **Status:** Open
- **Priority:** Low
- **Fix:** Test that `--fix` generates correct suggested changes for known patterns (e.g., `console.log` → structured logger, missing `.catch()`).

---

## Corpus Quality Improvements

### C1 — Fill Open Redirect Positive Example
- **Status:** Open
- **Priority:** High
- **Depends on:** B3
- **Fix:** Write a realistic positive example: `res.redirect(req.query.url)` or `res.redirect(req.body.next)`.

### C2 — Improve ts_god_function Positive Example
- **Status:** Open
- **Priority:** Medium
- **Problem:** Current positive is 100x `console.log()`. Too artificial to generalize.
- **Fix:** Replace with a function that mixes HTTP parsing, DB lookup, business logic, and response formatting in one block.

### C3 — Strengthen ts_llm_any_parameter Pattern
- **Status:** Open
- **Priority:** Medium
- **Problem:** `function processUser(id: any)` is extremely common in legacy TS codebases. High false positive risk.
- **Fix:** Add richer context: show `any` in a function that also lacks runtime type validation, with a negative that uses proper type narrowing.

### C4 — Strengthen ts_llm_console_log Pattern
- **Status:** Open
- **Priority:** Medium
- **Problem:** Positive is too broad — any function with `console.log`.
- **Fix:** Narrow to `console.log` inside route handlers or async functions specifically. Show structured logging in the negative.

### C5 — Improve rust_connection_leak Positive Example
- **Status:** Open
- **Priority:** Medium
- **Problem:** Current positive is 5 lines unconditional leak. Real leaks are early-return errors.
- **Fix:** Rewrite to show a function with error paths where happy-path calls close but error path returns without it.

### C6 — Improve ts_as_any_escape Pattern
- **Status:** Open
- **Priority:** Low
- **Problem:** `req.body as any` is common in Express handlers. Hard to distinguish from legitimate type assertions.
- **Fix:** Add negative that shows proper type validation before use.

### C7 — Improve Minimal Pattern Examples
- **Status:** Open
- **Priority:** Medium
- **Files:** `ts_csa_validate_unconditional`, `ts_csa_sanitize_passthrough`, `rust_clone_in_loop`
- **Problem:** Examples too minimal for similarity scorer to work well. `rust_clone_in_loop_positive.rs` is 5 lines. `ts_csa_validate_unconditional_positive.ts` is 2 lines. These teach the fingerprinter a shape too narrow to match real code.
- **Fix:** Enrich each to ≥15 lines with realistic context: variable declarations, parameter lists, control flow, return statements. No comments pointing to bug. `rust_clone_in_loop` should show a real loop with clone inside, surrounded by function context. Hollow validators should show intermediate steps before the return.

---

## New Corpus Patterns to Add

### P1 — SQL Injection (Security)
- **Status:** Open
- **Priority:** High
- **Positive:** Template literal interpolation into SQL string.
- **Negative:** Parameterized query with placeholders.

### P2 — Prototype Pollution (Security / TS+JS)
- **Status:** Open
- **Priority:** High
- **Positive:** `obj[userControlledKey]` assignment without key sanitization.
- **Negative:** Filter out `__proto__` and `constructor` keys.

### P3 — Path Traversal (Security)
- **Status:** Open
- **Priority:** High
- **Positive:** `fs.readFile(path.join(baseDir, req.params.filename))` without normalization.
- **Negative:** `path.normalize()` + prefix check.

### P4 — JWT Verification Bypass (Security)
- **Status:** Open
- **Priority:** High
- **Positive:** `jwt.decode(token)` used to authenticate.
- **Negative:** `jwt.verify(token, secret)`.

### P5 — SSRF (Security)
- **Status:** Open
- **Priority:** High
- **Positive:** `fetch(req.query.url)` in backend handler.
- **Negative:** Allowlist check before fetch.

### P6 — Unnecessary Arc Clone in Hot Path (Rust Performance)
- **Status:** Open
- **Priority:** Medium
- **Positive:** `arc_value.clone()` inside `for` or async loop.
- **Negative:** Clone moved outside loop.

### P7 — Integer Overflow in Index Arithmetic (Rust Correctness)
- **Status:** Open
- **Priority:** Medium
- **Positive:** `(a - b) as i32` where both are `usize`.
- **Negative:** Checked subtraction or bounds check.

### P8 — Ignoring Drop Order (Rust Correctness)
- **Status:** Open
- **Priority:** Low
- **Positive:** Struct field ordering causing guard-after-mutex drop.
- **Negative:** Explicit guard drop before scope exit.

### P9 — Missing Await on Async Function (TypeScript)
- **Status:** Open
- **Priority:** Medium
- **Positive:** `asyncDatabaseQuery()` called without `await`.
- **Negative:** `await asyncDatabaseQuery()`.

### P10 — Race Condition in Promise.all Order Dependence (TypeScript)
- **Status:** Open
- **Priority:** Medium
- **Positive:** `Promise.all([a(), b()])` where `b()` depends on `a()` result.
- **Negative:** Sequential `await a(); await b()`.

### P11 — CORS Misconfiguration (TypeScript)
- **Status:** Open
- **Priority:** Medium
- **Positive:** `Access-Control-Allow-Origin: *` on authenticated endpoint.
- **Negative:** Origin allowlist checking.

### P12 — Panic in Async Context (Rust)
- **Status:** Open
- **Priority:** Medium
- **Positive:** `unwrap()` inside `tokio::spawn` closure.
- **Negative:** `?` or explicit error handling.

### P13 — Dead Error Branch (Both Languages)
- **Status:** Open
- **Priority:** High
- **Positive:** `Err(_) => {}` or `.catch(() => {})`.
- **Negative:** Meaningful error handling or propagation.

---

## AI/ML Enhancements (from Section 10)

### M1 — Token N-Grams with IDF Weighting
- **Status:** Open
- **Priority:** High
- **Problem:** Current fingerprinting treats all n-grams equally. `let x =` carries same weight as `db::execute(`.
- **Fix:** Weight n-grams by inverse corpus frequency. Rare, diagnostic tokens score higher. Borrow TF-IDF from information retrieval.

### M2 — AST Edit Distance
- **Status:** Open
- **Priority:** Medium
- **Problem:** Jaccard over n-grams is set-based. Doesn't capture structural edit distance.
- **Fix:** Compute minimum edit operations (insert/delete/substitute AST nodes) between target function and positive example. Small edit distance = stronger match signal.

### M3 — Contextual Featurization (Word2Vec-Inspired)
- **Status:** Open
- **Priority:** Medium
- **Problem:** Fingerprinting only looks at a function's own tokens. Ignores call context.
- **Fix:** Compute context signature from call sites (try/catch blocks, argument types, callers). A `sanitize_*` in a web handler has different context than in a test.

### M4 — Byte Pair Encoding for Language-Neutral Tokenization
- **Status:** Open
- **Priority:** Low
- **Problem:** Tokenization is language-specific. Rust tokens ≠ TypeScript tokens.
- **Fix:** BPE subword units instead of full keywords. `async fn handle()` and `async function handle()` share subword overlap. Strengthens cross-language generalization.

### M5 — Confidence Calibration via Platt Scaling
- **Status:** Open
- **Priority:** Medium
- **Problem:** Confidence scores are raw similarity. Score of 0.8 doesn't mean 80% probability.
- **Fix:** Train logistic regression on labeled TP/FP dataset to calibrate scores to true probability.

### M6 — One-Class Classification (Anomaly Detection)
- **Status:** Open
- **Priority:** Low
- **Problem:** Current system requires positive+negative pairs. Cannot detect novel bugs.
- **Fix:** Build "normal function" model from negative examples only. Flag functions with low similarity to all negatives as anomalous.

### M7 — Edit-Based Feedback Loop (Reinforcement-Inspired)
- **Status:** Open
- **Priority:** Medium
- **Problem:** Suppressed findings don't improve future detection.
- **Fix:** When a finding is marked false positive, analyze what made it match. Use features to bias negative examples. Over time, corpus adapts to codebase.

### M8 — Cross-Lingual Transfer Weighting
- **Status:** Open
- **Priority:** Low
- **Problem:** Patterns written for Rust apply to TS at full weight even if transfer accuracy is unmeasured.
- **Fix:** Measure transfer accuracy per pattern. Full weight for accurate transfers, reduced weight until language-specific examples added.

### M9 — Sliding Window N-Gram Context
- **Status:** Open
- **Priority:** Low
- **Problem:** N-grams are flat. Position in function doesn't matter.
- **Fix:** Position-weighted n-grams encoding location relative to function boundaries (early, at return sites, near control flow).

---

## Documentation & Naming Cleanup

### D1 — Rename All GenSense References to Frensense
- **Status:** Open
- **Priority:** Medium
- **Depends on:** B6
- **Files to audit:** `SKILLS.md`, `BENCHMARK.md`, `debug_registry.txt`, `clippy_errors.txt`, `Makefile`, any other file referencing GenSense.
- **Fix:** Full rename pass. Verify with grep.

### D2 — Write Corpus Contribution Guide
- **Status:** Open
- **Priority:** Medium
- **Source:** CORPUS_CURATION_GUIDE.md §What a Good Pattern Pair, §How to Write Negative, §How to Write Positive, §Variation Principle, §What Not to Write
- **Fix:** Document minimum quality bar covering:
  - **Positive examples:** ≥15 lines simple, ≥25 lines with control/data flow, ≥30 lines async/transaction. Must look like real code — realistic variable names, intermediate variables, no comments pointing to bug. Must be a function (not file-level expression or type declaration).
  - **Negative examples:** Must be the fixed version of the positive — same function name, same parameters, same local variables, same return type, same number of statements. Only bug-specific lines change. Must not be so different from positive that engine cannot tell they are related.
  - **Variation principle:** 4-6 variations per bug class. Each varies along a different dimension (hops, source form, nesting depth). Structurally very similar variations go in different files. Multi-example files (baking strategy) contain structurally different variations.
  - **What NOT to write:** Patterns for things compiler/type checker catches. Style preferences. Patterns where positive/negative differ by single character. Patterns with multiple bugs in positive.
  - **Naming convention:** `{language}_{category}_{pattern_name}_{positive_or_negative}.{extension}`. Language: `rust`, `ts`. Category: `sec` (security), `arch` (architecture), `async` (concurrency), `llm`, `csa`, or none. Pattern names: 2-3 words describing the bug, not the detection method.
  - **Validation:** Tested against 3+ real codebases. Confirmed to fire on at least one real-world true positive.
- **Enforce via CI corpus quality checks.

### D3 — Document Pattern Quality Tiers
- **Status:** Open
- **Priority:** Low
- **Fix:** Add `tier` field to corpus metadata. `high` = rich/realistic examples, `medium` = minimal. Adapt `--suite` flag to filter by tier.

### D4 — Corpus Tracking Document
- **Status:** Open
- **Priority:** Medium
- **Source:** CORPUS_CURATION_GUIDE.md §Tracking Progress Toward 400
- **Problem:** No tracking document for corpus pattern status. Patterns without confirmed real-world true positives are indistinguishable from validated ones.
- **Fix:** Create `corpus/TRACKING.md` alongside corpus directory. For each pattern name, record:
  - Bug class (injection, hollow, LLM, etc.)
  - Language (Rust, TypeScript, both)
  - Number of variation pairs
  - Date of last validation against a real codebase
  - Whether pattern has confirmed real-world true positive (yes/no)
- **Validation column is most important.** Patterns without confirmed real-world TPs should be deprioritized or replaced with variations of patterns that have confirmed hits.
- **Target:** 400 patterns where ≥350 have confirmed real-world TPs and confirmed absence of consistent FPs on validation codebases. A corpus of 300 well-validated pairs outperforms 400 pairs where 100 have never been tested.

---

## Benchmarking & Trust

### B10 — Benchmark on Real Open-Source Projects
- **Status:** Open
- **Priority:** Medium
- **Fix:** Run Frensense on `axum`, `actix-web`, `hyper`, `express`, `fastify`. Manually classify every finding as TP/FP. Publish results.

### B11 — Historical Scan Regression Tracking
- **Status:** Open
- **Priority:** Low
- **Fix:** Surface `scripts/historical-benchmark.sh` as `frensense history --since <git-tag>` CLI command.

---

## New Features

### F1 — Patcher / Auto-Remediation as Stable --fix Flag
- **Status:** Open
- **Priority:** Medium
- **Fix:** Promote experimental patcher to `--fix` with scope: `--fix style`, `--fix security`. Patterns: `console.log` → structured logger, missing `.catch()` → add handler, `jwt.decode` → `jwt.verify`.

### F2 — MCP-Driven IDE Integration
- **Status:** Open
- **Priority:** Low
- **Fix:** VSCode extension or Neovim plugin that calls MCP server on save and annotates findings inline.

### F3 — Baseline Export/Import for Team Acknowledgment
- **Status:** Open
- **Priority:** Medium
- **Fix:** `frensense acknowledge --finding <id>` adds to team-shared baseline with metadata (who, why). Makes suppression traceable.

### F4 — Dependency Hallucination Check for Rust
- **Status:** Open
- **Priority:** Medium
- **Depends on:** B4
- **Fix:** Once `--check-deps` is implemented, verify every import in Rust code refers to a real crate in `Cargo.lock` via `cargo metadata`.

### F5 — Pattern Confidence Tuning Per-User
- **Status:** Open
- **Priority:** Medium
- **Problem:** `--threshold` applies globally. Users cannot set different thresholds for different pattern categories (e.g., security vs. style).
- **Fix:** Per-pattern-category threshold tuning. Security patterns at lower threshold (more sensitive), style patterns at higher threshold (more conservative).

### F6 — Corpus Pattern Browser
- **Status:** Open
- **Priority:** Low
- **Problem:** `--list-patterns` shows loaded patterns but not their accuracy characteristics.
- **Fix:** Enhanced display showing each pattern's confidence distribution — how often it fires, average score, clean vs. buggy code performance.

### F7 — Validate on Real-World LLM-Generated Code
- **Status:** Open
- **Priority:** Medium
- **Problem:** The six `llm_*` corpus entries are untested against actual AI-generated code reviewed by senior engineers.
- **Fix:** Partner with a team using AI coding assistants. Run Frensense on classified AI-generated code. Determine if llm_* patterns fire on real bugs or benign code.

### F8 — TP/FP Tracking System
- **Status:** Open
- **Priority:** Medium
- **Problem:** No ground truth data on precision. All precision claims are theoretical without manual classification.
- **Fix:** Run Frensense on `axum`, `actix-web`, `hyper`, `express`, `fastify`. Manually classify every finding as TP or FP. Use data to: validate precision, identify noisy patterns, provide calibration dataset for confidence scores.

### F9 — Scaling Validation on Large Projects
- **Status:** Open
- **Priority:** Medium
- **Problem:** Benchmark data covers 10–100 files. 1M+ LOC capability claim is unvalidated.
- **Fix:** Run and publish results on several well-known large open-source projects to prove scaling claims.

---

## Engine Features — Built But Not Wired

### W1 — Wire Temporal Analysis into Findings
- **Status:** Open
- **Priority:** High
- **Engine:** `frensense-engine/src/temporal.rs` — TemporalAnalyzer, TemporalRule, check_must_follow, check_must_not_follow
- **Problem:** Temporal violations (mutex held across await, missing unlock, blocking in async) are computed but may not emit advisories. The FSA over event sequences runs but results don't reach the user.
- **Fix:** Wire temporal violations into the advisory output in `runner.rs`. Each violation should produce an Advisory with rule_id, severity, observation, impact, improvement.
- **Safety:** Add unit tests that verify temporal violations produce findings. Test with real async Rust code.

### W2 — Wire Reachability Analysis as User-Facing Feature
- **Status:** Open
- **Priority:** Medium
- **Engine:** `frensense-engine/src/reachability.rs` — ReachabilityChecker, check_reachability, is_dead_branch
- **Problem:** Reachability analysis is built and used internally for taint accuracy, but not surfaced as a standalone feature. Dead code paths and unreachable error handlers go undetected.
- **Fix:** Expose reachability as a finding category: "Dead error branch" (Err handler unreachable), "Unreachable code path" (after return/panic), "Unreachable validation" (check exists but path is dead). Add `--check-reachability` flag.
- **Safety:** Validate against real codebases with dead code. Confirm no false positives on valid early returns.

### W3 — Wire CFG/Def-Use as User-Facing Feature
- **Status:** Open
- **Priority:** Medium
- **Engine:** `frensense-engine/src/cfg/` — ControlFlowGraph, build_def_use, compute_reaching_defs, dominance_frontier
- **Problem:** CFG and def-use chains are built and used internally for taint analysis, but not exposed. Common bugs like "variable used before definition" or "variable defined but never used" go undetected.
- **Fix:** Surface def-use findings: "Variable used before definition" (use with no reaching def), "Variable defined but never used" (def with no uses), "Variable shadowed" (multiple defs in same scope). Add `--check-def-use` flag.
- **Safety:** Test against real codebases. Confirm no false positives on valid Rust/TypeScript patterns.

### W4 — Wire Cross-File Taint into Findings
- **Status:** Open
- **Priority:** High
- **Engine:** `frensense-engine/src/data_flow/cross_file.rs` — Resolver, register_exposed_taint, resolve_taint, FunctionTaintSummary
- **Problem:** Cross-file taint resolution is built and tracks taint across file boundaries, but findings may not surface when taint flows from file A through a function to file B's sink.
- **Fix:** Ensure cross-file taint paths produce findings with file references. "Taint flows from req.body in handlers/login.ts through validate() to db.query in services/user.ts". Include both source and sink file paths in the advisory.
- **Safety:** Test with multi-file projects. Verify cross-file findings include correct file paths and line numbers.

### W5 — Implement User Rule Loading
- **Status:** Open
- **Priority:** High
- **Engine:** `src/engine/auditor/user_rules.rs` — load_user_rules() currently returns empty vectors
- **Problem:** `--extra-rule-dirs` flag exists but load_user_rules() always returns empty. Users cannot add custom rules.
- **Fix:** Implement YAML rule loading in user_rules.rs. Define a minimal rule format: `{id, severity, pattern, message}`. Parse rules from extra directories. Merge with built-in rules.
- **Safety:** Test with a sample custom rule. Verify it fires alongside built-in rules. Confirm `--disable-rule` works on custom rules.

### W6 — Wire Style Profile into Findings Pipeline
- **Status:** Open
- **Priority:** Medium
- **Engine:** `src/engine/profile.rs` — ProjectProfile, learn(), LanguageProfile, FileProfile
- **Problem:** Style profile is built and CLI flags exist (--learn-profile, --check-profile, --profile-threshold, --profile-stats), but anomaly detection may not be fully integrated into findings.
- **Fix:** Wire profile anomalies into advisory output. Functions with unusual n-gram distribution, unexpected naming patterns, or anomalous structural markers should produce findings with "Style anomaly" rule_id and explanation of what makes it unusual.
- **Safety:** Test on real codebases. Confirm anomalies are real style deviations, not false positives.

### W7 — Enable Dependency Check for Rust
- **Status:** Open
- **Priority:** Medium
- **Engine:** `frensense-engine/src/deps.rs` — DependencyResolver, scan_file, check_rust_import
- **Problem:** Dependency hallucination detection works for TS/JS (package.json) but is disabled for Rust. LLM-hallucinated crate names go undetected.
- **Fix:** Add `--check-deps` opt-in flag. When enabled, run `cargo metadata` to get crate list. Verify all `use` statements reference real crates. Report hallucinated imports as findings.
- **Safety:** Test with code that imports non-existent crates. Verify `cargo metadata` is required and fails gracefully if unavailable.

### W8 — Wire Pattern Canonical Form for Structural Matching
- **Status:** Open
- **Priority:** Low
- **Engine:** `frensense-engine/src/pattern/` — PatternCompiler, PatternMatcher, PatternScorer, CanonicalForm
- **Problem:** Pattern compiler and matcher are built but the current corpus uses fingerprint-based matching (n-grams). The canonical form module may be unused.
- **Fix:** Evaluate whether canonical form matching improves detection over n-gram fingerprinting. If yes, integrate as an alternative scoring method. If no, document why and keep n-gram approach.
- **Safety:** Compare detection results between canonical form and n-gram on the same corpus. Measure precision/recall difference.

### W9 — Surface Atomic Section Detection for C
- **Status:** Open
- **Priority:** Low
- **Engine:** `frensense-engine/src/atomic_section.rs` — AtomicSectionAnalyzer, AtomicOp, has_incomplete_sections
- **Problem:** TOCTOU/lock-pair detection for C is built but behind `c_lang` feature flag. Not exposed to users.
- **Fix:** When `c_lang` feature is enabled, wire atomic section findings into advisory output. "Incomplete lock section: lock() at line 10 without matching unlock()" with suggestion to add unlock or use RAII.
- **Safety:** Test with C code that has lock/unlock mismatches. Verify findings are accurate.

---

## Corpus Expansion Strategy

Source: `CORPUS_CURATION_GUIDE.md` + `CORPUS_BAKING_STRATEGY.md`

**Target:** 400 patterns (200 Rust + 200 TypeScript) from current 30
**Architecture:** Multi-example files (4 functions per file) + baked fingerprint bundle

### Phase 0 — Infrastructure (Prerequisites)

#### S1 — Multi-Example Loader
- **Status:** Open
- **Priority:** High
- **Depends on:** (none)
- **Source:** CORPUS_BAKING_STRATEGY.md §Multi-Example Strategy, §Naming Convention for Multi-Example Files
- **Problem:** Current loader extracts first parseable function only. Multi-example files (4 functions per file) are not supported.
- **Fix:** Modify `src/engine/corpus/` (currently empty dir — needs implementation) to extract all parseable functions from a corpus file. Each function generates one fingerprint record under the same pattern name. Scoring takes max across all records.
- **Naming:** Filename does NOT change. `rust_command_injection_positive.rs` still contains positive examples. What changes is that it now contains 4 functions instead of 1. The loader discovers all functions, processes them all, stores them all under identifier `rust_command_injection`. Multi-function structure is internal detail.
- **Diversity:** 4 functions per file, each varying along a different dimension: number of hops (1 vs 3 vs through helper), syntactic form of source (query vs body vs path vs header), presence/absence of irrelevant intermediate computation, nesting depth (top-level vs inside conditional vs inside loop). 6 is reasonable. >8 has diminishing returns.
- **Source code impact:** `src/engine/corpus/` needs new `loader.rs`. `extract_fingerprints()` in `src/engine/fingerprint.rs:164` already extracts all functions — loader needs to call it per-file and aggregate.
- **Safety:** Add unit tests with 2-function and 4-function corpus files before modifying loader. Verify existing 30 patterns still load correctly.

#### S2 — Corpus Build Tooling
- **Status:** Open
- **Priority:** High
- **Depends on:** S1
- **Source:** CORPUS_BAKING_STRATEGY.md §Build-Time Extraction, §Bundle Format
- **Problem:** No tooling to extract fingerprints from source corpus into bundle format.
- **Fix:** Create `scripts/build-corpus-bundle.rs` (or Python) that: reads `corpus/targets/`, runs fingerprinting, outputs `frensense-corpus.frc` binary bundle.
- **Bundle format:** Header (format version, count of entries, SHA-256 checksum of full content) + sequence of pattern records. Each record: pattern identifier (e.g., `rust_command_injection_v1`), language tag, polarity flag (positive/negative), n-gram hash set (sorted 64-bit hashes), structural marker vector (7-8 small integers), MinHash signature bands, optional advisory text (severity, observation, impact, improvement).
- **What does NOT go in bundle:** Original source tokens or source text. Only hash representations. Fingerprinting is one-way — cannot reconstruct source from hashes.
- **Size estimate at 400 patterns:** 4 functions per positive file × 4 per negative file = 3,200 fingerprint records. Each record 500-1,500 bytes. Total bundle: 1.6-4.8 MB. Embedded in Rust binary with compression: ~300 KB-1 MB additional binary size.
- **Safety:** Validate bundle against existing 30 patterns. Compare fingerprints from bundle vs. live loading to confirm equivalence.

#### S3 — Bundle Embedding in Binary
- **Status:** Open
- **Priority:** High
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Embedding the Bundle
- **Problem:** No mechanism to embed pre-built corpus in binary.
- **Fix:** Add `include_bytes!("../frensense-corpus.frc")` in `src/bin/frensense.rs`. At startup, parse embedded bytes, load fingerprint records into index. Fallback to source directory if `--corpus` specified.
- **Safety:** Keep source directory as fallback. Engine loads bundle first, then adds `--corpus` patterns on top. `--list-patterns` shows source (built-in vs. custom).

#### S4 — Bundle Versioning
- **Status:** Open
- **Priority:** Medium
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Versioning the Bundle
- **Problem:** Fingerprinting algorithm changes break bundle compatibility.
- **Fix:** Embed format version in bundle header. Engine refuses to load bundle with version > its own version. Clear error message. Rebuild bundle when algorithm changes.
- **Safety:** Test with intentionally wrong version number to verify rejection.

### Phase 1 — Fix and Enrich Existing 30

Cross-references existing tasks C1-C7. These must complete before Phase 2.

#### C1 — Fill Open Redirect Positive Example
- **Status:** Open (already in tasks)
- **Priority:** High
- **Depends on:** B3

#### C2 — Improve ts_god_function Positive Example
- **Status:** Open (already in tasks)
- **Priority:** Medium

#### C3 — Strengthen ts_llm_any_parameter Pattern
- **Status:** Open (already in tasks)
- **Priority:** Medium

#### C4 — Strengthen ts_llm_console_log Pattern
- **Status:** Open (already in tasks)
- **Priority:** Medium

#### C5 — Improve rust_connection_leak Positive Example
- **Status:** Open (already in tasks)
- **Priority:** Medium

#### C6 — Improve ts_as_any_escape Pattern
- **Status:** Open (already in tasks)
- **Priority:** Low

#### C7 — Improve Minimal Hollow Validator Patterns
- **Status:** Open (already in tasks)
- **Priority:** Medium

**Exit criteria for Phase 1:** All 30 existing pairs have ≥15 lines in positive example, realistic context, no comments pointing to bug. `rust_clone_in_loop`, `ts_csa_validate_unconditional`, `ts_csa_sanitize_passthrough`, `ts_open_redirect` are enriched. Run Frensense on 1 real codebase — existing patterns should not false-positive on clean code.

### Phase 2 — Security Patterns (80 new pairs)

10 pairs per sub-domain × 8 sub-domains. Each pair has examples in both Rust and TypeScript. Work by sub-domain, not by language — write TypeScript version immediately after Rust version to maintain semantic focus.

**Validation checkpoint after each sub-domain:** Run Frensense on a real project known to have this bug class. If patterns fire on everything → positive too broad. If they fire on nothing → positive too narrow.

#### SP1 — Command Injection (10 pairs)
- **Status:** Open
- **Priority:** High
- **Variations:** req.query → exec, req.body → exec, template literal injection, destructured input, multi-step flow, req.body source, child_process.spawn (TS) / std::process::Command (Rust), helper function passthrough, format string, header value
- **Files:** `rust_sec_command_injection_{1-5}_positive.rs`, `ts_sec_command_injection_{1-5}_positive.ts` + negatives (each pair has both languages)
- **Safety:** Validate against real Express/Axum handlers. Confirm no false positives on hardcoded commands.

#### SP2 — SQL Injection (10 pairs)
- **Status:** Open (extends existing P1)
- **Priority:** High
- **Variations:** template literal → query, string concat → query, req.body.field → query, multi-hop through variable, req.params → query, format!() (Rust) / template literal (TS), req.query → raw query, header value → query, destructured body → query, helper function passthrough
- **Safety:** Validate against real DB query code.

#### SP3 — Path Traversal (10 pairs)
- **Status:** Open (extends existing P3)
- **Priority:** High
- **Variations:** req.params.filename → readFile, req.query.path → writeFile, path.join without normalize, req.body.dest → fs, header value → path, multi-hop through variable, destructured params, Rust std::fs with user path / TS fs.readFile with user path, helper function passthrough, relative path escape
- **Safety:** Validate against real file-serving code.

#### SP4 — Open Redirect (10 pairs)
- **Status:** Open (extends existing B3/C1)
- **Priority:** High
- **Variations:** req.query.url → redirect, req.body.next → redirect, req.headers.referer → redirect, req.params.url → redirect, multi-hop through variable, template literal redirect, Rust axum redirect / TS res.redirect, helper function passthrough, protocol-relative URL, javascript: URL
- **Safety:** Validate against real redirect handlers.

#### SP5 — SSRF (10 pairs)
- **Status:** Open (extends existing P5)
- **Priority:** High
- **Variations:** req.query.url → fetch, req.body.url → fetch, req.headers.host → fetch, req.params.url → fetch, multi-hop through variable, Rust reqwest with user URL / TS fetch with user URL, helper function passthrough, header value → fetch, destructured body → fetch, template literal URL
- **Safety:** Validate against real backend handlers.

#### SP6 — Prototype Pollution (10 pairs, TypeScript only)
- **Status:** Open (extends existing P2)
- **Priority:** High
- **Variations:** obj[key] = value, Object.assign without filtering, merge without key check, req.body → obj assignment, header value → obj, multi-hop through variable, lodash merge, destructured body → obj, nested object assignment, recursive merge
- **Safety:** Validate against real Express/Koa handlers.

#### SP7 — Hardcoded Secrets (10 pairs)
- **Status:** Open (extends existing ts_hardcoded_secret)
- **Priority:** High
- **Variations:** API key in string, JWT in string, AWS key in string, DB connection string, private key in string, secret in variable, env var fallback with hardcoded default, config object with secret, secret in comment, secret in log statement
- **Safety:** Validate against real config files.

#### SP8 — Credential Flow (10 pairs)
- **Status:** Open (new)
- **Priority:** High
- **Variations:** secret → log, secret → HTTP, secret → DB without hash, secret → response, password in variable → println! (Rust) / console.log (TS), token → log, API key → fetch, credential → debug log, secret → error message, password → redirect
- **Safety:** Validate against real auth code.

**Phase 2 exit criteria:** 80 new pairs. Each validated against ≥1 real codebase. No pattern fires on clean code in validation codebase. Total corpus: 110 pairs.

### Phase 3 — Hollow Implementation Expansion (60 new pairs)

Structurally similar patterns — batch writing ensures each is distinct.

#### HP1 — Unconditional Success Return (15 pairs)
- **Status:** Open
- **Priority:** High
- **Variations:** validateUser, validateToken, validatePayload, checkPermission, checkAccess, verifySignature, verifyRequest, verifyIdentity, sanitizeInput, sanitizeQuery, sanitizePath, filterInput, isAuthorized, isAuthenticated, isValid (Rust + TypeScript)
- **Safety:** Each positive must differ in variable names, intermediate steps, return expression. No two positives should have identical bodies except function name.

#### HP2 — Passthrough Return (15 pairs)
- **Status:** Open
- **Priority:** High
- **Variations:** sanitize function returns input unchanged, validate function returns input, clean function returns input, format function returns input, transform function returns input, normalize function returns input, encode function returns input, decode function returns input, parse function returns input, convert function returns input, sanitize (Rust), validate (Rust), clean (Rust), format (Rust), transform (Rust)
- **Safety:** Each negative must show genuine transformation/validation.

#### HP3 — Unreachable Rejection Path (15 pairs)
- **Status:** Open
- **Priority:** High
- **Variations:** wrong condition in if-check, always-true condition, negated logic, type mismatch in condition, always-true regex, always-false regex, wrong comparison operator, inverted boolean, constant condition, dead else branch, Rust Result never Err, Rust Option always Some, TypeScript Promise always resolve, TypeScript callback never error
- **Safety:** Each negative must show correct branching logic.

#### HP4 — Dead Validation (15 pairs)
- **Status:** Open
- **Priority:** High
- **Variations:** check result stored but never branched, validation result ignored, regex match stored but not tested, boolean flag set but never used, error caught but not propagated, warning logged but not acted, check performed but return value discarded, validation function called but return ignored, assertion without effect, lint check with no action
- **Safety:** Each negative must show branching on the check result.

**Phase 3 exit criteria:** 60 new pairs. Each positive has different function name, different variable names, different intermediate steps. Total corpus: 170 pairs (30 + 80 + 60).

### Phase 4 — LLM Anti-Pattern Expansion (70 new pairs)

Source material: real AI-generated code from coding assistants.

#### LP1 — Hallucinated API Patterns (20 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** .toJSON() on plain object, .toArray() on non-array, .serialize() on custom type, method that existed in older version, renamed method, removed API, framework-specific method on wrong type, incorrect generic parameter, wrong trait bound (Rust), missing feature gate (Rust)
- **Safety:** Validate against real Copilot/Claude output.

#### LP2 — Partial Error Handling (20 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** catch block that does nothing, Err arm identical to Ok, error logged but not propagated, error swallowed silently, try/catch wrapping non-throwing code, match on Result with identical arms, unwrap on recoverable error, expect with misleading message, panic in library code, error type that is never constructed
- **Safety:** Validate against real AI-generated code.

#### LP3 — Confidence Expression Mismatches (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** get_or_create always creates, find_or_default panics, safe_divide no zero check, validated_input not validated, sanitized_output not sanitized, filtered_list not filtered, checked_add overflow, bounded_value unbounded, optional_return always Some, error_handler no handling
- **Safety:** Validate against real AI-generated code.

#### LP4 — Copy-Paste Degradation (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** two handlers one validated, two functions one sanitized, two methods one checked, two branches one secured, two paths one logged, two exports one tested, two modules one audited, two endpoints one rate-limited, two callbacks one error-handled, two templates one escaped
- **Safety:** Validate against real codebases with copy-paste patterns.

**Phase 4 exit criteria:** 70 new pairs. Positive examples sourced from real AI-generated code. Total corpus: 240 pairs.

### Phase 5 — Architecture and Resource Management (50 new pairs)

#### AP1 — Resource Leaks (20 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** file handle without with/using, DB connection without close, HTTP connection without timeout, transaction without commit/rollback, memory in loop without dealloc, socket without close, lock without unlock, reader without close, writer without flush, stream without drop
- **Safety:** Validate against real resource management code.

#### AP2 — Architectural Boundary Violations (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** I/O in pure function, network call in DB transaction, domain logic in view, presentation in domain, infrastructure in application, framework coupling in core, global state in library, side effect in constructor, mutation in iterator, async in sync context
- **Safety:** Validate against real architectural patterns.

#### AP3 — Type System Escapes (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** @ts-ignore on real error, Object.assign dropping properties, ! non-null on nullable, as any on complex type, unsafe block without comment, transmute between sizes, raw pointer dereference, FFI without safety comment, dyn without object safety, PhantomData misuse
- **Safety:** Validate against real TypeScript/Rust code.

**Phase 5 exit criteria:** 50 new pairs. Positive examples from real postmortem notes. Total corpus: 290 pairs.

### Phase 6 — Concurrency Completion (60 new pairs)

Written last — requires highest-quality thinking about negative safety.

#### CP1 — Mutex Held Across Await (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** guard in let binding across await, guard as struct field with async method, Mutex<Vec<T>> iterated async, guard in tokio::spawn closure, guard across select!, guard held while calling async fn, guard across .await in loop, guard in Arc<Mutex> across await, nested mutex across await, guard in async trait method
- **Safety:** Each negative must be genuinely safe — not just different but correct.

#### CP2 — Blocking in Async (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** std::thread::sleep, std::fs::read_to_string, std::net::TcpStream::connect, std::sync::Mutex::lock, std::io::stdin().read_line, std::fs::write, reqwest::blocking::get, std::process::Command::output, serde_json::from_str (large), regex::Regex::new in loop
- **Safety:** Each negative must show correct async alternative.

#### CP3 — Missing Await (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** async fn called without await, promise not awaited, fire-and-forget async, async in map without await, async in forEach without await, async in filter without await, async in reduce without await, async function returned without await, async in conditional without await, async in callback without await
- **Safety:** Each negative must show proper await.

#### CP4 — Async Race Conditions (15 pairs)
- **Status:** Open
- **Priority:** Medium
- **Variations:** Promise.all with order dependence, concurrent writes to shared object, event listener mutating shared state, async iteration with shared counter, parallel DB reads with shared cache, concurrent file writes, race between timeout and completion, async initialization race, double-await on same promise, Promise.allSettled with missing error handling
- **Safety:** Each negative must show correct synchronization.

**Phase 6 exit criteria:** 60 new pairs. Negatives verified safe by expert review. Total corpus: 350 pairs.

### Phase 7 — Final 50 Pairs + Validation

#### VP1 — Final Pattern Batch (50 pairs)
- **Status:** Open
- **Priority:** Medium
- **Source:** Fill remaining slots with highest-value patterns from all categories based on validation data.
- **Safety:** Run full validation suite on all 400 patterns.

#### VP2 — Full Corpus Validation
- **Status:** Open
- **Priority:** High
- **Depends on:** All phases
- **Fix:** Run Frensense on `axum`, `actix-web`, `hyper`, `express`, `fastify`. Classify every finding as TP/FP. Target: ≥350 patterns with confirmed real-world TP, <5% FP rate on clean code.
- **Safety:** This is the final quality gate before shipping 400 patterns.

---

## Corpus Baking Implementation

### BB1 — Private Corpus Repository Setup
- **Status:** Open
- **Priority:** Medium
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Private Corpus Repository
- **Fix:** Create private repo for source files. Public repo contains only bundle + build tooling. Contributors submit PRs to private repo. Validation pipeline runs there.
- **Safety:** Ensure bundle rebuild is a one-command operation.

### BB2 — Transition Plan Execution
- **Status:** Open
- **Priority:** Medium
- **Depends on:** S3, BB1
- **Source:** CORPUS_BAKING_STRATEGY.md §Transition Plan
- **Fix:** Stage 1: bundle alongside source. Stage 2: new patterns only in bundle. Stage 3: remove public source directory.
- **Safety:** Keep `--corpus` backward compatibility throughout. Test with user-provided corpus at each stage.

### BB3 — Advisory Text in Bundle
- **Status:** Open
- **Priority:** Medium
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Advisory Text
- **Fix:** Store observation, impact, improvement fields in pattern record within bundle. Fixed at build time for built-in patterns. Sidecar .toml for custom patterns.
- **Safety:** Verify finding output matches between bundle-loaded and source-loaded patterns.

### BB4 — --corpus Backward Compatibility
- **Status:** Open
- **Priority:** High
- **Depends on:** S3
- **Source:** CORPUS_BAKING_STRATEGY.md §Backward Compatibility
- **Fix:** `--corpus my-rules/` processes source files through live fingerprinting pipeline. Built-in bundle + user corpus coexist in index. User patterns extend, not replace.
- **Safety:** Test with user corpus containing same pattern name as built-in. Verify max-score approach works.
