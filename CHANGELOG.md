# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### [0.3.1] - 2026-05-22

### Changed
- **Consolidated single-binaries into unified crate**: `cargo install gensense` now produces both `gensense` (CLI) and `gensense-mcp` (MCP server) binaries. Removed separate `gensense-cli` and `gensense-mcp` workspace crates.
- **MCP filter params**: Added `language` (file extension matching) and `rules` (rule_id set) parameters to `gensense_audit` tool, applied server-side via `filter_advisories` after scan.
- **Clippy pedantic compliance**: All ~35 `clippy::pedantic` violations fixed. The four `-A` flags removed from CI and pre-commit hook.

### Added
- **License headers**: `// SPDX-License-Identifier: MIT` added to 13 unattributed files. Solidty rule changed from proprietary to MIT. Codebase is now 100% MIT-consistent.
- **Shared `RulesWrapper`**: Extracted duplicated `RulesWrapper + check_version()` into `src/engine/auditor/common.rs`.
- **Shared `is_in_async_scope`**: Extracted duplicated helper into `src/rules/rust/mod.rs`.

### Fixed
- **MCP tests**: `test_mcp_language_filter` and `test_mcp_rules_filter` verify server-side filtering works correctly. 36/36 MCP tests pass.
- **Macro test**: `test_cli_json_output` updated for consolidated crate structure.
- **Pre-commit hook**: Now runs full test suite (`cargo test`) instead of `cargo test --lib --bins`.
- **File extension**: `research/sparse_spectral_enginev2.rs` renamed to `.md`.
- **Package.json**: Removed non-existent `index.js` from `files` array.

### Changed (Breaking)
- **Rust API: `GenSenseAuditor::audit`**: Consolidated 10+ arguments into a single, extensible `AuditOptions` struct. This simplifies the call site and future-proofs the audit pipeline.
- **Rust API: `GenSenseRule::new_remediation`**: Added a mandatory `import: Option<String>` parameter to support auto-injection of missing imports during patching.
- **Rust API: `CoreRuleIr::query`**: Always returns the compiled query string; removed `use_query` guard. All YAML node-kind rules now participate in the combined query.
- **Data Model: `Advisory`**: Added mandatory fields `proposed_import`, `enclosing_symbol`, `confidence`, and `fingerprint` for higher fidelity result tracking.
- **Edition Upgrade**: The project now requires **Rust 2024 Edition**.
- **Parallelism removed**: Rayon `into_par_iter()` replaced with `into_iter()` in snapshot collection and audit phases — eliminates futex deadlock class while maintaining adequate performance (~4.5s for 68 files).

### Added
- **Schema Contract Validation**: New `ProjectFlowConstraint::SchemaContract` variant with `SchemaType`/`SchemaExtract` enums. Flattened DSL fields (`source_ext`, `source_pattern`, `source_file_glob`, `schema_type`, `schema_glob`, `schema_extract`) on `ProjectCoreRule`. Block-aware Prisma extractor replaces fragile line-based parser. Standard DB rules in `cross-layer-contracts.yml`.
- **MCP Server Binary**: `gensense-mcp` — full JSON-RPC 2.0 over stdin/stdout. Tool `gensense_audit` returns `{clean, advisories, auto_fixed, requires_human}` with `severity_threshold` filtering and optional `fix_auto`. 35 integration tests (34 pass, 1 ignored in debug mode).
- **Field-Path Taint Propagation**: `TaintRegistry::get_any_field_origin` detects whole-object leaks when only specific fields are tainted. `resolve_taint` now falls back to field-taint check for identifiers.
- **Combined Tree-Sitter Query**: Single AST traversal per language merges all rule queries into one multi-pattern query. Capture names encode rule IDs (`@{rule_id}.node`). `HashSet<(rule_idx, node_id)>` dedup prevents redundant `check()` calls. Eliminates O(F × R × T) scaling.
- **Cached `run_content` Init**: Only calls `initialize_auditor_and_config` on first invocation (~243ms saved per subsequent call).
- **Schema Documentation**: `BENCHMARK.md` with criterion benchmark results.
- **CI: Benchmark Dashboard**: `pages` job publishes criterion report via GitHub Pages.
- **CI: `--force` install**: `cargo install cargo-hack --locked` and `cargo install cargo-machete --locked` use `--force` to handle cached binaries.
- **Missing Tests**: `test_non_remediated_advisory_is_not_auto_fixable`, `test_requires_human_is_true_for_project_rule_advisories`, `test_prisma_extractor_handles_model_with_block_on_same_line`, `test_prisma_extractor_handles_multiple_schema_files`, `test_mcp_audit_response_contains_requires_human_field`, `test_mcp_audit_auto_fixed_is_zero_when_no_fixable_advisories`.

### Fixed
- **MCP Null-ID Hang**: JSON-RPC `"id": null` no longer hangs — `RequestId` enum (`Absent | Null | Value`) with custom deserializer replaces `Option<Value>`.
- **Sink/Source Matching**: `analyze_call` checks both short `fn_name` and full call expression text against sink/source regex — fixes `console.log(payload)` matching when semantic op extracts only `"log"`.
- **Cross-Layer Contracts Regex**: Table pattern requires PascalCase (`'"?([A-Z][a-zA-Z0-9]+)"?'`), column pattern requires double-quoted camelCase — drastically reduces false positives.
- **Prisma Extractor**: Block-aware state machine replaces line-based parser — correctly handles model/enum boundaries and same-line braces.
- **`find_project_root`**: Prefers `std::env::current_dir()` unconditionally over fragile source-path marker-file heuristics.
- **Governance Check**: `MISSING_SBOM` removed from automatic `run_detailed` — kept as `Engine::run_governance_checks()` opt-in helper.
- **Build Artifacts in Git**: Purged 53 build artifact files from git history via `git filter-repo`.
- **Benchmark Raw String**: Fixed `r#"..."#` delimiter clash in `engine_perf.rs`.
- **`SymbolKind::Method` → `Struct`**: Updated deprecated `SymbolKind` variant in benchmarks.
- **Baseline Resilience**: Resolved "Line-Drift" fragility where vertical code shifts would cause false positive regression failures.
- **Engine: Extension Matching**: Fixed `applies_to` to correctly handle multi-extension strings (e.g., `ts|js|tsx|jsx`).
- **Engine: Query Safety**: Implemented graceful skipping of language-incompatible Tree-sitter queries during cross-language scans.
- **CLI: Descriptive Reporting**: Updated reporting to include regression/resolved summaries and net change metrics.
- **JS/NAPI SRI Bridge**: Exposed semantic anchoring metadata to the Node.js API for consistent identity tracking.

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
