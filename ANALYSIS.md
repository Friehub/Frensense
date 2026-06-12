# Frensense v0.4.0 — Analysis & Remaining Work

## Current Architecture

Frensense detects semantic bugs through a **4-layer AND gate**:

```
L1: Corpus match    → "This function looks like a known violation shape"
L2: Taint path      → "And tainted data actually flows to a dangerous sink"
L3: Taint entropy   → "And the function branches on its tainted inputs" (built, unwired)
L4: Cross-fn check  → "And no near-duplicate function diverges" (MinHash)
```

A finding emits only when multiple layers confirm. This prevents corpus growth from linearly increasing false positives.

## What Works (Verified)

| Capability | Status | Tested On |
|-----------|--------|-----------|
| Corpus detection (L1) | Works | 30 patterns loaded, LSH pre-filter |
| Taint analysis (L2) | Works | Found password→db::execute on synthetic file |
| Secret scanning | Works | Regex+entropy for AWS/GitHub/JWT/private keys |
| MinHash duplicates (L4) | Works | >75% Jaccard near-duplicate detection |
| Taint confidence filter | Works | Kill-set analysis via CFG/def-use |
| Temporal analysis | Works | Lock/unlock, await-safety, event ordering |
| 5-language support | Works | Rust, TS, JS, Python, C via AbstractKind taxonomy |
| MCP server | Works | JSON-RPC 2.0 for AI agent integration |
| CLI + SARIF/JSON output | Works | --corpus, --severity, --strict, --baseline |
| Tokio scan (699 lines) | 0 findings | Correct — well-maintained crates should be clean |

## What Needs Attention

### Bugs
1. **Taint advisory over-flags**: `db::execute(conn, tainted_data)` creates advisories for BOTH args, flagging clean `conn` alongside tainted `data`. Fix in `resolve.rs::analyze_call`.

2. **Corpus TS file error**: `ts_hardcoded_secret_negative.ts` has no parseable function. Loader should skip gracefully without warning.

3. **Dep resolver disabled**: Hallucinated import detection unreliable on Rust without `cargo metadata`. Works for TS/JS (`package.json` is flat). Re-enable with `--check-deps` flag.

### Feature Gaps
4. **Externalize taint rules**: Six `TaintRule` structs hardcoded in Rust. Move to `taint_rules.toml` — policy in files, not code.

5. **Wire taint entropy filter**: `TaintMetrics` is computed but discarded. Functions named `validate_*` with `taint_branch_ratio < 0.2` are hollow validators — should suppress/downgrade corpus matches and taint findings.

6. **Test on real web API**: `realworld-axum-sqlx` is cloned in workspace. Run full scan to measure real-world detection quality.

### Cleanup
7. Remove `RulesWrapper` dead code in `cli/commands.rs`
8. Remove `gen`/`gensense` references in old files (`debug_registry.txt`, `clippy_errors.txt`, `Makefile`)
9. Clean up unused imports and warnings (2 remaining warnings)

## Engine Capability Matrix (Engine Crate)

| Module | State |
|--------|-------|
| `fingerprint` | Wired — n-gram hashing, structural markers, type usages |
| `lang` | Wired — AbstractKind taxonomy, 5-language mapper |
| `corpus` | Wired — loader, LSH-indexed registry, weighted Jaccard scorer |
| `data_flow/` | Wired — owned TaintRegistry, DataFlowEngine, AliasTracker, TaintConfidenceAdjuster, TaintMetrics, cross-file Resolver, PathSensitiveTaint |
| `cfg` | Wired — CFG with statement-level blocks, def-use with reaching defs |
| `pattern` | Wired — compiler, matcher, canonical form, scorer |
| `temporal` | Wired — FSA over event sequences |
| `minhash` | Wired — MinHash signatures, LSH buckets, Jaccard similarity |
| `secrets` | Wired — regex+entropy secret scanner |
| `deps` | Built, disabled — Cargo.lock/package.json dep resolution |
| `atomic_section` | Built, c_lang-gated — TOCTOU/lock-pair detection |
| `reachability` | Built — dead-branch-aware AST path analysis |

## Tests

- Engine: 58 passed, 0 failed
- Consumer: 0 tests (YAML rule tests deleted with rules)

## What Was Deleted

- YAML DSL and compiler (`src/rules/`, 67 embedded rules)
- `CoreRuleIr`, `FlowConstraint`, `FlowEvaluator`
- JSON schemas, rule authoring docs, YAML prompts
- 18 stale planning/docs files

## Key Design Decisions

- **No YAML rules** — detection is example-driven. Adding a pattern is copying two files.
- **No embedded policy** — taint rules and corpus patterns are external files.
- **Engine is analysis substrate** — no rules, no CLI, no policy. Consumer wires primitives.
- **Confidence is first-class** — every finding carries 0.0–1.0 confidence. Filtering and thresholds are built into the pipeline, not an afterthought.
