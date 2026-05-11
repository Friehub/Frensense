Core Improvement Areas — Analysis Only
Area 1: The TaintCache is doing the wrong job (Accuracy + Efficiency)
Location: src/rules/ir.rs:76-80, src/rules/core/mod.rs:79-83

What it does: Before running a taint analysis, it checks a cache keyed by (rule_id, root_node_id). If found, it returns early.

The problem: The cache key is the root node id — this is the file root, not the function scope. This means: once any single node in a file has been analyzed by a rule, the entire file is skipped for subsequent nodes. This produces false negatives. You miss real violations.

What to do instead: The unit of memoization for taint analysis should be the function body, not the file root. You already have find_function_at in SymbolRegistry — the natural cache key is (rule_id, file_path, function_start_line). No cache infrastructure needed; just a local HashSet inside check() per invocation.

Area 2: SemanticExtractor is called on every rule invocation (Efficiency)
Location: src/semantics/data_flow/tracking.rs:10-31

What it does: with_ops() calls SemanticExtractor::extract() on a node and memoizes the result in SemanticCache (the RefCell).

The problem: The extract is happening inside the Audit pass, per rule, per file. If 10 rules all run taint analysis on the same file, the SemanticExtractor runs 10 times (or hits the cache 9 times with a borrow_mut). This is the wrong phase for this work.

The correct place: SemanticExtractor::extract() should run in Pass 1 (Snapshot). Each FileSnapshot should carry its Vec<SemanticOp> already extracted. The Audit pass then receives these ops as read-only data — no extraction, no cache, no RefCell. This collapses 10 cache-checks into zero.

Area 3: Call graph edges are name-only — no module awareness (Accuracy)
Location: src/semantics/symbols.rs:106-130, src/engine/auditor/discovery.rs:88-128

What it does: scan_for_edges returns (caller_name, callee_name) as plain strings. add_call_edge then links them by name only — so if connect() is defined in three different files, all three get linked.

The problem: The dependency graph is imprecise. Two functions in different modules with the same name appear as a single node to any rule doing graph traversal. This produces false positives in interprocedural analysis.

The right approach: Qualify names with their file path during the snapshot phase. An edge should be (file::caller, file::callee). The SymbolRegistry already has file_path on every symbol — it just needs to be used during edge resolution. This is a natural fit for the Assembly pass where all symbols are already registered with their file context.

Area 4: SemanticExtractor does not handle destructuring or method calls correctly (Accuracy)
Location: src/semantics/data_flow/normalization.rs:127-135 (Rust) and 54-67 (TypeScript)

Rust destructuring (let (a, b) = tuple) — the pattern field captures the full destructure node as a single name, not individual bindings. Taint is tracked for the pattern string "(a, b)", not a and b separately. This is a confirmed false-negative path already logged in WEAKNESSES.md.

Method chains (obj.method().another()) — when the function field of a call_expression is a field_expression, the extractor captures the full chain as the function name (e.g., "obj.method"). This breaks taint propagation across chained calls.

The fix approach (still within our principle): In the snapshot phase, when extracting ops, walk destructure patterns recursively to produce individual Binding ops per identifier. For method chains, extract the receiver separately as a potential taint source.

Area 5: The CoreRule query dispatch has a fragile heuristic (Reliability)
Location: src/rules/core/mod.rs:55-61

rust
fn query(&self) -> Option<&str> {
    if self.on_node.contains("|") || !self.on_node.contains(" ") {
        None
    } else {
        Some(&self.on_node)
    }
}
This determines whether to use tree-sitter query matching or a full recursive walk based on the presence of a space or | in the YAML on_node field. This is a fragile string heuristic that a YAML rule author can accidentally trigger by adding spaces or pipes in ways that change the execution path silently.

Better approach: Add an explicit field to CoreRule: use_query: bool. The decision becomes intentional and auditable, not inferred from string shape.






Correctness Tests

Test	What it verifies
symbol_shadowing	find_at returns innermost symbol, not outermost, when two x exist at different scopes
taint_through_destructuring	Taint flows from let (a, b) = tainted_pair to both a and b
false_positive_isolation	A rule with source_pattern fires on actual taint paths, not on clean code
false_negative_scope	Taint tracked across push_scope / pop_scope boundaries does not leak
snapshot_determinism	Two runs on the same codebase produce identical advisories (pure function test)
suppression_correctness	// gensense-suppress RULE_ID prevents advisories on the suppressed line only
yaml_rule_taint_flow	A YAML rule with source_pattern + sink_pattern fires on a matching taint path
yaml_rule_temporal	A YAML must_follow rule fires when sequence is incomplete
advisory_deduplication	The same finding is not reported twice when a node matches multiple queries
Performance / Scaling Benchmarks

Benchmark	Target
bench_symbol_lookup_1k	find_at on 1,000 symbols completes in < 1ms
bench_symbol_lookup_100k	find_at on 100,000 symbols completes in < 5ms (verifies $O(\log S)$)
bench_discovery_100_files	Parallel discovery of 100 files completes in < 500ms
bench_discovery_1000_files	Parallel discovery of 1,000 files completes in < 3s
bench_audit_10_rules	Full audit pass with 10 rules on 100 files completes in < 1s
bench_assembly_phase	Sequential assembly of 50,000 symbols completes in < 200ms
bench_memory_5000_symbols	Peak RSS does not exceed 50MB for a 5,000-symbol project
Integration Tests

Test	What it verifies
e2e_user_yaml_rule_loaded	A .gensense/rules/custom.yml rule fires on a matching file
e2e_suppress_file_respected	.gensense-suppress.yml prevents named advisories from appearing
e2e_severity_override_config	gensense.yml severity_override changes the severity of a rule in output
e2e_scan_no_advisories_on_clean	A clean, well-written Rust file produces zero advisories
e2e_incremental_hash_skip	(Future) A file with unchanged content hash is skipped in re-analysis
Rule-Specific Tests (Per Rule)

Every Rust and TypeScript rule needs at minimum:

Test type	Description
fires_on_positive	The rule fires on a crafted snippet that contains the violation
silent_on_negative	The rule does NOT fire on equivalent clean code
fires_at_correct_line	The advisory.line matches the actual violation line
