# Frensense

Frensense (formerly GenSense) is a high-performance semantic diagnostic engine for Rust and TypeScript. It detects logical flaws, security risks, and unoptimized patterns that conventional linters miss — code that compiles and type-checks but still deadlocks, leaks secrets, or contains subtly wrong AI-generated logic.

Full documentation: [https://friehub.github.io/frensense](https://friehub.github.io/frensense)

---

## What Changed from GenSense

Frensense is the evolution of the GenSense engine, developed under Friehub. The core analysis architecture (SPG, CSA, temporal taint tracking, algebraic flow combinators) is unchanged. What changed:

- **Renamed** from `gensense` to `frensense` across all packages, binaries, and config files.
- **Embedded rules are being retired.** The 67 built-in rules previously compiled into the binary are being phased out. Going forward, Frensense ships with **no embedded rules by default**. All rules are supplied externally via `--rules-dir`. This makes the engine leaner, fully auditable, and eliminates the need to cut a new release every time a rule changes.
- **Rule distribution** will move to a dedicated rule catalogue repository, allowing teams to pin specific rule versions independently of the engine version.
- The `--suite` flag (`default|extended|all`) will be deprecated once embedded rules are fully removed.

This is a deliberate architectural choice: the engine handles analysis mechanics, rule files handle policy. They should evolve independently.

---

## v0.3.1 Key Features

- **Semantic Program Graph (SPG)**: Symbol index + cross-file call graph + temporal event chains exposed to every rule, including algebraic flow combinators (`AllOf`, `AnyOf`, `Not`, `Across`, `Without`, `Chain`) for precise cross-cutting queries.
- **Contextual Structural Analysis (CSA)**: 5 CSA rules for Rust and TypeScript that reason about function bodies — `body_must_contain`, `body_may_delegate_via`, inline suppression — to catch validate-without-reject, sanitize-passthrough, and never-empty patterns.
- **Temporal & Taint Analysis**: `RUST_CONNECTION_LEAK`, `RUST_NETWORK_IN_TXN`, `RUST_MUTATE_AFTER_RESPONSE` — must-follow, forbidden-between, and must-not-follow temporal constraints. High-precision intra/interprocedural taint tracking with configurable depth and confidence thresholds.
- **Algebraic Flow Combinators**: Recursive tree-walk evaluator for composing taint, temporal, scope, and cross-file constraints into compound rules — no Datalog or QL needed.
- **Configurable Tuning**: All analysis parameters exposed via CLI (`--taint-max-depth`, `--ngram-window`, `--min-ngram-count`, `--taint-conf-inter`/`--taint-conf-intra`, `--max-source-lines`, `--confidence-boost-*`, `--jaccard-threshold`).
- **External Rules Only (Roadmap)**: Embedded rule suites are being retired. Pass `--rules-dir` to supply your rules. See the roadmap section below.
- **Auto-Remediation**: Experimental `--fix` for rules with YAML-defined `fix_pattern`/`fix_with`.

---

## Why Frensense

Most linters enforce syntax rules and type constraints. Frensense operates one level higher — semantic intent:

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

Frensense ships a **Model Context Protocol (MCP) server** — a JSON-RPC 2.0 interface over stdin/stdout for AI agents.

```bash
cargo build --features mcp
./target/debug/frensense-mcp
```

Configure in your MCP client:

```json
{
  "mcpServers": {
    "frensense": {
      "command": "frensense-mcp",
      "args": []
    }
  }
}
```

### Cargo

```toml
[dependencies]
frensense = "0.3.1"
```

### NPM

```bash
npm install -g @friehub/frensense
```

```bash
npx @friehub/frensense .
```

---

## CLI Usage

```bash
# Scan the current directory (or specify a path)
frensense

# Scan a single file with a language filter
frensense --language rust main.rs

# Exit with code 1 on any finding (CI use)
frensense . --strict

# Filter by severity or confidence
frensense . --severity critical
frensense . --confidence high

# Output as JSON or SARIF
frensense . --json > report.json
frensense . --sarif > report.sarif

# Supply external rules (recommended — embedded rules are being retired)
frensense . --rules-dir .frensense/rules/

# Diff-only mode — scan files changed since last commit
frensense . --diff-only

# Baseline comparison (regression detection)
frensense . --emit-baseline baseline.json
frensense . --compare-baseline baseline.json

# Disable a noisy rule or override its severity
frensense . --disable-rule RUST_STD_OUTPUT
frensense . --override-severity RUST_HOST_INTERACTION:info

# Tuning
frensense . --taint-max-depth 10 --ngram-window 5

# Auto-fix
frensense . --fix
frensense . --diff

# Test a custom rule
frensense test-rule my_rule.yml --fixture test.ts --expect-finding MY_RULE_ID
```

See `frensense --help` for the full flag reference.

---

## Architecture

When Frensense scans a project it runs a multi-pass pipeline before rule execution:

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
mkdir -p .frensense/rules

cat > .frensense/rules/my_rules.yml << 'EOF'
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

frensense test-rule .frensense/rules/my_rules.yml \
  --fixture src/main.rs \
  --expect-finding MYCO_NO_PRINTLN

frensense src/ --rules-dir .frensense/rules/
```

See [docs/extending.md](docs/extending.md) for the full YAML rule reference.

---

## Roadmap: End of Embedded Rules

The 67 embedded rules will be fully retired in `v0.4.0`. The migration path:

| Today (v0.3.x) | v0.4.0+ |
| :--- | :--- |
| `frensense . --suite default` | `frensense . --rules-dir /path/to/frensense-rules/default/` |
| `frensense . --suite extended` | `frensense . --rules-dir /path/to/frensense-rules/extended/` |
| `frensense . --suite all` | `frensense . --rules-dir /path/to/frensense-rules/` |

The official Frensense rule catalogue will be published as a standalone repository and versioned independently. Teams will be able to pin to a specific rule set version without upgrading the engine.

---

## Suppression

### Inline

```rust
// frensense-ignore: RUST_UNWRAP_SAFETY
let config = load_config().unwrap(); // Guaranteed to succeed — pre-validated
```

### Project-level (`.frensense-suppress.yml`)

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
- name: Run Frensense
  run: npx @friehub/frensense . --strict --severity critical --rules-dir .frensense/rules/

# Baseline-based regression check
- name: Baseline comparison
  run: |
    npx @friehub/frensense . --json --rules-dir .frensense/rules/ > current.json
    npx @friehub/frensense . --compare-baseline baseline.json

# Custom rules only
- name: Custom rules
  run: frensense . --rules-dir .frensense/rules/ --strict
```

---

## Development

```bash
cargo build --features cli
cargo test
cargo clippy -- -W clippy::pedantic
./target/debug/frensense .
./target/debug/frensense --debug src/parser.rs
./target/debug/frensense --list-rules
cargo bench --bench engine_perf
```

---

## Contributing

Contributions welcome. All rules must include `id`, `severity`, `observation`, `impact`, and `improvement`. Run `cargo test` and `cargo clippy` before opening a PR. See [docs/extending.md](docs/extending.md).

---

## License

MIT
