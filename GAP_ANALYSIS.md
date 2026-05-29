# Gap Analysis — Remaining Work

Verified 2026-05-27. Split by crate: **engine** (`gensense-engine`) = pure analysis primitives, **consumer** (`gensense`) = rules, CLI, advisories.

---

## Theoretical Gaps (by Domain)

What the engine is missing to fully serve all three purposes (anomaly detection + security/correctness + AI guardrails), organized by capability domain.

### 1. Semantic Program Graph

| Missing | Why | Blocks |
|---------|-----|--------|
| **Control flow graph** | No basic blocks, branches, loop headers. `ReachabilityChecker` is a manual AST walk — can't model scopes properly. | Path-sensitive analysis, lock-scope tracking (drop before await), reaching definitions |
| **Transitive closure cache** | `has_call_path()` = DFS per query. O(n) every time. | Performance at project scale |
| **Cross-file edges** | Cross-file relationships live in `SymbolRegistry` but aren't first-class graph edges with kinds. | Cross-file taint, cross-file pattern matching |
| **Module/namespace nodes** | `use`/`import` isn't a graph node. Can't ask "does this import resolve?" | Dependency hallucination detection |
| **Def-use chains** | No reaching definitions at all. | Precision for any dataflow analysis |

### 2. Taint Analysis

| Missing | Why | Blocks |
|---------|-----|--------|
| **Owned taint state** | `TaintRegistry<'a>` borrows from one tree. Cross-file creates fresh registry with incompatible lifetimes. | Cross-file taint soundness |
| **Cross-file call resolution** | No engine-level mechanism to: parse callee → map args → analyze → return taint. Consumer's `DataFlowAnalyzer` fakes it but can't cross file boundaries. | Full interprocedural taint |
| **Caller↔callee mapping** | No positional/named arg-to-param mapping in engine. | Cross-file taint propagation |
| **Global/static taint** | Module-level statics not tracked. | Sound multi-call analysis |
| **Return value propagation** | No engine mechanism to mark return as tainted and propagate to caller. Consumer's `resolve.rs` does it ad-hoc. | Sound callee→caller flow |
| **Alias analysis** | `x.f` and `y.f` treated independently even if `x` and `y` alias. | False negatives on object taint |

### 3. Pattern System (replaces all YAML)

| Missing | Why | Blocks |
|---------|-----|--------|
| **Pattern type + compiler** | No representation for "this AST is a violation". Source examples → compiled AST signatures. | All pattern-based detection |
| **Pattern matcher + scorer** | Compare function AST against pattern signatures, produce normalized confidence (0.0–1.0). | Reporting without false positives |
| **Cross-language pattern IR** | Rust and TypeScript have different AST node kinds. Patterns need a language-agnostic IR so one pattern works for both. | Cross-language detection |
| **Pattern library format** | How patterns are shipped as data (embedded in binary or loaded from disk). | Distribution |

### 4. Language Support

| Missing | Why | Blocks |
|---------|-----|--------|
| **C language** | No `tree-sitter-c` dep, no `c`/`h` arm in `parser.rs`. C = the C programming language, not rewriting the engine in C. | `AtomicSection`/TOCTOU (mutex lock/unlock, pthreads), any C/C++ analysis |
| **Cross-language abstraction** | Every analysis is written once per language pattern. No shared IR. | Adding languages is expensive |

### 5. Dependency Resolution

| Missing | Blocks |
|---------|--------|
| Lockfile parser (`Cargo.lock`, `package.json`) | AI hallucination detection |
| Import → dependency matching | Verifying imports resolve to real packages |

### 6. String Analysis

| Missing | Blocks |
|---------|--------|
| Shannon entropy for string literals | Secrets detection (API keys vs UUIDs vs URLs) |
| Context classifier (parent AST node → string purpose) | Reducing false positives on secret scanning |

---

## Engine Gaps (gensense-engine)

### E1. Pattern Matching Engine (Phase X)

New infrastructure that replaces YAML+regex rules with example-driven detection:

- [x] **AST signature comparison** — compare function body AST against curated positive/negative examples using structural markers, node kind histograms
- [x] **Pattern→code scoring** — Jaccard on n-grams, structural marker similarity, taint-path shape matching, temporal event sequence matching
- [x] **Cross-language pattern data format** — example source files annotated with expected confidence, stored as binary data shipped in the engine

*Consumer counterpart:* Pattern library data (curated examples), CLI for running pattern-based checks.

---

### E2. `AtomicSection` Support — C Language (Phase 3a)

TOCTOU race detection — reads of mutex-protected variables outside the lock. Requires adding **C programming language** support (`tree-sitter-c` for parsing `.c`/`.h` files, pthread model).

- [ ] **tree-sitter-c grammar** — add `tree-sitter-c` as optional dep + `c`/`h` arm in `parser.rs`
- [ ] **Lock-set construction AST pass** — map `{variable → set of protecting mutexes}` from AST
- [ ] **Read-outside-lock detection** — flag reads of protected vars outside lock/unlock spans
- [ ] **Condition variable pairing** — verify `pthread_cond_signal`/`wait` hold the associated mutex

*Consumer counterpart:* `AtomicSection` YAML DSL, project rule variant.

---

### E3. MinHash / LSH (Phase 4a)

Similarity estimation for duplicate/copy-paste detection:

- [x] **`MinHashSignature { bands: Vec<u64> }`** with k=128 universal hash functions
- [x] **LSH bucket grouping** by band hash

*Consumer counterpart:* Project-level rule pass — emit advisory when Jaccard > 0.75 AND functions have different taint sinks.

---

### E4. Transitive Closure Cache on Call Graph (Phase 4b)

> **Decision 2026-05-27:** NOT Datalog. No query language, no fixed-point engine, no QL. Just `HashMap<NodeId, HashSet<NodeId>>` computed once via BFS per node.

- [ ] **Compute transitive closure** once per project scan, cache on `SemanticGraph`
- [ ] **Replace `has_call_path()`** with set-lookup on the closure

*Consumer counterpart:* `must_pass_through` YAML field → new `ProjectFlowConstraint` variant.

---

### E5. Taint Entropy / Branch Coverage (Phase 4c)

- [ ] **`TaintMetrics { tainted_uses, taint_branched_on, taint_branch_ratio }`** struct
- [ ] **Compute during `analyze_block()`** — count conditionals touching tainted variables
- [ ] **Return `TaintMetrics` from `DataFlowEngine::analyze_block()`** — changes engine API

*Consumer counterpart:* `min_taint_entropy` YAML field, `AI_TAINT_BYPASS` rule.

---

### E6. Full Cross-File Taint (Phase 0 deferred, v0.5, ~8h)

`TaintRegistry<'a>` blocks everything. Fixing it requires removing the lifetime and building cross-file resolution:

- [ ] **Remove `'a` from `TaintRegistry`** — switch to owned data (`String`, `Arc<str>`)
- [ ] **Cross-file call chain resolution** — engine-level mechanism to: parse callee file → map arguments → analyze → return taint
- [ ] **Caller→callee argument mapping** — positional and named arg-to-parameter matching across files
- [ ] **Callee→caller return value propagation** — mark return as tainted, propagate back to caller
- [ ] **Global/static variable taint** — track taint on module-level statics across calls
- [ ] **Alias analysis** — `x.f` and `y.f` treated as same object when `x` and `y` alias (see E11)

---

### E7. Dependency Resolution Module (v0.5)

- [ ] **New `deps` module** — resolve `use`/`import` statements against `Cargo.lock`, `package.json`
- [ ] **Detect hallucinated dependencies** — imports that don't exist in the lockfile

*Consumer counterpart:* `AI_HALLUCINATED_IMPORT` rule.

---

### E8. Secrets with AST Context (v0.5)

- [ ] **Context-aware string scanning** — given a string literal, determine if parent node kind implies API key, UUID, or benign value
- [ ] **Shannon entropy calculation** for string literals in assignment/argument positions

*Consumer counterpart:* `HARDCODED_SECRET` rule (replaces current regex-based version).

---

### E9. Control Flow Graph (v0.5+)

The engine has no CFG. `ReachabilityChecker` is a manual AST walk that can't model scopes or paths properly.

- [ ] **`ControlFlowGraph` type** — `Vec<BasicBlock>` with start_byte, end_byte, block kind (Entry, Exit, IfBranch, LoopHeader, LoopBody, Join, StraightLine)
- [ ] **CFG construction from AST** — tree-sitter AST → basic blocks + branch edges + fallthrough edges
- [ ] **CFG-based `ReachabilityChecker`** — replace manual AST walk with CFG traversal (proper scope tracking, dead branch pruning)
- [ ] **Def-use chain computation** — reaching definitions on top of CFG

*Why:* Unlocks path-sensitive analysis, proper lock-scope tracking (drop-before-await without false positives), and reaching definitions for taint precision.

---

### E10. Cross-Language Pattern Abstraction (Phase X)

Rust and TypeScript have different AST node kinds. Pattern examples written in one language can't match the other without an intermediate representation.

- [x] **Language-agnostic AST node taxonomy** — map `function_item` / `function_declaration` / `method_definition` → shared `FunctionDef` kind
- [x] **Cross-language pattern compilation** — pattern examples → IR → language-specific matchers
- [x] **Shared structural marker vocabulary** — so `RUST_LLM_NEVER_ERR` pattern works for `TS_RESULT_IGNORED` with no rewrite

*Note:* This is what makes the pattern library (E1) actually cross-language. Without it, every pattern must be duplicated per language.

---

### E11. Alias Analysis (v0.5+)

`TaintRegistry` has `taint_field()` (field sensitivity) but no alias analysis — `x.f` and `y.f` are tracked independently even when `x` and `y` point to the same object.

- [ ] **Pointer/alias graph** — track which variables may alias via assignments, references, and function calls
- [ ] **May-alias query** — `do_may_alias(x, y) → bool` for taint propagation
- [ ] **Integration with `TaintRegistry`** — when `x.f` is tainted, also taint `y.f` if `may_alias(x, y)`

---

## Consumer Gaps (gensense)

### C1. Pattern Library Data (Phase X)

Curated example files shipped in the binary:

- [ ] **Positive/negative pairs** for each detection capability (replaces current YAML rules)
- [ ] **Cross-language examples** — Rust + TypeScript/JavaScript patterns
- [ ] **Annotation format** — expected confidence, applicable languages, metadata

### C2. Profile CLI Polish (Phase 1 deferred)

- [ ] `gensense . --check-profile --diff-only` — score only new/changed files
- [ ] `gensense . --check-profile --strict` — fails if any function exceeds threshold
- [ ] Baseline regeneration post-merge
- [ ] Acceptance criteria: deterministic profile tests, LLM function > 0.5, normal < 0.3

### C3. SRI Diff-Only Baselines (Phase 3b, 7.5h)

- [ ] **Git diff detection** for changed symbols (`SymbolEntry.git_blob_oid`)
- [ ] **`.gensense/baseline.json`** — SRI-anchored advisory fingerprints from `main`
- [ ] **Baseline suppression** — advisories matching baseline excluded from output
- [ ] **`--diff-only`** and **`--diff-base`** CLI flags
- [ ] **`--update-baseline`** CI integration
- [ ] **Tests** — git-aware fixtures with branch switching

### C4. LLM Anti-Pattern Rules (v0.5 candidates)

| # | Pattern | Effort |
|---|---------|--------|
| 1 | **Over-abstracting** — `Box<dyn Trait>` where trait has ≤1 impl | 1 day |
| 2 | **Nested `match Ok/Err`** — manual match on Result instead of `?` | 2h |
| 3 | **String dispatch** — `match` on `&str` with >3 arms | 2h |
| 4 | **Unnecessary `.clone()`** — clone on last use (deferred — profile already catches) | — |
| 5 | **Premature async** — `async fn` with zero `.await` | 2h |
| 6 | **Over-built builder** — `impl` block with >10 `set_`/`with_` methods | 2h |

### C5. Rule Wizard (Phase 5, ~6h)

- [ ] `gensense rule new` subcommand — 6-step flow (language, example code, node picker, pattern auto-escape, identity, advisory text, verify)
- [ ] Node kind resolver — parse input line, return every named node spanning it
- [ ] Regex auto-escaper — plain text → regex-safe pattern
- [ ] Verification step — compile draft YAML, run on user fixture, report pass/fail
- [ ] `gensense test-rule` standalone command

### C6. Remaining YAML DSL fields

- [ ] `taint_max_depth` per rule ✅ (done — marked for tracking)
- [ ] `must_pass_through` YAML field (E4 counterpart)
- [ ] `min_taint_entropy` YAML field (E5 counterpart)
- [ ] `AtomicSection { shared_variable, guard_mutex }` YAML (E2 counterpart)

---

## Deferred (No Timeline)

| Gap | Why Deferred |
|-----|--------------|
| **Type inference** | Requires integrating with language type systems (tsserver, rust-analyzer) — architectural change |
| **Performance anti-patterns** (N+1, Arc in async) | 5h — no new engine primitive needed, just consumer rules |
| **`--severity` pre-filter** | ✅ done |

---

## Appendix — Capability Matrix

| Capability | Engine has? | Consumer has? | Gap |
|---|---|---|---|
| AST matching (tree-sitter) | ✅ `parser.rs` | ✅ `CoreRuleIr` | — |
| Intra-function taint | ✅ `DataFlowEngine` | ✅ `DataFlowAnalyzer` | — |
| Cross-file taint | ✅ `CrossFileResolver` | ✅ | — |
| Temporal event ordering | ✅ `TemporalChecker` | ✅ adapter | — |
| Contract surface (CSA) | ❌ (not needed in engine) | ✅ YAML rules | — |
| Call graph BFS | ✅ `has_call_path()` | ✅ | — |
| Schema/DB drift | ❌ | ✅ Prisma extractor | — |
| Fingerprinting | ✅ `extract_fingerprints` | ✅ CLI/integration | — |
| Profile/style surprise | ✅ `ProjectProfile` | ✅ CLI/integration | — |
| Pattern-based detection | ✅ | ✅ | **C1** (library + CLI) |
| MinHash / LSH | ✅ | ❌ | **C1** (library + CLI) |
| Taint entropy | ❌ | ❌ | **E5** |
| AtomicSection / C language | ❌ | ❌ | **E2** |
| Control flow graph | ✅ | ✅ | — |
| Cross-language pattern IR | ✅ | ✅ | — |
| Alias analysis | ❌ | ❌ | **E11** |
| SRI baselines | ❌ | ❌ | **C3** |
| Dependency resolution | ❌ | ❌ | **E7** |
| Secrets with AST context | ❌ | ❌ | **E8** |
| Rule wizard | ❌ | ❌ | **C5** |
| MCP integration | ❌ | ✅ | — |

---

## Summary

| Category | Count | Est. Time |
|---|---|---|
| **Engine gaps** (E2, E5, E7, E8, E11) | 5 items | ~22h |
| **Consumer gaps** (C1–C6) | 6 items | ~22h |
| **Deferred** | 2 items | — |
