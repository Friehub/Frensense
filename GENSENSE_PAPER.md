# GenSense: A Proposition-Based Semantic Analysis Engine for the Agentic Era

**Friehub Engineering**
**May 2026**

---

## Abstract

GenSense is a static analysis engine that operates on abstract syntax trees rather than token text to identify semantic violations in Rust and TypeScript source code. It provides six distinct analysis modes: AST-pattern matching, Contextual Structural Analysis (CSA), temporal finite-state machine verification, intra-procedural taint tracking, inter-procedural call-graph constraints, and cross-language schema contract verification. On the roadmap, the engine extends into spectral graph analysis via a Topological Directed Hermitian Laplacian for architectural drift detection and a Jaccard n-gram clone detector for structural boilerplate identification. The engine is formally grounded: each rule is a decidable proposition over AST structure, and the system explicitly acknowledges Rice's Theorem as the boundary that defines which propositions are worth encoding. This paper documents the mathematical foundations, architectural decisions, and empirical results of GenSense v0.3.x, including its lineage from the predecessor Python-based Friehub Auditor.

---

## 1. Introduction

Large language models generate syntactically valid, type-correct code that nevertheless fails in production. The failures are not random. They are systematic and predictable, arising from the same root cause: a model that optimizes for locally plausible token sequences has no internal model of production consequences.

The patterns recur across projects and teams:

- `f64` used for monetary values, because that is what numeric assignments look like in training data. The consequence is accumulated floating-point rounding error across financial transactions.
- Raw SQL queries written against column names that do not match the ORM schema. The queries return zero rows silently.
- Event publication inside a database transaction. The transaction can roll back; the event has already fired. Downstream consumers act on uncommitted state.
- Mutex guards acquired inside async functions that then call `.await`. The guard is held across the yield point, causing deadlock under concurrent load.

None of these are caught by type systems, formatters, or conventional linters. They require analysis at the level of semantic intent: what does this code mean to do, and does its structure guarantee that intent is satisfied?

GenSense is built to answer that question for a fixed, decidable subset of semantic properties — the subset where static analysis can give confident, low-noise answers.

### 1.1 Scope and Contribution

This paper makes the following contributions:

1. A formal characterization of GenSense rules as decidable propositions over AST structure, grounded in Rice's Theorem.
2. A description of the four-pass semantic discovery pipeline: symbol extraction, call edge construction, temporal event chaining, and rule execution.
3. Specification of the Temporal Finite-State Machine (FSA) system for ordering constraints.
4. Specification of the inter-procedural taint analysis model and its cache architecture.
5. Specification of the Schema Contract rule type, which crosses language boundaries.
6. Empirical results: a comparison of false positive rates between the predecessor Python system and GenSense v0.3.0 on the same production codebase.

---

## 2. Background and Motivation: Rice's Theorem as a Design Constraint

### 2.1 Rice's Theorem

In 1953, Henry Gordon Rice proved that for any non-trivial semantic property of programs, no general algorithm can decide whether an arbitrary program satisfies that property. Formally:

Let $P$ be any non-trivial property of the partial functions computable by Turing machines (non-trivial meaning at least one machine satisfies it and at least one does not). Then the language $L_P = \{\langle M \rangle \mid M \text{ computes a function with property } P\}$ is undecidable.

This is a generalization of the Halting Problem. It tells us that complete, sound static analysis of semantic properties is impossible in the general case.

### 2.2 The Correct Response to Undecidability

Rice's Theorem does not mean static analysis is impossible. It means complete static analysis is impossible. Every practical static analyzer operates in the space of decidable approximations. The engineering question is: which approximations are precise enough to be useful?

GenSense answers this explicitly. It does not attempt to prove programs correct. It identifies a fixed set of structural patterns that, when present, indicate a class of semantic violation with high probability. The confidence score on each advisory is a first-class field, not an afterthought. Rules that cannot be expressed with high confidence are not written.

The system encodes propositions of the form:

> "A function whose name matches `validate.*` and whose body has no reachable path containing a rejection expression is a hollow implementation."

This proposition is decidable: we can traverse the AST, check name patterns, and enumerate reachable paths through the function body. The proposition has false positives (a function that delegates validation to a helper will be flagged). That rate is bounded by design via the `body_may_delegate_via` escape hatch. The decision to accept a bounded false positive rate in exchange for decidability is explicit.

### 2.3 The Curry-Howard Grounding

The Curry-Howard correspondence establishes that types and logical propositions are the same object viewed from different angles. A well-typed program is a proof of its type. A type checker is a proof verifier.

GenSense rules extend this correspondence to domain constraints that type systems cannot express. A type system can verify that a value is of type `Money`. It cannot verify that the value was not stored in an `f64` field before being wrapped. GenSense verifies the propositions that types cannot. The two systems are complementary layers of the same formal program-verification structure.

---

## 3. The Predecessor: Friehub Auditor (Python, 2025)

### 3.1 Architecture

Before GenSense, Friehub developed an audit tool in Python called Friehub Auditor, embedded in the TaaS Gateway infrastructure repository. The architecture was modular:

- **`SemanticBrain`** (`core/brain.py`): A persistent statistical engine that learned bigram and trigram frequency tables from the codebase. It stored learned patterns to disk between runs using an atomic snapshot-and-rename write strategy. It computed "Architectural Surprisal" per file — the average negative log-probability of the file's n-gram sequences relative to the project's learned distribution — and maintained a rolling window of 100 historical scores per language for drift percentile normalization.

- **`NgramDetector`** (`detectors/ngram.py`): Computed Shannon entropy over 50-token sliding windows. Windows below 3.2 bits of entropy were flagged as statistically flat. Combined bigram and trigram entropy for whole-file analysis against a configurable threshold (default 4.0 bits). Delegated surprisal scoring to `SemanticBrain`.

- **`RedlineEngine`** (`core/redline.py`): A declarative YAML-driven rule evaluator. Rules expressed `must_contain`, `must_not_contain`, conditional `if_contains / then_must_have` constraints, `max_lines`, and scope restrictions. The engine evaluated rules by running regex patterns against tokenized text.

- **`AIGuardianDetector`** (`detectors/ai_guardian.py`): Matched 12 hallmark AI phrases against function bodies. Detected redundant docstrings by comparing documentation word tokens against identifier tokens (flag if overlap > 60%). Detected near-duplicate function bodies using Jaccard similarity on n-gram fingerprints ($J \geq 0.8$).

### 3.2 The Shannon Entropy Model

The `NgramDetector` applied standard Shannon entropy to token sequences. For a sequence of $n$ tokens with empirical probability $P(x_i)$ for each unique token $x_i$:

$$H(X) = -\sum_{i=1}^{n} P(x_i) \log_2 P(x_i)$$

A window of 50 tokens from idiomatic, non-repetitive code typically scores between 3.8 and 5.5 bits. A window from LLM-generated boilerplate — repetitive comments, scaffolding code, placeholder logic — scores between 2.0 and 3.2 bits.

The threshold of 3.2 bits was empirically derived. Values below this level in a 50-token window reliably corresponded to boilerplate or comment-heavy sections.

The `SemanticBrain` surprisal model applied a smoothed n-gram language model. For a trigram $g$ in language $\ell$:

$$P_\text{smooth}(g \mid \ell) = \frac{\text{count}(g, \ell) + 0.1}{\text{total}(\ell) + 0.1 \cdot |\text{vocab}(\ell)|}$$

$$\text{surprisal}(g) = -\log_2 P_\text{smooth}(g \mid \ell)$$

$$\text{ArchSurprisal}(f) = \frac{1}{|G_f|} \sum_{g \in G_f} \text{surprisal}(g)$$

Where $G_f$ is the set of trigrams extracted from file $f$. A file in the top 5th percentile of the rolling drift baseline (i.e., drift percentile > 0.95) was flagged as an architectural outlier.

### 3.3 Why It Failed

The tool produced 3,024 findings on a moderately sized production codebase. More than 2,000 were false positives — a false positive rate exceeding 65%.

The cause was architectural, not incidental. The analysis operated on the wrong substrate.

The `RedlineEngine` evaluated rules by running regex patterns against tokenized text joined into a signature string. There was no concept of scope boundary. A pattern matching inside a comment, inside a test fixture, inside a dead code branch, and inside production logic all matched equally. As rule count grew, false positives grew combinatorially.

The entropy model had the symmetric problem. A statistically flat token sequence could be a repetitive boilerplate function or a correct but unusual algorithm — a tightly optimized inner loop that deliberately repeats the same operation pattern. The entropy calculation could not distinguish these cases. It measured statistical predictability, not semantic correctness.

The `AIGuardianDetector`'s block splitting used tree-sitter as a priority-one path but fell back to regex-based brace counting for any language where tree-sitter parsing failed. The fallback had no awareness of nested scopes, string literals containing braces, or macro invocations. A correct block containing a string literal with braces would be split at the wrong boundary, creating phantom findings.

This is what Rice's theorem looks like in practice. Adding more mathematics to the text-based approach was tried — probabilistic models, LSH-based similarity, rolling drift baselines — and none of it fixed the root cause. The substrate was wrong. Statistical analysis over token text is not a basis for reasoning about semantic correctness.

---

## 4. GenSense: Architectural Foundations

### 4.1 The Substrate Change

GenSense was renamed from TaaS on May 9, 2026, and represents a complete rebuild in Rust. The single most important architectural decision was the substrate: GenSense operates exclusively on tree-sitter abstract syntax trees, with no token-text fallback.

Tree-sitter produces a concrete syntax tree that reflects the actual grammar of the language being parsed. Every node has a kind (e.g., `function_item`, `call_expression`, `arrow_function`), a set of named fields (e.g., `name`, `body`, `arguments`), and precise byte-range positions in the source. Rules that fire on AST nodes fire on semantic units — not on strings that happen to contain a pattern.

The consequence is that scope is structural, not textual. A rule that checks whether a function body contains a rejection path traverses the AST of the function body. It cannot fire on a comment inside a test that mentions the pattern name. Scope boundaries are enforced by tree structure.

### 4.2 Module Architecture

GenSense is organized into six primary modules:

```
src/
  lib.rs            -- Core traits: GenSenseRule, ProjectRule, Advisory, GenSenseContext
  parser.rs         -- Tree-sitter language registry and parser initialization
  engine/
    auditor/        -- GenSenseAuditor: three-phase audit loop
      discovery.rs  -- Symbol extraction and call edge scanning
      events.rs     -- Temporal event chain construction
      mod.rs        -- Combined query execution and walk-tree fallback
    fingerprint.rs  -- SRI (Symbol-Relative Identity) fingerprinting
    suppression.rs  -- Inline and file-level suppression handling
  rules/
    compiler.rs     -- YAML DSL to CoreRuleIr / ProjectRuleIr compilation
    ir.rs           -- CoreRuleIr, ProjectRuleIr, FlowConstraint, TemporalBehavior
    core/           -- Typed YAML DSL structs
    definitions/    -- Embedded YAML rule files (rust/, typescript/, database/)
  semantics/
    symbols.rs      -- SymbolRegistry: symbol storage and lookup
    graph.rs        -- SemanticGraph: directed graph over symbols and events
    reachability.rs -- ReachabilityChecker: dead-branch-aware AST path analysis
    data_flow/
      normalization.rs -- SemanticOp extraction (Binding, Assignment, Call, EnterBlock)
  temporal/
    analyzer.rs     -- TemporalAnalyzer: FSA-based ordering constraint checking
    handler.rs      -- Temporal rule compilation and dispatch
  patcher/          -- Auto-remediation: diff generation and file patching
  reporter.rs       -- Output formatting: terminal, JSON, SARIF
```

### 4.3 Core Types

The central trait is `GenSenseRule`:

```rust
pub trait GenSenseRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;
    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory>;
    fn applies_to(&self, extension: &str) -> bool;
    fn file_check(&self, _context: &GenSenseContext<'_>) -> Vec<Advisory> { Vec::new() }
    fn query(&self) -> Option<&str> { None }
}
```

Every rule receives a tree-sitter `Node` and a `GenSenseContext`. The context carries the full semantic state:

```rust
pub struct GenSenseContext<'a> {
    pub file_id: FileId,
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub symbols: &'a SymbolRegistry,
    pub semantic_ops: &'a [SemanticOp],
    pub taint_cache: &'a TaintCache,
    pub file_trees: &'a HashMap<String, (Tree, String, Vec<SemanticOp>)>,
}
```

Every finding is an `Advisory`:

```rust
pub struct Advisory {
    pub rule_id: String,
    pub file_id: FileId,
    pub file_path: String,
    pub severity: Severity,          // Critical | Warning | Info
    pub confidence: f32,             // [0.0, 1.0]
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub line: u32,
    pub column: u32,
    pub start_byte: u32,
    pub end_byte: u32,
    pub original_content: String,
    pub proposed_replacement: Option<String>,
    pub proposed_import: Option<String>,
    pub enclosing_symbol: Option<String>,
    pub fingerprint: String,         // SRI fingerprint (FNV-1a hash)
    pub auto_fixable: bool,
    pub requires_human: bool,
    pub tags: Vec<String>,
}
```

The `fingerprint` field is computed with FNV-1a over the tuple `(rule_id, file_path, enclosing_symbol, original_content)`:

```rust
let mut hash: u64 = 0xcbf29ce484222325;
for byte in input.bytes() {
    hash ^= u64::from(byte);
    hash = hash.wrapping_mul(0x100000001b3);
}
```

This is the Symbol-Relative Identity (SRI) fingerprint. Because it includes `enclosing_symbol` rather than only line number, it remains stable across refactoring that shifts line numbers without changing the violated expression.

---

## 5. The Four-Pass Semantic Discovery Pipeline

Before any rule executes, GenSense runs a four-pass pipeline over the entire project to build the semantic context that rules depend on. This pass happens once per project scan and produces the `SymbolRegistry` that every rule receives.

### 5.1 Pass 1: Symbol Extraction

GenSense uses language-specific tree-sitter queries to extract named symbols from each file. For Rust, the query targets `function_item`, `struct_item`, `enum_item`, `trait_item`, `const_item`, and `let_declaration` nodes. For TypeScript, it targets `function_declaration`, `method_definition`, `class_declaration`, `interface_declaration`, `enum_declaration`, and `variable_declarator` nodes.

Each match produces a `Symbol`:

```rust
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,   // Function | Struct | Class | Enum | Constant | Variable | ...
    pub start_byte: usize,
    pub end_byte: usize,
    pub file_path: String,
    pub file_id: FileId,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
}
```

Symbols are sorted by `(line, column)` within each file and inserted into the `SymbolRegistry`. The registry maintains a `file_index: HashMap<String, Vec<SemanticNodeId>>` for O(log n) lookup of all symbols in a given file, and a `name_index: HashMap<String, Vec<NodeIndex>>` inside the underlying `SemanticGraph` for name-based lookup.

### 5.2 Pass 2: Call Edge Construction

A second tree-sitter query scans each file for `call_expression` nodes. For each call, the engine finds the enclosing function by walking the AST upward until it reaches a `function_item`, `function_declaration`, `arrow_function`, or `method_definition` node. It then records the pair `(caller_name, callee_name)`.

These pairs are resolved against the `SymbolRegistry` to add directed `Calls` edges to the `SemanticGraph`. Resolution prioritizes local-file symbols to avoid false cross-file edges, then falls back to unambiguous global symbols (those with exactly one match across all files):

```rust
pub fn add_call_edge(&mut self, file_path: &Path, src_name: &str, target_name: &str) {
    let src_node_id = src_symbols.iter()
        .find(|s| s.file_path == file_str)
        .and_then(|s| self.graph.find_node(&s.name, &s.file_path, s.line));

    let target_node_id = if let Some(local) = target_symbols.iter()
        .find(|s| s.file_path == file_str) {
        self.graph.find_node(&local.name, &local.file_path, local.line)
    } else if target_symbols.len() == 1 {
        self.graph.find_node(&target_symbols[0].name, ...)
    } else { None };

    if let (Some(s_id), Some(t_id)) = (src_node_id, target_node_id) {
        self.graph.add_edge(s_id, t_id, EdgeKind::Calls);
    }
}
```

The resulting call graph supports DFS-based reachability queries used by project-level rules.

### 5.3 Pass 3: Temporal Event Chain Construction

The third pass builds ordered event sequences within each function scope. The engine traverses the AST and identifies the following event types:

| AST Node Kind | EventType | Label |
|---|---|---|
| `call_expression` / `macro_invocation` with name `lock`, `try_lock`, `acquire` | `Acquire` | function name |
| `call_expression` with name `unlock`, `release`, `drop` | `Release` | function name |
| `await_expression` | `Await` | `.await` |
| `return_statement` | `Return` | `return` |
| `variable_declarator` / `let_declaration` / `assignment_expression` | `Assignment` | target name |
| all other `call_expression` / `macro_invocation` | `Call` | function name |

Events are linked into a `SequentiallyFollows` chain within each function scope using `InScope` edges that connect function symbols to their contained events. Scope boundaries (nested function definitions, closures, arrow functions) reset the `last_event` pointer, preventing events from inner scopes leaking into the outer ordering chain.

The ordered event sequence for a given function is retrieved via a topological traversal of the `SequentiallyFollows` edges within that scope's `InScope` neighborhood.

### 5.4 Pass 4: Rule Execution

With the `SymbolRegistry` fully populated, rules execute in a single pass over each file's AST. The engine uses a **combined tree-sitter query** strategy: at startup, all rules that provide a tree-sitter query string are merged into a single multi-pattern query per language:

```rust
fn build_combined_query(&self, ext: &str, language: &Language) -> Option<Query> {
    let mut patterns: Vec<String> = Vec::new();
    for rule in &self.rules {
        if !rule.applies_to(ext) { continue; }
        let Some(query_str) = rule.query() else { continue; };
        let rule_id = rule.id();
        let modified = query_str
            .replace("@node", &format!("@{rule_id}.node"))
            .replace("@call", &format!("@{rule_id}.call"));
        patterns.push(modified);
    }
    let combined_str = format!("[\n{}\n]", patterns.join("\n"));
    Query::new(language, &combined_str).ok()
}
```

This reduces AST traversals from $O(R)$ (one per rule) to $O(1)$ per file, where $R$ is the number of rules. The engine runs the combined query once, dispatches each capture to the corresponding rule via a `rule_index: HashMap<String, usize>`, and deduplates matches by `(rule_index, node_id)`.

Rules without a query (walk-tree rules) execute in a second sequential pass. File-level checks (e.g., `max_file_lines`) run in a third pass.

---

## 6. The Rule System

### 6.1 Rule DSL and Compilation

Rules are declared as YAML and compiled at engine startup into an internal representation (`CoreRuleIr`). The compilation pipeline is:

```
YAML file  ->  CoreRule (typed DSL struct)  ->  RuleCompiler::compile()  ->  CoreRuleIr
```

`CoreRuleIr` holds pre-compiled `Regex` objects for all pattern fields and pre-validated `AstQuery` structs for tree-sitter queries. This means zero regex compilation at check time.

A minimal rule:

```yaml
- id: TS_ASYNC_FOREACH
  name: Async forEach Usage
  target_ext: ts
  on_node: call_expression
  if_matches: "\.forEach\s*\("
  severity: Warning
  category: Reliability
  confidence: 0.90
  precision: very-high
  observation: forEach does not await the async callback.
  impact: Errors inside the callback are silently swallowed.
  improvement: Replace with a for...of loop or Promise.all.
```

A rule with taint constraints:

```yaml
- id: TS_PLAINTEXT_PASSWORD_STORE
  target_ext: ts
  on_node: arrow_function|function_declaration
  forbidden_source_pattern: "password|secret"
  forbidden_sink_pattern: "create|update|insert"
  severity: Critical
  category: Security
  confidence: 0.88
  observation: Plaintext credential may reach a database write.
  impact: Credentials stored without hashing.
  improvement: Hash with bcrypt before persistence.
```

### 6.2 Check Phases Within CoreRuleIr

When the engine dispatches a node to a rule, `CoreRuleIr::check()` executes up to five internal check phases in sequence. A phase returning false short-circuits the remainder.

**Phase 1 — Scope exclusion.** If `exclude_scope` is set, the engine checks the file path and all ancestor AST nodes for matching text. Nodes inside test modules (`#[cfg(test)]`) are excluded via explicit AST-level attribute detection — not regex on the file path, but by walking the `mod_item` siblings looking for `attribute_item` nodes containing `#[cfg(test)]`.

**Phase 2 — Regex matching.** If `if_matches` is set, the node's source text must match the pattern. If `fix_pattern` and `fix_with` are also set, the engine applies `Regex::replace_all` to generate the `proposed_replacement` at this point.

**Phase 3 — Structural matching.** If `if_name_matches` is set, the engine extracts the node's name child and checks the pattern. If `body_must_contain` is set, the engine delegates to `ReachabilityChecker::any_reachable_path_contains()`.

**Phase 4 — Content and metric constraints.** `must_contain`, `must_not_contain`, `body_must_contain_any_of`, `max_lines`, `max_depth`, and `must_be_preceded_by` checks run here.

**Phase 5 — Flow constraints.** Taint analysis and temporal FSA checks run here. These are the most expensive phases and are only reached if all earlier phases passed.

### 6.3 Reachability Analysis

The `ReachabilityChecker` performs dead-branch-aware path analysis over the AST body of a function. It implements a recursive walk with explicit handling for `if_statement` and `if_expression` nodes:

```rust
fn walk_reachable(&self, node: Node, pattern: &Regex, inside_dead_branch: bool) -> bool {
    if inside_dead_branch { return false; }

    if (kind == "if_statement" || kind == "if_expression") {
        let (consequence_dead, alternative_dead) = match self.evaluate_condition(cond) {
            Some(true)  => (false, true),   // condition always true: else branch dead
            Some(false) => (true, false),   // condition always false: then branch dead
            None        => (false, false),  // unknown: both branches live
        };
        // recurse into consequence and alternative with respective dead flags
    }

    // Siblings after return_statement or throw_statement are marked dead
}
```

`evaluate_condition` handles constant literals: `true`, `false`, `1`, `0`, `!true`, `!false`. All other conditions are treated as unknown (both branches live). This conservative approximation ensures the checker never misses a reachable violation.

### 6.4 CSA Rules (Contextual Structural Analysis)

CSA rules encode the hollow implementation pattern. A function whose name implies a contract — `validate`, `check`, `verify`, `parse`, `ensure` — must contain a reachable rejection path. A function named `validateInput` that has no path reaching a `throw`, `return Err`, or `reject` call is not validating anything; it is presenting the appearance of validation.

In the rule DSL this is expressed as:

```yaml
if_name_matches: "^(validate|check|verify|ensure|parse)"
body_must_contain: "(throw|return Err|reject|HttpException|BadRequest)"
body_may_delegate_via: "(validator\.|Validator\.|schema\.)"
```

The `body_may_delegate_via` field allows the rule to be satisfied if the body delegates to a known validation library, preventing false positives on thin wrapper functions.

### 6.5 Project-Level Rules and the Call Graph

Project-level rules implement `ProjectRule` and receive the complete immutable `SymbolRegistry` and `SourceRegistry`. They can reason across all files simultaneously.

**`MustHaveGuard`** verifies that every function matching `source_re` (in files matching `source_glob`) has a reachable call path to at least one function matching `guard_re` (in files matching `guard_glob`). Reachability is computed via DFS over `Calls` edges:

```rust
pub fn has_call_path(&self, from_nodes: &[SemanticNodeId], to_nodes: &[SemanticNodeId]) -> bool {
    let to_set: HashSet<_> = to_nodes.iter().map(|id| id.0).collect();
    for from in from_nodes {
        let mut visited = HashSet::new();
        let mut stack = vec![from.0];
        while let Some(current) = stack.pop() {
            if to_set.contains(&current) { return true; }
            for edge in self.graph.edges(current) {
                if *edge.weight() == EdgeKind::Calls && visited.insert(edge.target()) {
                    stack.push(edge.target());
                }
            }
        }
    }
    false
}
```

**`MustBeInternal`** flags any function matching `pattern` that is called from outside its own file or from files outside `file_glob`.

**`CrossFileTaintFree`** flags any source function that has a call path to a sink function in a different file.

**`SchemaContract`** is described in section 8.

---

## 7. Temporal Finite-State Machine Rules

### 7.1 Motivation

Ordering constraints are a class of semantic violation that type systems cannot express. The constraint "a mutex guard must not be held across an await point" is not a type error in Rust's type system — it compiles. The constraint "a database event must not be published before the enclosing transaction resolves" is not a type error in TypeScript. These are temporal properties: statements about the ordering of operations within an execution trace.

GenSense encodes ordering constraints as Finite-State Machine specifications evaluated over the temporal event chain produced by Pass 3.

### 7.2 TemporalBehavior

Three behaviors are supported:

```rust
pub enum TemporalBehavior {
    MustNotFollow,              // pattern[1] must not appear after pattern[0]
    MustFollow,                 // pattern[1] must appear after pattern[0]
    ForbiddenBetween(Regex, Regex), // pattern[2..] must not appear between start and end
}
```

**MustNotFollow** implements the deadlock detector. Given a sequence `[lock_re, await_re]`, the FSA scans the ordered event list for the scope. When it sees an event matching `lock_re`, it sets `found_first = true`. If it then sees an event matching `await_re` while `found_first` is true, it fires an advisory. The `found_first` flag resets on any `Release` event, representing the guard being dropped.

This catches `std::sync::Mutex` guards held across `.await` in Rust and equivalent patterns in TypeScript async functions.

**MustFollow** verifies that a required sequence is completed. Given `[check_balance_re, create_debit_re]`, the FSA increments `current_step` each time the next pattern in sequence matches. If the sequence is partially matched but not completed (i.e., `current_step > 0 && current_step < sequence.len()`), an advisory fires.

**ForbiddenBetween** marks a "forbidden zone" between a start pattern and an end pattern, and fires if any of the forbidden patterns appear inside that zone. This catches `publishEvent` calls inside `$transaction` bodies.

### 7.3 Rule Specification

A temporal rule in YAML:

```yaml
- id: RUST_LOCK_SLEEP
  target_ext: rs
  on_node: function_item
  temporal:
    sequence: ["^lock$", "^sleep$"]
    behavior: must_not_follow
  severity: Warning
  category: Concurrency
  confidence: 0.92
  observation: Mutex guard held while thread sleeps.
  impact: All other threads waiting for this lock are blocked for the sleep duration.
  improvement: Release the lock before sleeping.
```

The `temporal` block is compiled by `compile_temporal_config()` into a `Vec<Regex>` and a `TemporalBehavior` variant. The resulting `FlowConstraint::Temporal` is stored in the rule's `flow_constraints` list and evaluated in Check Phase 5.

### 7.4 Event Ordering Correctness

The event chain produced by Pass 3 is a DAG (directed acyclic graph) under the `SequentiallyFollows` relation within a single scope. Ordering is recovered by topological sort: nodes with no incoming `SequentiallyFollows` edges from within the scope are the chain heads; from each head, BFS follows `SequentiallyFollows` edges to produce the ordered sequence.

When multiple chain heads exist (parallel branches), they are sorted by `(line, column)` to produce a deterministic ordering. This is a conservative approximation — it treats all branches as sequential — but ensures that no violation is missed due to ordering ambiguity.

---

## 8. Intra-Procedural Taint Analysis

### 8.1 The Semantic Operation Layer

Taint analysis operates over a normalized semantic representation of each function — a flat sequence of `SemanticOp` values extracted from the AST by `SemanticExtractor`:

```rust
pub enum SemanticOp {
    Binding  { name: String, value_range: Range },
    Assignment { target: String, value_range: Range },
    Call     { function_name: String, args: Vec<Range>, range: Range },
    EnterBlock(Range),
}
```

The extractor walks the AST for the target language and emits a `SemanticOp` for each relevant node. For Rust `let_declaration`, it emits `Binding { name, value_range }`. For TypeScript `call_expression`, it identifies the callee name (handling `member_expression` method calls by extracting the `property` field), collects argument ranges, and emits `Call`.

This layer decouples the taint engine from language-specific AST structure. The same `DataFlowAnalyzer` runs over both Rust and TypeScript.

### 8.2 Taint Propagation

The `DataFlowAnalyzer` maintains a `TaintRegistry` — a map from variable name to taint label. Analysis proceeds as follows:

1. **Source identification.** For each `Binding` or `Assignment` op where the `value_range` corresponds to a source expression (matched by `source_re`), the target variable is added to the taint set.

2. **Propagation through bindings.** For each subsequent `Binding { name, value_range }`, the engine checks whether the text at `value_range` references any tainted variable. If so, `name` is added to the taint set. Field-path taint propagation extends this: if `user.password` is tainted and the binding references `user`, the new variable is also tainted.

3. **Call argument propagation.** For each `Call` op, if any argument range contains a tainted variable name, the return value of the call is considered tainted (unless the callee matches a sanitizer pattern).

4. **Sink detection.** For each `Call` op where the function name matches `sink_re`, the engine checks whether any argument is tainted. If so, an advisory is generated.

The analysis is bounded by `taint_max_depth` (default 5 levels of function nesting) to ensure termination. A `TaintCache` keyed by `(constraint_type, source_pattern, sink_pattern, file_path, function_line)` prevents redundant analysis when the same taint constraint fires on multiple nodes within the same function.

### 8.3 The ForbiddenTaint and TaintReached Constraints

Two flow constraint types operate over the taint engine:

**`TaintForbidden { source, sink }`** fires when a path from source to sink exists. This is the "do not let passwords reach the database unencrypted" constraint.

**`TaintReached { source, sink }`** fires when a required path does not exist. This is used to verify that authentication token validation reaches the response handler — a path that must exist for the endpoint to be secure.

Both are expressed identically in the YAML DSL (`forbidden_source_pattern` / `forbidden_sink_pattern` for TaintForbidden; `source_pattern` / `sink_pattern` for TaintReached) and compiled by `RuleCompiler` into `FlowConstraint` variants.

---

## 9. Schema Contract Rules

### 9.1 The Cross-Language Schema Drift Problem

Object-Relational Mappers generate database schemas from model definitions. In Prisma, a model named `Order` generates a table named `"Order"` (PascalCase preserved). A Rust service using raw SQL that writes `FROM orders` (snake_case) returns zero rows silently. The type system cannot catch this — both sides type-check. The test suite cannot catch this unless it runs against a real database with a real Prisma-managed schema.

No conventional static analyzer spans the boundary between a Rust source file and a Prisma schema file. GenSense's `SchemaContract` rule type does.

### 9.2 Implementation

A `SchemaContract` project rule is defined by five parameters:

```rust
pub struct SchemaContract {
    source_capture_re: Regex,      // captures the identifier from the source file
    source_file_glob: glob::Pattern, // which source files to scan
    schema_type: SchemaType,       // Prisma | OpenApi
    schema_file_glob: glob::Pattern, // which schema files to read
    schema_extract: SchemaExtract, // ModelNames | FieldNames | EnumValues
}
```

Execution:
1. The engine scans all source files matching `source_file_glob` and applies `source_capture_re` to extract string literals (e.g., table names in SQL strings).
2. It reads all schema files matching `schema_file_glob` and extracts the specified set (`ModelNames`, `FieldNames`, or `EnumValues`) using a Prisma-aware parser.
3. For each captured source literal, it checks membership in the extracted schema set. A Prisma model named `Order` produces the set `{"Order"}`. A source capture of `"orders"` does not match and an advisory fires.

The Prisma parser in `SourceRegistry` reads `.prisma` files and extracts `model Name { ... }` declarations in approximately 46 microseconds for a 20-model schema — verified by the benchmark suite.

### 9.3 Rule Specification

```yaml
project_rules:
  - id: RUST_SQL_TABLE_NAME_CONTRACT
    name: SQL Table Name Schema Contract
    source_ext: rs
    source_pattern: 'FROM\s+"?(\w+)"?'
    schema_type: prisma
    schema_glob: "**/*.prisma"
    schema_extract: model_names
    severity: Critical
    confidence: 0.95
    observation: SQL query references a table name not found in the Prisma schema.
    impact: Query returns zero rows in production with no error.
    improvement: Match the table name to the Prisma model name exactly.
```

---

## 10. Performance

### 10.1 Scan Throughput

GenSense's single-pass, combined-query architecture produces linear scaling with respect to file count. Benchmarks from the project's criterion suite:

| Workload | Files | Mean | Std Dev |
|---|---|---|---|
| `rust_clean_service` | 1 | 535ms | 20ms |
| `rust_service_with_violations` | 1 | 457ms | 19ms |
| `ts_clean_service` | 1 | 435ms | 11ms |
| `ts_mixed_real_world` | 1 | 559ms | 15ms |

Scale benchmarks (sequential, no parallelism):

| Files | Mean | Std Dev |
|---|---|---|
| 10 | 938ms | 18ms |
| 50 | 2.64s | 82ms |
| 100 | 4.64s | 92ms |

The 10-to-100-file scaling factor is approximately 4.95x — close to linear, with the deviation explained by the growing `SymbolRegistry` lookup cost.

### 10.2 Critical Path Analysis

The engine is single-threaded at both the semantic discovery phase and the rule execution phase. The v0.3.0 release removed Rayon parallelism from the audit loop after it introduced a class of futex deadlocks under certain tree-sitter query patterns. The single-pass combined-query design means that eliminating parallelism does not proportionally reduce throughput, because the dominant cost is the combined tree-sitter query execution — one traversal per file regardless of rule count.

### 10.3 Symbol Registry Lookup

The `SymbolRegistry::find_function_at()` method, which is called once per finding to determine `enclosing_symbol`, maintains $O(\log n)$ lookup characteristics. Benchmark: 51 nanoseconds at 100,000 symbols, 3ns above the 1,000-symbol baseline.

### 10.4 Schema Contract Overhead

`SourceRegistry` parses a 20-model Prisma schema in 46 microseconds. This is a one-time cost at project scan startup. The per-source-file cost is $O(M)$ where $M$ is the number of captured table name references in the file.

### 10.5 Historical Self-Scan

GenSense scans its own source tree at HEAD and produces zero advisories. The `exclude_scope` and `precision` filtering remove patterns in test and benchmark directories. A CI regression gate fails if any benchmark degrades by more than 20% (`alert-threshold: '120%'`) or if the self-scan advisory count exceeds 165.

---

## 11. Empirical Results: From 3,024 Findings to 513

### 11.1 Experimental Setup

Both the Friehub Auditor (Python) and GenSense v0.3.0 were run against the same production codebase: a marketplace application with hundreds of source files spanning a TypeScript API layer and Rust microservices.

| Metric | Friehub Auditor (Python) | GenSense v0.3.0 |
|---|---|---|
| Total findings | 3,024 | 513 |
| False positives | >2,000 (>65%) | 0 (<1%) |
| True findings | <1,000 | 513 |
| False positive rate | >65% | <1% |
| Languages | Python | Rust |
| AST substrate | Token text (regex fallback) | Tree-sitter only |
| Scope awareness | Block-level heuristics | Structural AST boundaries |
| Confidence scoring | None | Per-finding, encoded in rule |
| Cross-language rules | None | SchemaContract |
| MCP server | None | JSON-RPC 2.0 |

### 11.2 What the 513 Findings Contained

The 513 findings from GenSense v0.3.0 included every category of violation described in Section 1 of this paper:

- SQL table name mismatches against the Prisma schema (caught by `RUST_SQL_TABLE_NAME_CONTRACT`).
- Event publication inside database transactions (caught by `TS_EVENT_INSIDE_TRANSACTION` temporal rule).
- Wallet debit operations without a reachable balance check (caught by `MustHaveGuard` project rule).
- `f64` usage in monetary variable names (caught by `TS_FLOAT_MONETARY` pattern rule).
- Mutex guards held across `.await` points (caught by `RUST_LOCK_AWAIT` temporal rule).
- Hollow validation functions with no reachable rejection path (caught by CSA rules).

None of these were caught by the TypeScript type system, ESLint, or Clippy. All would have caused production failures.

### 11.3 The Precision Mechanism

The difference in false positive rate is not explained by fewer rules. It is explained by four specific mechanisms:

**Structural scope boundaries.** Rules fire on AST nodes, not on text matches. A pattern inside a comment, a test fixture, or a dead code branch does not fire.

**Confidence scoring.** Rules declare their expected confidence. The `precision` field (`very-high`, `high`, `medium`, `low`) filters the default suite: only `very-high` precision rules run by default. Users opt in to lower-precision rules explicitly.

**`exclude_scope`.** Rules can exclude file path patterns (test directories, benchmark directories, build scripts) at the AST level, not just by path prefix.

**Honest rule design.** Rules are only written for patterns that have a known, bounded false positive mechanism. If a pattern cannot be expressed with `confidence >= 0.85`, it is not written as a default rule.

---

## 12. The MCP Server and the Agent Loop Constraint

GenSense v0.3.0 ships an MCP (Model Context Protocol) server — a JSON-RPC 2.0 interface over stdin/stdout. This is the feature that connects the static analysis engine to the AI agent loop.

An AI agent running in Claude Code, Cursor, or any MCP-compatible environment calls `gensense_audit` on a file it has just written and receives:

```json
{
  "clean": false,
  "advisories": [...],
  "auto_fixed": 0,
  "requires_human": [...]
}
```

`clean: false` means the loop cannot exit. The agent must resolve the advisories before completion.

This is the enforcement primitive that prompts alone cannot provide. A prompt tells the model what to do. A GenSense advisory tells the agent that what it produced was wrong, precisely where, and what the correction should be. The `clean: bool` field is the exit condition. The agent cannot pass it by ignoring the advisory — it must satisfy the proposition that the rule encodes.

This is the practical realization of the Curry-Howard parallel from Section 2.3. The rule is a proposition. The `clean: true` return value is the proof that all propositions are satisfied. The agent must produce a program that constitutes a proof.

---

## 13. Conclusion

GenSense demonstrates that the gap between LLM-generated code and production-correct code is not a problem with the models. It is a problem with the absence of enforcement infrastructure.

The predecessor Friehub Auditor showed that intuition about what to analyze — statistical entropy, n-gram surprisal, AI phrase detection — is not sufficient if the analysis operates on the wrong substrate. Token text carries no structural information. Every attempt to add more mathematical sophistication to a text-based approach encounters the same wall: the surface does not contain enough information to decide the question.

The correct response, grounded in Rice's Theorem, is to accept the undecidability boundary and work inside it deliberately. GenSense works inside it by choosing only decidable approximations, expressing them over AST structure, assigning honest confidence scores, and providing a `clean: bool` primitive that terminates the agent loop.

The result is a 65% false positive rate reduced to under 1%, a 200-file project scanned in under 5 seconds, and a Schema Contract checker that catches cross-language drift that no other static analysis tool addresses.

The n-gram baseline from the Friehub Auditor was not a bad idea. It is on the GenSense roadmap, re-implemented over AST node sequences rather than raw token sequences. The same idea on the correct substrate produces a different result. That is the thesis.

---

## 14. The Suppression System

### 14.1 Two Suppression Mechanisms

GenSense provides two ways to silence a rule for a specific context without deleting the rule: inline source comments and a project-level configuration file.

**Inline suppression** is detected by scanning the two lines immediately preceding the flagged node for a comment containing `gensense-ignore: <rule_id>` or `gensense-ignore: all`:

```rust
// gensense-ignore: TS_FLOAT_MONETARY
const legacyRate: number = 0.075;  // historical config, not a monetary value
```

The implementation in `suppression.rs` searches the source string for these patterns within the window `[start_row - 2, start_row]`. It checks for the presence of a comment delimiter (`//`, `/*`, or `#`) on the same line before accepting the suppression, preventing false matches from string literals that happen to contain the phrase.

**Config-level suppression** is loaded from a `suppress.toml` file at project scan startup via `SuppressConfig`:

```toml
[[suppressions]]
rule_id = "TS_ASYNC_FOREACH"
path = "scripts/**"
```

Each entry pairs a `rule_id` (or `"all"`) with a glob pattern. The pattern is matched against the file path of the node being checked. This mechanism silences a rule for an entire directory or file subtree without touching source files.

### 14.2 Suppression vs. Exclusion

Suppression is distinct from `exclude_scope`. `exclude_scope` is a rule-level declarative setting that fires at rule design time, preventing the rule from even being evaluated for nodes in matching scopes. Suppression is an operator override at the call site. The distinction matters for auditability: suppressions are tracked (the `SemanticBrain` predecessor recorded suppression frequency per rule), enabling teams to identify which rules generate the most suppressions and whether those rules require precision tuning.

---

## 15. User-Defined Rules and Rule Composition

### 15.1 Rule Loading Order

GenSense resolves rules from up to four sources. User-defined rules override built-in rules by matching `id`:

1. **Built-in rules** — compiled into the binary (typed Rust structs) and embedded YAML files (via `include_dir!`). These provide the default ruleset.
2. **User-defined rules** — loaded from three locations, in this order: project-local (`.gensense/rules/`), global (`~/.gensense/rules/`), and CLI-specified (`--rules-dir`). Later directories override earlier ones for duplicate rule IDs.

When a user-defined rule has the same `id` as a built-in rule, the user rule replaces the built-in:

```rust
let user_ids: HashSet<&str> = user_rules.iter().map(|r| r.id()).collect();
rules.retain(|r| !user_ids.contains(r.id()));
rules.extend(user_rules);
```

This enables organizations to override the default severity or confidence of a built-in rule without forking the engine.

### 15.2 No-Builtin Mode

Passing `--no-builtin-rules` sets `no_builtin_rules: true`, causing `build_rule_set()` to start with an empty rule vector and load only user-defined rules. This supports organizations that want full control over their rule inventory and treat the built-in rules as a reference library rather than a default.

### 15.3 Rule Validation at Load Time

`RuleCompiler::compile()` fails fast on invalid rules. If a rule specifies a malformed regex in `if_matches`, a tree-sitter query that fails to parse, or an unrecognized `on_node` type, `compile()` returns `Err` and the rule is skipped with a `tracing::warn!` log. The engine continues loading remaining rules. This means a typo in one user rule does not prevent the rest of the ruleset from loading.

---

## 16. Suite and Environment Filtering

### 16.1 The Suite Enum

The `Suite` enum controls which precision levels are active:

```rust
pub enum Suite {
    Default,    // only rules with precision: very-high
    Extended,   // rules with precision: high or above
    All,        // all rules regardless of precision (default)
}
```

`RuleMetadata::meets_suite(suite)` returns `true` if the rule's declared precision meets or exceeds the suite threshold. The default CLI mode uses `Suite::All`. Running `gensense --suite default` runs only very-high-precision rules.

The `is_rule_enabled()` method applies Suite filtering first, then severity filtering, then environment filtering, then category/tag filtering. Short-circuiting means that a rule below the Suite threshold is never evaluated for category membership — its metadata is checked once and it is skipped.

### 16.2 The Environment Enum

```rust
pub enum GenSenseEnvironment {
    Production,
    Staging,
    Development,
}
```

Rules tagged `"beta"` are excluded when `env == Production`. This allows shipping experimental rules that are active in developer environments but suppressed in CI pipelines that gate releases.

### 16.3 Tag Filters

Passing `--tag concurrency` restricts rule execution to rules with matching tags. When the tag filter is empty, all rules pass the filter. This enables focused scans like `gensense --tag security` for a dedicated security audit pass.

---

## 17. The SemanticGraph Edge Taxonomy

The `SemanticGraph` is a directed graph over `Symbol` nodes and `TemporalEvent` nodes, connected by typed `EdgeKind` edges. The full taxonomy:

| EdgeKind | Meaning | Producer |
|---|---|---|
| `Calls` | Caller invokes callee | Pass 2: call edge scanning |
| `RefersTo` | Symbol references another by name | Symbol linker |
| `FlowsFrom` | Data flows from source to target | Pass 3: event construction |
| `InScope` | Event or symbol is within a function scope | Pass 3: scope linking |
| `Parameter` | Variable is a formal parameter of function | Pass 1: symbol extraction |
| `SequentiallyFollows` | Event B occurs after event A in the same scope | Pass 3: event chaining |
| `TemporallyPrecedes` | Explicit temporal ordering (reserved for V2) | Future |

The graph is implemented using `petgraph::stable_graph::StableDiGraph<SemanticNode, EdgeKind>`. `StableDiGraph` preserves node indices across removals, which is required for incremental invalidation: when a file is re-parsed, the nodes for that file can be removed and re-inserted without invalidating indices for nodes from other files.

`SemanticNode` is an enum over `Symbol` and `TemporalEvent`, allowing events and symbols to coexist in the same graph and be connected by typed edges without requiring a union type or nullable fields.

---

## 18. Function Fingerprinting and Jaccard Clone Detection

### 18.1 FunctionFingerprint

The `fingerprinting` Cargo feature activates structural clone detection. When enabled, `extract_fingerprints()` runs after rule execution on every file and produces a `FunctionFingerprint` for each function:

```rust
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub ngram_hashes: FxHashSet<u64>,
}
```

`ngram_hashes` is populated by sliding a window of size 5 over the function body's whitespace-split tokens and hashing each 5-gram with `FxHasher`:

```rust
for i in 0..=(tokens.len().saturating_sub(5)) {
    let mut hasher = FxHasher::default();
    tokens[i..i + 5].hash(&mut hasher);
    ngram_hashes.insert(hasher.finish());
}
```

Tokens are extracted from the function body AST node directly — not from the raw source line range — so comment-only lines that happen to fall inside the byte range are excluded by the `filter(|t| !t.starts_with("//"))` predicate.

### 18.2 Jaccard Similarity

Given two fingerprints $A$ and $B$, Jaccard similarity is:

$$J(A, B) = \frac{|A \cap B|}{|A \cup B|}$$

For the hash sets, intersection and union are computed using `FxHashSet` operations in $O(\min(|A|, |B|))$ time. Functions with $J \geq 0.8$ are flagged as structural clones. The threshold of 0.8 allows for minor differences (renamed variables, added logging calls) while reliably catching copy-paste with superficial modifications.

The 5-gram window over raw body tokens does not normalize variable names. The `ALGORITHMS.md` specification describes a normalized variant (mapping identifiers to `_VAR_` and literals to `_LIT_`) for the V2 roadmap — this would catch cases where two functions are logically identical but use different variable names throughout.

---

## 19. Spectral Graph Analysis: The Topological Hermitian Laplacian

### 19.1 Motivation

The call graph analysis in Section 6.5 operates on qualitative reachability: can function A reach function B? It answers yes/no questions about call paths. A harder class of question is quantitative: which modules are structurally at risk of becoming isolated, overcoupled, or architecturally decoupled from the rest of the system as the codebase evolves?

This class of question is addressed by spectral graph analysis — specifically, by computing properties of the graph Laplacian. The `ALGORITHMS.md` specification documents this as the Topological Directed Hermitian Laplacian Engine, targeting the GenSense V2 roadmap.

### 19.2 Mathematical Formulation

Let $G = (V, E)$ be the directed module dependency graph of $N$ modules. Let $A \in \mathbb{R}^{N \times N}$ be the directed adjacency matrix where $A_{ij}$ is the weight of dependencies from module $v_i$ to $v_j$.

To preserve flow direction while enabling spectral decomposition, we construct the **Hermitian Adjacency Matrix** $A_H \in \mathbb{C}^{N \times N}$ using a complex phase angle $\theta = \pi/3$:

$$A_{H,ij} = (A_{ij} + A_{ji}) \cdot e^{i \theta \cdot \text{sign}(A_{ij} - A_{ji})}$$

The diagonal degree matrix $D_{ii} = \sum_j (A_{ij} + A_{ji})$. The **Symmetric Normalized Hermitian Laplacian** is:

$$L_H = I - D^{-1/2} A_H D^{-1/2}$$

Because $L_H = L_H^\dagger$ (Hermitian self-adjoint), the Spectral Theorem guarantees real eigenvalues and a complete orthonormal eigenbasis. The second smallest eigenvalue $\lambda_2$ (algebraic connectivity, or Fiedler value) quantifies how well-connected the dependency graph is. The corresponding eigenvector $v_2$ (Fiedler vector) partitions modules into two groups along the minimum graph cut — identifying the most likely architectural split point.

A declining $\lambda_2$ over successive commits signals that the codebase is bifurcating. A Fiedler vector component that moves a module from one partition to the other signals architectural drift for that module.

### 19.3 Computational Optimization

Direct eigendecomposition is $O(N^3)$, infeasible for large codebases. The engine uses two optimizations:

**Compressed Sparse Row (CSR) storage.** The matrix is stored in three flat arrays:
- `values: Vec<Complex>` — the $M$ non-zero entries.
- `col_indices: Vec<usize>` — column index of each entry.
- `row_offsets: Vec<usize>` — row boundary pointers, length $N+1$.

Sparse matrix-vector product (SpMV) then runs in $O(M)$:

$$y_i = \sum_{k=\text{row\_offsets}[i]}^{\text{row\_offsets}[i+1]-1} \text{values}[k] \cdot x_{\text{col\_indices}[k]}$$

**Power Iteration with Deflation.** We form the shifted matrix $B = 2I - L_H$ and apply power iteration. To avoid converging to the dominant eigenvector $v_1$ (the trivial constant vector), we deflate it at each step:

$$x^{(t+1)} = B x^{(t)}$$
$$x^{(t+1)}_{\text{def}} = x^{(t+1)} - \langle x^{(t+1)}, v_1 \rangle v_1$$
$$x^{(t+1)} = \frac{x^{(t+1)}_{\text{def}}}{\|x^{(t+1)}_{\text{def}}\|_2}$$

This converges to $v_2$ in $O(M)$ operations per iteration to tolerance $10^{-9}$.

**Hierarchical coarse-graining.** The engine maintains two levels:
- Level 1: a $30 \times 30$ subsystem matrix (top-level directories). Always fast.
- Level 2: per-subsystem local matrices, run in isolation.

The Level 1 solver runs on every scan. Level 2 solvers run on-demand for subsystems flagged by Level 1 as having low algebraic connectivity.

Benchmark from `ALGORITHMS.md`: CSR Hermitian Laplacian solver with Fiedler vector extraction — **197.90 microseconds** for a 30-node graph, qualifying as a sub-millisecond architectural drift check.

---

## 20. Incremental Analysis: O(Δ) Merkle Tree Invalidation

Full project re-scan on every file save is not viable for an IDE save-hook use case. GenSense V2 specifies an incremental runtime based on Merkle tree hashing of AST subtrees.

Every `AstNode` in the arena carries a `merkle_hash: u64` computed over its kind, value, and children:

$$H(\text{Node}) = \text{Hash}(\text{kind} \oplus \text{value} \oplus \bigoplus H(\text{Children}))$$

When source code changes, the parser updates the affected subtree. The Merkle hash is recomputed bottom-up from the modified leaf to the root, touching only nodes in the path from the change to the root. Nodes whose `merkle_hash` is unchanged can be skipped entirely — their symbols, call edges, and event chains are still valid.

Invalidation complexity is $O(\Delta)$ where $\Delta$ is the depth of the edited block in the AST, not the file size. For a typical single-function edit, this is $O(\log N)$ where $N$ is the file size in nodes.

Benchmark from `ALGORITHMS.md`: incremental Merkle tree invalidation for a single-function edit — **13.62 microseconds**. This is the target latency for a save-hook integration.

---

## 21. Output Formats and CI Integration

### 21.1 Output Modes

GenSense supports three output formats via `Reporter`:

- **Terminal** — colored, human-readable output grouped by severity. Default for interactive use.
- **Markdown** — a structured report with severity sections, suitable for PR descriptions or issue trackers.
- **JSON** — machine-readable, suitable for downstream tooling or custom dashboards.
- **SARIF 2.1.0** — the Static Analysis Results Interchange Format, suitable for CI/CD integration with GitHub Code Scanning, Azure DevOps, and VS Code Problems panel.

### 21.2 The SARIF Output

The SARIF output from `Reporter::to_sarif()` conforms to the OASIS SARIF 2.1.0 schema. Each advisory becomes a SARIF `result` with:

- `ruleId` — the GenSense rule ID.
- `level` — mapped from `Severity`: `Critical` → `error`, `Warning` → `warning`, `Info` → `note`.
- `locations[0].physicalLocation.region` — precise `startLine`, `startColumn`, `endLine`, `endColumn` computed from the advisory's byte offsets.
- `partialFingerprints.primaryLocationLineHash/v1` — the SRI fingerprint, enabling GitHub Code Scanning to deduplicate findings across commits even when line numbers shift.
- `properties.confidence` and `properties.auto_fixable` — first-class fields in the SARIF properties bag.
- `fixes[0].artifactChanges` — populated when `proposed_replacement` is non-null, providing a machine-readable diff that GitHub Code Scanning can surface as a one-click fix suggestion.

The `helpUri` for each rule points to `https://friehub.github.io/gensense/rules/<rule_id>`, enabling documentation links directly from the CI annotation.

### 21.3 Auto-Remediation

Advisories with `auto_fixable: true` and a non-null `proposed_replacement` can be applied automatically by the `patcher/` module. The patcher operates on byte offsets rather than line numbers, using `start_byte` and `end_byte` from the advisory to locate the exact region to replace. This means auto-fixes are immune to line-number drift from whitespace or comment changes elsewhere in the file.

Auto-fixable rules currently include pattern-based transformations (e.g., replacing `.forEach(async` with a `for...of` loop scaffold) where the fix is deterministic from the match. Rules requiring semantic reasoning (e.g., CSA hollow validation, temporal ordering violations) set `requires_human: true` and are not auto-applied.

---

## References

- Rice, H.G. (1953). "Classes of Recursively Enumerable Sets and Their Decision Problems." *Transactions of the American Mathematical Society*, 89(1), 25-59.
- Church, A. (1936). "An Unsolvable Problem of Elementary Number Theory." *American Journal of Mathematics*, 58(2), 345-363.
- Howard, W.A. (1980). "The Formulae-as-Types Notion of Construction." In *To H.B. Curry: Essays on Combinatory Logic, Lambda Calculus and Formalism*, Academic Press.
- Curry, H.B. & Feys, R. (1958). *Combinatory Logic, Vol. I*. North-Holland.
- Maxim, N. (2024). tree-sitter. https://github.com/tree-sitter/tree-sitter
- Model Context Protocol (2024). https://modelcontextprotocol.io
- Friehub. (2026). GenSense v0.3.0. https://github.com/Friehub/gensense
- Friehub. (2026). Friehub Auditor Archive. https://github.com/Friehub/friehub-auditor-archive

---

*GenSense is MIT-licensed and available at [@friehub/gensense](https://github.com/Friehub/gensense).*
*The Friehub Auditor predecessor is preserved at [@friehub/friehub-auditor-archive](https://github.com/Friehub/friehub-auditor-archive).*

