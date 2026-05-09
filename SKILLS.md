# GenSense: Contributor Engineering Standards

This document defines the operational mindset, technical principles, and behavioral expectations for anyone (human or AI agent) contributing to the GenSense codebase. It serves as a technical contract to ensure the production of correct, high-performance, and semantically accurate code.

---

## 1. Engineering Philosophy: "The Assembly Mindset"

We view software development through the lens of Computer Science fundamentals. We treat high-level languages with the same rigor as Assembly, focusing on how data actually flows through the machine and how the AST represents logic.

1.  **Correctness Over Convenience**: We prioritize code that is logically sound and mathematically verifiable over "clever" or "aesthetic" shortcuts.
2.  **Accuracy Over Speed**: We take the time to verify Tree-sitter queries and regex patterns against real code samples rather than assuming they "look correct."
3.  **Compositional Simplicity**: We favor composition and simple AST traversals. If a rule or function becomes too complex to explain in three sentences, it requires refactoring.
4.  **Deterministic Diagnostics**: Every engine finding should be reproducible. We use snapshots and regression testing to ensure the semantic analysis remains stable across versions.

---

## 2. Technical Skill Set (Core Competencies)

### Semantic AST Manipulation
*   **Tree-sitter Proficiency**: Deep expertise in writing high-precision queries using S-expressions.
*   **Canonical Captures**: Strict adherence to `@node` and `@capture` naming conventions to ensure engine compatibility and consistent diagnostic output.
*   **Multi-Language Grammar**: Context-aware handling of Rust, TypeScript, and Solidity grammars, respecting the unique idioms of each language.

### High-Performance Native Interop
*   **NAPI-RS**: Maintaining zero-latency bridges between the Rust engine and Node.js environments.
*   **Memory Safety**: Enforcing Rust's ownership model across the FFI (Foreign Function Interface) boundary to prevent leaks or race conditions.
*   **Build Hermeticity**: Managing feature-gated binary definitions and platform-specific linker flags with precision.

---

## 3. Discovery & Correction Protocol

Since high-complexity static analysis requires precision, we implement a protocol for handling ambiguity:

1.  **The "Stall & Search" Rule**: When encountering a version conflict, cryptic compiler error, or upstream API change, contributors should never "guess." Perform deep research using official documentation or community references.
2.  **Trade-off Visibility**: When a design decision has multiple valid paths (e.g., Engine Performance vs. Code Readability), contributors must document the trade-offs before proceeding.
3.  **Empirical Validation**: Before committing complex semantic logic, create small "scratch samples" (in `tests/samples/`) to verify assumptions against actual parser behavior.
4.  **Root-Cause Documentation**: When a logic error is discovered, analyze the root cause and update internal documentation (README or this document) to prevent recurrence.

---

## 4. Operational Guardrails

1.  **Context Sensitivity**: Be aware that static analysis is localized. Always consider cross-module side effects when modifying the core engine or common AST utilities.
2.  **Version Verification**: Always verify library dependencies and compiler toolchains against the current environment to avoid version drift.
3.  **Heuristic Honesty**: Acknowledge that static analysis is probabilistic. Flag potential issues clearly, distinguishing between "definitive bugs" and "semantic warnings."

---

## 📜 Continuous Improvement

This document is living. As the GenSense engine evolves, these standards will be refined to ensure we are building the most reliable and high-performance semantic engine possible.
