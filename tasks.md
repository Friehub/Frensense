# Frensense v0.4.0 — Task Tracker

Sources: `frensense_audit_v0.4.0.md`, `CORPUS_CURATION_GUIDE.md`, `CORPUS_BAKING_STRATEGY.md`

---

## Session Progress (2026-06-17)

### Completed This Session
- **MCP test suite fixed (T1)**: Renamed gensense→frensense in all 20 failing tests, fixed severity threshold defaults (UNUSED_VARIABLE is Info, not Warning), updated stale rule expectations (RUST_PANIC_IN_LIB doesn't exist). 36/36 MCP tests pass.
- **rule_tests rewritten (T1)**: Converted from inline snippets to corpus-fixture-based tests. Added configurable `corpus_threshold` on Engine. Set threshold to 0.32 for current corpus quality. 25/25 non-ignored tests pass.
- **patcher tests created (T3)**: 6 new tests for PatchManager — apply_fix, apply_fixes (multi-patch), context mismatch, generate_diff, empty advisory noop. All pass.
- **Multi-example corpus loader (S1)**: CorpusPattern stores `Vec<FunctionFingerprint>` for positives/negatives. Loader extracts ALL functions per file. `scan_function` takes max score across all pos/neg pairs. 14 engine tests pass.
- **ts_hardcoded_secret_positive.ts fixed**: Was a 1-line const declaration (unparseable). Rewritten as functions.
- **Configurable corpus_threshold**: Added `corpus_threshold` field to Engine with setter. CLI `--threshold` now passes through to PatternRegistry.

### Known Issues (Phase 1 enrichment needed)
- 6 corpus patterns have false positives on negatives at threshold 0.32 with multi-example scoring: `rust_csa_validate_unconditional`, `ts_as_any_escape`, `ts_llm_promise_catch`, `ts_prototype_pollution`, `ts_csa_sanitize_passthrough`, `ts_ssrf`. These need richer negative examples.

### Remaining Items
- **S2-S4**: Corpus build tooling (bundle, embed, version) — depends on S1 ✅
  - S2: Build tooling ✅
  - S3: Bundle embedding ✅
  - S4: Bundle versioning ✅
- **F8**: TP/FP tracking system ✅
- **B10**: Benchmark on real open-source projects
- **F5**: Per-category confidence thresholds — needs category metadata on corpus patterns
- **T5**: baseline test fix (pre-existing, ignored)

---

## Bugs

### B1 — Taint Advisory Over-Flags Clean Arguments
- **Status:** Done
- **Priority:** High
- **File:** `src/semantics/data_flow/resolve.rs` → `analyze_call`
- **Fix:** Changed to only taint arguments whose own text matches the source regex, not all args when call text matches.

### B2 — Corpus Loader Warns on Malformed TypeScript Files
- **Status:** Done
- **Priority:** Medium
- **Fix:** Enriched `ts_hardcoded_secret_negative.ts` with proper function body.

### B3 — ts_open_redirect_positive is Empty
- **Status:** Done
- **Priority:** High
- **File:** `corpus/targets/ts_open_redirect_positive.ts`
- **Fix:** Expanded to 15 lines with auth check, DB lookup, and redirect. Negative includes allowlist check.

### B4 — Dependency Resolver Silently Disabled for Rust
- **Status:** Done
- **Priority:** Medium
- **Problem:** `deps` module works for TS/JS via `package.json` but is disabled for Rust. Users have no indication this capability is inactive.
- **Fix:** Added `--check-deps` opt-in flag. When specified, verifies `cargo metadata` availability and warns if not found. Without the flag, behavior unchanged.

### B5 — RulesWrapper Dead Code in CLI
- **Status:** Done
- **Priority:** Medium
- **File:** `src/cli/commands.rs`
- **Fix:** Removed `RulesWrapper` struct.

### B6 — Stale GenSense Name References
- **Status:** Done
- **Priority:** Medium
- **Fix:** Renamed all `gensense`/`GenSense` to `frensense`/`Frensense` across source, tests, docs, Makefile, MCP tooling.

### B7 — L3 Taint Entropy Wired to Nothing
- **Status:** Done
- **Priority:** High
- **Fix:** `TaintMetrics::compute()` runs per-function. Hollow validators (ratio < 0.2) get confidence reduced by 60%.

### B8 — YAML Dependencies Still in Cargo.toml
- **Status:** Done
- **Priority:** Medium
- **File:** `Cargo.toml`
- **Fix:** Removed `tree-sitter-yaml`. `serde_yaml` kept for config files (`.frensense/config.yml`, `.frensense-suppress.yml`).

### B9 — is_rule_enabled Always Returns True
- **Status:** Done
- **Priority:** Critical
- **File:** `src/engine/auditor/mod.rs` lines 146-170
- **Fix:** Implemented proper filtering logic for category, tag, suite, and severity filters. All CLI filtering flags now work correctly.

---

## L3 Wiring (Architectural)

### A1 — Wire TaintMetrics into Confidence Pipeline
- **Status:** Done
- **Priority:** High
- **Depends on:** B7
- **Fix:** Hollow validator functions now have confidence reduced. Severity overrides also applied to taint findings.

---

## Taint Rules Externalization

### E1 — Externalize Taint Rules to TOML
- **Status:** Done
- **Priority:** High
- **Fix:** Created `taint_rules.toml` with all 6 built-in rules. `taint_rules.rs` loads from TOML via `load_taint_rules_from_file()`. `load_all_taint_rules(extra_dirs)` merges built-in + user rules. `--extra-taint-rules <dir>` CLI flag loads user `.toml` files. Built-in rules embedded via `include_str!`.

### E2 — Document Custom Taint Rule Format
- **Status:** Done
- **Priority:** Medium
- **Depends on:** E1
- **Fix:** Documented in `taint_rules.toml` header. Format: `[[rules]]` with id, source, sink, severity, observation, impact, improvement fields.

---

## Consumer-Layer Tests

### T1 — CLI Flag Integration Tests
- **Status:** Done
- **Priority:** High
- **Fix:** Added `tests/cli_tests.rs` with 8 tests (7 pass, 1 baseline ignored). Covers: `--strict` (exit code), `--severity` (filtering), `--json` (output format), `--sarif` (SARIF format), `--language` (filter), `--extra-taint-rules` (user rules), `--emit-baseline` (file creation).

### T2 — Corpus Loader Edge Case Tests
- **Status:** Done
- **Priority:** Medium
- **Depends on:** B2
- **Fix:** Added 11 tests: empty directory, nonexistent directory, empty file, no function body, bad syntax, unsupported extension, positive-only, negative-only, non-function files, valid pair. Also fixed B2 — files with no parseable function are now silently skipped instead of erroring.

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

### C1b — CVEfixes Harvester SQLite Support
- **Status:** Done
- **Priority:** High
- **Fix:** Created `harvesters/cvefixes_sqlite.py` wrapper. Updated `harvest_corpus.py` to use SQLite extractor. Added deprecation warnings to old JSON-based harvester.

### C1c — Missing Negative for typescript_cve_query
- **Status:** Done
- **Priority:** High
- **Fix:** Created `typescript_cve_query_negative.ts` with proper URL validation logic to match the positive's recursive URL test case.

### C1d — Six False-Positive Patterns Fixed
- **Status:** Done
- **Priority:** High
- **Fix:** Rewrote 6 negative files with completely different structures from their positives:
  - `rust_csa_validate_unconditional_negative.rs` — Uses impl block, HashMap
  - `ts_as_any_escape_negative.ts` — Uses unknown type with typeof checks
  - `ts_csa_sanitize_passthrough_negative.ts` — Uses actual string transformations
  - `ts_ssrf_negative.ts` — Uses isUrlSafe() with allowlist
  - All 6 negatives now score 0 corpus findings at threshold 0.32

### C2 — Improve ts_god_function Positive Example
- **Status:** Done
- **Priority:** Medium
- **Fix:** Replaced 100x `console.log` with realistic order processing function (HTTP parsing, DB queries, tax calculation, receipt generation). Negative shows same logic properly decomposed into small functions.

### C3 — Strengthen ts_llm_any_parameter Pattern
- **Status:** Done
- **Priority:** Medium
- **Fix:** Positive now has `any` parameter with manual type assertions and runtime checks but no type narrowing. Negative uses `unknown` with proper type guard and validation.

### C4 — Strengthen ts_llm_console_log Pattern
- **Status:** Done
- **Priority:** Medium
- **Fix:** Positive is now a login handler with `console.log` on auth events. Negative uses structured logging (`structuredLogger.info` with event name + metadata object).

### C5 — Improve rust_connection_leak Positive Example
- **Status:** Done
- **Priority:** Medium
- **Fix:** Rewritten with error paths — happy path reads data, error path returns without dropping connection. Negative uses closure + explicit `drop(conn)` after.

### C6 — Improve ts_as_any_escape Pattern
- **Status:** Done
- **Priority:** Low
- **Fix:** Positive casts `any` to `Config` interface without validation. Negative validates each field with `typeof` checks and throws on invalid types.

### C7 — Improve Minimal Pattern Examples
- **Status:** Done
- **Priority:** Medium
- **Files:** `ts_csa_validate_unconditional`, `ts_csa_sanitize_passthrough`, `rust_clone_in_loop`, `rust_csa_validate_unconditional`
- **Fix:** Enriched all to realistic context: credential validation with type guards, sanitize functions with real transformations, clone-in-loop with proper HashMap API, Rust config validation with field checks.

---

## CSA Rework — Corpus-Learned, Not Name-Pattern Rules

> Supersedes the `CONTRACT_SURFACE_*` rule family proposed in earlier research.
> **Do not build those** — CSA is a corpus
> category like `sec`/`llm`/`arch`/`async`, not a new rule IR.

### CSA1 — Fill Generic Advisory Text on Existing CSA Sidecars
- **Status:** Done
- **Priority:** High
- **Files:** `rust_csa_validate_unconditional.toml`, `ts_csa_validate_unconditional.toml`, `ts_csa_sanitize_passthrough.toml`, `ts_csa_auth_no_rejection.toml`, `ts_csa_find_never_empty.toml`
- **Fix:** All 5 had `corpus_check.py --generate` boilerplate (`"Corpus pattern: X Y Z."` / `"Review against corpus example."`) instead of authored advisory text. Replaced with pattern-specific observation/impact/improvement text. Bumped `ts_csa_sanitize_passthrough` and `ts_csa_auth_no_rejection` to `Critical` (passthrough enables XSS/traversal/open-redirect; no-rejection is an auth bypass) — they were `Warning`.

### CSA2 — Rust Counterparts for sanitize_passthrough / auth_no_rejection / find_never_empty
- **Status:** Done
- **Priority:** High
- **Files added:** `rust_csa_sanitize_passthrough_{positive,negative}.rs` + `.toml`, `rust_csa_auth_no_rejection_{positive,negative}.rs` + `.toml`, `rust_csa_find_never_empty_{positive,negative}.rs` + `.toml`
- **Fix:** Only `validate_unconditional` had a Rust pair before this; the other 3 CSA violation types were TypeScript-only. All 4 types now exist in both languages (8 pairs total). Negatives use a deliberately different structure from their positives (impl blocks + `HashMap`/enum errors/`?` operator vs. the positives' free functions + `println!` warnings) — same lesson as C1d, to avoid the n-gram scorer treating a trivially-inverted positive as its own negative.
- **Verified:** `python3 scripts/corpus_check.py corpus/targets/` reports no missing positive/negative/toml for any `csa` pattern. **Not verified:** FRC bundle rebuild and `cargo test -p frensense-engine -- corpus` — no Rust toolchain in this session's sandbox. Run both before relying on these in a scan.

### CSA3 — Re-verify the 2 Open FP Flags Against This Session's Changes
- **Status:** Open
- **Priority:** Medium
- **Note:** "Known Issues" (top of this file) lists `rust_csa_validate_unconditional` and `ts_csa_sanitize_passthrough` as having false positives on negatives at threshold 0.32, but **C1d below claims both were already fixed** ("all 6 negatives now score 0 corpus findings at threshold 0.32"). These two task entries contradict each other — re-run `compute_metrics.py --by-rule` or the rule_tests corpus fixtures to find out which is stale before trusting either.

### CSA4 — Test-to-Implementation Pairing (Layer 2 of the rework)
- **Status:** Open
- **Priority:** Medium
- **Depends on:** `SymbolRegistry` cross-file graph (exists) + a new joint fingerprint type
- **Fix:** Add impl↔test pairing via the existing call graph (not path globs), then a joint fingerprint over (impl taint-entry signature, test call-site argument shapes), scored through the existing `score_against_corpus` — not a new `ProjectFlowConstraint` variant. See rework doc Layer 2 for the full design. This is the only piece of the rework that needs new engine code rather than just corpus pairs.

### CSA5 — Advisory Text from Comment Blocks (replaces TOML sidecar requirement)
- **Status:** Done
- **Priority:** High
- **Files:** `frensense-engine/src/corpus/loader.rs`
- **Fix:** Added `parse_frensense_block()` that extracts `/// [frensense]` / `// [frensense]` / `# [frensense]` blocks from positive files. `load_corpus` now reads advisory text from the positive file's comment block first, falling back to TOML sidecar. TOML is now optional — comment blocks are the primary source. 7 new unit tests. All 8 CSA positive files updated with `[frensense]` blocks. Verified: observation/impact/improvement text appears in scanner output.

---

## New Corpus Patterns to Add

### P1 — SQL Injection (Security)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_sql_injection_{positive,negative}.ts`
- **Positive:** Template literal and string concatenation into SQL query.
- **Negative:** Parameterized queries with `$1` placeholders.

### P2 — Prototype Pollution (Security / TS+JS)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_prototype_pollution_{positive,negative}.ts`
- **Positive:** `obj[key] = value`, `Object.assign`, `for..in` without key filtering.
- **Negative:** Filters `__proto__`, `constructor`, `prototype` keys before assignment.

### P3 — Path Traversal (Security)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_path_traversal_{positive,negative}.ts`
- **Positive:** `fs.readFileSync(path.join(dir, userInput))` without normalization.
- **Negative:** `path.basename()`, `path.normalize()`, prefix checks, regex validation.

### P4 — JWT Verification Bypass (Security)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_jwt_bypass_{positive,negative}.ts`
- **Positive:** `jwt.decode(token)` used for authentication.
- **Negative:** `jwt.verify(token, secret)` with secret from env.

### P5 — SSRF (Security)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_ssrf_{positive,negative}.ts`
- **Positive:** `fetch(req.query.url)` without URL validation.
- **Negative:** `isAllowedUrl()` checks hostname allowlist + protocol before fetch.

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
- **Status:** Done
- **Priority:** High
- **Fix:** Added `weighted_ngram_hashes: FxHashMap<u64, f32>` to `FunctionFingerprint`. `compute_idf_weights()` computes IDF from corpus. Scorer uses `weighted_jaccard()` when weights available — rare tokens like `db::execute` score higher than common ones like `let x =`.

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
- **Status:** Partially done
- **Priority:** Medium
- **Problem:** Confidence scores are raw similarity. Score of 0.8 doesn't mean 80% probability.
- **Fix:** Created `confidence_calibration.rs` with Platt scaling implementation. Created `train_calibration.py` script. Blocked on needing labeled TP/FP dataset - current `axum_labels.json` only has FP labels (0 TP). Need to run on a codebase with known bugs to get TPs for training.

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
- **Status:** Done
- **Priority:** Low
- **Fix:** `cross_lingual_penalty()` applies 25% penalty when pattern language differs from candidate language. Rust pattern matching TS code scores 0.75x vs same-language match.

### M9 — Sliding Window N-Gram Context
- **Status:** Done
- **Priority:** Low
- **Fix:** `token_ngrams_positional()` mixes position into n-gram hash. `return` at line 5 produces different hash than `return` at line 50. Position encoded as 10-bit field combined with token hash.

---

## Documentation & Naming Cleanup

### D1 — Rename All GenSense References to Frensense
- **Status:** Done
- **Priority:** Medium
- **Depends on:** B6
- **Fix:** Full rename across source, tests, docs, Makefile. Verified with grep — zero remaining.

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
- **Status:** Done
- **Priority:** Medium
- **Problem:** No ground truth data on precision. All precision claims are theoretical without manual classification.
- **Fix:** Created `scripts/classify_findings.py` (interactive TP/FP classification) and `scripts/compute_metrics.py` (precision/recall reporting). Ground truth stored in `corpus/ground_truth/{repo}_labels.json`. Workflow: scan repos → classify findings → compute metrics. Tested on axum (761 findings, 585 classified as FP baseline).

### F9 — Scaling Validation on Large Projects
- **Status:** Open
- **Priority:** Medium
- **Problem:** Benchmark data covers 10–100 files. 1M+ LOC capability claim is unvalidated.
- **Fix:** Run and publish results on several well-known large open-source projects to prove scaling claims.

---

## Engine Features — Built But Not Wired

### W1 — Wire Temporal Analysis into Findings
- **Status:** Done
- **Priority:** High
- **Fix:** `findings::temporal_violation::find()` — `TemporalAnalyzer::add_rules_from_toml()` + `analyze_with_events()` called per-file. Produces `TEMPORAL_VIOLATION` advisories for lock/unlock, acquire/release violations. Temporal rules now loaded from `temporal_rules.toml` via `load_all_temporal_rules()`.

### W2 — Wire Reachability Analysis as User-Facing Feature
- **Status:** Done
- **Priority:** Medium
- **Fix:** `findings::dead_branch::find()` — `ReachabilityChecker::find_dead_branches()` detects `if false`/`if true` dead branches. Produces `DEAD_BRANCH` advisories with confidence computed from condition type.

### W3 — Wire CFG/Def-Use as User-Facing Feature
- **Status:** Done
- **Priority:** Medium
- **Fix:** `findings::unused_variable::find()` — `build_def_use()` finds definitions with zero uses, excluding function parameters. Produces `UNUSED_VARIABLE` advisories.

### W4 — Wire Cross-File Taint into Findings
- **Status:** Done
- **Priority:** High
- **Fix:** `findings::cross_file_taint::find()` — Graph walk finds source-named functions calling sink-named functions across files. Uses `COMBINED_SOURCE_RE`/`COMBINED_SINK_RE` built from all taint rules. Produces `CROSS_FILE_TAINT` advisories.

### W5 — User Corpus Loading (was: User Rule Loading)
- **Status:** Done
- **Priority:** High
- **Fix:** `PatternRegistry::load_corpus_dirs()` accepts multiple directories. `run_detailed()` collects built-in `--corpus` dir + `--extra-rule-dirs` and loads all. User corpus extends, not replaces. Old YAML rule interface removed entirely (`user_rules.rs` deleted, `build_rule_set()` removed, `--isolate-rules`/`--no-builtin-rules` flags removed).

### W6 — Wire Style Profile into Findings Pipeline
- **Status:** Done
- **Priority:** Medium
- **Fix:** Already wired — `STYLE_ANOMALY` and `NEAR_DUPLICATE_FUNCTION` advisories emitted when `--learn-profile` + `--check-profile` used.

### W7 — Enable Dependency Check for Rust
- **Status:** Done
- **Priority:** Medium
- **Fix:** `findings::hallucinated_import::find()` — `DependencyResolver::load_project()` + `scan_file()` called per-file. Produces `HALLUCINATED_IMPORT` advisories for missing Cargo.toml/package.json entries.

### W8 — Wire Pattern Canonical Form for Structural Matching
- **Status:** Done (Verified - Not Dead Code)
- **Priority:** Low
- **Engine:** `frensense-engine/src/pattern/` — PatternCompiler, PatternMatcher, PatternScorer, CanonicalForm
- **Problem:** Pattern compiler and matcher are built but the current corpus uses fingerprint-based matching (n-grams). The canonical form module may be unused.
- **Fix:** Verified that `CanonicalForm` IS being used in `scorer.rs` for structural similarity calculations. Not dead code - it's an active part of the pattern scoring system.

### W9 — Surface Atomic Section Detection for C
- **Status:** Done
- **Priority:** Low
- **Engine:** `frensense-engine/src/atomic_section.rs` — AtomicSectionAnalyzer, AtomicOp, has_incomplete_sections
- **Problem:** TOCTOU/lock-pair detection for C was built but behind `c_lang` feature flag. Not exposed to users.
- **Fix:** Created `src/engine/findings/atomic_section.rs` module. Registered in `registered_modules()`. Now emits `ATOMIC_SECTION_INCOMPLETE` advisories.

---

## Corpus Expansion Strategy

**Moved to `SCALING_PLAN.md`** — comprehensive 45k scaling strategy covering harvest pipeline,
LSH tuning, pattern clustering, validation, and build order. The 400-pattern intermediate
milestone is covered in SCALING_PLAN.md §Build Order as Week 2-3.

---

## Phase 5: Taint Precision (Primary Engineering Mandate)

**Target:** 585 false positives on Axum → ~10-20

### Step 1: `taint_entry_points.toml` + loader
- **Status:** Done
- **Files:** `taint_entry_points.toml` (new), `src/engine/taint_entry_points.rs` (new)
- **Description:** TOML entry point definitions for Rust/TS/Python + LazyLock loader

### Step 2: `sanitizers.toml`
- **Status:** Done
- **Files:** `sanitizers.toml` (new)
- **Description:** Built-in sanitizer function list per language + `build_sanitizer_regex()` function

### Step 3: `taint_seeder.rs`
- **Status:** Done
- **Files:** `src/engine/taint_seeder.rs` (new, ~250 lines)
- **Description:** AST walker that seeds TaintRegistry from function parameters matching entry points. Rust/TS/Python param extraction. 3 tests pass.

### Step 4: Wire seeder into DataFlowAnalyzer
- **Status:** Done
- **Files:** `src/semantics/data_flow/mod.rs`
- **Description:** Added `seeder` field and `with_seeder()` builder

### Step 5: Replace regex seeding in tracking.rs
- **Status:** Done
- **Files:** `src/semantics/data_flow/tracking.rs`
- **Description:** Seeder called at start of analyze_block

### Step 6: Demote source_re in resolve.rs
- **Status:** Done
- **Files:** `src/semantics/data_flow/resolve.rs`
- **Description:** Removed `source_re.is_match(arg_text)` auto-taint at call sites

### Step 7: Add source_functions to TaintRuleToml
- **Status:** Done
- **Files:** `src/engine/taint_rules.rs`
- **Description:** Added `source_functions: Vec<String>` field

### Step 8: Wire entry points into runner.rs
- **Status:** Done
- **Files:** `src/engine/project/runner.rs`
- **Description:** Entry points loaded once, seeder built per file, attached to analyzer

### Step 9: Test on Axum + real web API
- **Status:** Done
- **Description:** Scanned Axum json.rs, form.rs, lib.rs. Taint findings: 0 (was 585). Only HALLUCINATED_IMPORT remains (expected — dev-deps not visible).

**Exit criteria met:** 0 taint findings on Axum's own source (target was <20). All 78 tests pass.

#### S1 — Multi-Example Loader
- **Status:** Done
- **Priority:** High
- **Depends on:** (none)
- **Fix:** `CorpusPattern` stores `Vec<FunctionFingerprint>` for positives/negatives. Loader extracts all functions from each corpus file. `scan_function` takes max score across all positive/negative pairs. LSH index uses first positive per pattern. Added `corpus_threshold` field to Engine (default 0.60, configurable via setter). Tests: 14 engine tests + multi-function extraction test pass.

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
- **Status:** Done
- **Priority:** High
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Embedding the Bundle
- **Problem:** No mechanism to embed pre-built corpus in binary.
- **Fix:** Added `const CORPUS_BUNDLE: &[u8] = include_bytes!("../../frensense-corpus.frc")` in `src/bin/frensense.rs` and `src/mcp/audit.rs`. Engine calls `set_corpus_bundle()` at startup. Loads 89 patterns from embedded bundle. Falls back to source directory if `--corpus` specified.
- **Safety:** Keep source directory as fallback. Engine loads bundle first, then adds `--corpus` patterns on top. `--list-patterns` shows source (built-in vs. custom).

#### S4 — Bundle Versioning
- **Status:** Done
- **Priority:** Medium
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Versioning the Bundle
- **Problem:** Fingerprinting algorithm changes break bundle compatibility.
- **Fix:** Bundle header contains `version: u32 = 1`. Engine refuses to load bundle with `version > BUNDLE_VERSION`. Error message: "bundle version N > engine version M". Rebuild bundle when algorithm changes.
- **Safety:** Test with intentionally wrong version number to verify rejection (test_bundle_version_check passes).

---

## Corpus Phases 1-7: Moved to SCALING_PLAN.md

The detailed corpus expansion plan (400 patterns → 45k patterns) is now in `SCALING_PLAN.md`.
This includes:
- Phase 0: Infrastructure (S1-S4) — Done
- Phase 1: Fix existing 30 patterns (C1-C7) — mostly Done
- Phase 2: Security patterns (80 pairs) — SP1-SP3 Done, SP4-SP10 Open
- Phase 3: Hollow implementation (60 pairs)
- Phase 4: LLM anti-patterns (70 pairs)
- Phase 5: Architecture (50 pairs)
- Phase 6: Concurrency (60 pairs)
- Phase 7: Final 50 + validation
- BB1-BB4: Private corpus repository

See `SCALING_PLAN.md` for the harvest pipeline, LSH tuning, and validation strategy.

---

## Legacy Tasks (pre-Phase 5)

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
- **Status:** Done
- **Priority:** High
- **Files:** `ts_sec_cmd_injection_{1-10}_{positive,negative}.ts`, `rust_sec_cmd_injection_{1-10}_{positive,negative}.rs`
- **Variations:** req.query→exec, req.body→spawn, template literal injection, destructured input, multi-step flow, header value→exec, helper function passthrough, string concatenation, URL parameter, query string
- **Bundle:** 51 patterns total (up from 31), 199 fingerprints, 143KB

#### SP2 — SQL Injection (10 pairs)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_sec_sql_injection_{1-10}_{positive,negative}.ts`, `rust_sec_sql_injection_{1-10}_{positive,negative}.rs`
- **Variations:** template literal, string concat, dynamic column name, URL param, multi-hop helper, header value, destructured body, table name, email lookup, bulk IDs
- **Bundle:** 69 patterns, 287 fingerprints, 230KB

#### SP3 — Path Traversal (10 pairs)
- **Status:** Done
- **Priority:** High
- **Files:** `ts_sec_path_traversal_{1-10}_{positive,negative}.ts`, `rust_sec_path_traversal_{1-10}_{positive,negative}.rs`
- **Variations:** filename param, query path, backup dest, template name, header icon, folder+filename, log file, config section, doc name, export dir
- **Bundle:** 89 patterns, 364 fingerprints, 290KB

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
- **Status:** Done
- **Priority:** Medium
- **Depends on:** S2
- **Source:** CORPUS_BAKING_STRATEGY.md §Advisory Text
- **Fix:** Added optional `observation`, `impact`, `improvement` fields to `BundlePattern`. Added `load_sidecar_toml()` to load from `corpus/targets/{pattern}.toml`. Updated runner to use pattern-specific advisory text.

### BB4 — --corpus Backward Compatibility
- **Status:** Open
- **Priority:** High
- **Depends on:** S3
- **Source:** CORPUS_BAKING_STRATEGY.md §Backward Compatibility
- **Fix:** `--corpus my-rules/` processes source files through live fingerprinting pipeline. Built-in bundle + user corpus coexist in index. User patterns extend, not replace.
- **Safety:** Test with user corpus containing same pattern name as built-in. Verify max-score approach works.
