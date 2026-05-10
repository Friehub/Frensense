# GenSense Algorithm Guide

**Who this is for:** Everyone working on GenSense — human and AI agents alike.  
**Purpose:** Understand every algorithm the engine uses, what is weak about each one, and what we can build that is distinctly ours.

Read this before writing any engine code. When an agent is unsure whether a proposed improvement is on Path B or Path A, the answer is in this document.

---

## How to Read This Document

Each algorithm is described in four parts:

1. **What it is** — a plain English explanation of what the algorithm does, with no assumed knowledge
2. **How GenSense uses it right now** — the exact current implementation, including its weaknesses
3. **What we can improve** — concrete changes that stay on Path B (fast, precise, no complexity explosion)
4. **What we own** — the part that is specifically ours, that competitors cannot easily copy

---

## Algorithm 1: N-Gram Fingerprinting

### What it is

A fingerprint is a compact signature of a piece of text. An n-gram is a sequence of N consecutive tokens. Fingerprinting takes a block of code, splits it into tokens (words), slides a window of size N across them, hashes each window, and stores all those hashes in a set.

Example with N=3 and the text `lock acquire wait`:
- Window 1: `lock acquire wait` → hash → 4829301
- Window 2: `acquire wait release` → hash → 7193847
- Fingerprint = {4829301, 7193847, ...}

To compare two functions, compare their hash sets. The similarity score is:

```
similarity = |hashes_A ∩ hashes_B| / |hashes_A ∪ hashes_B|
```

This is called the Jaccard similarity coefficient. A score of 1.0 means identical. A score of 0.8 means 80% similar. This is O(min(|A|, |B|)) to compute.

### How GenSense uses it right now

File: `src/engine/fingerprint.rs`

The current implementation:
- Splits function bodies on whitespace
- Filters out empty tokens and comment tokens starting with `//`
- Uses a sliding window of size 5 (5-grams)
- Hashes each window using Rust's `DefaultHasher`
- Stores all hashes in a `HashSet<u64>`

**What is weak:**
- The fingerprints are computed but never compared against each other anywhere in the codebase. The `FunctionFingerprint` struct is created but the comparison logic that would detect copy-paste does not exist yet.
- `DefaultHasher` is not stable across Rust versions. The same code can produce different hashes in different builds.
- Whitespace splitting treats `let x=1` and `let x = 1` as different tokens, missing matches where only formatting differs.
- Variable names are not normalised. `let user_id = get_id()` and `let account_id = get_id()` will not match even though the structure is identical.

### What we can improve

**Improvement 1: Build the comparison pass.**

This is the most important gap. After all files are fingerprinted, compare every pair of functions using Jaccard similarity. Flag pairs with similarity > 0.8 that are in the same file and have at least one difference (to avoid flagging a function against itself).

The comparison is O(n²) in the number of functions. For a 100-function codebase that is 10,000 comparisons, each taking O(min tokens) time — fast enough. For a 10,000-function codebase you would need to bucket by approximate size first, but that is a future problem.

**Improvement 2: Token normalisation before hashing.**

Before computing hashes, normalise each token:
- Replace all identifier names with a placeholder: `let x = y` → `let VAR = VAR`
- Replace all string literals with `STR`
- Replace all number literals with `NUM`

This makes the fingerprint sensitive to structure, not names. Two functions that do the same thing with different variable names will now match. This is called structure-preserving normalisation and is the standard approach in clone detection research.

The normalisation step runs before fingerprinting and does not change the output format — it is a pre-processing step only.

**Improvement 3: Use a stable hash function.**

Replace `DefaultHasher` with `FxHasher` (from the `rustc-hash` crate, already common in Rust tooling) or a fixed-seed xxHash. This makes fingerprints deterministic and reproducible across builds, which matters for caching.

### What we own

The combination of structure-preserving normalisation with the comparison pass, scoped to functions in the same file, tuned specifically to detect LLM copy-paste patterns, is something we built. No general-purpose linter does this. Semgrep does not do it. Clippy does not do it. This is a genuinely novel detection mechanism for the LLM-code problem.

The specific insight — that LLM copy-paste differs from human copy-paste because the model changes names but preserves structure — is the intellectual contribution. The algorithm (Jaccard on normalised n-grams) is standard. The application is ours.

---

## Algorithm 2: Taint Propagation (Data Flow Analysis)

### What it is

Taint analysis tracks whether data from a "source" (something dangerous, like user input or an environment variable) ever reaches a "sink" (something that should not receive untrusted data, like a database query or a shell command).

The algorithm works like an infection spreading through a graph:
1. Mark all source nodes as "tainted"
2. Follow every data flow edge outward from tainted nodes
3. If a tainted value reaches a sink, emit a finding
4. Stop when no new nodes can be reached

This is a standard graph reachability problem. The formal name is "taint-tracking data flow analysis."

### How GenSense uses it right now

Two implementations exist simultaneously:

**Implementation A — AST-local (`src/semantics/data_flow/tracking.rs::analyze_block`):**
Walks the AST of a single function body. When it sees `let x = env::var(...)`, it marks `x` as tainted. When it sees a call where `x` is an argument, it checks if the call matches the sink pattern. If yes, it emits an advisory.

This is recursive — when a tainted argument is passed to another function, it looks up that function's definition (`find_definition`), maps the argument to the parameter name (`map_params`), and recurses into the called function. The recursion depth is capped at 5.

**Implementation B — Graph BFS (`src/semantics/data_flow/tracking.rs::check_taint_graph`):**
Starts from all source nodes in the `SemanticGraph`. Runs a BFS through the graph following `FlowsFrom`, `Calls`, and `SequentiallyFollows` edges (after Bug 3 fix). If BFS reaches a sink node, emits an advisory.

**What is weak:**
- **Implementation A** uses a flat `HashMap<String, String>` (variable name → source name). This means one variable can only have one taint origin. If `x` is tainted by source A and `y` is tainted by source B and then `let z = combine(x, y)`, the registry can only record one origin for `z`. The other is lost.
- **Implementation B** has no awareness of which function a sink belongs to, so it cannot tell you whether the taint path is actually reachable or just theoretically connected in the graph.
- Neither implementation handles destructuring: `let (a, b) = tainted_pair`. Both `a` and `b` should be tainted, but the registry cannot represent this.
- The recursion cap of 5 in Implementation A is arbitrary. A chain of 6 thin wrapper functions breaks the analysis.

### What we can improve

**Improvement 1: Taint sets instead of taint strings.**

Change the registry from `HashMap<String, String>` to `HashMap<String, HashSet<String>>`. Each variable maps to the set of all sources that taint it. When two tainted variables are combined, the result's taint set is the union of both input sets.

```
Before: { "x" -> "env::var", "y" -> "req.body" }
After:  { "x" -> {"env::var"}, "y" -> {"req.body"}, "z" -> {"env::var", "req.body"} }
```

This is called a lattice-based taint domain. The join operation at a merge point (combining two variables) is set union. This is the standard approach and is not complex to implement — it is a one-line change to the data structure and a small change to the merge logic.

**Improvement 2: Destructuring support.**

When the AST contains a `let (a, b) = pair` pattern (a `tuple_pattern` in tree-sitter), check if `pair` is tainted. If yes, mark both `a` and `b` as tainted. This requires recognising `tuple_pattern`, `struct_pattern`, and `slice_pattern` node kinds in the variable resolution step.

**Improvement 3: Sanitiser modelling.**

Add a `sanitisers` field to taint rules (YAML):

```yaml
source_pattern: "env::var|req\\.body"
sink_pattern: "execute_query|shell_exec"
sanitise_pattern: "html_escape|sanitize|validate"
```

When the BFS or AST walk encounters a call matching `sanitise_pattern`, remove all taint from the result of that call. This eliminates false positives for code that correctly sanitises before using tainted data.

### What we own

The combination of multi-source taint sets with sanitiser modelling expressed in a simple YAML rule format is not available in any open-source tool at GenSense's weight class (fast, pre-commit-friendly). Semgrep Pro has taint mode but it is not YAML-configurable at this level of detail, and it is not fast enough for pre-commit hooks. We own the ergonomics: a developer can write a taint rule in 5 lines of YAML.

---

## Algorithm 3: Temporal Event Ordering

### What it is

Temporal analysis checks whether events inside a function happen in the right order. An "event" is any significant operation: acquiring a lock, releasing a lock, calling `await`, returning from a function.

The algorithm works in two phases:

Phase 1 — Build the event chain. Walk the function body in execution order. Every time you find a significant call or expression, record it as an event node. Connect events with "happens before" edges. The result is a linked list of events for each function.

Phase 2 — Check the chain. For a `MustFollow` rule like "unlock must follow lock", scan the event list from start to end. Track which step of the required sequence has been reached. If you reach the end of the function without completing the sequence, emit a finding. For a `MustNotFollow` rule like "await must never follow lock", scan for the first pattern, then watch for the second. If both appear in that order, emit a finding.

This is formally equivalent to running a finite state machine (FSM) over a sequence of symbols. The sequence is the event list. The symbols are the event labels. The FSM states are the steps in the rule's sequence.

### How GenSense uses it right now

File: `src/semantics/temporal.rs`

The current implementation:
- Retrieves the ordered event list from the graph using `ordered_events_in_scope`
- For each event, checks it against each regex in the rule's sequence using a linear scan
- Tracks progress through the sequence with a single integer counter (`current_step`)
- Emits a finding if the sequence is incomplete (`MustFollow`) or if a forbidden pair appears (`MustNotFollow`)

**What is weak:**
- The sequence matching is implemented as a simple loop with an integer counter. It cannot express disjunction: "step 2 must be either `unlock` OR `drop`". The rule author must write two separate rules.
- `MustFollow` checks that the sequence appears in order but does not check that the sequence is balanced. A rule for "every lock must be followed by an unlock" cannot express "the number of locks must equal the number of unlocks."
- The implementation is linear scan (O(events × sequence_length)). For functions with hundreds of events and complex rules, this is acceptable. But it cannot be extended to support backtracking (trying alternative paths) without a redesign.
- The event labels are matched with regex, which is expressive but slow for large event lists. A hash-based pre-filter would be faster.

### What we can improve

**Improvement 1: Compile rules to NFAs instead of integer counters.**

A non-deterministic finite automaton (NFA) is a graph where:
- Nodes are states
- Edges are transitions labeled with patterns
- One node is the start state
- One or more nodes are accept states (violation found)

A `MustNotFollow(["lock", "await"])` rule compiles to:

```
State 0 (start) --"lock"--> State 1
State 0 --anything else--> State 0
State 1 --"await"--> State 2 (VIOLATION)
State 1 --"unlock"--> State 0 (reset — lock was released safely)
State 1 --anything else--> State 1
```

The NFA runs over the event list in O(events × states) time. The benefit is that the NFA can express:
- Disjunctive steps: "step 2 can be `unlock` OR `drop`" — add two edges from State 1
- Reset conditions: "after `unlock`, the lock count resets" — add a reset edge
- Counting: add a counter to a state and a self-loop that increments it

This is not Path A complexity. An NFA with 3-5 states compiles in microseconds. The event list is already built. This is a change to how the rule is checked, not to what the engine analyses.

**How to implement:** The `TemporalBehavior` enum gains a new variant: `NfaRule(Vec<NfaState>)`. The YAML compiler builds the NFA from the `sequence` and `behavior` fields. The `check_temporal` function in `temporal.rs` runs the NFA instead of the integer counter.

**Improvement 2: Balanced counting for acquire/release pairs.**

Add a `must_balance` behavior to the DSL:

```yaml
temporal:
  sequence: ["lock", "unlock"]
  behavior: must_balance
```

The checker counts occurrences of `sequence[0]` and `sequence[1]` in the event list. If they are not equal, emit a finding with the counts. This detects "you locked 3 times but only unlocked 2 times."

Implementation: a two-pass scan. Pass 1: count all matches for each pattern. Pass 2: compare counts. O(events) total.

**Improvement 3: Scope-bounded temporal rules.**

Currently all temporal rules apply to the entire function body. Add a `within_loop` and `within_block` scope qualifier to the YAML DSL:

```yaml
temporal:
  sequence: ["acquire", "release"]
  behavior: must_balance
  scope: loop_body
```

This restricts the event list to only the events inside each loop iteration, enabling detection of "acquire/release imbalance per loop iteration" rather than per function.

### What we own

The NFA-based temporal engine compiled from a YAML DSL, applied to a semantic event graph, is genuinely novel for a tool at this weight class. This is the technical core of what makes GenSense different. CodeQL has temporal analysis but it requires writing QL queries. Semgrep has no temporal analysis. Clippy has hand-coded temporal checks for specific patterns but no general mechanism.

The specific combination — YAML-authored temporal rules compiled to NFAs, run over a graph-derived event sequence, with balanced counting and reset conditions — is our algorithm. It does not exist elsewhere in this form.

---

## Algorithm 4: Interprocedural Call Graph Construction

### What it is

A call graph is a directed graph where:
- Each node is a function
- Each edge `A → B` means "function A calls function B"

Building a call graph lets you answer questions like: "if this function's input is tainted, which functions downstream also receive tainted data?"

For static analysis without running the code, you build an approximate call graph by scanning the source text for call expressions and matching them to function definitions by name.

### How GenSense uses it right now

File: `src/engine/auditor/discovery.rs::scan_for_edges`

The current implementation:
- Runs a tree-sitter query to find all call expressions in a file
- For each call, finds the enclosing function (the caller)
- Records the pair `(caller_name, callee_name)` as a string tuple
- These tuples are later added to the `SemanticGraph` as `EdgeKind::Calls` edges

**What is weak:**
- Name-based matching only. If two functions are both called `process`, the call graph cannot distinguish them. The edge connects the call to all functions named `process` in the registry.
- No handling of method calls. `self.process()` and `processor.process()` both produce the callee name `process`, but they likely refer to different functions on different types.
- Module paths are ignored. `utils::process()` and `process()` are treated as different callees even if they resolve to the same function.
- The call graph is built but its only consumer is the taint BFS. It is not used for any other analysis (dead code detection, call depth analysis, etc.).

### What we can improve

**Improvement 1: Qualified name resolution.**

When a call expression contains `::` (Rust module paths) or `.` (method calls), extract the full qualified name rather than just the last segment. Store functions in the registry by their qualified name as well as their simple name.

```
"utils::process" and "process" both index to the same symbol
```

This reduces false connections in the call graph without requiring type information.

**Improvement 2: Call depth metric.**

Once the call graph is built, compute the maximum call depth from `main` (or from any public API entry point) to each function. Functions with very high call depth (depth > 10) are candidates for complexity warnings.

This uses a standard BFS from the entry point. The depth of each node in the BFS tree is its call depth. O(V + E) where V is the number of functions and E is the number of call edges.

**Improvement 3: Dead function detection.**

Functions that appear in the registry but have no incoming `Calls` edges AND are not public API (not `pub fn`) are dead code — they are never called. This is a standard graph reachability check: build the set of all reachable functions from all entry points, then report any function not in that set.

This is O(V + E) using BFS or DFS from entry points.

### What we own

Call depth metrics and dead function detection expressed as rules in the YAML DSL is something we can build quickly and that no lightweight tool currently offers as configurable, project-specific rules. A team can write:

```yaml
- id: "TEAM_EXCESSIVE_CALL_DEPTH"
  domain: "maintainability"
  check: call_depth
  max_depth: 8
  observation: "Function is more than 8 calls deep from the entry point."
```

This is a practical, useful rule that teams will actually want. It is within Path B because the call graph is already built — the metric computation is a small addition.

---

## Algorithm 5: Scope-Aware Symbol Resolution

### What it is

Symbol resolution answers the question: "when this name appears in the code, which declaration does it refer to?" In most languages, the same name can refer to different things depending on where in the code it appears — this is called lexical scoping.

For example:

```rust
fn outer() {
    let x = 1;
    fn inner() {
        let x = 2;  // different x — inner scope shadows outer
        use(x);     // refers to inner x, not outer x
    }
    use(x);         // refers to outer x
}
```

A scope-aware resolver tracks which declarations are "in scope" at each point in the program.

### How GenSense uses it right now

File: `src/semantics/data_flow/lookup.rs::find_definition`

The current implementation runs a tree-sitter query over the entire file looking for any declaration with a matching name. It returns the first match. This is scope-blind: if two functions in the same file have a local variable named `x`, the resolver may return the wrong one.

The `SymbolRegistry` stores all symbols by name in a flat `HashMap<String, Vec<NodeIndex>>`. Multiple symbols with the same name all live in the same bucket. Resolving a name requires iterating the bucket and matching by file path and line number.

**What is weak:**
- No scope tracking. The resolver cannot tell whether a name at line 50 refers to the declaration at line 10 or the declaration at line 60 in an inner scope.
- The `find_definition` function walks all the way to the root of the file every time it is called. For a file with 1,000 functions, this is O(n) per lookup.
- Shadowing is not handled. In a function with a parameter named `x` and a local variable also named `x`, the resolver will find one arbitrarily.

### What we can improve

**Improvement 1: Scope stack during event traversal.**

The `traverse_for_events` function already walks the AST in execution order. Add a scope stack (a `Vec<HashMap<String, NodeIndex>>`) that is pushed when entering a block and popped when leaving. When a new variable declaration is encountered, add it to the current scope frame. When resolving a name, search the stack from top (innermost scope) to bottom (outermost scope). First match wins.

This is the standard textbook algorithm for lexical scope resolution and eliminates the class of false positives caused by scope confusion.

The scope stack is maintained during the traversal pass that already runs. The cost is a small amount of additional memory per function (proportional to nesting depth, not file size).

**Improvement 2: Block-local symbol index.**

Instead of one flat global index, maintain a per-function symbol index: `HashMap<FunctionId, HashMap<String, NodeIndex>>`. Lookups within a function only search that function's index, reducing lookup time from O(all symbols with that name across the project) to O(all symbols with that name in that function).

### What we own

A scope-aware resolver built directly into the event traversal pass, with a scope stack that mirrors the AST structure, is more precise than what most lightweight tools use. Most linters use flat name resolution. We are adding scoped resolution as a first-class property of the analysis, which directly reduces false positives in taint and temporal rules.

---

## Algorithm 6: Semantic Similarity for Documentation Drift

### What it is

Documentation drift means the doc comment says one thing and the code does another. Detecting this requires comparing the meaning of natural language text (the comment) against the structure of code (the function body).

For GenSense's purposes, we do not need full natural language understanding. We need only to check specific claims:
- "Returns None if X" → the return type must be `Option<T>` and the body must contain `None`
- "Returns Err if X" → the return type must be `Result<T, E>` and the body must contain `Err(...)`
- "Panics if X" → the body must contain `panic!`, `unwrap()`, or `expect()`
- "Thread-safe" → the type must implement `Send` and `Sync`

Each claim is a pattern-match problem, not a semantic understanding problem.

### How GenSense uses it right now

It does not. This algorithm does not yet exist in GenSense. It is one of the 10 new rules to be built (Rule 5: `AI_DOC_IMPL_DRIFT`).

### What we can improve

**The algorithm to build:**

1. Extract the doc comment block above a function. In Rust, doc comments are `///` lines. In TypeScript, they are `/** */` blocks. Tree-sitter has node kinds for both.

2. Scan the comment text for trigger phrases using a small table of patterns:

```
"returns None"      → check: return type contains "Option", body contains "None"
"returns Err"       → check: return type contains "Result", body contains "Err("
"returns null"      → check: body contains "null" or "undefined" (TypeScript)
"panics if"         → check: body contains "panic!" or ".unwrap()" or ".expect("
"will fail"         → check: body contains "Err(" or "return Err"
"not thread-safe"   → check: function is not marked async, no Mutex/Arc in body
"idempotent"        → check: body contains no state mutation (no `mut`, no `push`, no `insert`)
```

3. For each trigger phrase found in the comment, run the corresponding check on the function body. If the check fails, emit an advisory.

This is O(comment_length + body_length) per function. It is a pattern matching pass, not a semantic analysis pass.

**Improvement: Claim extraction with confidence.**

Not all phrases are equally specific. "May return None in some cases" is weaker than "always returns None when X". Add a confidence weight to each trigger phrase: specific claims (always, never, must) are flagged as Critical violations if the code contradicts them. Vague claims (may, sometimes, might) are flagged as Warning.

This is a simple lookup table:

```
High confidence triggers: "always returns", "never returns", "must", "guaranteed"
Low confidence triggers: "may return", "might", "sometimes", "can"
```

### What we own

A claim-extraction and code-verification pass that works across both Rust and TypeScript, driven by a configurable trigger phrase table in YAML, is novel for a lightweight tool. The key insight — that LLM-generated docstrings often describe the LLM's intent rather than what the code actually does — makes this a 2026-specific detector. This algorithm did not matter before LLMs were writing code. It matters now.

---

## Algorithm 7: The Analysis Pipeline Itself

### What it is

The analysis pipeline is how all the individual algorithms are connected and run. Currently it runs in three sequential phases (symbol discovery, edge discovery, event discovery) followed by parallel rule execution. The pipeline is the skeleton everything else hangs on.

### How GenSense uses it right now

File: `src/engine/project/mod.rs`

The pipeline for a multi-file project:
1. Walk the directory to find all source files
2. For each file, run symbol discovery (tree-sitter query → Symbol list)
3. Build the global `SymbolRegistry` from all symbols
4. For each file, run edge discovery (scan for call expressions → add Calls edges)
5. For each file, run event discovery (traverse AST → emit TemporalEvent nodes)
6. For each file, run all rules in parallel (Rayon) against each matching AST node
7. Collect all advisories and return them

**What is weak:**
- Steps 2–5 are sequential per file but not parallelised across files. Parsing 100 files one at a time when you have 8 CPU cores available is leaving 7 cores idle.
- The symbol registry is built globally and then shared across all rule executions. If two rules both read from the registry at the same time, there is no contention — but if the registry is ever written to during rule execution, there would be a data race. Currently this is safe, but it is fragile.
- There is no incremental analysis. Every run re-parses every file even if only one file changed. For a 50-file project this is fast enough. For a 500-file project it becomes noticeable.

### What we can improve

**Improvement 1: Parallel file parsing.**

Wrap the per-file symbol and event discovery in Rayon parallel iterators. Each file is independent during phases 2–5 (symbol discovery does not depend on other files' symbols). After all files are parsed in parallel, merge the results into the global registry in a single sequential merge step.

```rust
// Current (sequential):
for file in files {
    let symbols = discover_symbols(file)?;
    registry.extend(symbols);
}

// Improved (parallel):
let all_symbols: Vec<_> = files
    .par_iter()
    .map(|file| discover_symbols(file))
    .collect::<Result<Vec<_>>>()?;
for symbols in all_symbols {
    registry.extend(symbols);
}
```

Rayon is already in `Cargo.toml`. This is a small code change with a potentially large speedup on multi-core machines.

**Improvement 2: File change detection (incremental mode).**

Before parsing a file, compute a hash of its contents (a fast hash like xxHash or even a file modification timestamp). Store these hashes in `.gensense-cache/file_hashes.json`. On the next run, skip files whose hash has not changed and reuse their cached advisories.

This transforms the analysis from O(all files) to O(changed files) for subsequent runs. On a 200-file project where only 3 files changed, the analysis runs roughly 67× faster.

The cache invalidation rule: if any file changes, re-analyse that file and all files that import it (using the call graph to determine the import dependency). This is conservative — it may re-analyse some unchanged files, but it will never miss a finding caused by a changed dependency.

### What we own

The incremental analysis cache, combined with the AI enrichment cache (from the AI reasoning layer), gives GenSense a distinctive performance characteristic: the first run is fast (under 5 seconds on 50 files), and every subsequent run on a large project is dramatically faster. No other tool at this weight class offers both semantic depth and incremental performance together.

The file hash cache is simple to implement (it is just a JSON file on disk) but it is high-leverage. Once built, it changes how developers perceive the tool: it stops feeling like "a thing that runs" and starts feeling like "a thing that watches."

---

## Summary Table

| Algorithm | Current state | Key improvement | What we own |
|-----------|--------------|-----------------|-------------|
| N-gram fingerprinting | Computed but never compared | Add comparison pass + token normalisation | LLM copy-paste detection via structural similarity |
| Taint propagation | Flat single-origin registry | Taint sets (multi-source) + sanitiser modelling | YAML-configurable multi-source taint rules |
| Temporal event ordering | Integer counter sequence check | Compile rules to NFAs + balanced counting | YAML-authored NFA temporal rules |
| Call graph construction | Name-only, no qualification | Qualified names + call depth metric + dead code | Call depth and dead code as YAML-configurable rules |
| Scope-aware resolution | Flat global lookup | Scope stack during traversal | Scoped resolution built into event traversal |
| Documentation drift | Does not exist | Claim extraction + code verification | LLM-specific doc/impl mismatch detector |
| Analysis pipeline | Sequential per-file | Parallel parsing + incremental file cache | Incremental analysis with semantic depth |

---

## What to Build and When

**Phase 1 (bug fixes, weeks 1–2):** No algorithm work. Fix the three bugs. Establish the baseline.

**Phase 2 (DSL, weeks 3–8):** No new algorithms. Wire temporal YAML to the existing temporal engine.

**Phase 3 (pattern library, weeks 9–16):**
- Build the fingerprint comparison pass (Algorithm 1 improvement) for Rule 3 (copy-paste detection)
- Build the doc/impl drift detector (Algorithm 6) for Rule 5
- Use the existing taint system as-is for other rules — do not upgrade yet

**Phase 4 (AI layer, weeks 17–24):** No algorithm work. Wire the Anthropic API.

**After v1 ships:**
- Upgrade taint to multi-source sets (Algorithm 2 improvement)
- Upgrade temporal to NFA compilation (Algorithm 3 improvement)
- Add parallel file parsing (Algorithm 7 improvement)
- Add incremental file cache (Algorithm 7 improvement)
- Add call depth metric (Algorithm 4 improvement)

The post-v1 improvements are all Path B. They make existing analyses faster and more precise. None of them add new kinds of analysis. None of them compete with CodeQL's depth. All of them serve the core goal: fast, precise, easy to author.