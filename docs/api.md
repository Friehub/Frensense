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
| `environment` | `'development' \| 'staging' \| 'production'` | `'development'` | Sets the execution context. Rules tagged `beta` are suppressed in production. |
| `tags` | `string[]` | `[]` | Activates optional rule groups. An empty array activates all rules. |

**Example**

```javascript
const { GenSense } = require('@friehub/gensense');

// All rules active
const engine = new GenSense();

// Security and reliability rules only, in production mode
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

Recursively walks a directory or audits a single file on disk. Respects the same directory exclusion rules as the CLI (`target/`, `.git/`, `node_modules/` are skipped automatically).

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
  const critical = findings.filter(f => f.severity === 'Critical');
  if (critical.length > 0) {
    process.exit(1); // Fail CI on critical findings
  }
}
```

---

### Type: `Advisory`

The structured finding returned by the engine for each detected issue.

```typescript
interface Advisory {
  ruleId:      string;   // e.g. "RUST_ASYNC_MUTEX_DEADLOCK"
  severity:    string;   // "Critical" | "Warning" | "Info"
  observation: string;   // What was found (specific to this instance)
  impact:      string;   // Why it matters — concrete technical consequence
  improvement: string;   // What to do about it
  line:        number;   // 1-indexed line number
  column:      number;   // 1-indexed column number
  filePath:    string;   // The resolved file path
}
```

---

## Rust Library API

The Rust library exposes the engine directly for use in build tools, custom CLI utilities, or integration with other Rust projects.

### Dependency

```toml
[dependencies]
gensense = "0.1.7"
```

### Single File Audit

```rust
use gensense::{Engine, GenSenseAuditor};
use std::path::Path;

fn main() -> gensense::Result<()> {
    let auditor = GenSenseAuditor::default_auditor();
    let engine = Engine::new(auditor);

    let source = std::fs::read_to_string("src/main.rs")?;
    let advisories = engine.run_content(Path::new("src/main.rs"), &source)?;

    for adv in &advisories {
        eprintln!(
            "[{}] {} at {}:{}",
            adv.rule_id, adv.observation, adv.line, adv.column
        );
    }

    if advisories.iter().any(|a| a.severity == gensense::Severity::Critical) {
        std::process::exit(1);
    }

    Ok(())
}
```

### Project-Wide Audit

```rust
use gensense::{Engine, GenSenseAuditor};
use std::path::Path;

fn main() -> gensense::Result<()> {
    let auditor = GenSenseAuditor::default_auditor();
    let mut engine = Engine::new(auditor);

    // Activate optional tags
    engine.enable_tag("security");
    engine.enable_tag("governance");

    let advisories = engine.run(Path::new("./src"))?;

    println!("Total findings: {}", advisories.len());
    Ok(())
}
```

---

## CLI Reference

### Synopsis

```
gensense <path> [options]
gensense test-rule <rule.yml> [options]
gensense --list-rules
gensense --debug <file>
gensense --generate-docs
gensense --version
```

### Analysis Options

| Flag | Description |
| :--- | :--- |
| `--severity <level>` | Filter findings by severity: `critical`, `warning`, or `info` |
| `--tag <name>` | Enable an optional diagnostic tag (can be specified multiple times) |
| `--strict` | Exit with code 1 if any findings match the active filter |
| `--json` | Output findings as a JSON array |
| `--sarif` | Output findings in SARIF v2.1.0 format |
| `--fix` | Apply automated remediations where available |
| `--diff` | Preview proposed fixes as a unified diff |

### Custom Rule Options

| Flag | Description |
| :--- | :--- |
| `--rules-dir <path>` | Load additional YAML rules from the specified directory |
| `--no-builtin-rules` | Disable all embedded rules (run only user-supplied rules) |

### Utility Commands

| Command | Description |
| :--- | :--- |
| `--list-rules` | Print all active rules in a formatted catalog |
| `--generate-docs` | Write a `RULES.md` catalog file to the current directory |
| `--debug <file>` | Print the tree-sitter AST of a file (useful when writing rules) |
| `--version` | Print the engine version and enabled feature flags |

### Rule Testing

```bash
gensense test-rule <rule.yml> \
  --fixture <file> \
  --expect-finding <RULE_ID> \
  --expect-line <N>
```

| Flag | Description |
| :--- | :--- |
| `--fixture <file>` | The source file to run the rule against |
| `--expect-finding <id>` | Assert that a finding with this rule ID is produced |
| `--expect-line <n>` | Assert that the finding appears at this line number |

Exits 0 on pass, 1 on fail. Use in CI to verify rules before deploying them.

### Exit Codes

| Code | Meaning |
| :--- | :--- |
| `0` | Success — no findings, or findings present but `--strict` not set |
| `1` | Findings produced and `--strict` is set, or `test-rule` failed |
