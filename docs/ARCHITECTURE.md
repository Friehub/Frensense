# Frensense Architecture

## Module Map (frensense-engine/src/)

### Core Fingerprinting (`fingerprint.rs`)
16 dimensions extracted per function: n-grams (weighted by IDF), structural markers, signature,
param types, type usages, semantic markers, skeleton (AST node kind sequence), control flow
(branch/loop/return patterns), API calls + segments, motif hashes, data-flow path hashes,
tainted API calls, property accesses, name segments, comment density, raw call names.
Entry point: `extract_fingerprints()` → returns `Vec<(FunctionFingerprint, Node)>`.

### MinHash LSH (`minhash.rs`)
128 hash functions via FxHasher with seeded permutations. LSH with bands/rows configurable
(currently 40 bands, 3 rows). Per-band `HashMap<u64, Vec<u64>>` for scalable bucket storage
(essential for 45k target corpus). `jaccard_similarity_sorted()` for exact set similarity.

### AST Edit Distance (`ast_distance.rs`)
Tree edit distance via LCS on structural skeletons (capped at 256 nodes). Node kinds
normalized: for/while→`loop_node`, if/switch→`branch_node`, catch/try→`catch_node`.
Returns [0,1] where 0 = identical.

### Contrastive Scoring (`pattern/scorer.rs`)
`score = sim_to_best_positive × (1 - max_neg_sim) × cross_lingual_penalty × context_multiplier`
11 dimensions weighted by learned or default weights. Cross-lingual penalty: 1.0 if same
language or TS↔JS, 0.20 for genuinely different languages. Semantic multiplier: 0.30 if
semantic markers don't overlap, 2.0 if they do.

### Weight Learner (`pattern/weight_learner.rs`)
Per-category logistic regression on 11-dim feature vectors. Categories extracted as
`pattern_id.split('_').nth(1)`. Balanced gradient descent (equal pos/neg weight per-class),
200 iterations, learning rate 0.1, L1-normalized. Requires ≥20 training pairs per category.

### Pattern Registry (`corpus/registry.rs`)
Main orchestrator: holds patterns, LSH indices, IDF weights, category weights, calibration
sigmoids, source/sink registry, auto-filter stats. `score_function()` is the main entry point:
queries LSH, applies semantic filters, scores candidates, applies calibration.

### Bundle (`corpus/bundle.rs`)
FRC1 format: [u32 header_len] [BundleHeader] [BundlePayload]. Header: magic "FRC1", version,
pattern_count, blake3 checksum. Payload: patterns (Vec<BundlePattern>), API IDF weights,
category weights, auto-filter stats, calibration params. Embedded in binary via `include_bytes!()`.

### Semantic Filters (`corpus/semantic.rs`)
10 constraint types: `contains_call_to`, `function_name_regex`, `must_not_contain_call_to`,
`contains_node_type`, `must_not_contain_node_type`, `required_taint_flows`,
`must_not_match_function_name`, `must_not_match_file_path_pattern`,
`contains_import`, `must_not_contain_import`. Applied as gate before scoring.

### Auto-Filter (`auto_filter.rs`)
Learns 6 constraint types from corpus pairs: `contains_import`, `contains_call_to`,
`excludes_call`, `function_name_regex`, `excludes_node_type`, `excludes_function_name`.
Uses category-level exclusivity ratios (≥25% within category, ≥3× more frequent than outside)
and per-pattern negative comparison. Completely replaces hand-crafted filters.

### Motifs (`corpus/motifs.rs`)
9 canonical sink/source groups: CommandExecutionSink, SqlSink, HttpOutboundSink,
FileReadSink, FileWriteSink, DeserializeSink, EvalSink, HttpResponseSink, CryptoWeakSink.
Each maps 5-15 API call variants to one canonical name. Built as `LazyLock<HashMap>`.

### Flow Fingerprint (`corpus/flow_fingerprint.rs`)
Lightweight AST-only source→sink chain detection. O(n²) in assignments × calls.
Produces invariant paths like `UserInputSource → taint_flow → CommandExecutionSink`.

### Source/Sink Registry (`corpus/source_sink.rs`)
Corpus-learned: parameter types → sources, function calls → sinks. Three tiers:
HighConfidence, Standard (default), Suspicious. Plus 80+ hardcoded fallback sinks.

### Context Classification (`context/mod.rs`)
Environment detection from path + content: Test, Mock, Config, RouteHandler, Utility, Unknown.
20+ heuristics including `(req, res)`, `app.get(`, `router.post(`. DataSensitivity tracking.
Used for bidirectional context penalty in scoring.

### Function Role Classifier (`function_role.rs`)
5 roles: HttpHandler, DbQuery, ShellExecutor, DataTransformer, Unknown. Used as pre-filter
before scoring — incompatible roles skip scoring entirely.

## Data Flow (`data_flow/`)
- `engine.rs`: per-function taint summaries, global taint registries
- `propagators.rs`: taint propagation rules per AST node kind
- `cross_file.rs`: cross-file taint verification using learned sources/sinks
- `alias.rs`: variable alias tracking
- `sanitizer.rs`: sanitizer function detection
- `entropy.rs`: Shannon entropy for secret detection
- `pii.rs`: PII regex patterns
- `confidence.rs`: taint-verified confidence boost (+20%, capped 0.95)

## Per-Pattern Calibration (`per_pattern_calibration.rs`)
Logistic regression sigmoid: `P(tp | s) = 1 / (1 + exp(-(A×s + B)))`. Trained at build time
by holdout validation. Falls back to per-category Platt scaling when <10 examples.

## Directory: src/ (Binary + CLI)

### Runner (`engine/project/runner.rs`)
Main pipeline: `collect_files` → `fingerprint` → `score` → `verify_taint` → `report`.
Parallelizes file processing via Rayon. Captures timing, caches unchanged files.

### CLI (`cli/`)
- `options.rs`: argument parsing
- `commands.rs`: entry points
- `reporting.rs`: text/JSON/SARIF output, deduplication, filtering, baselines

### Reporter (`reporter.rs`)
SARIF 2.1.0 output with CWE relationships, JSON output with match evidence.
Format selection via `--json`, `--sarif` flags.

## Key Design Decisions

1. **Deterministic hashing**: No ML, no GPU, no floating point variance. Same code → same
   result every run. Enables CI/CD caching, auditability, and reproducible findings.

2. **Contrastive scoring**: Both positive AND negative examples scored at runtime.
   Negative similarity directly penalizes the final score — not just training signal.

3. **No hand-crafted filters**: All semantic constraints auto-learned from corpus.
   6 constraint types replace ~150 manually authored filters.

4. **11-dimensional similarity**: Each dimension independently tunable. Evidence block
   is a byproduct of scoring, not post-hoc explanation.

5. **Bundle embedding**: Pre-computed fingerprints in binary. Zero startup I/O.
   Falls back to directory loading when bundle format mismatches.

## Known Issues

1. **flow_sim weight = 0.02** — too low for data-flow paths to be the primary
   generalization signal. Should be 0.10-0.12 to make similarity API-invariant
   and eliminate the need for M1-M15 mutation variants. See SCORING_DIMENSIONS.md.

2. **1244 patterns below quality score 50** — corpus debt from LLM-generated toy code.
   Each rewrite directly improves FP/TP rates. Track via `cargo run --bin corpus-quality`.

3. **Bundle rebuild takes 10-15 minutes** — 4000+ files × tree-sitter parsing.
   Incremental rebuild planned but not implemented.
