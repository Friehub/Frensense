# v0.3.1 — Issues to Tackle

All issues discovered during v0.3.0 hardening. Each is mapped to a file, has a clear acceptance criterion, and is ordered by severity.

---

## 🔴 Critical

### CRIT-01 · `Engine::run()` Silently Returns Empty on Invalid Path
**File:** `src/engine/project/mod.rs:124` (and `run_detailed` at line 197)  
**Time:** 10 minutes  
**Impact:** Any caller that passes a non-existent path gets an empty advisory list instead of an error. CLI, MCP, JS binding all affected.

`collect_files` (line 383) uses `WalkDir::new(root).filter_map(Result::ok)` — when the root doesn't exist, `WalkDir` yields an error iterator entry, which gets silently filtered out, producing an empty file list and zero advisories.

Acceptance: `Engine::run()` and `run_detailed()` check `root.exists()` and return `Err(GenSenseError::Io(...))` for invalid paths. The NAPI layer (`src/js.rs`) should keep its own check as a belt-and-suspenders.

---

### CRIT-02 · Workspace Split to Eliminate Feature-Flag Cross-Contamination
**Files:** `Cargo.toml`, `src/bin/gensense.rs`, `src/bin/gensense-mcp.rs`, `src/js.rs`  
**Time:** 2–3 hours  
**Impact:** Feature flags (`mcp`/`cli`/`node`) leak between binary targets. `napi build` must exclude the MCP binary (fixed in 68e7ac7 with a band-aid), and `cargo test --features cli` must include `mcp` or the MCP tests silently fail.

Split the crate into a workspace with 4 crates:
- `gensense-lib` — core engine, no binaries, no napi
- `gensense-cli` — CLI binary, depends on `gensense-lib`
- `gensense-mcp` — MCP binary, depends on `gensense-lib`
- `gensense-node` — NAPI cdylib, depends on `gensense-lib` + `napi`

Each only compiles what it needs. No feature flags, no `required-features`, no accidental linking of napi symbols into a regular binary (root cause of the 4-hour NAPI debug earlier).

Acceptance: `cargo build -p gensense-cli`, `cargo build -p gensense-mcp`, `napi build` (for gensense-node) all work without special feature flags. `cargo test -p gensense-lib` runs without building any binaries. MCP tests use `cargo test -p gensense-mcp`.

---

## 🟡 Medium

### MED-01 · Benchmark CI is Fragile
**Files:** `.github/workflows/ci.yml:266-290`  
**Time:** 30 minutes  
**Impact:** The bench job has 3 serial steps (install cargo-criterion → run → jq convert) and takes ~5 minutes. Each has its own failure mode — network timeout on install, benchmark takes too long, jq parsing errors.

Acceptance: Add a `--quick` mode via env var (`GENENSE_BENCH_QUICK=1`) that reduces `sample_size` and `measurement_time` so CI completes in < 1 minute. The full benchmark suite can be triggered manually or on `main` merges.

---

### MED-02 · Version Drift: Docker, CI, MSRV
**Files:** `Dockerfile:2`, `Cargo.toml:14`, `.github/workflows/ci.yml:142`  
**Time:** 15 minutes  
**Impact:** Three different Rust versions: Docker had `1.75` (now `slim-bookworm` / latest), CI uses `stable`, Cargo.toml declares `rust-version = "1.88"`. If Docker resolves to a different major version than CI, builds can break silently.

Acceptance: Pin Docker to `rust:1.88-slim-bookworm` to match the declared MSRV. Add a CI step that runs `cargo check` with MSRV (currently `1.88`) so the MSRV claim stays verified.

---

### MED-03 · MCP Tests Not Hermetic
**File:** `tests/mcp_tests.rs:36`  
**Time:** 20 minutes  
**Impact:** `env!("CARGO_BIN_EXE_gensense-mcp")` at compile time. If the binary isn't built (e.g., wrong feature flags), all 34 tests fail with a confusing spawn error rather than a clear "binary not found" message.

Each test should wrap the spawn in a helper:
```rust
fn spawn_mcp() -> (Child, ChildStdin, BufReader<ChildStdout>) {
    let bin_path = env!("CARGO_BIN_EXE_gensense-mcp");
    assert!(
        Path::new(bin_path).exists(),
        "gensense-mcp binary not built (try --features mcp)"
    );
    // ... rest of spawn
}
```

Acceptance: Running tests without the `mcp` feature gives a clear panic message instead of 34 cryptic failures.

---

## 🟢 Low

### LOW-01 · Pre-Commit Hook Doesn't Run Integration Tests
**File:** `hooks/pre-commit`  
**Time:** 10 minutes  
**Impact:** The hook only runs `cargo test` (no filter), which runs 0 unit tests (lib has no unit tests). A formatting-only change that breaks integration tests passes the hook and fails in CI 10 minutes later.

Acceptance: Hook runs `cargo test --features full --tests` to at least run the integration suites, or just remove the test invocation from the hook and trust CI.

---

### LOW-02 · `package.json` Lists Non-Existent `index.js`
**File:** `package.json:9`  
**Time:** 5 minutes  
**Impact:** `"index.js"` is in the `files` array but doesn't exist on disk. Harmless for publishing (missing files are silently skipped), but confusing to anyone reading the manifest.

Acceptance: Either create `index.js` as a re-export of `gensense.js`, or remove it from the `files` array.

---

### LOW-03 · ~35 Remaining `clippy::pedantic` Lints Globally Allowed
**Files:** `.github/workflows/ci.yml`, `hooks/pre-commit`, `.git/hooks/pre-commit`  
**Time:** 2 hours (incremental)  
**Impact:** 4 lints are globally allowed (`collapsible_if`, `collapsible_match`, `unnecessary_sort_by`, `unnecessary_trailing_comma`) with ~35 violations total. Every new contributor has to know about these exceptions.

Acceptance: Fix all instances and remove the `-A clippy::...` flags from the clippy command. Each category should be a single focused PR: collapsible_if (17 instances), collapsible_match (1), unnecessary_sort_by (1), unnecessary_trailing_comma (2), plus doc_markdown (which was 1 instance and was fixed).

---

## Summary

| Severity | Count | Total Effort |
|----------|-------|-------------|
| 🔴 Critical | 2 | ~3 hours |
| 🟡 Medium | 3 | ~1 hour |
| 🟢 Low | 3 | ~2.5 hours |

Total: ~6.5 hours of focused work to ship a materially more robust v0.3.1.
