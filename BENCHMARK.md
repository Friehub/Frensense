# Benchmarks

Performance measurements for GenSense's audit pipeline using [criterion](https://github.com/bheisler/criterion.rs).

## Scan Throughput

Measured by scanning realistic project directories, running the full audit pipeline (parse + symbol discovery + rules).

| Benchmark | Mean | ±σ |
| :--- | ---: | ---: |
| `rust_clean_service` | 534ms | 14ms |
| `rust_service_with_violations` | 455ms | 10ms |
| `ts_clean_service` | 416ms | 27ms |
| `ts_mixed_real_world` | 532ms | 7ms |
| `ts_service_with_violations` | 461ms | 4ms |

All benchmarks run against ~20–30 source files each.

## Project Scale

Scaling behavior as file count increases.

| Files | Mean |
| ---: | ---: |
| 10 | 1.3s |

## Notes

- Benchmarks run in `--release` mode. Debug mode is 10–50× slower due to tree-sitter query overhead.
- The audit phase uses a single combined tree-sitter query per language (multi-pattern) instead of one query per rule, reducing AST traversals from O(R) to O(1) per file.
- Sequential iteration (no rayon) — adequate for typical project sizes and avoids futex deadlock risks.
