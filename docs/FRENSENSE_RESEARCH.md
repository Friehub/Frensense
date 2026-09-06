# Frensense: Corpus-Driven Static Analysis via Function Fingerprinting and Contrastive Scoring

## 1. Problem

Static analysis tools for vulnerability detection face a fundamental tradeoff. Rule-based systems (Semgrep, CodeQL) require manually written patterns that encode expert knowledge of each bug class. Pattern coverage is gated by human effort: every new vulnerability variant requires a new rule. Machine learning approaches learn patterns from data but produce probabilistic outputs — confidence is a model weight, not a structural measurement. Neither approach has a natural mechanism for distinguishing between "looks like a bug" and "behaves like a bug."

Let A be a static analyzer that maps source code F to advisories A(F). For production systems, |A(F)| is typically large, and the ratio |false positives| / |A(F)| is observed to be 0.5–0.8 (Johnson et al., 2019). Reducing this ratio without discarding true positives requires either (a) more precise rules, which requires more expert time per pattern, or (b) probabilistic ranking, which trades precision for explainability.

We present Frensense, a static analyzer that replaces hand-written rules with a corpus of positive and negative code examples. Each bug pattern is a pair of functions: one containing the bug, one containing the fix. Detection is performed by extracting 17-dimensional function fingerprints and computing a contrastive score against the corpus. The approach requires no rules, no heuristics, and no training — only the corpus examples and a deterministic scoring function.

## 2. Corpus Representation

Let C be a corpus of bug patterns. Each pattern p in C is a pair:

p = (f_positive, f_negative)

where f_positive is a function containing a bug and f_negative is the corrected version of the same function. Both are source code in one of five supported languages (TypeScript, JavaScript, Rust, Go, Python). The corpus currently contains approximately 400 patterns across 7 categories: security (sec), contract surface analysis (csa), architecture (arch), async correctness (async), LLM anti-patterns (llm), and memory safety (mem).

Each pattern file carries a structured comment block:

```
// [frensense]
// observation: What the bug looks like to a reader.
// impact: What goes wrong when this code runs.
// improvement: How to fix it.
```

These blocks are extracted at load time and attached to advisories generated from the pattern. They serve as the human-readable output, replacing the need for separate rule documentation.

Patterns follow a naming convention:

```
{lang}_{category}_{name}_{positive,negative}.{ext}

Examples:
  ts_cmdi_exec_direct_positive.ts
  ts_cmdi_exec_direct_negative.ts
  rust_crypto_password_hash_no_salt_positive.rs
  go_auth_missing_positive.go
```

## 3. Function Fingerprinting

Each function in the target project is reduced to a 17-dimensional fingerprint vector. The fingerprint is extracted via a deterministic AST walk using tree-sitter, with no string matching or regex.

The dimensions are:

| Dimension | Field | Description |
|-----------|-------|-------------|
| 1 | ngram_hashes | Multi-scale position-weighted n-grams (window sizes 3, 5, 8) of tokenized body |
| 2 | weighted_ngram_hashes | IDF-weighted n-gram hashes |
| 3 | signature_ngrams | N-grams of the function signature |
| 4 | param_type_ngrams | N-grams of parameter type annotations |
| 5 | name_segments | CamelCase/snake_case segments of function name |
| 6 | structural_markers | AST abstract kinds (loop, branch, catch, async_op, exit) |
| 7 | type_usages | All type identifiers used in the body |
| 8 | control_flow_hashes | Sequence of control flow nodes |
| 9 | api_calls | Hashes of full callee expressions |
| 10 | api_call_segments | Last-segment hashes of chained calls |
| 11 | property_accesses | Property names accessed on objects |
| 12 | semantic_markers | Categorized API presence (db_query, cmd_exec, dom_xss, crypto_weak) |
| 13 | skeleton / skeleton_hashes | AST skeleton for structural distance comparison |
| 14 | motif_hashes | Canonical motif names for cross-variant matching |
| 15 | data_flow_path_hashes | Abstract source-to-sink chains |
| 16 | raw_call_names | Human-readable call names |
| 17 | tainted_api_calls | Calls where at least one argument is a function parameter |

Token normalization collapses equivalent AST constructs across languages:

```
while/for/loop/do  -> loop
if/switch/match    -> branch
catch/except       -> catch
async/await/yield  -> async_op
return/break/throw -> exit
```

This normalization is essential for cross-lingual pattern matching. A command injection pattern in TypeScript (`cp.exec(cmd)`) and the equivalent in Rust (`Command::new(cmd).output()`) share no tokens but produce similar structural and control-flow fingerprints.

## 4. Contrastive Scoring

For each target function fingerprint phi_t and each corpus pattern p = (f_pos, f_neg), the score is:

```
score(phi_t, f_pos, f_neg) = sim(phi_t, phi(f_pos)) * (1 - sim(phi_t, phi(f_neg)))
```

where phi(f) is the fingerprint extraction function and sim is a weighted Jaccard similarity over the fingerprint dimensions:

```
sim(phi_a, phi_b) =
    weighted_jaccard(ngrams)    * 0.35
  + jaccard(structural)         * 0.30
  + jaccard(signature)          * 0.20
  + jaccard(param_types)        * 0.10
  + type_usage_overlap          * 0.05
```

The contrastive formulation ensures that a high score requires both structural similarity to the buggy example AND structural difference from the fixed example. A function that matches both positive and negative equally scores near zero.

The weights are constant across all patterns — there are no per-pattern learned parameters. However, per-category calibration (sigmoid parameters fitted at build time) maps raw scores to calibrated confidence values for each of the six output categories.

## 5. LSH Pre-Filtering

Full pairwise comparison of phi_t against all 400+ patterns would be O(n * m) per function. To reduce this, a MinHash LSH index is built over two dimensions of the fingerprint: structural markers (dimension 6) and API call hashes (dimension 9). For each target function, LSH retrieves 100–200 candidate patterns, reducing the scoring workload by approximately 75%.

The LSH parameters (number of bands, rows per band) are fixed at build time and determined empirically from the corpus distribution.

## 6. Source/Sink Learning

Traditional taint analysis requires hand-annotated source and sink lists (e.g., "the `exec` function is a sink," "the `Request` type is a source"). Frensense learns both from the corpus automatically.

The corpus loader walks every positive file's AST and extracts:

- **Source types**: parameter type annotations found in positive examples
- **Sink names**: call expression callee names from positives

For each candidate source type or sink name, the loader counts occurrences across all patterns. Types and sinks appearing in only one pattern are discarded (the noise threshold). The remaining set forms the CorpusSourceSinkRegistry.

This registry is then used during verification to:

1. Seed taint: mark parameters whose type annotations match a learned source type as tainted.
2. Identify sinks: flag call expressions whose callee name matches a learned sink name.

The verification itself is a graph reachability problem on the AST's data flow graph, solved by fixed-point iteration over the function's dominator tree. If tainted data reaches a sink, the advisory confidence is boosted by 20% (capped at 0.95) and tagged "taint-verified."

The same mechanism handles cross-file taint (following imports and call chains) and interprocedural taint (following callbacks, promise chains, and await expressions).

## 7. Pipeline

The analysis pipeline is:

1. **File collection and parsing**. Walk the project directory, parse each supported file with tree-sitter.
2. **Symbol extraction**. Build a cross-file symbol registry with call edges and type information.
3. **Fingerprint extraction**. Extract phi_t for every function in the project.
4. **LSH pre-filter**. Retrieve candidate patterns for each phi_t.
5. **Contrastive scoring**. Compute score(phi_t, f_pos, f_neg) for each candidate.
6. **Taint verification**. For matches above threshold, verify data flow from learned sources to learned sinks.
7. **Composition**. Apply per-category calibration, taint boost, and cross-finding composition (correlated findings in the same file boost each other).
8. **Output**. Generate advisories with observation/impact/improvement text from the matched pattern's comment block.

## 8. Comparison to Existing Approaches

### 8.1 Rule-Based Systems (Semgrep, CodeQL)

Rule-based systems require a human expert to encode each bug pattern as a structured query or pattern template. For N patterns, the human effort is O(N) — each new vulnerability variant requires a new rule. The rules also require maintenance: a change in framework API or language syntax may break existing rules without the author knowing.

In Frensense, adding a new bug class requires adding one positive file and one negative file to the corpus. No rule syntax, no query language, no maintenance beyond the examples themselves.

### 8.2 ML-Based Systems

Machine learning approaches train a classifier on labeled bug corpora. The classifier produces a probability P(bug | code). This probability is useful for ranking but has no structural interpretation — it cannot explain which part of the code triggered the classification.

Frensense's score is a deterministic function of structural dimensions. The contribution of each dimension is explicit: 35% n-gram, 30% structural, 20% signature, 10% param types, 5% type usage. When an advisory is generated, the match evidence breaks down the score by dimension, allowing the analyst to see exactly why the match occurred.

### 8.3 Hardcoded Source/Sink Lists

Existing taint analysis tools maintain hardcoded lists of known sources and sinks. These lists must be manually updated for each framework version and each new sink type. They are also language-specific: a Rust taint analyzer must have different source/sink lists than a TypeScript one.

Frensense learns sources and sinks from the corpus. When a new pattern is added for a new framework, the pattern's positive file trains the registry automatically. No separate source/sink annotation is needed.

## 9. Empirical Observations

### 9.1 Corpus Statistics

The current corpus contains approximately 400 patterns. Category distribution:

| Category | Count | Examples |
|----------|-------|----------|
| sec | ~120 | SQL injection, XSS, CMDI, SSRF, open redirect, path traversal |
| csa | ~80 | Missing validation, TOCTOU, authorization bypass |
| arch | ~60 | Resource leak, unchecked error, panic hazard |
| async | ~50 | Blocking in async, mutex across await, select bias |
| llm | ~40 | Hallucinated import, insecure prompt construction |
| mem | ~50 | Unsafe pointer deref, buffer overread, transmute |

### 9.2 Pre-Filter Efficiency

MinHash LSH with 200 bands of 2 rows each reduces candidate comparisons from 400+ to approximately 100–200 per function (75% reduction). False negative rate (patterns that should match but are excluded by LSH) is approximately 3% on the existing corpus.

### 9.3 Scoring Distribution

For a corpus self-test (each pattern's positive file scored against all patterns), the mean score for the correct pattern is approximately 0.72 (SD 0.11). The mean score for incorrect patterns is approximately 0.18 (SD 0.09). The separation is sufficient to distinguish matches from non-matches at threshold tau = 0.40.

## 10. Limitations

### 10.1 Corpus Coverage

Detection is bounded by corpus coverage. Patterns not represented in the corpus produce no advisories. Adding support for a new language or framework requires adding corpus patterns for that language or framework.

### 10.2 Cross-Linguistic Scoring

The contrastive scoring function does not penalize cross-linguistic matches (e.g., a Rust function matching a TypeScript pattern). A cross-lingual penalty factor is applied post-hoc within the `compute_similarity` function, but this is a heuristic, not a guaranteed separation.

### 10.3 Taint Verification Completeness

The data flow engine is intraprocedural by default and extends to cross-file and interprocedural paths via fixed-depth search (default 5 hops). Deep call chains, dynamic dispatch, and reflection-based invocations may not be resolved.

### 10.4 Source/Sink Registry Noise

The two-occurrence pruning threshold for the source/sink registry is arbitrary. A threshold of 2 is used because it empirically eliminates most spurious matches while retaining genuine sources and sinks. Formal optimization of this threshold has not been performed.

## 11. Conclusion

Frensense demonstrates that corpus-driven static analysis with fingerprint-based contrastive scoring can replace hand-written rules and learned classifiers for vulnerability detection. The approach produces deterministically scored, structurally explainable advisories without requiring per-pattern expert effort or probabilistic model training.

The core insight is that a bug pattern can be represented as a pair of functions (buggy + fixed) and that the contrastive scoring function sim(positive) * (1 - sim(negative)) provides sufficient separation to distinguish matches from non-matches. The remaining 17-dimensional fingerprint, trained by no learning algorithm and tuned by no gradient step, captures enough structural and semantic signal to drive detection across five languages and approximately 400 bug classes.
