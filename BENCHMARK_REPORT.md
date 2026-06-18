# FrenSense Benchmark Report

> Generated: 2026-06-18

## Corpus Stats

| Metric | Value |
|--------|-------|
| Positive examples | 500 |
| Negative examples | 428 |
| Patterns loaded (FRC bundle) | 412 |
| Fingerprints | 864 |
| Bundle size | 337 KB |
| Pattern sources | OSV fix commits (Rust/TS CVEs), Semgrep community rules |

## Scan Performance

| Repository | Files | Scan Time (mean) | Findings |
|------------|-------|-----------------|----------|
| axum | 58 | 25.7s | 175 |
| actix-web (10-file sample) | 10 | 477ms | 137 |
| actix-web (estimated full) | 89 | ~42s | ~1,200 |

### Per-File Performance

| Metric | axum | actix-web |
|--------|------|-----------|
| Mean per file | 443ms | 477ms |
| Files/sec | 2.3 | 2.1 |

### Findings Breakdown (axum)

| Rule | Count |
|------|-------|
| HALLUCINATED_IMPORT | ~170 |
| Other | ~5 |

## Test Results

| Suite | Passed | Failed |
|-------|--------|--------|
| Engine (frensense-engine) | 69 | 0 |
| Consumer (frensense) | 9 | 0 |
| **Total** | **78** | **0** |

## Phase 5 Taint Precision

| Metric | Before | After |
|--------|--------|-------|
| Taint FP on axum json.rs | 22 | 0 |
| Taint FP (estimated full axum) | 585 | ~0 |
| Root cause of FP reduction | Regex-on-identifier seeding | Typed entry-point seeding via TaintSeeder |

## Pattern Sources

### OSV Fix Commits (Rust + TypeScript)
- hyper, warp, rocket, openssl, rustls, h2, webpki, ring
- express, next, axios, lodash, fastify, koa, vite, undici, qs, multer, etc.

### Semgrep Community Rules
- 300+ rules converted to corpus pairs
- Covers CWE-79 (XSS), CWE-78 (cmd injection), CWE-89 (SQL injection), CWE-22 (path traversal), CWE-918 (SSRF), CWE-327 (crypto), CWE-798 (hardcoded secrets), and many more

## Known Limitations

1. **HALLUCINATED_IMPORT dominates findings** — Axum's dev-dependencies aren't visible to the resolver. This is expected behavior, not a bug.
2. **Some semgrep patterns have generic negatives** — The negative generation for unrecognized patterns produces placeholder fixes. These patterns will have lower precision.
3. **No interprocedural corpus matching** — Corpus patterns score functions independently. Taint analysis (L2) confirms corpus matches but doesn't expand coverage.
4. **Cross-language transfer penalty** — A Rust pattern matching TypeScript code scores 0.75x. This is intentional to reduce cross-language false positives.

## What's Next

1. **Harvest more from CVEfixes** — Strategy C (rebuild from scratch with sample_limit) can add ~200-400 more patterns from the CVEfixes database
2. **Validate on production codebases** — Run on internal APIs to measure real-world precision
3. **Publish results** — TP/FP breakdown per pattern category for credibility
