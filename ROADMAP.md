# Frensense v0.4.0 — Improvement Roadmap (2026 Edition)

> Written from full read of `tasks.md`, `frensense-engine/src/`, `src/`, and `corpus/targets/`.
> This is the single source of truth for what to build next. Ordered by impact.

---

## State of the Engine Right Now

The agent has shipped significantly more than earlier audits reflect. Key completed work:

- **89 corpus patterns** in the embedded FRC bundle (SP1–SP3 done: cmd injection × 10, SQL
  injection × 10, path traversal × 10, in Rust + TypeScript)
- **IDF-weighted n-gram scoring** (M1) — rare tokens score higher than common ones
- **Positional n-gram hashing** (M9) — `return` at line 5 vs. line 50 are different hashes
- **Cross-lingual penalty** (M8) — 25% score reduction when pattern language differs from target
- **Taint rules externalized to TOML** (E1) — TOML loader active, `--extra-taint-rules` flag works
- **Temporal violations wired** (W1) — `TEMPORAL_VIOLATION` advisories fire on lock/unlock mismatches
- **Hallucinated imports wired** (W7) — `HALLUCINATED_IMPORT` catches missing Cargo.toml entries
- **Dead branch detection wired** (W2) — `DEAD_BRANCH` advisories for `if false` / `if true`
- **Def-use / unused variables wired** (W3) — `UNUSED_VARIABLE` from CFG analysis
- **Taint entropy (L3) wired** (B7/A1) — hollow validators (ratio < 0.2) get 60% confidence reduction
- **Style profile wired** (W6) — `STYLE_ANOMALY` and `NEAR_DUPLICATE_FUNCTION` active
- **TP/FP tracking system** (F8) — `classify_findings.py` + `compute_metrics.py` in scripts/

Open state after 0% precision on axum: taint source seeding is still regex-based. This is the
primary remaining blocker for precision.

---

## Section 1: Taint Precision (The Gate)

The 585 false positives on axum are all from `TAINT_INPUT_TO_HTTP`. The source seeding
matches variables by name (regex on identifier text). A variable named `client` or `url`
matches because those words appear in the source regex — not because the value actually came
from user-controlled input.

### T-FIX-1: AST Entry-Point Based Source Seeding

**What to build:** `src/semantics/taint_entry_points.rs`

```rust
pub enum TaintEntryPoint {
    // Axum extractors — the type IS the source signal
    AxumPath,      // Path<T> parameter in handler function
    AxumQuery,     // Query<T> parameter
    AxumBody,      // Json<T>, Form<T>, Bytes parameter
    AxumHeader,    // HeaderMap, TypedHeader parameter

    // Actix
    ActixPath,     // web::Path<T>
    ActixQuery,    // web::Query<T>
    ActixJson,     // web::Json<T>

    // Express-style TS (tree-sitter detects req.params, req.query, req.body)
    ExpressParams, // req.params.*
    ExpressQuery,  // req.query.*
    ExpressBody,   // req.body.*
    ExpressHeaders,// req.headers.*

    // Fastify
    FastifyRequest,// request.params, request.query, request.body

    // Generic: function parameter from network boundary
    NetworkBoundary, // fn handler(data: T) where T is deserialized from network
}
```

**How it works:**
1. Walk the AST for handler function signatures
2. For Rust: detect parameter types matching `Path<_>`, `Query<_>`, `Json<_>`, `Form<_>`
3. For TypeScript: detect `req.params`, `req.query`, `req.body` member access patterns
4. Seed taint on those specific AST nodes, not on identifier name matches
5. Propagate from those nodes through the data flow graph

**Expected impact:** `TAINT_INPUT_TO_HTTP` false positive rate drops from 100% to under 15%.
Variables named `url` that are not derived from an HTTP parameter will not be tainted.

**File:** `src/semantics/data_flow/resolve.rs` — replace the `COMBINED_SOURCE_RE` match in
`analyze_call` with a call to `is_taint_entry_point(node, lang)` that checks AST structure.

---

### T-FIX-2: Type-Narrowing Along Propagation Paths

**What to build:** When taint propagates through a type assertion or validation function,
reduce confidence rather than maintain full taint confidence.

```rust
// Current: validation functions are detected by name (hollow validator check)
// New: detect when taint passes through a function that:
//   1. Has a parameter that is the tainted value
//   2. Has at least one branch that does NOT return/propagate the value (guard clause)
// If those conditions hold → reduce taint confidence by 40%
```

This is the complement to L3 entropy. L3 checks if the validation function is hollow.
T-FIX-2 checks if taint passes through a real guard.

---

### T-FIX-3: Sanitizer Recognition

The taint rules already have `sanitizers` in the TOML. The engine needs to stop taint
propagation when a sanitizer is called on the tainted value.

**Current state:** `taint_rules.toml` has a `sanitizers` field but `resolve.rs` does not
read it during propagation. Taint flows through `encodeURIComponent(userInput)` unchanged.

**Fix:** In `analyze_call`, if the called function name matches a rule's `sanitizers` list,
mark the return value as sanitized (taint confidence = 0) rather than propagating.

---

## Section 2: Corpus to 400 (The Coverage Goal)

Current: 89 patterns. Target: 400 validated patterns. 311 to build, in 5 phases.

### Phase 2 — Security Patterns (remaining 7 sub-domains × 10 pairs = 70 pairs)

SP1–SP3 are done. Remaining:

**SP4: Open Redirect (10 pairs)**
10 variations: req.query.url, req.body.next, req.headers.referer, req.params.url,
multi-hop through variable, template literal redirect, Rust axum redirect, helper function
passthrough, protocol-relative URL, `javascript:` URL scheme.

Naming: `ts_sec_open_redirect_{1-10}_{positive,negative}.ts`,
`rust_sec_open_redirect_{1-10}_{positive,negative}.rs`

**SP5: SSRF (10 pairs, extends existing `ts_ssrf`)**
10 variations covering: req.query.url → fetch, req.body.url → fetch, req.headers.host,
multi-hop variable, Rust reqwest with user URL, helper passthrough, header value, destructured
body, template literal URL, metadata URL.

**SP6: Prototype Pollution (10 pairs, TypeScript only)**
10 variations: obj[key] = value, Object.assign no filter, merge no key check, req.body →
obj, header → obj, multi-hop, lodash merge, destructured body → obj, nested assignment,
recursive merge. Negative always filters `__proto__`, `constructor`, `prototype`.

**SP7: Hardcoded Secrets (10 pairs)**
10 variations: API key literal, JWT string, AWS key, DB connection string, private key,
secret in variable, env var with hardcoded default (`process.env.KEY || "secret"`), config
object with secret, secret in comment, secret in log. Rust + TypeScript.

**SP8: Credential Flow (10 pairs)**
Credential value flowing to output: secret → log, secret → HTTP response, password → DB
without hash, token → error message, API key → fetch header (logged), credential → debug
output. This closes the TAINT_CREDENTIAL_TO_HTTP and TAINT_CREDENTIAL_TO_DB coverage.

**SP9: XSS Reflected (10 pairs)**
User input rendered into HTML response without escaping. Variations: string template,
innerHTML, document.write, server-side template injection, dangerouslySetInnerHTML.

**SP10: Timing Attack (10 pairs)**
String comparison on secrets using `===` instead of `crypto.timingSafeEqual` / `subtle.compare`.
Auth tokens, HMAC signatures, session IDs, API keys. Rust: `==` on secret bytes.

### Phase 3 — Hollow Implementation (60 pairs)

HP1–HP4 as specified in tasks.md. Each batch of 15 must have distinct function names,
variable names, and intermediate steps. The variation principle is strict — no two positives
should have identical bodies except the function name.

The corpus builder script must validate structural diversity across files in the same
sub-pattern. Two positives that are too similar inflate the fingerprint cluster and reduce
sensitivity. Minimum Jaccard distance between any two positives in the same category: 0.3.

### Phase 4 — LLM Anti-Patterns (70 pairs)

LP1–LP4 as specified in tasks.md. Key insight: source material should be real AI-generated
code, not synthetic examples. The six `ts_llm_*` and `rust_llm_*` entries in the current
corpus were human-authored. Phase 4 must use actual Copilot/Claude output as positive
examples — ask an LLM to implement a payment handler, a session validator, a file download
endpoint, and use the output directly as the positive example. The negative is the
corrected version.

This is the only reliable way to tune the detection to actual LLM output patterns rather
than human guesses about what LLMs produce.

### Phase 5 — Architecture and Resource Management (50 pairs)

AP1–AP3. Resource leak patterns (file handles, DB connections, HTTP connections, transactions,
sockets) are high-value for Rust because RAII makes the fix obvious — the negative is
always "wrap in a struct that implements Drop." For TypeScript, the negative uses
`using` declarations (TypeScript 5.2+ Explicit Resource Management).

AP3 (type system escapes) is the corpus coverage for Frensense's existing `ts_as_any_escape`
pattern — expanding from 1 pair to 15 covering the full space of type system bypasses.

### Phase 6 — Concurrency (60 pairs)

CP1–CP4. These are the highest-risk patterns to write because the negatives must be
genuinely safe, not just syntactically different. Every negative in CP1 (mutex held across
await) and CP2 (blocking in async) needs an expert review before committing.

**Validation requirement for Phase 6:** Each negative must be tested by running the Rust
async code under `loom` (the concurrency testing tool) or the TypeScript code under a race
condition test harness. If the negative triggers a race condition under testing, it is not a
valid negative.

---

## Section 3: ML Improvements (What Moves the Precision Number)

Ordered by expected precision improvement per implementation cost.

### M2: AST Edit Distance (High Impact, Medium Cost)

Jaccard over n-gram sets is a bag-of-words measure. Two functions that have the same tokens
in different control flow structures score identically. AST edit distance (tree edit distance
between the structural skeletons) catches structural similarity that n-gram misses.

**Implementation approach:**
- Extract the structural skeleton: remove all identifiers and literals, keep node kinds only
- Compute tree edit distance using Zhang-Shasha algorithm (well-documented, Rust crates exist:
  `edit-distance` or implement directly from the standard algorithm)
- Combine with n-gram score: `final_score = 0.6 * jaccard + 0.4 * (1.0 - normalized_edit_distance)`
- The edit distance component penalizes functions that have the same tokens but different
  structure — a loop vs. a recursive call vs. a sequence

**Expected impact:** Reduces false positives from structurally different functions that
happen to share token vocabulary.

### M3: Contextual Featurization (High Impact, High Cost)

A `sanitize_input` function in a web handler context is different from the same function in
a unit test. Current fingerprinting sees only the function body. M3 adds a context signature
from the call site.

**Implementation approach:**
- For each function, collect: the set of function names that call it (callers), the set of
  try/catch blocks it appears in, the set of `if` conditions guarding its call site
- Hash this into a `CallContextSignature`
- During matching, prefer corpus patterns that share call context with the target function
- This is a soft signal — weight it at 10% of the total score

**Expected impact:** Reduces false positives from utility functions that look like vulnerable
functions out of context.

### M5: Confidence Calibration via Platt Scaling (Medium Impact, Low Cost)

The current threshold (0.32 default, configurable) is hand-tuned. Score 0.32 does not mean
32% probability of a true positive — it is a raw similarity score.

**Implementation approach:**
- Take the labeled axum dataset (585 findings, all FP)
- Label 100 findings from a codebase that has known vulnerabilities (confirmed TPs)
- Train a logistic regression: `P(TP | score, pattern_category) = sigmoid(a * score + b)`
- Store `(a, b)` coefficients per pattern category in the FRC bundle
- Report `calibrated_probability` alongside raw score in findings output

**Expected impact:** Threshold setting becomes meaningful. Users can say "show me findings
with >70% probability of TP" rather than "show me findings with score > 0.32."

### M7: Edit-Based Feedback Loop (Medium Impact, Medium Cost)

When a finding is suppressed via `--emit-baseline`, the suppression should inform the
corpus. The features that caused the match — specific n-gram hashes — should be weighted
down for that pattern in that codebase.

**Implementation approach:**
- When a finding is added to the baseline, record the matching n-gram hashes
- These hashes become "negative evidence" for that pattern in this project
- Adjust the weighted Jaccard denominator to downweight those hashes
- This is per-project: the `frensense-baseline.json` stores the weight adjustments
- On next scan, the same function will score lower against the same pattern

**Expected impact:** Over 3–5 scan cycles, false positives on project-specific idioms
disappear without modifying the corpus.

### M6: One-Class Classification (Low Impact, High Value Long-Term)

Current corpus requires positive + negative pairs. You cannot detect novel bugs this way —
only bugs for which you have an example. M6 builds a "normal function" model from negatives
only and flags functions with anomalously low similarity to any negative.

**Implementation approach:**
- Build a MinHash LSH index of all negative examples
- For each scanned function, find its nearest negative neighbor
- If nearest-neighbor similarity < 0.2 (function is unusual relative to all clean examples),
  flag as `ANOMALOUS_FUNCTION` with low confidence
- This is exploratory detection — confidence is always low, requires human review

**Expected impact:** Catches novel LLM-generated patterns that have no corpus entry. These
will be high-noise initially but provide a research signal for where to add new corpus entries.

---

## Section 4: Features to Ship

### F1: `--fix` Flag (Auto-Remediation)

The patcher (`src/patcher/mod.rs`) is implemented. Not yet promoted to stable. Patterns
to support in the first stable release:

| Finding | Fix Applied |
|---|---|
| `ts_llm_console_log` | Replace `console.log(...)` with `logger.info({event: "...", ...})` |
| `ts_llm_promise_catch` | Add `.catch(err => logger.error({err}))` to floating promise |
| `ts_jwt_bypass` | Replace `jwt.decode(token)` with `jwt.verify(token, process.env.JWT_SECRET)` |
| `ts_hardcoded_secret` | Replace `const SECRET = "..."` with `const SECRET = process.env.SECRET ?? die("missing SECRET")` |
| `rust_llm_unwrap_in_handler` | Replace `.unwrap()` with `?` where function returns Result |
| `rust_panic_in_lib` | Replace `panic!(...)` with `return Err(...)` |

The fix must be scope-limited: `--fix style` only changes style patterns. `--fix security`
changes security patterns. Never apply both without explicit user flag.

### F3: `frensense acknowledge` (Team Suppressions)

Current baseline is anonymous — a finding is suppressed, no metadata why. Team workflows
need attribution.

```bash
frensense acknowledge --finding TAINT_INPUT_TO_HTTP:src/handlers.rs:42 \
  --reason "sanitized by validate_input() at line 38" \
  --reviewer "@username"
```

This writes to `frensense-suppressions.json` with author and reason. The baseline becomes
an audit trail, not just a noise filter.

### F5: Per-Category Thresholds

Security patterns should be more sensitive (lower threshold = more findings). Style patterns
should be more conservative (higher threshold = fewer findings). Current `--threshold` is
global.

```toml
# .frensense/config.toml
[thresholds]
sec = 0.28       # security: more sensitive
arch = 0.35      # architecture: medium
llm = 0.30       # LLM patterns: medium-sensitive
async = 0.38     # concurrency: more conservative (FP risk is high)
csa = 0.40       # code structure: conservative
```

### W8: Canonical Form Evaluation

`frensense-engine/src/pattern/` has a PatternCompiler, PatternMatcher, PatternScorer, and
CanonicalForm module that is built but not integrated into the detection pipeline. Before
building Phase 3+ corpus, this must be evaluated:

1. Run the canonical form matcher on the current 89 patterns
2. Compare TP/FP against the n-gram fingerprint matcher on the same corpus
3. If canonical form has better precision: integrate as the primary scorer
4. If n-gram is better: document why and remove the dead code
5. If they are complementary: blend scores

This evaluation should happen before Phase 3 corpus work begins because if canonical form
is superior, the corpus baking strategy changes (canonical forms are stored instead of
n-gram hashes).

### W9: Atomic Section Detection for C

`frensense-engine/src/atomic_section.rs` has `AtomicSectionAnalyzer` behind the `c_lang`
feature flag. When enabled, it detects lock/unlock mismatches in C code. This is particularly
relevant for detecting vulnerabilities in C libraries that TypeScript and Rust code calls
via FFI.

Enable: add `c_lang` feature to default features in `frensense-engine/Cargo.toml`.
Wire: add C file parsing in `src/engine/project/files.rs` and feed to atomic section analyzer.

### BB1–BB4: Private Corpus Repository

The source corpus files are currently in the public repo. This is a security problem — public
positive examples are a blueprint for attackers to study and evade detection. Three-stage
plan:

- **Stage 1 (now):** Bundle alongside source — FRC binary is in public repo, source stays too
- **Stage 2 (Phase 3 done):** New patterns only go in private repo → bundle → public binary
- **Stage 3 (Phase 6 done):** Remove `corpus/targets/` from public repo. Only `frensense-corpus.frc`
  ships. Users can still add custom corpus via `--corpus my-rules/`

---

## Section 5: Build Order for the Agent

Ordered by: unblocking other work first, then impact, then cost.

```
Week 1: Precision (unblocks meaningful benchmarking)
  T-FIX-1  src/semantics/taint_entry_points.rs (new file, ~200 lines)
            src/semantics/data_flow/resolve.rs  (replace regex seed with entry point check)
  T-FIX-3  src/semantics/data_flow/resolve.rs  (read sanitizers from taint rules, stop propagation)
  B4       src/cli/options.rs                   (--check-deps opt-in for Rust)
  VERIFY   python3 scripts/compute_metrics.py --by-rule
           Target: TAINT_INPUT_TO_HTTP FP rate < 20%

Week 2: Corpus Phase 2 completion (SP4–SP10, 70 pairs)
  SP4      corpus/targets/ts_sec_open_redirect_{1-10}_{pos,neg}.ts
           corpus/targets/rust_sec_open_redirect_{1-10}_{pos,neg}.rs
  SP5      corpus/targets/ts_sec_ssrf_{1-10}_{pos,neg}.ts
           corpus/targets/rust_sec_ssrf_{1-10}_{pos,neg}.rs
  SP6      corpus/targets/ts_sec_proto_pollution_{1-10}_{pos,neg}.ts
  SP7      corpus/targets/ts_sec_hardcoded_secret_{1-10}_{pos,neg}.ts
           corpus/targets/rust_sec_hardcoded_secret_{1-10}_{pos,neg}.rs
  SP8      corpus/targets/ts_sec_credential_flow_{1-10}_{pos,neg}.ts
           corpus/targets/rust_sec_credential_flow_{1-10}_{pos,neg}.rs
  SP9      corpus/targets/ts_sec_xss_reflected_{1-10}_{pos,neg}.ts
  SP10     corpus/targets/ts_sec_timing_attack_{1-10}_{pos,neg}.ts
           corpus/targets/rust_sec_timing_attack_{1-10}_{pos,neg}.rs
  REBUILD  cargo run --bin build-corpus-bundle -- --corpus corpus/targets --output frensense-corpus.frc
  VERIFY   cargo test -p frensense-engine -- corpus

Week 3: Canonical Form Evaluation (W8) + Corpus Phase 3 (HP1–HP4, 60 pairs)
  W8       Benchmark: canonical form vs n-gram on same 159 patterns
           Decision: integrate / discard / blend
  HP1-HP4  60 pairs of hollow implementation patterns
  REBUILD  FRC bundle
  VERIFY   Run on axum + express. Classify findings.

Week 4: ML Improvements (M2, M5)
  M2       frensense-engine/src/fingerprint.rs   (AST edit distance, blend at 40%)
  M5       frensense-engine/src/corpus/scorer.rs (Platt scaling coefficients per category)
  VERIFY   Precision on labeled dataset. Compare before/after.

Week 5: Corpus Phase 4 (LP1–LP4, 70 pairs) — LLM patterns
  SOURCE   Generate real AI code for each category using Claude/Copilot
  LP1-LP4  70 pairs
  REBUILD  FRC bundle
  VERIFY   Run on Blueprint Pro codebase as the primary LLM-generated test target

Week 6: Features
  F1       src/patcher/mod.rs — promote to stable --fix flag
  F3       src/cli/commands.rs — frensense acknowledge command
  F5       src/engine/project/config.rs — per-category thresholds from config.toml
  W9       frensense-engine/src/atomic_section.rs — enable c_lang, wire to findings

Week 7: Corpus Phase 5 + Phase 6 (110 pairs)
  AP1-AP3  50 architecture + resource management pairs
  CP1-CP4  60 concurrency pairs (loom validation on Rust negatives)
  REBUILD  FRC bundle — target: ~400 patterns
  VERIFY   Full VP2 benchmark: axum, actix-web, hyper, express, fastify

Week 8: Private Corpus Repository Migration
  BB1      Create private corpus repo
  BB2      Stage 2 transition: new patterns in private repo only
  D2       Write corpus contribution guide (tasks.md D2 spec)
  D4       Create corpus/TRACKING.md
  B10      Publish benchmark results: TP/FP on 5 real projects
```

---

## Section 6: Metrics Targets

| Metric | Current | Week 1 Target | Week 8 Target |
|---|---|---|---|
| Corpus patterns | 89 | 89 | 400 |
| TAINT FP rate | 100% | <20% | <8% |
| Corpus FP rate on clean code | Unknown | Measured | <5% |
| Precision (all rules combined) | 0% | >40% | >75% |
| Languages fully supported | Rust, TS, JS | Rust, TS, JS | + Python (loader fix) |
| Benchmark projects with published results | 0 | 0 | 5 |

The only metric that matters for external credibility is the last one. A tool that claims
75% precision without publishing benchmark results is indistinguishable from a tool with 0%
precision and a confident README. Publishing the axum/actix/express results — including the
false positives — is what makes Frensense trustworthy.
