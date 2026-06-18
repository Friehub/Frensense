# Frensense: Competitive Position and Corpus Strategy (2026)

> Written from full code read of `frensense-engine/src/fingerprint.rs`,
> `frensense-engine/src/corpus/`, and `corpus/targets/`.
> This is a working document for the agent building corpus coverage.

---

## 1. Where Frensense Sits vs Semgrep vs CodeQL

### Semgrep

**How it works:** Pattern matching against an AST using a YAML-defined pattern language. A
human writes `pattern: os.exec($CMD)` and Semgrep finds all call sites that match that shape.
No dataflow — the pattern is local to one expression or function. Community rules cover ~3,000
patterns.

**Strengths:**
- Extremely fast (single-pass AST match)
- Low friction to add a new rule
- Community rules cover the OWASP Top 10 in most languages
- CI integration is trivial

**Weaknesses:**
- No interprocedural taint. If user input enters function `A`, propagates through `B`, and
  reaches a sink in `C`, Semgrep does not catch it unless you write a taint rule that tracks
  the propagation explicitly.
- Rules are hand-written. A new zero-day or LLM-hallucinated pattern requires a human to
  recognize it and write a rule before Semgrep can catch it.
- High false-positive rate on renamed or wrapped sinks.

### CodeQL

**How it works:** Builds a full relational database of the program (AST, call graph, type
hierarchy, data flow). Queries are written in QL — a declarative logic language. True
interprocedural taint tracking: `isSource(node)` and `isSink(node)` with a dataflow solver
connecting them across files and function calls.

**Strengths:**
- The most precise open-source static analysis available
- GitHub runs it on millions of repos
- Interprocedural by design — taint flows across function boundaries correctly
- CVE queries exist for hundreds of known vulnerability classes

**Weaknesses:**
- Slow. A large codebase takes minutes to hours for a database build.
- QL has a steep learning curve. Writing a new query correctly requires deep knowledge of
  the query language and the CodeQL library for each language.
- Heavy toolchain — not suitable as a lightweight CLI or pre-commit hook.
- Java/C# coverage is excellent. Rust support is experimental and incomplete.
- Does not learn from examples. Every new pattern requires a new QL query.

### Frensense

**How it works:** Three independent detection layers run in parallel:

1. **Corpus layer** — `FunctionFingerprint` similarity scoring. Extracts positional n-gram
   hashes, structural markers, parameter type n-grams, and type usage vectors from each
   function. Matches against a compiled FRC1 bundle. No human-written rule needed — the
   corpus teaches the engine what a vulnerable function looks like by example.

2. **Taint layer** — `DataFlowAnalyzer` with alias tracking and cross-file call graph.
   Walks from taint sources (currently: regex-seeded identifiers — this is the 0% precision
   bug documented in `AUDIT.md`) through propagation edges to sinks defined in
   `taint_rules.toml`.

3. **Temporal layer** — `TemporalAnalyzer` checking that paired operations appear in the
   right order (lock/unlock, open/close, connect/disconnect). Currently hardcoded Rust
   structs — TOML loader is the next build item.

**Where Frensense wins over Semgrep:**
- The corpus layer requires no hand-written rules. Give it 10 vulnerable functions and 10
  clean functions for a new bug class and it learns the pattern. Semgrep needs a human to
  recognize the pattern and encode it as a YAML rule first.
- The `ts_llm_*` and `rust_llm_*` corpus entries catch LLM-hallucinated patterns that no
  existing Semgrep rule covers because those patterns didn't exist before LLMs became
  mainstream. This is a genuine coverage gap in every other tool.

**Where Frensense wins over CodeQL:**
- Frensense is a single Rust binary. No database build. Sub-second scan on a typical project.
- The corpus approach generalizes to Rust natively. CodeQL's Rust support is beta and lacks
  the standard library dataflow models.
- Adding a new pattern costs one positive example file + one negative example file. Adding a
  CodeQL query for the same costs days of QL development.

**Where Frensense currently loses:**
- The taint engine has 0% precision due to regex-seeded sources (AUDIT.md Phase 2). CodeQL
  and Semgrep do not have this bug — their taint is correct for the rules they have.
- No cross-file taint for Python or Go. Only Rust and TypeScript have cross-file call graph
  resolution currently.
- No interprocedural type refinement — Frensense knows a variable flows from an HTTP handler
  to a DB call, but does not narrow the type along the propagation path the way CodeQL does.

---

## 2. What LLM-Generated Code Breaks — and Why Frensense Is Positioned to Catch It

LLMs are trained on the entire public internet including CVE writeups, vulnerable code
examples, StackOverflow answers with security mistakes, and pre-2023 code that predates
modern security practices. They reproduce known patterns with high fidelity — including the
vulnerable ones.

The existing `ts_llm_*` and `rust_llm_*` corpus entries already cover:
- `ts_llm_any_parameter` — LLMs default to `any` type to avoid type errors, eliminating
  TypeScript's safety guarantees
- `ts_llm_console_log` — LLMs log request bodies, auth tokens, and PII to stdout in debug
  handlers
- `ts_llm_promise_catch` — LLMs write `.catch(e => {})` swallowing errors silently
- `ts_llm_mutate_after_response` — LLMs write `res.json(data); res.status(200)` causing
  "cannot set headers after they are sent" crashes in Node.js
- `rust_llm_await_in_sync` — LLMs call `.await` in synchronous Rust contexts
- `rust_llm_clone_literal` — LLMs clone string literals instead of using references
- `rust_llm_never_err` — LLMs write `unwrap()` in production paths instead of propagating
  errors

**The gap:** No tool other than Frensense has a `ts_llm_*` category at all. Semgrep has
rules for the underlying bug class (e.g., empty catch blocks) but not specifically tuned to
the patterns that LLMs produce vs. human-written code. This is the clearest differentiation.

**What to add next** (documented in Section 4):
- LLMs consistently produce SQL string concatenation instead of parameterized queries even
  when they know better — particularly in ORM fallback paths
- LLMs write race conditions in async code: read-then-write without locking
- LLMs produce JWT verification that accepts `alg: none` in custom middleware
- LLMs write CORS `*` + credentials: true combinations that browsers reject but the code
  contains as a security gap
- LLMs generate hard-coded example credentials that survive into production

---

## 3. Corpus Sources Available in 2026

In 2020, building a security corpus required manual extraction from CVE writeups. In 2026,
multiple machine-readable datasets exist that map directly to Frensense's
`_positive` / `_negative` pair format.

### Tier 1 — Direct Source (Already Structured as Before/After Patches)

**CVEfixes Dataset**
- URL: `https://github.com/secureIT-project/CVEfixes`
- Contents: 8,991 CVE fixes across C, C++, Java, Python, JavaScript, PHP, Go, Rust
- Format: git patches — each CVE has a before commit (vulnerable) and after commit (fixed)
- Mapping to FRC: before = `_positive`, after = `_negative`
- Coverage: covers CWE-79, CWE-89, CWE-22, CWE-78, CWE-119, CWE-416 (use-after-free),
  CWE-362 (race condition), CWE-476 (null deref) and ~400 others
- Action: Write a script that clones the dataset, filters by language (`.rs`, `.ts`, `.js`),
  extracts function-level diffs, and writes pairs to `corpus/targets/`

**GitHub Advisory Database (GHSA)**
- URL: `https://github.com/advisories` + `https://api.github.com/advisories`
- Contents: Machine-readable GHSA records, most link to the fixing commit in the affected repo
- The fixing commit URL gives you the before/after patch
- Filter by ecosystem: `npm` for TypeScript/JavaScript, `crates.io` for Rust
- Action: Script that queries the GHSA API, extracts `references` with type `FIX`,
  fetches the patch from the linked commit, extracts changed functions

**OSV.dev**
- URL: `https://osv.dev/list` + `https://api.osv.dev/v1/query`
- Contents: 40,000+ vulnerabilities in machine-readable JSON, all linked to fix commits
- Covers npm, PyPI, crates.io, Go modules
- Action: Same pipeline as GHSA — query OSV, follow fix commit URLs, extract function pairs

### Tier 2 — Labeled Datasets (Requires Extraction Script)

**BigVul**
- URL: `https://github.com/ZeoVan/MSR_20_Code_vulnerability_commits_dataset`
- Contents: 3,754 C/C++ CVE-fixing commits with function-level labels
- Less useful for Frensense's target languages but the extraction pattern is reusable

**D2A (Devign-derived)**
- URL: Hugging Face `google/D2A`
- Contents: 1.3M function-level samples labeled vulnerable/not vulnerable for C/C++
- Language mismatch but the labeling methodology is worth studying

**Juliet Test Suite (NIST)**
- URL: `https://samate.nist.gov/SARD/test-suites/112`
- Contents: Synthetic CWE-labeled test cases — each CWE has multiple bad variants and good
  variants in C, C++, Java, C#
- Not real-world code but high signal-to-noise ratio — every function is intentionally
  vulnerable or intentionally correct
- The TypeScript/JavaScript Juliet equivalents are sparse — this is a gap to fill

**SecurityEval**
- URL: `https://github.com/s2e-lab/SecurityEval`
- Contents: 130 Python code samples from LLM generation, labeled with CWE
- Specifically evaluates LLM-generated code — directly maps to `ts_llm_*` / `rust_llm_*`
  categories in Frensense

### Tier 3 — Scrape and Label (Higher Effort, High Value)

**Semgrep Community Rules**
- URL: `https://github.com/semgrep/semgrep-rules`
- Contents: 3,000+ rules in YAML. Each rule has a `pattern` (the vulnerable form) and
  often a `fix` (the correct form). Many rules include test fixtures — `_bad.ts` and
  `_ok.ts` files.
- These test fixtures are directly usable as corpus pairs. The `_bad.ts` = positive,
  `_ok.ts` = negative.
- Action: Script that walks the semgrep-rules repo, finds language dirs (typescript, python,
  javascript, rust), extracts test fixtures, and renames them to Frensense naming convention
- This gives Frensense coverage of every CWE that Semgrep community rules cover — and the
  corpus-based detection is harder to bypass than the syntactic Semgrep pattern

**GitHub Code Search (CVE in commit message)**
- Query: `git log --all --grep="CVE-" --format="%H %s"` on popular repos in the target
  language
- For JavaScript: express, fastify, next.js, koa commit history
- For Rust: actix-web, axum, tokio commit history
- High manual curation effort but produces real-world patterns that synthetic datasets miss

---

## 4. Specific Corpus Files to Add Now

Organized by priority. Each item is one positive file + one negative file.

### Priority 1 — LLM-Generated Patterns (Frensense's Unique Differentiation)

These patterns are LLM-specific. No Semgrep rule covers them as a category.

```
ts_llm_sql_concat_positive.ts       -- LLM writes: db.query(`SELECT * FROM users WHERE id = ${userId}`)
ts_llm_sql_concat_negative.ts       -- Fixed: db.query('SELECT * FROM users WHERE id = $1', [userId])

ts_llm_jwt_alg_none_positive.ts     -- LLM verifies JWT without checking alg field
ts_llm_jwt_alg_none_negative.ts     -- Fixed: explicitly reject alg: none

ts_llm_cors_credentials_positive.ts -- LLM writes: Access-Control-Allow-Origin: * + credentials: true
ts_llm_cors_credentials_negative.ts -- Fixed: specific origin when credentials are included

ts_llm_race_read_write_positive.ts  -- LLM writes: read balance, await external call, write balance (TOCTOU)
ts_llm_race_read_write_negative.ts  -- Fixed: atomic compare-and-swap or pessimistic lock

ts_llm_hardcoded_secret_positive.ts -- LLM leaves: const SECRET = "example-secret-change-me"
ts_llm_hardcoded_secret_negative.ts -- Fixed: process.env.SECRET with validation

rust_llm_unwrap_in_handler_positive.rs  -- LLM writes: let val = op().unwrap(); in an axum handler
rust_llm_unwrap_in_handler_negative.rs  -- Fixed: let val = op().map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;

rust_llm_panic_on_parse_positive.rs -- LLM writes: let id: Uuid = body.id.parse().unwrap();
rust_llm_panic_on_parse_negative.rs -- Fixed: proper error propagation
```

### Priority 2 — CWE Coverage Gaps (Missing From Current Corpus)

The current corpus has CWE-78 (cmd injection), CWE-89 (SQL injection), CWE-22 (path
traversal), CWE-918 (SSRF). Missing:

```
ts_xss_reflected_positive.ts / negative.ts          -- CWE-79: unescaped user input in HTML response
ts_insecure_deserialization_positive.ts / negative   -- CWE-502: JSON.parse on untrusted data + eval
ts_regex_dos_positive.ts / negative.ts               -- CWE-1333: catastrophic backtracking regex
ts_xxe_positive.ts / negative.ts                     -- CWE-611: XML external entity in xml2js
ts_timing_attack_positive.ts / negative.ts           -- CWE-208: string comparison for secrets
ts_session_fixation_positive.ts / negative.ts        -- CWE-384: session ID not rotated after login
rust_use_after_free_positive.rs / negative.rs        -- CWE-416: unsafe block UAF (real CVEs exist)
rust_integer_overflow_positive.rs / negative.rs      -- CWE-190: arithmetic without overflow check
```

### Priority 3 — Framework-Specific Patterns (High Signal for LLM Code)

LLMs write framework-specific code constantly. These patterns are language-specific and
framework-specific — Semgrep covers some but not with the structural learning Frensense uses.

```
ts_axum_unauth_route_positive.rs         -- Axum route with no auth middleware
ts_axum_unauth_route_negative.rs         -- Same route with RequireAuth layer

ts_nextjs_server_action_injection_positive.ts
ts_nextjs_server_action_injection_negative.ts

ts_prisma_raw_injection_positive.ts      -- Prisma.$queryRaw with template literal interpolation
ts_prisma_raw_injection_negative.ts      -- Prisma.$queryRaw with Prisma.sql tagged template
```

---

## 5. Corpus Acquisition Pipeline

The pipeline to turn CVEfixes / GHSA / OSV into corpus pairs:

```python
# corpus_harvester.py — pseudocode, agent should implement this

import subprocess, json, requests
from pathlib import Path

CORPUS_DIR = Path("corpus/targets")
LANGUAGES = {"rs": "rust", "ts": "typescript", "js": "javascript"}

def fetch_osv_for_ecosystem(ecosystem: str) -> list[dict]:
    """Query OSV.dev for all vulnerabilities in a package ecosystem."""
    r = requests.post("https://api.osv.dev/v1/query", json={"package": {"ecosystem": ecosystem}})
    return r.json().get("vulns", [])

def extract_fix_commits(vuln: dict) -> list[str]:
    """Extract git commit URLs from OSV references with type FIX."""
    return [r["url"] for r in vuln.get("references", []) if r.get("type") == "FIX"]

def fetch_patch(commit_url: str) -> str | None:
    """Fetch the diff for a commit."""
    # GitHub: convert blob URL to API URL
    if "github.com" in commit_url and "/commit/" in commit_url:
        parts = commit_url.split("github.com/")[1].split("/commit/")
        repo, sha = parts[0], parts[1]
        r = requests.get(f"https://api.github.com/repos/{repo}/commits/{sha}",
                         headers={"Accept": "application/vnd.github.diff"})
        return r.text if r.ok else None
    return None

def extract_function_pairs(diff: str, ext: str) -> list[tuple[str, str]]:
    """
    Given a unified diff, extract (before, after) function-level pairs.
    Uses hunk headers to identify changed functions and extracts the full
    function context around each hunk.
    """
    # Implementation: parse @@ -N,M +N,M @@ function_name headers
    # Extract -lines (before) and +lines (after)
    # Wrap in minimal compilable file
    ...

def write_corpus_pair(pattern_name: str, positive: str, negative: str, ext: str):
    CORPUS_DIR.mkdir(parents=True, exist_ok=True)
    (CORPUS_DIR / f"{pattern_name}_positive.{ext}").write_text(positive)
    (CORPUS_DIR / f"{pattern_name}_negative.{ext}").write_text(negative)
```

**Key constraint:** Frensense's corpus loader requires that each file contain at least one
function body that tree-sitter can parse. The harvester must wrap extracted code snippets in
a compilable function if they are not already in function form.

**Naming convention:**
```
{lang}_{cwe_or_category}_{source}_{n}_{positive|negative}.{ext}

rs_cwe22_cvefixes_1_positive.rs
ts_cwe89_ghsa_1_positive.ts
ts_llm_jwt_alg_none_positive.ts
```

---

## 6. The FRC Bundle Build Process

After adding corpus files, rebuild the bundle:

```bash
cargo run --bin build-corpus-bundle -- \
  --corpus corpus/targets \
  --output frensense-corpus.frc

# Verify:
cargo test -p frensense-engine -- corpus
```

The bundle is embedded into the binary via `include_bytes!("../../frensense-corpus.frc")` in
`runner.rs`. After rebuild, the new patterns are active in every scan without shipping any
external files.

---

## 7. What Frensense Catches That Semgrep and CodeQL Miss

| Pattern | Semgrep | CodeQL | Frensense |
|---|---|---|---|
| LLM-specific hallucination patterns (`ts_llm_*`) | No | No | Yes (corpus) |
| Vulnerable code that is structurally similar but syntactically different (renamed vars, different APIs) | No — pattern must match exactly | Yes (dataflow) | Yes (fingerprint similarity) |
| Temporal violations (lock without unlock, connect without disconnect) | No | Partial (Java only) | Yes (TemporalAnalyzer) |
| Axum/FastAPI typed-parameter taint seeding | No built-in | Partial (Python) | Planned (TaintEntryPoint — AUDIT.md Phase 2) |
| Interprocedural taint across files | Partial (requires explicit taint rules) | Yes | Yes (DataFlowAnalyzer, when source seeding is fixed) |
| Zero-config — no rules to write for new patterns | No | No | Yes (corpus from examples) |

| Pattern | Semgrep | CodeQL | Frensense |
|---|---|---|---|
| Known OWASP Top 10 patterns (YAML rules) | 3,000+ rules | 500+ queries | 0 YAML needed — covered by corpus |
| Cross-language consistency | Some | Excellent | Rust + TS + JS today; Python blocked by loader |
| CI speed on large codebase | Fast (seconds) | Slow (minutes) | Fast (sub-second per file) |
| Precise interprocedural taint (today) | Weak | Strong | Broken (0% precision — AUDIT.md C1) |

The taint precision fix (AUDIT.md Phase 2: replace regex seeding with `TaintEntryPoint`) is
the single highest-impact item. Once that ships, Frensense's taint layer becomes comparable
to Semgrep's taint rules — and the corpus layer is already ahead of anything Semgrep does.

---

## 8. Build Order for the Agent

```
Step 1: Corpus harvester script
  File: scripts/harvest_corpus.py
  Inputs: OSV.dev API + GitHub Advisory API
  Output: corpus/targets/{lang}_{cwe}_{source}_{n}_{pos|neg}.{ext}
  Priority: ts and rs files only (py is blocked by loader — AUDIT.md Issue 3.3)

Step 2: Add Priority 1 LLM corpus files (manually authored)
  Files: ts_llm_sql_concat, ts_llm_jwt_alg_none, ts_llm_cors_credentials,
         ts_llm_race_read_write, rust_llm_unwrap_in_handler, rust_llm_panic_on_parse
  Rationale: these are Frensense's clearest differentiation from Semgrep/CodeQL

Step 3: Add Priority 2 CWE gap files
  Files: ts_xss_reflected, ts_regex_dos, ts_timing_attack, rust_use_after_free,
         rust_integer_overflow
  Source: Juliet Test Suite + CVEfixes filtered to .ts/.rs

Step 4: Rebuild FRC bundle
  Command: cargo run --bin build-corpus-bundle -- --corpus corpus/targets --output frensense-corpus.frc

Step 5: Fix Python corpus loader (AUDIT.md Issue 3.3)
  File: frensense-engine/src/corpus/loader.rs — add Python arm to match ext
  Unblocks: py corpus files for all Priority 1–2 patterns

Step 6: Fix taint source seeding (AUDIT.md Phase 2 — TaintEntryPoint)
  Files: src/engine/taint_seeder.rs (new), src/engine/taint_entry_points.rs (new)
  This is the prerequisite for Frensense's taint precision matching Semgrep

Step 7: Run metrics
  Command: python3 scripts/compute_metrics.py --by-rule
  Target: TAINT_INPUT_TO_HTTP FP rate below 10% (from current 100%)
```
