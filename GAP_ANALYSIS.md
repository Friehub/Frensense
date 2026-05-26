# Gap Analysis → Build Plan

Verified 2026-05-24 against commit `ae315d6`. Phases ordered by dependency — each phase unblocks or accelerates the next.

---

## ✅ Already Done (No Action Needed)

| Item | Evidence |
|------|----------|
| `body_must_contain` uses AST `ReachabilityChecker` (not raw regex) | `src/rules/ir.rs:291-316` — fixed in v0.3.1 commit `6f4274a` |
| Advisory agent fields (`confidence`, `auto_fixable`, `requires_human`) | `src/lib.rs:110,125,126` — present in `Advisory` struct |
| Native TypeScript `TS_TAUTOLOGICAL_ASSERT` rule | `src/rules/typescript/ts_tautological_assert.rs` — registered, 7 test cases |
| `SemanticGraph` exposed to all rules via `GenSenseContext` | `src/lib.rs:219`, commit `e1d3e1c` |
| Taint flow materialized as `TaintFlow` graph edges | `src/semantics/graph.rs:22-275`, commit `e1d3e1c` |
| Algebraic constraint combinators (`AllOf`, `Across`, `Without`, etc.) | `src/rules/ir.rs:36-52` + `FlowEvaluator`, commit `e1d3e1c` |

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

## Phase 1 — v0.4.0 Core: Style-Anomaly Detection

**Why second:** The style profile is v0.4.0's headline feature (8.5h estimated). It extends the existing n-gram fingerprinting into a project-wide statistical model that catches LLM-generated code violating unwritten conventions. This is the highest-value new capability.

### 1a. Expand Fingerprint Extraction

Current `extract_fingerprints` at `src/engine/fingerprint.rs` extracts only body 5-gram `FxHashSet<u64>`. v0.4.0 needs richer features:

| Feature | Source | Example |
|---------|--------|---------|
| Body n-grams | Whitespace-split tokens (existing) | `["async", "fn", "name", "(", "params"]` |
| Signature n-grams | Function declaration tokens | `["export", "const", "name", "=", "{"]` |
| Parameter type n-grams | Type annotations in params | `["userId:", "string", "cartId:", "string"]` |
| Method name segments | CamelCase/PascalCase splits | `createFromCart` → `["create", "From", "Cart"]` |
| Structural markers | AST node kinds in body | `["variable_declarator", "call_expression", "return"]` |
| Type usage | Type annotation occurrences | `["string", "number", "Decimal", "any"]` |
| Comment density | Comment bytes / total bytes | `0.02` |

- [ ] **Add each feature** as a new field on `FunctionFingerprint` (frequency maps, not presence sets)
- [ ] **Language-aware extraction** — Rust gets `function_item`, TypeScript gets `arrow_function` + `method_definition`, Solidity gets `function_definition`

### 1b. Project Profile + Serialization

- [ ] **`ProjectProfile` struct** (`ngram_frequencies`, `file_profiles`, `total_ngrams` per language)
- [ ] **Serialized to `.gensense/profile.json`** — committed to repo, portable "memory"

### 1c. `style_surprise` Scoring

- [ ] **`style_surprise()` function** — fraction of n-grams that are rare/unseen in the project profile
- [ ] **Threshold:** flag at `> 0.5` (strict) or `> 0.7` (default), configurable

### 1d. File-Level Profile Isolation

A project-global profile flags test files as anomalous (they have different conventions). Mitigation:

- [ ] **Separate profiles** for `src/`, `tests/`, `scripts/`
- [ ] **New directory detection** — score against closest matching profile

### 1e. CLI + API

- [ ] `gensense --learn-profile` — scans project, builds `.gensense/profile.json`
- [ ] `gensense . --check-profile` — audit with profile-based anomaly detection
- [ ] `gensense . --check-profile --diff-only` — score only new/changed files
- [ ] `gensense --profile-stats` — view profile stats
- [ ] `Engine::with_profile()` and `run_with_profile()` Rust API

### 1f. `STYLE_ANOMALY` Rule

- [ ] **Advisory message templates** — "Function 'X' has N% unfamiliar token patterns. This project uses camelCase (seen 1,247x). 'X' uses PascalCase — seen 0x."

### 1g. CI Integration

- [ ] `gensense . --check-profile --strict` — fails if any function exceeds threshold
- [ ] Baseline regeneration post-merge

### 1h. Acceptance Criteria

- [ ] LLM-generated function with `any` types, PascalCase, or class syntax in const-service project scores `> 0.5`
- [ ] Normal project function scores `< 0.3`
- [ ] Profile is deterministic (same project → same hash, ignoring timestamps)
- [ ] No false positives for test files (file-level profiles)
- [ ] `REDUNDANT_BOILERPLATE` rule still works (backward compat with existing n-grams)

---

## Phase 2 — Rule Coverage Hardening

**Why third:** Existing rules have coverage gaps. These are quick wins (no new infrastructure) that raise the reliability baseline before adding new capabilities.

### 2a. CSA Rule Test Coverage

6 corpus fixtures exist for 3 of 7 CSA rules. 1 automated test function. 4 rules have zero coverage.

| Rule | Fixture | Test | Status |
|------|---------|------|--------|
| `RUST_CSA_VALIDATE_UNCONDITIONAL` | ✅ | ✅ | Done |
| `TS_CSA_VALIDATE_UNCONDITIONAL` | ✅ | ❌ | ⚠️ |
| `TS_CSA_AUTH_NO_REJECTION` | ✅ | ❌ | ⚠️ |
| `SOL_CSA_VALIDATE_UNCONDITIONAL` | ❌ | ❌ | ❌ |
| `SOL_CSA_SANITIZE_PASSTHROUGH` | ❌ | ❌ | ❌ |
| `TS_CSA_SANITIZE_PASSTHROUGH` | ❌ | ❌ | ❌ |
| `TS_CSA_FIND_NEVER_EMPTY` | ❌ | ❌ | ❌ |

- [ ] **Add corpus fixtures** (positive + negative pairs) for all 4 uncovered rules
- [ ] **Add `run_test()` calls** in `tests/rule_tests.rs` for all 7 rules
- [ ] **Test `body_may_delegate_via`** suppression paths

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
- [ ] **Tests** — corpus fixtures + `run_test()` for all three

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

**New variants on `FlowConstraint`** (`src/rules/ir.rs:36-52`):

| Variant | Meaning | YAML Field |
|---------|---------|-----------|
| `AllOf(Vec<FlowConstraint>)` | All sub-constraints must match (AND) | `all_of: [...]` |
| `AnyOf(Vec<FlowConstraint>)` | At least one matches (OR) | `any_of: [...]` |
| `Not(Box<FlowConstraint>)` | Negation — fires when sub-constraint doesn't match | `not: { ... }` |
| `Across { constraint, boundary_re }` | Sub-constraint must cross a temporal/structural boundary | `across_boundary: "\\.await"` |
| `Without { constraint, exclusion }` | Primary matches, exclusion doesn't | `without_constraint` + `without_exclusion` |
| `Chain { source, through, sink }` | Source reaches sink AND passes through intermediate | (future YAML for 7b) |

**`FlowEvaluator`** (`src/rules/ir.rs:1203-1345`): Recursive tree-walking evaluator. `AllOf` short-circuits on first miss. `Across` checks temporal events in the enclosing function scope. `Without` checks primary then exclusion.

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

## Summary

| Priority | Phase | Est. Time | Key Deliverables |
|----------|-------|-----------|------------------|
| **P0** | Taint soundness | ~2h | `taint_max_depth`, visited-set, match arm returns |
| **P1** | Style profile (v0.4 core) | ~8.5h | Richer fingerprints, `ProjectProfile`, `STYLE_ANOMALY`, CLI flags |
| **P2** | Rule hardening | ~6h | CSA coverage (4 rules), deadlock_guard fix, 3 temporal rules |
| **P3** | Advanced constraints | ~19h | `AtomicSection`, SRI baselines, `--severity` pre-filter |
| **P4** | Advanced analysis | ~12h | MinHash/LSH, Datalog closure, taint entropy |
| **P5** | Developer experience | ~6h | Rule wizard, `gensense test-rule` |
| **P6** | **SPG (done)** | — | Graph in context, taint edges, algebraic combinators |
| **v0.5** | Future | ~17+8h | AI hallucination, secrets, perf patterns, taint lifetime |
