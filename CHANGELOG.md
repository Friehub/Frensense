# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-05-15

### Added
- **Auto-Remediation Engine**: Many rules now support automated fixes via the `--fix` and `--diff` flags, including prisma select injection and service-layer replacement.
- **Rule Authoring & Schemas**: Added v0.3.0 and v0.2.2 JSON schemas for IDE IntelliSense and strict LLM validation.
- **Rule Generation Expert**: Released the `rule-expert.md` system prompt to enable non-tech users and LLMs to generate high-precision GenSense rules.
- **Contract Surface Analysis (CSA)**: New framework to detect semantically hollow code by verifying implementation bodies against function name promises.
- **Symbol-Relative Identity (SRI)**: Transitioned to logical symbol-anchored identity, eliminating "line-drift" false positives in CI baselines.
- **Performance Benchmarks**: Integrated `criterion` to track engine scanning, rule compilation, and patching throughput.
- **Self-Audit**: The engine now scans its own source code for quality and security regressions, integrated as a hard gate in CI.
- **Fuzzy Baseline Comparison**: Implemented a resilient matching engine that recognizes findings across line shifts, stabilizing technical debt tracking.
- **CSA Rule Family**: Integrated 8 new rules for Validators, Sanitizers, Finders, and Auth across TypeScript, Rust, and Solidity.
- **Structural AST Auditing**: Extended DSL with `if_name_matches` and `body_must_contain` for cross-language semantic verification.
- **Baseline / Regression Mode**: Added `--emit-baseline` and `--compare-baseline` 
flags to enable "diagnostic ratcheting" and enforce quality gates in CI.

### Fixed
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
