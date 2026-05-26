# Formal Audit: GENSENSE_FORMAL_FOUNDATIONS.md vs. v0.3.1 Implementation

This document audits every claim in `GENSENSE_FORMAL_FOUNDATIONS.md` against the
actual v0.3.1-tasks source code. Each section is classified as **VERIFIED**, **INCORRECT**,
**INCOMPLETE**, or **UNDOCUMENTED**. Where the document is wrong or missing, the correct
description is supplied so that the foundations document can be updated precisely.

Branch verified: `v0.3.1-tasks`
Files read: `src/semantics/data_flow/tracking.rs` (680 lines), `src/engine/fingerprint.rs`
(89 lines), `src/semantics/reachability.rs` (113 lines), `src/rules/ir.rs` (1503 lines),
`src/semantics/graph.rs`, `src/temporal/analyzer.rs`, `src/semantics/data_flow/normalization.rs`, `src/engine/suppression.rs`, `src/semantics/consistency.rs`.

---

## Section 22.1 — Taint Analysis

### 22.1.1 The Lattice — VERIFIED

The two-element boolean lattice claim is correct. `TaintOrigin` is a binary presence/absence
type (a variable is either in the registry or not). The registry (`TaintRegistry`) maps
variable names to `TaintOrigin` values. There is no multi-level taint label — no
`{untainted, user_input, system_input}` hierarchy. The document's $\mathcal{L} = (\{0,1\},\leq)$
accurately represents the implementation.

### 22.1.2 Transfer Functions — PARTIALLY INCORRECT

**Binding (VERIFIED).** `process_binding` at line 121 matches the formal definition. Source
detection checks both `name` and `val_code` against `source_re`, which the formal function
expresses as a disjunction — correct.

**Assignment — INCORRECT.** The formal definition states:

$$F_{\text{assign}}(\sigma)(x) = 0 \text{ if } x = target \text{ and } \nexists y \\in \text{refs}(value\_range).\\ \sigma(y) = 1$$

This says assignment *resets* taint when the right-hand side is clean. The actual code at
line 158–169 does **not** reset taint on clean assignment. If `target` was previously tainted
and the new value is clean, the old taint entry is overwritten only because `registry.taint()`
inserts a new entry with the `UserInput` origin from `source_re` matching — but if no origin is
found, `registry.taint()` is simply not called and the previous entry persists unchanged.

The correct transfer function for `process_assignment` is:

$$F_{\text{assign}}(\sigma)(x) = \begin{cases}
1 & \text{if } x = target \text{ and source\_re matches } name \text{ or } val\_code \\
1 & \text{if } x = target \text{ and } \exists y \in \text{refs}(value\_range).\\ \sigma(y) = 1 \\
\sigma(x) & \text{otherwise (no reset)}
\end{cases}$$

This is a monotone increasing function (taint never removed), which preserves the lattice
monotonicity theorem but differs from the document's claim that assignment resets taint.
The consequence: a variable that becomes tainted then assigned a clean value remains tainted.
This is a deliberate over-approximation (no false negatives, possible false positives on
clean reassignment).

**Call transfer function — INCOMPLETE.** The document describes the call transfer function
as producing a return-value taint based on argument taint. The actual `resolve_call_taint`
function (line 467) additionally:

1. Checks method-chain receiver taint: if `obj.method(args)` and `obj` is tainted, the
   entire call is tainted regardless of arguments. The formal definition omits this.
2. Checks `source_re.is_match(fn_name)` directly — if the function name matches the source
   pattern, the call itself is a source. The formal definition omits this.
3. Propagates taint through the callee return value by calling `find_returns()` on the callee
   body. The document only states that call taint marks $r_{\text{return}}$ tainted — it does
   not specify that the engine actually traverses return expressions in the callee body.

The correct call transfer function has three parts the document is missing:

$$F_{\text{call}}(\sigma)(x) = \begin{cases}
1 & \text{if source\_re matches } fn\_name \text{ (source function)} \\
1 & \text{if receiver is tainted (method chain)} \\
1 & \text{if } x = r_{\text{return}} \text{ and callee return expressions are tainted} \\
\sigma(x) & \text{otherwise}
\end{cases}$$

### 22.1.3 Monotonicity — VERIFIED

Theorem 22.1 holds. The corrected `process_assignment` transfer function (no reset) is in
fact *more* monotone than the documented version, since it never maps $1 \to 0$.

### 22.1.4 Single Forward Pass — VERIFIED WITH CORRECTION

The linear forward pass claim is correct. However, `resolve_taint` uses an iterative stack
(line 406–463) over the value expression's AST, not a simple text reference lookup. This means
taint propagation through nested expressions (e.g., `f(g(h(x)))`) is handled recursively within
a single binding op. The document's $\text{refs}(r, s)$ notation implies a flat reference set
but the implementation traverses the full expression tree. This does not affect soundness but
should be documented accurately.

The loop false negative claim is correct — `EnterBlock` recurses into nested blocks via
`process_enter_block` but does not iterate to a fixpoint. Loop bodies are analyzed once.

### 22.1.5 Interprocedural Extension — VERIFIED WITH ADDITIONS

`map_params()` is confirmed present (line 296 in `analyze_call`, line 552 in
`resolve_call_taint`). The $k$-bounded claim is correct — `depth < max_depth` guards both
call sites.

**Missing from the document:** The visited set $V$ is `RefCell<HashSet<(FilePath, StartByte)>>`
shared across all recursive calls via `self.visited.borrow_mut()` (line 546). This is a
**per-analysis-run** visited set, not a per-path visited set. If the same callee appears on
two different taint paths from the same root analysis, the second traversal is skipped. This
is an additional false negative source not mentioned in the document.

**Also missing:** `discover_symbols()` is called on each sub-analyzer (line 582) before
`analyze_block()`. This pre-populates the sub-registry with all binding names in the callee,
which ensures that the callee's internal bindings are visible to `resolve_taint` lookups.
The document does not mention this initialization step.

**Also missing:** `sanitizer_re` is mentioned in the document but does not exist as a field
in `CoreRuleIr` (verified at lines 65–92 of ir.rs). There is no sanitizer pattern field.
The document's sanitization clause in the call transfer function has no implementation
counterpart. This is a **phantom feature** in the foundations document.

---

## Section 22.2 — Temporal Analysis

### 22.2.2 MustNotFollow — VERIFIED

`found_first` flag, Release reset, and the LTL formula are all confirmed correct against
`src/temporal/analyzer.rs`.

### 22.2.3 MustFollow — VERIFIED

`current_step` counter, non-fire at `current_step = 0`, fire at partial match — all confirmed.

### 22.2.4 ForbiddenBetween — VERIFIED

`in_forbidden_zone` flag with start/end pattern detection confirmed.

### 22.2.5 Finite Trace Completeness — VERIFIED WITH CORRECTION

The parallel branch approximation (multiple chain heads ordered by source position) is
implemented correctly. The document's claim that this is a conservative approximation
producing possible false positives is accurate.

**Correction needed:** The document says BFS is used for event ordering. The actual
implementation in `analyzer.rs` uses the `SequentiallyFollows` edge chain built in Pass 3,
retrieved by walking `InScope` → event nodes in the `SemanticGraph`. The traversal is not
a standalone BFS — it follows pre-built edges. This is equivalent but the document implies
BFS is computed at check time, not pre-computed during discovery. Clarify that the topological
ordering is pre-built in Pass 3, not computed by the temporal checker.

---

## Section 22.3 — Jaccard N-gram Similarity

### MAJOR FINDING: Comparison Algorithm Does Not Exist in v0.3.1

`src/engine/fingerprint.rs` contains only `extract_fingerprints()` — it produces
`FunctionFingerprint { ngram_hashes: FxHashSet<u64> }` structs. **There is no Jaccard
comparison code, no inverted index, and no early-exit bound implementation anywhere in the
v0.3.1 source tree.**

The formal foundations document in Sections 22.3.2, 22.3.3, and 22.3.4 describes a complete
clone detection system. This system does not exist in v0.3.1. The `FunctionFingerprint` structs
are produced but never compared at runtime.

**Classification: INCOMPLETE (partially implemented — extraction only, comparison absent)**

What does exist in v0.3.1:
- `FxHashSet<u64>` for ngram hashes: VERIFIED (line 15 of fingerprint.rs)
- `FxHasher` for hashing: VERIFIED (not FNV-1a — see hash note below)
- Token extraction from body via `split_whitespace()`: VERIFIED
- Window size: taken as a parameter (`window_size`), not hardcoded to 5 — the document's
  claim that $w = 5$ is not a code-level constant, it is a caller convention checked in
  `GenSenseContext::ngram_window_size`

What is missing from v0.3.1 (planned for later):
- Jaccard similarity computation
- Early-exit bounds (size ratio and intersection bound)
- Inverted hash index for $O(N + M_\tau)$ candidate generation
- Any advisory generation from fingerprint comparison

**The formal foundations section 22.3 must be marked as a roadmap specification, not
a description of current behavior.** The document does not make this clear — Section 22.6
explicitly labels itself as 0.4.0 roadmap, but Section 22.3 does not, implying it is
implemented. This is misleading.

### Hash Function — INCORRECT LABEL

The document states "FNV-1a hashes via `FxHashSet<u64>`". `FxHasher` is from the
`rustc-hash` crate. It is a variation of FNV-1a but with different constants and a
non-standard folding step optimized for speed. It is not the standard FNV-1a algorithm
(offset basis `0xcbf29ce484222325`, prime `0x100000001b3`). The SRI fingerprint in
`new_advisory()` (ir.rs line 678–691) **does** use exactly FNV-1a constants, but the
n-gram hash in `fingerprint.rs` uses `FxHasher`. These are two different hash functions
and the document conflates them.

Correct labeling:
- SRI advisory fingerprint: **FNV-1a** (confirmed by constants in ir.rs line 679–690)
- N-gram hash in `FunctionFingerprint`: **FxHasher** (rustc-hash, FNV-1a variant)

---

## Section 22.4 — Contract Surface Analysis

### VERIFIED

All claims confirmed:
- `any_reachable_path_contains` at line 20 of reachability.rs: VERIFIED
- Dead branch pruning via `evaluate_condition` (true/false/1/0/!true/!false): VERIFIED
- `path_terminated` flag on `return_statement`/`throw_statement` (lines 94–97): VERIFIED
- `body_may_delegate_via` field in `CoreRuleIr` (line 83 of ir.rs): VERIFIED
- `body_must_contain` field (line 82): VERIFIED
- Check implementation at lines 314–348 of ir.rs: VERIFIED

**One addition missing from the document:** The `body_must_contain_any_of` field (line 84
of ir.rs) inverts the logic — it fires an advisory when the pattern IS found rather than
when it is absent. The formal definition in 22.4.1 only covers `body_must_contain` (absence
of required pattern). The `must_not_contain` / `body_must_contain_any_of` combination is
a second CSA variant (forbidden body content) not formally defined.

**Proposed definition for missing variant:**

**Definition 22.1b (CSA Forbidden Body Violation).** Function $f$ violates a forbidden-body
rule $r'$ if and only if:

$$\rho_{r'} \models \text{name}(f)$$
$$\land\\ \exists p \in \text{body}(f).\\ \beta_{r'} \models p$$
$$\land\\ \nexists d \in \delta_{r'}.\\ d \in \text{body}(f)$$

This is the dual: the name matches, the forbidden pattern IS reachable, and no bypass
cancels it.

---

## Section 22.5 — Call Graph Reachability

### VERIFIED

DFS (stack-based), `EdgeKind::Calls` filter, safe failure on ambiguous names — all confirmed.

**One correction:** The document states "no edge is added" on ambiguous names. The actual
resolution in `src/semantics/registry.rs` (add_call_edge) for same-file vs global disambiguation
was verified in graph.rs. The document is correct in principle but the implementation resolves
ambiguity per the priority rule (same-file first, then singleton global), not a flat "no edge
if any ambiguity" rule. The document's description is a simplification.

---

## Section 22.6 — N-gram Style Profile

### CORRECTLY LABELED AS ROADMAP

This section explicitly states it is a v0.4.0 roadmap formalization. No verification needed
against v0.3.1. The formal object is pre-defined for the implementation to be verified against.

---

## Major Undocumented Engine Subsystems

The formal foundations document strictly covers the *analysis logic* (Taint, Temporal, Clone Detection, CSA, Call Graph). It completely omits the formalization of the **core infrastructure** that makes GenSense operate. The following major components are missing from the formal document and should be added for a complete arXiv-grade paper:

### 1. Semantic Normalization (IR Generation)
**Location:** `src/semantics/data_flow/normalization.rs`
**What it is:** The function that maps language-specific AST nodes (Rust, TypeScript, JavaScript) into a universal, 4-element semantic algebra (`SemanticOp`):
- `Binding`
- `Assignment`
- `Call`
- `EnterBlock`
**Why it matters:** This is the core mathematical transformation that makes GenSense "Zero-Opinion" and cross-language. The taint analysis operates entirely on `SemanticOp`, not ASTs. The formal document assumes `SemanticOp` exists but does not define the mapping function $\mathcal{N} : \text{AST} \to \text{Seq}(\text{SemanticOp})$.

### 2. Auto-Remediation (Patcher System)
**Location:** `src/patcher/mod.rs` and `src/rules/ir.rs` (Regex replacements)
**What it is:** The system that consumes `fix_pattern` and `fix_template` to generate `proposed_replacement` strings, which are then applied by the patcher while maintaining valid AST bounds.
**Why it matters:** The remediation engine performs AST-aware text substitution. Formalizing the bounds of this substitution (why it avoids breaking syntax) is critical for a complete engine description.

### 3. Suppression Engine (Linter Invariants)
**Location:** `src/engine/suppression.rs`
**What it is:** The module that checks for `gensense-ignore: rule_id` within the 2 lines preceding an AST node.
**Why it matters:** In any static analysis formalization, the exact boundary of user overrides is necessary to bound the false-positive reporting rate.

### 4. Semantic Consistency Checking
**Location:** `src/semantics/consistency.rs`
**What it is:** Computes the mathematical divergence between a direct AST-walk analysis and a Graph-based analysis. `set_a == set_b`.
**Why it matters:** This acts as a formal equivalence checker between the single-file fast path and the multi-file interprocedural path.

### 5. Symbol Registry and Graph Construction
**Location:** `src/semantics/symbols.rs` and `src/semantics/registry.rs`
**What it is:** The $O(N)$ multi-pass resolution system that builds the `StableDiGraph` before any analysis runs.
**Why it matters:** Call Graph reachability (Section 22.5) depends entirely on this construction.

### 6. External Schema Integration (Prisma/OpenAPI)
**Location:** `src/rules/schema_contract/prisma_extractor.rs`
**What it is:** Extracts `ModelNames`, `FieldNames`, and `EnumValues` from external non-code schemas to inject dynamically into AST regex rules.
**Why it matters:** GenSense is not just analyzing code; it is validating code against external data layer contracts.

### 7. Overlap Confidence Boosting (Heuristic Multiplier)
**Location:** `src/engine/project/mod.rs` (`boost_overlap_confidence`)
**What it is:** An algorithmic pass that iterates over all generated advisories and artificially inflates their confidence score (`confidence = min(1.0, confidence + (overlap * rate))`) if multiple distinct rules flag the exact same file and line.
**Why it matters:** This fundamentally alters the statistical distribution of confidence scores output by the engine. It is a probabilistic heuristic overlaid on top of the formal deterministic lattice, and must be documented as an algorithmic adjustment.

### 8. The Model Context Protocol (MCP) Interface
**Location:** `src/mcp/`
**What it is:** The GenSense engine is exposed natively as an MCP server, allowing LLMs and external IDE agents to query the semantic graph, request taint analysis runs, and pull rule metadata dynamically.
**Why it matters:** This defines the integration boundary of the engine. The formal paper describes the engine in isolation, but its primary deployment architecture is as a networked, stateful context provider for AI agents.

### 9. Output Standardization (SARIF Translation)
**Location:** `src/reporter.rs`
**What it is:** The mapping function that translates abstract `Advisory` outputs (and their underlying lattice states/confidence intervals) into the OASIS SARIF v2.1.0 standard.
**Why it matters:** Connects the theoretical output of the engine to standard CI/CD and security ingestion formats.

---

## Summary of Findings

| Section | Status | Issue |
| :--- | :---: | :--- |
| 22.1.1 — Lattice | VERIFIED | Correct |
| 22.1.2 — Binding transfer fn | VERIFIED | Correct |
| 22.1.2 — Assignment transfer fn | **INCORRECT** | Document says taint resets on clean assignment. Code does not reset. Monotone-only. |
| 22.1.2 — Call transfer fn | **INCOMPLETE** | Missing: method-chain receiver taint, source-name match, callee return traversal |
| 22.1.2 — Sanitizer pattern | **PHANTOM** | `sanitizer_re` field does not exist in `CoreRuleIr`. Feature undocumented as unimplemented. |
| 22.1.3 — Monotonicity theorem | VERIFIED | Still holds with corrected assignment |
| 22.1.4 — Forward pass | VERIFIED | Correct; expression traversal is deeper than docs imply |
| 22.1.5 — Interprocedural | **INCOMPLETE** | Missing: per-run visited set semantics, `discover_symbols()` init step |
| 22.2.2 — MustNotFollow | VERIFIED | Correct |
| 22.2.3 — MustFollow | VERIFIED | Correct |
| 22.2.4 — ForbiddenBetween | VERIFIED | Correct |
| 22.2.5 — Finite trace | **INCORRECT (minor)** | BFS not computed at check time; pre-built in Pass 3 |
| 22.3 — Jaccard n-gram | **INCOMPLETE** | Extraction only in v0.3.1. Comparison, index, and bounds are not implemented. Document does not flag this. |
| 22.3 — Hash function label | **INCORRECT** | SRI uses FNV-1a; n-gram uses FxHasher. These are distinct. |
| 22.4 — CSA | VERIFIED | Correct; missing formal definition of `body_must_contain_any_of` variant |
| 22.5 — Call graph DFS | VERIFIED | Correct; minor simplification in ambiguity description |
| 22.6 — Style profile | CORRECTLY LABELED | Roadmap; not verifiable against v0.3.1 |

---

## Items That Improve Research Value If Added

These are **undocumented behaviors** in v0.3.1 that have research significance and should
be formally described:

1. **Object field taint propagation.** `propagate_object_taint()` (line 337 of tracking.rs)
   tracks taint at the field level within object literals — `{password: req.body.password}`
   taints the `password` field of the returned object, not the whole object. This is a
   field-sensitive taint analysis. The formal foundations do not mention field sensitivity.
   Formal description would be: the taint state $\sigma : \text{Var} \cup (\text{Var} \times \text{Field}) \to \mathcal{L}$
   is a field-sensitive extension of the two-element lattice.

2. **Spread element taint propagation.** `{...taintedSource, explicit: clean}` correctly
   taints all fields from the spread and then handles explicit overrides. This is a specific
   extension of the call transfer function for object spread syntax.

3. **`ScopeConstraint` flow type.** The `FlowConstraint::ScopeConstraint { pattern, invert }`
   variant (ir.rs line 31) checks whether a node's AST ancestor matches a pattern. Used to
   fire when code appears inside (or outside) a named scope. This constraint type has no
   formal treatment in the foundations document.

4. **Composite flow constraints (`AllOf`, `AnyOf`, `Not`, `Across`, `Without`, `Chain`).**
   The `FlowEvaluator` is dispatched for these at ir.rs line 645. These are logical
   combinators over `FlowConstraint`. None are formally defined in the foundations document.
   They have high research value: `Chain` in particular is a composition of three
   `FlowConstraint` objects into a source-through-sink path constraint.

5. **`TaintReached` vs `TaintForbidden` distinction.** Both call the same
   `evaluate_taint_constraint()` function with a different `constraint_type` string. The
   actual distinction in behavior is in how findings are interpreted by the consumer — both
   produce `Vec<Advisory>` when taint flows are found. `TaintReached` fires when flow IS
   found (assertion of required flow). The formal foundations document mentions both but does
   not formally define the `TaintReached` semantics beyond the taint analysis itself.

6. **Confidence differentiation by depth.** At line 281–285 of tracking.rs, findings at
   `depth > 0` (interprocedural) receive `taint_confidence_interprocedural` instead of
   `taint_confidence_intraprocedural`. This is a first-class confidence adjustment that
   the formal foundations do not mention. It represents a calibration decision: cross-function
   taint flows are assigned lower confidence than within-function flows.
