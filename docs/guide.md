# Getting Started

## What is GenSense

GenSense is an experimental semantic diagnostic engine. It analyzes code at the Abstract Syntax Tree (AST) level to detect logical, security, and architectural patterns that conventional linters cannot identify.

A compiler or type-checker tells you that code is syntactically and type-theoretically valid. GenSense tells you whether it is semantically sound — whether the intent expressed in the code matches what will actually happen at runtime.

### The Problem It Solves

Consider the following Rust snippet:

```rust
async fn handle_request(db: Arc<Mutex<Pool>>) {
    let guard = db.lock().unwrap(); // Mutex acquired
    let result = query_database(&guard).await; // Awaiting while holding lock
}
```

This code compiles cleanly. `rustc`, `clippy`, and `rustfmt` produce no errors. But it contains a potential deadlock: the mutex guard is held across an `.await` point. If another task tries to acquire the same mutex while this one is suspended, the process deadlocks.

GenSense detects this pattern as `RUST_ASYNC_MUTEX_DEADLOCK` and generates a structured advisory explaining the risk and how to correct it.

---

## Supported Languages

| Language | Status | Feature Flag |
| :--- | :--- | :--- |
| Rust | Stable | `rust` |
| TypeScript / JavaScript | Stable | `typescript` / `node` |
| Solidity | Experimental | `solidity` |

---

## Installation

### Global CLI

Install globally to use from any project:

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
gensense = "0.1.0"
```

---

## Quick Start

### Audit a Directory

```bash
gensense audit .
```

### Audit a Single File

```bash
gensense audit src/main.rs
```

### Filter by Tag

Tags allow you to scope analysis to a specific domain. Run only security-related checks:

```bash
gensense audit . --tag security
```

Available tags: `security`, `reliability`, `observability`, `governance`, `performance`.

### Using `npx` (No Installation)

```bash
npx @friehub/gensense audit .
```

---

## Understanding the Output

Each finding from GenSense follows a consistent four-field advisory format:

```
[Warning] RUST_ASYNC_MUTEX_DEADLOCK  at src/server.rs:42
  Observation : Potential deadlock: a lock guard is held across an .await point.
  Impact      : If another task attempts to acquire this lock while this task is suspended,
                the process will deadlock with no error or panic.
  Improvement : Move the lock acquisition to a scope that ends before the .await point,
                or use a tokio::sync::Mutex instead of std::sync::Mutex.
```

This format is designed to be read as a peer-review comment, not an error code. Each field is required for all rules.

---

## Suppression

### Inline (per-line)

```rust
// gensense-ignore: RUST_UNWRAP_SAFETY
let config = load_config().unwrap(); // Guaranteed to succeed at startup
```

### Project-Level (`.gensense-suppress.yml`)

```yaml
suppress:
  - rule: RUST_STD_OUTPUT
    paths:
      - src/bin/
      - tests/
```

---

## Next Steps

- Read the [API Reference](/api) for programmatic usage.
- Browse the [Rule Catalog](/rules) to see all available rules.
- See [Editor Integration](/editor) to set up VS Code integration.
