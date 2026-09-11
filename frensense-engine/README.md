# frensense-engine: Core Analysis Engine

`frensense-engine` is the mathematical and structural heart of the Frensense static analysis tool. It handles the heavy lifting of code fingerprinting, similarity scoring, graph reachability, and taint analysis without being tied to any specific programming language.

## Architecture & Integration

`frensense-engine` intentionally isolates itself from language-specific logic:
- It consumes semantic abstractions (ASTs, data-flows) purely from `frensense-lang`. 
- It relies on `frensense-providers` for deep, compiler-level semantic insights (e.g., Rust-Analyzer HIR, Oxc semantic models).

This isolation ensures that `frensense-engine` can support any new language strictly through mathematical alignment and tree-sitter integration in `frensense-lang`, minimizing the need for complex, hand-written heuristics.

## Core Responsibilities

### 1. Code Fingerprinting (`fingerprint/`)
Converts Abstract Syntax Trees (ASTs) into high-dimensional vector representations (`FeatureVec`). 
- **AST Walkers**: Slices target files into granular functions and top-level statements.
- **Structural Markers**: Extracts structural skeletons, n-grams, and literal pattern hashes.
- **Data-Flow Fingerprinting**: Evaluates variable lifecycles, def-use chains, and config assignments.

### 2. Similarity Matching (`pattern/`)
Computes structural and semantic similarities between the target codebase and known vulnerable/secure corpus examples.
- Uses optimized `jaccard_sorted` calculations to avoid `O(N^2)` memory/time degradation.
- Leverages **Multi-Scale Hashing (MSH)** for fast-path textual overlap verification.

### 3. Taint & Graph Reachability (`data_flow/`)
- Fuses structural Program Dependence Graphs (PDGs) with abstract data-flow definitions.
- Identifies exact graph-reachability paths from external input (Sources) to vulnerable execution parameters (Sinks).
- Tracks `IdentifierReference`, `BindingIdentifier`, and `property_identifier` assignments to prevent phantom propagation.

### 4. Scoring & Calibration (`scoring/`)
Synthesizes a final confidence score by calibrating dimensions across multiple scales:
- Disables aggressive threshold shortcuts (e.g. `threshold.min(0.16)`) to preserve global threshold fidelity and eliminate false positive spikes.
- Leverages per-category weights learned directly from the corpus.
- Employs strict deduplication logic to aggregate overlapping mutations (TryCatch variants, Async wrappers) down to a single, high-confidence advisory per vulnerability class per function.
