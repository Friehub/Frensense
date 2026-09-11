# frensense-providers: Compiler-Backed Semantics

`frensense-providers` acts as a deep-semantic bridge between the Frensense engine and language-specific compilers or advanced frontends.

While `frensense-lang` uses Tree-sitter for fast, structural AST parsing, `frensense-providers` integrates actual compiler frontends to extract 100% accurate semantic data (type resolution, module resolution, and definition maps) when running in high-accuracy mode (`--use-compiler`).

## Supported Providers

### 1. Oxc (JavaScript & TypeScript)
Integrates the `oxc_semantic` and `oxc_resolver` crates.
- **Capabilities:** Resolves complex TypeScript interfaces, exact module imports/exports, and precise scope/variable shadowing that Tree-sitter alone cannot decipher.

### 2. Rust-Analyzer HIR (Rust)
Integrates the `ra_ap_hir` (High-Level Intermediate Representation) crates from `rust-analyzer`.
- **Capabilities:** Provides perfect type inference, trait resolution, and macro expansion tracking for Rust codebases.

## Architecture
Providers are optional and feature-gated (`#[cfg(feature = "oxc")]`, `#[cfg(feature = "rust-hir")]`). When enabled, the engine enriches its standard structural data-flow with guaranteed compiler facts, vastly reducing false positives in highly dynamic codebases.
