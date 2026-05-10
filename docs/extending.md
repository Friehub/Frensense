# Writing Custom Rules

GenSense is designed so that any developer can add new rules without modifying the engine or recompiling from source. Rules are declarative YAML files that the engine discovers and loads at runtime.

---

## How Rule Loading Works

GenSense uses a two-tier loading system:

| Tier | Source | Who controls it |
| :--- | :--- | :--- |
| **Embedded rules** | Baked into the binary at compile time | GenSense core team |
| **User rules** | Read from disk at runtime, no recompile needed | You |

At startup, the engine reads user rules from:

1. **`.gensense/rules/`** in the project root — rules committed alongside your code
2. **`~/.gensense/rules/`** — global rules that apply to every project on your machine
3. **`--rules-dir <path>`** — any additional path you pass on the command line

All rule sets are merged into one pipeline. If a user rule has the same `id` as an embedded rule, the user rule wins (override semantics).

---

## Writing a YAML Rule

Create a `.yml` file in `.gensense/rules/` with this structure:

```yaml
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
```

You can put multiple rules in one file. The filename does not matter — the engine discovers all `.yml` files recursively.

---

## Field Reference

| Field | Required | Description |
| :--- | :--- | :--- |
| `id` | Yes | Unique identifier in `SCREAMING_SNAKE_CASE`. Must be unique across all rules. |
| `domain` | Yes | Category: `security`, `reliability`, `performance`, `maintainability`, `quality` |
| `target_ext` | Yes | File extension to target: `rs`, `ts`, `tsx`, `js`, `jsx`, `sol`, or `*` for all |
| `on_node` | Yes | The tree-sitter node kind to match. The rule fires once per matching node. |
| `if_matches` | No | Regex. Rule fires only if the node's full text matches this pattern. |
| `must_contain` | No | Regex. Rule fires if this pattern is **NOT** found in the node (inverted check). |
| `must_not_contain` | No | Regex. Rule fires if this pattern **IS** found in the node. |
| `max_lines` | No | Rule fires if the node spans more than this many lines. |
| `max_depth` | No | Rule fires if the nesting depth inside the node exceeds this number. |
| `within_scope` | No | Only fire if inside a parent node of this kind (e.g. `function_item`). |
| `source_pattern` | No | For taint rules: regex matching the taint source variable name. |
| `sink_pattern` | No | For taint rules: regex matching the sink call. Both required together. |
| `severity` | No | `Critical`, `Warning`, or `Info`. Defaults to `Warning`. |
| `observation` | Yes | What was found — factual, first-person, specific to this instance. |
| `impact` | Yes | The concrete technical consequence if this is not addressed. |
| `improvement` | Yes | A specific, actionable suggestion. |
| `temporal` | No | Temporal ordering block. See [Temporal Rules](#temporal-rules) below. |

---

## Finding the Right `on_node` Value

Run `gensense --debug <file>` to dump the full tree-sitter AST of any file. Find the node kind that wraps the pattern you want to detect.

```bash
gensense --debug src/main.rs
```

Common node kinds:

| Pattern | `on_node` |
| :--- | :--- |
| Any function | `function_item` (Rust), `function_declaration` (TS) |
| Any function call | `call_expression` |
| Macro call | `macro_invocation` |
| Variable declaration | `let_declaration` (Rust), `lexical_declaration` (TS) |
| Match expression | `match_expression` |
| If expression | `if_expression` |
| For loop | `for_expression` (Rust), `for_statement` (TS) |
| Arrow function | `arrow_function` |
| Import statement | `import_statement` |
| Test function | `function_item` with `if_matches: "#\\[test\\]"` |

For alternating between multiple node types, use tree-sitter bracket syntax:

```yaml
on_node: "[ (function_declaration) (arrow_function) (method_definition) ] @node"
```

---

## Testing Your Rule

Before deploying a rule to your project, verify it with the `test-rule` command:

```bash
gensense test-rule .gensense/rules/my_rules.yml \
  --fixture tests/samples/bad_code.rs \
  --expect-finding MYCO_NO_PRINTLN \
  --expect-line 5
```

This command:
1. Loads **only** the rules from the specified YAML file
2. Runs them against only the fixture file
3. Checks that the expected rule fires at the expected line
4. Prints `[PASS]` or `[FAIL: <reason>]`
5. Exits with code 0 for pass, 1 for fail

You should always write two fixture files:

| File | Purpose |
| :--- | :--- |
| `bad_code.rs` | Code the rule must fire on |
| `good_code.rs` | Correct version — rule must NOT fire |

Run the test against both:

```bash
gensense test-rule .gensense/rules/my_rules.yml --fixture bad_code.rs --expect-finding MYCO_NO_PRINTLN
gensense test-rule .gensense/rules/my_rules.yml --fixture good_code.rs  # expects no findings
```

---

## Temporal Rules

A temporal rule checks the **order** of events inside a function. For example: "a `.lock()` must never be followed by an `.await`."

```yaml
rules:
  - id: "MYCO_LOCK_BEFORE_SEND"
    domain: "reliability"
    target_ext: "rs"
    on_node: "function_item"
    observation: "A mutex guard is held across a channel send."
    impact: "If the receiver blocks, this creates a deadlock."
    improvement: "Drop the guard before sending."
    severity: Critical
    temporal:
      sequence: ["lock", "send"]
      behavior: must_not_follow
```

### `behavior` options

| Value | Meaning |
| :--- | :--- |
| `must_follow` | All events in `sequence` must appear in that order. Missing any step is a violation. |
| `must_not_follow` | The second event must never appear after the first in the same function. |

### How sequence matching works

Each item in `sequence` is a regex matched against the event label. Event labels are the base names of calls. For example:
- `mutex.lock()` → label `lock`
- `.await` expression → label `.await`
- `tokio::spawn(...)` → label `spawn`

---

## Taint Rules

A taint rule tracks data flow from a source to a sink across variable assignments.

```yaml
rules:
  - id: "MYCO_SECRET_TO_LOG"
    domain: "security"
    target_ext: "ts"
    on_node: "[ (function_declaration) (arrow_function) ] @node"
    observation: "A variable named 'password' or 'secret' flows into a logging sink."
    impact: "Sensitive credentials are being written to logs."
    improvement: "Redact or mask the value before logging."
    severity: Critical
    source_pattern: "password|secret|token|api_key"
    sink_pattern: "console\\.log|logger\\."
```

Both `source_pattern` and `sink_pattern` must be specified together. The engine traces variable assignments between the source variable name and the sink call within the same function scope.

---

## Examples

### Flag all uses of `todo!()` and `unimplemented!()` in production code

```yaml
rules:
  - id: "MYCO_NO_PLACEHOLDER_PANICS"
    domain: "reliability"
    target_ext: "rs"
    on_node: "macro_invocation"
    if_matches: "^(todo|unimplemented)!"
    observation: "A placeholder panic was detected in production code."
    impact: "This path will panic at runtime if reached."
    improvement: "Implement the missing logic or return a proper error."
    severity: Critical
```

### Flag imports from a deprecated internal module

```yaml
rules:
  - id: "MYCO_DEPRECATED_IMPORT"
    domain: "maintainability"
    target_ext: "ts"
    on_node: "(import_statement) @node"
    if_matches: "from ['\"]@internal/legacy"
    observation: "An import from the deprecated '@internal/legacy' module was detected."
    impact: "This module is scheduled for removal."
    improvement: "Migrate to '@internal/core'. See CHANGELOG.md."
    severity: Warning
```

### Enforce a size limit on functions

```yaml
rules:
  - id: "MYCO_MAX_FUNCTION_SIZE"
    domain: "quality"
    target_ext: "rs"
    on_node: "(function_item) @node"
    max_lines: 50
    observation: "Function exceeds the 50-line limit set by this project's standards."
    impact: "Large functions are difficult to test, review, and maintain."
    improvement: "Extract inner logic into smaller helper functions."
    severity: Warning
```

---

## Advisory Content Guidelines

All rules must produce advisories that follow these guidelines regardless of whether they are YAML or procedural:

- **Observation**: State what was found as a fact. First-person is acceptable. Be specific to the instance, not generic.
- **Impact**: Explain the concrete, technical consequence. What will deadlock, panic, leak, or fail?
- **Improvement**: Give a specific, actionable suggestion. Name the alternative API, pattern, or approach.

Avoid marketing language, vague terms like "bad code", and superlatives. The advisory is a peer-review comment, not a warning banner.

---

## Project Configuration

Create `.gensense/config.yml` in your project root to configure engine behavior without CLI flags:

```yaml
version: 1
rules_dir: .gensense/rules/

# Disable specific embedded rules for this project
disabled_rules:
  - RUST_STD_OUTPUT
  - GLOBAL_TODO_PLACEHOLDER

# Override the severity of specific rules
severity_override:
  RUST_UNWRAP_SAFETY: Info
```

---

## Using Only Custom Rules (No Embedded Rules)

To run exclusively your own rules and suppress all embedded defaults:

```bash
gensense . --rules-dir .gensense/rules/ --no-builtin-rules
```

This is useful for organizations that want full control over which rules are active and prefer to curate their own ruleset from scratch.
