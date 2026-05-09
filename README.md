# TaaS Static Auditor 🛡️

**TaaS Static Auditor** is a high-precision, production-grade semantic analysis engine designed for auditing protocol-level logic, security vulnerabilities, and institutional redline enforcement. Built in Rust for maximum performance and memory safety, it serves as the primary diagnostic layer for the Friehub TaaS Gateway ecosystem.

## 🚀 Core Philosophy

Unlike generic linters, the TaaS Auditor focuses on **Semantic Integrity**. It doesn't just look for syntax patterns; it understands the flow of data, the cost of operations, and the architectural risks inherent in decentralized and high-concurrency systems.

## ✨ Key Capabilities

### 1. Semantic Taint Tracking
The engine implements sophisticated data-flow analysis for TypeScript and Rust:
*   **Object Destructuring**: Correctly tracks tainted data through complex patterns like `const { sensitive: data } = req.body;`.
*   **Intermediate Bindings**: Follows variables across multiple local reassignments and scope boundaries.
*   **Function Aliasing**: Detects security risks even when core functions are aliased (e.g., `const execute = eval; execute(code);`).

### 2. Structural N-Gram Fingerprinting
Identify redundant boilerplate and institutional knowledge gaps using N-gram based structural comparison:
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
*   **Solidity**: Protocol-level checks for Reentrancy (Checks-Effects-Interactions) and security redlines.

---

## 🛡️ Institutional Integrity Stack

The auditor includes a comprehensive developer toolset to ensure the diagnostic engine itself remains reliable:

| Tool | Feature | Purpose |
| :--- | :--- | :--- |
| **Pre-commit Hooks** | `make setup` | Enforces zero-warning clippy and rustfmt before every commit. |
| **Snapshot Verification** | `make test` | Detects "diagnostic noise" by comparing findings against a verified baseline. |
| **Parser Fuzzing** | `make fuzz` | Uses `cargo-fuzz` to feed adversarial input into the engine to ensure stability. |
| **Integrity Makefile** | `make all` | A single entry point for semantic, security, and quality checks. |

---

## 🛠️ Installation & Setup

### Prerequisites
*   Rust (Latest Stable)
*   `libfuzzer` (for fuzzing support)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/Friehub/auditor.git
cd auditor

# Install the Institutional Integrity Stack (Hooks)
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

## 📝 Rule Definition

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

## 📈 Roadmap & Evolution

*   **Multi-File Taint Propagation**: Extending semantic analysis across module boundaries.
*   **AI-Assisted Remediation**: Automated generation of patch-sets for identified vulnerabilities.
*   **WASM Target**: Running the auditor directly in-browser for Zero-Knowledge auditing interfaces.

---

## ⚖️ License & Intellectual Property

Proprietary - Friehub (TaaS Gateway).  
Copyright (c) 2026 Friehub. All rights reserved.

Designed and engineered for the Friehub TaaS Gateway ecosystem to ensure the safety and integrity of decentralized protocols.
