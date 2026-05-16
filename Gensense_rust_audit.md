What a senior Rust engineer sees first
Before anything else, they run cargo clippy -- -W clippy::pedantic and read lib.rs. If the public API has no doc comments and the types feel stringly-typed, they stop trusting the codebase. That's the first impression problem.

1. Everything is pub — there's no API surface
Almost every field on every struct is pub. A senior engineer reads this as "the author didn't decide what the contract is yet."
rust// Current — entire internals exposed
pub struct TaintRegistry<'a> {
    pub scopes: Vec<HashMap<&'a str, &'a str>>,
    pub symbols: Vec<HashMap<&'a str, Node<'a>>>,
}

pub struct DataFlowAnalyzer<'a, 'ctx> {
    pub context: &'ctx GenSenseContext<'a>,
    pub root: Node<'a>,
    pub depth: usize,
    pub max_depth: usize,
}
Fix: Make fields private, expose only what callers need. The TaintRegistry mutation methods already exist — the fields don't need to be public at all.
rustpub struct TaintRegistry<'a> {
    scopes: Vec<HashMap<&'a str, &'a str>>,
    symbols: Vec<HashMap<&'a str, Node<'a>>>,
}

pub struct DataFlowAnalyzer<'a, 'ctx> {
    context: &'ctx GenSenseContext<'a>,
    root: Node<'a>,
    depth: usize,
    max_depth: usize,
}
Same applies to SemanticGraph::graph, SemanticGraph::name_index, GenSenseAuditor::rules, GenSenseAuditor::suppressions. Expose them through methods with clear intent, not raw field access.

2. Stringly-typed origins in the taint registry
rustpub scopes: Vec<HashMap<&'a str, &'a str>>, // var -> origin
The origin is a &str — currently always the string literal "source". This means the taint system can't distinguish between different source kinds (user input vs environment variable vs file read). An experienced engineer immediately asks "what are the valid values of this string?" and the answer isn't in the type.
Fix: Make origin a proper type.
rust#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaintOrigin {
    UserInput,
    EnvVar,
    FileRead,
    NetworkRequest,
    External(&'static str), // for custom rule-defined sources
}

pub struct TaintRegistry<'a> {
    scopes: Vec<HashMap<&'a str, TaintOrigin>>,
    symbols: Vec<HashMap<&'a str, Node<'a>>>,
}
Now the taint finding message can say "Tainted by EnvVar" instead of "from 'source'", and rules can pattern-match on origin kind rather than string comparison.

3. The ScanResult type alias hides intent
rustpub type ScanResult = (Vec<Advisory>, Vec<FunctionFingerprint>);
Tuple return types are a code smell in public APIs. When you call audit() you get back result.0 and result.1 — neither is self-documenting at the call site.
Fix:
rustpub struct ScanResult {
    pub advisories: Vec<Advisory>,
    pub fingerprints: Vec<FunctionFingerprint>,
}
Now every call site reads result.advisories and result.fingerprints. The intent is clear without checking the type alias definition.

4. run_recursive is a god traversal with no visitor pattern
rustpub fn run_recursive<'a>(
    &self,
    node: Node<'a>,
    rule: &dyn GenSenseRule,
    context: &GenSenseContext<'a>,
) -> Vec<Advisory> {
    let mut advisories = Vec::new();
    if !is_suppressed(...) {
        advisories.extend(rule.check(node, context));
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        advisories.extend(self.run_recursive(child, rule, context));
    }
    advisories
}
This is a recursive accumulator — the classic shape that Rust engineers replace with an explicit stack or a visitor. The recursive version has two problems: it allocates a fresh Vec at every tree node (expensive), and it has no depth limit (stack overflow on pathologically deep ASTs).
Fix: An iterative walk with a pre-allocated buffer.
rustfn walk_tree<'a>(
    &self,
    root: Node<'a>,
    rule: &dyn GenSenseRule,
    context: &GenSenseContext<'a>,
    out: &mut Vec<Advisory>,
) {
    let mut cursor = root.walk();
    loop {
        let node = cursor.node();
        if !is_suppressed(&self.suppressions, node, rule.id(), context.source_code, context.file_path) {
            out.extend(rule.check(node, context));
        }

        // Descend, then move to next sibling, then ascend
        if cursor.goto_first_child() { continue; }
        loop {
            if cursor.goto_next_sibling() { break; }
            if !cursor.goto_parent() { return; }
        }
    }
}
This uses tree-sitter's TreeCursor the way it's designed to be used — stateful traversal without recursion. One allocation for the output buffer, no stack risk.

5. AnalysisRegistry uses Any downcasting for no reason
rustpub struct AnalysisRegistry {
    taint_results: HashMap<(String, ScopeId), Arc<dyn std::any::Any + Send + Sync>>,
}

pub fn get_or_compute<T, F>(&mut self, rule_id: &str, scope: ScopeId, compute: F) -> Arc<T>
where T: 'static + Send + Sync, F: FnOnce() -> T
{
    // ...
    entry.clone().downcast::<T>().expect("Type mismatch in AnalysisRegistry")
}
Arc<dyn Any> with downcast and a panic on mismatch is a Java pattern in a Rust codebase. The expect("Type mismatch") is a time bomb — it panics in production if the same (rule_id, scope) key is ever used with two different T types. An experienced Rust engineer sees this and immediately asks why a generic enum wasn't used instead.
Fix: Since the only thing stored today is taint results, just type the map correctly. If future types are needed, use an enum.
rustpub struct AnalysisRegistry {
    taint_results: HashMap<(String, ScopeId), Arc<Vec<Advisory>>>,
}

impl AnalysisRegistry {
    pub fn get_or_compute(
        &mut self,
        rule_id: &str,
        scope: ScopeId,
        compute: impl FnOnce() -> Vec<Advisory>,
    ) -> Arc<Vec<Advisory>> {
        Arc::clone(
            self.taint_results
                .entry((rule_id.to_string(), scope))
                .or_insert_with(|| Arc::new(compute())),
        )
    }
}
No Any, no expect, no type mismatch at runtime.

6. #[allow(clippy::too_many_arguments)] is a suppression, not a fix
rust#[allow(clippy::too_many_arguments)]
pub fn audit<'a>(
    &self,
    file_id: FileId,
    path: &'a Path,
    content: &'a str,
    tree: &'a tree_sitter::Tree,
    semantic_ops: &'a [...],
    symbols: &'a SymbolRegistry,
    category_filter: &HashSet<String>,
    tag_filter: &HashSet<String>,
    env: crate::GenSenseEnvironment,
) -> Result<ScanResult>
Nine parameters. The #[allow] annotation tells Clippy to stop complaining but doesn't fix the underlying problem. An experienced engineer reads #[allow(clippy::...)] as "the author knew this was wrong and chose not to fix it."
Fix: The filter/env parameters belong in a dedicated options type. The file-specific parameters can stay.
rust/// Options controlling which rules run and in what environment.
#[derive(Debug, Default)]
pub struct AuditOptions {
    pub category_filter: HashSet<String>,
    pub tag_filter: HashSet<String>,
    pub env: GenSenseEnvironment,
}

pub fn audit<'a>(
    &self,
    file_id: FileId,
    path: &'a Path,
    content: &'a str,
    tree: &'a tree_sitter::Tree,
    semantic_ops: &'a [SemanticOp],
    symbols: &'a SymbolRegistry,
    options: &AuditOptions,
) -> Result<ScanResult>
Now the call site reads naturally, and AuditOptions can grow new fields without breaking every caller.

7. Mutation hidden inside Vec::extend chains
rustpub fn audit(...) -> Result<ScanResult> {
    let mut advisories = Vec::new();
    let mut fingerprints = Vec::new();
    // ...
    for rule in &self.rules {
        // ...
        advisories.extend(rule.check(capture.node, &context));
        // ...
        advisories.extend(self.run_recursive(...));
    }
    // ...
    extract_fingerprints(tree.root_node(), content, path, &mut fingerprints);
    Ok((advisories, fingerprints))
}
The extract_fingerprints function mutates fingerprints via &mut Vec — a C-style output parameter. The Rust idiom is to return the value and let the caller collect it, or use an iterator. Hidden mutation through output parameters is the pattern Rust's ownership system was designed to make unnecessary.
Fix:
rust// fingerprint.rs
pub fn extract_fingerprints(node: Node, source_code: &str, path: &Path) -> Vec<FunctionFingerprint> {
    let mut fingerprints = Vec::new();
    // ... populate internally ...
    fingerprints
}

// audit()
let fingerprints = extract_fingerprints(tree.root_node(), content, path);
Ok(ScanResult { advisories, fingerprints })

8. SemanticGraph leaks petgraph internals through its public API
rustpub struct SemanticGraph {
    pub graph: DiGraph<SemanticNode, EdgeKind>,  // petgraph exposed
    pub name_index: HashMap<String, Vec<NodeIndex>>,
}
Callers outside the module can directly call self.graph.add_node(), self.graph.edges_directed(), self.graph.node_weights(). This is seen in symbols.rs:
rustself.graph
    .graph  // accessing the inner DiGraph directly
    .edges_directed(idx, petgraph::Direction::Incoming)
    .filter(|e| *e.weight() == EdgeKind::Calls)
The double .graph.graph is a code smell — the abstraction layer is leaking. If the graph library ever changes, every call site breaks.
Fix: Add domain-specific query methods to SemanticGraph and make graph and name_index private.
rustimpl SemanticGraph {
    pub fn callers_of(&self, idx: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .edges_directed(idx, petgraph::Direction::Incoming)
            .filter(|e| *e.weight() == EdgeKind::Calls)
            .map(|e| e.source())
    }

    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.graph.node_weights().filter_map(|n| {
            if let SemanticNode::Declaration(s) = n { Some(s) } else { None }
        })
    }
}
Now symbols.rs calls self.graph.callers_of(idx) — one level of indirection, the petgraph dependency is contained.

9. The DefaultHasher in fingerprinting is non-deterministic across runs
rustuse std::hash::{Hash, Hasher};
let mut hasher = std::collections::hash_map::DefaultHasher::new();
tokens[i..i + 5].hash(&mut hasher);
ngram_hashes.insert(hasher.finish());
DefaultHasher uses a randomized seed in Rust since 1.36 (via HashDoS protection). This means fingerprint hashes change between program runs, which completely breaks SRI baseline comparison — the whole point of the fingerprinting feature. Two runs on the same unchanged file will produce different hashes.
Fix: Use a deterministic hasher. FxHasher from the rustc-hash crate or a simple FNV-1a implementation:
rust// Add to Cargo.toml: rustc-hash = "1"
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

let mut hasher = FxHasher::default();
tokens[i..i + 5].hash(&mut hasher);
ngram_hashes.insert(hasher.finish());
This is a silent correctness bug — SRI baselines would silently expire on every run, making the CI noise-suppression feature useless. An experienced engineer doing a security review would catch this immediately.

10. No doc comments on any public type or method
A senior engineer reads the docs before the code. Currently there are none. Not on GenSenseRule, not on Advisory, not on SymbolRegistry. The trait GenSenseRule is the central abstraction of the entire codebase — it deserves a doc comment that explains what implementors are expected to do.
rust// Current
pub trait GenSenseRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;
    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory>;
    fn applies_to(&self, extension: &str) -> bool;
}
Fix:
rust/// A single semantic analysis rule.
///
/// Rules are the core unit of analysis in GenSense. Each rule receives every
/// AST node in a file (or a pre-filtered subset via [`Self::query`]) and returns
/// zero or more [`Advisory`] findings.
///
/// # Implementation notes
/// - `check` is called from a rayon thread pool; implementations must be `Send + Sync`.
/// - Return an empty `Vec` (not an `Err`) for nodes that don't match the rule.
/// - Use [`Self::new_advisory`] to construct findings — it populates `enclosing_symbol`
///   and other fields automatically.
pub trait GenSenseRule: Send + Sync {
    /// Metadata describing this rule: ID, severity, category, and human-readable text.
    fn metadata(&self) -> &RuleMetadata;

    /// Core analysis logic. Called once per matching AST node per file.
    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory>;

    /// Returns true if this rule applies to files with the given extension.
    fn applies_to(&self, extension: &str) -> bool;

    /// Optional tree-sitter query string. When `Some`, the engine runs the query
    /// and calls `check` only on matching nodes, which is faster than a full traversal.
    fn query(&self) -> Option<&str> { None }
}

Priority order
Do these in this sequence — each one builds trust with a human reviewer:

DefaultHasher → FxHasher — silent correctness bug, fix first
ScanResult named struct — one line, immediate clarity
run_recursive → iterative cursor walk — performance and safety
Remove pub from internal fields — signals the codebase has a defined API contract
TaintOrigin enum — removes stringly-typed core data
AuditOptions struct — eliminates the #[allow] suppression
SemanticGraph domain methods — hides petgraph, removes .graph.graph
AnalysisRegistry typed map — removes the Any downcast panic
extract_fingerprints return value — removes output parameter mutation
Doc comments on all public API surfaces — the thing that makes engineers trust the codebase is maintained by humans who care