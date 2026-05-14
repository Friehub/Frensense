# GenSense: Algorithmic Grounding for the LLM Era
## Mathematics Already in the Codebase + What to Mix In

---

## What GenSense Already Has (The Mathematical Foundation)

Before introducing anything new, it is worth naming precisely what is already in the code, 
because the additions below are *mixtures and extensions* of these — not replacements.

| Component | Location | Mathematical Structure |
|---|---|---|
| N-gram hashing | `fingerprint.rs` | Shingling over token sequences → Jaccard-comparable sets |
| Taint registry | `data_flow/mod.rs` | Scoped symbol table → lattice of taint propagation |
| Semantic graph | `graph.rs` + petgraph | Directed labeled graph (DAG in practice) |
| Temporal analyzer | `temporal.rs` | Finite automaton over ordered event sequences |
| Consistency check | `consistency.rs` | Set equality over advisory collections |
| BFS in ProjectRule | `ir.rs` | Breadth-first reachability on the call graph |
| Cross-file taint | `ir.rs` CrossFileTaintFree | Reachability with a sink predicate |

The pattern across all of these: GenSense is already doing graph theory, finite automata, 
and set operations on code structure. The additions below are chosen specifically because 
they compose with what is already here — using the same data structures, adding new 
mathematical lenses on top of them.

---

## Three Algorithms to Mix In

---

### 1. MinHash / SimHash on the N-gram Fingerprint
**Already has:** n-gram hash sets in `fingerprint.rs`  
**Add:** Locality-Sensitive Hashing (LSH) for similarity estimation

**The current fingerprint** computes 5-token n-gram hashes and stores them as a `HashSet<u64>`. 
This is a shingling approach. It is already correct for exact deduplication. What it cannot 
do is answer "how similar are these two functions?" efficiently.

**Jaccard similarity** between two functions A and B is:
```
J(A, B) = |A ∩ B| / |A ∪ B|
```
where A and B are their n-gram hash sets. This is the most principled definition of 
"structural similarity" for code — more so than token edit distance, because it is 
order-insensitive to local rearrangements.

**MinHash** estimates Jaccard similarity in O(k) time using k hash functions instead 
of materializing the full sets. The estimate error is 1/√k. At k=128, error ≈ 8.8%.

**Why this matters for LLM code specifically:**

LLMs produce *structurally similar but semantically variant* code across a codebase. 
When a developer asks an LLM to implement `validate_email` and `validate_phone`, the LLM 
will often produce bodies that are 70-85% similar in token n-gram space — same structure, 
same skeleton — but one of them has the validation inverted or a case missing. This is 
invisible to all current rules because it requires comparing functions *to each other*, 
not functions to patterns.

MinHash enables a new advisory class: **"These two functions are 84% structurally similar 
but their return paths diverge. Likely copy-generated from the same LLM prompt."**

**What changes in the code:**

`FunctionFingerprint` already stores `ngram_hashes: HashSet<u64>`. Add:

```rust
pub struct MinHashSignature {
    pub bands: Vec<u64>,  // k=128 minhash values, stored as 16 bands of 8
}

impl FunctionFingerprint {
    pub fn minhash(&self, k: usize) -> MinHashSignature {
        // For each of k hash functions h_i, compute min(h_i(x)) over all x in ngram_hashes
        // Using universal hashing: h_i(x) = (a_i * x + b_i) mod p  (prime p)
    }
}
```

The `ProjectRule` pass already does BFS over the call graph. Add a pre-pass that:
1. Computes MinHash signatures for all functions
2. Groups functions by LSH bucket (band hashing)
3. For each bucket with 2+ functions: emit an advisory if Jaccard > 0.75 AND the 
   functions have different reachable sinks in the taint graph

The second condition (different sinks) is what filters noise — purely similar utility 
functions are fine. It is only suspicious when two near-identical functions have 
different security-relevant behavior.

**Complexity:** O(n × k) to compute signatures, O(n × b) for bucket grouping. 
For a 10,000-function codebase with k=128: ~1.28M operations. Negligible.

---

### 2. Datalog Semantics on the Existing Call Graph
**Already has:** `SemanticGraph` (petgraph DiGraph), BFS in `ProjectRule`, `EdgeKind` enum  
**Add:** Datalog-style fixed-point reasoning over the graph

**The current BFS** in `ProjectFlowConstraint::CrossFileTaintFree` and `MustHaveGuard` 
is manually written breadth-first search. It works but it has a fundamental limitation: 
each constraint is evaluated independently. You cannot express "taint flows from A to C 
*only through B*" or "every call path from auth to database must pass through validation."

These are naturally Datalog rules:

```datalog
// Reachable(x, y) ← Calls(x, y)
// Reachable(x, z) ← Calls(x, y), Reachable(y, z)
// TaintPath(source, sink) ← Tainted(source), Reachable(source, sink), IsSink(sink)
// Guarded(source, sink) ← TaintPath(source, guard), TaintPath(guard, sink), IsGuard(guard)
// Violation(source, sink) ← TaintPath(source, sink), NOT Guarded(source, sink)
```

Datalog is the theoretical grounding behind *every major production static analyzer*: 
CodeQL is Datalog, Doop is Datalog, Chord is Datalog. The reason is that fixed-point 
iteration over a monotone lattice is guaranteed to terminate and produces the most-precise 
result expressible in the logic.

**GenSense does not need a full Datalog engine.** It needs the *pattern* applied to 
its existing petgraph structure. The key operation is semi-naive evaluation:

```
Reachable := ∅
repeat:
    Reachable_new := Reachable ∪ { (x,z) | Calls(x,y) ∧ (x,y) ∈ Reachable }
until Reachable_new == Reachable
```

This is a fixed-point iteration. In petgraph terms, it is transitive closure with an 
early-exit predicate. GenSense already computes this manually in the BFS loops in `ir.rs`. 
The improvement is to:

1. Compute transitive closure *once* at project scan time (cached in `AnalysisRegistry`)
2. Express all reachability queries as set membership on the closure
3. Compose predicates: "guarded path" = path ∩ guard_nodes ≠ ∅

The practical gain: the `MustHaveGuard` constraint currently does BFS per source symbol. 
With pre-computed transitive closure, it becomes a hash-set lookup. For a 5,000-function 
project, this is the difference between O(n²) and O(n) for project-rule evaluation.

**More importantly for LLM code:** Datalog composition enables a rule that does not 
exist today — **path-sensitive guard validation**:

> "Every call path from a function whose name matches `process_*payment*` must pass 
through a function matching `validate_*` before reaching any function matching `db.*`."

This is inexpressible in the current `ProjectFlowConstraint` IR because it requires 
*composing* three predicates on a path. Datalog fixed-point makes this natural. The 
YAML rule would add a `must_pass_through` field:

```yaml
- id: PAYMENT_MUST_VALIDATE
  source_pattern: process.*payment
  sink_pattern: db\.(query|execute|insert)
  must_pass_through: validate.*
```

This is the most direct algorithmic path to catching the largest class of LLM security 
errors: functions that look like they validate but the LLM generated the call graph in 
the wrong order.

---

### 3. Entropy Measurement on the Taint Lattice
**Already has:** `TaintRegistry` as a scoped lattice, `SemanticOp` normalization  
**Add:** Shannon entropy over branch coverage of the taint lattice

This is the most novel of the three — it does not appear in existing static analyzers 
in this form.

**The observation:** The taint lattice for a function assigns each variable one of 
{clean, tainted, unknown}. A well-implemented function that handles user input *should* 
have high entropy in its branch coverage — it branches on whether inputs are valid, 
which means the clean/tainted states are meaningfully separated by conditionals.

An LLM-generated function that appears to validate but does not will show *low entropy* 
in its branch structure relative to its taint surface. The inputs flow through without 
being branched on.

**Shannon entropy** over branch decisions touching tainted variables:

```
H = -Σ p(branch_i) × log₂(p(branch_i))
```

where `p(branch_i)` is the probability (in the uniform input model) that a tainted 
variable reaches that branch. A function with H near 0 means tainted data flows through 
without conditional decisions. A function with H near log₂(n) (maximum entropy) means 
tainted data is fully branched on.

**In practice, this does not require probabilistic modeling.** GenSense already has:
- The taint registry (which variables are tainted)
- The AST (which nodes are branch conditions)
- The semantic ops (which ops involve tainted variables)

The metric to compute is simpler than full Shannon entropy: **taint-touching branch ratio**:

```
taint_branch_ratio = (conditionals containing tainted vars) / (total tainted uses)
```

A ratio near 0 = tainted data flows through without being checked = LLM likely 
generated plausible-looking code that does not actually validate.

This becomes a new signal on every taint-reachable function, not just a per-rule check. 
It slots naturally into the `DataFlowAnalyzer` as a metric computed during `analyze_block`:

```rust
pub struct TaintMetrics {
    pub tainted_uses: usize,
    pub taint_branched_on: usize,
    pub taint_branch_ratio: f32,  // taint_branched_on / tainted_uses
}
```

The new rule `AI_TAINT_BYPASS` fires when `taint_branch_ratio < 0.2` in a function 
whose name implies validation. This catches the case the current rules cannot: a 
`validate_user_input` function that receives tainted data, does some operations with it, 
but never branches on whether it is valid — the LLM wrote the skeleton without the logic.

**Why entropy specifically:** It is the right measure because it is *relative to the 
taint surface*. A utility function that is supposed to pass data through (a serializer, 
a logger) legitimately has low taint-branch ratio. A validator that has low taint-branch 
ratio is wrong. The name-based contract promise from the previous document is the 
*selector* that decides which functions the entropy check applies to.

These two ideas — contract surface analysis and taint entropy — are designed to compose. 
Contract surface says "this function promises validation." Taint entropy says "this 
function does not branch on its tainted inputs." Together they produce a finding with 
near-zero false-positive rate.

---

## How These Three Compose

The three algorithms are not independent additions. They form a layered system:

```
Layer 1: MinHash similarity (function-to-function)
    → Identifies structurally similar functions
    → Feeds into: "these two similar functions have different taint entropy — one is wrong"

Layer 2: Datalog fixed-point (project-wide path reasoning)  
    → Replaces manual BFS with composable path predicates
    → Enables: must_pass_through guards, cross-file path safety
    → Feeds into: "this payment path has no validation node in its transitive closure"

Layer 3: Taint entropy (function-level branch coverage)
    → Measures how much tainted data is actually reasoned about
    → Feeds into: "this validator never branches on its tainted input"
```

The reason to introduce them together rather than separately: each one reduces the 
false-positive rate of the others. MinHash alone generates noise (similar functions are 
not always suspicious). Datalog alone is expressive but verbose. Entropy alone is 
imprecise about *which* functions to measure. Together:

- MinHash + Entropy: "These two similar functions have divergent taint entropy — one is an LLM copy that lost the validation logic"
- Datalog + Entropy: "The path from input to database has no high-entropy node in it — the validation step exists structurally but is a passthrough"
- All three: The full LLM-era failure signature — generated at scale, similar in structure, missing semantic grounding

---

## What Does Not Change

The snapshot model, the two-pass architecture, the YAML rule system, the NAPI bridge, 
the taint registry data structure — none of these change. These three additions are:

- MinHash: a new method on `FunctionFingerprint`, a new ProjectRule pass
- Datalog fixed-point: replaces the BFS loops in `ir.rs` with a pre-computed closure 
  stored in `AnalysisRegistry`
- Taint entropy: a new struct `TaintMetrics` produced alongside `Vec<Advisory>` in 
  `DataFlowAnalyzer::analyze_block`

The existing rule IR gets two new optional YAML fields: `must_pass_through` (Datalog) 
and `min_taint_entropy` (entropy threshold). Everything else stays backward-compatible.

---

## The Theoretical Claim

These three algorithms, combined with the contract surface analysis from the previous 
document, give GenSense a formal description of what it is doing:

> GenSense is a **compositional semantic analyzer** that checks behavioral contracts 
> (contract surface), path safety (Datalog reachability), structural consistency 
> (MinHash similarity), and decision coverage (taint entropy) — across the full call 
> graph of a codebase — without requiring type annotations, user-supplied specifications, 
> or compilation.

No other tool in the static analysis space has this combination applied specifically to 
the LLM failure mode. The reason is that each of these techniques was developed for 
different problems (MinHash for near-duplicate document detection, Datalog for program 
verification, entropy for information-theoretic security analysis) and none of the 
existing tool vendors has had reason to combine them under this framing.

The framing is the moat. The algorithms are all published and well-understood. What 
is new is their composition on top of GenSense's existing semantic substrate, aimed 
specifically at the structural failure patterns of LLM-generated code.
