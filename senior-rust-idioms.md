# Senior Rust Idioms — A Complete Reference for LLMs

> This document teaches how experienced Rust engineers think and write code.
> Every rule here has a "why" — senior engineers don't follow rules blindly,
> they follow them because they understand the consequence of not doing so.
> When generating Rust, apply every principle here by default unless the
> context explicitly overrides it.

---

## 0. The Mental Model

Before any specific rule: understand how a senior Rust engineer reads code.

They ask four questions in order:

1. **What is the contract?** — What does this type promise? What are the valid states?
2. **Who owns what?** — Who is responsible for cleanup? Who can mutate?
3. **What can fail, and how?** — Is failure visible in the type? Is it handled or deferred?
4. **What does this cost?** — Does this allocate? Does this block? Does this clone?

Code that makes these four questions easy to answer is considered good Rust.
Code that makes them hard is considered unidiomatic regardless of whether it compiles.

---

## 1. API Visibility — Default to Private

### The Rule

Fields and methods are `pub` only when external callers genuinely need them.
Everything else is private. This is not about secrecy — it is about defining a contract.

### Why Seniors Care

A `pub` field is a promise that the type will always have that field with that type.
Changing it is a breaking change. Private fields can be refactored freely.
When everything is `pub`, there is no API — there is just a struct.

### What LLMs Typically Write (Wrong)

```rust
pub struct TaintRegistry<'a> {
    pub scopes: Vec<HashMap<&'a str, &'a str>>,
    pub symbols: Vec<HashMap<&'a str, Node<'a>>>,
    pub depth: usize,
}
```

### What a Senior Writes

```rust
/// Tracks taint origins across nested lexical scopes during data flow analysis.
pub struct TaintRegistry<'a> {
    scopes: Vec<HashMap<&'a str, TaintOrigin>>,
    symbols: Vec<HashMap<&'a str, Node<'a>>>,
}

impl<'a> TaintRegistry<'a> {
    pub fn taint(&mut self, var: &'a str, origin: TaintOrigin) { ... }
    pub fn get_origin(&self, var: &str) -> Option<TaintOrigin> { ... }
    pub fn push_scope(&mut self) { ... }
    pub fn pop_scope(&mut self) { ... }
}
```

### The Test

Ask: "If I change this field's type, how many files break outside this module?"
If the answer is more than zero, that field should be private with an accessor method.

---

## 2. Types Over Strings — Make Invalid States Unrepresentable

### The Rule

When a value has a fixed set of valid states, use an enum.
When a string carries semantic meaning, wrap it in a newtype.
Never use raw strings as discriminants in business logic.

### Why Seniors Care

Stringly-typed code fails at runtime with confusing messages.
Type-safe code fails at compile time with a clear error.
The compiler is the best reviewer — give it enough information to help.

### What LLMs Typically Write (Wrong)

```rust
// var -> origin, where origin is always the string "source"
pub scopes: Vec<HashMap<&'a str, &'a str>>,

fn analyze(&self, kind: &str) {
    if kind == "taint" { ... }
    else if kind == "flow" { ... }
    // What other values are valid? Nobody knows.
}
```

### What a Senior Writes

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaintOrigin {
    UserInput,
    EnvVar,
    FileRead,
    NetworkResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisKind {
    Taint,
    DataFlow,
}

fn analyze(&self, kind: AnalysisKind) {
    match kind {
        AnalysisKind::Taint => { ... }
        AnalysisKind::DataFlow => { ... }
        // Compiler warns if a new variant is added and this match isn't updated
    }
}
```

### Newtype Pattern for Domain Semantics

```rust
// Wrong — two usize values, easy to swap
fn find_at(file_id: usize, line: usize) -> Option<Symbol> { ... }

// Right — distinct types, impossible to swap
pub struct FileId(u32);
pub struct LineNumber(u32);

fn find_at(file: FileId, line: LineNumber) -> Option<Symbol> { ... }
```

---

## 3. Error Handling — Errors Are Part of the API

### The Rule

Use `Result<T, E>` with a meaningful `E` for anything that can fail.
Use `?` to propagate. Never `unwrap()` in library code without a documented invariant.
Never `panic!` in a function that a caller cannot control.

### Why Seniors Care

A panic in library code crashes the caller's entire process.
The caller had no opportunity to handle it. This is considered hostile API design.
`unwrap()` without a comment says "I didn't think about this failing."

### What LLMs Typically Write (Wrong)

```rust
fn load_config(path: &Path) -> Config {
    let content = std::fs::read_to_string(path).unwrap(); // panics in production
    serde_json::from_str(&content).unwrap();               // panics on bad JSON
}

fn get_node(idx: usize) -> Node {
    self.nodes[idx] // panics on out-of-bounds
}
```

### What a Senior Writes

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Cannot read config file at {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("Config file contains invalid JSON: {0}")]
    Parse(#[from] serde_json::Error),
}

fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::Io { path: path.to_owned(), source: e })?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

fn get_node(&self, idx: usize) -> Option<&Node> {
    self.nodes.get(idx) // returns None, never panics
}
```

### The Documented Invariant Exception

`unwrap()` is acceptable exactly when you can prove it cannot fail and you document why:

```rust
// SAFETY: We only call this after `validate_headers()` has confirmed the
// Authorization header is present. The unwrap cannot fail here.
let token = headers.get("Authorization").unwrap();

// Or use expect() with a message that helps debug the invariant violation:
let root = tree.root_node()
    .descendant_for_byte_range(start, end)
    .expect("byte range was validated against this tree's source length");
```

### Error Type Design

```rust
// Wrong — all errors are strings, callers can't match on them
fn parse(input: &str) -> Result<Ast, String> { ... }

// Right — callers can pattern-match and handle specific cases
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token '{token}' at line {line}")]
    UnexpectedToken { token: String, line: usize },
    #[error("Unterminated string literal starting at line {0}")]
    UnterminatedString(usize),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

---

## 4. Mutation — Be Explicit, Minimize Scope

### The Rule

Mutation should be local and obvious. Prefer returning new values over mutating
through parameters. When mutation is necessary, keep the `mut` binding as short-lived
as possible.

### Why Seniors Care

Hidden mutation through output parameters (`&mut Vec`) makes code hard to reason about.
You cannot tell at the call site that the function changes the argument.
The Rust idiom is to make data flow explicit through return values.

### What LLMs Typically Write (Wrong)

```rust
// C-style output parameter — mutation is hidden at the call site
fn extract_findings(node: Node, source: &str, out: &mut Vec<Advisory>) {
    if matches_pattern(node, source) {
        out.push(Advisory::new(node));
    }
    for child in node.children(&mut node.walk()) {
        extract_findings(child, source, out); // mutation buried in recursion
    }
}

// Called as:
let mut advisories = Vec::new();
extract_findings(root, source, &mut advisories); // reader must know this mutates
```

### What a Senior Writes

```rust
fn extract_findings(node: Node, source: &str) -> Vec<Advisory> {
    let mut findings = Vec::new();
    if matches_pattern(node, source) {
        findings.push(Advisory::new(node));
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        findings.extend(extract_findings(child, source));
    }
    findings
}

// Called as:
let advisories = extract_findings(root, source); // data flow is obvious
```

### When `&mut` Output Parameters Are Acceptable

Only when performance is critical and you are accumulating into a pre-allocated
buffer across many recursive calls, and you have measured that allocation is the
bottleneck. Always document why:

```rust
/// Accumulates fingerprints into `out` to avoid repeated allocation across
/// the recursive tree walk. Callers should pass a pre-allocated `Vec`.
fn extract_fingerprints(node: Node, source: &str, out: &mut Vec<Fingerprint>) { ... }
```

---

## 5. Struct Design — Builder Pattern and Options Types

### The Rule

Functions with more than four or five parameters should take a struct.
Functions with boolean flags should take an enum or options struct, never bare `bool`.
Use the builder pattern when construction has many optional fields.

### Why Seniors Care

Function signatures with many parameters are impossible to call correctly from memory.
Boolean parameters at call sites are unreadable: `audit(path, true, false, true)` —
what do those booleans mean? The caller must read the signature to find out.

### What LLMs Typically Write (Wrong)

```rust
#[allow(clippy::too_many_arguments)] // ← the suppression is the red flag
pub fn audit(
    &self,
    file_id: FileId,
    path: &Path,
    content: &str,
    tree: &Tree,
    semantic_ops: &[SemanticOp],
    symbols: &SymbolRegistry,
    category_filter: &HashSet<String>,
    tag_filter: &HashSet<String>,
    env: GenSenseEnvironment,
    verbose: bool,
    include_info: bool,
) -> Result<ScanResult>
```

### What a Senior Writes

```rust
/// Controls which rules run during a scan and how results are reported.
#[derive(Debug, Default)]
pub struct AuditOptions {
    /// Only emit findings in these categories. Empty means all categories.
    pub category_filter: HashSet<String>,
    /// Only emit findings with these tags. Empty means all tags.
    pub tag_filter: HashSet<String>,
    /// The deployment environment — gates beta rules.
    pub env: GenSenseEnvironment,
    /// Emit Info-level findings in addition to Warning and Critical.
    pub include_info: bool,
}

pub fn audit<'a>(
    &self,
    file_id: FileId,
    path: &'a Path,
    content: &'a str,
    tree: &'a Tree,
    semantic_ops: &'a [SemanticOp],
    symbols: &'a SymbolRegistry,
    options: &AuditOptions,
) -> Result<ScanResult>
```

### Builder Pattern for Optional Construction

```rust
pub struct Advisory {
    rule_id: String,
    severity: Severity,
    file_path: String,
    line: u32,
    column: u32,
    observation: String,
    // Optional fields
    proposed_replacement: Option<String>,
    proposed_import: Option<String>,
    enclosing_symbol: Option<String>,
}

pub struct AdvisoryBuilder {
    rule_id: String,
    severity: Severity,
    file_path: String,
    line: u32,
    column: u32,
    observation: String,
    proposed_replacement: Option<String>,
    proposed_import: Option<String>,
    enclosing_symbol: Option<String>,
}

impl AdvisoryBuilder {
    pub fn new(rule_id: impl Into<String>, severity: Severity, ...) -> Self { ... }

    pub fn with_replacement(mut self, replacement: impl Into<String>) -> Self {
        self.proposed_replacement = Some(replacement.into());
        self
    }

    pub fn with_enclosing_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.enclosing_symbol = Some(symbol.into());
        self
    }

    pub fn build(self) -> Advisory { ... }
}

// Call site reads like a sentence:
let advisory = AdvisoryBuilder::new("RUST_UNWRAP_SAFETY", Severity::Warning, ...)
    .with_replacement("unwrap_or_else(|| default_value())")
    .with_enclosing_symbol("parse_config")
    .build();
```

---

## 6. Iterators — Prefer Chains Over Loops

### The Rule

When transforming, filtering, or collecting a sequence, use iterator chains.
Imperative loops with `mut` accumulators are acceptable but should have a clear
reason over the iterator equivalent.

### Why Seniors Care

Iterator chains are declarative — they say *what* you want, not *how* to get it.
They compose, they short-circuit correctly, and they often optimize better.
A `for` loop with a `mut` accumulator inside is also fine — this is not dogma —
but gratuitous loops where an iterator reads more clearly are a style signal.

### What LLMs Typically Write (Wrong)

```rust
let mut results = Vec::new();
for symbol in self.graph.all_symbols() {
    if re.is_match(&symbol.name) {
        results.push(symbol);
    }
}
results
```

### What a Senior Writes

```rust
self.graph
    .all_symbols()
    .filter(|s| re.is_match(&s.name))
    .collect()
```

### More Complex Chains — Readable Line Breaks

```rust
// Wrong — one long line, unreadable
let callers = self.graph.graph.edges_directed(idx, petgraph::Direction::Incoming).filter(|e| *e.weight() == EdgeKind::Calls).filter_map(|e| self.graph.get_symbol(e.source())).collect::<Vec<_>>();

// Right — each operation on its own line
let callers = self.graph
    .edges_directed(idx, petgraph::Direction::Incoming)
    .filter(|e| *e.weight() == EdgeKind::Calls)
    .filter_map(|e| self.graph.get_symbol(e.source()))
    .collect::<Vec<_>>();
```

### When to Use a Loop Instead

Use a `for` loop when:
- You need to `break` or `continue` conditionally mid-iteration
- You are accumulating into multiple output variables simultaneously
- The body has side effects that depend on external mutable state
- The iterator equivalent would require `.enumerate()` plus complex index arithmetic

```rust
// Loop is correct here — updating two separate accumulators with early exit
let mut found = None;
let mut count = 0;
for node in nodes {
    count += 1;
    if node.matches_target() {
        found = Some(node);
        break; // .find() would work, but we need count too
    }
}
```

---

## 7. Lifetime Design — Name Lifetimes Meaningfully

### The Rule

When a struct or function has multiple lifetime parameters, name them after what
they represent, not `'a`, `'b`. Single-lifetime structs may use `'a` by convention.

### Why Seniors Care

`'a` and `'b` in a complex struct signature tell you nothing about the relationship
between the lifetimes. Named lifetimes document the intent.

### What LLMs Typically Write (Wrong)

```rust
pub struct DataFlowAnalyzer<'a, 'b> {
    pub context: &'b GenSenseContext<'a>,
    pub root: Node<'a>,
    pub depth: usize,
}
```

### What a Senior Writes

```rust
pub struct DataFlowAnalyzer<'src, 'ctx>
where
    'src: 'ctx, // source code outlives the analysis context
{
    /// The analysis context for the current file being scanned.
    context: &'ctx GenSenseContext<'src>,
    /// The AST root node for the scope being analyzed.
    root: Node<'src>,
    /// Current recursion depth, checked against `max_depth`.
    depth: usize,
    max_depth: usize,
}
```

---

## 8. Trait Design — Small, Focused Traits

### The Rule

Traits should have one responsibility. A trait with ten methods is almost always
several smaller traits combined. Provide default implementations where the behaviour
is derivable from other methods in the same trait.

### Why Seniors Care

Fat traits are hard to implement correctly and hard to mock in tests.
Small traits compose — you can `impl Display + Debug + Clone` as a bound.
The standard library sets the example: `Iterator` has one required method,
everything else is a default implementation built on it.

### What LLMs Typically Write (Wrong)

```rust
pub trait GenSenseRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;
    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory>;
    fn applies_to(&self, extension: &str) -> bool;
    fn query(&self) -> Option<&str>;
    fn id(&self) -> &str;  // this is just metadata().id — why require implementors to write it?
    fn new_advisory(&self, ...) -> Advisory; // this is a helper, not a contract
    fn new_remediated_advisory(&self, ...) -> Advisory; // ditto
}
```

### What a Senior Writes

```rust
/// A semantic analysis rule that can be applied to individual AST nodes.
///
/// Implement only `metadata` and `check`. All other methods have correct
/// default implementations derived from these two.
pub trait GenSenseRule: Send + Sync {
    /// Static metadata: ID, severity, human-readable text, tags.
    fn metadata(&self) -> &RuleMetadata;

    /// Inspect `node` and return any findings. Return an empty Vec if the
    /// node does not violate this rule. Never panic — unknown node kinds
    /// should return `vec![]`.
    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory>;

    // --- Default implementations — implementors should not override these ---

    fn id(&self) -> &str {
        self.metadata().id.as_ref()
    }

    fn applies_to(&self, extension: &str) -> bool {
        self.metadata().target_extensions.contains(extension)
    }

    /// Optional tree-sitter query. When `Some`, the engine skips nodes that
    /// don't match, which is significantly faster than a full tree walk.
    fn query(&self) -> Option<&str> {
        None
    }
}

// Helper functions live as free functions in the module, not trait methods.
// This way they don't pollute the trait's required interface.
pub fn make_advisory(rule: &dyn GenSenseRule, node: &Node, context: &GenSenseContext, observation: String) -> Advisory {
    ...
}
```

---

## 9. `match` — Exhaustive, No Wildcards That Hide Cases

### The Rule

`match` arms should cover every case explicitly. Wildcard `_` arms are acceptable
only for genuinely uninteresting cases, and even then, a comment explains why.
A wildcard that silently ignores a new enum variant is a latent bug.

### Why Seniors Care

The compiler's exhaustiveness check is a superpower. A new enum variant added
without updating all match arms becomes a compile error. A wildcard arm silences
that check for all future variants.

### What LLMs Typically Write (Wrong)

```rust
match node.kind() {
    "function_item" => handle_function(node),
    "struct_item" => handle_struct(node),
    _ => {} // silently ignores ALL other node kinds, including new ones
}

match severity {
    Severity::Critical => emit_critical(adv),
    _ => emit_warning(adv), // Severity::Info is now treated as Warning — silent bug
}
```

### What a Senior Writes

```rust
match node.kind() {
    "function_item" => handle_function(node),
    "struct_item" => handle_struct(node),
    // tree-sitter returns hundreds of node kinds (punctuation, keywords, etc.)
    // We intentionally ignore non-declaration nodes here.
    _ => {}
}

// For domain enums you control, always be exhaustive:
match severity {
    Severity::Critical => emit_critical(adv),
    Severity::Warning => emit_warning(adv),
    Severity::Info => {
        if options.include_info {
            emit_info(adv);
        }
    }
}
```

### Using `#[non_exhaustive]` on Enums You Publish

```rust
// If external crates match on this enum, adding a variant is a breaking change.
// #[non_exhaustive] forces them to have a wildcard arm, making future variants safe to add.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}
```

---

## 10. Documentation — Every Public Item Has a Doc Comment

### The Rule

Every `pub` type, every `pub` function, every `pub` trait, every `pub` field
has a `///` doc comment. The comment explains *why* the item exists and any
non-obvious invariants, not just *what* it is (the name already says that).

### Why Seniors Care

Documentation is part of the API contract. `cargo doc` is the first thing a new
contributor runs. Empty doc pages signal an unfinished codebase.
Comments that restate the function name (`/// Gets the id. Returns the id.`)
are worse than no comment because they waste the reader's attention.

### What LLMs Typically Write (Wrong)

```rust
pub struct SymbolRegistry { ... }          // no doc
pub fn find_at(&self, ...) -> Option<...>  // no doc

/// Gets the symbol.
pub fn get_symbol(&self, idx: NodeIndex) -> Option<&Symbol> { ... } // restates the name
```

### What a Senior Writes

```rust
/// The project-wide index of all discovered symbols across all scanned files.
///
/// Backed by a [`SemanticGraph`] for relationship queries (callers, callees,
/// inheritance) and a per-file index for fast location-based lookups.
///
/// Constructed during the discovery pass and then treated as immutable during
/// rule evaluation. Do not call `insert` after the discovery pass completes.
pub struct SymbolRegistry { ... }

/// Finds the innermost symbol named `name` that contains `line` in `file`.
///
/// "Innermost" is defined as the symbol with the smallest line span that still
/// contains `line`, which handles variable shadowing correctly: a local variable
/// is found before the function parameter with the same name.
///
/// Returns `None` if no symbol with that name exists at that location.
pub fn find_at(&self, name: &str, file: &str, line: usize) -> Option<&Symbol> { ... }
```

---

## 11. Recursion — Prefer Iterative Tree Walks

### The Rule

Recursive functions over tree structures risk stack overflow on deep inputs.
Rust's default stack is 8MB and async tasks have even smaller stacks.
The tree-sitter `TreeCursor` API is designed for iterative traversal — use it.

### Why Seniors Care

A pathologically deep AST (deeply nested closures, generated code, macros) will
silently overflow the stack. The error is a segfault or OS signal, not a Rust panic —
it cannot be caught or recovered from.

### What LLMs Typically Write (Wrong)

```rust
fn run_recursive<'a>(&self, node: Node<'a>, rule: &dyn GenSenseRule, context: &GenSenseContext<'a>) -> Vec<Advisory> {
    let mut advisories = rule.check(node, context); // allocates Vec at every node
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        advisories.extend(self.run_recursive(child, rule, context)); // recursion + extend = O(N²) allocations
    }
    advisories
}
```

### What a Senior Writes

```rust
fn walk_tree<'a>(
    &self,
    root: Node<'a>,
    rule: &dyn GenSenseRule,
    context: &GenSenseContext<'a>,
    out: &mut Vec<Advisory>, // one allocation, threaded through the walk
) {
    let mut cursor = root.walk();

    // Iterative pre-order traversal using tree-sitter's cursor API
    loop {
        let node = cursor.node();

        // Visit current node
        if !self.is_suppressed(node, rule.id(), context) {
            out.extend(rule.check(node, context));
        }

        // Descend into children first
        if cursor.goto_first_child() {
            continue;
        }

        // No children: try next sibling
        loop {
            if cursor.goto_next_sibling() {
                break; // found a sibling, continue outer loop
            }
            // No sibling: ascend
            if !cursor.goto_parent() {
                return; // back at root, traversal complete
            }
        }
    }
}
```

---

## 12. Hashing — Use Deterministic Hashers for Persistent Data

### The Rule

`std::collections::hash_map::DefaultHasher` is randomized per-process for HashDoS
protection. Never use it to produce hashes stored in files, databases, or compared
across runs. Use a deterministic hasher for any persistent or cross-run comparison.

### Why Seniors Care

Code that uses `DefaultHasher` for persistent fingerprints produces different values
on every run. This silently invalidates cached baselines, making diff-based features
useless without any error. The bug is invisible until you notice the cache never hits.

### What LLMs Typically Write (Wrong)

```rust
use std::collections::hash_map::DefaultHasher; // randomized!
use std::hash::{Hash, Hasher};

let mut hasher = DefaultHasher::new();
tokens.hash(&mut hasher);
let fingerprint = hasher.finish(); // different value every run
```

### What a Senior Writes

```rust
// Cargo.toml: rustc-hash = "1"
use rustc_hash::FxHasher; // deterministic, fast, same output every run
use std::hash::{Hash, Hasher};

let mut hasher = FxHasher::default();
tokens.hash(&mut hasher);
let fingerprint = hasher.finish(); // stable across runs
```

Other acceptable deterministic hashers: `fnv`, `ahash` (with a fixed seed),
`blake3` or `sha256` for cryptographic contexts.

---

## 13. Abstraction Layers — Don't Leak Dependencies

### The Rule

If a type wraps a library type, the library type should not appear in the public API.
Callers should not need to import the underlying library to use your type.

### Why Seniors Care

Exposing `petgraph::DiGraph` in a public field means every caller imports petgraph.
Changing from petgraph to another graph library becomes a breaking change for all callers.
Wrapping the field and providing domain-specific methods keeps the dependency internal.

### What LLMs Typically Write (Wrong)

```rust
// petgraph leaks into every file that uses SemanticGraph
pub struct SemanticGraph {
    pub graph: DiGraph<SemanticNode, EdgeKind>,     // petgraph exposed
    pub name_index: HashMap<String, Vec<NodeIndex>>, // petgraph NodeIndex exposed
}

// Call site in symbols.rs:
self.graph
    .graph  // the double .graph is the smell
    .edges_directed(idx, petgraph::Direction::Incoming) // caller imports petgraph
```

### What a Senior Writes

```rust
pub struct SemanticGraph {
    graph: DiGraph<SemanticNode, EdgeKind>,     // private
    name_index: HashMap<String, Vec<NodeIndex>>, // private
}

impl SemanticGraph {
    /// Returns all symbols that directly call `idx`.
    pub fn callers_of(&self, idx: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .edges_directed(idx, petgraph::Direction::Incoming)
            .filter(|e| *e.weight() == EdgeKind::Calls)
            .map(|e| e.source())
    }

    /// Iterates all declared symbols in the graph.
    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.graph.node_weights().filter_map(|n| match n {
            SemanticNode::Declaration(s) => Some(s),
            _ => None,
        })
    }
}

// Call site — no petgraph import needed:
for caller in registry.graph.callers_of(idx) { ... }
```

---

## 14. `Clone` — Clone Late, Reference Early

### The Rule

Pass references rather than cloning. Clone only when you genuinely need ownership.
Clone inside a loop is almost always a bug.
If you find yourself cloning to avoid a borrow-checker error, redesign the ownership.

### Why Seniors Care

Cloning allocates. Cloning inside a loop allocates on every iteration.
In a tight loop over millions of AST nodes this is measurable throughput loss.
More importantly, gratuitous cloning signals the author didn't understand the borrow
checker — they cloned to silence errors rather than fix the ownership model.

### What LLMs Typically Write (Wrong)

```rust
// Clones the entire registry on every EnterBlock — O(depth × registry_size) allocations
for op in self.context.semantic_ops {
    if let SemanticOp::EnterBlock(body_range) = op {
        let sub_result = sub_analyzer.analyze_block(
            body_node,
            source_re,
            sink_re,
            rule,
            registry.clone(), // expensive, inside loop
        );
        advisories.extend(sub_result);
    }
}
```

### What a Senior Writes

```rust
// Push/pop scope instead of cloning the entire registry
for op in self.context.semantic_ops {
    if let SemanticOp::EnterBlock(body_range) = op {
        registry.push_scope();
        // analyze_block now mutates registry in place
        sub_analyzer.analyze_block(body_node, source_re, sink_re, rule, &mut registry, &mut advisories);
        registry.pop_scope();
    }
}
```

When you genuinely need a snapshot (e.g., to restore on a branch), clone once before the loop:

```rust
let baseline = registry.clone(); // clone once, deliberately
for branch in branches {
    let mut branch_registry = baseline.clone(); // clone from snapshot
    analyze_branch(branch, &mut branch_registry);
}
```

---

## 15. `Arc<dyn Any>` — Almost Always Wrong

### The Rule

`Arc<dyn Any + Send + Sync>` with `downcast` is a Java-style type-erasure pattern
that loses compile-time type safety. In Rust there is almost always a better approach:
an enum, a generic type parameter, or a properly typed map.

### Why Seniors Care

`downcast` can fail at runtime. A `panic!` or `expect` on a downcast failure
is a production crash waiting to happen. The whole point of Rust's type system
is to move these failures to compile time. Using `Any` to escape the type system
is considered a design smell.

### What LLMs Typically Write (Wrong)

```rust
pub struct AnalysisRegistry {
    results: HashMap<(String, ScopeId), Arc<dyn std::any::Any + Send + Sync>>,
}

pub fn get_or_compute<T: 'static + Send + Sync, F: FnOnce() -> T>(
    &mut self, rule_id: &str, scope: ScopeId, compute: F
) -> Arc<T> {
    let entry = self.results.entry(...).or_insert_with(|| Arc::new(compute()));
    entry.clone().downcast::<T>().expect("Type mismatch") // runtime panic
}
```

### What a Senior Writes

```rust
// If only one result type exists, just type the map correctly:
pub struct AnalysisRegistry {
    taint_results: HashMap<(String, ScopeId), Arc<Vec<Advisory>>>,
}

// If multiple result types exist, use an enum:
pub enum AnalysisResult {
    Taint(Vec<Advisory>),
    DataFlow(DataFlowGraph),
    Fingerprints(Vec<Fingerprint>),
}

pub struct AnalysisRegistry {
    results: HashMap<(String, ScopeId), AnalysisResult>,
}
```

---

## 16. Testing — Tests Are First-Class Code

### The Rule

Tests live in `#[cfg(test)]` modules in the same file as the code they test,
or in `tests/` for integration tests. Tests have descriptive names that read as
sentences. Tests cover failure cases, not just the happy path.

### Why Seniors Care

Tests that only test the happy path give false confidence. The interesting bugs
live in edge cases: empty input, maximum depth, Unicode in identifiers, files
with Windows line endings. A test suite that doesn't cover these is a test suite
that doesn't find bugs.

### What LLMs Typically Write (Wrong)

```rust
#[test]
fn test_find() {
    let registry = SymbolRegistry::new();
    // ... (empty test, or tests only the case that obviously works)
    assert!(registry.find("foo").is_empty());
}
```

### What a Senior Writes

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_at_returns_innermost_symbol_on_shadowing() {
        // Arrange: outer function and inner let binding with the same name
        let mut registry = SymbolRegistry::new();
        registry.insert(Symbol { name: "x".into(), line: 1, end_line: 20, kind: SymbolKind::Function, .. });
        registry.insert(Symbol { name: "x".into(), line: 5, end_line: 10, kind: SymbolKind::Variable, .. });

        // Act: query at line 7 — inside both symbols
        let found = registry.find_at("x", "main.rs", 7);

        // Assert: the innermost (smallest span) is returned
        assert_eq!(found.map(|s| s.kind), Some(SymbolKind::Variable));
    }

    #[test]
    fn find_at_returns_none_for_unknown_name() {
        let registry = SymbolRegistry::new();
        assert!(registry.find_at("nonexistent", "main.rs", 1).is_none());
    }

    #[test]
    fn find_at_returns_none_outside_symbol_range() {
        let mut registry = SymbolRegistry::new();
        registry.insert(Symbol { name: "x".into(), line: 5, end_line: 10, .. });
        // Query before the symbol starts
        assert!(registry.find_at("x", "main.rs", 3).is_none());
        // Query after the symbol ends
        assert!(registry.find_at("x", "main.rs", 15).is_none());
    }
}
```

---

## 17. Performance — Measure Before Optimizing, But Know the Costs

### The Rule

Do not optimize prematurely. But know the cost of what you write.
String allocation, heap allocation, `Arc` reference counting, `Mutex` locking —
these have known costs. Writing code that obviously avoids unnecessary cost is
not premature optimization, it is basic competence.

### The Cost Table Every Senior Has Internalized

| Operation | Relative Cost | Notes |
|---|---|---|
| Stack variable access | 1× | Zero cost |
| `&str` / slice reference | 1× | Zero cost |
| `String::clone()` | ~50-500× | Proportional to string length |
| `Vec::clone()` | ~50-500× | Proportional to element count |
| Heap allocation (`Box::new`, `Vec::new`) | ~20-100× | Depends on allocator |
| `Arc::clone()` | ~5× | Atomic increment, cheap |
| `Mutex::lock()` | ~20-50× | Uncontended; much higher contended |
| File I/O | ~1,000,000× | Microseconds to milliseconds |

### The Idiom

Identify *where* allocations happen, not just *whether* they happen.

```rust
// Wrong — allocates a new String for every node in the tree
fn get_kind(node: Node) -> String {
    node.kind().to_string() // heap allocation
}

// Right — return the &str, caller decides if they need ownership
fn get_kind(node: Node) -> &str {
    node.kind() // zero cost
}

// Wrong — allocates Vec at every recursive call
fn walk(node: Node) -> Vec<Advisory> { ... }

// Right — one allocation, thread it through
fn walk(node: Node, out: &mut Vec<Advisory>) { ... }
```

---

## 18. Formatting — Let `rustfmt` Decide, Never Fight It

### The Rule

Run `rustfmt` on every file. Configure it in `rustfmt.toml` if needed.
Never manually format code in ways that `rustfmt` would undo — you will lose
that formatting on the next `cargo fmt` run.

### Why Seniors Care

Inconsistent formatting is a distraction during code review. Reviewers focus on
logic, not braces. A project that runs `cargo fmt` in CI has one fewer thing to
argue about. A project that doesn't is a project where every PR has formatting noise.

### The Only Things That Need Explicit Attention

```rust
// Long function signatures: one parameter per line
pub fn complicated_function(
    first_param: FirstType,
    second_param: SecondType,
    third_param: ThirdType,
) -> ReturnType {

// Long match arms: use a block
match something {
    VeryLongVariantName(inner) => {
        do_something_with(inner);
        another_thing()
    }
    Short => quick_value(),
}

// Long where clauses: one bound per line
fn generic_function<T, U>(t: T, u: U) -> Result<Output, Error>
where
    T: Clone + Send + Sync + 'static,
    U: Into<String> + Debug,
{
```

---

## Quick Reference Checklist

When reviewing or generating Rust code, run through this list:

**Visibility**
- [ ] Fields are private unless there is a specific reason for `pub`
- [ ] `pub(crate)` is used for module-internal-but-not-public items

**Types**
- [ ] No stringly-typed discriminants — use enums
- [ ] No bare tuple return types for named concepts — use structs
- [ ] No `Arc<dyn Any>` — use enums or typed maps
- [ ] Newtype wrappers for domain-specific primitives (`FileId`, `LineNumber`)

**Errors**
- [ ] All fallible functions return `Result`
- [ ] Error types use `thiserror` and are descriptive
- [ ] No `unwrap()` without a `// SAFETY:` or `// INVARIANT:` comment
- [ ] No `panic!` in library code

**Mutation**
- [ ] No output parameters (`&mut Vec`) unless performance is demonstrably critical
- [ ] `mut` bindings are as short-lived as possible
- [ ] No `clone()` inside loops without a comment explaining why

**Structure**
- [ ] No functions with more than ~5 parameters — use an options struct
- [ ] No `#[allow(clippy::...)]` without a comment explaining why the suppression is correct
- [ ] No recursive tree walks — use iterative cursor traversal
- [ ] No `DefaultHasher` for persistent or cross-run data

**Documentation**
- [ ] Every `pub` item has a `///` doc comment
- [ ] Comments explain *why*, not *what*
- [ ] Non-obvious invariants and preconditions are documented

**Tests**
- [ ] Tests cover failure cases, not just the happy path
- [ ] Test names are sentences describing the scenario
- [ ] Edge cases: empty input, maximum size, boundary values

**Dependencies**
- [ ] Library types are not exposed in public APIs
- [ ] `use` imports at the top of each file, grouped: std → external → internal

---

## The Single Most Important Principle

> **Make the type system do the work.**

If a bug can be caught at compile time, it should be caught at compile time.
If valid states can be encoded in types, encode them.
If a function has a precondition, make violating it a type error, not a runtime panic.

Every senior Rust engineer's instinct, when they see a `panic!`, an `unwrap()`,
a stringly-typed enum, or an `Arc<dyn Any>`, is: *the compiler could have caught this.*
The goal is to write code where the compiler is your pair programmer,
not your adversary.
