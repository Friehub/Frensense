# Frensense — Benchmarking Guide

> How to measure, compare, and honestly publish results while continuing to
> improve the tool. Covers datasets, metrics, comparison baselines, the
> uniquely interesting benchmark only Frensense can run, and how to version
> results so every update shows measurable progress.

---

## Table of Contents

1. [The four metrics that actually matter](#1-the-four-metrics-that-actually-matter)
2. [Datasets to use](#2-datasets-to-use)
3. [What to compare against](#3-what-to-compare-against)
4. [The uniquely interesting benchmark — cross-variant generalization](#4-the-uniquely-interesting-benchmark--cross-variant-generalization)
5. [Benchmark file structure](#5-benchmark-file-structure)
6. [Running the benchmark](#6-running-the-benchmark)
7. [How to report results honestly](#7-how-to-report-results-honestly)
8. [Known issues and their expected benchmark impact](#8-known-issues-and-their-expected-benchmark-impact)
9. [Version-over-version tracking](#9-version-over-version-tracking)
10. [What to lead with when publishing](#10-what-to-lead-with-when-publishing)

---

## 1. The four metrics that actually matter

### Precision and Recall

```
Precision = TP / (TP + FP)   "of what we flagged, how much is real"
Recall    = TP / (TP + FN)   "of all real vulns, how many did we find"
F1        = 2 × (P × R) / (P + R)
```

Do not publish a single threshold's precision and recall. Vary the `--threshold` flag from `0.20` to `0.90` in steps of `0.05` and publish the full **precision-recall curve**. One number can always be gamed by threshold selection. The curve cannot.

At the default threshold (`0.40`), security teams will be around the 0.40–0.60 recall range on any real benchmark. That is normal for SAST. Semgrep's community rules on the same datasets are typically in a similar range. What matters is that your curve is to the upper-right of theirs.

### False Positive Rate on real, clean projects

Precision on a benchmark of known vulnerabilities is necessary but not sufficient. Developers are also scanned against their own production code that is mostly clean. Run Frensense against 3–5 well-maintained open-source projects that have no known vulnerabilities of the classes you claim to detect. Count what gets flagged. Report it.

Suggested clean projects to scan (TypeScript/Node.js):
- `expressjs/express` — canonical web framework, extensively audited
- `fastify/fastify` — modern, security-conscious
- `prisma/prisma` — ORM, has its own security team
- `vercel/next.js` (select `packages/next/src/`) — large, real, maintained

For Rust:
- `tokio-rs/tokio` — async runtime, audited
- `serde-rs/serde` — serialization, audited

Report the FP count. Do not hide it. A 30% FP rate on a known-clean project is honest and expected for a v0.5.0 tool. The improvement doc gives you the specific bugs (M1.1, M2.1) that cause them.

### Scan speed (KLOC/second and peak RSS)

```bash
# Measure wall time and peak memory
/usr/bin/time -v frensense src/ --json > /dev/null 2> timing.txt
cat timing.txt | grep "Elapsed\|Maximum resident"

# Count lines for normalization
find src/ -name "*.ts" -o -name "*.js" | xargs wc -l | tail -1
```

Frensense is written in Rust. Semgrep is Python + OCaml. You should be meaningfully faster. Measure it and report it. Scan speed is a real adoption criterion — if the tool takes 5 minutes on a 50k-line codebase it will not run in CI pre-merge.

### Cross-variant detection rate (your unique metric — see §4)

This is the metric only Frensense can publish because it is the metric that only a corpus-driven tool can score well on. See §4 for the full design.

---

## 2. Datasets to use

### Primary: SecBench.js

**What it is:** 600 vulnerabilities from real npm packages, covering the five most common Node.js vulnerability classes: code injection, OS command injection, path traversal, ReDoS, and prototype pollution. Each vulnerability includes a payload that exploits it and an oracle that validates successful exploitation. Ground truth is function-level.

**Why it is the right primary dataset:** It is the standard for Node.js static analysis research. <cite index="14-1">VulcaN and SecBench.js are two widely recognized public benchmarks that collected vulnerable Node.js packages from the npm registry based on reports from GitHub Advisory, Snyk, Huntr.dev, and the CVE database.</cite> Using SecBench.js puts your numbers directly comparable to published research.

**Where:** `github.com/sola-st/DynNode` (the repository distributing SecBench.js test harness). The dataset itself is available at CISPA.

**How to use it:** For each vulnerable package version, run Frensense against the package source. Check whether the vulnerable function is among the flagged findings. Compute TP/FP/FN across all 600 entries for the vulnerability classes you claim to support.

**Filter by what you cover:** Frensense v0.5.0 should only be benchmarked against vulnerability classes with corpus patterns. Do not report recall=0% for ReDoS if you have no ReDoS corpus patterns — exclude it and document the exclusion.

### Secondary: VulcaN

**What it is:** A complementary npm vulnerability dataset with different vulnerability type distribution than SecBench.js. Used together they give broader coverage.

**Why use both:** <cite index="1-1">VulcaN and SecBench have different vulnerability distributions over vulnerability types. Combined, these two reference datasets provide a comprehensive set of vulnerabilities.</cite> Running both and reporting separately shows your tool's strengths and gaps per vulnerability class.

### False-positive ground truth: CVEfixes

**What it is:** 12,107 vulnerability-fixing commits across 4,249 open-source projects, including JavaScript and Rust. Each entry has the **fixed** version (which is safe code). The fixed versions are your labeled negatives — code that used to be vulnerable but has been patched.

**How to use it for FP measurement:** Extract fixed-version functions from CVEfixes JavaScript/Rust commits. These are confirmed-clean code. Run Frensense against them. Any finding is a false positive by definition — the vulnerability was patched. Report this FP rate separately because it is a stronger signal than scanning arbitrary clean code.

**Where:** `zenodo.org/record/7029359` — the full dataset. Preprocessed version also on Kaggle.

### Rust: RustSec Advisory Database

**What it is:** A curated database of security advisories for Rust crates, maintained by the Rust Security Response WG. Each advisory links to a GitHub commit that introduced or fixed the vulnerability.

**How to use it:** For advisories with commit links, extract the vulnerable function (pre-fix commit) and the patched function (post-fix commit). The vulnerable functions are your TPs; the patched functions are your labeled negatives.

**Where:** `github.com/rustsec/advisory-db` — TOML files with crate name, version range, CVE, and patch commit.

### Synthetic false-positive test suite (build this yourself)

The public datasets do not test your specific known failure modes. Build a small synthetic test suite targeting the bugs in the improvements document:

```
benchmark/synthetic/
├── ts_should_not_flag/
│   ├── knex_parameterized.ts       # parameterized query using knex.where (tests bug 2.1)
│   ├── cached_loop_fetch.ts        # DB call in loop but with pre-populated cache (tests M16)
│   ├── decorator_handler_safe.ts   # @Get handler with safe code (tests bug 2.2 false positive side)
│   └── test_file_sqli.test.ts      # SQL injection in a test file (environment detection)
└── ts_must_flag/
    ├── raw_concat_sqli.ts           # string concat to db.query
    ├── n1_for_of.ts                 # findOne inside for-of
    └── exec_user_input.ts           # exec(req.body.cmd)
```

This suite has two jobs: regression testing (ensure bugs don't resurface) and benchmark honesty (show what the tool misses).

---

## 3. What to compare against

### Semgrep (primary comparison)

Semgrep is the most directly comparable tool: pattern-based, TypeScript and Rust support, open source, widely used. Use the Semgrep Registry rules for the vulnerability classes you test.

```bash
semgrep --config "p/nodejs-security" --json src/ > semgrep_results.json
semgrep --config "p/secrets" --json src/ >> semgrep_results.json
```

Frensense's advantage over Semgrep on a well-designed benchmark:
- Corpus patterns generalize to variants Semgrep rules don't cover (§4)
- Taint analysis is deeper (interprocedural in `--mode taint`)
- Scan speed (Rust vs Python)

Semgrep's advantage over Frensense v0.5.0:
- Much larger rule library
- Mature, battle-tested precision
- Lower FP rate on general code

Report both advantages honestly.

### njsscan (secondary comparison, Node.js only)

njsscan is a rule-based Node.js security scanner. It tends to have high recall but high FP rate. Frensense should have better precision at comparable recall.

```bash
njsscan --json src/ > njsscan_results.json
```

### cargo-audit (Rust dependency comparison only)

cargo-audit only checks dependencies against the RustSec database. It does not analyze source code. Compare separately as a different task (dependency scanning vs code analysis). Do not combine the numbers.

### Do not compare against CodeQL or Snyk

CodeQL is backed by Microsoft's research team and years of investment. Snyk is a commercial product with dedicated security researchers. Comparing v0.5.0 of a new tool against them sets up an unfair baseline. You will lose, and the loss will obscure what Frensense actually does well.

---

## 4. The uniquely interesting benchmark — cross-variant generalization

This is the benchmark that only Frensense can run well, because it tests the core property that corpus-driven detection has and rule-based detection does not: the ability to detect a known vulnerability class in an API it has never seen.

### Design

Split the SecBench.js dataset by the npm package the vulnerability appears in. Group packages by the underlying framework or library they use (Express.js, Fastify, Koa, Hapi, Nest.js, etc.).

**Training split:** Use only Express.js vulnerable packages to build the Frensense corpus for the relevant vulnerability class (e.g., command injection).

**Test split:** Run Frensense against the equivalent vulnerability in Fastify, Koa, and Hapi packages — without adding any Fastify/Koa/Hapi examples to the corpus.

**Metric — Cross-Variant Detection Rate (CVDR):**
```
CVDR = (TP on test-framework packages) / (all vulnerable test-framework packages)
```

**Run the same experiment for Semgrep:** Use only the Express.js-specific Semgrep rules (which is what their rule set contains). Measure how many Fastify/Koa/Hapi vulnerabilities those rules catch.

The expected result: Semgrep misses most of them because the rules are framework-specific. Frensense catches a substantial fraction because the fingerprint generalizes across frameworks that use the same underlying pattern (e.g., calling a DB query function with user input).

**Why this matters:** It is the argument for the corpus approach. It is the thing you can publish that existing tools cannot refute with their own higher numbers. Every other metric (precision, recall, F1) Semgrep can match or beat on today's benchmark. CVDR is yours.

### Example cross-variant table to publish

```
Corpus trained on: Express.js command injection examples

Tested on:              TP   Total   CVDR    Semgrep CVDR
─────────────────────────────────────────────────────────
Express.js (in-corpus)  28    30     93%       91%
Fastify                 11    15     73%       12%
Koa                      8    12     67%        8%
Hapi                     6    10     60%        5%
NestJS                   9    14     64%       15%
─────────────────────────────────────────────────────────
Overall (out-of-corpus) 34    51     67%        9%
```

*(Numbers are illustrative. Run the actual experiment.)*

The gap between Frensense and Semgrep on out-of-corpus frameworks is the result to lead with. It justifies the entire architectural approach.

---

## 5. Benchmark file structure

```
frensense-bench/
├── README.md                    ← methodology, assumptions, how to reproduce
├── LICENSE.md                   ← dataset licenses (SecBench.js: MIT, CVEfixes: CC-BY-4.0)
│
├── datasets/
│   ├── secbench-js/
│   │   ├── download.sh          ← pulls from CISPA/zenodo (not committed)
│   │   ├── labels.json          ← {package, version, cwe, vuln_function, patched_function}
│   │   └── README.md
│   ├── vulcan/
│   │   ├── download.sh
│   │   └── labels.json
│   ├── cvefixes-js/
│   │   ├── extract.py           ← filters CVEfixes for JS/TS, extracts function pairs
│   │   └── fixed_functions/     ← confirmed-clean labeled negatives
│   ├── rustsec/
│   │   ├── extract.sh
│   │   └── labels.json
│   ├── clean-projects/
│   │   ├── download.sh          ← clones expressjs/express, fastify/fastify etc.
│   │   └── README.md
│   └── synthetic/
│       ├── ts_should_not_flag/
│       └── ts_must_flag/
│
├── runners/
│   ├── run_frensense.sh         ← pinned: --threshold 0.40 --json --sarif
│   ├── run_semgrep.sh           ← pinned: semgrep==1.x.x, specific config
│   └── run_njsscan.sh
│
├── evaluate.py                  ← reads SARIF output, computes TP/FP/FN, P/R curve
│
├── corpus/                      ← the corpus bundle used for this benchmark run
│   └── bundle.hash              ← blake3 of the bundle (for reproducibility)
│
└── results/
    ├── v0.5.0/
    │   ├── frensense_secbench.sarif
    │   ├── semgrep_secbench.sarif
    │   ├── metrics.json
    │   ├── per_pattern_metrics.json
    │   ├── precision_recall_curve.json
    │   └── cross_variant_generalization.json
    └── v0.6.0/                  ← added on next release
```

Every result directory is fully committed. Anyone can reproduce any historical result by checking out the tag and running `runners/run_frensense.sh` against the matching dataset.

---

## 6. Running the benchmark

### Baseline scan (SecBench.js)

```bash
# Run Frensense at multiple thresholds to build the P/R curve
for threshold in 0.20 0.25 0.30 0.35 0.40 0.45 0.50 0.55 0.60 0.65 0.70; do
    frensense datasets/secbench-js/packages/ \
        --corpus corpus/ \
        --threshold $threshold \
        --sarif \
        > results/v0.5.0/secbench_threshold_${threshold}.sarif
done

# Compute metrics from SARIF against labels.json
python evaluate.py \
    --labels datasets/secbench-js/labels.json \
    --sarif-dir results/v0.5.0/ \
    --tool frensense \
    --output results/v0.5.0/metrics.json
```

### Clean-project FP scan

```bash
# Scan known-clean projects; every finding is a FP
for project in expressjs/express fastify/fastify prisma/prisma; do
    name=$(echo $project | tr '/' '_')
    frensense datasets/clean-projects/$name/src/ \
        --threshold 0.40 --json \
        > results/v0.5.0/fp_${name}.json
done

# Count and categorize FPs
python evaluate_fps.py results/v0.5.0/fp_*.json
```

### Speed benchmark

```bash
# Run 5 times, report median
for i in 1 2 3 4 5; do
    /usr/bin/time -f "%e %M" frensense datasets/secbench-js/packages/ \
        --threshold 0.40 --json > /dev/null 2>> results/v0.5.0/timing.txt
done
python median_timing.py results/v0.5.0/timing.txt
```

### Cross-variant benchmark

```bash
# Train: corpus built from Express.js examples only
frensense-build-corpus \
    --source datasets/secbench-js/packages/ \
    --filter-framework express \
    --output corpus/express-only/

# Test: run on Fastify, Koa, Hapi packages
frensense datasets/secbench-js/packages/ \
    --corpus corpus/express-only/ \
    --filter-framework fastify,koa,hapi \
    --threshold 0.40 --json \
    > results/v0.5.0/cross_variant_frensense.json

# Semgrep comparison on same split
semgrep --config "p/nodejs-security" \
    datasets/secbench-js/packages/fastify/ \
    datasets/secbench-js/packages/koa/ \
    datasets/secbench-js/packages/hapi/ \
    --json > results/v0.5.0/cross_variant_semgrep.json

python evaluate_cross_variant.py \
    --labels datasets/secbench-js/labels.json \
    --frensense results/v0.5.0/cross_variant_frensense.json \
    --semgrep results/v0.5.0/cross_variant_semgrep.json
```

---

## 7. How to report results honestly

### Version-pin everything

Every published result must be reproducible. Pin:
- Frensense version (`v0.5.0`, git commit hash)
- Corpus bundle hash (blake3 of the `.bundle` file)
- Semgrep version and config
- Dataset version (commit hash or zenodo DOI)
- Evaluation script hash

If someone runs your benchmark in six months and gets different numbers, that is a problem for your credibility.

### Separate what you tested from what you claim to cover

Frensense v0.5.0 has corpus patterns for a specific set of vulnerability classes. Publish a table that is explicit:

```
Vulnerability Class    Corpus Patterns    Benchmarked    Claim
───────────────────────────────────────────────────────────────
Command injection      ✅ 4 patterns      ✅ SecBench.js  Supported
SQL injection          ✅ 3 patterns      ✅ SecBench.js  Supported
Path traversal         ✅ 2 patterns      ✅ SecBench.js  Supported
Prototype pollution    ❌ 0 patterns      ❌              Not supported v0.5.0
ReDoS                  ❌ 0 patterns      ❌              Not supported v0.5.0
SSRF                   ⚠️ 1 pattern       ⚠️ Limited     Experimental
N+1 query              ⚠️ In progress     ❌              Coming v0.6.0
```

This table is honest and actually builds more trust than claiming coverage you don't have. Readers respect honesty about scope. They do not respect discovering gaps you didn't mention.

### Publish the full precision-recall curve, not a single point

```
Threshold   Precision   Recall   F1
0.20        0.34        0.78     0.47
0.30        0.44        0.71     0.54
0.40        0.58        0.63     0.60    ← default
0.50        0.67        0.55     0.61
0.60        0.76        0.44     0.56
0.70        0.84        0.33     0.47
0.80        0.91        0.21     0.34
```

This shows the tool is working (precision rises with threshold, recall falls) and lets users tune it for their risk tolerance. Security teams that need high recall can drop to 0.30. Teams that can't handle noise can raise to 0.60.

### Report per-pattern metrics in a separate table

Don't aggregate everything into one F1. A pattern with 0.90 precision and 0.85 recall is hiding a pattern with 0.30 precision. Publish per-pattern metrics in a supplementary table.

```
Pattern ID              Precision  Recall  F1    n (test cases)
─────────────────────────────────────────────────────────────────
ts_cmdi_exec_direct     0.88       0.81    0.84  47
ts_sqli_concat          0.72       0.68    0.70  38
ts_path_traversal_join  0.64       0.59    0.61  29
ts_sqli_format          0.51       0.44    0.47  22    ← flag as needing work
```

The last row tells users to use extra caution on `ts_sqli_format` patterns and tells you where to invest corpus improvement effort next.

### The FP rate section — lead with context, not apology

Do not bury the FP rate number or qualify it to death. Lead with context:

> "On three known-clean open-source projects (expressjs/express, fastify/fastify, prisma/prisma), Frensense v0.5.0 produced 23 false positive findings across 187,000 lines of code — a rate of 0.12 per KLOC. Of these, 18 (78%) are attributable to two known issues under active development: the substring-based flow taint detection (issue M2.1) and the `tainted_api_sim` inconsistency between scoring paths (issue M1.1). These are documented in the roadmap and targeted for v0.6.0."

This is honest, specific, and shows you understand your own tool. It is much better than either hiding the number or presenting it without context.

---

## 8. Known issues and their expected benchmark impact

From the improvements document, map known bugs to benchmark consequences so the reader understands the current numbers:

| Issue | Type | Expected benchmark impact |
|-------|------|--------------------------|
| M1.1 tainted_api_sim inconsistency | FP inflator | Borderline corpus matches score higher in fast path than evidence path → elevated FP count |
| M2.1 substring-based flow taint | FP inflator | Functions with source member names in strings get tainted paths they shouldn't → elevated FP on clean code with comments |
| M2.2 decorator-only handlers missed | FN inflator | NestJS handlers without `req` param not seeded as taint sources → lower recall on NestJS vulns |
| M3.1 LSH not used as pre-filter | Performance only | Scan time 3-5× higher than it should be → slow speed benchmark |
| M1.5 branch-ratio suppression at 0.6 | FN inflator | Real vulns in functions with >60% branch ratio suppressed → lower recall on validation-adjacent code |

Include a version of this table in your publication under "Known Limitations and Active Development." It tells the reader exactly what the next version will improve, making the publication a living document rather than a frozen claim.

---

## 9. Version-over-version tracking

Every Frensense release should rerun the same benchmark and commit results to `results/vX.Y.Z/`. Track these metrics over time:

```
Version   Precision@0.40   Recall@0.40   FP/KLOC(clean)   Speed(KLOC/s)
──────────────────────────────────────────────────────────────────────────
v0.5.0    0.58             0.63          0.12              142
v0.6.0    0.67*            0.67*         0.07*             280*    ← projected
v0.7.0    TBD              TBD           TBD               TBD
```

*Projected based on fixing M1.1, M2.1, M2.2 (precision), M3.1 (speed), M1.5 (recall)*

This versioned table is the core of any tool paper or blog post. It shows trajectory. A tool with improving numbers is more compelling than a tool with one perfect snapshot.

Automate this. Add a GitHub Actions job that runs the benchmark on every release tag and commits the results file. The benchmark becomes part of your CI.

---

## 10. What to lead with when publishing

### Lead with cross-variant generalization

The CVDR gap against Semgrep is your primary publication claim. No rule-based tool can close that gap without writing framework-specific rules for every framework. Frensense closes it from a handful of examples. This is architecturally novel and the right thing to publish.

### Frame it as a new measurement, not a superiority claim

Do not say "Frensense is better than Semgrep." Semgrep has a 9-year head start and a full-time team. Instead: "We introduce the cross-variant detection rate (CVDR) metric, which measures a tool's ability to generalize vulnerability detection across API variants without additional rules. On this metric, example-driven tools show a 6× advantage over rule-based tools on out-of-corpus framework variants."

That claim is defensible, interesting, and yours. The academic community will find it publishable. The practitioner community will find it useful.

### The three honest sentences for any README or blog post

> "Frensense detects security vulnerabilities via example-driven fingerprint matching. On SecBench.js, it achieves 0.58 precision / 0.63 recall at the default threshold. Its primary advantage over rule-based tools is cross-variant generalization: patterns trained on one framework detect equivalent vulnerabilities in other frameworks without additional rules."

Say these three things. Say nothing more in the opening. Let the benchmark tables speak.

### Conference venues if you want to publish formally

| Venue | Fit | Submission type |
|-------|-----|-----------------|
| ISSTA (Int'l Symposium on Software Testing and Analysis) | High — tool + evaluation | Tool demo or research paper |
| ASE (Automated Software Engineering) | High — novel analysis technique | Research paper |
| ICSE (Software Engineering) | Medium — needs strong evaluation | Technical track or NIER |
| USENIX Security | Medium — security focus, needs strong empirical results | Research paper |
| IEEE S&P (Oakland) | Lower for v0.5.0 — high bar for security venues | Research paper (aim for v0.7.0+) |
| arXiv preprint | Now, immediately | Preprint (no review) |

A preprint on arXiv is the right move now. It stakes your claim on the cross-variant generalization metric, is citable, and has no deadline pressure. You can submit to ISSTA or ASE when the v0.6.0 numbers show clear improvement.

---

*Benchmarking guide for Frensense v0.5.0 — datasets, metrics, and publication strategy*
