# FrenSense: The Unexplored Space
## Strategic Research Direction for the LLM-Written Code Era

**Prepared:** May 2026  
**For:** FrenSense 3.0+ Research Planning

---

## The Core Observation

FrenSense was built with a powerful insight: static analysis tools designed for human-written code miss a whole class of semantic bugs because they look at *syntax*, not *intent*. The snapshot model, taint summary engine, and temporal analyzer are all expressions of that insight applied to code humans write.

But the world has shifted. LLMs now write most of the code in active development. This does not make FrenSense obsolete — it makes FrenSense uniquely positioned, because LLMs introduce a *new class of failure mode* that no existing tool category is designed to catch.

The unexplored space is this:

> **LLM-generated code is not random. It fails in systematic, predictable, statistically consistent ways — and those patterns are invisible to every tool that treats correctness as a binary property.**

FrenSense is already the closest tool in existence to catching this. But it is currently catching the *symptoms* (placeholder panics, tautological assertions, redundant comments) rather than the *structure* of LLM failure.

---

## What the Research Record Shows

The program analysis literature from 2010–2020 built the foundations FrenSense runs on:

- **Typestate analysis** (Fink et al., 2008) — sequences of valid operations matter, not just individual calls. FrenSense's temporal analyzer is a direct expression of this.
- **Points-to and taint analysis** (Arzt et al., FlowDroid 2014) — data provenance must be tracked through the heap and across function boundaries. FrenSense's cross-file taint is this applied without the full heap model.
- **Compositional verification** (Cousot & Cousot, abstract interpretation) — summaries instead of inlining. FrenSense's taint summary model is precisely this trade-off.
- **Property-based specification** (CSLang, SLIC) — user-defined behavioral contracts beyond types. FrenSense's YAML rule system is this for patterns rather than full contracts.

What research *did not* address, because the problem did not exist, is the failure signature of a system that has learned to produce *plausible-looking but semantically hollow* code. That is a new problem class. The research for it does not exist yet.

---

## The New Problem Class: Semantic Confidence Without Semantic Grounding

When a human writes a bug, it is usually local: a wrong operator, a missing null check, a misunderstood API. The surrounding code reflects genuine reasoning about the problem.

When an LLM writes a bug, the failure mode is different in kind:

**The code is coherent locally but incoherent across its obligations.** The function signature promises something the body does not deliver. The test asserts something that does not exercise the actual risk. The comment describes a behavior the implementation does not have. The error handler looks right but silently succeeds on the exact case that should fail.

These are not syntax errors. They are not type errors. They are not even logic errors in the traditional sense. They are **contract surface violations** — gaps between what the code *claims* to do at its interface boundary and what it *actually does* under analysis.

FrenSense already has three detectors for the most obvious surface expressions of this:
- `AI_TAUTOLOGICAL_ASSERT` — the test looks like a test but cannot fail
- `AI_USELESS_TEST` — the test has the form of a test but no assertions
- `AI_PLACEHOLDER_PANIC` — the function has the form of an implementation but panics unconditionally

These are the tip. The full structure underneath them has never been formalized.

---

## The Unexplored Space: Contract Surface Analysis

The research gap is a discipline that does not yet have a name. Provisionally: **Contract Surface Analysis (CSA)**.

The core idea:

> Every symbol in a codebase implicitly or explicitly carries a behavioral contract — what it promises to callers, what it requires from inputs, what invariants it maintains across calls. In human-written code, violations of this contract are rare and usually mechanical. In LLM-generated code, violations are *structurally common* because the LLM optimizes for local plausibility rather than global contract fulfillment. CSA is the discipline of statically verifying that contract surfaces are coherent.

This is different from what Semgrep, CodeQL, or existing static analyzers do. They check known-bad patterns. CSA checks whether the *relationship* between a symbol's promise and its implementation is coherent — without requiring the human to specify the contract in advance.

---

## What This Looks Like Concretely in FrenSense

FrenSense already has all the infrastructure needed to build this. Here is what the new capability looks like at each layer:

### Layer 1: Promise-Body Coherence (No Database Required)

The simplest form: given a function, does the body do what the name and signature imply it does?

FrenSense already has:
- `RedundantComment` — name-comment overlap detection
- `SymbolRegistry` with full name and signature
- `SemanticGraph` with call edges

What it does not have: a model of what a function *name* implies about its behavior, checked against what its *body* actually does.

**Concrete example:**
```rust
// Function name promises validation
fn validate_user_input(input: &str) -> bool {
    true  // LLM placeholder — always returns true
}
```

A name-containing `validate` implies: the body must contain at least one conditional branch that can return `false`. A body that unconditionally returns `true` is a contract surface violation. This is detectable with FrenSense's existing AST infrastructure — no database required.

The pattern generalizes:
- Names containing `validate`/`verify`/`check` → must contain a conditional that can return a falsy/error value
- Names containing `sanitize`/`clean`/`escape` → must transform the input, not return it unmodified
- Names containing `ensure`/`assert`/`require` → must contain a guard that can panic/error
- Names containing `find`/`get`/`fetch` → must contain code that can return None/null/empty
- Names containing `create`/`build`/`make` → result must be used or returned, not discarded

Each of these is a structural rule derivable from the name, checkable against the body AST. Zero ML, zero database, pure static analysis. FrenSense can express all of these in its existing rule IR.

---

### Layer 2: Test-to-Implementation Contract Coherence

This is where FrenSense's multi-file analysis creates a genuinely new capability.

The current `AI_USELESS_TEST` only checks if a test has assertions. What it does not check: whether the assertions test the *right thing* relative to the implementation.

With FrenSense's `SymbolRegistry` and call graph, you can check:

**Does the test exercise the code path that the implementation's name implies is risky?**

```typescript
// Implementation claims to handle auth
async function authenticateUser(token: string): Promise<User> {
    // LLM wrote 40 lines of real-looking auth code
    // But the token expiry check is bypassed with: if (true) {
    return user;
}

// Test looks complete
test("authenticateUser validates token", async () => {
    const user = await authenticateUser("test-token");
    expect(user).toBeDefined();  // Only tests non-null return
    // Never tests expired token, never tests invalid token
});
```

The advisory here is not "your test has no assertions" — it has one. The advisory is: **"Your test for `authenticateUser` does not exercise the negative path. Authentication functions require at least one test case where the input is invalid or expired."**

This requires cross-file reasoning: find the test for function X, check whether the test exercises the contract-critical paths of X. FrenSense's `ProjectRule` infrastructure is exactly the right abstraction for this. The `MustHaveGuard` constraint is the starting point — this is a new constraint type: `TestMustExerciseNegativePath`.

---

### Layer 3: The Coherence Graph — Where Memory/Database Becomes Relevant

This is the long-term research direction and the one that requires introducing persistent storage.

The insight: LLMs do not produce random violations. They produce *consistent* violations across similar contexts. If an LLM generated a `validate_*` function that always returns `true` in your codebase, there are probably more of them. The violation is not one symbol — it is a *class* of symbols with a shared contract surface failure.

What a Coherence Database would store:

```
symbol_name | contract_promise | body_actually_delivers | coherence_score | first_seen | last_seen
"validate_user_input" | {must_branch, must_have_false_path} | {unconditional_true} | 0.1 | ... | ...
"authenticate_user"   | {must_verify_expiry, must_have_error_path} | {no_expiry_check} | 0.2 | ... | ...
```

This is structurally closer to what Semgrep's registry does for known-bad patterns, but inverted: instead of storing *what bad code looks like*, you store *what good code for a given contract promise should look like*, and flag deviations.

The database enables things that per-run analysis cannot:

1. **Cross-run coherence drift detection** — "This function had coherence score 0.9 last month and now has 0.3. Something changed."
2. **Codebase-wide contract surface summaries** — "22% of your `validate_*` functions have no false-path. This is a systemic LLM output problem, not isolated bugs."
3. **Learning from fixes** — When a developer corrects a contract surface violation, the fix becomes evidence for what coherent code for that contract type looks like.

The database does not need to be large or complex. A SQLite file with three tables (symbols, contract_promises, coherence_runs) is sufficient for the first version. This is far simpler than Semgrep's registry because you are not storing patterns — you are storing observations about your own codebase's history.

---

## Why This Space Is Unexplored

Three reasons this research gap has not been filled:

**1. The problem is new.** Two years ago, LLMs did not write most code. The failure mode did not exist at scale. No research program was aimed at it because there was no corpus to study.

**2. Existing tools are pattern-oriented, not contract-oriented.** Semgrep, CodeQL, and ESLint all work by matching known-bad patterns. Contract surface analysis requires reasoning about the *relationship* between a symbol's identity and its behavior — a fundamentally different operation.

**3. The required insight — that names carry implicit behavioral contracts — was considered too imprecise for formal analysis.** Formal methods requires specification. Name-based contract inference seems informal. But in the LLM era, names are where LLMs are *most* reliable (they name things correctly) and bodies are where they are *least* reliable (they implement plausibly but shallowly). This asymmetry is exploitable.

---

## What FrenSense Can Build That No One Else Can

FrenSense's specific architectural advantages for this space:

| FrenSense Capability | How It Enables CSA |
|---|---|
| Tree-sitter AST for multiple languages | Name + body structure extraction without ML |
| `SymbolRegistry` with cross-file graph | Contract surface visible across the whole project |
| `ProjectRule` with BFS over call graph | Cross-file test-to-implementation coherence checking |
| Snapshot model | Per-run coherence scores are deterministic and cacheable |
| YAML rule extensibility | Teams can define their own contract promise patterns |
| Existing `AI_*` detector family | Natural home for new contract surface rules |

No other tool has this combination. Semgrep operates on patterns, not symbol graphs. CodeQL requires a full compilation and a query language most developers cannot write. Existing AI-code detectors operate on heuristics without the semantic substrate.

---

## Proposed Research Roadmap

### Phase 1: Name-Body Coherence Rules (No New Infrastructure)
*~3-4 weeks, buildable today*

Add a new rule family: `CONTRACT_SURFACE_*`

- `CONTRACT_SURFACE_VALIDATE_UNCONDITIONAL` — `validate_*` functions with no false path
- `CONTRACT_SURFACE_SANITIZE_PASSTHROUGH` — `sanitize_*`/`escape_*` that return input unmodified
- `CONTRACT_SURFACE_FIND_NEVER_EMPTY` — `find_*`/`get_*` that cannot return None/null
- `CONTRACT_SURFACE_AUTH_NO_REJECTION` — `auth*`/`authenticate*` with no error path

Each rule: parse the function name, extract the contract promise class, check the body AST for structural evidence of that promise being honored.

**Expected result:** Catches the single most common LLM output failure. High precision because the name-to-contract mapping is tight. Fits naturally into the existing YAML rule system.

---

### Phase 2: Cross-File Test Coverage Coherence
*~4-6 weeks, requires multi-file rule extension*

Add a new `ProjectFlowConstraint` variant: `TestMustExerciseNegativePath`

For functions in a given glob (e.g., `**/auth*`, `**/validate*`, `**/payment*`), verify that the test file exercising them contains at least one case with an input that should produce an error/null/false result.

This requires:
- Extending the `ProjectRuleIr` with a new constraint type
- Building a "test file finder" that maps implementation symbols to their test counterparts
- Checking test call sites for negative-input patterns

**Expected result:** Eliminates the largest class of LLM-generated test inadequacy. Especially valuable for payment, auth, and validation code — exactly the critical paths LLMs get wrong.

---

### Phase 3: Coherence Database
*~6-8 weeks, introduces persistent storage*

A lightweight SQLite-backed coherence store, invoked as `gensense --track`:

```
gensense audit . --track
# Writes coherence observations to .gensense/coherence.db
```

Stores: symbol → contract promise class → coherence score → run timestamp.

Enables new CLI surface:
```
gensense coherence-report
# Shows: symbols with declining coherence scores, systemic failure classes,
# functions that have never had their negative path exercised in any run
```

The database schema is simple:
```sql
CREATE TABLE symbols (id, name, file_path, first_seen, last_seen);
CREATE TABLE contract_promises (symbol_id, promise_class, honored BOOL, run_id);
CREATE TABLE runs (id, root_path, timestamp, gensense_version);
```

This is the minimum viable memory system. It does not require an LLM, a vector store, or any ML infrastructure. It is a structured audit trail of contract surface observations.

**Expected result:** Enables the only static analysis tool that can tell you "your codebase's coherence has declined 12% since you started using AI-generated code at scale." That is a new category of finding with no competitor.

---

## The Positioning

If you build Phase 1 alone, FrenSense becomes the first static analyzer with a dedicated theory of LLM output quality. That is a defensible, novel position.

If you build all three phases, FrenSense becomes something that does not exist: a **semantic coherence auditor** — a tool that tracks whether the promises a codebase makes about its own behavior are actually honored, run over run, as LLMs write more of it.

Semgrep catches known bad patterns. FrenSense would catch *unknown systematic failures* — the ones that emerge specifically from how LLMs reason about code, which is coherent locally and incoherent globally.

The research is unexplored. The infrastructure to build it is already in your codebase. The moment to build it is now, because in two years this problem will be well-understood and the space will be crowded.
