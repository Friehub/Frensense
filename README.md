<div align="center">
  <h1>Frensense</h1>
  <p><strong>A deterministic, corpus-driven security and diagnostic engine for Rust, TypeScript, and JavaScript.</strong></p>
</div>

<br />

Frensense detects semantic bugs, architectural violations, and AI hallucinations—code that compiles but doesn't do what it says it does. It operates without brittle YAML rules, regex patterns, or handwritten DSLs.

```bash
cargo install frensense
frensense . --corpus corpus/targets/
```

## How It Works

Starting in `v0.5.0`, Frensense completely abolished manual rule writing. All detection is driven by the **Frensense Rule Corpus (.frc)**. 

The engine fingerprints every function in your project, scores it against the pre-compiled `.frc` bundle, and emits findings when multiple layers confirm the violation:

1. **Corpus Match (Structural)** — Your function's AST shape mathematically matches a known violation pattern in the corpus.
2. **Taint Path (DataFlow)** — Tainted data dynamically flows from a source to a vulnerable sink without sanitization.
3. **Cross-Function Consistency** — Ensures sibling functions do not diverge on the same pattern.

A finding only fires when the structural match and dataflow composition agree, guaranteeing a near-zero false positive rate.

## What It Catches

Frensense actively encodes and enforces three distinct categories of code patterns:

- **Security Vulnerabilities:** SQL Injection, SSRF, Path Traversal, and Credentials flowing to logs/HTTP.
- **Architectural Invariants:** `validate_*()` functions with no rejection path, missing payment gates, or hollow validators that pass input through unchanged.
- **LLM Hallucinations:** Hardcoded tokens, AI-generated `any` parameters, `console.log` in production, and `await` in synchronous blocks.

## Performance & Exclusions

Frensense is highly optimized for large codebases. To maintain sub-second scan times, the engine automatically ignores:
- **Build directories:** `node_modules`, `target`, `dist`, `build`, `vendor`, `out`, and hidden directories (`.*`).
- **Test files:** Files matching `*.test.*`, `*.spec.*`, `__tests__`, or `mocks`.
- **Generated bundles:** Files matching `*.min.js`, `*.bundle.js`, or `*.chunk.js`.
- **Large files:** Any source file larger than 1MB is skipped.

## Quick Start

```bash
# Basic scan
frensense .

# With corpus pattern detection
frensense . --corpus corpus/targets/ --threshold 0.65

# Only critical findings
frensense . --severity critical --strict

# JSON output
frensense . --json

# SARIF for GitHub Advanced Security
frensense . --sarif

# Diff-only (changed files since last commit)
frensense . --diff-only --strict

# Baseline suppression
frensense . --baseline baseline.json

# List loaded patterns
frensense --list-patterns --corpus corpus/targets/
```

## Adding a Detection: Pure Code, Zero Config

To teach Frensense a novel vulnerability or business logic flaw specific to your architecture, you simply drop two code snippets into the `corpus/targets/` directory.

```bash
cp my_bug.ts    corpus/targets/ts_my_bug_positive.ts
cp fixed.ts     corpus/targets/ts_my_bug_negative.ts
```

**You do not write TOML or YAML.** Instead, you provide the advisory text directly inside the `_positive` source code file using a `[frensense]` comment block:

```typescript
// [frensense]
// observation: Writing user-provided data directly to a datastore...
// impact: Any user can overwrite or corrupt data...
// improvement: Call a central auth resolver...

export async function handleDataSync(request: Request, db: Database) {
  // Bad code here...
}
```

Run the builder to compile your new custom `.frc` bundle:
```bash
frensense corpus/targets/ --build-bundle
```
Frensense parses your comment block straight from the AST and bakes it into the `.frc` bundle.

## AI Agent Integration (MCP)

Frensense ships with native support for the **Model Context Protocol (MCP)**, allowing AI agents (like Claude or Antigravity) to interact with the engine.

```bash
# Start the MCP server
frensense-mcp
```
Agents can dynamically query the workspace, request taint path resolutions, and validate their own generated code against the corpus before committing changes.

## Citation

The Frensense detection corpus is built from real-world vulnerability data:

> Moonen, L., Vidziunas, L., & Bhandari, G. P. (2024). *CVEfixes: Automated Collection of Vulnerabilities and Their Fixes from Open-Source Software* (v1.0.8). 17th International Conference on Predictive Models and Data Analytics in Software Engineering (PROMISE), Athens, Greece. Zenodo. https://doi.org/10.5281/zenodo.13138703

> Semgrep, Inc. (2024). *Semgrep Rules Repository*. GitHub. https://github.com/semgrep/semgrep-rules

## License

MIT
