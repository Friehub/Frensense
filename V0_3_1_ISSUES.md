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

### MED-01 · Benchmark CI Runs on Every Branch, Causing gh-pages Conflicts
**Files:** `.github/workflows/ci.yml:266-290`  
**Time:** 30 minutes  
**Impact:** The bench job had no branch guard, so it ran on every push (feature branches, PRs). The `benchmark-action` with `auto-push: true` attempted to push to `gh-pages` from every run, causing push conflicts between concurrent CI runs. The `dev/bench/` directory never appeared on `gh-pages` because concurrent pushes to the same branch from different feature branches conflicted and were rejected.

Fix applied: Added `if: github.ref == 'refs/heads/main' && github.event_name == 'push'` to the bench job (line 239). Benchmarks now only run on `main` pushes, eliminating the conflict.

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

### MED-04 · MCP: Stream Large Scan Results
**Files:** `src/bin/gensense-mcp.rs`, `src/engine/project/mod.rs`  
**Time:** 1 hour  
**Impact:** The `gensense_audit` tool returns all findings in a single JSON-RPC response. For a 500-file monorepo with thousands of findings, the response can exceed message size limits or timeout. AI agents need incremental results to show progress.

Add a `stream: true` parameter to `gensense_audit` that emits findings as JSON-RPC notifications (one per finding or batch of findings) instead of a single response. End with a final result containing the count.

Acceptance: `gensense_audit` with `stream: true` sends `{"jsonrpc":"2.0","method":"notification","params":{"type":"finding","data":...}}` for each advisory, then a final `{"jsonrpc":"2.0","id":1,"result":{"total":N}}`.

---

### MED-05 · MCP: Add Filter Parameters to `gensense_audit`
**Files:** `src/bin/gensense-mcp.rs`  
**Time:** 30 minutes  
**Impact:** AI agents must post-process results to filter by severity, language, or rule ID. This wastes tokens and time. The MCP tool should accept optional filters.

Add optional params: `severity` (Critical|Warning|Info), `language` (rust|typescript|solidity), `rules` (list of rule IDs to include/exclude). Apply filters server-side before returning.

Acceptance: `gensense_audit` with `{"severity": "Critical", "language": "rust"}` returns only critical Rust findings. Filtering does not increase scan time (filter happens after audit, during serialization).

---

### MED-06 · MCP: Add `ping` Health-Check Method

### MED-07 · No Benchmark for `post_process_ngrams` (O(n²) Risk)
**File:** `src/engine/project/helpers.rs:42`  
**Time:** 30 minutes  
**Impact:** The Jaccard similarity comparison in `post_process_ngrams` is O(n²) over all function fingerprints — every function is pairwise compared against every other. There's no benchmark tracking its performance. As the codebase grows, this could quietly degrade without anyone noticing until it's a problem.

Add a criterion benchmark that measures `post_process_ngrams` across increasing fingerprint counts (10, 50, 200, 500 functions). Track the result in the benchmark dashboard. This is especially important before v0.4.0 when the n-gram style baseline introduces more fingerprint features.

Acceptance: CI bench job includes `post_process_ngrams` throughput (functions/second) across 4 sizes. A regression alert triggers if throughput drops below 80% of baseline.

---

### MED-06 · MCP: Add `ping` Health-Check Method
**File:** `src/bin/gensense-mcp.rs`  
**Time:** 10 minutes  
**Impact:** The JSON-RPC spec defines `ping` as a standard method. Without it, clients must call `tools/list` to check if the server is alive — which is wasteful and semantically wrong.

Acceptance: `{"jsonrpc":"2.0","id":1,"method":"ping"}` returns `{"jsonrpc":"2.0","id":1,"result":"pong"}`.

---

## 🟢 Low

### LOW-04 · MCP: Improve Startup Error Messages
**File:** `src/bin/gensense-mcp.rs`  
**Time:** 15 minutes  
**Impact:** When the engine fails to initialize (e.g., missing rules directory), the MCP server silently exits with a generic "failed to spawn" error that gives no debugging context to the AI agent.

Wrap the main initialization in a try-catch that writes a proper JSON-RPC error response to stdout before exiting: `{"jsonrpc":"2.0","id":null,"error":{"code":-32000,"message":"Engine init failed: ..."}}`.

Acceptance: A startup failure writes a JSON-RPC error to stdout so the client knows exactly what went wrong.

---

### LOW-05 · Website Not Updated for v0.3.0 (Missing MCP Docs, Changelog)
**Files:** `docs/mcp.md`, `docs/changelog.md`, `docs/.vitepress/config.mjs`, `README.md`  
**Time:** 1 hour  
**Impact:** The VitePress site at https://friehub.github.io/gensense had no MCP server documentation and no changelog/release history. Users had no way to learn about the MCP server. The README also had no MCP section.

Fix applied: Created `docs/mcp.md` (MCP server usage, tool reference, protocol details, client config examples), created `docs/changelog.md` (full release history), updated `docs/.vitepress/config.mjs` (nav + sidebar entries for MCP and Changelog), updated `README.md` (MCP server section with install and config examples).

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

## Known Bottlenecks (Unfixed)

Discovered during real-world validation (jumia-clone scan). Not yet addressed — deferred to v0.4.0.

### BTL-01 · Full Re-Scan Every Run
**File:** `src/engine/project/mod.rs:299`  
**Time:** 2–3 hours  
**Impact:** Every `gensense .` re-parses and re-audits every file. On a 500-file monorepo, even a 1-line change triggers a full scan. No incremental/diff-only mode.
**Fix:** `--diff-only` flag that compares against git index (`git diff --name-only HEAD`) and only scans changed files. Combine with content-hash cache to skip unchanged files entirely.

### BTL-02 · `post_process_ngrams` O(n²) — No Optimization
**File:** `src/engine/project/helpers.rs:44`  
**Time:** 2 hours  
**Impact:** 500 functions → 125k pairwise Jaccard comparisons. At 2000 functions it's 2M (16x more). Benchmark exists (MED-07) but no optimization was applied.
**Fix:** Add early-exit threshold (min similarity before comparison), cluster by language first, or use MinHash/LSH for approximate nearest neighbor.

### BTL-03 · No Accuracy Corpus / Ground Truth
**File:** `tests/corpus/` (does not exist)  
**Time:** 3–4 hours  
**Impact:** No one knows gensense's false positive or false negative rate. A new release could regress silently.
**Fix:** Seed `tests/corpus/` with 50–100 real PRs where known bugs were fixed. "Before" = ground-truth buggy, "after" = ground-truth clean. CI step reports TP/FP/FN per release.

### BTL-04 · Filters Are Post-Scan
**File:** `src/bin/gensense.rs:318`  
**Time:** 1 hour  
**Impact:** Changing `--severity` or `--language` re-runs the full scan. Filtering happens after audit completes — no early-exit for irrelevant files.
**Fix:** Push `--language` filter into `collect_files` so only matching extensions are walked. Push `--severity` into the audit pipeline so rules below threshold aren't even checked.
**Status: ✅ DONE.** `--language` filter now applies at `collect_files` time. File collection walks only matching extensions. Both `collect_files` (directory scan) and `run_files` (diff-only) use `ParserRegistry::extensions_for()` as the single extension lookup. `--severity` push deferred to v0.4.0.

### BTL-06 · `panic!` / `println!` Rules Fire Inside `#[cfg(test)]` Blocks
**Files:** All rules checking `panic!` or `println!` in library context  
**Time:** 1 hour  
**Impact:** `RUST_PANIC_IN_LIB` and `RUST_STD_OUTPUT` flag `panic!` and `println!` inside `#[cfg(test)]` modules and `#[test]` functions, where they are standard and expected. ~25% false positive rate in tested repos.
**Fix:** Walk AST ancestors of the matched node. If any ancestor is a `#[cfg(test)]` module or `#[test]` function, skip the finding. Implement as a shared helper in `src/rules/rust/mod.rs`.

### BTL-05 · No Content-Hash Cache
**File:** `src/engine/project/mod.rs`  
**Time:** 1–2 hours  
**Impact:** Same file scanned identically on consecutive runs. No watermark to detect that file hasn't changed.
**Fix:** On first scan, store `(path → blake3_hash)` in `.gensense/cache.json`. On subsequent runs, skip files whose hash matches.

## Resolved (Not Fixed — Closed by Decision)

| Issue | Resolution |
|-------|-----------|
| CRIT-02 · Workspace Split | Closed: Single-crate `[[bin]]` chosen over workspace split for simpler publishing. Both binaries ship via `cargo install gensense`. |
| MED-06 · MCP ping | Implemented. `{"method":"ping"}` returns `"pong"`. |
| LOW-04 · MCP Startup Errors | Resolved by lazy engine init — engine initializes on first `tools/call`, not at startup. |
| LOW-05 · Website Docs | Fixed. MCP docs, changelog, and README all updated. |

## Summary

| Severity | Count | Total Effort |
|----------|-------|-------------|
| 🔴 Critical | 2 | ~3 hours |
| 🟡 Medium | 7 | ~3 hours 15 min |
| 🟢 Low | 5 | ~3.5 hours |

Total: ~9 hours 45 min of focused work to ship a materially more robust v0.3.1.
