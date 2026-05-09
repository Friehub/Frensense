# Extending GenSense

GenSense supports two methods of adding rules: declarative YAML for pattern-matching, and procedural Rust for complex semantic analysis.

---

## Declarative Rules (YAML)

Declarative rules are the fastest way to add new checks. They are defined in `.yml` files inside the `rules/` directory. The engine loads them at startup alongside the built-in procedural rules.

### Rule Schema

```yaml
rules:
  - id: "RULE_ID"
    domain: "reliability"
    target_ext: "rs"
    on_node: "(call_expression) @node"
    if_matches: "pattern_regex"
    observation: "What was found."
    impact: "Why it matters."
    improvement: "What to do about it."
```

| Field | Required | Description |
| :--- | :--- | :--- |
| `id` | Yes | Unique rule identifier. Use `UPPERCASE_SNAKE_CASE`. |
| `domain` | Yes | Logical grouping tag (e.g., `security`, `reliability`, `observability`). |
| `target_ext` | Yes | File extension this rule applies to (`rs`, `ts`, `js`, `sol`). |
| `on_node` | Yes | A Tree-sitter query pattern to select the AST node to inspect. |
| `if_matches` | Yes | A regular expression matched against the text content of the selected node. |
| `observation` | Yes | A factual, first-person description of what was found. No marketing language. |
| `impact` | Yes | An explanation of what can go wrong if this is not addressed. |
| `improvement` | Yes | A concrete, actionable suggestion. |

### Example: Custom Rust Rule

Flag any usage of `std::mem::forget`, which can cause memory leaks:

```yaml
rules:
  - id: "RUST_MEM_FORGET"
    domain: "reliability"
    target_ext: "rs"
    on_node: "(call_expression) @node"
    if_matches: "std::mem::forget"
    observation: "A call to 'std::mem::forget' was detected."
    impact: "This bypasses the Drop destructor, potentially causing resource leaks (file handles, sockets, allocations)."
    improvement: "Ensure that forgetting the value is intentional. Consider using ManuallyDrop if ownership transfer to FFI is the goal."
```

### Example: Custom TypeScript Rule

Flag any import from a deprecated internal module:

```yaml
rules:
  - id: "TS_DEPRECATED_IMPORT"
    domain: "reliability"
    target_ext: "ts"
    on_node: "(import_statement) @node"
    if_matches: "from ['\"]@internal/legacy"
    observation: "An import from the deprecated '@internal/legacy' module was detected."
    impact: "This module is scheduled for removal. Code depending on it will break in a future release."
    improvement: "Migrate to the '@internal/core' module. See the migration guide in CHANGELOG.md."
```

---

## Procedural Rules (Rust)

Procedural rules are implemented directly in Rust and have full access to the Tree-sitter AST. Use this approach when the pattern is too complex for a regex or when the rule requires cross-node semantic analysis.

### File Location

Add a new file to `src/rules/`:

```
src/
  rules/
    mod.rs
    my_custom_rule.rs   <-- Add here
```

### Implementing the Rule Trait

```rust
use crate::rules::{Advisory, Rule, Severity};
use tree_sitter::Node;

pub struct MyCustomRule;

impl Rule for MyCustomRule {
    fn id(&self) -> &'static str {
        "MY_CUSTOM_RULE"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn domain(&self) -> &'static str {
        "reliability"
    }

    fn check(
        &self,
        node: &Node,
        source: &str,
        file_path: &str,
    ) -> Option<Advisory> {
        // Only apply to Rust files
        if !file_path.ends_with(".rs") {
            return None;
        }

        // Access the raw text of the node
        let node_text = &source[node.byte_range()];

        if node_text.contains("my_dangerous_pattern") {
            return Some(Advisory {
                rule_id: self.id().to_string(),
                severity: "Warning".to_string(),
                observation: "A dangerous pattern was detected.".to_string(),
                impact: "This can cause X at runtime.".to_string(),
                improvement: "Replace with Y instead.".to_string(),
                line: node.start_position().row + 1,
                column: node.start_position().column + 1,
                file_path: file_path.to_string(),
            });
        }

        None
    }
}
```

### Registering the Rule

In `src/rules/mod.rs`, add your rule to the registry:

```rust
mod my_custom_rule;
pub use my_custom_rule::MyCustomRule;

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // ... existing rules
        Box::new(MyCustomRule),
    ]
}
```

---

## Advisory Content Guidelines

All rules, whether YAML or Rust, must produce advisories that follow these guidelines:

- **Observation**: State what was found as a fact. First-person is acceptable (`"I noticed..."`). Avoid vague terms like "bad" or "wrong".
- **Impact**: Explain the concrete, technical consequence. What will break, leak, deadlock, or panic?
- **Improvement**: Give a specific, actionable suggestion. Name the alternative API, pattern, or approach.

Avoid marketing language, filler words, and superlatives. The advisory is a peer-review comment, not a warning message.
