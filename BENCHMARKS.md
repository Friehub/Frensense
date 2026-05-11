# GenSense Benchmark Registry

This document tracks performance metrics across engine versions. As per `DISCIPLINE.md`, no optimization work should happen before benchmarks exist.

## Baseline Metrics (2026-05-11)

| Target | Avg Runtime (s) | Peak RSS (KB) | Engine Version | Notes |
| :--- | :--- | :--- | :--- | :--- |
| `src/` | 7.23 | 25364 | 0.1.7 | Self-analysis (Rust) |
| `tests/samples` | 0.86 | 31348 | 0.1.7 | Small multi-language samples |
| `1k lines` | 4.66 | 11976 | 0.1.7 | Scaling test (synthetic) |
| `10k lines` | >120.0 | - | 0.1.7 | **FAIL (O(n²) Bottleneck)** |
| `100 files / 5k funcs` | >1200.0 | ~1200 MB | 0.1.7 | **FAIL (Scale Exhaustion)** |
| `Memory Stability` | Stable | ~25 MB | 0.1.7 | 20 runs on `src/` (No Leaks) |

## Correctness Metrics (2026-05-11)

| Metric | Score | Notes |
| :--- | :--- | :--- |
| **Accuracy** | 100.00% | Based on `correctness_samples/` |
| **Precision** | 100.00% | No False Positives detected |
| **Recall** | 100.00% | All expected positives detected |
| **False Positive Rate** | 0.00% | |

## Historical Trends

*No data yet.*

---

## Benchmarking Protocol

1. Ensure binary is built with `--release --features cli`.
2. Run `./scripts/benchmark.sh`.
3. Record the average of 3 runs.
4. Note any significant architectural changes (e.g., "Enabled parallel parsing").
