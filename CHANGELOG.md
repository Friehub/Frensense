# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-07-07

> **Note:** Versions 0.3.1 and 0.4.0 were major internal architectural iterations and were not published to NPM/Crates.io. Their changes are rolled into the 0.5.0 release, but their specific changelogs are preserved below for historical tracking.

### Added
- **Multi-Layered Composition**: Frensense now operates via a Layer 1 structural fast-pass and a Layer 2 semantic verification pass. False positives are drastically culled via the new `verify_taint_flow` integration into the `PatternScorer`.
- **Harvested Real-World Vulnerabilities**: 6 new generalized vulnerability patterns derived directly from backend audit reports (e.g., Unauthenticated DB Writes, Hyphen Drop Regex, JWT Bypass).
- **OWASP Juice Shop Proven**: Scanner generalizes to find 77 true-positive vulnerabilities in the unseen OWASP Juice Shop repository without explicit fine-tuning.

### Changed
- **Fully corpus-driven detection**: All detection is now driven by positive/negative example pairs. Removed hardcoded TOCTOU detector (`check_then_act.rs`), temporal rules TOML (empty), and taint-as-detection. Detection layers: corpus fingerprint match → taint verification → taint entropy → cross-function consistency.
- **CorpusSourceSinkRegistry**: Sources and sinks are learned from corpus AST at load time. Replaces three inconsistent `framework_types` arrays and two duplicate `identify_sink()` functions.
- **Temporal detection is corpus-driven**: `rust_temporal_lock_unlock`, `rust_temporal_lock_sleep`, `ts_temporal_open_close`, `ts_temporal_connect_disconnect` patterns replace hardcoded TOML rules.
- **TOCTOU generalization**: `ts_toctou_typeorm`, `ts_toctou_sequelize` patterns extend detection beyond Prisma-only.
- **Corpus suppression support**: `.frensense-suppress.yml` now applies to corpus findings.
- **603 positive corpus patterns** (from 89 in v0.3.x). 1214 fingerprints. 3.0MB FRC bundle.

### Removed
- `check_then_act.rs` — hardcoded TOCTOU detector (replaced by corpus patterns)
- `temporal_rules.toml` rules — replaced by corpus patterns (file retained as empty)
- `ROADMAP.md` — superseded by tasks.md and SCALING_PLAN.md
- Stale test references to `TAINT_CREDENTIAL_TO_LOG`, `TAINT_INPUT_TO_EXEC` (taint-as-detection removed)
- Commodity detectors: `dead_branch.rs`, `unused_variable.rs`, `atomic_section.rs`, `secrets.rs` — Clippy/GitLeaks do them better
- `reachability.rs` — only used by removed dead_branch detector

## [0.4.0] - Unreleased / Internal

## [0.3.1] - Unreleased / Internal

### Added
- **`taint_max_depth` Rule DSL field**: Rules can set `taint_max_depth: <N>` in YAML to control cross-function taint chain length per-rule. Falls back to 5 (existing default) when unset.
- **Visited-set cycle detection in `resolve_call_taint`**: Prevents re-analysis and infinite recursion when the same callee is encountered multiple times during taint resolution. Tracks `(file_path, start_byte)` pairs per analysis.
- **Match-arm and if-expression return propagation**: `find_returns()` now explicitly walks `match_expression` arms and `if_expression` consequence/alternative branches as potential return values, improving intra-procedural taint flow through conditional logic.
- **Rule quality pipeline**: Every rule now carries a `precision` tier (`very-high | high | medium | low`), letting users choose a rule suite via `--suite {default|extended|all}`. `default` runs only `very-high` rules (battle-tested, near-zero false positives). `extended` adds `high` rules (well-tested, occasional FP). `all` runs every rule (current behavior, unchanged as default).
- **`--suite` CLI flag**: `frensense --suite default path/` filters to high-confidence findings only. Backward compatible — existing invocations without `--suite` behave identically.
- **Historical self-scan benchmark**: `scripts/historical-benchmark.sh` scans a target repo at every tagged version with the current frensense binary and outputs a CSV showing how advisory counts evolved over time. Documented in `BENCHMARK.md`.

### Changed
- **Consolidated single-binaries into unified crate**: `cargo install frensense` now produces both `frensense` (CLI) and `frensense-mcp` (MCP server) binaries. Removed separate `frensense-cli` and `frensense-mcp` workspace crates.
- **MCP filter params**: Added `language` (file extension matching) and `rules` (rule_id set) parameters to `frensense_audit` tool, applied server-side via `filter_advisories` after scan.
- **Clippy pedantic compliance**: All ~35 `clippy::pedantic` violations fixed. The four `-A` flags removed from CI and pre-commit hook.

### Added
- **License headers**: `// SPDX-License-Identifier: MIT` added to 13 unattributed files. Solidty rule changed from proprietary to MIT. Codebase is now 100% MIT-consistent.
- **Shared `RulesWrapper`**: Extracted duplicated `RulesWrapper + check_version()` into `src/engine/auditor/common.rs`.
- **Shared `is_in_async_scope`**: Extracted duplicated helper into `src/rules/rust/mod.rs`.

### Fixed
- **MCP tests**: `test_mcp_language_filter` and `test_mcp_rules_filter` verify server-side filtering works correctly. 36/36 MCP tests pass.
- **Binary file crash**: `collect_files` now filters to supported extensions via `ParserRegistry::is_supported`, preventing `"stream did not contain valid UTF-8"` panic on binary files (`.term`, `.idx`, `.store`, etc.).
- **Macro test**: `test_cli_json_output` updated for consolidated crate structure.
- **Pre-commit hook**: Now runs full test suite (`cargo test`) instead of `cargo test --lib --bins`.
- **File extension**: `research/sparse_spectral_enginev2.rs` renamed to `.md`.
- **Package.json**: Removed non-existent `index.js` from `files` array.

### Added
- **Native TypeScript rule `TS_TAUTOLOGICAL_ASSERT`**: Detects `expect(x).toBe(x)`, `expect(true).toBeTruthy()`, `expect(null).toBeNull()` via AST walk. Registered under `#[cfg(feature = "typescript")]`. 7 test cases.
- **`temporal` feature flag**: New Cargo feature gates `TemporalAnalyzer`, `TemporalConfig`, and all temporal compilation/execution paths. On by default. Allows `cargo build --no-default-features` to exclude temporal analysis.
- **Feature ownership map**: `FEATURE_MAP.md` documents exactly which files each differentiator (temporal, schema_contract, mcp, csa) owns — no more guessing what lives where.
- **Gap analysis → build plan**: `GAP_ANALYSIS.md` restructured into 6 priority-ordered phases (P0–P5) with tickable checkboxes, aligned to v0.4.0 plan.

### Changed
- **Temporal analyzer moved to dedicated folder**: `src/temporal/` consolidates `TemporalAnalyzer`, `TemporalConfig`, and handler delegation. Scattered call sites in `ir.rs` and `compiler.rs` reduced to 1-line delegations. Old `src/semantics/temporal.rs` removed.
- **Precision assigned to all 60 rules**: 10 Rust hand-written rules set to `very-high`, 2 AI-pattern rules set to `high`, 48 YAML rules tiered by confidence score (39 `very-high`, 9 `high`).

### Removed
- **14 style/noise YAML rules**: Self-audit findings dropped from 186 to 69.
- **10 Solidity rules and `solidity` feature**: Dead code — feature not compiled, no tree-sitter support. Includes 7 core, 2 security, 2 CSA rules.
- **Old bug tracking docs**: `V0_3_1_ISSUES.md`, `V0_3_1_REPORT.md`, `AUDIT_V0.3.0_REPORT.md`.

## [0.2.2] - 2026-05-14

### Added
- **Solidity Beta Support**: Integrated tree-sitter-solidity. Enable with `--features solidity`.
- **Enhanced Temporal Rules**: Added `forbidden_between` behavior to allow detecting prohibited calls between specific state boundaries.
- **Architectural Guardrails**: Added comprehensive documentation and project-level rules for enforcing system architecture (e.g., service/handler separation).

## [0.2.1] - 2026-05-13

### Fixed
- **CI: Missing Rust toolchain in `publish-npm` job**: `cargo run` was called before Rust was installed, causing NPM to never publish past v0.1.4.
- **CI: NAPI features included in `cargo publish`**: Default features pulled in `napi-build` unconditionally, causing `cargo package` to fail in environments without NAPI headers. Scoped to `--no-default-features --features rust,typescript,fingerprinting,remediation`.
- **CI: `aarch64-apple-darwin` cross-compiled on wrong host**: Changed `macos-latest` (x86) to `macos-14` (Apple Silicon) for native arm64 builds.
- **CI: Mutable action tags**: All GitHub Action references pinned to immutable SHA digests.
- **CI: Broad branch trigger**: `ci.yml` narrowed from `["**"]` to `main`, `feature/**`, and `fix/**`.
- **CI: `npm install` fallback**: Replaced `npm ci || npm install` with strict `npm ci`.
- **CI: Sync loop guard**: `sync.yml` now also skips its own "sync version" commits to prevent re-trigger loops.
- **CI: Concurrency controls**: Added `concurrency` groups to all three workflows to prevent duplicate runs.

## [0.2.0] - 2026-05-13

### Added
- **Multi-File Scanning (GenSense 2.0)**: Stabilized the core semantic architecture to support project-wide auditing.
- **Graph-First Semantic Engine**: Migrated to a global symbol graph for high-precision inter-procedural taint analysis across files.
- **Multi-Pass Audit Loop**: Implemented a sophisticated audit pipeline that performs local AST checks followed by cross-file project-level enforcement.
- **Cross-File Project Rules**: Full support for `MustHaveGuard`, `MustBeInternal`, and `CrossFileTaintFree` rules.
- **Node.js API Extensions**: Exposed `auditProject` and `auditPath` for full project-wide scanning from JavaScript.
- **Supply Chain Security**: Integrated `cargo deny`, `npm audit`, and SLSA Level 3 provenance for production-grade security.
- **Egress Auditing**: Integrated StepSecurity Harden Runner for CI network protection.

### Fixed
- **BFS Deduplication**: Fixed a critical bug where BFS traversal incorrectly deduplicated symbols with the same name across different files.
- **Project Advisory Metadata**: Resolved an issue where `original_content` was empty for project-level findings, unblocking `--fix` mode.
- **JS API Gaps**: Fixed `audit_content` incorrectly being used for project-wide rules; redirected users to the new `audit_project` method.
- **Clippy Violations**: Fixed several redundant cast warnings and unnecessary cloning identified by static analysis.

### Changed
- **MSRV Bump**: Increased Minimum Supported Rust Version (MSRV) to **1.77** to support modern build script features.
- **CI Hardening**: Pinned all GitHub Action references to full immutable SHAs to prevent supply chain attacks.
- **DSL Cleanup**: Removed the redundant and unused `domain` field from rule definitions to simplify the DSL.

### Removed
- Unused `domain` metadata from all core and embedded rule definitions.
