# Getting Started

## What is GenSense

GenSense is a semantic diagnostic engine. It analyzes source code at the Abstract Syntax Tree (AST) level to detect logical, security, and architectural patterns that conventional linters cannot identify.

### Core Concepts

- **Symbol-Relative Identity (SRI)**: GenSense anchors findings to the enclosing symbol (function or class) rather than a line number. This means your CI baseline stays green even if you move code around or refactor files.
- **Contextual Structural Analysis (CSA)**: GenSense can reason about the "guards" surrounding an operation across multiple files, ensuring that sensitive logic (like a database delete) is always protected by validation code.

### The Problem It Solves

Consider the following Rust snippet:

```rust
async fn handle_request(db: Arc<Mutex<Pool>>) {
    let guard = db.lock().unwrap(); // Mutex acquired
    let result = query_database(&guard).await; // Awaiting while holding lock
}
```

This code compiles cleanly. `rustc`, `clippy`, and `rustfmt` produce no warnings. But it contains a potential deadlock: the mutex guard is held across an `.await` point. If another task tries to acquire the same mutex while this one is suspended, the process deadlocks with no panic or error message.

GenSense detects this as `RUST_ASYNC_MUTEX_DEADLOCK` and produces a structured advisory explaining the risk and how to fix it.

---

## What GenSense Is NOT

GenSense is a *semantic* analyzer, not a replacement for other tools. Understand what it doesn't do:

| Tool | Purpose | GenSense? |
|------|---------|-----------|
| **ESLint / Clippy** | Syntax, formatting, basic errors | ❌ No — use alongside |
| **TypeScript / Rust compiler** | Type checking | ❌ No — not a type checker |
| **Rustfmt / Prettier** | Code formatting | ❌ No — not a formatter |
| **Performance profiler** | Runtime metrics | ❌ No — not a profiler |
| **Secret Scanner** | Finding hardcoded secrets | ✅ Yes — built-in rule |
| **Linter for logic errors** | Runtime correctness | ✅ Yes — primary purpose |
| **AI code pattern detector** | Catching AI-generated bugs | ✅ Yes — specialized rules |

**Key Point:** GenSense works *with* your existing toolchain, not instead of it. Run GenSense *after* your linter passes.

### Complementary Tool Integration

**Typical workflow:**
```bash
# 1. Check formatting and basic lint
npm run lint       # ESLint / Clippy

# 2. Check types
npm run typecheck  # TypeScript / rustc

# 3. Check runtime logic
npm run audit      # GenSense (catches what #1 and #2 miss)

# 4. Deploy
npm run build && deploy
```

All three steps are necessary.

---

## When to Use GenSense

### Use GenSense If You:

✅ **Write async/concurrent code** (Rust, TypeScript)
- GenSense specializes in concurrency patterns (deadlocks, race conditions, missing timeouts)
- If your codebase has async/await, tokio, or promises, GenSense catches bugs other tools miss

✅ **Use AI coding assistants** (Copilot, Claude, ChatGPT)
- AI often generates placeholder panics (`todo!()`), dead code, and tautological logic
- GenSense catches these patterns before they reach production

✅ **Need security audit or compliance checks**
- GenSense detects hardcoded secrets, unsafe patterns, and architectural violations
- Generates SARIF reports for integration with security dashboards

✅ **Define architectural standards for your team**
- Write custom YAML rules (no recompilation) to enforce team patterns
- Examples: "Prisma queries must use `select()`", "All API calls must have timeouts"

✅ **Gate deployments on code quality** (CI/CD)
- Use `--strict` mode to fail builds on critical findings
- Integrates with GitHub Actions in one line

### Skip GenSense If You:

❌ **Only write synchronous code**
- GenSense's concurrency analysis won't help; you'd only get basic findings
- Still run your standard linter instead

❌ **Have strict type checking elsewhere**
- If TypeScript or Rust compiler catches your issues, GenSense won't add value
- Though it still catches runtime logic bugs, so optional

❌ **Need a code formatter** (Rustfmt, Prettier)
- GenSense is analysis-only, not formatting
- Use Rustfmt/Prettier for formatting

❌ **Don't have async code, security concerns, or AI-assisted development**
- GenSense's value drops in fully synchronous, low-risk codebases
- Standard linting is probably sufficient

---

## Supported Languages

| Language | Status | Feature Flag |
| :--- | :--- | :--- |
| Rust | Stable | `rust` |
| TypeScript / JavaScript | Stable | `typescript` |
| YAML | Stable | built-in |


---

## Installation

### Global CLI (via NPM)

```bash
npm install -g @friehub/gensense
```

### Local Project Dependency

```bash
npm install --save-dev @friehub/gensense
```

### Rust Crate

```toml
[dependencies]
gensense = "0.3.0"
```

---

## Quick Start

### Audit a Directory

```bash
gensense .
```

### Audit a Single File

```bash
gensense src/main.rs
```

### Filter by Severity

```bash
gensense . --severity critical
```

### Enable a Diagnostic Tag

Tags activate optional rule groups. Available tags: `security`, `reliability`, `observability`, `governance`, `sbom`.

```bash
gensense . --tag security
```

### View the Active Rule Catalog

```bash
gensense . --list-rules
```

### Use Without Installing

```bash
npx @friehub/gensense .
```

---

## Understanding the Output

Each finding follows a consistent four-field advisory format:

```
[WARNING] RUST_ASYNC_MUTEX_DEADLOCK: Potential deadlock: async lock guard held across .await point. (src/server.rs:42:5)
   - Impact: Holding a std::sync::Mutex guard across an .await point blocks the async executor thread.
   - Suggestion: Drop the guard before the .await, or use tokio::sync::Mutex instead.
```

This format is designed to read as a peer-review comment, not an error code. Every field (`observation`, `impact`, `improvement`) is required for all rules.

---

## Suppression

### Inline (per-line)

```rust
// gensense-ignore: RUST_UNWRAP_SAFETY
let config = load_config().unwrap(); // Guaranteed: config is pre-validated at startup
```

### Project-Level (`.gensense-suppress.yml`)

```yaml
suppressions:
  - rule_id: RUST_STD_OUTPUT
    path: src/bin/**
  - rule_id: GLOBAL_TODO_PLACEHOLDER
    path: docs/**
```

---

## Output Formats

```bash
# Human-readable text (default)
gensense .

# JSON — ideal for programmatic consumption
gensense . --json

# SARIF — standard format for IDE and CI tool integration
gensense . --sarif
```

---

## CI Mode

Use `--strict` to exit with code 1 if any findings are produced. Combine with `--severity` to gate only on critical issues:

```bash
gensense . --strict --severity critical
```

Full GitHub Actions example:

```yaml
- name: Run GenSense Audit
  run: npx @friehub/gensense . --strict --severity critical
```

---

## Integration Examples

### Pre-Commit Hook

Add to `.git/hooks/pre-commit` (or use [pre-commit.com](https://pre-commit.com)):

```bash
#!/bin/bash
gensense --strict --severity critical
if [ $? -ne 0 ]; then
  echo "GenSense audit failed. Fix issues or use 'gensense-ignore' comments."
  exit 1
fi
```

### Monorepo Setup

Audit multiple packages with shared rules:

```bash
gensense packages/*/src --rules-dir .gensense/rules/
```

### VS Code Integration

Add to `.vscode/tasks.json`:

```json
{
  "label": "GenSense Audit",
  "type": "shell",
  "command": "npx @friehub/gensense ${workspaceFolder}",
  "group": "build",
  "presentation": { "reveal": "always", "panel": "shared" }
}
```

Run via **Terminal > Run Task** or `Ctrl+Shift+B`.

---

## Automated Fixes

Some rules include a `proposed_replacement`. Use `--fix` to apply them automatically, or `--diff` to preview the changes:

```bash
# Preview proposed changes
gensense . --diff

# Apply changes in place
gensense . --fix
```

---

## Next Steps

- [Write Custom Rules](/extending) — add project-specific checks without recompiling
- [API Reference](/api) — programmatic usage from Node.js or Rust
- [Rule Catalog](/rules) — browse all available rules
