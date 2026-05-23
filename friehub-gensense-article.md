# We Built a Static Analyser for the Agentic Era. Then We Hit Rice's Theorem.

**By Friehub**

---

## The Complaint Everyone Is Making

Walk into any engineering discussion about AI-assisted development and you will hear the same thing. Developers with years of experience saying LLMs write bad code. Production incidents getting blamed on AI. The criticism is loud enough that many engineers have decided LLMs have no real value for serious software work.

I understand the frustration. But I think most of it is aimed at the wrong target.

The problem is not that LLMs are bad at programming. The problem is that we have been using them without building the infrastructure that makes their output trustworthy. We hand a model a vague prompt, get vague code back, and call it a failure. That is the same outcome you would get handing a vague spec to a junior developer. The tool reflects the quality of the context you give it.

At Friehub we took a different approach. Rather than complain, we asked a precise question: **what kind of mistakes do LLMs make, and why do they make them?**

The answer to that question is what became Gensense.

---

## What LLMs Actually Get Wrong

LLMs do not make random mistakes. They make systematic, predictable ones, all driven by the same failure mode: they optimise for locally plausible output rather than globally correct behaviour.

When an LLM writes a payment service, it reaches for `f64` for the amount field because that is what numeric values look like in training data. It has no model of what happens when floating point arithmetic accumulates rounding errors across ten thousand transactions. It does not know that `0.1 + 0.2 != 0.3` in binary floating point. It knows what the code looks like, not what it costs when it fails.

When an LLM writes a database query in a Rust service, it writes `FROM orders WHERE user_id = $1` because that is what SQL looks like in training data. It does not know that Prisma (the ORM used in the project) creates a table named `"Order"` with a column named `"userId"`. The query returns zero rows in production, silently, with no error.

When an LLM writes an event publish inside a database transaction, it looks correct. The code compiles. The tests pass. In production, when the transaction rolls back, the event has already been published. Downstream consumers act on state that was never committed.

None of these are exotic bugs. They are the same bugs, appearing in every project, because the model's heuristic for "this is how you write this" is wrong in the same direction every time.

That is the insight behind Gensense. **The failure patterns are predictable. Predictable patterns can be encoded as rules. Rules can be enforced as gates.**

---

## Where It Started: The Python Auditor

Before there was a Rust codebase called Gensense, there was a Python tool called **Friehub Auditor**, a purpose-built audit platform embedded in the TaaS Gateway at `taas-gateway/friehub-auditor/`.

It was not a simple script. It had a proper architecture: a `SemanticBrain` that learned "Golden Patterns" from the codebase and tracked "Architectural Drift" using bigram and trigram frequency tables. A YAML-driven `RedlineEngine` for deploying new rules without touching Python. An `AIGuardianDetector` that scanned function blocks for chatty AI-isms, redundant docstrings, and phantom logic. An `NgramDetector` that computed Shannon entropy over 50-token windows and flagged code segments below a predictability threshold of 3.2 bits.

The `SemanticBrain` persisted its learned patterns to disk between runs using an atomic snapshot-and-rename write strategy. It tracked which rules were being suppressed most frequently, storing the counts in what the code called a "noise" register, so you could identify rules that developers found irritating rather than useful. It could compute surprisal scores per file relative to the project's historical baseline.

This was not a naive tool. It reflected real thought about the problem of AI-generated code quality.

It also produced over three thousand findings on a moderately sized project, with more than two thousand of them being noise.

The reason was architectural, not incidental. The `SemanticBrain`'s n-gram model learned statistical patterns over raw token sequences. A token sequence that looks unusual by entropy is not the same as a token sequence that is semantically wrong. The entropy threshold had no knowledge of what the code was supposed to do. It only knew whether the token stream was statistically predictable. A correct but unusual algorithm would trip the detector. A subtly broken but conventional-looking function would not.

The `AIGuardianDetector`'s regex approach had the same problem in reverse. It matched patterns like `"this function ensures"` or `"feel free to"` in raw text. A comment inside a test fixture explaining what a rule catches would also trigger the rule. There was no scope boundary. There was no concept of "this text is inside a comment, inside a test, inside a function that exists to verify the detector works."

The YAML `RedlineEngine` rules matched forbidden patterns as raw strings or simple regexes against file content. Every rule could fire on every line of every file. As the rule count grew, so did the false positive rate, combinatorially.

We had no architectural boundary between what the engine did and what the rules said. Rules were data processed by Python string operations. The more rules we added, the more the signal degraded.

---

## The First Attempt to Save It

Before abandoning the Python approach, we tried adding more mathematics to it. Probabilistic models. More sophisticated n-gram analysis. A rolling drift baseline that normalised surprisal scores against the last hundred files scanned rather than against a fixed threshold.

The Neural-Lite 2.1 upgrade added LSH-based pattern recognition and Jaccard similarity scoring for near-duplicate block detection. The `AIGuardianDetector` used fingerprint buckets with hash-based bucketing to detect boilerplate across files in O(N log N) time. The `BlockBasedDetector` added tree-sitter integration as a priority-one path for splitting source into semantic blocks, with regex-based block parsing as a fallback.

Every upgrade made the tool more sophisticated. None of them addressed the root cause, which was not the sophistication of the detector but the surface it was operating on. Statistical analysis over token text is the wrong substrate for reasoning about semantic correctness.

---

## Hitting Rice's Theorem

Before the Rust rebuild we tried something else. We tried adding more mathematics to the Python version: probabilistic models, more sophisticated pattern matching, statistical learning on the rule outputs.

Every approach ran into the same wall.

In 1953, Henry Gordon Rice proved that any non-trivial semantic property of programs is undecidable. You cannot write a program that, for all possible programs, correctly determines whether they satisfy a semantic property. Not with more compute. Not with better algorithms. It is a fundamental impossibility result, in the same family as the halting problem.

We hit Rice's theorem. Not in a textbook. In practice, while trying to build a tool.

The right response to Rice's theorem is not to give up. It is to accept the boundary and work inside it deliberately. Every static analyser makes this choice. The question is not "can we decide all semantic properties" (the answer is no). The question is "which decidable approximations are useful enough to be worth building."

We made that choice explicitly. Gensense does not try to prove programs correct. It catches the systematically decidable failure patterns (structural violations, ordering constraints, schema mismatches, hollow implementations) with high confidence and low false positives. Everything is scoped, honest, and bounded.

That discipline is what made Gensense work.

---

## The Rebuild: May 9, 2026

The rename happened on May 9, 2026. The commit message was direct: `chore: rename TaaS to GenSense`. That day, a new Rust codebase replaced the Python tool as the primary audit engine for Friehub's codebases.

The decision to rebuild in Rust was not about performance. It was about the AST.

Tree-sitter gives you a real abstract syntax tree, not a token stream or a regex match, but a structured parse of the program that reflects how the language grammar works. A rule that operates on an AST node operates on a semantic unit. A function node contains its parameters, its return type, its body. A call expression contains the callee and the arguments. When you write a rule that fires on an `arrow_function` node that contains a `publishEvent` call inside a `$transaction` callback, you are reasoning about the structure of the code, not about whether a certain string appears within a certain number of characters of another string.

That is the difference that eliminates false positives. Scope is structural, not textual.

The architecture was rebuilt from the ground up with three clear separations:

**The engine** handles AST parsing, symbol discovery, call graph construction, taint tracking, and rule execution. It knows nothing about what rules say.

**The rules** are YAML files that declare what patterns to match, what the impact is, and how confident the finding should be. They know nothing about how the engine works.

**The advisory output** carries enough context for both humans and agent loops to act without ambiguity: confidence scores, `auto_fixable` flags, `requires_human` flags, proposed replacements, and SRI fingerprints anchored to symbols rather than line numbers.

---

## The Early Bugs

The Rust rewrite did not ship clean.

**v0.1.x** had CI that could not publish past version 0.1.4. The NPM publish job called `cargo run` before Rust was installed in that CI step, silently failing for weeks while the crate was never distributed. The `aarch64-apple-darwin` cross-compilation ran on an x86 macOS host instead of Apple Silicon, producing wrong-architecture binaries.

**v0.2.0** (May 13) shipped the graph-first semantic engine: a global symbol graph for inter-procedural taint analysis across files, a multi-pass audit loop, and the first cross-file project rules: `MustHaveGuard`, `MustBeInternal`, `CrossFileTaintFree`. But it had a critical bug in the BFS traversal. The visited set was keyed by function name alone. In any real project with common names like `new`, `run`, or `handle`, the BFS would terminate early when it saw a name it had already visited in a different file. Guards that existed were not found. Taint paths were missed. The rules gave wrong answers silently.

**v0.2.1** (May 13) fixed the BFS bug by keying the visited set by `(name, file_path)` tuple. It also fixed a second silent failure: the Node.js `audit_content` API, which editors and integrations used as the primary entry point, did not run project rules at all. No warning. No error. Cross-file rules simply did not execute when called from JavaScript. The fix added an explicit `audit_project()` method and updated the JSDoc to make the distinction transparent.

**v0.2.2** (May 14) added Solidity beta support and enhanced temporal rule DSL.

**v0.3.0** (May 21) was the biggest release. It uncovered one more class of architectural bug in the existing engine: Rayon parallelism. The engine used `into_par_iter()` for the audit phase, which introduced a class of futex deadlocks under certain tree-sitter query patterns. The fix was to remove Rayon entirely from the snapshot collection and audit phases and go back to sequential iteration. The correctness trade-off was worth it: the engine's single-pass architecture meant that eliminating parallelism had a smaller impact on throughput than expected. 4.5 seconds for 68 files is still fast enough for CI.

v0.3.0 also fixed the MCP server's handling of JSON-RPC requests where the `id` field was `null`. The server would hang indefinitely because `Option<Value>` could not distinguish between an absent `id` and a null `id`. The fix was a purpose-built `RequestId` enum with a custom deserializer: `Absent | Null | Value`. A small fix. The kind of bug that only appears when you integrate with real MCP clients that do not follow the spec precisely.

---

## What Gensense 0.3.0 Actually Is

Gensense is a semantic analysis engine for TypeScript and Rust. It uses tree-sitter to parse source files into real abstract syntax trees, then runs a set of analysis passes over those trees.

**The rule types it supports:**

*Pattern rules* match AST node types with regex or tree-sitter queries. This is the baseline, catching `async forEach`, non-null assertions on context fields, `f64` in monetary variable names.

*CSA rules (Contract Surface Analysis)* check that functions whose names imply a contract actually satisfy it. A function named `validateInput` that has no rejection path is flagged. A function named `parseRequest` that never returns an error is flagged. This catches the hollow implementation pattern: code that looks like it does something without actually doing it.

*Temporal FSA rules* check that operations occur in the correct order within a function scope. Events must not be published before transactions commit. Balance must be checked before a debit is created. Authentication must precede data writes. These are the ordering violations that type systems cannot express.

*Taint analysis* tracks sensitive data from sources to sinks. Password fields must not flow to log outputs. User-controlled input must not flow to raw SQL strings. The taint engine follows variable assignments and field accesses through the function scope, with field-path taint propagation that tracks taint through object properties rather than just variable names.

*Project rules (must_have_guard)* verify call graph properties across files. A function that creates a ledger debit must have a reachable balance check. A reserve operation must have a reachable release operation.

*Schema contract rules* are the most novel. They capture a string literal from a source file (a table name in a raw SQL query in a Rust file, for example) and verify it exists in a schema file from a different language. A Rust service querying `FROM orders` against a Prisma schema that defines a model named `Order` (PascalCase, as Prisma generates) is caught by this rule. No other static analysis tool has a rule type that crosses this boundary.

Every finding carries a confidence score, `auto_fixable` and `requires_human` flags, and a `proposed_replacement`. The engine returns a single `clean: bool` value that agent loops can use as an exit condition.

---

## From 3024 Findings to 513

We ran both versions of Gensense against the same production codebase: a marketplace application with hundreds of files spanning a TypeScript API layer and Rust microservices.

The Python Friehub Auditor: 3024 findings, more than 2000 false positives.

Gensense 0.3.0: 513 findings, all real.

The difference is not that we wrote fewer rules. It is that the rules are AST-aware rather than text-aware, confidence-scored rather than binary, and scoped rather than global. A rule that fires on every string that looks like a URL is not useful. A rule that fires when a URL string is used in an `Authorization` header without going through a configuration layer is useful.

The 513 findings included the exact bugs we described earlier: the SQL table name mismatch, the event inside a transaction, the wallet debit without a balance check. None of them were caught by the type checker. None of them were caught by the linter. All of them would have caused production failures.

---

## The Benchmark Numbers

The engine is fast enough to run on every save in an IDE extension or on every commit in CI.

Single file scan time is approximately **16 milliseconds**, consistent across clean files and files with multiple violations. The AST is parsed once and cached. All rules run in a single pass over the cached tree. Rule firing adds less than one millisecond of overhead regardless of how many rules match.

Project scale is **linear**:
- 10 files: 133ms
- 50 files: 568ms
- 100 files: 1.1 seconds
- 200 files: 2.2 seconds

200 files in 2.2 seconds is the proof that the single-pass architecture is correct. The engine does not slow down as you add rules because rules share the parse cost.

Taint analysis at 100-variable chain depth takes 17ms, only 1.5ms more than at 5-variable depth. Linear scaling confirmed.

The SchemaContractChecker parses a 20-model Prisma schema and extracts all model names in **46 microseconds**. It costs essentially nothing.

Symbol registry lookup at 100,000 symbols takes **51 nanoseconds**, three nanoseconds more than at 1,000 symbols. That is O(log n) confirmed by measurement.

Engine cold start: **97 nanoseconds**.

These numbers come from a real CI benchmark suite running on every commit, with regression detection that fails CI on any benchmark that slows by more than 10%.

---

## The MCP Server and the Agent Loop

Gensense 0.3.0 ships with an MCP server. This is the feature that connects everything.

MCP (Model Context Protocol) is the interface that allows LLMs to call external tools. With the Gensense MCP server, an AI agent running in Claude Code or any MCP-compatible environment can call `gensense_audit` on a file it just wrote and receive a structured response:

```json
{
  "clean": false,
  "advisories": [...],
  "auto_fixed": 0,
  "requires_human": [...]
}
```

`clean: false` means the agent cannot consider its work done. It must address the advisories before the loop can exit. This is the enforcement model that prompts cannot provide.

A prompt tells the LLM what to do. A Gensense advisory tells the agent that what it did was wrong, precisely where, and what the fix should be. The agent cannot claim the code is correct. It has to actually pass the gate.

This is the difference between advisory and constraint. LLMs do not reliably follow advisory guidance. They understand the instruction, but the locally plausible output pulls in a different direction. A constraint with a boolean exit condition changes the incentive structure of the generation loop entirely.

---

## The Architecture Decision Behind the Rules

One thing worth understanding about how Gensense rules work: they are YAML files, not code.

A developer writing a Gensense rule does not write Rust. They write a YAML declaration that describes what the rule catches, what the impact is, what the fix is, and how confident the engine should be. The engine compiles the rule at startup.

```yaml
- id: TS_EVENT_INSIDE_TRANSACTION
  name: Event Published Inside Prisma Transaction
  target_ext: ts
  on_node: arrow_function
  if_matches: "\\$transaction\\s*\\("
  must_not_contain: "publishEvent"
  severity: Critical
  category: Reliability
  observation: publishEvent is called inside a Prisma transaction body.
  impact: If the transaction rolls back the event is already published.
  improvement: Move publishEvent calls to after the transaction resolves.
  confidence: 0.85
  auto_fixable: false
  requires_human: false
```

This is the same design principle the Python `RedlineEngine` was aiming for: rules as YAML, not code. The Python engine compiled those YAML rules into regex operations over raw text. The Gensense engine compiles them into tree-sitter queries and AST node predicates. The YAML surface looks the same. The substrate is completely different.

The knowledge that makes LLM output trustworthy is encoded in a form any developer can read, write, and contribute. The failure patterns you have learned from experience (ordering violations, financial precision requirements, authorization gaps) become rules. The rules become gates. The gates run on every commit.

The accumulated knowledge of your engineering team, encoded once, enforced automatically, forever.

---

## What This Has to Do with Church and Curry-Howard

We have been studying formal language theory and type theory to understand where Gensense sits in the broader landscape of CS theory. It turns out it sits in a very specific place.

Alonzo Church's lambda calculus, published in 1936, showed that all computation can be expressed in three rules: variables, abstraction (functions), and application (calling functions). Every programming language, regardless of syntax, is the lambda calculus with a concrete surface and a type system on top.

The Curry-Howard correspondence, developed through the 1950s and 1960s, showed that the lambda calculus and formal logic are the same object viewed from different angles. A function type `A → B` is simultaneously a proof that if A holds then B holds. A well-typed program is simultaneously a proof of its type. A type checker is simultaneously a proof verifier.

What this means for Gensense: every rule is a proposition about the program. `TS_ASYNC_FOREACH` says "this program does not have the property of handling all async errors." `RUST_SQL_SNAKE_CASE_TABLE` says "this program does not satisfy the contract that table names match the schema." `clean: true` means all propositions are satisfied. `clean: false` means at least one proposition is violated.

Gensense is, formally, a proposition checker over ASTs. The rules are propositions. The engine checks them. Rice's theorem tells us which propositions are decidable. We only write rules for decidable propositions. That is why the false positive rate is low and the confidence scores are honest.

The type system catches the propositions expressible in the type language. Gensense catches the propositions about domain constraints that types cannot express. They are complementary layers of the same formal system.

---

## Why LLMs Do Not Replace Your Knowledge

Here is the point that the critics miss.

LLMs made the knowledge of senior engineers more valuable, not less. When a junior developer writes code, the senior engineer reviews it and catches the violations of domain knowledge: ordering constraints, financial precision requirements, authorization gaps. That review is the knowledge.

With LLMs generating code, the review surface is larger but the knowledge required for the review is the same. The senior engineer who understands why `f64` breaks financial systems is more valuable in an AI-assisted team than one who doesn't, because more code is being generated that contains that mistake.

Gensense is that knowledge made into infrastructure. The rules encode what senior engineers know about production failure modes. Once encoded, the review happens automatically on every commit, for every developer, without human attention. The knowledge scales.

This is not the LLM replacing the engineer. This is the engineer's knowledge constraining what the LLM can ship. The constraint is only as good as the knowledge behind it. Deep CS knowledge (formal language theory, program analysis, type theory) makes the constraints more precise, more complete, and more honest about their own boundaries.

Studying Church and Curry-Howard does not make you better at writing prompts. It makes you better at building the infrastructure that makes LLM output trustworthy. That is a different and more durable skill.

---

## What Is Coming

Gensense 0.3.0 is the foundation. The roadmap through 0.5.x addresses four more systematic failure patterns.

**Git-aware analysis:** `gensense . --diff-only` flags only advisories in symbols that changed in this branch versus main. No more teams ignoring five hundred pre-existing issues because they cannot tell which ones are new. The SRI fingerprinting that already exists in the engine makes this clean: it tracks symbol identity, not line numbers.

**AI hallucination detection:** LLMs invent API calls that do not exist. `import { magicFunction } from '@company/sdk'` where the export does not exist in the package. `use nonexistent_crate::Something` where the crate is not in Cargo.toml. Simple resolution against the dependency tree. High value, no configuration required.

**Secrets with AST context:** entropy scanners flag anything random-looking, producing enormous noise. Gensense can ask whether a string is used as a credential: passed to an `Authorization` header, assigned to a `password` field, used in a database connection string. The AST tells you the usage context, which filters ninety percent of false positives while catching secrets that entropy scanners miss entirely.

**N+1 query detection and performance anti-patterns:** a `findUnique` call inside a loop where `findMany` with `include` would do. Unnecessary `clone()` in Rust hot paths. `std::sync::Mutex` in async code where `tokio::sync::Mutex` is required. These are the performance bugs that look correct, compile cleanly, and destroy throughput under production load.

**N-gram style baseline:** learned from the project itself, not from written rules. Build a statistical fingerprint of the codebase's patterns (naming conventions, structural patterns, call patterns) and flag code that is statistically alien to the project. This is what the Python Friehub Auditor was trying to do with its `SemanticBrain`. The difference is that the baseline will operate over AST node sequences, not raw token sequences. The substrate matters. The same idea on the right substrate produces a different result.

---

## The Thesis

We started from a simple observation: LLMs make systematic, predictable mistakes because they optimise for locally plausible output without a model of production consequences.

The response to that observation is not to complain, prompt differently, or wait for better models. The response is to build infrastructure that makes the systematic failures expensive to ship. Gates with boolean exit conditions that the agent loop cannot pass until the violation is resolved.

Gensense is that infrastructure. It took a Python tool called Friehub Auditor that accumulated too much noise to be useful, a dead end while trying to add more mathematics to a text-based approach, and a complete rebuild in Rust (renamed to Gensense on May 9, 2026) with a clear scope, real ASTs, and honest confidence bounds.

It took three more months of bugs after the rename: CI that could not publish its own binaries, a BFS traversal that silently gave wrong answers on multi-file projects, a JavaScript API that omitted an entire class of rules without warning, and a parallelism approach that introduced a deadlock class in production workloads.

The result of fixing all of it is a tool that went from 3024 noisy findings to 513 real ones. An engine that scans a file in 16 milliseconds and a 200-file project in 2.2 seconds. A SchemaContractChecker that catches cross-language schema drift that no other tool catches. An MCP server that makes `clean: bool` a first-class primitive in the agent loop.

The critics are right that LLMs produce bad code when used without infrastructure. They are wrong that the solution is to abandon the tools. The solution is to build what is missing.

That is what engineers do.

---

*Gensense is open source and available at [@friehub/gensense](https://github.com/Friehub/gensense). Built at [Friehub](https://friehub.dev).*

*The Python predecessor described in this article, Friehub Auditor, is published as a read-only archive at [@friehub/friehub-auditor-archive](https://github.com/Friehub/Friehub-auditor) for readers who want to inspect the original implementation.*
