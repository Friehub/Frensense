# Frensense: CVE Fix Coverage Map

> What CVE classes are currently covered by the corpus, what is missing, and how to
> fill the gaps from CVEfixes, OSV, and authored examples.

---

## Current Coverage

### Rust

| Pattern Prefix | CWE | Source | Pairs |
|---|---|---|---|
| `rust_sec_cmd_injection_*` | CWE-78 | Authored | 10 |
| `rust_sec_path_traversal_*` | CWE-22 | Authored | 10 |
| `rust_sec_sql_injection_*` | CWE-89 | Authored | 10 |
| `rust_cve_derive_*` | CVE-specific | Authored | 1 |
| `rust_cve_fixed_function_*` | CVE-specific | Authored | 1 (shallow) |
| `rust_cve_stateless_*` | CVE-specific | Authored | 1 |
| `rust_llm_await_in_sync` | LLM-specific | Authored | 1 |
| `rust_llm_clone_literal` | LLM-specific | Authored | 1 |
| `rust_llm_never_err` | LLM-specific | Authored | 1 |
| `rust_transmute_*` | CWE-843 | Authored | 1 |
| `rust_async_blocking_io_*` | Arch | Authored | 1 |
| `rust_clone_in_loop_*` | Perf | Authored | 1 |
| `rust_connection_leak_*` | CWE-772 | Authored | 1 |
| `rust_mutate_after_response_*` | Arch | Authored | 1 |
| `rust_network_in_txn_*` | Arch | Authored | 1 |
| `rust_panic_in_lib_*` | CWE-248 | Authored | 1 |

**Total Rust pairs: ~54**

### TypeScript

| Pattern Prefix | CWE | Source | Pairs |
|---|---|---|---|
| `ts_sec_cmd_injection_*` | CWE-78 | Authored | 10 |
| `ts_sec_path_traversal_*` | CWE-22 | Authored | 10 |
| `ts_sec_sql_injection_*` | CWE-89 | Authored | 10 |
| `typescript_cve_*` | CVE-specific | Authored | ~11 |
| `ts_llm_any_parameter` | LLM-specific | Authored | 1 |
| `ts_llm_console_log` | LLM-specific | Authored | 1 |
| `ts_llm_mutate_after_response` | LLM-specific | Authored | 1 |
| `ts_llm_promise_catch` | LLM-specific | Authored | 1 |
| `ts_as_any_escape` | CWE-843 | Authored | 1 |
| `ts_cookie_security` | CWE-614 | Authored | 1 |
| `ts_hardcoded_secret` | CWE-798 | Authored | 1 |
| `ts_jwt_bypass` | CWE-287 | Authored | 1 |
| `ts_open_redirect` | CWE-601 | Authored | 1 |
| `ts_prototype_pollution` | CWE-1321 | Authored | 1 |
| `ts_ssrf` | CWE-918 | Authored | 1 |
| `ts_sql_injection` | CWE-89 | Authored | 1 |
| `ts_path_traversal` | CWE-22 | Authored | 1 |

**Total TypeScript pairs: ~53**

---

## Coverage Gaps (Not Yet In Corpus)

### High Priority (from `CORPUS_STRATEGY.md` Priority 1 and 2)

| CWE | Description | Language | Notes |
|---|---|---|---|
| CWE-79 | XSS — reflected, stored, DOM | TS | High frequency in npm CVEs |
| CWE-416 | Use-after-free in `unsafe` blocks | Rust | Present in servo, hyper CVEs |
| CWE-190 | Integer overflow | Rust | Common in crypto/parsing crates |
| CWE-502 | Insecure deserialization | TS | `JSON.parse` + `eval` chains |
| CWE-1333 | ReDoS (catastrophic backtracking) | TS | `new RegExp(userInput)` |
| CWE-611 | XXE in xml2js | TS | Documented in CVEfixes |
| CWE-208 | Timing attack (string comparison) | TS | Secret comparison with `==` |
| CWE-384 | Session fixation | TS | Session ID not rotated post-login |

### LLM-Specific Gaps (Frensense Differentiators)

These are not in any existing dataset — they must be authored or scraped from LLM
output evaluations:

| Pattern | Description | Priority |
|---|---|---|
| `ts_llm_sql_concat` | LLM writes template literal SQL | High |
| `ts_llm_jwt_alg_none` | LLM skips `alg` field check | High |
| `ts_llm_cors_credentials` | LLM writes `*` + credentials | High |
| `ts_llm_race_read_write` | LLM writes non-atomic read-modify-write | High |
| `ts_llm_hardcoded_secret` | LLM leaves `example-secret-change-me` | Medium |
| `rust_llm_unwrap_in_handler` | LLM uses `.unwrap()` in axum handler | High |
| `rust_llm_panic_on_parse` | LLM uses `.parse().unwrap()` | Medium |

---

## CVEfixes Query Map

The following queries extract pairs for each gap from the SQLite database. Run after
following `docs/CVEFIXES_INTEGRATION.md` to obtain `CVEfixes.db`.

### CWE-79 (XSS) — TypeScript

```sql
SELECT mc.name, mc.code, mc.before_change, cv.cve_id
FROM method_change mc
JOIN file_change f ON mc.file_change_id = f.file_change_id
JOIN commits c ON f.hash = c.hash
JOIN fixes fx ON c.hash = fx.hash
JOIN cve cv ON fx.cve_id = cv.cve_id
JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
WHERE f.programming_language IN ('TypeScript', 'JavaScript')
  AND cc.cwe_id = 'CWE-79'
  AND mc.code IS NOT NULL
  AND length(mc.code) > 50
ORDER BY cv.cve_id, mc.before_change DESC;
```

### CWE-416 (Use-After-Free) — Rust

```sql
SELECT mc.name, mc.code, mc.before_change, cv.cve_id
FROM method_change mc
JOIN file_change f ON mc.file_change_id = f.file_change_id
JOIN commits c ON f.hash = c.hash
JOIN fixes fx ON c.hash = fx.hash
JOIN cve cv ON fx.cve_id = cv.cve_id
JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
WHERE f.programming_language = 'Rust'
  AND cc.cwe_id = 'CWE-416'
  AND mc.code IS NOT NULL
ORDER BY cv.cve_id, mc.before_change DESC;
```

### CWE-190 (Integer Overflow) — Rust

```sql
SELECT mc.name, mc.code, mc.before_change, cv.cve_id
FROM method_change mc
JOIN file_change f ON mc.file_change_id = f.file_change_id
JOIN commits c ON f.hash = c.hash
JOIN fixes fx ON c.hash = fx.hash
JOIN cve cv ON fx.cve_id = cv.cve_id
JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
WHERE f.programming_language = 'Rust'
  AND cc.cwe_id = 'CWE-190'
  AND mc.code IS NOT NULL
ORDER BY cv.cve_id, mc.before_change DESC;
```

### CWE-1333 (ReDoS) — TypeScript

```sql
SELECT mc.name, mc.code, mc.before_change, cv.cve_id
FROM method_change mc
JOIN file_change f ON mc.file_change_id = f.file_change_id
JOIN commits c ON f.hash = c.hash
JOIN fixes fx ON c.hash = fx.hash
JOIN cve cv ON fx.cve_id = cv.cve_id
JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
WHERE f.programming_language IN ('TypeScript', 'JavaScript')
  AND cc.cwe_id = 'CWE-1333'
  AND mc.code IS NOT NULL
ORDER BY cv.cve_id, mc.before_change DESC;
```

---

## Target State After CVEfixes Integration

| Metric | Current | Target (post-integration) |
|---|---|---|
| Total pairs | ~103 | ~600–700 |
| Rust pairs | ~54 | ~180–220 |
| TypeScript pairs | ~49 | ~400–480 |
| CWE classes covered (Rust) | 8 | 14+ |
| CWE classes covered (TS) | 11 | 18+ |
| LLM-specific patterns | 7 | 14 |

---

## Bundle Rebuild Checklist

After adding pairs from CVEfixes:

```bash
# 1. Check file count
ls corpus/targets/ | wc -l

# 2. Deduplicate
python3 scripts/deduplicate_corpus.py --corpus corpus/targets --dry-run
python3 scripts/deduplicate_corpus.py --corpus corpus/targets

# 3. Rebuild bundle
cargo run --bin build-corpus-bundle -- \
  --corpus corpus/targets \
  --output frensense-corpus.frc

# 4. Full test suite
cargo test -p frensense-engine

# 5. Recall validation
python3 scripts/validate_recall.py \
  --corpus corpus/targets \
  --ground-truth corpus/ground_truth/axum_labels.json

# 6. Rebuild binary to embed new FRC
cargo build --release
```
