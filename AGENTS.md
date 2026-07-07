# AGENTS.md — Frensense Codebase Guide for AI Agents

> This file helps AI agents understand the frensense codebase quickly.
> Read this before making changes. Update this when architecture changes.

---

## Quick Start

```bash
# Build
cargo build --release

# Run
frensense . --corpus corpus/targets/ --threshold 0.65

# Test
cargo test

# Rebuild corpus bundle
cargo run --bin build-corpus-bundle
```

---

## Architecture Overview

**Detection is 100% corpus-driven.** Every bug pattern is a pair of files: a positive (the bug) and a negative (the fix). There are no hardcoded rules, no regex patterns, no YAML DSL.

```
frensense/
├── frensense-engine/          # Core engine (no CLI, no rules, no policy)
│   ├── src/
│   │   ├── fingerprint.rs     # 7-dimensional function fingerprinting
│   │   ├── corpus/            # Pattern loading, LSH index, scoring
│   │   │   ├── registry.rs    # PatternRegistry — main entry point
│   │   │   ├── loader.rs      # Loads *_positive.* and *_negative.* pairs
│   │   │   ├── bundle.rs      # FRC1 binary bundle format
│   │   │   ├── source_sink.rs # Corpus-learned source types & sink names
│   │   │   └── semantic.rs    # Semantic filters (AST constraints)
│   │   ├── pattern/           # AST pattern compiler, matcher, scorer
│   │   ├── minhash.rs         # MinHash LSH for candidate retrieval
│   │   ├── data_flow.rs       # TaintRegistry, DataFlowEngine
│   │   ├── lang/              # AbstractKind taxonomy + per-language mapper
│   │   ├── temporal.rs        # Finite automaton for lock/await/unlock
│   │   ├── secrets.rs         # Regex + entropy secret scanner
│   │   ├── deps.rs            # Dependency resolver (Cargo.lock, package.json)
│   │   └── semantic_patterns/ # Hardcoded detectors (check_then_act, etc.)
│   └── Cargo.toml
│
├── src/                        # CLI binary + analysis pipeline
│   ├── cli/
│   │   ├── commands.rs         # CLI entry points
│   │   └── options.rs          # Argument parsing
│   ├── engine/
│   │   ├── project/
│   │   │   ├── runner.rs       # Main orchestration (corpus scan + taint verification)
│   │   │   ├── config.rs       # Project configuration
│   │   │   └── cache.rs        # File cache for incremental scans
│   │   ├── auditor/            # Per-file audit logic
│   │   ├── findings/           # Finding modules (unused_var, dead_branch, etc.)
│   │   └── suppression.rs      # Baseline suppression
│   ├── semantics/
│   │   ├── symbols.rs          # SymbolRegistry (cross-file symbol table)
│   │   ├── data_flow/
│   │   │   ├── cross_file.rs   # Cross-file taint verification (uses corpus-learned sources/sinks)
│   │   │   ├── interprocedural.rs # Interprocedural taint (uses corpus-learned sources/sinks)
│   │   │   ├── corpus_seeder.rs # Seeds taint from corpus matches (uses registry)
│   │   │   ├── tracking.rs     # Intra-procedural taint propagation
│   │   │   ├── normalization.rs # SemanticOp extraction from AST
│   │   │   └── lookup.rs       # Taint lookup helpers
│   │   ├── consistency.rs      # Cross-path consistency checking
│   │   └── simple_taint.rs     # Lightweight taint check
│   ├── patcher/                # Auto-remediation (--fix)
│   ├── reporter/               # Output: text, JSON, SARIF
│   └── lib.rs                  # Public API
│
├── corpus/
│   └── targets/                # Positive/negative example pairs
│       ├── *_positive.{rs,ts}  # Buggy code (with [frensense] comment block)
│       ├── *_negative.{rs,ts}  # Fixed code
│       └── *.toml              # Optional metadata (overrides comment block)
│
├── tests/                      # Integration tests
├── scripts/                    # Build, validation, metrics tools
└── docs/                       # Documentation
    ├── TECHNICAL_REFERENCE.md  # Deep source code analysis
    └── LIMITATIONS_MAP.md      # Visual limitations guide
```

---

## How Detection Works

### 1. Corpus Loading (`frensense-engine/src/corpus/loader.rs`)

- Reads all `*_positive.{rs,ts}` and `*_negative.{rs,ts}` files
- Extracts function fingerprints (7 dimensions)
- Parses `/// [frensense]` comment blocks from positive files for advisory text
- **Builds a `CorpusSourceSinkRegistry`** by walking every positive file's AST — learns which parameter types are sources and which function calls are sinks based on frequency across patterns

### 2. Fingerprint Scanning (`frensense-engine/src/corpus/registry.rs`)

- Project functions fingerprinted using the same 7 dimensions
- LSH pre-filters to ~100-200 candidate patterns
- 5-dimensional contrastive scoring:
  ```
  score = sim_to_positive × (1 - sim_to_negative) × cross_lingual_penalty

  sim_to_positive =
      weighted_jaccard(ngrams) × 0.35
    + jaccard(structural) × 0.30
    + jaccard(signature) × 0.20
    + jaccard(param_types) × 0.10
    + type_usage_overlap × 0.05
  ```

### 3. Taint Verification (`src/semantics/data_flow/cross_file.rs`)

For each corpus finding, `verify_taint_flow()` checks if tainted data actually flows from source to sink:

- **Source seeding is corpus-learned**: `CrossFileVerifier::seed_taint()` uses `CorpusSourceSinkRegistry` to taint parameters whose type annotations match types seen in positive examples (e.g., `Request`, `Json<T>`, `Query<T>`)
- **Sink identification is corpus-learned**: `check_call_for_sink()` uses the registry to identify dangerous function calls
- Taint propagation follows assignments, member expressions, and function calls through the AST
- If verified → confidence boosted +20% (capped at 0.95), tagged `taint-verified`

### 4. Composition Layer (`src/engine/project/runner.rs`)

- Corpus match alone → down-weight ×0.6
- Corpus + taint verification → full confidence
- High branch ratio (hollow validator) → suppress ×0.3
- Per-category calibration (sec, csa, llm, arch, async)

### 5. Output

- Advisories with observation/impact/improvement from the positive file's `[frensense]` comment block
- Confidence-scored, severity-classified findings

---

## Corpus-Driven Source/Sink Registry

**File:** `frensense-engine/src/corpus/source_sink.rs`

Instead of hardcoded lists, the engine learns what's a "source" and what's a "sink" from the corpus:

- **Source types**: extracted from function parameter type annotations in positive files. If `Request` appears as a parameter type in multiple positives, it's a source.
- **Sink names**: extracted from call expressions in positive files. If `exec` appears as a callee in multiple positives, it's a sink.
- Only types/sinks appearing in ≥2 distinct patterns are promoted (prunes noise).

This replaces the old hardcoded `framework_types` arrays and `identify_sink()` functions.

---

## How to Add New Detection

### Corpus Pattern (Preferred)

```bash
# 1. Create positive example (the bug) with [frensense] block
corpus/targets/ts_my_pattern_positive.ts

# 2. Create negative example (the fix)
corpus/targets/ts_my_pattern_negative.ts

# 3. Rebuild bundle
cargo run --bin build-corpus-bundle
```

**Advisory text format** — put a `[frensense]` block at the top of the positive file:

```rust
// [frensense]
// observation: What the bug looks like to a reader.
// impact: What goes wrong when this code runs.
// improvement: How to fix it.
fn my_buggy_function() { ... }
```

Works with `///` (Rust), `//` (TS/JS), `#` (Python). The TOML sidecar is optional — if a `.toml` exists it overrides the comment block, but you almost never need it.

---

## Corpus Pattern Naming Convention

```
{lang}_{category}_{name}_{positive,negative}.{ext}

Examples:
  rust_csa_validate_unconditional_positive.rs
  rust_csa_validate_unconditional_negative.rs
  ts_check_then_act_toctou_positive.ts
  ts_check_then_act_toctou_negative.ts
  ts_sec_sql_injection_1_positive.ts
```

Categories:
- `csa` — Contract Surface Analysis (validate, sanitize, auth)
- `sec` — Security (SQL injection, XSS, SSRF, etc.)
- `llm` — LLM anti-patterns
- `arch` — Architecture (resource leaks, etc.)
- `async` — Concurrency (mutex across await, etc.)

---

## Critical Files

| File | Purpose |
|------|---------|
| `src/engine/project/runner.rs` | Main orchestration — corpus scan + taint verification |
| `frensense-engine/src/fingerprint.rs` | 7-dimensional function fingerprinting |
| `frensense-engine/src/corpus/registry.rs` | PatternRegistry — scoring, LSH, IDF weights |
| `frensense-engine/src/corpus/loader.rs` | Loads corpus pairs + builds source/sink registry |
| `frensense-engine/src/corpus/source_sink.rs` | Corpus-learned source types & sink names |
| `src/semantics/data_flow/cross_file.rs` | Taint verification using learned sources/sinks |

---

## Known Limitations

| Limitation | Impact | Status |
|------------|--------|--------|
| Severity mismatch (TOML "Critical" → output "Warning") | Misleading severity in output | Open — TOML severity not propagated |
| Secret scanner gaps (no Azure, GitLab, npm, PyPI) | Misses some token types | Open |
| Patcher TypeScript-only | Import injection won't work for Rust/Python | Open |
| Semantic filters are manual AST constraints | Requires hand-written TOML per pattern | Open — could be deleted if scorer is strong enough |
| Scorer false positives on trivial code | `fn main()` matches unrelated corpus patterns | Open — needs tighter scoring |

---

## Feature Flags

**File:** `Cargo.toml` (root)

```toml
[features]
default = ["rust", "typescript", "fingerprinting", "temporal"]
rust = ["dep:tree-sitter-rust"]
typescript = ["dep:tree-sitter-typescript", "dep:tree-sitter-javascript"]
c_lang = ["frensense-engine/c_lang"]
python = ["frensense-engine/python", "dep:tree-sitter-python"]
fingerprinting = ["frensense-engine/serialize"]  # Enables FRC bundle
temporal = []
```

## Finding Modules Execution Order

**File:** `src/engine/findings/mod.rs`

```
1. TemporalViolation — NO-OP (temporal detection is corpus-driven)
2. HallucinatedImport — uses DependencyResolver
3. CrossFileTaint    — NO-OP (handled by corpus layer)
4. SemanticPatterns  — NO-OP (TOCTOU detection is corpus-driven)
```

Removed commodity detectors (Clippy/GitLeaks do them better):
- ~~DeadBranch~~ — Clippy catches `if false`
- ~~UnusedVariable~~ — Clippy, rust-analyzer, tsc
- ~~AtomicSection~~ — C-specific lock/unlock, too niche
- ~~Secret scanning~~ — GitLeaks, TruffleHog have 100+ patterns

---

## Testing

```bash
# All tests
cargo test

# Engine tests only
cargo test -p frensense-engine

# Source/sink registry tests
cargo test -p frensense-engine -- source_sink

# Data flow tests (cross-file, interprocedural, corpus seeder)
cargo test -p frensense --lib -- data_flow

# Specific pattern test
cargo test -p frensense-engine -- toctou
```

---

## Configuration

### CLI Flags

```bash
--corpus <dir>              # Load corpus patterns
--threshold <0-1>           # Corpus match threshold (default: 0.40)
--severity <level>          # Minimum severity: critical, warning, info
--diff-only                 # Only scan changed files
--json                      # JSON output
--sarif                     # SARIF output for GitHub
--fix [scope]               # Auto-remediation: style, security, all
--diff [scope]              # Show diff without applying
--strict                    # Exit code 1 if any findings
--confidence <tier>         # high (≥0.85), medium (≥0.60), low (≥0.30), any
--min-confidence <0-1>      # Raw confidence floor
--check-deps                # Enable dependency checking (Rust)
--learn-profile             # Build project style profile
--check-profile             # Check code against learned profile
--emit-baseline <file>      # Save current findings as baseline
--compare-baseline <file>   # Compare against baseline
--disable-rule <id>         # Suppress specific rule
--override-severity <id>:<level>  # Change rule severity
```

---

## Build Artifacts

- `frensense-corpus.frc` — Pre-compiled corpus bundle (embedded in binary)
- `target/release/frensense` — Release binary

---

## References

- `docs/TECHNICAL_REFERENCE.md` — Deep source code analysis
- `docs/LIMITATIONS_MAP.md` — Visual limitations guide
- `ROADMAP.md` — What to build next
- `tasks.md` — Current task tracker
- `research/csa-corpus-rework.md` — CSA as corpus category design
- `frensense-thesis-updated.md` — Why corpus, why not rules
