# GenSense v0.3.0 Audit: Jumia-Clone False Positives Report

This report documents the sources of "noise" and false positives identified during the stress test of GenSense v0.3.0 against the `jumia-clone` repository (3,070 total findings).

## 1. Summary of Noise Sources

| Category | Finding Count | Root Cause |
| :--- | :--- | :--- |
| **Rule Spam** | 1,112 | `TS_ANY_TYPE` flags every `any` usage, which is standard in early-stage TS codebases. |
| **Rule Overlap** | 1,303 | `TS_NESTING_LIMIT` (3) and `JUMIA_NESTING_LIMIT` (4) both fire on the same code blocks. |
| **AST Duplication** | ~350 | Rules defined on generic `call_expression` nodes fire multiple times for nested/chained calls. |
| **False Logic** | ~300 | Heuristic-based rules (regex on raw text) lack the context of existing reachability or guards. |

## 2. Technical Breakdown of False Positives

### A. Overlapping Thresholds
The codebase contains many deeply nested functions. Because we enabled both `TS_NESTING_LIMIT` and a custom `JUMIA_NESTING_LIMIT`, a single function with depth 5 produced two findings for the same architectural "smell."
*   **Fix:** Rule inheritance or mutual exclusion flags should be implemented in `compiler.rs`.

### B. Recursive AST Matching
In chaining patterns common in Rust (e.g., `reqwest` or `axum`), a rule targeting `.unwrap()` would fire on every parent node of the unwrap call if the selector was too broad.
*   **Example:** `Router::new().route(...).unwrap().layer(...)` triggers twice because the outer `layer` call contains the `.unwrap()` text in its source.
*   **Fix:** Refine `RUST_UNWRAP_SAFETY` to target specifically the `Identifier` or `MemberExpression` node, not the whole `CallExpression` chain.

### C. Contextual Blindness (The "High-Signal Hint" Problem)
Heuristic rules like `body_must_contain` check for the presence of a string (e.g., `validator`) but don't know if that string is actually executed or if the code is unreachable.
*   **Impact:** A finding might say "Missing validation" even if the validation is performed in a middleware or a separate module that the current file doesn't "see."

## 3. Mitigation Strategies Implemented/Proposed

1.  **Confidence Scores:** Added a `confidence: f32` field to `Advisory`. AST-proven findings (e.g., `TautologicalAssert`) get 0.85+, while regex heuristics get 0.55.
2.  **Rule Consolidation:** Merged redundant nesting rules into a parameterized config.
3.  **Strict Mode:** Introduced `--min-confidence` flag to the CLI to filter out heuristic noise in CI environments.
4.  **Reachability Anchors:** Rules now support `requires_reachability: true` to prevent firing in dead code paths.
