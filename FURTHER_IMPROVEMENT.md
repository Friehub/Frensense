Tier 1 — Buildable Now (No new infrastructure)
1. Function Summaries (Taint Propagation Model)

This is the single highest-leverage addition. Instead of inlining callee bodies on every call site, compute a summary of what a function does to its inputs:

fn process(x: tainted) -> tainted   // propagates
fn sanitize(x: tainted) -> clean    // cleans
fn log(x: tainted) -> ()            // sink
Store these in the SymbolRegistry during the Assembly pass. During audit, a rule checks the summary — no inlining, no recursion, no depth limit. This is what separates SAST tools from linters and allows GenSense to scale to 1M LOC without exponential blowup. The Symbol struct already has start_byte/end_byte — attaching a TaintSummary enum costs nothing architecturally.

2. File Hash–Based Incremental Analysis

The Snapshot model already gives us this for free. Each FileSnapshot is produced from a file's content. If we hash the content before building the snapshot and store that hash alongside the snapshot, we can skip re-parsing unchanged files on the next run.

cache: HashMap<PathBuf, (content_hash, Vec<Symbol>, Vec<SemanticOp>)>
On a 1,000-file project where 10 files change, this reduces Pass 1 from 1000 × parse time to 10 × parse time. This is the difference between a 4-second CI check and a 400-millisecond one.

3. Confidence Scoring on Advisories

Advisories today are binary — found or not found. A confidence score changes the output from noise to signal. The score is simple: how many constraints were satisfied?

1 constraint met (pattern match only)    → 40% confidence
2 constraints met (pattern + scope)      → 65% confidence
3 constraints met (pattern + scope + flow) → 90% confidence
This requires no new data structures — just a confidence: u8 field on Advisory. Rules that only pattern-match stay at 40%. Rules that trace data flow to a confirmed sink report 90%. The CLI can filter by confidence threshold.

4. Queryable Graph via Closures

Expose the SymbolRegistry as a queryable surface after the Assembly pass:

rust
engine.query(|graph| {
    graph.symbols_where(|s| s.kind == Function)
         .that_call("exec")
         .from_files_matching("handler")
})
This is not a new language or protocol — it is just iterator combinators over the existing SemanticGraph. Users can write custom cross-file queries without writing rules. This is what makes GenSense programmable rather than just configurable.

Tier 2 — Short-Term (One phase of work)
5. Dependency Boundary Modeling

GenSense currently treats all call targets as either internal (resolvable) or unknown (ignored). The next step is modeling the external boundary — knowing which functions are from dependencies and what their taint contracts are.

For Rust: parse Cargo.toml to know the dependency set. Maintain a built-in "known sink" registry:

serde_json::from_str    → propagates taint from input
reqwest::Client::get    → network sink
std::process::Command   → command injection sink
This requires no cross-crate compilation — just a curated, versioned JSON registry of known sinks and sources. Rules that use FlowConstraint::TaintReached immediately become aware of the entire ecosystem, not just the local codebase.

6. Ownership-Aware Rules for Rust

Rust is unique because the compiler already enforces memory safety. What GenSense can add is semantic ownership reasoning that the compiler does not check:

Arc<Mutex<T>> held across an .await (the DeadlockGuard already exists, but ownership tracking would make it zero false positives)
unsafe blocks that dereference pointers from external sources (taint + unsafe boundary)
Send + Sync bounds violated by holding non-Send types across thread boundaries
The tree-sitter grammar already exposes unsafe_block, reference_expression, and lifetime annotations. No new parser infrastructure is needed.

What GenSense Becomes
Fix the 4 issues + add Tier 1 + Tier 2:

