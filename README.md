# GenSense

GenSense is a **semantic diagnostic engine** designed to detect logical flaws and security risks in codebases, particularly those heavily influenced by AI-generated patterns. It uses Tree-sitter for high-precision AST traversal and provides a lightweight, language-agnostic way to enforce safety standards.

> [!NOTE]
> GenSense is a diagnostic assistant. It complements existing linters (like Clippy or ESLint) by focusing on cross-language semantic patterns and "logical sense-checks" that traditional tools often miss.

---

## 🛠 Usage

### 1. CLI (Rust)
Run the engine directly against a local directory:
```bash
# Standard analysis
cargo run -- /path/to/your/code

# Enable optional diagnostic tags (e.g., SBOM, Governance)
cargo run -- /path/to/your/code --tag sbom
```

### 2. Node.js (Native Addon)
GenSense is available as a high-performance native bridge for Node.js.
```javascript
const { Engine, GenSenseAuditor } = require('@friehub/gensense');

const engine = new Engine(GenSenseAuditor.default());
engine.enableTag('sbom'); // Opt-in to specific audits
const advisories = engine.run('/path/to/project');
```

---

## 🛠 Developer-Centric Diagnostics

GenSense is built for developers, not for auditors. We prioritize **technical impact** and **engineering clarity**:
*   **No Institutional Jargon**: Advisories focus on technical debt, maintenance overhead, and security risks.
*   **Optional Audits**: High-level governance checks (like SBOM verification) are **opt-in**. We don't nag you about project management unless you ask us to.
*   **Actionable Feedback**: Every suggestion is designed to be a peer-review comment you'd actually want to receive.

---

## ✍️ Extending GenSense: Adding Your Own Rules

Developers can extend GenSense using two mechanisms: **Declarative YAML** (simple pattern matching) and **Procedural Rust** (complex semantic analysis).

### 1. Declarative Rules (YAML)
The easiest way to add a rule is via a `.yml` file in the `rules/` directory.

**Format (`rules/custom_rules.yml`):**
```yaml
rules:
  - id: "MY_CUSTOM_RULE"
    domain: "security"
    target_ext: "rs"
    on_node: "(call_expression) @node" # Tree-sitter query
    if_matches: "dangerous_func\\("     # Regex for the node content
    observation: "A dangerous function was detected."
    impact: "This could lead to unauthorized access."
    improvement: "Use 'safe_func' instead."
    severity: "warning"
```

*   **`on_node`**: Uses Tree-sitter query syntax. **Must include a named capture** (e.g., `@node`).
*   **`if_matches`**: (Optional) A regex filter applied to the text of the matched node.
*   **`must_contain` / `must_not_contain`**: (Optional) Additional regex constraints for the node's scope.

### 2. Procedural Rules (Rust)
For rules requiring deep AST traversal or stateful analysis, implement the `GenSenseRule` trait in Rust.

```rust
pub struct MyComplexRule;

impl GenSenseRule for MyComplexRule {
    fn id(&self) -> &str { "MY_COMPLEX_RULE" }
    
    fn query(&self) -> Option<&str> {
        Some("(function_item) @func")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        // Implement your custom logic here using the tree-sitter node
        advisories
    }
}
```

---

## 🔍 Using Existing Rules

GenSense comes with a robust library of built-in rules for **Rust**, **TypeScript**, and **Solidity**.

### Listing Rules
To see all currently loaded rules and their documentation:
```bash
cargo run -- --generate-docs
```
This generates a `RULES.md` file containing the full catalog of observations, impacts, and remediation steps.

### Suppression
If a rule triggers a false positive or an intentional deviation, suppress it via `.gensense-suppress.yml` in your project root:
```yaml
suppressions:
  - rule_id: "RUST_ASYNC_MUTEX_DEADLOCK"
    path: "src/legacy_wrapper.rs"
    reason: "Internal wrapper ensures lock is dropped before await."
```

---

## 🏗 Developer Integrity Stack

GenSense enforces high standards on its own codebase to ensure diagnostic reliability:
*   **`make fmt`**: Enforces strict style guidelines.
*   **`make check`**: Runs semantic lints via Clippy.
*   **`make test`**: Executes the full regression suite (Rust + Node.js bindings).
*   **`make audit`**: Scans dependencies for security vulnerabilities.

---

## License
Proprietary - Friehub (TaaS Gateway).  
Copyright (c) 2026 Friehub. All rights reserved.
