# Frensense Technical Reference
## How FrenSense Works — Source Code Deep Dive

> Generated from direct code analysis. Every claim references specific files and line numbers.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [The Detection Pipeline](#the-detection-pipeline)
3. [Fingerprinting System](#fingerprinting-system)
4. [Corpus Pattern Matching](#corpus-pattern-matching)
5. [Taint Analysis](#taint-analysis)
6. [Semantic Patterns (Hardcoded)](#semantic-patterns-hardcoded)
7. [Finding Modules](#finding-modules)
8. [Known Limitations](#known-limitations)
9. [How to Add New Detection](#how-to-add-new-detection)

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              FRENSENSE CLI                                  │
│                            src/cli/commands.rs                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PROJECT RUNNER                                    │
│                    src/engine/project/runner.rs                             │
│                                                                             │
│  Orchestrates: symbol discovery → event chains → rule execution             │
│                → corpus scan → taint analysis → confidence filtering        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
            ┌───────────────────────┼───────────────────────┐
            ▼                       ▼                       ▼
┌───────────────────┐   ┌───────────────────┐   ┌───────────────────┐
│  ENGINE LAYER     │   │  SEMANTICS LAYER  │   │  FINDINGS LAYER   │
│ frensense-engine/ │   │    src/semantics/ │   │  src/engine/      │
│                   │   │                   │   │  findings/        │
│ - fingerprint     │   │ - symbols         │   │ - unused_var      │
│ - corpus          │   │ - data_flow       │   │ - dead_branch     │
│ - pattern         │   │ - consistency     │   │ - temporal        │
│ - minhash         │   │ - simple_taint    │   │ - hallucinated    │
│ - data_flow       │   │ - cross_file      │   │ - secret_scanner  │
│ - temporal        │   │                   │   │ - style_anomaly   │
│ - secrets         │   │                   │   │ - near_duplicate  │
│ - deps            │   │                   │   │                   │
└───────────────────┘   └───────────────────┘   └───────────────────┘
```

---

## The Detection Pipeline

**File:** `src/engine/project/runner.rs`

The runner executes these phases in order:

```
Phase 1: File Discovery & Parsing
    └─ Walk project tree, parse with tree-sitter
    └─ Build FileSnapshot { id, path, content, tree, symbols, edges, semantic_ops }

Phase 2: Symbol Registry Construction
    └─ Collect all function definitions across files
    └─ Build call graph edges (caller → callee)
    └─ Discover events (lock/unlock, await, etc.)

Phase 3: Parallel Audit (per-file)
    └─ For each file: run all finding modules
    └─ Modules: UNUSED_VARIABLE, DEAD_BRANCH, TEMPORAL_VIOLATION,
                HALLUCINATED_IMPORT, SECRET_*, STYLE_ANOMALY, NEAR_DUPLICATE

Phase 4: Corpus Pattern Matching
    └─ For each function: extract fingerprint
    └─ Compare against corpus patterns via MinHash LSH
    └─ Score with 5-dimensional contrastive scoring

Phase 5: Taint Analysis
    └─ Seed taint on source variables (regex-based)
    └─ Propagate through data flow graph
    └─ Check if tainted data reaches sinks
    └─ Apply confidence adjustments (L3 entropy, sanitizers)

Phase 6: Cross-File Taint (if enabled)
    └─ Trace taint across function boundaries
    └─ Follow call graph through SymbolRegistry

Phase 7: Severity Overrides & Composition
    └─ Apply user-defined severity overrides
    └─ Adjust confidence based on multiple confirming signals

Phase 8: Output
    └─ Generate Advisory structs with rule_id, severity, observation, impact, improvement
```

---

## Fingerprinting System

**File:** `frensense-engine/src/fingerprint.rs`

Every function is fingerprinted into 7 dimensions:

```rust
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub language: String,

    // DIMENSION 1: Token N-grams (position-encoded)
    pub ngram_hashes: FxHashSet<u64>,
    // "return" at line 5 ≠ "return" at line 50

    // DIMENSION 2: IDF-weighted N-grams
    pub weighted_ngram_hashes: FxHashMap<u64, f32>,
    // Rare tokens (db::execute) score higher than common ones (let x)

    // DIMENSION 3: Signature N-grams
    pub signature_ngrams: FxHashSet<u64>,
    // Parameter names + types

    // DIMENSION 4: Parameter Type N-grams
    pub param_type_ngrams: FxHashSet<u64>,
    // Just the types, not names

    // DIMENSION 5: Name Segments
    pub name_segments: Vec<String>,
    // "validateUser" → ["validate", "User"]

    // DIMENSION 6: Structural Markers
    pub structural_markers: FxHashSet<u64>,
    // Abstract AST node kinds (language-normalized)

    // DIMENSION 7: Type Usages
    pub type_usages: Vec<String>,
    // All type identifiers used in the body
}
```

### Position-Weighted N-Grams (M9)

**File:** `frensense-engine/src/fingerprint.rs:65-89`

```rust
fn token_ngrams_positional(tokens: &[String], window_size: usize) -> FxHashSet<u64> {
    // For each window of `window_size` tokens:
    //   1. Hash the token sequence
    //   2. Encode relative position (0.0 = start, 1.0 = end)
    //   3. Combine into final hash
    //
    // Result: "return" at line 5 produces different hash than "return" at line 50
}
```

### IDF Weighting (M1)

**File:** `frensense-engine/src/fingerprint.rs:34-49`

```rust
pub fn compute_idf_weights(fingerprints: &[FunctionFingerprint]) -> FxHashMap<u64, f32> {
    // TF-IDF style: tokens that appear in fewer patterns get higher weight
    // "db::execute" appears in 2 patterns → high weight
    // "let x" appears in 80 patterns → low weight
}
```

---

## Corpus Pattern Matching

**File:** `frensense-engine/src/corpus/registry.rs`

### How Patterns Are Stored

```
corpus/targets/
├── rust_csa_validate_unconditional_positive.rs    ← buggy code
├── rust_csa_validate_unconditional_negative.rs    ← fixed code
├── rust_csa_validate_unconditional.toml           ← metadata
├── ts_check_then_act_toctou_positive.ts
├── ts_check_then_act_toctou_negative.ts
├── ts_check_then_act_toctou.toml
└── ... (89 patterns total)
```

### Pattern Loading

```rust
// registry.rs:42-48
pub fn load_corpus(&mut self, corpus_dir: &Path) -> Result<usize, String> {
    let patterns = load_corpus(corpus_dir)?;  // Parse all *_positive.* and *_negative.*
    let count = patterns.len();
    self.patterns = patterns;
    self.compute_and_apply_idf();  // M1: IDF weights
    self.build_lsh_index();        // MinHash LSH for fast retrieval
    Ok(count)
}
```

### MinHash LSH Candidate Retrieval

**File:** `frensense-engine/src/minhash.rs`

```
┌─────────────────────────────────────────────────────────────────┐
│                    MinHash LSH Index                            │
│                    16 bands × 8 rows = 128 hash slots          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Scanned Function                                               │
│       │                                                         │
│       ▼                                                         │
│  Compute 128-hash MinHash signature                             │
│       │                                                         │
│       ▼                                                         │
│  Hash into 16 band buckets                                      │
│       │                                                         │
│       ▼                                                         │
│  Retrieve candidate patterns that share ≥1 band                 │
│       │                                                         │
│       ▼                                                         │
│  Score only candidates (not all 89+ patterns)                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5-Dimensional Contrastive Scoring

**File:** `frensense-engine/src/corpus/registry.rs:scan_function()`

```
                    Scanned Function Fingerprint
                              │
                              ▼
            ┌─────────────────────────────────────┐
            │         Compare to POSITIVE         │
            │                                     │
            │  sim_to_positive =                  │
            │    weighted_jaccard(ngrams) × 0.35  │
            │  + jaccard(structural) × 0.30       │
            │  + jaccard(signature) × 0.20        │
            │  + jaccard(param_types) × 0.10      │
            │  + type_usage_overlap × 0.05        │
            └─────────────────────────────────────┘
                              │
                              ▼
            ┌─────────────────────────────────────┐
            │         Compare to NEGATIVE         │
            │                                     │
            │  sim_to_negative = same formula     │
            └─────────────────────────────────────┘
                              │
                              ▼
            ┌─────────────────────────────────────┐
            │         Contrastive Score           │
            │                                     │
            │  final = sim_pos × (1 - sim_neg)   │
            │          × cross_lingual_penalty    │
            │                                     │
            │  Cross-lingual: 0.75 if languages   │
            │  differ, 1.0 if same                │
            └─────────────────────────────────────┘
                              │
                              ▼
            ┌─────────────────────────────────────┐
            │         Threshold Check             │
            │                                     │
            │  if final >= threshold (default 0.32)│
            │    → emit CORPUS_{pattern_id}       │
            └─────────────────────────────────────┘
```

### What the Score Means

```rust
// registry.rs:scan_function()
let score = sim_to_positive * (1.0 - sim_to_negative) * cross_lingual_penalty;

// Example:
// sim_to_positive = 0.85 (function looks like the buggy pattern)
// sim_to_negative = 0.10 (function does NOT look like the fixed pattern)
// final = 0.85 × (1 - 0.10) × 1.0 = 0.765
//
// If threshold = 0.32 → MATCH (0.765 > 0.32)
```

---

## Taint Analysis

**File:** `src/semantics/data_flow/resolve.rs`

### How Taint Is Seeded (Current: Regex-Based)

```rust
// resolve.rs — COMBINED_SOURCE_RE
// Built from all taint rules' `source` patterns
// Example regex: "input|user_input|req\\.body|param|query|url|data|..."

fn is_taint_source(name: &str) -> bool {
    COMBINED_SOURCE_RE.is_match(name)
}
```

**LIMITATION:** This matches by variable NAME, not by actual data origin.

```rust
// This gets tainted (name matches "url"):
let url = format!("https://internal-api/{}", id);  // FALSE POSITIVE

// This also gets tainted (name matches "input"):
let input = read_from_database();  // FALSE POSITIVE
```

### How Taint Propagates

```
Source Variable (name matches regex)
       │
       ▼
┌─────────────────────────────────────┐
│  Assignment: x = source             │
│  → taint propagates to x            │
├─────────────────────────────────────┤
│  Member access: req.body.name       │
│  → taint propagates to name         │
├─────────────────────────────────────┤
│  Function call: sanitize(x)         │
│  → taint propagates to return value │
│  (unless sanitizer recognized)      │
├─────────────────────────────────────┤
│  Array/Object: { key: tainted }     │
│  → taint propagates to value        │
└─────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────┐
│  SINK DETECTION                     │
│                                     │
│  If tainted data reaches:           │
│  - exec/system/shell → TAINT_INPUT_TO_EXEC (Critical)
│  - db.insert/update  → TAINT_CREDENTIAL_TO_DB (Critical)
│  - log/console/print → TAINT_CREDENTIAL_TO_LOG (Warning)
│  - fetch/http/request → TAINT_INPUT_TO_HTTP (Warning)
│  - write/open/remove → TAINT_INPUT_TO_FS (Warning)
└─────────────────────────────────────┘
```

### Taint Rules (TOML)

**File:** `taint_rules.toml`

```toml
[[rules]]
id = "TAINT_CREDENTIAL_TO_DB"
source = "password|secret|token|credential"
sink = "insert|update|db.execute|query"
severity = "critical"
observation = "Credentials flowing to database operations."
impact = "Database logs may expose credentials."
improvement = "Hash passwords before storage. Use environment variables for secrets."
sanitizers = ["bcrypt.hash", "hash_password"]  # Not currently used!
```

**LIMITATION:** The `sanitizers` field exists but is NOT read during propagation.

---

## Semantic Patterns (Hardcoded)

**File:** `frensense-engine/src/semantic_patterns/`

These are Rust code detectors, NOT corpus-based:

```
semantic_patterns/
├── check_then_act.rs      ← CHECK_THEN_ACT_TOCTOU (Prisma-only!)
├── helpers.rs              ← Shared utilities
├── mod.rs                  ← Pattern registry
└── registry.rs             ← Pattern trait definition
```

### CHECK_THEN_ACT_TOCTOU Deep Dive

**File:** `frensense-engine/src/semantic_patterns/check_then_act.rs`

```
Detection Algorithm:
───────────────────

1. SCAN function bodies for database READS
   └─ Pattern: calls to findUnique, findFirst, findMany, etc.
   └─ Extract: variable name, entity name

2. SCAN subsequent statements for CONDITIONAL CHECKS
   └─ Pattern: if_statement, conditional_expression
   └─ Check: does condition reference the read variable?

3. SCAN remaining statements for DATABASE WRITES
   └─ Pattern: calls to create, update, delete, etc.
   └─ Check: is write inside $transaction? → skip
   └─ Check: does write target same entity as read?

4. IF all three found → emit CHECK_THEN_ACT_TOCTOU finding
```

**HARDCODED PATTERNS (helpers.rs):**

```rust
// Read methods (Prisma-specific)
let read_methods = [
    "findUnique", "findFirst", "findMany", "findUniqueOrThrow",
    "findFirstOrThrow", "findFirst_", "findMany_",
    "aggregate", "count", "groupBy",
];

// Write methods (Prisma-specific)
let write_methods = [
    "create", "createMany", "update", "updateMany",
    "upsert", "delete", "deleteMany",
    "executeRaw", "executeRawUnsafe",
];

// Transaction detection (Prisma-specific)
text.contains("$transaction") || text.contains("transaction")
```

**LIMITATION:** Only works with Prisma ORM. TypeORM, Sequelize, Knex, raw SQL → NOT DETECTED.

---

## Finding Modules Execution Order

**File:** `src/engine/findings/mod.rs:84-93`

```
1. DeadBranch       — uses ReachabilityChecker (semantics/reachability.rs)
2. UnusedVariable   — uses CFG def-use chains (frensense-engine/src/cfg/def_use.rs)
3. TemporalViolation — uses TemporalAnalyzer (temporal rules from TOML)
4. HallucinatedImport — uses DependencyResolver (frensense-engine/src/deps.rs)
5. CrossFileTaint    — NO-OP! Returns empty vec (handled by corpus layer)
6. AtomicSection     — uses AtomicSectionAnalyzer (C lock/unlock pairing)
7. SemanticPatterns  — uses PatternRunner (CHECK_THEN_ACT_TOCTOU only)
```

## Finding Modules

**File:** `src/engine/findings/`

These are independent detection passes that run per-file:

```
findings/
├── mod.rs              ← Module registry
├── unused_variable.rs  ← UNUSED_VARIABLE (from CFG def-use chains)
├── dead_branch.rs      ← DEAD_BRANCH (if false/if true)
├── temporal.rs         ← TEMPORAL_VIOLATION (lock/unlock mismatches)
├── hallucinated_import.rs ← HALLUCINATED_IMPORT (missing from lockfile)
├── secret_scanner.rs   ← SECRET_* (regex + entropy)
├── style_anomaly.rs    ← STYLE_ANOMALY (structural dissimilarity)
└── near_duplicate.rs   ← NEAR_DUPLICATE_FUNCTION (85%+ similarity)
```

### Module Interface

```rust
// findings/mod.rs
pub trait FindingModule {
    fn run(&self, snapshot: &FileSnapshot, ctx: &mut FindingContext) -> Vec<Advisory>;
}
```

Each module receives the full file snapshot and can access:
- `snapshot.tree` — AST root
- `snapshot.content` — source text
- `snapshot.symbols` — all symbols in the file
- `ctx.symbols` — cross-file symbol registry
- `ctx.dep_resolver` — dependency resolution
- `ctx.data_flow_engine` — taint analysis

---

## Known Limitations

### 1. Taint Source Seeding Is Regex-Based (CRITICAL)

**File:** `src/semantics/data_flow/resolve.rs`

**Problem:** Variables are tainted by NAME, not by actual data origin.

**Impact:** 100% false positive rate on some codebases (e.g., axum).

**Example:**
```rust
// This gets tainted because name matches "url":
let url = format!("https://api.internal/{}", user_id);
// → TAINT_INPUT_TO_HTTP fires (FALSE POSITIVE)
```

**Fix:** AST-based entry point detection (T-FIX-1 in ROADMAP.md).

---

### 2. TOCTOU Detector Is Prisma-Only

**File:** `frensense-engine/src/semantic_patterns/check_then_act.rs`

**Problem:** Hardcoded to recognize only Prisma ORM patterns.

**Impact:** TOCTOU bugs in TypeORM, Sequelize, Knex, raw SQL → NOT DETECTED.

**Example:**
```typescript
// TypeORM — NOT DETECTED:
const user = await userRepository.findOne({ where: { id } });
if (user.balance >= amount) {
    await userRepository.update({ id }, { balance: user.balance - amount });
}

// Prisma — DETECTED:
const user = await prisma.user.findUnique({ where: { id } });
if (user.balance >= amount) {
    await prisma.user.update({ where: { id }, data: { balance: user.balance - amount } });
}
```

**Fix:** Add corpus patterns for each ORM, OR make detector generic.

---

### 3. Sanitizer Propagation Not Implemented

**File:** `src/semantics/data_flow/resolve.rs`

**Problem:** `taint_rules.toml` has `sanitizers` field but it's not used.

**Impact:** Tainted data flows through sanitizers unchanged.

**Example:**
```typescript
// This still triggers TAINT_INPUT_TO_HTTP:
const clean = encodeURIComponent(userInput);  // sanitizer
fetch(`https://api.com?q=${clean}`);  // FALSE POSITIVE
```

**Fix:** T-FIX-3 in ROADMAP.md.

---

### 4. Corpus Advisory Text (PARTIALLY FIXED)

**File:** `src/engine/project/runner.rs:158-162`

**Current state:** Runner DOES use sidecar TOML text when available:
```rust
let impact = m.impact.unwrap_or_else(|| "Function shape matches a known violation pattern.".to_string());
let improvement = m.improvement.unwrap_or_else(|| "Review against corpus example.".to_string());
let observation = m.observation.unwrap_or_else(|| {
    format!("Corpus pattern: {} (score {:.2}) in '{}'", m.pattern_id, m.score, fp.function_name)
});
```

**Remaining issue:** Many patterns still lack sidecar TOMLs, so they get generic fallback text.

---

### 5. Cross-File Taint Is Limited

**File:** `src/semantics/data_flow/cross_file.rs`

**Problem:** Follows call graph but limited depth and no heap modeling.

**Impact:** Taint that flows through complex object manipulation may be lost.

---

### 6. No Dynamic Analysis

**What it means:** FrenSense only looks at source code structure, not runtime behavior.

**Impact:** Cannot detect:
- Actual race conditions (only temporal constraints)
- Runtime type errors
- Memory safety issues (in managed languages)
- Actual data flow at runtime

---

## How to Add New Detection

### Option A: Corpus Pattern (Example-Based)

This is the preferred approach — example-based, no code changes needed.

**Step 1:** Create positive example (the bug)
```bash
# Example: TypeORM TOCTOU
corpus/targets/ts_toctou_typeorm_positive.ts
```

```typescript
// TOCTOU: Check outside transaction, write inside
async function deductBalance(userId: string, amount: number) {
    const user = await userRepository.findOne({ where: { id: userId } });
    if (user.balance < amount) {
        throw new Error('INSUFFICIENT_BALANCE');
    }
    // BUG: Balance check is outside transaction
    await userRepository.update(
        { id: userId },
        { balance: user.balance - amount }
    );
}
```

**Step 2:** Create negative example (the fix)
```bash
corpus/targets/ts_toctou_typeorm_negative.ts
```

```typescript
// FIXED: Check inside transaction
async function deductBalance(userId: string, amount: number) {
    return await dataSource.transaction(async (manager) => {
        const user = await manager.findOne(User, { where: { id: userId } });
        if (user.balance < amount) {
            throw new Error('INSUFFICIENT_BALANCE');
        }
        return await manager.update(
            User,
            { id: userId },
            { balance: user.balance - amount }
        );
    });
}
```

**Step 3:** Create sidecar TOML
```bash
corpus/targets/ts_toctou_typeorm.toml
```

```toml
id = "TS_TOCTOU_TYPEORM"
severity = "Critical"
observation = "Database read and conditional check happen outside a transaction, while the write happens inside or without atomicity."
impact = "Concurrent requests can bypass the check and cause double-spending, overselling, or data corruption."
improvement = "Move the check inside the transaction. Use SELECT ... FOR UPDATE or optimistic locking."
```

**Step 4:** Rebuild bundle
```bash
cargo run --bin build-corpus-bundle -- --corpus corpus/targets --output frensense-corpus.frc
```

**Step 5:** Test
```bash
cargo test -p frensense-engine -- toctou_typeorm
```

---

### Option B: Hardcoded Detector (Rust Code)

For patterns that can't be expressed as example pairs.

**Step 1:** Create detector file
```bash
frensense-engine/src/semantic_patterns/my_detector.rs
```

**Step 2:** Implement SemanticPattern trait
```rust
use super::{PatternFinding, SemanticPattern};

pub struct MyDetector;

impl SemanticPattern for MyDetector {
    fn id(&self) -> &str { "MY_DETECTOR" }
    fn description(&self) -> &str { "Detects X pattern" }
    fn severity(&self) -> &str { "Warning" }

    fn scan(&self, tree: tree_sitter::Node, source: &str, file_path: &str) -> Vec<PatternFinding> {
        // Your detection logic here
        vec![]
    }
}
```

**Step 3:** Register in mod.rs
```rust
// semantic_patterns/mod.rs
pub mod my_detector;

pub fn registered_patterns() -> Vec<Box<dyn SemanticPattern>> {
    vec![
        Box::new(check_then_act::CheckThenAct),
        Box::new(my_detector::MyDetector),
    ]
}
```

---

## Semantic Filters (Reduce False Positives)

**File:** `corpus/semantic_filters.toml` + `frensense-engine/src/corpus/semantic.rs`

Before fingerprint scoring, patterns pass through AST-level constraints:

```toml
# Promise chain without .catch()
[ts_llm_promise_catch]
contains_call_to = [".then"]
must_not_contain_call_to = [".catch", ".finally"]

# Sanitizer function that just returns input unchanged
[ts_csa_sanitize_passthrough]
function_name_regex = "^sanitize"
must_not_contain_call_to = [".replace", ".encode", "encodeURIComponent"]

# Clone in loop (Rust)
[rust_clone_in_loop]
contains_call_to = [".clone"]
contains_node_type = ["loop_expression", "for_expression", "while_expression"]
```

### Filter Types

| Filter | Purpose | Example |
|--------|---------|---------|
| `contains_call_to` | Must contain these function calls | `[".then"]` for promise patterns |
| `must_not_contain_call_to` | Must NOT contain these calls | `[".catch"]` for uncaught promises |
| `function_name_regex` | Function name must match pattern | `"^sanitize"` for sanitizer patterns |
| `contains_node_type` | Must contain these AST node types | `["await_expression"]` for async |
| `must_not_contain_node_type` | Must NOT contain these node types | `["async_block"]` for sync-only |

---

## Taint Verification for Corpus Findings

**File:** `src/engine/project/runner.rs:175-215`

Corpus findings can be verified against taint flow:

```
Corpus Match (score > threshold)
       │
       ▼
┌─────────────────────────────────────┐
│  Taint Verification                 │
│                                     │
│  1. Seed taint on function params   │
│  2. Propagate through function body │
│  3. Check if tainted data reaches   │
│     a dangerous sink                │
└─────────────────────────────────────┘
       │
       ├── Verified ──→ Confidence +20% (max 0.95)
       │                Tag: "taint-verified"
       │                Impact includes taint detail
       │
       └── Not verified → Use raw corpus score
```

---

## Canonical Form Pattern Matching (Wired In)

**Files:** `frensense-engine/src/pattern/`

This is a **fully wired** alternative to n-gram fingerprinting:

```
pattern/
├── canonical.rs    ← Converts AST to canonical form (normalized identifiers)
├── compiler.rs     ← Compiles AST nodes to PatternNode trees with wildcards
├── matcher.rs      ← Matches PatternNode against tree-sitter Nodes
└── scorer.rs       ← 5-dimensional scoring with AST edit distance
```

### Scoring Weights (scorer.rs)

```
final_score =
    ast_distance × 0.40    ← Tree edit distance (structural similarity)
  + ngram_score × 0.20     ← Token n-gram similarity
  + semantic × 0.20        ← Semantic markers (e.g., "contains exec call")
  + signature × 0.10       ← Function signature similarity
  + param_types × 0.05     ← Parameter type similarity
  + type_usage × 0.05      ← Type usage similarity
```

### Wildcard Matching

The compiler supports three pattern types:
- `Exact` — must match exactly
- `Structural` — match AST node kinds, ignore text
- `Semantic` — match with additional constraints

Wildcards capture named spans: `$EXPR` matches any expression.

---

## MCP Server (JSON-RPC 2.0)

**Files:** `src/mcp/`

### Supported Methods

| Method | Description |
|--------|-------------|
| `initialize` | Protocol handshake |
| `tools/list` | List available tools |
| `tools/call` | Execute `frensense_audit` tool |
| `ping` | Returns "pong" |
| `shutdown` | Graceful shutdown |

### Tool Parameters

```json
{
  "path": "string (required)",
  "fix_auto": "boolean (default: false)",
  "severity_threshold": "critical|warning|info (default: warning)",
  "language": "string (filter by extension)",
  "rules": ["array of rule IDs"],
  "stream": "boolean (default: false)"
}
```

### Streaming Mode

When `stream=true`, findings are sent as JSON-RPC notifications:
```json
{"method": "notification", "params": {"type": "progress", "current": 0, "total": 5}}
{"method": "notification", "params": {"type": "finding", "current": 1, "total": 5, "data": {...}}}
```

---

## Reporter Output Formats

**File:** `src/reporter.rs`

| Format | Method | Details |
|--------|--------|---------|
| Markdown | `to_markdown()` | Grouped by severity, professional report |
| SARIF 2.1.0 | `to_sarif()` | Full SARIF with rules, results, fixes |

**Note:** JSON output is only available through MCP server, not CLI.

---

## Secret Scanner Patterns

**File:** `frensense-engine/src/secrets.rs`

| Pattern | Regex | Confidence |
|---------|-------|------------|
| AWS Access Key | `AKIA[0-9A-Z]{16}` | 0.90 |
| AWS Secret Key | `aws...'[0-9a-zA-Z/+]{40}'` | 0.95 |
| GitHub Token | `ghp_[0-9a-zA-Z]{36}` | 0.95 |
| Generic API Key | `api_key/secret/token...'[0-9a-zA-Z_-]{16,64}'` | 0.70 |
| JWT Token | `eyJ[...]` | 0.85 |
| Private Key | `BEGIN RSA/DSA/EC/OPENSSH/PGP PRIVATE KEY` | 0.95 |
| Connection String | `mongodb/postgresql/mysql/redis://...` | 0.85 |
| Slack Token | `xox[baprs]-...` | 0.90 |
| Google API Key | `AIza...` | 0.85 |

---

## Confidence Composition (Layer Signal AND-Gate)

**File:** `src/engine/composition.rs`

FrenSense uses a 4-layer AND-gate to compose confidence:

```
Layer 1 (L1): Corpus Pattern Match — structural shape similarity
Layer 2 (L2): Taint Flow — data actually flows from source to sink
Layer 3 (L3): Taint Branch Ratio — function branches on tainted input
Layer 4 (L4): Near-Duplicate — inconsistent behavior across similar functions
```

### Composition Rules

```rust
// composition.rs:30-55
fn compose_confidence(signals: &LayerSignals, base_score: f32) -> f32 {
    let mut score = base_score;

    // L2 confirms L1: tainted data reaches sink → full confidence
    if signals.corpus_match && signals.taint_flow {
        // No penalty
    } else if signals.corpus_match && !signals.taint_flow {
        // Structural match with no dataflow → down-weight
        score *= 0.6;
    }

    // L3 can SUPPRESS L1: high branch ratio means real validator
    if let Some(ratio) = signals.taint_branch_ratio {
        if ratio > 0.6 {
            score *= 0.3;  // Suppress: function actually validates
        }
    }

    // L4 inconsistency boosts confidence
    if signals.near_duplicate {
        score *= 1.2;
    }

    score.min(1.0)
}
```

### Visual

```
                    Corpus Match (L1)
                         │
            ┌────────────┼────────────┐
            │            │            │
            ▼            ▼            ▼
     + Taint Flow   No Taint     High Branch
       (L2)         Flow         Ratio (L3)
            │            │            │
            ▼            ▼            ▼
      Full Score    ×0.6 Score   ×0.3 Score
      (no penalty)  (downweight) (suppress)
            │            │            │
            └────────────┼────────────┘
                         │
                    + Near-Duplicate (L4)
                         │
                         ▼
                    ×1.2 Score (boost)
```

---

## Confidence Calibration (Platt Scaling)

**File:** `src/engine/confidence_calibration.rs`

Raw similarity scores are calibrated to probabilities using logistic regression:

```rust
// calibration.rs:35-38
pub fn calibrate(&self, raw_score: f64) -> f64 {
    let z = self.a * raw_score + self.b;
    1.0 / (1.0 + (-z).exp())  // sigmoid
}
```

Training uses gradient descent on labeled TP/FP data:
- 500 iterations, learning rate 0.1
- Cross-entropy loss minimization
- Reports accuracy on training set

---

## Summary: What FrenSense Can vs Cannot Catch

| Bug Type | Detected? | How |
|----------|-----------|-----|
| Hollow validators (`validate_*` → always true) | ✅ | Corpus: `csa_validate_unconditional` |
| Sanitize passthrough | ✅ | Corpus: `csa_sanitize_passthrough` |
| TOCTOU (Prisma) | ✅ | Hardcoded: `CHECK_THEN_ACT_TOCTOU` |
| TOCTOU (TypeORM, Sequelize, etc.) | ❌ | Need corpus patterns |
| Taint flow to exec/DB/HTTP | ⚠️ | Regex seeding causes high FP |
| Mutex held across await | ✅ | Hardcoded: `TEMPORAL_VIOLATION` |
| Dead code (if false) | ✅ | Finding: `DEAD_BRANCH` |
| Unused variables | ✅ | Finding: `UNUSED_VARIABLE` |
| Hallucinated imports | ✅ | Finding: `HALLUCINATED_IMPORT` |
| Hardcoded secrets | ✅ | Finding: `SECRET_*` |
| Near-duplicate functions | ✅ | Finding: `NEAR_DUPLICATE_FUNCTION` |
| Actual race conditions | ❌ | No dynamic analysis |
| Runtime type errors | ❌ | No dynamic analysis |
| Cross-file taint (complex) | ⚠️ | Limited depth |
| Novel bug patterns | ❌ | Only known patterns |
