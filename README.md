# GenSense

GenSense is a **high-precision semantic diagnostic engine** designed to detect logical flaws and security risks in codebases, particularly those influenced by AI-generated patterns. It uses Tree-sitter for industrial-grade AST traversal and provides a lightweight, developer-centric way to enforce safety standards.

> [!TIP]
> GenSense acts as a **Semantic Linter**. It complements existing tools like Clippy or ESLint by focusing on architectural risks (e.g., deadlocks, async safety) that traditional syntax-based linters often miss.

---

## 🚀 Quick Start

### 1. Global CLI (via NPM)
You can run GenSense directly on any project using `npx`:

```bash
# Audit the current directory
npx gensense audit .

# Enable specific diagnostic tags (e.g., security, performance)
npx gensense audit . --tag security --tag performance
```

### 2. Programmatic API (Node.js)
GenSense provides a professional, class-based API for integration into your own tools or CI pipelines:

```javascript
const { GenSense } = require('@friehub/gensense');

// 1. Initialize the engine
const engine = new GenSense({
  environment: 'development',
  tags: ['security'] // Enable specific rule groups
});

// 2. Audit a code string (ideal for IDE plugins)
const code = `
fn main() {
    let x: Option<i32> = None;
    x.unwrap(); // This will be flagged for safety
}
`;

const findings = engine.auditContent('app.rs', code);
findings.forEach(a => console.log(`[${a.severity}] ${a.ruleId}: ${a.observation}`));

// 3. Audit a directory on disk
const projectFindings = engine.auditPath('./src');
```

---

## 🛠 Developer-Centric Design

GenSense is built for engineers who value precision and clarity:
*   **Peer-Review Tone**: Advisories are written as actionable engineering feedback, not institutional jargon.
*   **Opt-in Governance**: High-level checks (like SBOM/Governance) are strictly **opt-in**. We don't nag you about metadata unless you ask us to.
*   **Stateful Analysis**: Unlike simple linters, GenSense tracks semantic symbols across your project to identify complex cross-function risks.

---

## ✍️ Extending GenSense: Adding Your Own Rules

Developers can extend GenSense using **Declarative YAML** (simple pattern matching) or **Procedural Rust** (complex semantic analysis).

### Declarative Rules (YAML)
Add a `.yml` file in the `rules/` directory:

```yaml
rules:
  - id: "RUST_DANGEROUS_UNWRAP"
    domain: "reliability"
    target_ext: "rs"
    on_node: "(call_expression) @node"
    if_matches: ".*\\.unwrap\\(\\)"
    observation: "I noticed an '.unwrap()' call that might be risky."
    impact: "Unwrapping can cause a panic if the value is None/Err."
    improvement: "Use 'match' or '?' for safer error handling."
```

---

## 🔍 Suppression & Ignoring

GenSense respects inline comments for fine-grained control:

```rust
// gensense-ignore: RUST_UNWRAP_SAFETY
let config = parse_config().unwrap(); // I know this is safe because...
```

You can also use a `.gensense-suppress.yml` in your project root for path-based exclusions.

---

## 🏗 Development Stack

GenSense enforces high standards on its own codebase:
*   **`npm run build`**: Builds the native NAPI-RS bridge with full language support.
*   **`npm test`**: Runs the cross-language integration suite.
*   **`scripts/local-ci.sh`**: Full stack validation (Format, Lint, Test, Build).

---

## License
Proprietary - Friehub (TaaS Gateway).  
Copyright (c) 2026 Friehub. All rights reserved.
