# Getting Started

## What is Frensense?

Frensense is a **Compositional Taint Analysis Engine**. It analyzes source code at the Abstract Syntax Tree (AST) level to detect logical flaws, security vulnerabilities, and architectural anti-patterns that conventional linters cannot identify.

Unlike traditional tools, Frensense is **100% Corpus-Driven**. It does not use handwritten YAML rules, regex patterns, or complex DSLs. Instead, it relies on an embedded `.frc` (Frensense Rule Corpus) bundle containing hundreds of real-world vulnerability structures.

### The Two-Phase Architecture

Frensense operates on a strict multi-layered pipeline designed to balance speed and accuracy:

#### Phase 1: The Structural Fast-Pass
The engine uses multi-scale AST n-grams (fingerprints) to quickly scan the codebase. It compares the raw shape of your code against the positive and negative vulnerability pairs in the `.frc` bundle. If the AST matches the shape of a known vulnerability above the `corpus_threshold` (0.15), it passes to Phase 2.

#### Phase 2: Semantic Verification (Composition)
If a node passes Layer 1, it must survive three rigorous semantic verification layers to prove it is an actual bug:
1. **Taint Verification:** The `DataFlowEngine` traces the data paths. It checks if there is a verified path from an untrusted Source to a vulnerable Sink without passing through a registered Sanitizer.
2. **Taint Entropy:** The engine evaluates the complexity and branching of the data flow. If the taint path is too deeply buried in highly conditional loops (high entropy), the confidence score is dynamically scaled down to prevent noisy false positives.
3. **Cross-Function Consistency:** The `ProjectProfile` checks if the vulnerability breaks architectural invariants across multiple files.

---

## Engineering Challenges

Building Frensense required making several hard trade-offs. We believe in being fully transparent about the limitations of static analysis and the problems we faced during development.

### 1. The State Explosion Problem
Tracing inter-procedural taint across massive TypeScript codebases initially caused severe memory exhaustion and CPU lockups. Computing every possible execution path in highly dynamic code is mathematically intractable. As a compromise, we implemented `taint_max_depth` (defaulting to 5 levels) to artificially cap the cross-function trace length. While this keeps performance fast, it means Frensense may miss extremely convoluted, deeply nested vulnerabilities.

### 2. False Positives in Layer 1
The Layer 1 structural fast-pass uses AST Jaccard similarity. However, code that *looks* structurally similar is not always logically identical. Without Layer 2, the engine produced far too many false positives. We had to build the Layer 2 DataFlowEngine to dynamically cull structural matches that lacked a provable, un-sanitized taint path.

### 3. JavaScript's Dynamic Weakness
Because JavaScript and TypeScript are highly dynamic languages, tracing data flow through deep object destructuring, dynamic imports, and `any` types remains a known limitation. When the type constraints are too loose, the DataFlowEngine struggles to mathematically prove the taint path, which can result in false negatives.

---

## What Frensense Is NOT

| Tool | Purpose | Frensense? |
|------|---------|-----------|
| **ESLint / Clippy** | Syntax, formatting, basic errors | ❌ No (use alongside) |
| **TypeScript / Rust compiler** | Type checking | ❌ No (not a type checker) |
| **Rustfmt / Prettier** | Code formatting | ❌ No (not a formatter) |
| **Traditional SAST** | Blind data flow tracing | ❌ No (we use compositional taint) |

**Key Point:** Frensense works *with* your existing toolchain. Run Frensense *after* your linter passes.

---

## Supported Languages

| Language | Status |
| :--- | :--- |
| Rust | Stable |
| TypeScript / JavaScript | Stable |

---

## Installation

### Cargo (CLI)

```bash
cargo install frensense
```
This installs both the `frensense` CLI and the `frensense-mcp` server binary.

### Rust Crate (Library)

```toml
[dependencies]
frensense = "0.5.0"
frensense-engine = "0.5.0"
```

---

## Quick Start

### Audit a Directory

```bash
frensense .
```

### Audit a Single File

```bash
frensense src/main.rs
```

### Filter by Rule Suite

```bash
frensense . --suite default
```
* `default`: Runs only the highest precision rules.
* `extended`: Adds high-confidence rules that might occasionally trigger false positives.
* `all`: Runs the entire corpus.

### Output Formats

```bash
# Human-readable text (default)
frensense .

# JSON (ideal for programmatic consumption)
frensense . --json

# SARIF (standard format for IDE and CI tool integration)
frensense . --sarif
```

---

## Suppression

Frensense supports `.frensense-suppress.yml` at the root of your project to globally ignore specific corpus patterns across files or directories.

```yaml
suppressions:
  - rule_id: CORPUS_TS_UNAUTHENTICATED_DB_WRITE
    path: tests/**
```

---

## Next Steps

- [The Corpus](/corpus) (Understand how Frensense learns from examples)
- [MCP Integration](/mcp) (Connect Frensense to AI Agents)
- [References](/references) (Acknowledgments and CS Foundations)
