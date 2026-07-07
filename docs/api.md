# API Reference

## Node.js Programmatic API

The Node.js API provides a class-based interface to the Frensense engine via a native NAPI-RS binding. No Rust toolchain is required at runtime.

### Installation

```bash
npm install @friehub/frensense
```

---

### Class: `Frensense`

The primary entry point for the diagnostic engine.

#### Constructor

```typescript
new Frensense(options?: FrensenseOptions)
```

**Parameters**

| Option | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `environment` | `'development' \| 'staging' \| 'production'` | `'development'` | Sets the execution context. Rules tagged `beta` are suppressed in production. |
| `tags` | `string[]` | `[]` | Activates optional rule groups. An empty array activates all rules. |

**Example**

```javascript
const { Frensense } = require('@friehub/frensense');

// All rules active
const engine = new Frensense();

// Security and reliability rules only, in production mode
const engine = new Frensense({
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
const { Frensense } = require('@friehub/frensense');

const engine = new Frensense({ tags: ['security'] });
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
  impact:      string;   // Why it matters (concrete technical consequence)
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
frensense = "0.5.0"
frensense-engine = "0.5.0"
```

### Single File Audit

```rust
use frensense::{Engine, FrensenseAuditor};
use std::path::Path;

fn main() -> frensense::Result<()> {
    let auditor = FrensenseAuditor::default_auditor();
    let engine = Engine::new(auditor);

    let source = std::fs::read_to_string("src/main.rs")?;
    let advisories = engine.run_content(Path::new("src/main.rs"), &source)?;

    for adv in &advisories {
        eprintln!(
            "[{}] {} at {}:{}",
            adv.rule_id, adv.observation, adv.line, adv.column
        );
    }

    if advisories.iter().any(|a| a.severity == frensense::Severity::Critical) {
        std::process::exit(1);
    }

    Ok(())
}
```

### Project-Wide Audit

```rust
use frensense::{Engine, FrensenseAuditor};
use std::path::Path;

fn main() -> frensense::Result<()> {
    let auditor = FrensenseAuditor::default_auditor();
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
frensense <path> [options]
frensense <corpus_path> --build-bundle [--build-bundle-output <out.frc>]
frensense --learn <positive_file> <negative_file> --learn-output <bundle.frc>
frensense --version
```

### Analysis Options

| Flag | Description |
| :--- | :--- |
| `--suite <name>` | Defines the rule suite to run: `default` (near-zero FP), `extended`, or `all`. |
| `--severity <level>` | Filter findings by severity: `critical`, `warning`, or `info`. |
| `--tag <name>` | Enable an optional diagnostic tag (can be specified multiple times). |
| `--language <ext>` | Filter analysis to a specific file extension (e.g., `ts`, `rs`). |
| `--strict` | Exit with code 1 if any findings match the active filter. |
| `--json` | Output findings as a JSON array. |
| `--sarif` | Output findings in SARIF v2.1.0 format. |
| `--fix [<scope>]` | Apply automated remediations where available (`style`, `security`, `all`). |
| `--diff [<scope>]` | Preview proposed fixes as a unified diff. |
| `--diff-only` | Only output the diff, silencing all other stdout (useful for CI formatters). |
| `--build-bundle` | Compiles a custom `.frc` bundle from the specified corpus directory path. |
| `--build-bundle-output <path>` | Path to save the compiled bundle (default: `frensense-corpus.frc`). |

### Confidence & Taint Thresholds

| Flag | Description |
| :--- | :--- |
| `--confidence <tier>` | Set minimum confidence: `high` (0.85), `medium` (0.60), `low` (0.30), `any` (0.0). |
| `--min-confidence <float>` | Set a raw float confidence threshold (e.g., `0.75`). |
| `--threshold <float>` | Layer 1 structural match threshold (default: `0.15`). |
| `--jaccard-threshold <float>` | Set the AST Jaccard similarity threshold for fingerprinting. |
| `--taint-max-depth <int>` | Maximum depth for cross-function taint tracing (default: `5`). |

### Overrides & Baselines

| Flag | Description |
| :--- | :--- |
| `--disable-rule <id>` | Ignore all findings for a specific rule ID. |
| `--override-severity <ID>:<level>` | Change a rule's severity dynamically (e.g. `CORPUS_TS_XSS:critical`). |
| `--emit-baseline <path>` | Save all current findings to a baseline file to ignore them in the future. |
| `--compare-baseline <path>` | Suppress any findings that exist in the specified baseline file. |

---

## Live Learning (`--learn`)

Frensense does not require you to recompile the engine to teach it a new vulnerability pattern. You can dynamically learn a new bug directly from the command line using the `--learn` flag.

### How it Works

Simply provide a code snippet containing the vulnerability (Positive), and a snippet containing the safe/remediated code (Negative). Frensense will calculate the structural AST fingerprints and generate a custom `.frc` bundle on the fly.

```bash
# Teach Frensense a new structural bug
frensense --learn ./buggy_auth.ts ./safe_auth.ts --learn-output custom_corpus.frc

# Scan your project using your custom learned corpus
frensense ./src --corpus ./custom_corpus.frc
```

This allows security engineers to write a bug reproduction and instantly scale its detection across thousands of repositories without ever writing a YAML rule.

### Exit Codes

| Code | Meaning |
| :--- | :--- |
| `0` | Success (no findings, or findings present but `--strict` not set) |
| `1` | Findings produced and `--strict` is set, or an error occurred |
