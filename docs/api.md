# API Reference

## Node.js Programmatic API

The Node.js API provides a class-based interface to the GenSense engine via a native NAPI-RS binding. No Rust toolchain is required at runtime.

### Installation

```bash
npm install @friehub/gensense
```

---

### Class: `GenSense`

The primary entry point for the diagnostic engine.

#### Constructor

```typescript
new GenSense(options?: GenSenseOptions)
```

**Parameters**

| Option | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `environment` | `'development' \| 'staging' \| 'production'` | `'development'` | Sets the execution context. Some rules are gated by environment. |
| `tags` | `string[]` | `[]` | Activates rule groups. An empty array activates all rules. |

**Example**

```javascript
const { GenSense } = require('@friehub/gensense');

// Activate all rules
const engine = new GenSense();

// Activate only security and reliability rules
const engine = new GenSense({
  environment: 'production',
  tags: ['security', 'reliability']
});
```

---

#### `engine.auditContent(filePath, content)`

Runs a full semantic audit on a string of source code. The `filePath` is used to determine the file type and apply the correct language-specific rules. No file is read from disk.

**Signature**

```typescript
auditContent(filePath: string, content: string): Advisory[]
```

**Parameters**

| Parameter | Type | Description |
| :--- | :--- | :--- |
| `filePath` | `string` | A virtual or real path used to determine the language (e.g., `'app.rs'`, `'server.ts'`). |
| `content` | `string` | The raw source code string to analyze. |

**Returns**: `Advisory[]`

**Example**

```javascript
const fs = require('fs');
const { GenSense } = require('@friehub/gensense');

const engine = new GenSense({ tags: ['security'] });
const source = fs.readFileSync('src/server.ts', 'utf-8');
const findings = engine.auditContent('src/server.ts', source);

for (const f of findings) {
  console.log(`[${f.severity}] ${f.ruleId} at line ${f.line}: ${f.observation}`);
}
```

---

#### `engine.auditPath(targetPath)`

Recursively walks a directory or audits a single file on disk.

**Signature**

```typescript
auditPath(targetPath: string): Advisory[]
```

**Parameters**

| Parameter | Type | Description |
| :--- | :--- | :--- |
| `targetPath` | `string` | Absolute or relative path to a file or directory. |

**Example**

```javascript
const findings = engine.auditPath('./src');

if (findings.length === 0) {
  console.log('No issues found.');
} else {
  process.exit(1); // Fail CI if findings are present
}
```

---

### Type: `Advisory`

The structured finding returned by the engine for each detected issue.

```typescript
interface Advisory {
  ruleId:      string;  // e.g., "RUST_ASYNC_MUTEX_DEADLOCK"
  severity:    string;  // "Warning" | "Critical"
  observation: string;  // What was found
  impact:      string;  // Why it matters
  improvement: string;  // What to do about it
  line:        number;  // 1-indexed line number
  column:      number;  // 1-indexed column number
  filePath:    string;  // The file path passed to auditContent or resolved by auditPath
}
```

---

## Rust Library API

The Rust library exposes the engine directly for use in build tools, custom CLI utilities, or integration with other Rust projects.

### Dependency

```toml
[dependencies]
gensense = "0.1.0"
```

### Usage

```rust
use gensense::Engine;

fn main() {
    let mut engine = Engine::new();

    let source = std::fs::read_to_string("src/main.rs").unwrap();
    let findings = engine.run_content("src/main.rs", &source);

    for finding in &findings {
        eprintln!(
            "[{}] {} at {}:{}",
            finding.severity, finding.rule_id, finding.line, finding.column
        );
        eprintln!("  {}", finding.observation);
    }

    if findings.iter().any(|f| f.severity == "Critical") {
        std::process::exit(1);
    }
}
```

---

## CLI Reference

The CLI is the primary interface for project-level auditing.

```
USAGE:
    gensense <COMMAND> [OPTIONS]

COMMANDS:
    audit <path>        Run a semantic audit on a file or directory
    --generate-docs     Generate a RULES.md catalog from all loaded rules

OPTIONS:
    --tag <tag>         Filter analysis by rule tag (can be specified multiple times)
    --help              Print help information
```

### Exit Codes

| Code | Meaning |
| :--- | :--- |
| `0` | No findings, or only informational output |
| `1` | One or more findings were produced |
