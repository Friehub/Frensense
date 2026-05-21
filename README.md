# GenSense

GenSense is a fast, modular semantic diagnostic engine. It uses **Contextual Structural Analysis (CSA)** and **Symbol-Relative Identity (SRI)** to detect logical flaws, security risks, and unoptimized patterns that conventional linters miss.

It operates on **semantic intent** — not just syntax. Code that compiles cleanly can still deadlock, leak secrets, or contain unoptimized database queries. GenSense catches those classes of problems and can **automatically remediate** them.

Full documentation: [https://friehub.github.io/gensense](https://friehub.github.io/gensense)

---

## v0.3.0 Key Features

- **Symbol-Relative Identity (SRI)**: Findings are anchored to logical symbols (functions, classes) instead of line numbers, making CI baselines immune to refactoring.
- **Contextual Structural Analysis (CSA)**: Rules can now reason across multiple files to verify that sensitive operations are protected by appropriate guards.
- **Auto-Remediation Engine**: Many rules now support automated fixes via the `--fix` flag.
- **High-Precision Taint Analysis**: Track data flow from sensitive sources to unsafe sinks across function bodies.

---

## Why GenSense

Most linters enforce syntax rules and type constraints. GenSense operates one level higher:

- An async block acquires a `std::sync::Mutex` guard and then awaits — a deadlock waiting to happen.
- A `todo!()` or `unimplemented!()` call sits on a code path reachable in production.
- A hardcoded secret, API key, or environment URL was committed to the repository.
- AI-generated code added an assertion that is always true, a test that tests nothing, or an error branch that silently returns a default value.
- A Prisma query fetches all fields when only one is needed.

None of these are caught by `rustfmt`, `clippy`, `eslint`, or a type system. GenSense is built for exactly these patterns.

---

## Supported Languages

| Language | Status |
| :--- | :--- |
| Rust | Stable |
| TypeScript / JavaScript | Stable |
| YAML | Stable (rule files) |
| Solidity | Beta (enable with `--features solidity`) |

---

## Installation

### MCP Server (AI Agent Integration)

GenSense ships a **Model Context Protocol (MCP) server** — a JSON-RPC 2.0 interface over stdin/stdout that lets AI agents (Claude Code, Cursor, etc.) audit code as part of their workflow.

```bash
# Build the MCP server
cargo build --features mcp

# Run it (stdin/stdout JSON-RPC)
./target/debug/gensense-mcp
```

Configure in your MCP client:

```json
{
  "mcpServers": {
    "gensense": {
      "command": "gensense-mcp",
      "args": []
    }
  }
}
```

The server exposes a single tool `gensense_audit` with `path`, `fix_auto`, and `severity_threshold` parameters. See [MCP Server docs](https://friehub.github.io/gensense/mcp) for the full reference.

---

### CLI (via NPM)

```bash
npm install -g @friehub/gensense
```

Or use without installing:

```bash
npx @friehub/gensense .
```

### Rust (via Cargo)

```toml
[dependencies]
gensense = "0.3.0"
```

### Node.js Programmatic API

```bash
npm install @friehub/gensense
```

---

## Usage

### CLI

```bash
# Audit a directory
gensense <path>

# Audit a single file
gensense src/main.rs

# Filter by severity
gensense . --severity critical

# Enable optional diagnostic tags
gensense . --tag security
gensense . --tag governance
gensense . --tag sbom

# Output as JSON or SARIF
gensense . --json
gensense . --sarif

# Exit with code 1 if any findings match the filter (CI mode)
gensense . --strict

# Print the active rule catalog
gensense . --list-rules

# Generate RULES.md documentation
gensense . --generate-docs

# Dump the AST of a file (for writing rules)
gensense --debug src/main.rs

# Apply automated fixes where available
gensense . --fix

# Preview proposed fixes as a unified diff
gensense . --diff

# Load additional custom YAML rules from a directory
gensense . --rules-dir .gensense/rules/

# Test a single YAML rule against a fixture file
gensense test-rule .gensense/rules/my_rule.yml \
  --fixture tests/samples/bad_code.rs \
  --expect-finding MY_RULE_ID \
  --expect-line 14
```

### Node.js API

```javascript
const { GenSense } = require('@friehub/gensense');

const engine = new GenSense({
  environment: 'development',
  tags: ['security', 'reliability']
});

// Audit a string of source code
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
use gensense::{Engine, GenSenseAuditor};
use std::path::Path;

fn main() -> gensense::Result<()> {
    let auditor = GenSenseAuditor::default_auditor();
    let mut engine = Engine::new(auditor);

    let advisories = engine.run(Path::new("./src"))?;

    for adv in &advisories {
        println!("[{}] {} at {}:{}", adv.severity, adv.rule_id, adv.file_path, adv.line);
        println!("  {}", adv.observation);
    }

    Ok(())
}
```

---

## Advisory Format

Every finding follows a consistent structure:

| Field | Type | Description |
| :--- | :--- | :--- |
| `rule_id` | `string` | Unique identifier for the rule that triggered |
| `severity` | `string` | `Critical`, `Warning`, or `Info` |
| `observation` | `string` | What was detected in this specific instance |
| `impact` | `string` | Why it matters — concrete technical consequence |
| `improvement` | `string` | Recommended corrective action |
| `line` | `number` | Line number of the finding (1-indexed) |
| `column` | `number` | Column number of the finding (1-indexed) |
| `file_path` | `string` | Full path to the file that was analyzed |

---

## Suppression

### Inline Suppression

Add an inline comment directly above the flagged line:

```rust
// gensense-ignore: RUST_UNWRAP_SAFETY
let config = load_config().unwrap(); // Guaranteed to succeed — config is pre-validated
```

### File-Level Suppression

Create a `.gensense-suppress.yml` file in your project root:

```yaml
suppressions:
  - rule_id: RUST_STD_OUTPUT
    path: src/bin/**
  - rule_id: GLOBAL_TODO_PLACEHOLDER
    path: docs/**
```

---

## Custom Rules

GenSense is designed so that writing a new rule requires no Rust knowledge and no recompile.

### Quick Start

```bash
# Create the rules directory in your project
mkdir -p .gensense/rules

# Write a rule
cat > .gensense/rules/my_rules.yml << 'EOF'
rules:
  - id: "MYCO_NO_PRINTLN"
    domain: "maintainability"
    target_ext: "rs"
    on_node: "macro_invocation"
    if_matches: "println!"
    observation: "Direct println! usage detected."
    impact: "All output must route through the company logger."
    improvement: "Replace with log::info!() or tracing::info!()."
    severity: Warning
EOF

# Test the rule against a fixture before deploying it
gensense test-rule .gensense/rules/my_rules.yml \
  --fixture src/main.rs \
  --expect-finding MYCO_NO_PRINTLN

# Run with all rules merged (embedded + custom)
gensense src/
```

See [docs/extending.md](docs/extending.md) for the full YAML rule reference, temporal rules, and advanced patterns.

---

## Semantic Discovery

When GenSense scans a project it runs a four-pass pipeline before rule execution:

1. **Symbol Discovery** — extracts all named functions, variables, types, and constants into a `SymbolRegistry`.
2. **Call Edge Discovery** — maps function call relationships into a `SemanticGraph`.
3. **Event Discovery** — builds a temporal event chain (lock, await, return, assignment) inside each function scope.
4. **Rule Execution** — runs all rules in parallel (via Rayon). Each rule receives the AST node and the full semantic context.

The `[INFO] Semantic Discovery: Indexed N symbols` line you see at runtime is output from this phase.

---

## Suppression

### Inline
```rust
// gensense-ignore: RULE_ID
```

### Project-level (`.gensense-suppress.yml`)
```yaml
suppressions:
  - rule_id: RUST_STD_OUTPUT
    path: src/bin/**
```

---

## CI Integration

```yaml
# .github/workflows/audit.yml
- name: Run GenSense
  run: npx @friehub/gensense . --strict --severity critical
```

To gate on custom rules only:
```yaml
- name: Run custom rules
  run: gensense . --rules-dir .gensense/rules/ --no-builtin-rules --strict
```

---

## Development

```bash
# Build the CLI binary
cargo build --features cli

# Build the native Node.js addon
npm run build

# Run all tests
cargo test

# Run with verbose output on the GenSense codebase itself
./target/debug/gensense .

# Dump AST for a file (use this to write rules)
./target/debug/gensense --debug src/parser.rs

# Generate the full rule catalog
./target/debug/gensense . --generate-docs
```

---

## Contributing

Contributions are welcome. All rules must include `id`, `severity`, `observation`, `impact`, and `improvement`. Follow the advisory content guidelines in [docs/extending.md](docs/extending.md). Run `cargo test` and `cargo clippy` before opening a pull request.

---

## License

MIT
