# Frensense Corpus Milestones: 103 to 45,000

> This document answers two questions at every step: **how do you reach that number**,
> and **can Frensense ship to users at that number**.
>
> Current state: ~103 pairs (207 files in `corpus/targets/`).
> Target: 45,000 pairs.

---

## The Honest Baseline

The 103 pairs we have right now are entirely hand-authored. That is both a strength
and a ceiling. Hand-authored pairs are high quality — every positive is a genuine
vulnerable pattern, every negative is a correct fix. The ceiling is that one engineer
cannot hand-author 45,000 pairs. The path from 103 to 45,000 requires moving from
hand authoring to automated extraction with spot-checking.

The other constraint: the corpus alone is not the only blocker on shipping. The taint
engine currently has 0% precision on real codebases (all findings are false positives
due to regex-based source seeding — AUDIT.md Phase 5). A corpus of 45,000 patterns on
top of a broken taint layer is still a broken tool. The milestones below track both
dimensions.

---

## Milestone 0: Current — 103 Pairs (Can Ship: Limited)

**What works:**
- Corpus fingerprint layer fires on known-vulnerable patterns from `corpus/targets/`
- Temporal violation detection (lock/unlock, acquire/release)
- Dead branch, unused variable, hallucinated import detection
- Secret scanning

**What does not work:**
- Taint analysis: 100% false positive rate on any real Rust/Axum codebase
- CLI filters (`--category`, `--tag`, `--severity`) silently do nothing
- Corpus covers only CWE-78, CWE-89, CWE-22 with meaningful depth

**Shippable as:** A development/research tool for Friehub internal use. Not shippable
to external users who will run it on their Axum codebase and immediately see it is broken.

**Blocker to resolve before any public release:** T-FIX-1 in ROADMAP.md — replace
regex source seeding with AST-based `TaintEntryPoint` detection. This is the gate.

---

## Milestone 1: 1,000 Pairs

### How to get there

| Source | Expected Yield | Effort |
|---|---|---|
| CVEfixes SQLite (Rust, TypeScript) | ~350 pairs | Run `extract_cvefixes_targeted.py` — ~2 hrs including download |
| OSV.dev harvester (`crates.io` + `npm`) | ~150 pairs | Run `harvest_corpus.py --source osv` with `GITHUB_TOKEN` set |
| Semgrep rule fixtures | ~300 pairs | New harvester: walk `semgrep-rules/` repo, rename `_bad`→`_positive`, `_ok`→`_negative` |
| Hand-authored (LLM patterns, CWE gaps) | ~100 pairs | 2–3 days of writing |

**Total: ~900–1,000 pairs**

The Semgrep fixture harvester does not exist yet. It is the highest-yield single action
available: the semgrep-rules repository has 3,000+ rules, most with `tests/` directories
containing `_bad.ts` / `_ok.ts` or `_bad.js` / `_ok.js` files. These map directly to
Frensense's `_positive` / `_negative` convention. Walking the repo and renaming the
files is a one-evening script.

```bash
# Rough shape of the semgrep fixture harvester
git clone --depth 1 https://github.com/semgrep/semgrep-rules /tmp/semgrep-rules
python3 scripts/harvesters/semgrep.py \
  --repo /tmp/semgrep-rules \
  --languages typescript javascript rust \
  --output corpus/targets \
  --limit 400
```

### FRC bundle impact

At 1,000 pairs: bundle size ~900 KB (from current 289 KB). Still embedded in binary
with `include_bytes!`. No architecture change required.

### Engine changes required

- Deduplication run (`scripts/deduplicate_corpus.py`) is mandatory before FRC rebuild —
  the Semgrep fixtures have near-duplicates for the same CWE across different rule files
- LSH band configuration auto-scales at >1000 patterns (already implemented in
  `frensense-engine/src/corpus/registry.rs`) — no manual change needed

### Can Frensense ship at 1,000 pairs?

**Yes, with one precondition: T-FIX-1 (taint precision) must ship first.**

At 1,000 pairs, the corpus layer covers:
- CWE-78, CWE-79, CWE-89, CWE-22, CWE-918, CWE-601, CWE-287 with 10–30 examples each
- LLM-specific patterns (expanded from current 7 to ~15)
- Structural patterns (hollow validator, prototype pollution, type escape)

This is a meaningful detection suite. A user who runs Frensense on a TypeScript API
will get genuine signal on real patterns. The corpus FP rate on clean code should be
under 10% at this scale if the deduplication threshold is correct.

**What to call this release:** `v0.5.0 — Early Access`. Honest about limitations.
Publish the axum FP benchmark result — fixed after T-FIX-1 ships.

**Time estimate from now:** 2–3 weeks (T-FIX-1 + dataset download + Semgrep harvester
+ dedup + FRC rebuild + testing).

---

## Milestone 2: 5,000 Pairs

### How to get there (from 1,000)

| Source | Expected Yield | Effort |
|---|---|---|
| GHSA API (npm + crates.io advisories) | ~1,500 pairs | New `harvesters/ghsa.py` — similar to OSV harvester |
| Remaining CVEfixes languages (JS, Python) | ~500 pairs | Already possible with `extract_cvefixes_targeted.py --language javascript` |
| OSV second pass (broader package list) | ~600 pairs | Expand PACKAGES list in `osv.py` to 30+ packages per ecosystem |
| Juliet Test Suite (NIST) — C/C++ adapted | ~400 pairs | Port synthetic CWE cases to TypeScript (low effort, high volume) |
| LLM-generated corpus (Claude/Copilot output) | ~500 pairs | Generate real AI code for each LLM pattern category |
| Community contributions (if public) | ~500 pairs | Open corpus contribution guide |

**Total: ~4,000–5,000 additional pairs → ~5,000–6,000 cumulative**

### The GHSA harvester

The GitHub Security Advisory database has structured JSON for every npm and crates.io
advisory including the `references` array with a `FIX` type entry pointing to the
fixing commit. This is the same pipeline as OSV but higher quality — GHSA entries are
reviewed by GitHub's security team.

```bash
# GHSA API endpoint
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/advisories?ecosystem=npm&per_page=100" \
  | jq '.[].references[] | select(.type == "FIX") | .url'
```

The fixing commit URL follows the same GitHub diff extraction path already implemented
in `osv.py`.

### FRC bundle impact

At 5,000 pairs: bundle ~4.5 MB embedded. Still comfortably within `include_bytes!`
limits. Binary size increases by ~4 MB — acceptable.

### Engine changes required for this milestone

- **M2: AST edit distance** (ROADMAP.md Section 3) becomes important at 5,000 pairs.
  With more patterns, the fingerprint space is denser and n-gram-only similarity starts
  producing more false positives from structurally different functions sharing tokens.
  AST edit distance blended at 40% weight separates them.

- **Incremental FRC rebuild** (SCALING_PLAN.md Week 3) becomes important — full rebuild
  of 5,000 patterns takes ~2–3 minutes. The manifest-based tracker in
  `scripts/update_bundle_manifest.py` is already written but the bundle builder still
  does a full rebuild. Wire the incremental path before this milestone.

### Can Frensense ship at 5,000 pairs?

**Yes. This is the first version worth marketing.**

At 5,000 pairs, coverage reaches:
- All OWASP Top 10 in both Rust and TypeScript with multiple examples per CWE
- LLM-specific patterns covering the 10 most common AI-generated anti-patterns
- Structural and concurrency patterns
- CWE classification on every finding

The corpus FP rate should be under 5% with AST edit distance blending. The taint layer
(T-FIX-1 already shipped) adds genuine interprocedural analysis.

**What to call this release:** `v1.0.0`. First stable release. Publish benchmark
results on axum, actix-web, express, fastify.

**Time estimate from 1k milestone:** 6–8 weeks.

---

## Milestone 3: 10,000 Pairs

### How to get there (from 5,000)

| Source | Expected Yield | Effort |
|---|---|---|
| Full OSV.dev bulk export (all ecosystems) | ~3,000 pairs | OSV provides bulk JSON download — no API rate limit |
| Semgrep rules — second pass (all languages) | ~1,000 pairs | Python and Go rules from semgrep-rules |
| NVD historical CVE corpus (2018–2024) | ~1,500 pairs | Script that queries NVD API for CVEs with GitHub fix links |
| Generated variants of existing patterns | ~500 pairs | M6 one-class classifier identifies anomalies → manual review → new patterns |

**Total: ~5,500–6,000 additional pairs → ~10,000–11,000 cumulative**

### Architecture change: LSH tuning

At 10,000 patterns, the LSH index auto-scales from 16 to 32 bands (already implemented).
Run the deduplication script first — at 10,000 raw patterns, expect ~20–30% to be
near-duplicates (Jaccard > 0.85) from the same CWE appearing in multiple data sources.
After dedup: ~7,000–8,000 unique clusters.

### OSV bulk export

OSV provides a full data export at `https://osv.dev/dl/all.zip` (renewed daily).
This bypasses the per-package API query and gives access to all 40,000+ advisories
at once. The yield for npm + crates.io is significantly higher than per-package queries.

```bash
wget https://osv.dev/dl/all.zip -O /tmp/osv_all.zip
# ~2 GB compressed
unzip -p /tmp/osv_all.zip "npm/*.json" > /tmp/osv_npm.jsonl
unzip -p /tmp/osv_all.zip "crates.io/*.json" > /tmp/osv_crates.jsonl
python3 scripts/harvesters/osv_bulk.py \
  --input /tmp/osv_npm.jsonl /tmp/osv_crates.jsonl \
  --output corpus/targets \
  --limit 3000
```

### Can Frensense ship at 10,000 pairs?

**Yes. This is the version that competes with Semgrep on coverage.**

Semgrep community rules: ~3,000 rules. Frensense at 10,000 pairs covers more CWE
classes with more examples per class. The key differentiator: Frensense patterns are
example-based (structural similarity to real vulnerable code), not syntactic
(exact token match). A Semgrep rule that checks for `os.exec($CMD)` misses
`subprocess.run([cmd], shell=True)`. Frensense's structural approach catches
semantic equivalents.

**What to call this release:** `v1.1.0`. Publish "Frensense vs Semgrep: Pattern
Coverage Comparison" benchmark.

**Time estimate from 5k milestone:** 8–10 weeks.

---

## Milestone 4: 15,000 Pairs

### How to get there (from 10,000)

| Source | Expected Yield | Effort |
|---|---|---|
| Python corpus (loader fix required first) | ~2,000 pairs | Fix Python tree-sitter loader, then harvest from CVEfixes Python data |
| Go corpus (new parser) | ~1,500 pairs | Add tree-sitter-go, harvest from OSV Go ecosystem |
| LLM-generated bulk patterns | ~1,500 pairs | Prompt engineering pipeline: generate 50 variants per CWE per LLM |

**Total: ~5,000 additional pairs → ~15,000 cumulative**

### Python corpus loader

AUDIT.md Issue 3.3 (status: partially addressed with warning) — Python `.py` files are
silently skipped by the corpus loader. Enabling Python requires:

1. Add `tree-sitter-python` to `frensense-engine/Cargo.toml`
2. Add `"py"` arm to the match in `frensense-engine/src/corpus/loader.rs`
3. Add Python handler entry points to `taint_entry_points.toml`

Once the loader works, CVEfixes has ~2,000 Python method changes covering Django, Flask,
aiohttp, and starlette — directly relevant for teams running Python backends.

### LLM pattern generation pipeline

At this scale, manual authoring of 1,500 pairs is not feasible. The approach is
programmatic generation:

```
For each CWE in [CWE-78, CWE-89, CWE-22, CWE-79, CWE-918, ...]:
  For each framework in [Express, Fastify, Axum, Actix, Django, Flask]:
    Prompt: "Write a TypeScript/Rust/Python function for a {framework} handler
             that {description of CWE scenario}. Make it realistic — name variables
             as a developer would, include comments."
    positive = LLM output (the vulnerable version)
    negative = LLM output with fix prompt: "Now fix the {CWE} vulnerability in
               the above function while keeping the same business logic."
```

This generates semantically valid patterns grounded in real framework idioms. The
positive examples are what an LLM actually produces when asked to implement something —
which is exactly the threat model Frensense is designed for.

### Can Frensense ship at 15,000 pairs?

**Yes. This is the version that competes with CodeQL on Rust.**

CodeQL's Rust support is experimental. At 15,000 pairs covering Rust, TypeScript,
JavaScript, and Python, Frensense has broader language coverage than any CodeQL setup
that supports Rust natively. The corpus approach also makes adding a new language faster
than CodeQL's approach (which requires writing a QL library for the language).

**What to call this release:** `v1.2.0 — Python and Go support`. Expand the benchmark
to include Python FastAPI and Django codebases.

**Time estimate from 10k milestone:** 10–12 weeks.

---

## Milestone 5: 45,000 Pairs (Full Target)

### How to get there (from 15,000)

The remaining 30,000 pairs come from three sources:

**Source A: NVD full corpus extraction (~10,000 pairs)**

The National Vulnerability Database has structured data for every CVE published since
1999. Most CVEs after 2015 include GitHub references. A script that:
1. Queries NVD API for all CVEs with GitHub references (CVSS ≥ 7.0)
2. Identifies references with type `"Patch"` in the reference data
3. Follows the commit URL and extracts function diffs

...can systematically extract fixing commits for the entire historical CVE record.
This yields C/C++ dominated output, but TypeScript/Rust CVEs are growing year over year.

**Source B: Community corpus repository (~10,000 pairs)**

By the time Frensense is at 15,000 pairs with published benchmarks, it is credible
enough to attract community contributions. The private corpus repository (ROADMAP.md
BB1–BB4) allows:
- External security researchers to submit new patterns via pull request
- Automated validation (tree-sitter parse check, dedup check) in CI
- Human review before merge

10,000 community-contributed pairs over 6–12 months is realistic if the contribution
workflow is frictionless. The Semgrep community rules repo has 3,000+ rules with active
maintainers — comparable scale is achievable.

**Source C: Automated synthesis with dedup (~10,000 pairs)**

At 15,000 base patterns, generate structural variants automatically:
- Take each existing pattern and generate 2–3 variants by:
  - Renaming functions and variables (different surface form, same structure)
  - Changing the framework (Express version of an Axum pattern)
  - Changing the severity (direct sink vs. one-hop through helper function)
- Filter with MinHash dedup at Jaccard > 0.7 to eliminate near-duplicates
- The dedup step cuts roughly 30% of generated variants

This is not the same as the LLM generation approach — this is structural mutation of
existing validated patterns, not prompt-based generation. The resulting pairs are
grounded in validated examples.

### Architecture requirements at 45,000

At 45,000 pairs:

| Requirement | Status | Action |
|---|---|---|
| LSH 32 bands × 4 rows | Auto-scales at >1000 | No action |
| FRC bundle ~40 MB embedded | Feasible with compression | Test binary size — if >50 MB, move to memory-mapped file |
| Incremental bundle rebuild | Partial | Wire manifest tracker to fingerprint cache in `build_bundle()` |
| Parallel file scanning | Not implemented | Add `rayon` parallelization — 4× speedup on multi-core |
| Private corpus repository | Not implemented | Required before Stage 3 (remove `corpus/targets/` from public repo) |
| Pattern deduplication to ~15k clusters | Script exists | Run with Jaccard 0.85 threshold; merge advisory text |

### Scan performance at 45,000

Per SCALING_PLAN.md projections:
- 25 files: ~800ms (from current 535ms)
- 100 files: ~7s
- 1,000 files: ~70s

This is still faster than a CodeQL database build on any codebase. The pre-commit
use case (scan only changed files) remains sub-second at any corpus size.

### Can Frensense ship at 45,000 pairs?

**Yes. This is the version that requires no qualification.**

At 45,000 pairs with ~15,000 unique pattern clusters after deduplication, Frensense
covers more vulnerability classes in more languages than any comparable tool. The
corpus is a compiled knowledge base of every known vulnerability pattern in
Rust, TypeScript, JavaScript, and Python — embedded in a single sub-second binary.

The 45k number itself is not the marketing claim. The marketing claim is:
- **Zero configuration** — no YAML rules to write, no QL to learn
- **Sub-second** — faster than any database-build-based tool
- **Example-based** — detects structural variants of known patterns, not just
  exact syntactic matches
- **Auditable** — every finding cites a specific pattern with a CVE/CWE reference

**What to call this release:** `v2.0.0`. Publish a full whitepaper with precision/
recall numbers on 20 real production codebases.

---

## Shippability Summary

| Milestone | Pairs | Taint Precision | Corpus Coverage | Ship Status |
|---|---|---|---|---|
| Current | ~103 | 0% (broken) | 3 CWE classes | Internal only |
| Pre-ship gate | ~103 | T-FIX-1 required | 3 CWE classes | Gate, not a milestone |
| M1: 1k | ~1,000 | Fixed | OWASP Top 5 | Ship as v0.5.0 Early Access |
| M2: 5k | ~5,000 | Fixed + M2 blending | OWASP Top 10 + LLM | Ship as v1.0.0 |
| M3: 10k | ~10,000 | Calibrated (M5) | Semgrep parity | Ship as v1.1.0 |
| M4: 15k | ~15,000 | Production-grade | + Python, Go | Ship as v1.2.0 |
| M5: 45k | ~45,000 | Production-grade | Full coverage | Ship as v2.0.0 |

---

## The One Precondition That Applies to All of Them

Corpus size does not fix a broken taint layer. A tool with 45,000 corpus patterns
and 100% taint false positive rate is still a tool that cries wolf on every file.
Users will disable taint findings immediately and the tool becomes corpus-only.

**T-FIX-1 ships before any public release.** Everything else is negotiable on
timeline. That is not.

Estimated effort for T-FIX-1: ~450 lines across 10 files (AUDIT.md Phase 5 build
order). With the AUDIT.md specifications already written, this is a 2–3 day
implementation task, not a design task.

---

## Realistic Timeline to Each Milestone

| Milestone | Calendar Duration (from today) | Cumulative |
|---|---|---|
| T-FIX-1 gate | 2–3 days | Week 1 |
| M1: 1,000 pairs | 2–3 weeks | Week 3 |
| M2: 5,000 pairs | 6–8 weeks | Week 10 |
| M3: 10,000 pairs | 8–10 weeks | Week 20 |
| M4: 15,000 pairs | 10–12 weeks | Week 32 |
| M5: 45,000 pairs | 20–24 weeks from M4 | Week 56 |

The M4→M5 gap is large because 30,000 of the remaining pairs require either
community contributions (which take time to accumulate) or the NVD bulk extraction
pipeline (which requires engineering the GHSA + NVD integration on top of what
exists).

Reaching 15,000 pairs with T-FIX-1 shipped is a more credible 6-month goal than
45,000. At 15,000, Frensense is a production tool. The path to 45,000 runs in
parallel with shipping and growing users — community contributions accelerate it.
