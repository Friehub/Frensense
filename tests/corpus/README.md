# Accuracy Regression Corpus

Detects regressions (new false positives, lost true positives) by fixing ground-truth expectations and comparing against them in CI.

## Two-tier Architecture

### Tier 1: Curated Targets (CI-gated)

```
tests/corpus/targets/          ← hand-crafted source files
tests/corpus/baselines/targets.json  ← expected advisories
```

**CI runs**: `gensense tests/corpus/targets --json --compare-baseline tests/corpus/baselines/targets.json`

12 files (6 rules × positive/negative). The baseline contains 4 findings — only the positive variants that currently fire. Negative variants exist to document "should stay silent."

The CI step **fails** if:
- A new finding appears in any target (potential FP regression)
- A baseline finding disappears (potential FN regression)

### Tier 2: Real-repo Baselines (reference)

```
tests/corpus/baselines/tokio-1.52.3.json  ← 680 findings, 14 rules
```

Not CI-gated (requires `/tmp/tokio` at the same path). Used for manual drift analysis:
```bash
# After engine changes, compare against the reference
gensense /tmp/tokio/tokio --json --compare-baseline tests/corpus/baselines/tokio-1.52.3.json
```

## How Matching Works

`Advisory::fuzzy_identity()` returns `(rule_id, file_path, enclosing_symbol, original_content)`. The baseline comparison computes a `HashSet` of these tuples from both the baseline and the current scan, then diffs them.

Benefits:
- **Line-drift resilient**: renames/additions/deletions of surrounding code don't break identity
- **Content-sensitive**: if the matched source text changes, it's treated as a new/different finding
- **Path-anchored**: `file_path` must match (so the tokio baseline requires the same checkout path)

## Updating Baselines

When the engine intentionally changes behavior (new rule, rule relaxation, FP fix):

```bash
# Regenerate targets baseline
cargo build && ./target/debug/gensense tests/corpus/targets \
  --json --emit-baseline tests/corpus/baselines/targets.json
# Manually verify the diff:
git diff tests/corpus/baselines/targets.json
```

For tokio, same command against `/tmp/tokio/tokio`.

Commit the updated baseline alongside the engine change so CI stays green.

## Adding New Entries

### Adding a target pair

1. Create `tests/corpus/targets/<rule>_positive.<ext>` with code that MUST trigger the rule
2. Create `tests/corpus/targets/<rule>_negative.<ext>` with similar code that MUST NOT trigger
3. Regenerate the targets baseline (see above)
4. Verify the baseline includes the finding for the positive file
5. Commit both source files and the updated baseline

### Adding a real-repo baseline

1. Clone the repo to a stable path (e.g., `/tmp/tokio`)
2. Run `gensense <path> --json --emit-baseline tests/corpus/baselines/<repo>-<version>.json`
3. Document: version, scan date, command used, environment
4. Commit

## Current Coverage

### Targets baseline (4 findings across 4 rules)

| Rule | Positive file | Baseline count |
|------|--------------|----------------|
| `TS_UNAWAITED_TEST_ASSERTION` | `ts_unawaited_assertion_positive.ts` | 1 |
| `TS_GOD_FUNCTION` | `ts_god_function_positive.ts` | 1 |
| `TS_SSRF_VULNERABILITY` | `ts_ssrf_positive.ts` | 1 |
| `RUST_BLOCKING_IN_ASYNC` | `rust_async_blocking_io_positive.rs` | 1 |

**Documented but not currently firing** (fixture files exist, no baseline entry):
| Rule | Positive file |
|------|--------------|
| `RUST_PANIC_IN_LIB` | `rust_panic_in_lib_positive.rs` |
| `RUST_CLONE_IN_LOOP` | `rust_clone_in_loop_positive.rs` |

### Tokio baseline (680 findings across 14 rules)

| Rule | Count |
|------|-------|
| `RUST_UNSAFE_BLOCK` | 402 |
| `RUST_PANIC_IN_LIB` | 64 |
| `RUST_UNWRAP_SAFETY` | 62 |
| `RUST_BLOCKING_IN_ASYNC` | 48 |
| `RUST_STD_OUTPUT` | 30 |
| `RUST_ASYNC_BLOCKING_IO` | 21 |
| `RUST_GOD_FUNCTION` | 19 |
| `RUST_CONSTRUCTOR_BLOAT` | 7 |
| `RUST_ASYNC_PANIC_PREVENTION` | 6 |
| `RUST_TOKIO_SELECT_ELSE` | 6 |
| `RUST_CLONE_IN_LOOP` | 6 |
| `RUST_MUTEX_POISONING_UNHANDLED` | 4 |
| `RUST_SQL_COLUMN_MUST_EXIST_IN_PRISMA` | 4 |
| `RUST_HOST_INTERACTION` | 1 |

## CI Integration

Defined in `.github/workflows/ci.yml` in the `test-rust` job. Runs after the CLI smoke test, before MCP smoke test.

```yaml
- name: Baseline Regression Check
  run: |
    binary="./target/debug/gensense"
    echo "Checking baseline comparison against corpus targets..."
    "$binary" tests/corpus/targets --json --compare-baseline tests/corpus/baselines/targets.json
    echo "Baseline regression check passed"
```

Exit code 1 on any regression; the CI step fails and blocks the workflow.
