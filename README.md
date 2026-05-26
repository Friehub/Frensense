# GenSense

GenSense is a high-performance semantic diagnostic engine for Rust and TypeScript. It detects logical flaws, security risks, and unoptimized patterns that conventional linters miss — code that compiles and type-checks but still deadlocks, leaks secrets, or contains subtly wrong AI-generated logic.

Full documentation: [https://friehub.github.io/gensense](https://friehub.github.io/gensense)

---

## v0.3.1 Key Features

- **Semantic Program Graph (SPG)**: Symbol index + cross-file call graph + temporal event chains exposed to every rule, including algebraic flow combinators (`AllOf`, `AnyOf`, `Not`, `Across`, `Without`, `Chain`) for precise cross-cutting queries.
- **Contextual Structural Analysis (CSA)**: 5 CSA rules for Rust and TypeScript that reason about function bodies — `body_must_contain`, `body_may_delegate_via`, inline suppression — to catch validate-without-reject, sanitize-passthrough, and never-empty patterns.
- **Temporal & Taint Analysis**: `RUST_CONNECTION_LEAK`, `RUST_NETWORK_IN_TXN`, `RUST_MUTATE_AFTER_RESPONSE` — must-follow, forbidden-between, and must-not-follow temporal constraints. High-precision intra/interprocedural taint tracking with configurable depth and confidence thresholds.
- **Algebraic Flow Combinators**: Recursive tree-walk evaluator for composing taint, temporal, scope, and cross-file constraints into compound rules — no Datalog or QL needed.
- **Configurable Tuning**: All analysis parameters exposed via CLI (`--taint-max-depth`, `--ngram-window`, `--min-ngram-count`, `--taint-conf-inter`/`--taint-conf-intra`, `--max-source-lines`, `--confidence-boost-*`, `--jaccard-threshold`).
- **Rule Suites**: `--suite default|extended|all` — precision-tiered rule selection from 67 built-in rules.
- **Auto-Remediation**: Experimental `--fix` for rules with YAML-defined `fix_pattern`/`fix_with`.

---

## Why GenSense

Most linters enforce syntax rules and type constraints. GenSense operates one level higher — semantic intent:

- An async block acquires a `std::sync::Mutex` guard and `await`s — a deadlock.
- A `todo!()` or `unimplemented!()` on a reachable production path.
- A secret, API key, or environment URL committed to the repo.
- AI-generated code with an assertion that is always true, a test that tests nothing, or an error branch that silently returns a default.
- A database query fetching all columns when only one is needed.
- A `validate()` function that always returns `true` (no rejection path).
- A `sanitize()` function that passes input through unchanged.
- A connection acquired but never released, or network I/O inside a transaction.
- A public tRPC mutation without auth check, or a Prisma query with `select *`.

None of these are caught by `rustfmt`, `clippy`, `eslint`, or a type system.

---

## Supported Languages

| Language | Status |
| :--- | :--- |
| Rust | Stable |
| TypeScript / JavaScript | Stable |
| YAML | Stable (rule files) |

---

## Installation

### MCP Server (AI Agent Integration)

GenSense ships a **Model Context Protocol (MCP) server** — a JSON-RPC 2.0 interface over stdin/stdout for AI agents.

```bash
cargo build --features mcp
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

### Cargo

```toml
[dependencies]
gensense = "0.3.1"
```

### NPM

```bash
npm install -g @friehub/gensense
```

```bash
npx @friehub/gensense .
```

---

## CLI Usage

```bash
# Scan the current directory (or specify a path)
gensense

# Scan a single file with a language filter
gensense --language rust main.rs

# Exit with code 1 on any finding (CI use)
gensense . --strict

# Filter by severity or confidence
gensense . --severity critical
gensense . --confidence high

# Output as JSON or SARIF
gensense . --json > report.json
gensense . --sarif > report.sarif

# Use extended rule suite and custom rules
gensense . --suite extended --rules-dir .gensense/rules/

# Diff-only mode — scan files changed since last commit
gensense . --diff-only

# Baseline comparison (regression detection)
gensense . --emit-baseline baseline.json
gensense . --compare-baseline baseline.json

# Disable a noisy rule or override its severity
gensense . --disable-rule RUST_STD_OUTPUT
gensense . --override-severity RUST_HOST_INTERACTION:info

# Tuning
gensense . --taint-max-depth 10 --ngram-window 5

# Auto-fix
gensense . --fix
gensense . --diff

# Test a custom rule
gensense test-rule my_rule.yml --fixture test.ts --expect-finding MY_RULE_ID
```

See `gensense --help` for the full flag reference.

---

## Architecture

When GenSense scans a project it runs a multi-pass pipeline before rule execution:

1. **Symbol Discovery** — extracts all named functions, variables, types, and constants.
2. **Call Edge Discovery** — maps function call relationships into the `SemanticGraph`.
3. **Event Discovery** — builds temporal event chains (acquire, await, release, return) inside each function scope.
4. **SPG Assembly** — the graph is exposed to every rule via `AuditOptions.graph`, enabling cross-cutting queries.
5. **Rule Execution** — all rules run against each file. Each receives the AST node and the full SPG context.

After execution, taint flows are materialized as `TaintFlow` edges in the graph, and algebraic combinators evaluate compound constraints via recursive tree-walk.

---

## Custom Rules

Writing a new rule requires no Rust knowledge and no recompile. YAML rules support:

- **Pattern matching**: `on_node`, `if_matches`, `if_not_matches`, regex patterns on AST node text.
- **File-level CSA**: `body_must_contain` (function body must contain pattern), `body_may_delegate_via` (accept delegation as suppression).
- **Flow constraints**: `TaintReached`, `TaintForbidden`, `ScopeConstraint`, `Temporal`, and algebraic combinators (`all_of`, `any_of`, `not`, `across_boundary`, `without`, `chain`).
- **Project rules**: `MustHaveGuard`, `MustBeInternal`, `CrossFileTaintFree`, `SchemaContract`.

```bash
mkdir -p .gensense/rules

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

gensense test-rule .gensense/rules/my_rules.yml \
  --fixture src/main.rs \
  --expect-finding MYCO_NO_PRINTLN

gensense src/ --rules-dir .gensense/rules/
```

See [docs/extending.md](docs/extending.md) for the full YAML rule reference.

---

## Suppression

### Inline

```rust
// gensense-ignore: RUST_UNWRAP_SAFETY
let config = load_config().unwrap(); // Guaranteed to succeed — pre-validated
```

### Project-level (`.gensense-suppress.yml`)

```yaml
suppressions:
  - rule_id: RUST_STD_OUTPUT
    path: src/bin/**
  - rule_id: GLOBAL_TODO_PLACEHOLDER
    path: docs/**
```

---

## CI Integration

```yaml
# .github/workflows/audit.yml
- name: Run GenSense
  run: npx @friehub/gensense . --strict --severity critical

# Baseline-based regression check
- name: Baseline comparison
  run: |
    npx @friehub/gensense . --json --suite extended > current.json
    npx @friehub/gensense . --compare-baseline baseline.json

# Custom rules only
- name: Custom rules
  run: gensense . --rules-dir .gensense/rules/ --no-builtin-rules --strict
```

---

## Development

```bash
cargo build --features cli
cargo test
cargo clippy -- -W clippy::pedantic
./target/debug/gensense .
./target/debug/gensense --debug src/parser.rs
./target/debug/gensense --list-rules
cargo bench --bench engine_perf
```

---

## Contributing

Contributions welcome. All rules must include `id`, `severity`, `observation`, `impact`, and `improvement`. Run `cargo test` and `cargo clippy` before opening a PR. See [docs/extending.md](docs/extending.md).

---

## License

MIT
