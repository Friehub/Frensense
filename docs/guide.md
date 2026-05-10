# Getting Started

## What is GenSense

GenSense is a semantic diagnostic engine. It analyzes source code at the Abstract Syntax Tree (AST) level to detect logical, security, and architectural patterns that conventional linters cannot identify.

A compiler or type-checker tells you that code is syntactically and type-theoretically valid. GenSense tells you whether it is semantically sound — whether the intent expressed in the code matches what will actually happen at runtime.

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

## Supported Languages

| Language | Status | Feature Flag |
| :--- | :--- | :--- |
| Rust | Stable | `rust` |
| TypeScript / JavaScript | Stable | `typescript` |
| YAML | Stable | built-in |
| Solidity | Disabled | `solidity` (version mismatch) |

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
gensense = "0.1.7"
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
