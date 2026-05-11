# GenSense Known Weaknesses Log

This document tracks known limitations, semantic ambiguities, and performance bottlenecks in the GenSense engine. It is a mandatory part of the **Stabilization Phase** as defined in `DISCIPLINE.md`.

---

## 1. Parser Limitations

### Solidity Version Mismatch
- **Status:** SEVERE
- **Description:** The Solidity parser is currently disabled in `src/parser.rs` due to a version mismatch with the expected tree-sitter-solidity API.
- **Impact:** Solidity files cannot be analyzed.

### Simple Symbol Queries
- **Status:** STABLE but LIMITED
- **Description:** Symbol and call queries in `src/parser.rs` are relatively flat and may miss complex nested patterns or advanced language features.
- **ALGORITHM.md Note:** Whitespace splitting treats `let x=1` and `let x = 1` as different tokens. Variable names are not normalized, missing matches where only names or formatting differ.

---

## 2. Semantic Ambiguity & Core Logic

### Scope-Blind Symbol Resolution
- **Status:** KNOWN WEAKNESS
- **Description:** The current `find_definition` logic in `src/semantics/data_flow/lookup.rs` is scope-blind. It returns the first name match in the file without considering lexical scope (shadowing, blocks).
- **Impact:** False positives in taint and temporal rules where variables share names across different scopes.

### Flat Taint Registry
- **Status:** KNOWN WEAKNESS
- **Description:** `TaintRegistry` uses a flat `HashMap<String, String>`, mapping a variable to a single source. If a variable is tainted by multiple sources, only one is tracked.
- **ALGORITHM.md Note:** One variable can only have one taint origin. If `x` and `y` from different sources are combined into `z`, one origin is lost.

### Reachability Blindness (Graph BFS)
- **Status:** KNOWN WEAKNESS
- **Description:** The graph-based taint BFS has no awareness of which function a sink belongs to.
- **Impact:** Cannot distinguish between actually reachable paths and theoretically connected nodes in the graph.
- **Impact:** Incomplete taint tracking in complex data flow scenarios.

### Destructuring Blindness
- **Status:** KNOWN WEAKNESS
- **Description:** Neither the AST-local nor the Graph BFS taint implementations handle destructuring (e.g., `let (a, b) = tainted_pair`).
- **Impact:** Taint is lost during destructuring operations.

### Interprocedural Name Collisions
- **Status:** KNOWN WEAKNESS
- **Description:** Call graph construction is name-based only. If multiple functions share the same name (even in different modules/files), they are treated as the same node.
- **Impact:** Imprecise call graphs and potential false positives in cross-function analysis.

---

## 3. Temporal Analysis Weaknesses

### No Disjunction in Sequences
- **Status:** LIMITED
- **Description:** The temporal engine uses an integer counter and cannot express "Step A OR Step B".
- **Impact:** Requires writing redundant rules for similar patterns.

### Missing Balanced Counting
- **Status:** MISSING FEATURE
- **Description:** Cannot verify if `lock` and `unlock` calls are balanced (equal number of occurrences).
- **Impact:** Cannot detect leaked resources where a lock is released but not in all paths or multiple times.

---

## 4. Performance & Scaling

### Sequential File Parsing
- **Status:** BOTTLENECK
- **Description:** The analysis pipeline parses files sequentially.
- **Impact:** Underutilizes multi-core CPUs on large repositories.

### No Incremental Analysis
- **Status:** BOTTLENECK
- **Description:** Every run re-parses every file regardless of whether it changed.
- **Impact:** Slow feedback loop on large codebases.

### O(n²) Scaling Bottleneck in Large Files
- **Status:** SEVERE BOTTLENECK
- **Description:** Analyzing a single large file (e.g., 10,000 lines) with many local variables shows super-linear (likely O(n²)) complexity. 1k lines take ~4.6s, while 10k lines exceed 2 minutes.
- **Impact:** Engine may hang or become unusable on large generated files or massive monoliths.
- **Root Cause:** Likely the scope-blind `find_definition` walking the entire file or the flat symbol registry lookup iterating many candidates.

### Multi-file Scale Exhaustion
- **Status:** CRITICAL PERFORMANCE RISK
- **Description:** Analyzing a project with 5,000 functions across 100 files resulted in analysis time exceeding 20 minutes and memory usage surpassing 1.2GB.
- **Impact:** The engine is currently unsuitable for large enterprise-scale projects or massive monorepos.
- **Root Cause:** Total lack of parallelism in the file traversal loop, and likely inefficient graph node retention in the `SemanticGraph`.

---

## 5. Reliability & Correctness

### Unstable Fingerprint Hashes
- **Status:** UNSTABLE
- **Description:** `DefaultHasher` is not stable across Rust versions.
- **Impact:** Fingerprints are not reproducible across different builds/environments, breaking future caching layers.

### Whitespace Sensitivity in Tokens
- **Status:** WEAKNESS
- **Description:** Token splitting is sensitive to whitespace differences.
- **Impact:** Misses matches where only formatting differs.
