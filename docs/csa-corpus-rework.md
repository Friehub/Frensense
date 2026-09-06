# Contract Surface Analysis — Corpus-Learned Rework

> Supersedes the CSA proposal from earlier research.
> That document's Layer 1 and Layer 2 are rule tables wearing a research framing.
> This rework keeps the CSA *concept* — code makes implicit promises that bodies can
> violate — but routes detection entirely through the corpus pipeline that already
> exists, instead of introducing a new hand-maintained rule family.

---

## Why the original proposal breaks the "no rules" principle

The original Layer 1 design (`CONTRACT_SURFACE_*`) works like this: maintain a lookup
table mapping name prefixes to promise classes —

```
validate/verify/check  → must contain a conditional with a falsy return
sanitize/clean/escape  → must transform input, not return it unmodified
ensure/assert/require  → must contain a guard that can panic/error
find/get/fetch         → must contain code that can return None/null/empty
create/build/make      → result must be used or returned, not discarded
```

— then write one hand-coded AST predicate per row to check the body against the promise.

This is structurally identical to the YAML `CoreRuleIr` system v0.4.0 already deleted.
It has the same failure modes:

- **Closed vocabulary.** `isLegit`, `confirmEligibility`, `assertCompliance` don't match
  the table. LLMs (and humans) vary naming constantly; every miss is a silent false negative.
- **One predicate per class, hand-written.** Adding `normalize_*` or `ensure_*` means
  someone writes a new Rust function, not two example files.
- **No confidence gradient.** The check is binary — table match or no match — so it can't
  participate in the 4-layer AND gate the way corpus scores do.

Layer 2's `TestMustExerciseNegativePath` has the same shape with globs
(`**/auth*`, `**/validate*`, `**/payment*`) standing in for the name table.

---

## The rework: CSA is a corpus category, not a rule family

Frensense already has the category — `AGENTS.md` lists `csa` (Contract Surface Analysis)
alongside `sec`, `llm`, `arch`, `async` as a recognized pattern category, with examples
like `rust_csa_validate_unconditional_positive.rs` already in the naming convention.
The rework formalizes that this is the *only* mechanism — the `CONTRACT_SURFACE_*` rule
family in the research doc should never be built.

### Layer 1 — Promise-Body Coherence, as example pairs

Instead of a name table + predicate functions, each violation type becomes a normal
corpus pair, scored by the existing 5-dimensional contrastive scorer
(`frensense-engine/src/pattern/scorer.rs`):

```
corpus/targets/rust_csa_validate_unconditional_positive.rs   -- always returns true
corpus/targets/rust_csa_validate_unconditional_negative.rs   -- branches on input
corpus/targets/ts_csa_sanitize_passthrough_positive.ts       -- returns input unmodified
corpus/targets/ts_csa_sanitize_passthrough_negative.ts       -- actually encodes/strips
corpus/targets/rust_csa_auth_no_rejection_positive.rs
corpus/targets/rust_csa_auth_no_rejection_negative.rs
corpus/targets/ts_csa_find_never_empty_positive.ts
corpus/targets/ts_csa_find_never_empty_negative.ts
```

What this buys, mechanically:

| What the rule table hand-encoded | What the existing fingerprint already captures |
|---|---|
| Name → promise class lookup | `name_segments` (camelCase/snake_case split) scored via Jaccard against the pair — `validateUserInput` and `verifyEligibility` land close in this space without a maintained synonym list, because both decompose to overlapping segments and similar `signature_ngrams` |
| "Body must contain a falsy-returning conditional" | `structural_markers` (AST node kind hashes) — a body with no reachable false path scores high against the `unconditional_true` positive and low against a branching negative. This is the identical mechanism already used for SQL-injection shape matching, pointed at a different corpus bucket |
| One predicate per new class | Two files per new class |

No new `ProjectFlowConstraint`, no new YAML field, no new Rust predicate function.
Phase 1 of the original roadmap ("buildable today") becomes: author ~15–20 corpus
pairs across the four violation types, in Rust and TypeScript. It plugs into the
`PatternRegistry` / FRC bundle pipeline that exists right now.

This also improves with scale in a way the rule table can't: every additional pair
sharpens the contrastive boundary; the rule table's coverage is fixed until someone
edits code.

### Layer 2 — Test-to-Implementation coherence, without globs

Original: a hand-maintained glob list decides which functions need "negative path"
test coverage, and a hand-written check looks for negative-input call sites in the
paired test.

Rework: fingerprint the **(implementation, test) pair** as one unit instead of
fingerprinting the implementation alone, and score that pair against corpus examples
of the same shape:

```
corpus pair:
  positive = (impl with no error/expired/invalid-handling code path,
              test that only calls the happy path)
  negative = (impl with explicit rejection logic,
              test that calls it with an invalid/expired/empty input)
```

Mechanically this needs one new engine primitive — pairing an implementation's
existing `SymbolRegistry` call-graph node with its test counterpart (test discovery is
already a cross-file graph traversal Frensense does elsewhere, not a new globbing
layer) — and a joint fingerprint that concatenates the impl's taint-entry signature
with the test body's call-site argument shapes. That joint fingerprint goes through
the *same* `score_against_corpus` function already in `scorer.rs`. No new constraint
type, no `must_pass_through` field, no maintained path list.

The practical difference: a glob either fires or doesn't. A corpus score is a
confidence the existing AND gate already knows how to combine with taint
verification — `auth*` functions that happen to be named oddly aren't silently
skipped, and globs that overreach don't manufacture noise.

### Layer 3 — Coherence Database: keep, drop the taxonomy column

The original schema:

```sql
contract_promises(symbol_id, promise_class, honored BOOL, run_id)
```

`promise_class` was going to be populated from the name table. Replace it with
`pattern_id` — the identifier `PatternMatch` already returns from the FRC bundle:

```sql
contract_observations(symbol_id, pattern_id, score, run_id, timestamp)
```

There's no separate taxonomy to define or maintain. The taxonomy *is* the corpus —
it only grows by adding example pairs, the same way every other detection category
already grows. Everything else in the original Phase 3 (drift detection, systemic
failure-class summaries, "fixes become evidence") is unchanged and is actually a
better fit this way: "a developer's fix becomes evidence" already describes turning a
correction into a new corpus pair, which is exactly how the existing `_positive`/
`_negative` mechanism works.

---

## What to explicitly retire from the roadmap

- The name → promise-class lookup table (`validate/verify/check`, `sanitize/clean/escape`,
  `ensure/assert/require`, `find/get/fetch`, `create/build/make`)
- The `CONTRACT_SURFACE_*` rule family and any dedicated YAML/IR fields for it
- The glob-based `TestMustExerciseNegativePath` constraint and its path lists

None of this code should be written. If any of it already exists from a prior session,
it's a deletion candidate, not a wiring task.

---

## Concrete next step

1. Author the first ~16 `csa` corpus pairs (4 violation types × Rust + TS) using the
   existing naming convention — zero engine changes, slots into the current
   `PatternRegistry`/FRC pipeline today.
2. Layer 2 needs one real engine addition: impl↔test pairing + joint fingerprinting.
   This is new code, but it's an extension of `fingerprint.rs`'s existing dimensions,
   not a new rule IR or constraint type.
3. Layer 3's schema only needs the `promise_class` → `pattern_id` rename whenever it's
   actually built — no urgency, no design risk.

---

## Net effect

CSA was the one piece of the LLM-era research direction that reintroduced a rule table
under a research label. Reworked this way, every detection category in Frensense —
corpus shape matching, MinHash consistency, taint entropy, and now CSA — is
example-driven end to end. No category requires a human to enumerate cases in code;
every category requires only the example pairs the corpus model already demands.
