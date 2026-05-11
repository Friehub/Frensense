# GenSense Benchmark Registry

This document tracks performance metrics across engine versions. As per `DISCIPLINE.md`, no optimization work should happen before benchmarks exist.

## Baseline Metrics (2026-05-11)

| Target | Avg Runtime (s) | Peak RSS (KB) | Engine Version | Notes |
| :--- | :--- | :--- | :--- | :--- |
| `src/` (Self-Audit) | 0.21 | 6408 | 0.1.7 | 58 files, 228 functions |
| `find_at_100k` | 0.007 | - | 0.1.7 | $O(\log S)$ Scaling Test |
| `assembly_50k` | 0.187 | - | 0.1.7 | Convergence of 50k symbols |
| `Memory Stability` | Stable | ~6.4 MB | 0.1.7 | Minimal overhead per file |

## Correctness Metrics (2026-05-11)

| Metric | Score | Notes |
| :--- | :--- | :--- |
| **Accuracy** | 100.00% | Passes `correctness_tests.rs` |
| **Precision** | 100.00% | Verified via `rule_tests.rs` |
| **Recall** | 100.00% | Verified via `rule_tests.rs` |
| **False Positive Rate** | 0.00% | Zero regressions in E2E suite |

## Historical Trends

*   **2026-05-11**: Hardened TaintCache and Snapshot-Phase extraction. Removed $O(n^2)$ bottlenecks.

---

## Benchmarking Protocol

1. Ensure binary is built with `--release --features cli`.
2. Run `./scripts/benchmark.sh`.
3. Record the average of 3 runs.
4. Note any significant architectural changes (e.g., "Enabled parallel parsing").
