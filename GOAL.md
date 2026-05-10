# GenSense: Complete Agent Brief

**Version:** 1.0  
**Project:** GenSense v0.1.7 → v1.0  
**Who this is for:** Every AI agent working on GenSense. Read this entire document before touching any file.

---

## 1. What GenSense Is

GenSense is a static analysis tool written in Rust. It reads source code files (Rust, TypeScript, Solidity) and finds bugs, bad patterns, and dangerous code. It produces a list of "advisories" — each one describing what was found, why it matters, and how to fix it.

It is NOT a compiler. It does NOT run your code. It reads it and reasons about it.

**The tool runs as:**
- A CLI binary: `gensense <path>` — scans a folder and prints findings
- A library crate: other Rust programs can import it
- A Node.js addon (optional): for JavaScript tooling integrations

---

## 2. What Path B Means (Read This Carefully)

There are two ways GenSense could grow:

**Path A (FORBIDDEN):** Make the analysis engine deeper — add control flow graphs, path-sensitive analysis, type inference. This takes years and competes with CodeQL, which has a 17-year head start. We will lose.

**Path B (WHAT WE ARE BUILDING):** Stay fast. Make rules easy to write. Detect bugs that AI coding assistants (like Copilot, Cursor, Claude) introduce. Add an AI layer that explains findings in plain language.

**The three rules of Path B:**
1. Analysis must complete in under 5 seconds on a 50-file project. If it gets slower, fix it before adding anything new.
2. Writing a new rule must be possible in one day without touching the Rust engine.
3. Every rule must be precise. If a rule fires on more than 1 in 100 lines of real code, it is too noisy and must be tightened or removed.

**Path A drift signals — stop work immediately if you are about to:**
- Add a control flow graph (CFG) to the engine
- Add path-sensitive analysis
- Add a type inference system
- Modify `temporal.rs`, `graph.rs`, or `tracking.rs` for any reason other than a bug fix
- Build anything that would make analysis take longer than 5 seconds

If you think you need to do one of these things to implement your task, stop. Ask for clarification. The answer is almost always "find a simpler approach."

---

## 3. The Codebase Map

Every file an agent will touch is listed here. Read the ones relevant to your task before writing code.

```
gensense/
├── Cargo.toml                          # Dependencies and feature flags
├── rules/                              # YAML rule files (embedded at compile time)
│   ├── core.yml                        # Core rules (unsafe blocks, host interaction, etc.)
│   ├── quality.yml                     # Quality rules (function size, nesting depth)
│   ├── rust/                           # Rust-specific YAML rules
│   ├── typescript/                     # TypeScript-specific YAML rules
│   └── database/                       # Database-related YAML rules
├── src/
│   ├── lib.rs                          # Public API: Advisory, GenSenseRule, Severity, GenSenseContext
│   ├── bin/
│   │   └── gensense.rs                 # CLI binary — all command-line argument handling
│   ├── engine/
│   │   ├── mod.rs                      # Engine struct — runs the full analysis pipeline
│   │   ├── auditor/
│   │   │   ├── mod.rs                  # GenSenseAuditor — orchestrates rule execution
│   │   │   ├── rules.rs                # Loads YAML rules + hardcoded rules into one Vec
│   │   │   ├── events.rs               # traverse_for_events — builds temporal event chain
│   │   │   └── discovery.rs            # discover_symbols — finds functions, vars, types
│   │   ├── fingerprint.rs              # N-gram hashing for copy-paste detection
│   │   ├── suppression.rs              # Handles gensense-ignore comments
│   │   └── project/
│   │       ├── mod.rs                  # Project-level analysis (multi-file)
│   │       └── consistency.rs          # Verifies taint paths are consistent
│   ├── rules/
│   │   ├── mod.rs                      # Re-exports
│   │   ├── ir.rs                       # CoreRuleIr, FlowConstraint, TemporalBehavior
│   │   ├── compiler.rs                 # RuleCompiler: CoreRule (YAML) → CoreRuleIr
│   │   ├── core/
│   │   │   ├── mod.rs                  # CoreRule struct — the YAML-deserialized rule type
│   │   │   └── helpers.rs              # serde_regex_opt, calculate_peak_depth, check_parent_scope
│   │   ├── global/
│   │   │   ├── ai_patterns/
│   │   │   │   ├── mod.rs              # Declares all AI pattern rule modules
│   │   │   │   ├── dead_result.rs      # AI_DEAD_RESULT_DISCARD
│   │   │   │   ├── placeholder_panic.rs # AI_PLACEHOLDER_PANIC
│   │   │   │   ├── redundant_comment.rs # AI_REDUNDANT_COMMENT
│   │   │   │   ├── tautological_assert.rs # AI_TAUTOLOGICAL_ASSERT
│   │   │   │   ├── ts_floating_promise.rs # AI_TS_FLOATING_PROMISE
│   │   │   │   └── useless_test.rs     # AI_USELESS_TEST
│   │   │   ├── secret_guard.rs         # Detects hardcoded secrets
│   │   │   └── todo_guard.rs           # Detects TODO/FIXME comments
│   │   └── rust/
│   │       ├── deadlock_guard.rs       # RUST_ASYNC_MUTEX_DEADLOCK
│   │       ├── async_safety.rs         # RUST_ASYNC_PANIC_SAFETY
│   │       ├── blocking_io.rs          # RUST_BLOCKING_IO
│   │       ├── fake_async.rs           # RUST_FAKE_ASYNC
│   │       ├── timeout_guard.rs        # RUST_TIMEOUT_GUARD
│   │       └── tracing_guard.rs        # RUST_TRACING_GUARD
│   ├── semantics/
│   │   ├── mod.rs                      # Re-exports SymbolRegistry, DataFlowAnalyzer
│   │   ├── graph.rs                    # SemanticGraph — the main graph data structure
│   │   ├── symbols.rs                  # SymbolRegistry — stores all named symbols
│   │   ├── temporal.rs                 # TemporalAnalyzer — checks event ordering
│   │   ├── consistency.rs              # ConsistencyCheck — verifies analysis agreement
│   │   └── data_flow/
│   │       ├── mod.rs                  # DataFlowAnalyzer struct
│   │       ├── tracking.rs             # Taint BFS (check_taint_graph)
│   │       └── lookup.rs               # find_definition, map_params
│   ├── parser.rs                       # ParserRegistry — gets tree-sitter language by file ext
│   ├── reporter.rs                     # Reporter — formats output as SARIF or text
│   └── patcher/
│       └── mod.rs                      # PatchManager — applies proposed_replacement fixes
└── tests/
    ├── temporal_fsm_tests.rs           # Tests for temporal ordering
    └── samples/                        # Fixture files used by tests
        ├── bug_test.rs
        ├── taint_test.ts
        └── ...
```

---

## 4. How the Engine Works (End to End)

Understanding this is mandatory before writing any rule or any test.

When you run `gensense audit myproject/`, here is exactly what happens:

**Step 1 — File discovery.** The engine walks the directory and finds all `.rs`, `.ts`, `.tsx`, `.sol` files.

**Step 2 — Symbol discovery.** For each file, `discover_symbols` runs a tree-sitter parse and extracts all named things: function definitions, variable declarations, struct definitions, imports. These go into a `SymbolRegistry`. Each symbol has a name, file path, line number, and kind (Function, Variable, Type, etc.).

**Step 3 — Call edge discovery.** The engine scans each file for function calls and adds `EdgeKind::Calls` edges in the `SemanticGraph` linking callers to callees. This is what enables cross-file analysis.

**Step 4 — Event discovery.** `traverse_for_events` walks each file's AST and emits `TemporalEvent` nodes into the graph. Events are things like: a `.lock()` call, an `.await` expression, a variable assignment, a return statement. Each event gets a `SequentiallyFollows` edge to the next event in the same scope.

**Step 5 — Rule execution.** Every rule's `check()` method is called on every matching AST node in every file. Rules run in parallel (via Rayon). Each rule returns a `Vec<Advisory>`.

**Step 6 — Output.** All advisories are collected, optionally filtered by severity, and printed as text, JSON, or SARIF.

**Key data types you will use:**

```rust
// An advisory is one finding. This is what a rule returns.
pub struct Advisory {
    pub rule_id: String,          // e.g. "AI_CONFIDENCE_THEATRE"
    pub severity: Severity,       // Critical, Warning, or Info
    pub observation: String,      // What was found (specific to this instance)
    pub impact: String,           // Why it matters
    pub improvement: String,      // How to fix it
    pub line: usize,              // Line number in the file (1-indexed)
    pub column: usize,            // Column number (1-indexed)
    pub file_path: String,        // Full path to the file
    pub original_content: String, // The exact code that triggered the finding
    pub proposed_replacement: Option<String>, // Optional: the fixed code
}

// The context passed to every rule's check() method
pub struct GenSenseContext<'a> {
    pub file_path: &'a Path,      // Path to the file being analyzed
    pub source_code: &'a str,     // Full source code of the file as a string
    pub symbols: &'a SymbolRegistry, // All symbols discovered in the project
}

// Severity levels
pub enum Severity { Critical, Warning, Info }
```

---

## 5. How Rules Work

There are two ways to write a rule. Choose the simplest one that works.

### Method 1: YAML Rule (preferred for most patterns)

A YAML rule is a block in a `.yml` file in the `rules/` directory. These are embedded into the binary at compile time. You do not need to touch any Rust code to add a YAML rule.

**Full example of a YAML rule:**

```yaml
rules:
  - id: "RUST_SILENT_FAILURE"
    domain: "reliability"
    target_ext: "rs"
    on_node: "function_item"
    if_matches: "Err.*=>.*Ok\\(\\(\\)\\)"
    observation: "Error branch returns Ok(()) — failure is silently swallowed."
    impact: "The caller has no way to know the operation failed."
    improvement: "Propagate the error with ? or return Err(...)."
    severity: Warning
```

**What each field means:**

| Field | Required | What it does |
|-------|----------|-------------|
| `id` | Yes | Unique identifier. Use SCREAMING_SNAKE_CASE. Must be unique across ALL rules. |
| `domain` | Yes | Category. One of: `security`, `reliability`, `performance`, `maintainability`, `quality` |
| `target_ext` | Yes | File extension to target. `rs`, `ts`, `tsx`, or `*` for all files |
| `on_node` | Yes | The tree-sitter node kind to match. The rule fires once per matching node. |
| `if_matches` | No | Regex. If set, the rule only fires if the node's full text matches this pattern. |
| `must_contain` | No | Regex. Rule fires if this pattern is NOT found in the node. (Inverted — violation means missing) |
| `must_not_contain` | No | Regex. Rule fires if this pattern IS found in the node. |
| `max_lines` | No | Rule fires if the node has more than this many lines. |
| `max_depth` | No | Rule fires if the nesting depth exceeds this number. |
| `within_scope` | No | Only fire if inside a parent node of this kind. e.g. `async_fn` |
| `source_pattern` | No | For taint rules: regex matching the taint source. |
| `sink_pattern` | No | For taint rules: regex matching the taint sink. Both required together. |
| `severity` | No | `Critical`, `Warning`, or `Info`. Defaults to `Warning`. |
| `observation` | Yes | Plain English description of what was found. |
| `impact` | Yes | Plain English description of why it matters. |
| `improvement` | Yes | Plain English description of how to fix it. |
| `temporal` | No | Temporal ordering block. See Section 6. |

**How to find the right `on_node` value:**

Run `gensense --debug myfile.rs` on a file that contains the pattern you want to detect. This prints the tree-sitter AST. Find the node kind that wraps the pattern. Common ones:

| Pattern | `on_node` |
|---------|-----------|
| Any function | `function_item` (Rust), `function_declaration` (TS) |
| Any function call | `call_expression` |
| Variable declaration | `let_declaration` (Rust), `lexical_declaration` (TS) |
| Macro call | `macro_invocation` |
| Match expression | `match_expression` |
| If expression | `if_expression` |
| For loop | `for_expression` (Rust), `for_statement` (TS) |
| Test function | `function_item` with `if_matches: "#\\[test\\]"` |
| Arrow function | `arrow_function` |

**Where to put the YAML file:**

- Rules for all languages: `rules/core.yml` or a new `rules/<domain>.yml`
- Rust-specific rules: `rules/rust/core.yml`
- TypeScript-specific rules: `rules/typescript/core.yml`

The loader in `src/engine/auditor/rules.rs` automatically finds and loads all `.yml` files in the `rules/` directory recursively. You do not need to register the file anywhere.

**How to register a new YAML file:**

You do not need to. All `.yml` files in the `rules/` directory are automatically discovered and loaded by the engine. Just create the file.

---

### Method 2: Procedural Rust Rule (for complex patterns only)

Use this when the pattern cannot be expressed with regex + node matching. Examples: walking the AST to look at child nodes, comparing two parts of the same node, using the semantic graph.

**Step 1.** Create a new file in the right directory:
- Global rule (all languages): `src/rules/global/ai_patterns/my_rule.rs`
- Rust-only rule: `src/rules/rust/my_rule.rs`

**Step 2.** Write the rule. Copy this template exactly:

```rust
// src/rules/global/ai_patterns/confidence_theatre.rs

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct ConfidenceTheatre;

impl GenSenseRule for ConfidenceTheatre {
    fn id(&self) -> &str {
        "AI_CONFIDENCE_THEATRE"
        // RULE: Must be unique. Check all existing rule IDs before choosing one.
    }

    fn description(&self) -> &str {
        "Error handled by logging but silently continues with a default value."
    }

    fn category(&self) -> &str {
        "AI Patterns"
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
        // RULE: Return true only for the extensions this rule targets.
        // Use: ext == "rs" || ext == "ts" for multiple languages.
    }

    fn severity(&self) -> crate::Severity {
        crate::Severity::Warning
        // RULE: Use Critical only for security bugs or guaranteed runtime failures.
    }

    fn impact(&self) -> &str {
        "The operation silently failed. The caller proceeds as if nothing went wrong."
    }

    fn improvement(&self) -> &str {
        "Propagate the error with ? or return Err(...) instead of Default::default()."
    }

    fn query(&self) -> Option<&str> {
        // This is a tree-sitter query. The engine will call check() once
        // for every node that matches this query in the file.
        // Use (node_kind) @capture_name format.
        // Return None to have check() called on every node (slower, avoid if possible).
        Some("(match_expression) @match")
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // Get the full source text of this node
        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // Your detection logic goes here.
        // Example: detect Err arm that logs but returns a default value
        if code.contains("Err(") && code.contains("eprintln!") && code.contains("Default::default()") {
            advisories.push(self.new_advisory(
                &node,
                // observation: describe what was found in THIS specific node
                "Err arm logs the error but returns Default::default() — failure is hidden.".to_string(),
                self.impact().to_string(),
                self.improvement().to_string(),
            ));
        }

        advisories
        // RULE: Always return the Vec even if empty. Never panic. Never unwrap without handling.
    }
}
```

**Step 3.** Register the new struct in the module file.

If you created `src/rules/global/ai_patterns/confidence_theatre.rs`, add this line to `src/rules/global/ai_patterns/mod.rs`:

```rust
pub mod confidence_theatre;
```

**Step 4.** Register the rule in `src/engine/auditor/rules.rs` inside the `default_rules()` function:

```rust
rules.push(Box::new(
    crate::rules::global::ai_patterns::confidence_theatre::ConfidenceTheatre,
));
```

---

## 6. How Temporal Rules Work

A temporal rule checks the ORDER of events inside a function. For example: "a `.lock()` call must never be followed by an `.await` in the same function."

Temporal rules use the event graph. The engine records every significant call and expression inside each function as a `TemporalEvent`, linked in order with `SequentiallyFollows` edges.

### Writing a temporal rule in YAML

Add a `temporal:` block to your rule definition:

```yaml
rules:
  - id: "RUST_LOCK_BEFORE_AWAIT"
    domain: "reliability"
    target_ext: "rs"
    on_node: "function_item"
    observation: "A mutex lock is held across an await point."
    impact: "Holding a std::sync::Mutex guard across .await blocks the entire executor thread."
    improvement: "Drop the guard before the .await, or use tokio::sync::Mutex instead."
    severity: Critical
    temporal:
      sequence: ["lock", "await"]
      behavior: must_not_follow
```

**`behavior` options:**

| Value | Meaning |
|-------|---------|
| `must_follow` | All items in `sequence` must appear in that order. Missing any step is a violation. |
| `must_not_follow` | The second item must NEVER appear after the first. |
| `forbidden_between` | Items in `sequence` must not appear between the two `between` patterns. Requires the `between` field. |

**`forbidden_between` example:**

```yaml
temporal:
  sequence: ["spawn"]
  behavior: forbidden_between
  between: ["lock", "unlock"]
```

This fires if `spawn` appears between a `lock` and an `unlock` in the same function.

**How `sequence` is matched:** Each item in the sequence is a regex matched against the event label. Event labels are the names of calls and variables. For example:
- A call to `mutex.lock()` produces an event with label `lock`
- An `.await` expression produces an event with label `.await`
- A call to `tokio::spawn(...)` produces an event with label `spawn`

---

## 7. Phase 1 — Bug Fixes (Weeks 1–2)

**Agent role in this phase: Agent B (test writer)**

The three bugs have already been fixed. The fixed files are:
- `src/semantics/graph.rs` — Bug 1 (O(n²) start-node search)
- `src/engine/auditor/events.rs` — Bug 2 (closures in temporal traversal)
- `src/semantics/data_flow/tracking.rs` — Bug 3 (over-taint via InScope edges)

**Your job: write one regression test per bug.**

Tests live in `tests/`. Look at `tests/temporal_fsm_tests.rs` for the exact pattern to follow.

**Test for Bug 2 — closure isolation:**

```rust
// tests/regression_tests.rs

#[test]
fn test_closure_events_do_not_bleed_into_outer_scope() {
    // This code has a lock in the outer function and an await INSIDE a closure.
    // Before the fix, events inside the closure incorrectly chained into the
    // outer scope's temporal sequence, causing a false positive deadlock warning.
    // After the fix, zero advisories should be produced.
    let content = r#"
        async fn process(mutex: &tokio::sync::Mutex<i32>) {
            let guard = mutex.lock().await;
            let handler = |x: i32| {
                let result = x + 1;
                result
            };
            drop(guard);
        }
    "#;

    // ... set up parser, registry, auditor as shown in temporal_fsm_tests.rs
    // Run DeadlockGuard rule against this content
    // Assert: advisories.is_empty()
}
```

**Test for Bug 3 — taint does not bleed across unrelated functions:**

```rust
#[test]
fn test_taint_does_not_cross_unrelated_functions() {
    // function_a has a taint source.
    // function_b has a sink.
    // Neither calls the other.
    // Before the fix, taint bled via InScope edges — false positive.
    // After the fix, zero advisories should be produced.
    let content = r#"
        fn function_a() {
            let secret = std::env::var("SECRET").unwrap();
            println!("{}", secret);
        }

        fn function_b() {
            execute_query("SELECT 1");
        }
    "#;

    // Assert: no taint findings for function_b
}
```

**Test for Bug 1 — correct start node in ordered events:**

```rust
#[test]
fn test_ordered_events_finds_true_start_node() {
    // A function with a nested block that creates events before the outer events.
    // Before the fix, the start node could be wrong due to out-of-order insertion.
    // After the fix, events must be returned in source-order.
    let content = r#"
        fn outer() {
            let a = acquire();
            {
                let b = inner_call();
            }
            let c = release();
        }
    "#;

    // Set up and run ordered_events_in_scope
    // Assert: events[0].label == "acquire" (not "inner_call")
}
```

**Exit criterion for Phase 1:** All three tests pass. Run `cargo test` — the full suite must be green. Run `gensense audit .` on the GenSense codebase itself and record the number of findings. This is your baseline.

---

## 8. Phase 2 — Complete the DSL (Weeks 3–8)

### 8A — Expose temporal rules in YAML

**Files to modify:**
1. `src/rules/core/mod.rs` — add `TemporalRuleConfig` struct and `temporal` field to `CoreRule`
2. `src/rules/compiler.rs` — read the `temporal` field and emit `FlowConstraint::Temporal`

The full updated `src/rules/core/mod.rs` has already been written. It is available as `core_mod.rs` in the outputs. Copy it to `src/rules/core/mod.rs`.

**Verify it works by converting one existing Rust rule to YAML:**

Create `rules/rust/temporal.yml`:

```yaml
rules:
  - id: "RUST_LOCK_BEFORE_AWAIT_YAML"
    domain: "reliability"
    target_ext: "rs"
    on_node: "function_item"
    observation: "A mutex lock is held across an await point (YAML rule)."
    impact: "Holding a std::sync::Mutex guard across .await blocks the entire async executor thread."
    improvement: "Drop the guard before the .await, or switch to tokio::sync::Mutex."
    severity: Critical
    temporal:
      sequence: ["lock", "await"]
      behavior: must_not_follow
```

Run this against `tests/samples/bug_test.rs`. It should produce the same finding as the Rust `DeadlockGuard` rule. If both produce the same finding, the DSL is working.

### 8B — Build `gensense test-rule` CLI command

**File to modify:** `src/bin/gensense.rs`

**What the command does:**

```bash
gensense test-rule rules/rust/temporal.yml \
  --fixture tests/samples/bug_test.rs \
  --expect-finding RUST_LOCK_BEFORE_AWAIT_YAML \
  --expect-line 14
```

This command:
1. Loads ONLY the rules from the specified YAML file
2. Runs them against ONLY the fixture file
3. Checks that a finding with `rule_id == RUST_LOCK_BEFORE_AWAIT_YAML` appears at line 14
4. Prints `PASS` or `FAIL` with details

**How to add it to the CLI (in `src/bin/gensense.rs`):**

Add a new argument check before the main analysis block. Look at how `--debug` is handled for the pattern:

```rust
if let Some(pos) = args.iter().position(|a| a == "test-rule") {
    // Get the YAML file path (next arg after "test-rule")
    let rule_file = args.get(pos + 1).expect("test-rule requires a YAML file path");

    // Parse --fixture, --expect-finding, --expect-line from remaining args
    // Load only the rules from that YAML file
    // Run against the fixture file
    // Check that expected finding appears at expected line
    // Print PASS or FAIL
    // Exit 0 for pass, 1 for fail
    std::process::exit(0);
}
```

**Why this matters:** Without this command, Agent A cannot verify their YAML rules without writing a full Rust integration test. This unblocks the entire pattern library phase. It is the most important thing built in Phase 2.

### 8C — Rule composition (`only_if_rule_fired`)

**Skip this for v1.** It is useful but not required for the first release. Implement after the pattern library has 10+ rules.

---

## 9. Phase 3 — LLM Pattern Library (Weeks 9–16)

**Agent role in this phase: Agent A (rule writer)**

**The job:** Write 2 new rules per week. Each rule detects a bug that AI coding assistants (Copilot, Cursor, Claude) commonly introduce. Each rule ships with a fixture file and a test.

**The workflow for every single rule:**

1. Write the rule (YAML preferred, Rust if needed)
2. Write a fixture file in `tests/samples/ai_patterns/` that demonstrates the bad pattern
3. Write a fixture file that demonstrates the CORRECT version of the same code (to verify no false positive)
4. Run `gensense test-rule` against both fixtures. Bad version must fire. Good version must not fire.
5. Run the rule against a real Rust project (use the GenSense codebase itself). Count false positives.
6. If false positives > 1 per 100 lines, tighten the rule or remove it.

**The 10 rules to build, in order:**

---

### Rule 1: AI_CONFIDENCE_THEATRE

**What it detects:** An `Err` arm that logs the error but returns a default value instead of propagating it. The function appears to handle errors but actually swallows them silently.

**Why LLMs do this:** LLMs always add error handling because they know it is expected. But they often pick the laziest form: log it and continue. The code looks correct in review but fails silently in production.

**Example of bad code:**

```rust
match do_something() {
    Ok(v) => process(v),
    Err(e) => {
        eprintln!("Error: {e}");
        Default::default()  // <-- silent failure
    }
}
```

**Example of good code:**

```rust
match do_something() {
    Ok(v) => process(v),
    Err(e) => return Err(e),  // propagate
}
```

**How to detect:** The node kind is `match_expression`. Check if the node text contains both `Err(` and any of `eprintln!`, `println!`, `log::`, `tracing::`, `warn!`, `error!` AND also contains `Default::default()` or `Vec::new()` or `String::new()` or `0` or `false` as a return value in the same arm.

**Fixture files:**
- `tests/samples/ai_patterns/confidence_theatre_bad.rs` — the bad pattern
- `tests/samples/ai_patterns/confidence_theatre_good.rs` — correct error propagation

**Rule ID:** `AI_CONFIDENCE_THEATRE`  
**Severity:** Warning  
**Target:** `rs`

---

### Rule 2: AI_TEST_THEATRE

**What it detects:** A test function where every assertion compares a literal value against a call with only literal arguments. These tests only verify that one specific input produces one specific output — they test the LLM's own output, not the function's behaviour under varying inputs.

**Why LLMs do this:** When asked to "write tests", LLMs call the function with the values they just made up in the implementation and assert the result they know will come out. These tests never catch regressions.

**Example of bad code:**

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);  // <-- proves nothing about add(x, y)
    assert_eq!(add(0, 0), 0);
}
```

**Example of good code:**

```rust
#[test]
fn test_add() {
    let x = 10;
    let y = 20;
    assert_eq!(add(x, y), x + y);  // behaviour-based
}
```

**How to detect:** The node kind is `function_item`. Check if the node contains `#[test]` or `#[tokio::test]`. Then check if every `assert_eq!` or `assert!` in the body has only literal arguments (numbers, string literals, boolean literals). If ALL assertions are literal-only, flag it.

**Rule ID:** `AI_TEST_THEATRE`  
**Severity:** Warning  
**Target:** `rs`

---

### Rule 3: AI_COPY_PASTE_SCOPE_ERROR

**What it detects:** Two blocks of code in the same file that have more than 80% token overlap but differ in small details (variable names changed, one line modified). This is the fingerprint of LLM copy-paste where the model adapted a block but forgot to update something.

**How to detect:** The fingerprinting system already exists in `src/engine/fingerprint.rs`. Read that file. It computes n-gram hashes. Compare hashes between blocks in the same file. If overlap > 80% and blocks are different, flag both locations.

**This is the only rule that requires reading `src/engine/fingerprint.rs`.**

**Rule ID:** `AI_COPY_PASTE_SCOPE_ERROR`  
**Severity:** Warning  
**Target:** `rs`, `ts`

---

### Rule 4: AI_HALLUCINATED_DEFAULT

**What it detects:** An `unwrap_or(0)` or `unwrap_or(-1)` or `unwrap_or("")` call on a value whose name suggests it is a timeout, limit, capacity, or size. A default of 0 for a timeout means infinite wait. A default of -1 for a retry count is undefined behaviour.

**Example of bad code:**

```rust
let timeout = config.get("timeout").unwrap_or(0);    // 0 = infinite wait
let max_retry = config.get("retries").unwrap_or(-1); // -1 = ???
```

**How to detect:** The node kind is `call_expression`. Check if the call ends in `.unwrap_or(` and the argument is `0`, `-1`, `""`, `false`, or `Vec::new()`. Then check if the variable being assigned (or the field name before the call) contains any of: `timeout`, `retry`, `limit`, `max`, `capacity`, `size`, `count`, `bound`, `threshold`.

**Rule ID:** `AI_HALLUCINATED_DEFAULT`  
**Severity:** Warning  
**Target:** `rs`

---

### Rule 5: AI_DOC_IMPL_DRIFT

**What it detects:** A function whose doc comment claims it returns `None` or `Err` under some condition, but the function body has no code path that returns `None` or `Err`.

**Example of bad code:**

```rust
/// Returns None if the user is not found.
fn get_user(id: u32) -> User {  // <-- return type is not Option<User>
    database.fetch(id)
}
```

**How to detect:** The node kind is `function_item`. Get the text of the doc comment (lines starting with `///` immediately before the function). Check if it contains phrases like `returns None`, `returns Err`, `returns null`, `panics if`, `will fail`. Then check the function's return type and body:
- If the doc says `returns None` but the return type does not contain `Option`, flag it.
- If the doc says `returns Err` but the return type does not contain `Result`, flag it.
- If the doc says `panics if` but the body contains no `panic!`, `unwrap()`, `expect()`, or `assert!`, flag it.

**Rule ID:** `AI_DOC_IMPL_DRIFT`  
**Severity:** Warning  
**Target:** `rs`

---

### Rule 6: AI_ASYNC_CLOSURE_MISUSE

**What it detects:** A `tokio::spawn` or `std::thread::spawn` call whose closure captures a variable whose name or type suggests it is a lock guard, database connection, or non-Send resource.

**Example of bad code:**

```rust
let guard = mutex.lock().unwrap();
tokio::spawn(async move {
    use_value(*guard);  // guard is captured — MutexGuard is not Send
});
```

**How to detect:** The node kind is `call_expression` where the call matches `tokio::spawn|thread::spawn`. Check if the closure argument contains variable names matching: `guard`, `lock`, `conn`, `connection`, `db`, `transaction`, `session`. If yes, flag it.

**Rule ID:** `AI_ASYNC_CLOSURE_MISUSE`  
**Severity:** Critical  
**Target:** `rs`

---

### Rule 7: AI_OVER_SPECIFIED_GENERICS

**What it detects:** A generic function with more than 4 trait bounds where fewer than half the bounds are actually used in the function body.

**Example of bad code:**

```rust
fn process<T: Clone + Debug + Display + Serialize + Default + Hash>(val: T) -> T {
    val.clone()  // only Clone is actually used
}
```

**How to detect:** The node kind is `function_item`. Count the trait bounds in the `where` clause or inline generic parameters. Count how many of those trait names actually appear in the function body. If total bounds > 4 and used_bounds < total_bounds / 2, flag it.

**Rule ID:** `AI_OVER_SPECIFIED_GENERICS`  
**Severity:** Info  
**Target:** `rs`

---

### Rule 8: AI_EMPTY_CATCH (TypeScript)

**What it detects:** A `try/catch` block where the `catch` body is empty or only contains a comment.

**Example of bad code:**

```typescript
try {
    await fetchData();
} catch (e) {
    // ignore
}
```

**How to detect:** The node kind is `try_statement`. Find the `catch_clause` child. Get its body. If the body is empty (`{}`) or contains only comments (no actual statements), flag it.

**Rule ID:** `AI_EMPTY_CATCH`  
**Severity:** Warning  
**Target:** `ts`

---

### Rule 9: AI_PROMISE_NOT_AWAITED (TypeScript)

**What it detects:** A function call that returns a Promise that is neither awaited nor assigned to a variable nor chained with `.then()`. The operation runs but its result and errors are discarded.

**Example of bad code:**

```typescript
async function save(data: Data) {
    writeToDatabase(data);  // returns Promise, not awaited
    return "saved";
}
```

**How to detect:** This rule already exists as `AI_TS_FLOATING_PROMISE` in `src/rules/global/ai_patterns/ts_floating_promise.rs`. Read that rule. Write a companion YAML rule that catches a slightly different form: async function calls used as standalone expression statements without `await`, `.then()`, or `.catch()`.

**Rule ID:** `AI_PROMISE_STATEMENT`  
**Severity:** Warning  
**Target:** `ts`

---

### Rule 10: AI_UNREACHABLE_AFTER_RETURN

**What it detects:** Code that appears after a `return` statement in the same block. LLMs sometimes generate this when editing a function — they add a new return path but leave the old code below it.

**Example of bad code:**

```rust
fn compute(x: i32) -> i32 {
    if x > 0 {
        return x * 2;
    }
    return -1;
    let result = x + 1;  // unreachable
    result
}
```

**How to detect:** The node kind is `block`. Walk the children in order. When you find a `return_statement`, check if there are any non-comment children after it. If yes, flag the first statement after the return.

**Rule ID:** `AI_UNREACHABLE_AFTER_RETURN`  
**Severity:** Warning  
**Target:** `rs`, `ts`

---

## 10. Phase 4 — AI Reasoning Layer (Weeks 17–24)

**Agent role in this phase: Agent C (AI layer)**

### What this phase builds

Right now GenSense emits an `Advisory` with static strings: the same `observation`, `impact`, and `improvement` text for every instance of a rule, regardless of what the specific code looks like.

This phase adds an enrichment step that:
1. Takes each `Advisory` plus the surrounding code context
2. Sends it to the Anthropic API
3. Gets back a specific explanation and fix for that exact code
4. Returns an `EnrichedAdvisory`

### New file: `src/engine/enrichment.rs`

Create this file from scratch. Here is the complete specification:

```rust
// src/engine/enrichment.rs

use crate::Advisory;

/// An advisory enriched with AI-generated context-specific explanation and fix.
pub struct EnrichedAdvisory {
    pub base: Advisory,
    pub explanation: String,      // Why THIS specific code is a problem
    pub fix: Option<String>,      // Concrete replacement for base.original_content
    pub confidence: Confidence,   // How certain the model is this is a real bug
    pub is_intentional: bool,     // Did context suggest this is deliberate?
}

pub enum Confidence {
    High,    // Definite bug — always show
    Medium,  // Likely issue — show with note
    Low,     // Heuristic — suppress by default in filter mode
}
```

### The enrichment function

```rust
pub async fn enrich(
    advisory: &Advisory,
    source_code: &str,
    rule_description: &str,
) -> EnrichedAdvisory
```

**How to get the context window:**

The `advisory` has `line` and `file_path`. The `source_code` is the full file content. Extract ±15 lines around the finding:

```rust
let lines: Vec<&str> = source_code.lines().collect();
let start = advisory.line.saturating_sub(16); // 15 lines before
let end = (advisory.line + 15).min(lines.len());
let context_window = lines[start..end].join("\n");
```

**The prompt to send:**

```
You are a senior {language} engineer reviewing a static analysis finding.

Rule: {rule_id} — {rule_description}
File: {file_path}, Line: {line}

Flagged code:
{original_content}

Surrounding context:
{context_window}

Tasks:
1. In 2-3 sentences, explain why this specific instance is a real problem,
   or why it might be a false positive given the surrounding context.
2. If it is a real problem, provide the corrected version of ONLY the flagged
   code — no explanation, just the replacement.
3. Rate your confidence: High (definite bug), Medium (likely issue),
   Low (heuristic, may be intentional).
4. If you see a comment, type annotation, or surrounding pattern that suggests
   this is intentional, set intentional to true.

Respond ONLY in JSON with no preamble or markdown:
{
  "explanation": "...",
  "fix": "...",
  "confidence": "High|Medium|Low",
  "intentional": true|false
}
```

**The API call:**

```rust
let response = reqwest::Client::new()
    .post("https://api.anthropic.com/v1/messages")
    .header("Content-Type", "application/json")
    // API key comes from ANTHROPIC_API_KEY environment variable
    // Do NOT hardcode it. Do NOT put it in any file.
    .json(&serde_json::json!({
        "model": "claude-haiku-4-5-20251001",  // Use Haiku for cost efficiency
        "max_tokens": 500,
        "messages": [{"role": "user", "content": prompt}]
    }))
    .send()
    .await?;
```

Add `reqwest` to `Cargo.toml` under `[dependencies]` with the `json` feature:

```toml
reqwest = { version = "0.12", features = ["json"], optional = true }
```

Add a feature flag:

```toml
[features]
enrichment = ["dep:reqwest"]
```

### Three operating modes

**Mode 1 — Filter mode (default, enabled with `--enrich` flag):**
Run enrichment after analysis. If `confidence == Low` AND `is_intentional == true`, move the advisory to a suppressed list — do not show it. This is the false positive filter.

**Mode 2 — Explain mode (`--explain` flag):**
Show the full `EnrichedAdvisory` for every finding. Each finding shows:
- The original observation
- The AI-written explanation for this specific code
- The AI-generated fix (if any)
- The confidence level

**Mode 3 — CI mode:**
Enrich only `Critical` severity findings. All others get the base `Advisory`. This keeps CI fast.

### Cache layer (mandatory — do not ship without this)

Cache enrichments so the same bug in the same code never costs two API calls.

Cache key: `format!("{}:{}", advisory.rule_id, sha256(advisory.original_content))`

Cache location: `.gensense-cache/` directory in the project root. Each cache entry is a JSON file named by the cache key.

```rust
// Before calling the API, check the cache
let cache_key = format!("{}:{}", advisory.rule_id, sha256(&advisory.original_content));
let cache_path = PathBuf::from(".gensense-cache").join(&cache_key);

if cache_path.exists() {
    // Read and deserialize the cached EnrichedAdvisory
    return read_from_cache(&cache_path);
}

// Cache miss — call the API
let enriched = call_api(advisory, source_code, rule_description).await?;

// Write to cache
write_to_cache(&cache_path, &enriched);

enriched
```

### Batching (mandatory — do not ship without this)

Do NOT make one API call per finding. Group all findings from one file into a single API call using a JSON array prompt. A file with 8 findings makes 1 API call, not 8.

### Hard timeout

Wrap every API call in a 10-second timeout. If the API is slow, emit the base `Advisory` without enrichment rather than blocking the user.

```rust
let result = tokio::time::timeout(
    std::time::Duration::from_secs(10),
    call_api(advisory, source_code, rule_description)
).await;

match result {
    Ok(enriched) => enriched,
    Err(_timeout) => EnrichedAdvisory {
        base: advisory.clone(),
        explanation: advisory.observation.clone(), // fall back to static text
        fix: advisory.proposed_replacement.clone(),
        confidence: Confidence::Medium,
        is_intentional: false,
    }
}
```

---

## 11. What Each Agent Should Never Do

These are absolute rules. Breaking any of them requires the human to review and revert before anything else continues.

**Agent A (rule writer) must never:**
- Modify any `.rs` file in `src/` except to add a new rule file in `src/rules/global/ai_patterns/` or `src/rules/rust/`
- Skip writing the fixture file
- Ship a rule without running `gensense test-rule` to verify it fires on the bad fixture and does not fire on the good fixture
- Use `unwrap()` in rule code without a comment explaining why it cannot fail

**Agent B (test writer) must never:**
- Write a test that passes without actually testing the behaviour (no `assert!(true)` style tests)
- Skip the test for any bug fix or new rule
- Modify production source files

**Agent C (AI layer) must never:**
- Hardcode the API key anywhere
- Make more than one API call per file (use batching)
- Remove the cache layer
- Remove the 10-second timeout
- Make the enrichment step block the main analysis — it must run after findings are already collected

**All agents must never:**
- Add a control flow graph to the engine
- Add type inference to the engine
- Modify `temporal.rs`, `graph.rs`, or `tracking.rs` except for approved bug fixes
- Make analysis slower — run the speed gate (`cargo test speed_gate`) before every commit
- Use `panic!` in production code paths — return `Vec::new()` and log the error instead

---

## 12. Skills Required by Task

| Task | Skills needed |
|------|--------------|
| Writing YAML rules | Reading tree-sitter AST output (`gensense --debug`), regex, YAML syntax |
| Writing procedural Rust rules | Rust basics, tree-sitter Node API, understanding of `GenSenseRule` trait |
| Writing tests | Rust test syntax, tree-sitter Parser setup (copy from `temporal_fsm_tests.rs`) |
| Building `gensense test-rule` CLI | Rust, std::env::args parsing, file I/O, understanding of `Engine::run` |
| Building AI enrichment layer | Rust async (tokio), reqwest HTTP client, JSON parsing (serde_json), file caching |
| Temporal YAML rules | Understanding of `TemporalBehavior` and `FlowConstraint::Temporal` in `src/rules/ir.rs` |

---

## 13. Definition of Done for v1

GenSense v1 is ready to release when ALL of the following are true:

- [ ] Three bug fixes applied and three regression tests passing
- [ ] `gensense test-rule` CLI command working
- [ ] Temporal rules expressible in YAML (at least `must_follow` and `must_not_follow`)
- [ ] At least 10 LLM pattern rules shipped, each with fixture files and passing tests
- [ ] No rule fires more than once per 100 lines on the GenSense codebase itself
- [ ] Analysis of a 50-file project completes in under 5 seconds (`cargo test speed_gate` passes)
- [ ] AI enrichment layer working with filter mode and explain mode
- [ ] Cache layer operational (second run on same codebase makes zero API calls for unchanged files)
- [ ] `cargo test` passes with zero failures
- [ ] `cargo clippy` produces zero warnings
- [ ] README updated with: installation, usage, YAML rule authoring guide, list of all rules

---

## 14. Quick Reference: File Locations

| What you want to do | File to create or edit |
|---------------------|----------------------|
| Add a YAML rule for Rust | `rules/rust/core.yml` (append to existing) or new `rules/rust/<name>.yml` |
| Add a YAML rule for TypeScript | `rules/typescript/core.yml` |
| Add a YAML rule for all languages | `rules/core.yml` |
| Add a new procedural Rust rule (global) | Create `src/rules/global/ai_patterns/<name>.rs`, register in `mod.rs` and `rules.rs` |
| Add a new procedural Rust rule (Rust-only) | Create `src/rules/rust/<name>.rs`, register in `mod.rs` and `rules.rs` |
| Add a test | Create or edit `tests/regression_tests.rs` |
| Add a fixture file | Create `tests/samples/ai_patterns/<name>.rs` or `.ts` |
| Modify the CLI | `src/bin/gensense.rs` |
| Modify how YAML rules are compiled | `src/rules/compiler.rs` |
| Modify the CoreRule struct | `src/rules/core/mod.rs` |
| Add AI enrichment | Create `src/engine/enrichment.rs` |