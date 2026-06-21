# Frensense Thesis, Updated: Why Corpus, Why Not Rules, Why Not an LLM Detector

> Supersedes the framing (not the mechanics) of earlier research docs.
> Those docs are still useful for
> implementation detail; this doc is the "why" they were missing, written after a
> session of finding that the project's own engine still had regex in the one place
> the thesis says it shouldn't.

---

## The premise that changed

Pre-LLM, the bug-finding landscape split two ways: hand-written rules (Semgrep, ESLint,
CodeQL's QL libraries) or compiler-enforced invariants (Rust's borrow checker, type
systems). Both predate a world where most new code is LLM-written, shipped same-day,
and patched same-hour. Two things follow from that:

1. **Rules encode what a human anticipated.** An LLM doesn't write bugs the way a human
   does — it writes confident-looking code with a specific, learnable failure
   signature (a validator that always returns true, a sanitizer that logs and passes
   through, an auth check with a fallback identity instead of a rejection). Semgrep's
   rule library was built against human bug patterns. It has no opinion on LLM
   patterns because it predates them.
2. **The fix cycle compressed from weeks to hours.** A rule-based tool's blind spot is
   permanent until someone notices and writes a rule. A codebase that ships fixes
   hourly needs a detector that generalizes to a *shape* it has never seen, not one
   that waits for a human to name the shape first.

This is the actual argument for corpus-based, contrastive-example detection over a rule
DSL: it's not just "rules are tedious to maintain," it's "the thing producing the bugs
now doesn't think in rules, so a detector that only knows rules is structurally behind."

## What Rice's theorem actually constrains here

Worth being precise about this, because it changes what Frensense can honestly claim.
Rice's theorem says: for any non-trivial semantic property of programs (true of some,
false of others, and dependent only on what the program *does*, not how it's written),
there is no general algorithm that decides it correctly for every possible program.
"Does this function have a SQL injection vulnerability" is exactly such a property.

That means no static analyzer — Frensense, CodeQL, Semgrep, the Rust compiler's lint
passes, anything — can be both sound (no false positives) and complete (no false
negatives) on arbitrary code. There's always a tradeoff surface. This isn't a
limitation specific to Frensense's design; it's why every real static analyzer ships
a severity/confidence tier instead of a binary "safe/unsafe" verdict, and it's why
running tests, fuzzing, and dynamic analysis catch a different slice of bugs than
static analysis ever can — those tools observe what the program actually does at
runtime, not what its structure implies it might do.

**What this should change in how the project talks about itself:** nothing about the
architecture, but the docs and README should never imply Frensense *proves* a codebase
is bug-free or *decides* correctness. The honest claim — already structurally true of
the corpus-confidence-threshold design — is "raises confidence-scored hypotheses about
structural similarity to known-bad and known-good shapes." That's compatible with
missing bugs and compatible with occasional false positives. It is not compatible with
"static analysis replaces testing," and the docs should say so explicitly rather than
leave it implied. This is a documentation fix, not a code fix — but it's worth making
because overclaiming what static analysis can do is exactly the kind of credibility gap
that makes a corpus-based tool indistinguishable from marketing to a skeptical reader.

## Why regex is specifically wrong for this architecture (not just generally fragile)

This session found the clearest concrete example of the principle: `T-FIX-1` in
`ROADMAP.md` is still open. Taint *source seeding* — deciding whether a variable's
value came from untrusted input — currently works by regex matching on identifier
names. A variable named `url` taints because the word "url" matched a pattern, not
because it's provably derived from `req.query`. That's 585 false positives on the axum
benchmark, all from one rule.

The reason this matters more than "regex is generally brittle" is that it's the same
mistake the corpus thesis already rejected once, showing up in a different layer. The
argument against `CONTRACT_SURFACE_*` name-pattern rules (this session's earlier CSA
rework) was: matching on surface text (a name, a prefix) instead of structure misses
synonyms and can't tell intent from shape. Regex source seeding is the identical
mistake — matching on the *name* "url" instead of the *structural fact* "this value
flows from an Axum `Query<T>` extractor." Fixing T-FIX-1 isn't a separate taint-engine
task that happens to also be on the roadmap; it's the same principle the rest of this
session's work was built on, applied to the one place it hasn't been applied yet. Until
it ships, "trace bugs semantically across files" is not yet true of the taint layer —
it's true of the corpus layer (fingerprint similarity, not name matching) but not yet
of taint (still name matching).

## Why an LLM-as-detector is the wrong tool, and what the MCP server actually is

Worth disambiguating one thing precisely, because "train on CVEfixes so it understands
structure" and "MCP server" can get conflated into "use an LLM to catch bugs," which is
not the design and not what should get built:

- **Training a model to *detect* bugs** means non-deterministic output, a cost per scan,
  latency that scales with code size, and a black-box decision that can't cite which
  known-bad example a finding resembles. This is the Semgrep-but-slower-and-fuzzier
  failure mode the project already wants to avoid.
- **What Frensense actually does:** a deterministic Rust engine matches structural
  fingerprints against a corpus of positive/negative example pairs — no model inference
  at detection time, sub-second, and every finding can cite the specific corpus pattern
  it matched. The corpus *data* can absolutely be LLM-sourced (prompt an LLM for
  realistic vulnerable code, then for the fix — this is already milestone M2's
   "LLM-generated corpus" in the scaling plan), but the *matching* stays
  non-LLM.
- **What the MCP server is for:** exposing that deterministic engine as a tool an
  LLM/agent can call mid-generation (Claude Code, Cursor, any MCP client) — "catch it
  while it's written" means the static engine runs inside the generation loop, not that
  an LLM judges its own output. The cost/risk problem with LLM-as-detector doesn't apply
  here because the detector itself never invokes a model.

## What "the LLM's own implicit rules" actually means, and where it's already being built

The phrase "what if the LLM rules are what the rules can't catch" names something real
and specific: LLMs have learnable, repeated failure shapes — not random bugs, but
confident-looking anti-patterns baked in by training (a validator that always passes,
a sanitizer that logs instead of transforming, an auth check with a fallback identity).
This is not a future problem to solve — it's exactly what this session's CSA work and
the `llm` corpus category already target, and the scaling plan's "10 most common
AI-generated anti-patterns" line is the same idea stated as a milestone target. The gap
isn't conceptual; it's coverage and volume — there are 4 CSA violation types with 8
pairs total right now. Treating "LLM-typical anti-pattern" as its own deliberately
tracked corpus line (not folded into general `sec`) and growing it specifically by
prompting LLMs for real code and capturing their failure modes is the concrete next
step, not a new category of work.

## Concrete gaps, ranked by what they unlock

| Gap | What it unlocks | Status |
|---|---|---|
| **T-FIX-1** — AST-based taint entry points, replacing regex source seeding | Makes "trace across files" actually structural rather than name-matching; un-caps the value of every corpus finding that relies on taint corroboration | Open, confirmed this session. ~450 lines, 2–3 days, specs written (`ROADMAP.md`) |
| **Dark pattern backlog** — path traversal (20) + command injection (18) | Detection capability that's already written (positive examples exist) but inert | Open, queued from last session |
| **LLM-anti-pattern corpus growth** | The specific "bugs LLMs write that rules never anticipated" — the core differentiator vs. Semgrep-era tools | Conceptually proven (CSA), needs volume |
| **AST edit-distance blending** (`ROADMAP.md` M2) | Keeps "generalizes to a novel structurally-similar bug" from degrading into false positives as corpus grows past ~1,000 patterns | Designed, not wired |
| **Docs/claims correction** | Stops the project from implying static analysis decides correctness — protects credibility, costs nothing technical | Not started |

T-FIX-1 is first on this list because it's the one gap that's both fully specified and
sits underneath several others — taint corroboration affects how much every corpus
finding (including the dark-pattern fixes from last session) is actually worth.
