# Frensense: 45,000-Pattern Scaling Plan

> How to scale FrenSense from 89 patterns to 45,000 known bugs with examples — treating the corpus like training data for a structural similarity engine.

---

## The Thesis

FrenSense was built in the AI agent era. Existing tools (Semgrep, CodeQL, Checkmarx) were built before LLMs existed. FrenSense's corpus layer is the first static analysis engine designed to learn from examples at scale — the same way we train AI models, but with explainable, deterministic output.

With 45,000 patterns, FrenSense becomes a **compiled knowledge base of every known vulnerability pattern**, embedded in a single binary, scannable in sub-second time per file.

---

## Why 45k Is Possible (Architecture)

### LSH Pre-Filter Eliminates Linear Scaling

At scan time, a function's 128-hash MinHash signature is queried against the LSH index. Only patterns sharing a band bucket (~100-200 candidates out of 45,000) get scored. This is the difference between O(45,000) and O(200) per function.

**Math for a 10,000-function codebase with 45k patterns:**
1. Fingerprint every function: O(n × k) where k=128 hashes
2. MinHash LSH query per function: O(bands × bucket_size) — returns ~100-200 candidates
3. 5-dimensional scoring per candidate: O(candidates × dimensions)
4. Total: ~10k functions × ~200 candidates × ~5 set ops = 10M ops — **still sub-second**

Without LSH: 10k × 45k × 2 (pos/neg) × 5 = 4.5B ops. With LSH: 10M ops. **450× reduction.**

### Bundle Embedding

45,000 patterns × ~2 positive/negative pairs × ~500 bytes per fingerprint = ~45MB. With `include_bytes!`, this is embedded in the binary. Zero disk I/O at startup. Binary size increase: ~45MB compressed.

### Cross-Language Transfer

A pattern trained on Rust matches TypeScript via the AbstractKind taxonomy. 45k patterns in Rust/TypeScript covers Python, C, and JS automatically. Effective coverage: ~135k language-pattern combinations.

---

## What Exists Today vs What's Needed

| Component | Today (89 patterns) | At 45k Patterns |
|-----------|---------------------|-----------------|
| FRC bundle | ~80KB, embedded | ~45MB, embedded |
| LSH index | 16 bands × 4 rows | 32 bands × 4 rows (better recall) |
| Corpus harvest | Manual (2 files per pattern) | Automated pipeline from CVE datasets |
| Pattern clustering | None | MinHash deduplication on corpus itself |
| Bundle build | Full rebuild | Incremental rebuild |
| Scan time (100 files) | ~500ms | ~800ms (estimated) |
| Memory at scan | ~1MB | ~50MB for LSH + fingerprints |

---

## The Harvest Pipeline

The bottleneck is not engine capacity — it's corpus acquisition. 45k patterns come from automated extraction, not manual authoring.

### Data Sources

| Source | Records | Format | Mapping to FrenSense |
|--------|---------|--------|---------------------|
| **CVEfixes** | 8,991 CVE fixes | Git patches (before/after commits) | before = `_positive`, after = `_negative` |
| **OSV.dev** | 40,000+ vulns | JSON with fix commit URLs | Follow fix commit → extract function diff |
| **GHSA** | 25,000+ advisories | API with linked fix commits | Same pipeline as OSV |
| **Semgrep rules** | 3,000+ rules | YAML with `_bad.ts`/`_ok.ts` test fixtures | Rename to FrenSense convention |
| **NIST Juliet** | CWE-labeled cases | Synthetic good/bad variants | Direct mapping (C/C++/Java focus) |
| **SecurityEval** | 130 LLM samples | CWE-labeled Python | LLM-specific patterns |

### Pipeline Architecture

```
scripts/harvest_corpus.py
├── harvesters/
│   ├── cvefixes.py        # Clone CVEfixes dataset, filter by language
│   ├── osv.py             # Query OSV API, follow fix commits
│   ├── ghsa.py            # Query GHSA API, extract fix references
│   ├── semgrep.py         # Walk semgrep-rules repo, extract test fixtures
│   └── juliet.py          # Parse Juliet test suite cases
├── extractors/
│   ├── git_diff.py        # Extract function-level diffs from patches
│   ├── tree_sitter_wrap.py # Wrap code snippets in compilable functions
│   └── validator.py       # Verify both files parse with tree-sitter
├── deduplicator/
│   ├── minhash_dedup.py   # MinHash the corpus itself to find duplicates
│   └── cluster.py         # Group patterns by CWE, keep representatives
└── output/
    └── corpus/targets/    # Write {lang}_{cwe}_{source}_{n}_{pos|neg}.{ext}
```

### Extraction Steps

1. **Clone CVEfixes** (~2GB): `git clone https://github.com/secureIT-project/CVEfixes`
2. **Filter by language**: `.rs`, `.ts`, `.js` files only
3. **Extract function-level diffs**: Parse `@@ -N,M +N,M @@` headers, extract full function context around each hunk
4. **Wrap in compilable functions**: If diff snippet is not already a function body, wrap in minimal function
5. **Validate with tree-sitter**: Both positive and negative must parse successfully
6. **Name convention**: `rs_cwe89_cvefixes_1_positive.rs`, `ts_cwe79_osv_1_negative.ts`
7. **Deduplicate**: MinHash all positives, cluster with Jaccard > 0.85, keep one representative per cluster
8. **Rebuild FRC bundle**: `cargo run --bin build-corpus-bundle`

### Expected Yield

| Source | Raw Pairs | After Dedup | After Validation |
|--------|-----------|-------------|------------------|
| CVEfixes | ~8,991 | ~6,000 | ~5,000 (parseable) |
| OSV | ~15,000 | ~10,000 | ~8,000 |
| GHSA | ~10,000 | ~7,000 | ~5,500 |
| Semgrep fixtures | ~2,000 | ~1,800 | ~1,800 |
| Manual LLM patterns | ~500 | ~500 | ~500 |
| **Total** | **~36,500** | **~25,300** | **~20,800** |

To reach 45k, supplement with:
- Hand-authored patterns for uncovered CWEs (estimated ~10,000)
- LLM-generated patterns using GPT-4/Claude as pattern generators (estimated ~15,000)
- Community contributions via corpus contribution guide

---

## LSH Index Tuning at Scale

### Current Configuration
- 128 hash functions, 16 bands, 8 rows per band
- Threshold for candidate match: any band bucket overlap

### At 45k Patterns
- Increase to 32 bands × 4 rows (same 128 hashes)
- More bands = lower false-negative rate (patterns that should match are less likely to be missed)
- Fewer rows per band = more candidates per query (acceptable tradeoff at 45k)

### Recall Optimization
- For 45k patterns, estimate ~150-250 candidates per function query
- At 5 dimensions × 2 (pos/neg) = 10 Jaccard computations per candidate
- Total scoring: ~2,500 Jaccard computations per function — negligible

---

## Pattern Clustering and Deduplication

At 45k patterns, redundancy is inevitable. 200 different SQL injection patterns may be 90% structurally similar.

### Deduplication Strategy

1. **MinHash all positive fingerprints** from the corpus
2. **LSH cluster** with Jaccard threshold 0.85
3. **For each cluster**: keep the pattern with highest validation score (TP rate on real codebases)
4. **Merge advisory text**: combine observation/impact/improvement from all cluster members

### Expected Reduction
- 45k raw patterns → ~25k unique clusters → ~15k representative patterns after merging
- Each representative has advisory text from the best-validated member

---

## FRC Bundle at Scale

### Size Estimates

| Patterns | Fingerprints (4 pos + 4 neg per pattern) | FRC Size | Compressed in Binary |
|----------|------------------------------------------|----------|---------------------|
| 89 | 364 | 80KB | ~40KB |
| 1,000 | 8,000 | ~900KB | ~450KB |
| 10,000 | 80,000 | ~9MB | ~4.5MB |
| 45,000 | 360,000 | ~40MB | ~20MB |

### Incremental Rebuild

Full rebuild of 45k patterns takes ~5-10 minutes (tree-sitter parsing + fingerprinting). For development iteration:

1. **Hash each source file** in `corpus/targets/`
2. **Compare against stored hashes** in a manifest file
3. **Only rebuild fingerprints** for changed files
4. **Merge into existing FRC bundle**

This reduces rebuild time from minutes to seconds for single-pattern changes.

---

## Scan Performance at Scale

### Benchmark Projections

Based on current benchmarks (535ms for ~25 files at 89 patterns):

| Files | 89 Patterns | 1,000 Patterns | 45,000 Patterns |
|-------|-------------|----------------|-----------------|
| 25 | 535ms | ~600ms | ~800ms |
| 100 | 4.6s | ~5.2s | ~7s |
| 1,000 | ~46s | ~52s | ~70s |
| 10,000 | ~8min | ~9min | ~12min |

The scaling is sub-linear because LSH filters 99%+ of patterns before scoring. The bottleneck at 45k is fingerprint extraction (tree-sitter parsing), not pattern matching.

### Optimization: Parallel File Scanning

Current engine scans files sequentially. With rayon:
- 4 threads: 100 files → ~1.8s (from ~7s sequential)
- 8 threads: 100 files → ~1.0s

The `Sequential` iteration note in BENCHMARK.md says "no rayon — adequate for typical project sizes." At 45k patterns, parallelization becomes worthwhile.

---

## Validation Strategy

### Phase 1: Automated Validation (Week 1-2)

1. **Harvest 10,000 CVE pairs** from CVEfixes
2. **Scan known-vulnerable repos** (historical versions with known CVEs)
3. **Measure recall**: what percentage of known CVEs does FrenSense detect?
4. **Measure precision**: what percentage of findings are true positives?

### Phase 2: LLM Pattern Validation (Week 3-4)

1. **Generate 1,000 LLM-written functions** (500 buggy, 500 clean) using GPT-4/Claude
2. **Scan with FrenSense** — measure detection rate on LLM-specific patterns
3. **Compare against Semgrep** on the same dataset
4. **Tune threshold** to maximize F1 score

### Phase 3: Production Validation (Week 5-6)

1. **Scan 5 real production codebases** (internal APIs, open-source projects)
2. **Classify every finding** as TP/FP (use `scripts/classify_findings.py`)
3. **Publish results** with TP/FP breakdown per pattern category
4. **Target**: ≥80% precision, ≥70% recall on production code

---

## Build Order

### Week 1: Harvest Pipeline (Scaffolded)
```
scripts/harvest_corpus.py                    # Main orchestrator ✅
scripts/harvesters/cvefixes.py               # CVEfixes dataset extraction ✅
scripts/harvesters/osv.py                    # OSV.dev API extraction ⚠️ (limited — API lacks FIX refs for most ecosystems)
scripts/extractors/git_diff.py               # Function-level diff extraction (inlined in harvesters)
scripts/extractors/tree_sitter_wrap.py       # Wrap snippets in compilable functions (inlined in harvesters)
scripts/extractors/validator.py              # Validate both files parse (inlined in harvesters)
```

**Status:** Pipeline scaffolded. CVEfixes harvester needs dataset clone. OSV harvester
limited by API (no FIX references for crates.io/npm/pypi — only ADVISORY/WEB links).
Best harvest path: CVEfixes dataset (has before/after commits) + manually authored
corpus pairs for high-value CWEs.

### Week 2: LSH Tuning + Clustering (Done)
```
frensense-engine/src/corpus/registry.rs      # Auto-scale bands 16→32 at >1000 patterns ✅
scripts/deduplicate_corpus.py                # MinHash dedup with LSH bucketing + union-find ✅
scripts/cluster_patterns.py                  # Merged into deduplicate_corpus.py ✅
```

### Week 3: Incremental Bundle Build (Partially Done)
```
src/bin/build-corpus-bundle.rs               # Full rebuild (incremental needs Rust-side changes)
scripts/update_bundle_manifest.py            # SHA-256 file tracking ✅
```
**Note:** True incremental bundle build requires modifying `build_bundle()` in
`frensense-engine/src/corpus/bundle.rs` to accept a fingerprint cache. The manifest
tracker detects which files changed; the bundle builder still re-fingerprints everything.

### Week 4: Validation Harness
```
scripts/validate_recall.py                   # Scan known-vulnerable repos, measure recall
scripts/validate_precision.py                # Classify findings on production code
scripts/benchmark_scale.py                   # Measure scan time at 1k, 10k, 45k patterns
```

### Week 5-6: Production Validation + Publishing
```
scripts/classify_findings.py                 # Interactive TP/FP classification
scripts/compute_metrics.py                   # Precision/recall reporting
corpus/TRACKING.md                           # Pattern validation status
BENCHMARK_SCALE.md                           # Published benchmark results
```

---

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Harvested pairs are noisy (refactoring, not bugs) | Low precision | Contrastive negative filtering in scorer; manual validation on sample |
| LSH recall drops at 45k patterns | Missing real bugs | Increase bands to 32; tune threshold; add secondary MinHash pass |
| FRC bundle too large for embedded binary | Slow startup | Use compression; fall back to memory-mapped file |
| Pattern clustering merges distinct patterns | Lost coverage | Keep cluster members accessible via `--list-patterns --show-cluster` |
| CVE dataset has language bias (more C/C++) | Low Rust/TS coverage | Supplement with GHSA (npm/crates.io) and manual patterns |
| Cross-language transfer produces false positives | Noise in non-target languages | Tune cross-lingual penalty from 0.75 to 0.50 at scale |

---

## Success Criteria

| Metric | Target | How Measured |
|--------|--------|--------------|
| Corpus patterns | 45,000 | `frensense --list-patterns \| wc -l` |
| Unique clusters | ~15,000 | Dedup script output |
| Scan time (100 files) | <10s | `benchmark_scale.py` |
| Precision on production code | ≥80% | `classify_findings.py` + manual review |
| Recall on known CVEs | ≥70% | `validate_recall.py` against CVEfixes |
| Binary size increase | <50MB | `ls -lh target/release/frensense` |
| False positive rate | <5% on clean code | Scan clean repos, count findings |

---

## References

- **CVEfixes** — Moonen, L., Vidziunas, L., & Bhandari, G. P. (2024). *CVEfixes: Automated Collection of Vulnerabilities and Their Fixes from Open-Source Software* (v1.0.8). 17th International Conference on Predictive Models and Data Analytics in Software Engineering (PROMISE), Athens, Greece. Zenodo. https://doi.org/10.5281/zenodo.13138703
  - 12,107 vulnerability-fixing commits, 4,249 open-source projects, 11,873 CVEs, 272 CWE types
  - Dataset covers all published CVEs up to 23 July 2024
  - Data license: CC BY 4.0

- **Semgrep Community Rules** — Semgrep, Inc. (2024). *Semgrep Rules Repository*. GitHub. https://github.com/semgrep/semgrep-rules
  - 3,000+ rules with `_bad`/`_ok` test fixtures
  - Test fixtures directly usable as corpus pairs (`_bad` = positive, `_ok` = negative)
