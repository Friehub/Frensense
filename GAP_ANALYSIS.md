# Gap Analysis → Build Plan

Verified 2026-05-24 against commit `ae315d6`. Phases ordered by dependency — each phase unblocks or accelerates the next.

---

## ✅ Already Done (No Action Needed)

| Item | Evidence |
|------|----------|
| `body_must_contain` uses AST `ReachabilityChecker` (not raw regex) | `src/rules/ir/core.rs` — fixed in v0.3.1 commit `6f4274a` |
| Advisory agent fields (`confidence`, `auto_fixable`, `requires_human`) | `src/lib.rs:110,125,126` — present in `Advisory` struct |
| Native TypeScript `TS_TAUTOLOGICAL_ASSERT` rule | `src/rules/typescript/ts_tautological_assert.rs` — registered, 7 test cases |
| `SemanticGraph` exposed to all rules via `GenSenseContext` | `src/lib.rs:219`, commit `e1d3e1c` |
| Taint flow materialized as `TaintFlow` graph edges | `src/semantics/graph.rs:22-275`, commit `e1d3e1c` |
| Algebraic constraint combinators (`AllOf`, `Across`, `Without`, etc.) | `src/rules/ir/flow.rs` + `FlowEvaluator`, commit `e1d3e1c` |
| **Phase 1 (v0.4.0 style profile) — Core implemented** | — |
| `FunctionFingerprint` with 7 feature types | `src/engine/fingerprint.rs` |
| `ProjectProfile` with per-language frequency maps + JSON serialization | `src/engine/profile.rs` |
| `style_surprise()` scoring with configurable threshold | `src/engine/profile.rs` |
| File-level profile isolation (src/ vs tests/) | `src/engine/profile.rs` |
| CLI: `--learn-profile`, `--check-profile`, `--profile-threshold`, `--profile-stats` | `src/cli/options.rs` |
| Engine API: `with_profile()`, `profile()`, `set_profile_threshold()` | `src/engine/project/builder.rs` |
| `STYLE_ANOMALY` advisories in `run_detailed()` | `src/engine/project/runner.rs` |
| `find_profile()` — walks parent dirs for `.gensense/profile.json` | `src/cli/extras.rs` |
| **Self-audit clean** | — |
| 5 `FILE_TOO_LONG` violations resolved | `src/rules/ir/` split into 5 files; `src/bin/gensense.rs` → `src/cli/`; `src/bin/gensense-mcp.rs` → `src/mcp/`; `src/engine/project/mod.rs` → builder/runner/files; `src/semantics/data_flow/tracking.rs` → resolve/handlers |
| All 109 tests pass, clippy clean, fmt clean | commit `ad94b0f` |

---

## Phase 0 — Taint Engine Soundness Fixes

**Why first:** These are correctness bugs in the core taint analysis pipeline. Every taint-based rule (existing and future) depends on this engine. Without these fixes, deeper analyses (style profile, CSA) may produce false negatives on anything involving deep call chains, re-analyzed callees, or match arm returns.

### 0a. Configurable `taint_max_depth` per rule

`max_depth: 5` is hardcoded at `src/semantics/data_flow/mod.rs:170`. A YAML rule cannot override it. Taint through a 6-deep call chain silently stops tracking with no warning.

- [x] **Add `taint_max_depth: Option<usize>`** to `CoreRule` and `CoreRuleIr`
- [x] **Pass through in `evaluate_taint_constraint()`** when constructing `DataFlowAnalyzer`
- [x] **YAML field** — `taint_max_depth: 8` for SSRF rules, omit for default (5)

### 0b. Visited-set in `resolve_call_taint()`

`resolve_call_taint()` at `src/semantics/data_flow/tracking.rs` re-analyzes the same callee on every taint path. No cycle detection — mutually recursive functions can loop infinitely.

- [x] **Add `HashSet<(file_path, start_byte)>`** through the recursion to prevent re-analysis and detect cycles
- [x] **Soundness improvement** — not a capability gap, but prevents silent infinite loops

### 0c. Match arm return propagation in `find_returns()`

`find_returns()` recurses through most node kinds but stops at nested function definitions (`fn`, `\| \|`, `method`). Returns inside closures that are immediately invoked (`(\|\| { return tainted; }())`) are missed — ~20 lines to fix.

- [x] **Add `if_expression` and `match_expression`** as explicit cases that extract taint from each arm's last expression

### Deferred: Taint lifetime model redesign

`TaintRegistry<'a>` holds references tied to a single tree-sitter tree. Cross-file callee analysis creates a fresh registry — caller taint state (globals, closures, out-params) does not propagate. Fixing this requires changing the `'a` lifetime model throughout `src/semantics/data_flow/`. **Deferred to v0.5.0.**

---

## Phase 1 — v0.4.0 Core: Style-Anomaly Detection ✅ (Completed)

**Completed 2026-05-26 at commit `f9a051d`.** See `src/engine/profile.rs` for `ProjectProfile`, `src/engine/fingerprint.rs` for `FunctionFingerprint`, `src/cli/` for CLI flags.

### Remaining Phase 1 items (low priority, deferred)

- [ ] `gensense . --check-profile --diff-only` — score only new/changed files
- [ ] `gensense . --check-profile --strict` — fails if any function exceeds threshold
- [ ] Baseline regeneration post-merge
- [ ] Acceptance criteria: deterministic profile tests, LLM function > 0.5, normal < 0.3

---

## Phase 2 — Rule Coverage Hardening

**Why third:** Existing rules have coverage gaps. These are quick wins (no new infrastructure) that raise the reliability baseline before adding new capabilities.

### 2a. CSA Rule Test Coverage

All CSA rules now have corpus fixtures, automated tests, and suppression tests.

| Rule | Fixture | Test | Status |
|------|---------|------|--------|
| `RUST_CSA_VALIDATE_UNCONDITIONAL` | ✅ | ✅ | Done |
| `TS_CSA_VALIDATE_UNCONDITIONAL` | ✅ | ✅ | Done |
| `TS_CSA_AUTH_NO_REJECTION` | ✅ | ✅ | Done |
| `TS_CSA_SANITIZE_PASSTHROUGH` | ✅ | ✅ | Done |
| `TS_CSA_FIND_NEVER_EMPTY` | ✅ | ✅ | Done |

> **Note:** `SOL_*` (Solidity) rules were removed from the codebase in commit f527355. No coverage needed.

- [x] **Corpus fixtures** — all TS/RS CSA rules have positive + negative fixture pairs
- [x] **`run_test()` calls** — all 5 CSA rules test both positive and negative paths
- [x] **`body_may_delegate_via`** — delegation suppression test added for `TS_CSA_VALIDATE_UNCONDITIONAL`

### 2b. Fix `deadlock_guard.rs` Byte-Scan

`deadlock_guard.rs` walks raw source bytes for `.lock` before `.await` — never calls `TemporalAnalyzer::check_temporal()`. False positive when guard is dropped by scope exit before the await:

```rust
{ let _guard = mutex.lock().unwrap(); }  // dropped here
something().await;  // safe — temporal analyzer would reset found_first = false
```

- [x] **Replace byte-scan** with call to `TemporalAnalyzer::check_temporal()` using events from `ordered_events_in_scope()`

### 2c. Three Temporal Rules

The `TemporalAnalyzer` at `src/semantics/temporal.rs` has three behaviors, zero consumers. Unlock them:

- [x] **`RUST_CONNECTION_LEAK`** — `MustFollow`: `get_connection`/`acquire` not followed by `close`/`release`/`drop`
- [x] **`RUST_NETWORK_IN_TXN`** — `ForbiddenBetween`: `fetch`/`http` between `begin_transaction` and `commit`/`rollback`
- [x] **`RUST_MUTATE_AFTER_RESPONSE`** — `MustNotFollow`: `write`/`modify` after `send_response`/`reply`/`commit`
- [x] **Tests** — corpus fixtures + `run_test()` for all three temporal rules

---

## Phase 3 — v0.4.0 Appendices: Advanced Constraints

**Why fourth:** These are larger features (19h total) that build on the hardened Phase 0–2 foundation. They introduce genuinely new detection capabilities.

### 3a. `AtomicSection` Constraint (11.5h)

A new `ProjectFlowConstraint` variant detecting TOCTOU races: reads of mutex-protected variables occurring outside the lock.

- [ ] **Phase 1 — Lock-set construction AST pass** — map `{variable → set of protecting mutexes}`
- [ ] **Phase 2 — Read-outside-lock detection** — flag reads of protected vars outside lock/unlock spans
- [ ] **Phase 3 — Condition variable pairing** — verify `pthread_cond_signal`/`wait` hold the associated mutex
- [ ] **YAML DSL** — `AtomicSection { shared_variable, guard_mutex }`
- [ ] **C target** — tree-sitter-c grammar support
- [ ] **Tests** — sleeping barber fixture

### 3b. SRI Diff-Only Baselines (7.5h)

Filter advisories to only symbols changed in the current branch vs main, using Symbol-Relative Identity fingerprints.

- [ ] **Git diff detection** for changed symbols (`SymbolEntry.git_blob_oid`)
- [ ] **`.gensense/baseline.json`** — SRI-anchored advisory fingerprints from `main`
- [ ] **Baseline suppression** — advisories matching baseline are excluded from output
- [ ] **`--diff-only`** and **`--diff-base`** CLI flags
- [ ] **`--update-baseline`** CI integration
- [ ] **Tests** — git-aware fixtures with branch switching

### 3c. `--severity` Pre-Filter

Deferred from v0.3.x. Push `--severity` into the rule dispatcher so rules below threshold are never evaluated.

- [x] **Rule dispatcher filter** — skip rules below severity threshold before evaluation

### 3d. Algebraic Constraint Combinators ✅ (Completed in SPG Phase 6c)

**Completed 2026-05-26** — `across_boundary`, `all_of`, `any_of`, `not`, `without_constraint`/`without_exclusion` YAML fields. `FlowEvaluator` with recursive constraint tree evaluation. No new analysis engine needed — composes existing `FlowConstraint` leaves.

- [x] `FlowConstraint::Across`, `AllOf`, `AnyOf`, `Not`, `Without`, `Chain` variants (`src/rules/ir.rs:36-52`)
- [x] `FlowEvaluator` (`src/rules/ir.rs:1203-1345`)
- [x] YAML fields on `CoreRule` (`src/rules/core/mod.rs:78-89`)
- [x] Compilation in `RuleCompiler` (`src/rules/compiler.rs:78-133`)

---



## Phase 4 — Advanced Analysis Algorithms

**Why fifth:** These are medium-to-high risk additions that build on the Phase 1 style profile infrastructure. They add significant new detection capabilities but require careful testing.

### 4a. MinHash / LSH on N-gram Fingerprints

Prerequisites confirmed (`FunctionFingerprint.ngram_hashes`, `post_process_ngrams`). Add LSH for similarity estimation.

- [ ] **`MinHashSignature { bands: Vec<u64> }`** with k=128 universal hash functions
- [ ] **LSH bucket grouping** by band hash
- [ ] **Project-level rule pass** — emit advisory when Jaccard > 0.75 AND functions have different taint sinks

### 4b. Datalog Fixed-Point on Call Graph

Prerequisites confirmed (`SemanticGraph`, `has_call_path()`). Replace per-query DFS with pre-computed transitive closure.

- [ ] **Compute transitive closure** once per project scan, cache in `AnalysisRegistry`
- [ ] **Replace `has_call_path()`** with set-lookup on the closure
- [ ] **`must_pass_through` YAML field** → new `ProjectFlowConstraint` variant
- [ ] **Compose path predicates** — "source reaches sink AND passes through guard"

### 4c. Taint Entropy / Branch Coverage

Highest-risk addition because it changes `analyze_block()` return type. New rule `AI_TAINT_BYPASS`.

- [ ] **`TaintMetrics { tainted_uses, taint_branched_on, taint_branch_ratio }`**
- [ ] **Compute during `analyze_block`** — count conditionals touching tainted variables
- [ ] **`min_taint_entropy` YAML field**
- [ ] **`AI_TAINT_BYPASS` rule** — fires when `taint_branch_ratio < 0.2` AND function name implies validation

---

## Phase 5 — Developer Experience

**Why last:** These improve adoption and usability but don't add detection capability. They depend on the CLI and YAML compiler being stable.

### 5a. Rule Wizard (`gensense rule new`)

6-step interactive wizard wired to three existing code paths:

| Building Block | Location | Role |
|---|---|---|
| `handle_debug_ast` | `src/bin/gensense.rs:192` | Filter AST nodes at a specific line → user picks `on_node` |
| `handle_test_rule` | `src/bin/gensense.rs:77` | Compile YAML, run on fixture, confirm rule fires |
| `RuleMetadata` | `src/lib.rs:69` | Every interactive input maps 1:1 to a YAML field |

- [ ] **5a-1. `gensense rule new` subcommand** — 6-step flow (language, example code → node picker, pattern auto-escape, identity, advisory text, verify)
- [ ] **5a-2. Node kind resolver** — parse input line, return every named node spanning it, deduplicated by kind
- [ ] **5a-3. Regex auto-escaper** — plain text → regex-safe pattern (`.` → `\.`, `(` → `\(`)
- [ ] **5a-4. Verification step** — compile draft YAML, run on user fixture, report pass/fail with line numbers
- [ ] **5a-5. `gensense test-rule` standalone command** — re-test rules without re-running wizard

---

## Appendix A — Capability Comparison

GenSense vs existing tool categories. Rows marked with `✗` are genuine gaps; `~` is partial support.

| Capability | Formatter | Style linter | Pattern scanner | Static analyzer | Semantic analyzer | GenSense |
|---|---|---|---|---|---|---|
| Single-node AST match | ✗ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Intra-function taint | ✗ | ✗ | ✗ | ✓ | ✓ | ✓ |
| Temporal event ordering | ✗ | ✗ | ✗ | ~ | ✓ | ✓ |
| **Contract surface check (CSA)** | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** |
| Cross-file call graph BFS | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ |
| Schema / DB drift detection | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** |
| No build required | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ |
| Type inference | ✗ | ✗ | ✗ | ✓ | ✓ | **✗** |
| Full interprocedural taint | ✗ | ✗ | ✗ | ~ | ✓ | **✗** |
| MCP / agent integration | ✗ | ✗ | ✗ | ✗ | ✗ | **✓** |

Two genuine gaps exposed:

- **Type inference** (`✗`): GenSense matches symbols by name (`find_definition()` at `src/semantics/data_flow/lookup.rs`) with no type resolution. Cannot distinguish overloads or module-scoped definitions. This is an architectural limitation — fixing requires integrating with the language's type system (e.g., TypeScript's `tsserver` or Rust's `rust-analyzer` type info). **Deferred — no timeline.**

- **Full interprocedural taint** (`✗`): The `TaintRegistry<'a>` lifetime model prevents taint from flowing caller→callee (globals, closures) and callee→caller (out-params, ref mutations). Already documented as the taint lifetime deferred item in Phase 0. **Planned for v0.5.0.**

Differentiators (unique to GenSense, not gaps): **CSA**, **schema/DB drift**, **MCP integration** — no other tool category has these.

---

## Phase 6 — SPG (Semantic Program Graph) Foundation ✅ (Completed)

**Completed 2026-05-26 at commit `e1d3e1c`.** Three-layer change that enables cross-cutting queries across analysis subsystems.

### Phase 6a. Graph Exposed to All Rule Types

`&SemanticGraph` was added to `GenSenseContext` and `AuditOptions`. Previously only `ProjectRule` implementations could access the graph. Now every `GenSenseRule::check()` receives it.

- [x] `graph: &'a SemanticGraph` in `GenSenseContext` (`src/lib.rs:219`)
- [x] `graph: &'a SemanticGraph` in `AuditOptions` (`src/engine/auditor/mod.rs:45`)
- [x] Wired through all 3 construction sites in `auditor.audit()`, plus 2 in `tracking.rs`

### Phase 6b. Taint as Queryable Graph Edges

Instead of taint living ephemerally in a `TaintRegistry` HashMap (unreachable after each rule check), taint flows are now materialized as first-class `TaintFlow` edges on the `SemanticGraph`. After each file audit, the Engine iterates taint-related advisories and calls `graph.record_taint_flow()`.

- [x] `TaintFlow` edge kind (`src/semantics/graph.rs:22`)
- [x] `TaintFlowRecord { function_name, file_path, source_pattern, sink_pattern, rule_id }` (`src/semantics/graph.rs:27-32`)
- [x] `record_taint_flow()`, `has_taint_flow()`, `taint_flows_for()`, `taint_flows()` methods (`src/semantics/graph.rs:244-275`)
- [x] Post-audit materialization in `perform_parallel_audit` (`src/engine/project/mod.rs:654-667`)

Now queryable: `graph.has_taint_flow("validateInput", "src/handler.ts")` from any rule.

### Phase 6c. Algebraic Constraint Combinators (no Datalog)

Added combinatorial `FlowConstraint` variants that compose existing leaf constraints. A `FlowEvaluator` recursively evaluates constraint trees.

**New variants on `FlowConstraint`** (`src/rules/ir/flow.rs`):

| Variant | Meaning | YAML Field |
|---------|---------|-----------|
| `AllOf(Vec<FlowConstraint>)` | All sub-constraints must match (AND) | `all_of: [...]` |
| `AnyOf(Vec<FlowConstraint>)` | At least one matches (OR) | `any_of: [...]` |
| `Not(Box<FlowConstraint>)` | Negation — fires when sub-constraint doesn't match | `not: { ... }` |
| `Across { constraint, boundary_re }` | Sub-constraint must cross a temporal/structural boundary | `across_boundary: "\\.await"` |
| `Without { constraint, exclusion }` | Primary matches, exclusion doesn't | `without_constraint` + `without_exclusion` |
| `Chain { source, through, sink }` | Source reaches sink AND passes through intermediate | (future YAML for 7b) |

**`FlowEvaluator`** (`src/rules/ir/flow.rs`): Recursive tree-walking evaluator. `AllOf` short-circuits on first miss. `Across` checks temporal events in the enclosing function scope. `Without` checks primary then exclusion.

**How this differs from Datalog (CodeQL):** No query language. Fixed typed predicates + 5 combinators kept within a known-decidable boundary (per Rice's Theorem thesis in §2 of the paper). Users express compositions via YAML fields, not QL.

### Key Benefit

Before SPG, a rule could not ask "does this taint path cross an await?" — taint and temporal analysis were separate subsystems with no intersection. After SPG, the `Across` combinator does exactly that: `forbidden_source_pattern + forbidden_sink_pattern + across_boundary: "\\.await"` in a single YAML rule.

### Files Changed

`src/semantics/graph.rs`, `src/lib.rs`, `src/engine/auditor/mod.rs`, `src/engine/project/mod.rs`, `src/rules/ir.rs`, `src/rules/core/mod.rs`, `src/rules/compiler.rs`, `src/semantics/data_flow/tracking.rs` + 3 test files. 485 insertions, 3 deletions across 11 files.

---

## v0.5.0+ (Planned, Outside v0.4 Scope)

| Feature | Est. Time | Description |
|---------|-----------|-------------|
| AI hallucination detection | 6h | Resolve `use`/`import` against actual dependency tree (Cargo.lock, package.json) |
| Secrets with AST context | 4h | Context-aware string scanning — suppress UUIDs, flag API keys |
| Performance anti-patterns | 5h | N+1 ORM queries, unnecessary `clone()`, `Arc<Mutex>` in async code |
| Taint lifetime model redesign | ~8h | Remove `'a` parameter from `TaintRegistry` — enable cross-file caller→callee taint flow |

---

## LLM Anti-Pattern Rules (Proposed — v0.5.0 Candidates)

Six common LLM code-generation patterns that human developers instinctively avoid. These are candidates for new GenSense rules — some straightforward (tree-sitter AST query), some approximate (heuristics), one deferred (requires type system).

| # | Pattern | Detection approach | Confidence | Effort | Status |
|---|---------|-------------------|------------|--------|--------|
| 1 | **Over-abstracting** — `Box<dyn Trait>` where trait has a single impl | `ProjectRule`: collect traits + impl counts across project; flag trait-object usage for traits with ≤1 project impl | High | 1 day | **Plan to build** |
| 2 | **Nested `match Ok/Err`** — manual match on `Result` instead of `?` | AST query: `match_expression` where arm patterns are `Ok`/`Err` identifiers | High | 2h | Easy — tree-sitter pattern match |
| 3 | **String dispatch** — `match` on `&str`/`String` with >3 arms | AST query: `match_expression` with `match_value` of string type and >3 arms | Medium | 2h | Easy — tree-sitter pattern match |
| 4 | **Unnecessary `.clone()`** — clone on last use | Heuristic: walk function body, count `.clone()` calls where the cloned variable is never referenced again | Low–Medium | 2 days | Deferred — profile *already catches clone-density anomalies* via style surprise |
| 5 | **Premature async** — `async fn` with zero `.await` calls | AST query: `async` function/block containing no `.await` expression | High | 2h | Easy — tree-sitter pattern match |
| 6 | **Over-built builder** — impl block with >10 `fn set_*`/`fn with_*` methods | AST query: `impl` block counting method names matching `set_`/`with_` prefix | High | 2h | Easy — tree-sitter pattern match |

**Priority rationale:** #1 is worth building because it's sound (single impl + dyn usage = near-zero false positives when scoped to project-defined traits). #2, #3, #5, #6 are 2h each and make excellent onboarding rules. #4 is explicitly deferred — the style profile already catches excessive cloning as an anomaly signal; a standalone rule without type information would have too many false positives.

---

| Priority | Phase | Est. Time | Key Deliverables |
|----------|-------|-----------|------------------|
| **P0** | Taint soundness | ~2h | `taint_max_depth`, visited-set, match arm returns |
| **P1** | Style profile (v0.4 core) | ~8.5h | Richer fingerprints, `ProjectProfile`, `STYLE_ANOMALY`, CLI flags |
| **P2** | Rule hardening | ~2h ✅ | CSA coverage (4 rules), deadlock_guard fix, 3 temporal rules, delegation suppression |
| **P3** | Advanced constraints | ~19h | `AtomicSection`, SRI baselines, `--severity` pre-filter |
| **P4** | Advanced analysis | ~12h | MinHash/LSH, Datalog closure, taint entropy |
| **P5** | Developer experience | ~6h | Rule wizard, `gensense test-rule` |
| **P6** | **SPG (done)** | — | Graph in context, taint edges, algebraic combinators |
| **v0.5** | Future | ~17+8h | AI hallucination, secrets, perf patterns, taint lifetime |
