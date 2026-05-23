# Benchmarks

Performance measurements for GenSense's audit pipeline using [criterion](https://github.com/bheisler/criterion.rs).

## Scan Throughput

Measured by scanning realistic project directories, running the full audit pipeline (parse + symbol discovery + rules).

| Benchmark | Mean | ±σ |
| :--- | ---: | ---: |
| `rust_clean_service` | 535ms | 20ms |
| `rust_service_with_violations` | 457ms | 19ms |
| `ts_clean_service` | 435ms | 11ms |
| `ts_mixed_real_world` | 559ms | 15ms |
| `ts_service_with_violations` | 438ms | 8ms |

All benchmarks run against ~20–30 source files each. No measurable regression
from the v0.3.1 precision pipeline (`is_rule_enabled` overhead is negligible).

## Project Scale

Scaling behavior as file count increases.

| Files | Mean | ±σ |
| ---: | ---: | ---: |
| 10 | 938ms | 18ms |
| 50 | 2.64s | 82ms |
| 100 | 4.64s | 92ms |

## Notes

- Benchmarks run in `--release` mode. Debug mode is 10–50× slower due to tree-sitter query overhead.
- The audit phase uses a single combined tree-sitter query per language (multi-pattern) instead of one query per rule, reducing AST traversals from O(R) to O(1) per file.
- Sequential iteration (no rayon) — adequate for typical project sizes and avoids futex deadlock risks.

## Historical Self-Scan

Tracks how findings evolve across a target repo's tagged history using the current GenSense binary
([`scripts/historical-benchmark.sh`](scripts/historical-benchmark.sh)).

### Usage

```bash
# Scan a target repo across all its tags
git clone https://github.com/tokio-rs/tokio.git /tmp/tokio
./scripts/historical-benchmark.sh /tmp/tokio tokio/src

# Sample every 5th tag for large repos
./scripts/historical-benchmark.sh /tmp/tokio tokio/src --sample 5
```

Output is a CSV with columns `tag,advisories,critical,warning,info`.

### GenSense Evolution (fixed target: tokio 1.47.3)

How GenSense's own rule set evolved across releases. Each version scans the same
codebase ([tokio](https://github.com/tokio-rs/tokio) `tokio/src`), isolating
rule changes from target drift.

| GenSense Version | Total | Critical | Warning | Info | Notes |
| :--------------- | ----: | -------: | ------: | ---: | :---- |
| v0.2.2 | 1,939 | 11 | 1,728 | 200 | 200 info-level noise, no `exclude_scope` |
| v0.3.0 | 1,330 | 22 | 1,308 | 0 | `info` removed; still no `exclude_scope` |
| **v0.3.1** | **397** | **9** | **388** | **0** | `exclude_scope` filters tests/benches |

Key takeaway: the 70% drop from v0.3.0 (1,330) to v0.3.1 (397) is driven by
`exclude_scope` — rules now skip test, benchmark, and build directories,
eliminating hundreds of noise findings from tokio's extensive test suite.

### Tokio Evolution (fixed tool: current GenSense HEAD)

How tokio's code quality evolved across tagged releases, as measured
by the current GenSense binary scanning `tokio/src` (sampled every 20th
tag, 388 total → 20 scanned).

| Tag | Total | Critical | Warning | Notes |
| :-- | ----: | -------: | ------: | :---- |
| 0.1.0 | 0 | 0 | 0 | Pre-modern tokio, no `tokio/src` |
| tokio-0.2.6 | 368 | 16 | 352 | First release with substantial `tokio/src` |
| tokio-0.3.0 | 305 | 14 | 291 | Transitional |
| tokio-1.6.1 | 301 | 5 | 296 | Stable, criticals dropping |
| tokio-1.13.1 | 298 | 8 | 290 | Plateau |
| tokio-1.20.3 | 245 | 2 | 243 | Lowest critical count |
| tokio-1.28.2 | 367 | 3 | 364 | Spike from new modules |
| tokio-1.40.0 | 441 | 16 | 425 | Peak — process/fs/task churn |
| tokio-1.47.3 | 397 | 9 | 388 | Stabilizing |
| tokio-1.40→1.47 | –9.9% | –43.8% | –8.7% | Downward trend |

*(Run `./scripts/historical-benchmark.sh /tmp/tokio tokio/src` for current data.)*

### Self-Scan (current GenSense on its own repo)

| Tag | Total | Critical | Warning |
| :-- | ----: | -------: | ------: |
| **HEAD** | **0** | **0** | **0** |

*(Run `./scripts/historical-benchmark.sh .` for data across all gensense tags.)*

HEAD reaches 0 warnings because `exclude_scope` and `precision` filtering
exclude the patterns found in GenSense's own source. The threshold in
`self_audit_report_warnings` is set at 165 as a safety net for regressions.
