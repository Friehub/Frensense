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

```
frensense/
├── frensense-engine/          # Core engine (no CLI, no rules, no policy)
│   ├── src/
│   │   ├── fingerprint.rs     # 7-dimensional function fingerprinting
│   │   ├── corpus/            # Pattern loading, LSH index, scoring
│   │   │   ├── registry.rs    # PatternRegistry — main entry point
│   │   │   ├── loader.rs      # Loads *_positive.* and *_negative.* pairs
│   │   │   ├── bundle.rs      # FRC1 binary bundle format
│   │   │   └── semantic.rs    # Semantic filters for corpus patterns
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
│   │   │   ├── runner.rs       # Main orchestration (8-phase pipeline)
│   │   │   ├── config.rs       # Project configuration
│   │   │   └── cache.rs        # File cache for incremental scans
│   │   ├── auditor/            # Per-file audit logic
│   │   ├── findings/           # Finding modules (unused_var, dead_branch, etc.)
│   │   └── suppression.rs      # Baseline suppression
│   ├── semantics/
│   │   ├── symbols.rs          # SymbolRegistry (cross-file symbol table)
│   │   ├── data_flow/
│   │   │   ├── resolve.rs      # Taint source seeding (REGEX-BASED — known limitation)
│   │   │   ├── tracking.rs     # Intra-procedural taint propagation
│   │   │   ├── interprocedural.rs # Cross-function taint
│   │   │   ├── cross_file.rs   # Cross-file taint via call graph
│   │   │   └── corpus_seeder.rs # Seeds taint from corpus matches
│   │   ├── consistency.rs      # Cross-path consistency checking
│   │   └── simple_taint.rs     # Lightweight taint check
│   ├── patcher/                # Auto-remediation (--fix)
│   ├── reporter/               # Output: text, JSON, SARIF
│   └── lib.rs                  # Public API
│
├── corpus/
│   ├── targets/                # Positive/negative example pairs
│   │   ├── *_positive.{rs,ts}  # Buggy code
│   │   ├── *_negative.{rs,ts}  # Fixed code
│   │   └── *.toml              # Metadata (severity, observation, impact)
│   ├── ground_truth/           # Validation datasets
│   └── baselines/              # Baseline scans
│
├── tests/                      # Integration tests
├── scripts/                    # Build, validation, metrics tools
└── docs/                       # Documentation
    ├── TECHNICAL_REFERENCE.md  # Deep source code analysis
    └── LIMITATIONS_MAP.md      # Visual limitations guide
```

---

## Critical Files to Understand

### 1. The Detection Pipeline
**File:** `src/engine/project/runner.rs`

This is the main orchestration file. It runs 8 phases:
1. File discovery & parsing
2. Symbol registry construction
3. Parallel audit (per-file finding modules)
4. Corpus pattern matching
5. Taint analysis
6. Cross-file taint
7. Severity overrides & composition
8. Output

### 2. Fingerprinting
**File:** `frensense-engine/src/fingerprint.rs`

Every function is fingerprinted into 7 dimensions:
- `ngram_hashes` — Positional token n-grams
- `weighted_ngram_hashes` — IDF-weighted n-grams
- `signature_ngrams` — Function signature text
- `param_type_ngrams` — Parameter types only
- `name_segments` — camelCase/snake_case split
- `structural_markers` — Abstract AST node kinds
- `type_usages` — All type identifiers used

### 3. Corpus Scoring
**File:** `frensense-engine/src/corpus/registry.rs`

5-dimensional contrastive scoring:
```
score = sim_to_positive × (1 - sim_to_negative) × cross_lingual_penalty

sim_to_positive =
    weighted_jaccard(ngrams) × 0.35
  + jaccard(structural) × 0.30
  + jaccard(signature) × 0.20
  + jaccard(param_types) × 0.10
  + type_usage_overlap × 0.05
```

### 4. Taint Source Seeding (KNOWN LIMITATION)
**File:** `src/semantics/data_flow/resolve.rs`

**CRITICAL:** Taint is seeded by variable NAME (regex), not by actual data origin.
This causes 100% false positive rates on some codebases.

### 5. TOCTOU Detector (KNOWN LIMITATION)
**File:** `frensense-engine/src/semantic_patterns/check_then_act.rs`

**CRITICAL:** Only recognizes Prisma ORM patterns.
TypeORM, Sequelize, Knex, raw SQL → NOT DETECTED.

---

## Known Limitations

| Limitation | File | Impact | Fix |
|------------|------|--------|-----|
| Taint seeded by regex (name-based) | `frensense-engine/src/data_flow/resolver.rs` | High FP rate | T-FIX-1 (AST entry points) |
| TOCTOU detector Prisma-only | `frensense-engine/src/semantic_patterns/check_then_act.rs` | Misses TOCTOU in other ORMs | Add corpus patterns |
| Sanitizers not recognized | `frensense-engine/src/data_flow/resolver.rs` | Taint flows through sanitizers | T-FIX-3 |
| Cross-file taint limited depth | `frensense-engine/src/data_flow/cross_file.rs` | Complex taint paths missed | Increase depth limit |

## Undocumented Modules

These modules exist in the engine but aren't documented anywhere:

| Module | File | What It Does |
|--------|------|--------------|
| `ast_distance` | `frensense-engine/src/ast_distance.rs` | Tree edit distance (used by scorer for structural similarity) |
| `cfg` | `frensense-engine/src/cfg.rs` | Control flow graph with def-use chains |
| `reachability` | `frensense-engine/src/reachability.rs` | Reachability analysis |
| `profile` | `frensense-engine/src/profile.rs` | Project profiling from fingerprints |
| `graph` | `frensense-engine/src/graph.rs` | SemanticGraph (call graph, taint flow, temporal events) |

## Known Code Duplication

| What | Location 1 | Location 2 | Issue |
|------|-----------|-----------|-------|
| SemanticGraph | `frensense-engine/src/graph.rs` | `src/semantics/graph.rs` | Near-identical with subtle behavior differences |
| Temporal event extraction | `frensense-engine/src/temporal.rs:273` | `frensense-engine/src/graph.rs:289` | Different pattern matching logic |

## Important Features (Often Overlooked)

### Semantic Filters (Reduce Corpus False Positives)

**File:** `corpus/semantic_filters.toml` + `frensense-engine/src/corpus/semantic.rs`

Before scoring, patterns pass through AST-level constraints:

```toml
# Example: Promise chain without .catch()
[ts_llm_promise_catch]
contains_call_to = [".then"]
must_not_contain_call_to = [".catch", ".finally"]

# Example: Sanitizer passthrough
[ts_csa_sanitize_passthrough]
function_name_regex = "^sanitize"
must_not_contain_call_to = [".replace", ".encode", "encodeURIComponent"]
```

### Taint Verification for Corpus Findings

**File:** `src/engine/project/runner.rs:175-215`

Corpus findings are verified against taint flow. If verified:
- Confidence boosted by 20% (capped at 0.95)
- Tagged with `taint-verified`
- Impact text includes taint flow detail

### Per-Category Calibration

**File:** `src/engine/project/runner.rs:166-173`

Confidence can be calibrated per pattern category:
- `sec` — security patterns
- `csa` — contract surface analysis
- `llm` — LLM anti-patterns
- `arch` — architecture patterns
- `async` — concurrency patterns

---

## Patcher Limitations

**File:** `src/patcher/mod.rs`

- Import injection is **TypeScript-only** (regex `^import\s+.*`) — won't work for Rust `use` or Python
- No rollback mechanism — if atomic rename fails, `.patch_tmp` file is left behind
- No multi-file atomic patches — each file patched independently
- Context mismatch is fatal — no partial application

## Secret Scanner Limitations

**File:** `frensense-engine/src/secrets.rs`

- No Azure/Azure DevOps, GitLab, npm, PyPI tokens
- `generic_api_key` requires quotes around value — unquoted `apiKey = abc123` missed
- No base64 detection
- `scan_tree` only looks at string nodes — misses secrets in comments

## Temporal Rules Limitations

**File:** `frensense-engine/src/temporal.rs`

- Line-number-based ordering only — doesn't account for control flow
- No scope awareness — events from different functions mixed
- Built-in rules: lock/unlock, acquire/release, open/close, connect/disconnect, lock+sleep

## Config Limitations

**File:** `src/engine/project/config.rs`

- Only 3 YAML options: `rules_dir`, `disabled_rules`, `severity_override`
- Many CLI flags (thresholds, corpus config) not configurable via config file
- Silent fallback to defaults on parse error

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

## Taint Rules (Embedded, Not in File)

**Note:** There is no `taint_rules.toml` file. Taint rules are embedded in the engine code.
The `--extra-taint-rules` flag loads user-provided TOML files that extend (not replace) built-in rules.

## Build Scripts

**File:** `scripts/`

| Script | Purpose |
|--------|---------|
| `classify_findings.py` | Classify findings as TP/FP |
| `compute_metrics.py` | Compute precision/recall metrics |
| `corpus_check.py` | Validate corpus pattern completeness |
| `validate_csa_depth.py` | Validate CSA pattern depth criteria |
| `train_calibration.py` | Train Platt scaling for confidence calibration |
| `harvest_corpus.py` | Harvest corpus patterns from CVEfixes |
| `deduplicate_corpus.py` | Remove duplicate corpus patterns |
| `learn_from_pairs.py` | Learn from positive/negative pairs |
| `benchmark.sh` | Run benchmark suite |
| `local-ci.sh` | Local CI pipeline |

## How to Add New Detection

### Corpus Pattern (Preferred)

```bash
# 1. Create positive example (the bug)
corpus/targets/ts_my_pattern_positive.ts

# 2. Create negative example (the fix)
corpus/targets/ts_my_pattern_negative.ts

# 3. Create sidecar TOML
corpus/targets/ts_my_pattern.toml

# 4. Rebuild bundle
cargo run --bin build-corpus-bundle
```

### Hardcoded Detector

```rust
// 1. Create detector
frensense-engine/src/semantic_patterns/my_detector.rs

// 2. Implement SemanticPattern trait
impl SemanticPattern for MyDetector {
    fn id(&self) -> &str { "MY_DETECTOR" }
    fn scan(&self, tree: Node, source: &str, file_path: &str) -> Vec<PatternFinding> { ... }
}

// 3. Register in semantic_patterns/mod.rs
```

---

## Testing

```bash
# All tests
cargo test

# Engine tests only
cargo test -p frensense-engine

# Specific pattern test
cargo test -p frensense-engine -- toctou

# Integration tests
cargo test --test rule_tests

# MCP tests
cargo test --test mcp_tests
```

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
--diff [scope]              # Show diff without applying: style, security, all
--strict                    # Exit code 1 if any findings
--confidence <tier>         # high (≥0.85), medium (≥0.60), low (≥0.30), any
--min-confidence <0-1>      # Raw confidence floor
--extra-taint-rules <dir>   # Custom taint rules
--check-deps                # Enable dependency checking (Rust)
--learn-profile             # Build project style profile
--check-profile             # Check code against learned profile
--profile-threshold <0-1>   # Surprise threshold for anomaly detection
--emit-baseline <file>      # Save current findings as baseline
--compare-baseline <file>   # Compare against baseline
--update-baseline           # Update baseline with current findings
--disable-rule <id>         # Suppress specific rule
--override-severity <id>:<level>  # Change rule severity
--tag <tag>                 # Filter findings by tag
--language <lang>           # Filter by language
--ngram-window <N>          # Token n-gram window size
--min-ngram-count <N>       # Minimum n-gram occurrences
--jaccard-threshold <0-1>   # Similarity threshold for near-duplicates
--max-source-lines <N>      # Skip files with more than N lines
--taint-conf-inter <0-1>    # Inter-procedural taint confidence
--taint-conf-intra <0-1>    # Intra-procedural taint confidence
--taint-max-depth <N>       # Max taint propagation depth
```

### Taint Rules (TOML)

```toml
[[rules]]
id = "CUSTOM_RULE"
source = "input|user_input"
sink = "exec|eval"
severity = "critical"
observation = "Description of the bug."
impact = "What could go wrong."
improvement = "How to fix it."
sanitizers = ["sanitize", "escape"]  # Not yet implemented!
```

---

## Build Artifacts

- `frensense-corpus.frc` — Pre-compiled corpus bundle (embedded in binary)
- `target/release/frensense` — Release binary
- `.frensense/` — Project configuration directory

---

## References

- `docs/TECHNICAL_REFERENCE.md` — Deep source code analysis with diagrams
- `docs/LIMITATIONS_MAP.md` — Visual guide to what works vs what doesn't
- `ARCHITECTURE.md` — Architecture decisions and gaps
- `ROADMAP.md` — What to build next, ordered by impact
- `tasks.md` — Current task tracker
