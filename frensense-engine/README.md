# frensense-engine: Core Analysis Engine

`frensense-engine` is the mathematical heart of the Frensense static analysis tool. It handles the heavy lifting of code fingerprinting, similarity scoring, and taint analysis without being tied to any specific programming language.

## Core Responsibilities

- **Code Fingerprinting (`fingerprint/`)**: Converts Abstract Syntax Trees (ASTs) into high-dimensional vectors. It extracts structural markers, n-grams, data-flow sequences, and control-flow paths.
- **Similarity Matching (`pattern/`)**: Computes structural and semantic similarities between the target codebase and known vulnerable/secure corpus examples using optimized Jaccard distances and Multi-Scale Hashing.
- **Taint Tracking (`data_flow/`)**: Executes cross-file, intra-procedural, and inter-procedural data-flow analysis to trace untrusted inputs (sources) to sensitive execution points (sinks).
- **Scoring & Confidence (`scoring/`)**: Applies confidence boosting and penalization (e.g., negative mining, taint-verified boosts, and noise-gate thresholds) to synthesize a final confidence score for a finding.

## Architecture & Integration

`frensense-engine` intentionally isolates itself from language-specific logic. 
- It consumes semantic data and AST node classifications from the `frensense-lang` crate. 
- It relies on `frensense-providers` for deep, compiler-level semantic insights (like Rust-Analyzer HIR or Oxc semantic models).

This isolation ensures that `frensense-engine` can support any new language purely through mathematical alignment and tree-sitter integration in `frensense-lang`, minimizing the need for complex, hand-written analysis rules.
