# FrenSense Benchmark Report

> Generated: 2026-07-10
> Baseline: HEAD = 62bed13 (v0.5.0 era, branch v0.4.0-tasks)

## Corpus Stats

| Metric | Value |
|--------|-------|
| Corpus files (`corpus/targets/`) | 93 files (82 .rs/.ts, 9 .toml) |
| Unique pattern names | 51 |
| Patterns loaded (FRC bundle) | 42 |
| Bundle size | 499 KB |
| Pattern sources | Manual CSA rework, OSV fix commits, semantic AI pairs |

## E2E Precision & Recall: OWASP Juice Shop

This is the first external, formally mapped ground-truth benchmark for the v0.5.0 engine.
The benchmark scans the backend directories of [OWASP Juice Shop](https://github.com/juice-shop/juice-shop) (`routes`, `lib`, `models`, `server.ts`, `app.ts`) and maps findings to intentional vulnerability challenges via `challengeUtils.solveIf()`.

**Scan Details:**
- Files scanned: 215 JS/TS files (excluding `frontend` and `node_modules`)
- Default Engine Settings: `threshold=0.40`
- Target: `OWASP/juice-shop` (commit: HEAD)

### Global Signal

| Metric | Result |
|--------|--------|
| **Total True Positives (TP)** | 53 |
| **Total False Positives (FP)** | 24 |
| **Global Precision** | 68.83% |

### Per-Pattern Signal Gradient

This table ranks corpus patterns by their discriminative utility on the Juice Shop codebase. Patterns that fired are shown below:

| Pattern | TP | FP | Precision | Notes |
|---------|----|----|-----------|-------|
| `CORPUS_TS_RACE_CONDITION_READ_CHECK_WRITE` | 34 | 7 | 82.93% | Extremely strong signal. The primary driver of the benchmark success. |
| `CORPUS_TS_JWT_BYPASS` | 8 | 8 | 50.00% | High recall, but needs contextual featurization (M3) to drop FPs on mock/test JWT generation. |
| `CORPUS_TS_LLM_FALLBACK_AUTH` | 3 | 1 | 75.00% | Strong signal for custom auth fallback logic. |
| `CORPUS_TS_CSA_FIND_NEVER_EMPTY` | 2 | 0 | 100.0% | Perfect precision. Validates the Layer 1 structural fast-pass. |
| `CORPUS_TS_PATH_TRAVERSAL` | 1 | 0 | 100.0% | |
| `CORPUS_TS_CSA_MISSING_OWNERSHIP_CHECK` | 1 | 0 | 100.0% | |
| `CORPUS_TS_LLM_PROMISE_CATCH` | 1 | 0 | 100.0% | |
| `CORPUS_TSX_UNDEFINED_API_PROPERTY` | 1 | 2 | 33.33% | |
| `CORPUS_TS_DESERIALIZATION` | 1 | 1 | 50.00% | |
| `CORPUS_TS_UNAUTHENTICATED_DB_WRITE` | 1 | 3 | 25.00% | Likely firing on intentional unauthenticated writes (e.g. registration) without context. |
| `CORPUS_TS_LLM_INSECURE_RANDOM` | 0 | 1 | 0.00% | False positive. |
| `CORPUS_TS_EVAL` | 0 | 1 | 0.00% | False positive. |

*Note: The remaining 30 loaded patterns did not fire on Juice Shop backend files above the 0.40 threshold.*

## Analysis of the Benchmark

1. **The 77 TP Claim (v0.5.0):** The CHANGELOG claimed 77 TPs. The automated mapped script found **77 total findings** (53 TP + 24 FP) in the backend. It is highly likely the original author counted all 77 findings as TPs without mapping them to `challengeUtils.solveIf()`. 
2. **Actual Precision (68.8%):** A 68% precision rate on an unseen codebase with purely semantic AST patterns is an extraordinarily strong result for a static analyzer. 
3. **The Workhorses:** A single pattern (`ts_race_condition_read_check_write`) contributed 41 total findings. This confirms that Frensense generalizes very effectively on specific structural shapes, but also means that overall dataset coverage is heavily skewed.
4. **Context Featurization (M3):** The 24 FPs highlight the need for M3. `ts_unauthenticated_db_write` and `ts_jwt_bypass` are failing because they lack call-site context (e.g. is this a test file? is this the registration route?).

## Test Results

| Suite | Passed | Failed | Ignored |
|-------|--------|--------|---------|
| Total (v0.5.0) | 114 | 0 | 15 |
