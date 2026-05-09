# TaaS Static Auditor

TaaS Static Auditor is a high-precision, production-grade semantic analysis engine designed for auditing protocol-level logic, security vulnerabilities, and standardized safety enforcement. Built in Rust for maximum performance and memory safety, it serves as the primary diagnostic layer for the Friehub TaaS Gateway ecosystem.

## Rationale: Why TaaS Auditor Exists

Modern decentralized systems face a unique set of challenges that generic static analysis tools are not equipped to handle. While traditional linters focus on stylistic consistency and local syntax errors, the TaaS Auditor was engineered to address:

1.  **Protocol-Level Semantic Risk**: Detecting logical flaws in cross-language environments where data from a web frontend (TypeScript) directly impacts high-stakes protocol logic (Rust/Solidity).
2.  **Safety Standard Enforcement**: Providing a mechanism to enforce strict protocol safety standards (e.g., "no uninstrumented async calls") that are too specialized for general-purpose tools.
3.  **Complexity at Scale**: Handling the semantic complexity of modern codebases, such as complex object destructuring and intermediate variable tainting, which often baffle standard AST-based search tools.

## Comparative Analysis

| Feature | TaaS Auditor | Standard Linters (ESLint/Clippy) | Security Scanners (Slither) |
| :--- | :--- | :--- | :--- |
| **Scope** | Cross-Language Semantic Flow | Single-Language Syntax/Style | Domain-Specific (Smart Contracts) |
| **Taint Tracking** | Inter-procedural and Destructure-aware | Limited/Non-existent | Robust but Language-Locked |
| **Safety Enforcement** | High (Custom YAML/Rust Rules) | Medium (Complex Plugin Setup) | Medium (Built-in Rules) |
| **Boilerplate Detection** | Structural N-Gram Analysis | None | Basic Hash-based |
| **Env Isolation** | Native Beta/Stable Filtering | None | None |

The TaaS Auditor is designed to complement, not replace, single-language linters. It acts as a specialized security and quality layer that sits above the standard toolchain.

## Key Capabilities

### 1. Semantic Taint Tracking
The engine implements sophisticated data-flow analysis for TypeScript and Rust:
*   **Object Destructuring**: Correctly tracks tainted data through complex patterns like `const { sensitive: data } = req.body;`.
*   **Intermediate Bindings**: Follows variables across multiple local reassignments and scope boundaries.
*   **Function Aliasing**: Detects security risks even when core functions are aliased (e.g., `const execute = eval; execute(code);`).

### 2. Structural N-Gram Fingerprinting
Identify redundant boilerplate and safety gaps using N-gram based structural comparison:
*   **Boilerplate Detection**: Automatically identifies large blocks of duplicated logic across the codebase.
*   **Anonymous Resolution**: Intelligently resolves names for anonymous arrow functions and class methods to make reports actionable.
*   **Redundancy Scoring**: Quantifies code duplication to guide refactoring and reduce technical debt.

### 3. Environment-Aware Rule Isolation
Deploy and test new audit rules safely using the built-in isolation system:
*   **Beta Tagging**: Tag experimental rules as `beta` to keep them isolated from Production environments.
*   **Staging Rollouts**: Run the full suite in Staging/Dev while maintaining a strictly stable baseline for Production CI/CD gates.

### 4. Cross-Language Parity
*   **TypeScript/TSX**: Unified handling of modern web logic with deep JSX/TSX support.
*   **Rust**: Specialized rules for `tokio` async safety, mutex deadlock prevention, and tracing instrumentation.
*   **Solidity**: Protocol-level checks for Reentrancy (Checks-Effects-Interactions) and security standards.

---

## Developer Integrity Suite

The auditor includes a comprehensive developer toolset to ensure the diagnostic engine itself remains reliable:

| Tool | Feature | Purpose |
| :--- | :--- | :--- |
| **Pre-commit Hooks** | `make setup` | Enforces zero-warning clippy and rustfmt before every commit. |
| **Snapshot Verification** | `make test` | Detects "diagnostic noise" by comparing findings against a verified baseline. |
| **Parser Fuzzing** | `make fuzz` | Uses `cargo-fuzz` to feed adversarial input into the engine to ensure stability. |
| **Integrity Makefile** | `make all` | A single entry point for semantic, security, and quality checks. |

---

## Installation and Usage

### Prerequisites
*   Rust (Latest Stable)
*   `libfuzzer` (for fuzzing support)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/Friehub/auditor.git
cd auditor

# Install the Developer Integrity Suite (Hooks)
make setup

# Run a semantic audit on a target directory
cargo run -- /path/to/project
```

### Configuration
The auditor can be configured via `.taas-suppress.yml` in your project root to handle intentional deviations:
```yaml
suppressions:
  - rule_id: "RUST_CLONE_IN_LOOP"
    reason: "Intentional clone for thread-local storage in legacy module."
    path: "src/legacy/mod.rs"
```

## Rule Definition

### YAML Rules (Declarative)
Create rules in `rules/*.yml` for quick pattern matching:
```yaml
id: "TS_DATA_LEAK"
observation: "Sensitive data from request body leaked to console."
severity: "Critical"
category: "Security"
tags: ["stable"]
if_matches: "req\\.body"
must_not_contain: "console\\.log"
```

### Rust Rules (Procedural)
For complex semantic checks, implement the `AuditorRule` trait:
```rust
impl AuditorRule for SecretGuard {
    fn check(&self, node: Node, context: &AuditContext) -> Vec<Advisory> {
        // High-precision procedural logic here
    }
}
```

---

## Roadmap and Evolution

*   **Multi-File Taint Propagation**: Extending semantic analysis across module boundaries.
*   **AI-Assisted Remediation**: Automated generation of patch-sets for identified vulnerabilities.
*   **WASM Target**: Running the auditor directly in-browser for Zero-Knowledge auditing interfaces.

---

## License and Intellectual Property

Proprietary - Friehub (TaaS Gateway).  
Copyright (c) 2026 Friehub. All rights reserved.

Designed and engineered for the Friehub TaaS Gateway ecosystem to ensure the safety and integrity of decentralized protocols.

