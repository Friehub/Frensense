# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-13

### Added
- **Cross-File Project Rules**: Full support for `MustHaveGuard`, `MustBeInternal`, and `CrossFileTaintFree` rules in the core engine.
- **Node.js API Extensions**: Exposed `auditProject` and `auditPath` to the Node.js bindings for comprehensive project-wide analysis.
- **Supply Chain Security**: Integrated `cargo deny` for dependency auditing and `npm audit` for JS package security.
- **SLSA Provenance**: Added Level 3 provenance attestation to release artifacts for cryptographic build verification.
- **Egress Auditing**: Integrated StepSecurity Harden Runner for outbound network call monitoring in CI.
- **E2E Testing**: Added comprehensive integration tests for project-wide rules and configuration overrides.

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
