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

## Detection Modules

Beyond corpus pattern matching, the engine runs these independent detection passes:

| Module | Rule ID | What it catches |
|--------|---------|-----------------|
| **Unused variables** | `UNUSED_VARIABLE` | `let x = 1;` where `x` is never read. Walks def-use chains, excludes function parameters. |
| **Dead branches** | `DEAD_BRANCH` | `if false { ... }` or `if true { ... }` — code that can never execute. |
| **Temporal violations** | `TEMPORAL_VIOLATION` | `lock()` without matching `unlock()`, `acquire()` without `release()` in the same scope. Finite automaton over ordered event sequences. |
| **Cross-file taint** | `CROSS_FILE_TAINT` | A function in file A calls a sink-named function in file B with tainted data from a source-named function. Graph walk across the project symbol table. |
| **Hallucinated imports** | `HALLUCINATED_IMPORT` | `use nonexistent_crate::foo` where the crate isn't in `Cargo.lock` or `package.json`. Dependency resolution against the lockfile. |
| **Secrets** | `SECRET_*` | Regex + entropy-based detection of AWS keys, GitHub tokens, JWT, private keys, database connection strings in source. |
| **Style anomaly** | `STYLE_ANOMALY` | Functions structurally dissimilar from the rest of the codebase (requires `--learn-profile` + `--check-profile`). |
| **Near-duplicate** | `NEAR_DUPLICATE_FUNCTION` | Two functions with 85%+ structural similarity — copy-paste detection. |

---

## CLI Reference

```
frensense [path] [options]
```

### Detection

| Flag | Description |
|------|-------------|
| `--corpus <dir>` | Load corpus patterns from a directory (extends built-in patterns) |
| `--threshold <0-1>` | Corpus match threshold (default: 0.65). Lower = more sensitive. |
| `--language <lang>` | Filter by language: `rust`, `typescript`, `javascript` |
| `--diff-only` | Only scan files changed since the last git commit |
| `--severity <level>` | Minimum severity: `critical`, `warning`, `info` |
| `--suite <level>` | Rule tier: `default`, `extended`, `all` |

### Confidence & Tuning

| Flag | Description |
|------|-------------|
| `--confidence <tier>` | Preset thresholds: `high` (≥0.85), `medium` (≥0.60), `low` (≥0.30), `any` (≥0.0) |
| `--min-confidence <0-1>` | Raw confidence floor (default: 0.0) |
| `--jaccard-threshold <0-1>` | Similarity threshold for near-duplicate detection |
| `--max-source-lines <N>` | Skip files with more than N lines |

### Taint Tuning

| Flag | Description |
|------|-------------|
| `--extra-taint-rules <dir>` | Load additional taint rules from TOML files in the directory |
| `--taint-conf-inter <0-1>` | Inter-procedural taint confidence (default: 0.80) |
| `--taint-conf-intra <0-1>` | Intra-procedural taint confidence (default: 0.90) |
| `--taint-max-depth <N>` | Max taint propagation depth (default: 5) |

### N-Gram Tuning

| Flag | Description |
|------|-------------|
| `--ngram-window <N>` | Token n-gram window size (default: 5) |
| `--min-ngram-count <N>` | Minimum n-gram occurrences (default: 3) |
| `--confidence-boost-rate <0-1>` | Confidence boost per overlapping rule (default: 0.10) |
| `--confidence-boost-max <0-1>` | Max confidence boost from overlaps (default: 0.30) |

### Rule Management

| Flag | Description |
|------|-------------|
| `--disable-rule <id>` | Suppress a specific rule (repeatable) |
| `--override-severity <id>:<level>` | Change a rule's severity, e.g. `--override-severity FILE_TOO_LONG:info` |
| `--tag <tag>` | Filter findings by tag (repeatable) |

### Output

| Flag | Description |
|------|-------------|
| `--json` | Output findings as JSON |
| `--sarif` | Output findings in SARIF format (GitHub Advanced Security) |
| `--strict` | Exit with code 1 if any findings match the filter |
| `--emit-baseline <file>` | Save current findings as a baseline file |
| `--compare-baseline <file>` | Compare current scan against a baseline (suppresses known findings) |
| `--update-baseline` | Update the baseline file with current findings |

### Auto-Remediation

| Flag | Description |
|------|-------------|
| `--fix [scope]` | Apply automated fixes. Scope: `all` (default), `style` (quality/dead-code), `security` (taint/secrets) |
| `--diff [scope]` | Show unified diff of proposed changes without applying them |

### Style Profiling

| Flag | Description |
|------|-------------|
| `--learn-profile` | Build a project style profile by fingerprinting the current codebase |
| `--check-profile` | Check code against the learned profile, flag style anomalies |
| `--profile-threshold <0-1>` | Surprise threshold for anomaly detection (default: 0.7) |
| `--profile-stats` | Display profile statistics after learning |

### Informational

| Flag | Description |
|------|-------------|
| `--list-patterns` | List loaded corpus patterns |
| `--debug <file>` | Dump anonymized AST for a source file |
| `--version` | Display version and enabled features |

---

## Taint Rules (TOML Format)

Taint rules are externalized to TOML files. The engine ships with 6 built-in rules in `taint_rules.toml`. Users can add custom rules.

### Built-in rules

| Rule ID | Source | Sink | Severity |
|---------|--------|------|----------|
| `TAINT_CREDENTIAL_TO_DB` | password, secret, token, credential | insert, update, db.execute | Critical |
| `TAINT_INPUT_TO_EXEC` | input, body, param, query | exec, system, shell, eval | Critical |
| `TAINT_CREDENTIAL_TO_LOG` | password, secret, token | log, console, print | Warning |
| `TAINT_INPUT_TO_FS` | input, body, path, file_name | write, open, remove | Warning |
| `TAINT_INPUT_TO_HTTP` | input, body, param, header | fetch, http, request | Warning |
| `TAINT_CREDENTIAL_TO_HTTP` | password, secret, api_key | fetch, http, url | Critical |

### Writing custom rules

Create a `.toml` file with `[[rules]]` entries:

```toml
[[rules]]
id = "CUSTOM_SANITIZE_BYPASS"
source = "input|user_input|req\\.body"
sink = "innerHTML|document\\.write|eval"
severity = "critical"
observation = "Untrusted input reaches an XSS sink without sanitization."
impact = "Cross-site scripting allows session hijacking."
improvement = "Use textContent instead of innerHTML. Sanitize with DOMPurify."
```

Load with:

```bash
frensense . --extra-taint-rules /path/to/my-rules/
```

Multiple directories can be passed (repeatable). User rules extend — they do not replace — built-in rules.

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

### Multi-example files

Each file can contain multiple functions. The loader extracts all parseable functions and generates one fingerprint per function under the same pattern name. Scoring takes the max score across all examples.

```
ts_csa_validate_unconditional_positive.ts
  → validateCredentials()  → fingerprint 1
  → formatOutput()         → fingerprint 2
```

4 functions per file, each varying along a different dimension (number of hops, source form, nesting depth), gives the strongest detection signal.

### CLI usage

```bash
frensense . --corpus corpus/targets/ --threshold 0.65
frensense --list-patterns                    # show loaded patterns
frensense --list-patterns --corpus my-rules/ # show custom corpus
```

The engine ships with 89 patterns (178 example files) across Rust and TypeScript. Adding a pattern is copying two files — no compiler changes, no YAML, no DSL.

---

## Corpus Bundle (FRC1 Format)

At scale (400 patterns, 3,200 fingerprints), parsing source files at startup is slow. The bundle pre-compiles all corpus fingerprints into a single binary blob embedded in the engine.

### Building

```sh
cargo run --bin build-corpus-bundle
```

Reads every `*_positive.*` and `*_negative.*` from `corpus/targets/`, extracts fingerprints, writes `frensense-corpus.frc` (~80KB for 89 patterns).

### Binary layout

```
u32 LE   header_length          (4 bytes)
BundleHeader (bincode)          (~52 bytes)
  magic:        [u8; 4] = "FRC1"
  version:      u32     = 1
  pattern_count: u32
  checksum:     [u8; 32] (blake3 of data section)
Vec<BundlePattern> (bincode)    (variable)
  id: String
  positives: Vec<FunctionFingerprint>
  negatives: Vec<FunctionFingerprint>
```

### What's in a fingerprint

`FunctionFingerprint` contains:
- `ngram_hashes` — token n-gram sets (hashed)
- `weighted_ngram_hashes` — IDF-weighted n-grams
- `structural_markers` — control flow structure hashes
- `signature_ngrams` — function signature hashes
- `param_type_ngrams` — parameter type hashes
- `type_usages` — referenced types
- `language`, `function_name`, `file_path`

No source text is stored. Fingerprints are one-way — you cannot reconstruct code from them.

### How it loads

The engine embeds the bundle via `include_bytes!("../frensense-corpus.frc")`. On startup:

1. Try embedded bundle → instant load, zero disk I/O
2. If bundle missing/corrupt/wrong version → fall back to `corpus/targets/` source files
3. User-provided `--corpus dir/` always loads from source (extends, not replaces)

### Versioning

The header contains a format version (`BUNDLE_VERSION = 1`). If the `FunctionFingerprint` struct changes, bump the version. The engine rejects bundles with a newer version to prevent deserialization mismatches.

### Adding a new pattern

1. Create `corpus/targets/{lang}_{name}_positive.{ext}` — the buggy code
2. Create `corpus/targets/{lang}_{name}_negative.{ext}` — the fixed code
3. Run `cargo run --bin build-corpus-bundle`
4. Commit both the source files and the updated `frensense-corpus.frc`

---

## Scoring Details

### Fingerprint matching

Each function is tokenized into n-grams (default window: 5 tokens). The scorer computes:

```
sim_to_positive = ngram_sim × 0.35
               + structural_sim × 0.30
               + signature_sim × 0.20
               + param_type_sim × 0.10
               + type_usage_sim × 0.05

score = sim_to_positive × (1 − sim_to_negative) × transfer_penalty
```

### IDF weighting

Rare tokens (`db::execute`, `bcrypt::hash`) score higher than common ones (`let x`, `return`). IDF weights are computed from the corpus and stored in the bundle. When weights are available, weighted Jaccard replaces plain Jaccard for n-gram similarity.

### Position-weighted n-grams

`token_ngrams_positional()` encodes the relative position of each n-gram in the function body. `return` at line 1 produces a different hash than `return` at line 50. This captures structural placement, not just token presence.

### Cross-lingual transfer

When a pattern trained on Rust matches TypeScript code (or vice versa), a 25% penalty is applied: `score × 0.75`. Same-language matches pass at 1.0. Unknown languages pass at 1.0.

### Multi-example scoring

With multiple positive/negative examples per pattern, the engine computes `score(pos_i, neg_j)` for every pair and takes the maximum. This means the pattern fires if ANY positive example matches well AND ANY negative example doesn't match better.

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
| `corpus` | Pattern loading, LSH-indexed registry, weighted Jaccard scoring, FRC1 bundle support |
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
- Patcher: automated remediation

---

## MCP Server

Frensense ships a JSON-RPC 2.0 server over stdin/stdout for AI agent integration.

### Tool definition

```json
{
  "name": "frensense_audit",
  "description": "Run semantic analysis on a file or directory.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": { "type": "string", "description": "File or directory to audit" },
      "severity_threshold": { "type": "string", "enum": ["critical", "warning", "info"], "default": "warning" },
      "language": { "type": "string", "description": "Filter by language extension" },
      "rules": { "type": "array", "items": { "type": "string" }, "description": "Only include these rule IDs" },
      "fix_auto": { "type": "boolean", "default": false },
      "stream": { "type": "boolean", "default": false }
    },
    "required": ["path"]
  }
}
```

### Example request

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "frensense_audit",
    "arguments": { "path": "/project/src/main.rs", "severity_threshold": "info" }
  }
}
```

### Response format

```json
{
  "clean": false,
  "advisories": [
    {
      "rule_id": "UNUSED_VARIABLE",
      "severity": "Info",
      "confidence": 0.5,
      "file_path": "src/main.rs",
      "line": 5,
      "observation": "Variable 'x' is defined but never used.",
      "impact": "Unused variables clutter code.",
      "improvement": "Remove the variable or prefix with `_`.",
      "auto_fixable": true,
      "requires_human": true
    }
  ],
  "auto_fixed": 0,
  "requires_human": [...]
}
```

### Streaming mode

Set `"stream": true` to receive findings as JSON-RPC notifications:

```json
{"method": "notification", "params": {"type": "progress", "current": 0, "total": 5}}
{"method": "notification", "params": {"type": "finding", "current": 1, "total": 5, "data": {...}}}
```

---

## CI Integration

```bash
# Basic scan — exit 1 if any finding
frensense . --strict

# Only critical findings
frensense . --severity critical

# Only changed files (diff mode)
frensense . --diff-only

# Baseline workflow — suppress known findings
frensense . --emit-baseline baseline.json      # save current findings
frensense . --compare-baseline baseline.json   # regressions only

# Output formats
frensense . --json                             # JSON for tooling
frensense . --sarif                            # GitHub Advanced Security

# Auto-remediation
frensense . --fix security                     # apply security fixes
frensense . --diff style                       # preview style fixes

# Disable noisy rules
frensense . --disable-rule UNUSED_VARIABLE --strict

# Override severity
frensense . --override-severity FILE_TOO_LONG:info --strict

# Custom taint rules
frensense . --extra-taint-rules ./my-taint-rules/

# Profile-based detection
frensense . --learn-profile                    # build profile
frensense . --check-profile                    # find anomalies
```

---

## What Frensense Does Not Do

- Run code (no dynamic analysis)
- Verify algorithmic correctness
- Detect runtime race conditions not expressible as temporal constraints
- Replace security audits for production financial/medical systems

It catches bugs detectable from source structure and data flow. For everything else, it surfaces findings that warrant review.
