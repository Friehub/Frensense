# Frensense: Example-Driven Code Analysis

Frensense detects semantic bugs — code that compiles and type-checks but doesn't do what it says it does. It works by comparing code *shape* against known violation patterns, then confirming hits with structural analysis.

No YAML. No regex rules. No `on_node`. Detection is driven by example pairs — a file showing the bug and a file showing what correct code looks like.

---

## What It Catches

**Security — credential leaks and injection**
- Plaintext passwords, tokens, and API keys reaching database writes, network calls, or log statements
- Untrusted input flowing to shell exec, filesystem operations, or HTTP requests without sanitization
- Credentials in logs captured by monitoring systems indefinitely

**Concurrency — deadlocks and resource leaks**
- Mutex guards held across `.await` points in async Rust
- `lock()` without matching `unlock()` in the same scope
- Thread sleeping while holding a lock

**Validation — hollow implementations**
- `validate_*()` functions with no reachable rejection path — they always return true
- `check_*()` / `verify_*()` / `sanitize_*()` that pass input through unchanged
- Taint entropy: functions named like validators that never branch on tainted data

**LLM-generated anti-patterns**
- Near-duplicate functions (85%+ similar) with divergent security behavior
- Hallucinated imports referencing crates or packages that don't exist in the lockfile
- Hollow validators — plausible-looking code that never actually checks the input

**Secrets and infrastructure**
- Hardcoded AWS keys, GitHub tokens, JWT, private keys, database connection strings in source

**Style anomalies**
- Functions structurally dissimilar from the rest of the codebase (LLM fingerprinting)

---

## How Detection Works: The Four-Layer AND Gate

A finding only emits when multiple independent layers confirm it. This keeps false positives low as the detection corpus grows.

### L1: Corpus Pattern Match

Every function is fingerprinted — its token n-grams, structural markers, signature patterns, and type usages are hashed into comparable sets. The engine compares each function against a corpus of positive/negative example pairs:

```
corpus/targets/rust_clone_in_loop_positive.rs   ← this IS a bug
corpus/targets/rust_clone_in_loop_negative.rs   ← this is NOT a bug
```

Score = similarity to positive × (1 − similarity to negative). Above threshold → candidate.

Adding a new detection is copying two files. No code changes needed.

### L2: Taint Path Confirmation

If the corpus matches, the engine traces whether tainted data actually flows from a source to a sink. Six security-critical taint flows are hardcoded:

| Source | Sink | Severity |
|--------|------|----------|
| password, secret, token, credential | db insert/update/query/execute | Critical |
| user input, request body, param | exec, system, shell, command, eval | Critical |
| password, secret, token | log, console, print, debug | Warning |
| user input, path, file_name | filesystem write/open/remove | Warning |
| user input, request, body, header | HTTP fetch/request/post/get | Warning |
| password, secret, api_key, token | HTTP fetch/url/endpoint | Critical |

Sanitizer recognition: `html_escape(x)`, `bcrypt.hash(x)`, and similar calls clear taint on the return value.

### L3: Taint Entropy Verification

A function named `validate_input()` that corpus-matched but never branches on its tainted input is a hollow implementation. The engine computes:

```
taint_branch_ratio = conditionals referencing tainted vars / total tainted uses
```

Ratio < 0.2 in a validation-named function → finding suppressed or confidence reduced. This catches the LLM failure mode: plausible code with no actual logic.

### L4: Cross-Function Consistency (MinHash)

If two functions are 85% structurally similar but have different taint sink behavior, one is likely a copy-paste that lost its security fix. This catches the case where an LLM generates two similar functions and only one gets the validation logic correct.

---

## Corpus-Driven Detection

Detection patterns live in `corpus/targets/`. A pattern is two files:

```
{lang}_{pattern_name}_positive.{ext}  ← the bug
{lang}_{pattern_name}_negative.{ext}  ← correct code
```

Optional `.toml` sidecar for advisory text:

```toml
id = "RUST_CLONE_IN_LOOP"
severity = "Warning"
observation = "A .clone() call was found inside a loop body."
impact = "Repeated allocation inside a loop degrades performance."
improvement = "Pull the clone outside the loop or use a reference."
```

CLI usage:

```bash
frensense . --corpus corpus/targets/ --threshold 0.65
frensense --list-patterns                    # show loaded patterns
frensense --list-patterns --corpus my-rules/ # show custom corpus
```

The engine ships with 30 patterns (60 example files) across Rust and TypeScript. Adding a pattern is copying two files — no compiler changes, no YAML, no DSL.

---

## Multi-Language Support

Five languages, one abstract kind taxonomy. Tree-sitter parses code into language-specific AST nodes. The mapper translates them into 32 abstract kinds (`FunctionDef`, `Call`, `Conditional`, `Loop`, `Match`, `Throw`, etc.) shared across all languages. A pattern trained on Rust examples matches TypeScript and Python code automatically.

| Language | Status |
|----------|--------|
| Rust | Complete |
| TypeScript / JavaScript | Complete |
| Python | Complete (opt-in: `--features python`) |
| C | Complete (opt-in: `--features c_lang`) |

Adding a language: add a tree-sitter dep, a mapper arm, and a query string. ~1 day.

---

## Architecture

### Engine (`frensense-engine`)

The analysis substrate. No rules, no CLI, no policy.

| Module | Capability |
|--------|-----------|
| `fingerprint` | Function n-gram hashing, structural markers, type usages |
| `lang` | AbstractKind taxonomy + per-language mapper |
| `corpus` | Pattern loading, LSH-indexed registry, weighted Jaccard scoring |
| `data_flow` | Owned `TaintRegistry`, `DataFlowEngine` with summary caching, `AliasTracker`, `TaintConfidenceAdjuster`, `TaintMetrics` (entropy), cross-file `Resolver` |
| `cfg` | Control flow graph with statement-level blocks, def-use chains with reaching definitions |
| `pattern` | AST pattern compiler, matcher, canonical form, scorer |
| `temporal` | Finite automaton over ordered event sequences (lock/await/unlock/etc) |
| `minhash` | MinHash signatures, LSH bucket index, Jaccard similarity |
| `secrets` | Regex + entropy-based secret scanner (AWS, GitHub, JWT, private keys) |
| `deps` | Dependency resolution against `Cargo.lock` / `package.json` |

### Consumer (`frensense`)

The CLI binary and analysis pipeline that wires engine primitives together:
- Project runner orchestrates symbol discovery → event chains → rule execution → corpus scan → taint analysis → confidence filtering
- MCP server for AI agent integration
- Reporter: text, JSON, SARIF output
- Patcher: automated remediation (experimental)

---

## MCP Server

Frensense ships a JSON-RPC 2.0 server over stdin/stdout for AI agent integration:

```json
{
  "method": "frensense_audit",
  "params": { "path": "/project/src/main.rs" }
}
```

Returns structured findings with confidence scores, auto-fixability flags, and human-readability indicators. AI agents call Frensense as a post-generation validator and self-correct.

---

## CI Integration

```bash
frensense . --strict                          # exit 1 on any finding
frensense . --severity critical                # only critical findings
frensense . --diff-only                        # only changed files
frensense . --baseline baseline.json           # suppress known findings
frensense . --json                             # JSON output
frensense . --sarif                            # GitHub Advanced Security
```

---

## What Frensense Does Not Do

- Run code (no dynamic analysis)
- Verify algorithmic correctness
- Detect runtime race conditions not expressible as temporal constraints
- Replace security audits for production financial/medical systems

It catches bugs detectable from source structure and data flow. For everything else, it surfaces findings that warrant review.
