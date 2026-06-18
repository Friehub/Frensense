# Frensense Corpus Audit: Current State and Fix Coverage

> Audit date: 2026-06-18
> Scope: `corpus/targets/`, `corpus/ground_truth/`, `frensense-corpus.frc`,
> and the full harvester pipeline under `scripts/`.
> Cross-references: `AUDIT.md` (engine issues), `CORPUS_STRATEGY.md` (growth plan).

---

## 1. Current Corpus Inventory

### 1.1 File Count and Coverage

```
corpus/targets/     207 files  (103 positive + 104 negative = ~103 pairs)
corpus/ground_truth/ 2 files   (axum_labels.json, latest_scan.json)
```

| Language | Pairs | Categories Covered |
|---|---|---|
| Rust | ~54 | cmd_injection, path_traversal, sql_injection, cve_derive, cve_stateless, llm_await_in_sync, llm_clone_literal, llm_never_err, transmute, async_blocking_io, clone_in_loop, connection_leak, csa_validate_unconditional, mutate_after_response, network_in_txn, panic_in_lib |
| TypeScript | ~49 | cmd_injection, path_traversal, sql_injection, as_any_escape, command_injection, cookie_security, csa_auth_no_rejection, csa_find_never_empty, csa_sanitize_passthrough, csa_validate_unconditional, god_function, hardcoded_secret, jwt_bypass, llm_any_parameter, llm_console_log, llm_mutate_after_response, llm_promise_catch, open_redirect, prototype_pollution, ssrf, unawaited_assertion, cve_allowedorigins, cve_body, cve_bodychunks, cve_cachedir, cve_host, cve_html3, cve_matchedsegment, cve_maxpostponedstatesize, cve_rel, cve_urlparts |

### 1.2 Ground Truth Files

| File | Size | Status |
|---|---|---|
| `corpus/ground_truth/axum_labels.json` | 777 KB | Active — used by `validate_recall.py` |
| `corpus/ground_truth/latest_scan.json` | 674 KB | Active — most recent scan output |

---

## 2. Corpus Quality Issues

### Issue C1: Harvester Schema Mismatch

**File:** `scripts/harvesters/cvefixes.py`

The existing CVEfixes harvester expects the dataset in JSON commit format:
```python
commits_dir = dataset / "git_commits"
for commit_file in sorted(commits_dir.glob("*.json")):
```

The actual CVEfixes 1.0.8 dataset distributed via Zenodo is a **SQLite database**
(`CVEfixes.db`), not a directory of JSON files. The `git_commits/` directory does not
exist in the Zenodo distribution. Running the harvester against the Zenodo download
will yield zero results silently.

**Fix:** Use `scripts/extract_cvefixes_targeted.py` (new file) which queries the
SQLite database directly via the `method_change` / `file_change` / `cve` schema.
See `docs/CVEFIXES_INTEGRATION.md` for the full acquisition workflow.

---

### Issue C2: `_extract_functions_from_diff` Calls an Undefined Name

**File:** `scripts/harvesters/cvefixes.py` — line 152

```python
cwe = commit_data_cwe(filepath)  # noqa: F821 — placeholder
```

`commit_data_cwe` is never defined in the module. This is a `NameError` at runtime
that will crash the harvester on the first pair it attempts to process. The `# noqa: F821`
suppresses the linter warning, hiding the bug.

**Fix:** Replace with a safe fallback:
```python
cwe = "cve"  # CWE extracted from DB in the SQL-based extractor
```

The SQLite extractor (`extract_cvefixes_targeted.py`) resolves this properly by joining
against `cwe_classification` in the query.

---

### Issue C3: `typescript_cve_query_positive.ts` Has No Corresponding Negative

**File:** `corpus/targets/typescript_cve_query_positive.ts`

This file has a `_positive` variant but no `_negative` counterpart. The corpus loader
in `frensense-engine/src/corpus/loader.rs` requires paired files. The pattern will be
loaded with zero negative examples, which means the fingerprint matcher has no "clean"
signal to separate it from — the pattern will match nothing or produce only false
positives.

**Fix:** Author a `typescript_cve_query_negative.ts` showing the fixed version of the
same query construction. Check the CVE the positive is based on and write the mitigation.

---

### Issue C4: Rust CVE Corpus Entries Are Structurally Shallow

**Files:** `corpus/targets/rust_cve_fixed_function_positive.rs` (24 bytes),
`corpus/targets/rust_cve_fixed_function_negative.rs` (93 bytes)

The `rust_cve_fixed_function` pair consists of trivially short stubs that do not contain
a parseable function body with sufficient structure for fingerprint extraction. A 24-byte
positive file is almost certainly just a function signature with no body.

**Impact:** The fingerprint extractor in `frensense-engine/src/fingerprint.rs` computes
positional n-gram hashes and structural markers. A function with fewer than 3 statements
produces a fingerprint with near-zero discriminative power — it will match unrelated code
or match nothing.

**Fix:** Replace with a full-body example (15+ lines). Source from the actual CVE
commit referenced when the file was authored. See the naming convention in
`docs/CVEFIXES_INTEGRATION.md` Section 7 for canonical naming.

---

### Issue C5: Corpus Contains No Pairs for CWE-79 (XSS) in Either Language

**Gap:** `CORPUS_STRATEGY.md` Section 4 documents XSS as a Priority 2 item. No
`ts_xss_*` or `rust_xss_*` files exist in `corpus/targets/`.

XSS in TypeScript/Node.js (unescaped user input written to HTML response) is one of
the most frequent CVEs in the npm ecosystem. Its absence means Frensense cannot detect
this vulnerability class via the corpus layer.

**Fix:** Add minimum 3 positive/negative pairs for CWE-79:
- `ts_xss_reflected_positive.ts` / `ts_xss_reflected_negative.ts`
- `ts_xss_stored_positive.ts` / `ts_xss_stored_negative.ts`
- `ts_xss_dom_positive.ts` / `ts_xss_dom_negative.ts`

Source from CVEfixes rows where `cwe_id = 'CWE-79'` and `programming_language IN
('TypeScript', 'JavaScript')`.

---

### Issue C6: Corpus Contains No Pairs for CWE-416 (Use-After-Free) in Rust

**Gap:** Rust's memory model prevents most UAF in safe code, but `unsafe` blocks are
present in real Rust CVEs. The CVEfixes dataset contains UAF entries for Rust
(e.g., in `servo`, `hyper`, `crossbeam`). None are currently in the corpus.

**Fix:** Query CVEfixes with:
```sql
SELECT mc.name, mc.code, mc.before_change
FROM method_change mc
JOIN file_change f ON mc.file_change_id = f.file_change_id
JOIN commits c ON f.hash = c.hash
JOIN fixes fx ON c.hash = fx.hash
JOIN cve cv ON fx.cve_id = cv.cve_id
JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
WHERE f.programming_language = 'Rust'
  AND cc.cwe_id = 'CWE-416';
```

---

## 3. Harvester Pipeline Audit

### 3.1 `scripts/harvest_corpus.py`

| Check | Status |
|---|---|
| Runs in both `cvefixes` and `osv` modes | Correct |
| `--dataset-path` wired to `cvefixes.harvest_cvefixes` | Correct |
| `--limit` is partitioned between sources | Correct (OSV gets `limit - total_harvested`) |
| Works with Zenodo SQLite format | **No** — see Issue C1 |
| OSV harvester dependency (`requests`) check | Correct — graceful skip if missing |

### 3.2 `scripts/harvesters/osv.py`

| Check | Status |
|---|---|
| Queries OSV API for `crates.io` (Rust) | Correct |
| Queries OSV API for `npm` (TypeScript) | Correct |
| Fetches GitHub diffs via API | Correct |
| Rate limiting / auth header | Missing — no `GITHUB_TOKEN` support; will hit 60 req/hr unauthenticated |
| Deduplication across packages | Correct — `seen_fixes` set |
| Handles non-GitHub fix URLs | Correct — skips gracefully |

**Missing: GitHub token support in OSV harvester.**

Add to `_fetch_github_diff`:
```python
import os
token = os.environ.get("GITHUB_TOKEN")
headers = {"Accept": "application/vnd.github.diff"}
if token:
    headers["Authorization"] = f"Bearer {token}"
```

Without a token, the OSV harvester will be rate-limited and produce far fewer pairs
than the configured `--limit`. Set `GITHUB_TOKEN` in the environment before running.

### 3.3 `scripts/deduplicate_corpus.py`

Not inspected in this audit. Run with `--dry-run` first to confirm it handles the
CVEfixes naming convention (`{lang}_cvefixes_{cwe}_{cve}_{fn}`) correctly. If the
deduplicator uses prefix-based grouping it may not group CVEfixes pairs correctly.

---

## 4. FRC Bundle State

The compiled bundle at `frensense-corpus.frc` (289 KB) was built from the current
`corpus/targets/` state. After adding CVEfixes pairs:

```bash
# Rebuild is required after any corpus/ change
cargo run --bin build-corpus-bundle -- \
  --corpus corpus/targets \
  --output frensense-corpus.frc

# Verify no checksum errors and correct pattern count
cargo test -p frensense-engine -- corpus::tests
```

The bundle is embedded into the binary at compile time via `include_bytes!`. A rebuild
of the FRC bundle alone is **not sufficient** — you must also `cargo build` (or
`cargo run`) to pick up the new embedded bytes.

---

## 5. Fix Priority List

| Priority | Issue | File | Action |
|---|---|---|---|
| 1 | C1: Schema mismatch — harvester won't find data | `scripts/harvesters/cvefixes.py` | Use `extract_cvefixes_targeted.py` against SQLite |
| 2 | C2: `NameError` on `commit_data_cwe` | `scripts/harvesters/cvefixes.py:152` | Replace with `cwe = "cve"` |
| 3 | C3: Unpaired `typescript_cve_query_positive.ts` | `corpus/targets/` | Author the negative counterpart |
| 4 | C5: Zero XSS pairs | `corpus/targets/` | Add 3 CWE-79 pairs from CVEfixes |
| 5 | C4: Shallow Rust CVE stubs | `corpus/targets/rust_cve_fixed_function_*` | Replace with full-body examples |
| 6 | C6: Zero UAF pairs for Rust | `corpus/targets/` | Query CVEfixes for CWE-416 Rust rows |
| 7 | OSV: no GitHub token support | `scripts/harvesters/osv.py` | Add `GITHUB_TOKEN` env var header |
