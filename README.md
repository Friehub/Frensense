# GenSense

GenSense is an experimental semantic diagnostic engine that analyzes code at the AST level to detect logical flaws, security risks, and architectural patterns that conventional linters miss.

It is designed for teams who want a second pair of eyes on code that is structurally valid but semantically problematic: unchecked error propagation, async safety violations, hardcoded secrets, AI-generated placeholder logic, and more.

Full documentation: [https://friehub.github.io/gensense](https://friehub.github.io/gensense)

---

## Why GenSense

Most linters operate on syntax trees and enforce stylistic or type-level constraints. GenSense operates on semantic patterns:

- A function compiles successfully but panics at runtime if a specific branch is reached.
- An async block acquires a mutex lock and then awaits — a deadlock waiting to happen.
- A developer left `todo!()` in a path that is reachable in production.
- A hardcoded secret or URL was committed to the repository.
- AI-generated code added an assertion that is always true.

None of these are caught by `rustfmt`, `clippy`, `eslint`, or a type system. GenSense is built to catch exactly these classes of problems.

---

## Supported Languages

| Language | Status |
| :--- | :--- |
| Rust | Stable |
| TypeScript / JavaScript | Stable |
| Solidity | Experimental |

---

## Installation

### CLI (via NPM)

```bash
npm install -g @friehub/gensense
```

Or use without installing:

```bash
npx @friehub/gensense audit .
```

### Rust Library (via Cargo)

Add to your `Cargo.toml`:

```toml
[dependencies]
gensense = "0.1.0"
```

### Node.js Programmatic API

```bash
npm install @friehub/gensense
```

---

## Usage

### CLI

```bash
# Audit the current directory
gensense audit .

# Audit a specific file
gensense audit src/main.rs

# Filter by tag
gensense audit . --tag security
gensense audit . --tag reliability
gensense audit . --tag observability

# Generate a full rule catalog
gensense --generate-docs
```

### Node.js API

```javascript
const { GenSense } = require('@friehub/gensense');

const engine = new GenSense({
  environment: 'development',
  tags: ['security', 'reliability']
});

// Audit a string of code (ideal for editor extensions or CI pipelines)
const findings = engine.auditContent('src/handler.rs', sourceCode);

findings.forEach(finding => {
  console.log(`[${finding.severity}] ${finding.ruleId} at line ${finding.line}`);
  console.log(`  Observation : ${finding.observation}`);
  console.log(`  Impact      : ${finding.impact}`);
  console.log(`  Improvement : ${finding.improvement}`);
});

// Audit an entire directory
const projectFindings = engine.auditPath('./src');
```

### Rust Library API

```rust
use gensense::Engine;

fn main() {
    let mut engine = Engine::new();
    let findings = engine.run_content("handler.rs", source_code);

    for finding in &findings {
        println!("[{}] {} at line {}", finding.severity, finding.rule_id, finding.line);
    }
}
```

---

## Advisory Format

Every finding returned by GenSense follows a consistent structure:

| Field | Type | Description |
| :--- | :--- | :--- |
| `ruleId` | `string` | Unique identifier for the rule that triggered |
| `severity` | `string` | `Warning` or `Critical` |
| `observation` | `string` | What was detected |
| `impact` | `string` | Why it matters |
| `improvement` | `string` | Recommended corrective action |
| `line` | `number` | Line number of the finding |
| `column` | `number` | Column number of the finding |
| `filePath` | `string` | File path that was analyzed |

---

## Suppression

### Inline Suppression

Add an inline comment directly above the flagged line:

```rust
// gensense-ignore: RUST_UNWRAP_SAFETY
let config = load_config().unwrap();
```

### File-Level Suppression

Create a `.gensense-suppress.yml` file in your project root:

```yaml
suppress:
  - rule: RUST_STD_OUTPUT
    paths:
      - src/bin/
      - tests/
  - rule: GLOBAL_TODO_PLACEHOLDER
    paths:
      - docs/
```

---

## Editor Integration

### VS Code

GenSense does not yet ship an official VS Code extension, but you can integrate it into your existing editor workflow today:

**Option 1: Task Runner Integration**

Add a task to `.vscode/tasks.json` to run GenSense on save or on demand:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "GenSense: Audit Project",
      "type": "shell",
      "command": "npx @friehub/gensense audit ${workspaceFolder}",
      "group": "build",
      "presentation": {
        "reveal": "always",
        "panel": "shared"
      },
      "problemMatcher": []
    }
  ]
}
```

You can then run it via `Ctrl+Shift+B` or the Command Palette.

**Option 2: On-Save Integration (via Run on Save extension)**

Install the [Run on Save](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave) extension and add to your `settings.json`:

```json
{
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": "\\.(rs|ts|js|sol)$",
        "cmd": "npx @friehub/gensense audit ${file}"
      }
    ]
  }
}
```

**Option 3: Pre-commit Hook**

Install [husky](https://github.com/typicode/husky) and add a pre-commit hook:

```bash
npx husky add .husky/pre-commit "npx @friehub/gensense audit ."
```

### CI / GitHub Actions

```yaml
- name: Run GenSense Audit
  run: npx @friehub/gensense audit . --tag security
```

---

## Extending GenSense: Custom Rules

### Declarative Rules (YAML)

Add a `.yml` file inside the `rules/` directory of the engine:

```yaml
rules:
  - id: "RUST_DANGEROUS_UNWRAP"
    domain: "reliability"
    target_ext: "rs"
    on_node: "(call_expression) @node"
    if_matches: ".*\\.unwrap\\(\\)"
    observation: "An '.unwrap()' call was detected without a documented safety justification."
    impact: "If the value is None or Err, this will panic the process at runtime."
    improvement: "Use 'match', '?', or 'unwrap_or_else' for safe error handling. If unwrap is intentional, add a '// SAFETY:' comment."
```

### Procedural Rules (Rust)

For complex multi-node semantic analysis, implement the `Rule` trait in `src/rules/`:

```rust
pub struct MyCustomRule;

impl Rule for MyCustomRule {
    fn id(&self) -> &'static str { "MY_CUSTOM_RULE" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, node: &Node, source: &str, ctx: &Context) -> Option<Advisory> {
        // Implement custom AST traversal logic here
        None
    }
}
```

---

## Development

```bash
# Build the native Node.js addon
npm run build

# Build in debug mode (faster)
npm run build:debug

# Run integration tests
npm test

# Run Rust unit tests
cargo test --features cli

# Serve the documentation locally
npm run docs:dev

# Generate the full rule catalog markdown
cargo run --features cli -- --generate-docs
```

---

## Contributing

Contributions are welcome. Please follow the existing code style enforced by `rustfmt` and `clippy`. All rules must include an `id`, `severity`, `observation`, `impact`, and `improvement` field to maintain a consistent advisory format.

---

## License

MIT License
