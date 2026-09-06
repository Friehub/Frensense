# Auto-Filter Learning System

The auto-filter replaces all hand-crafted semantic filters. It learns 6 constraint types
directly from the corpus positive/negative pairs. **No manual rules, no TOML, no YAML.**

## How It Learns (Algorithm)

`compute_auto_filters()` in `frensense-engine/src/auto_filter.rs` runs during bundle
building (`build-corpus-bundle`) and also as a fallback during directory-based loading.

### Phase 1: Category-Level Exclusivity (Lines 48-108)

Groups patterns by category (e.g. `ssrf`, `cmdi`, `sqli` — extracted as
`pattern_id.split('_').nth(1)`). For each category, counts which imports and call targets
appear. A call is added to `contains_call_to` for ALL patterns in the category if:

1. It appears in ≥25% of category positives (`CALL_WITHIN_RATIO = 0.25`)
2. It is ≥3× more frequent in this category than outside (`CALL_EXCLUSIVITY = 3.0`)
3. Requires ≥5 patterns in the category (`MIN_POSITIVES = 5`)

**Example**: 20 `ssrf` patterns, 15 use `fetch`, 5 use `request`. `fetch` appears in 75%
of ssrf but only 2% of non-ssrf patterns. Ratio = 37.5× → learned as `contains_call_to`.

**Why multi-API variants help**: If SSRF has 5 API variants (`fetch`, `request`, `axios`,
`got`, `http.get`), no single API reaches 25% frequency. The constraint relaxes from
"requires `fetch`" to "requires any of these" — the scorer becomes the discriminator.

### Phase 2: Per-Pattern Negative Exclusivity (Lines 119-173)

For each pattern individually, compares positives vs negatives:

| Constraint | Logic | Effect |
|------------|-------|--------|
| `contains_call_to` | Calls in positives NOT in negatives | "Must call this API" |
| `excludes_call` | Calls in negatives NOT in positives | "Must NOT call this API" |
| `excludes_node_type` | Node types in negatives NOT in positives | "Must not have this AST node" |
| `excludes_function_name` | Function names in negatives NOT in positives | "Must be named differently" |
| `function_name_regex` | Common prefix among ALL positives (≥4 chars) | "Must start with this prefix" |

These use `extract_call_targets()` which now emits BOTH qualified names (`res.redirect`)
and short names (`redirect`), enabling fine-grained constraints.

## How Constraints Are Applied at Scan Time

In `registry.rs` → `scan_function()`, line 359-365:

```rust
let merged_filter = match (&pattern.semantic_filter, &self.auto_filter_stats) {
    (Some(hand), Some(auto)) => Some(merge_filters(hand, Some(auto), &pattern.id)),
    (None, Some(auto)) => Some(merge_filters(&Default::default(), Some(auto), &pattern.id)),
    (None, None) => None,
};
```

`merge_filters()` (in `auto_filter.rs` line 186-230) combines the pattern's built-in filter
(now always empty since hand-crafted filters were removed) with the auto-derived stats.

The merged filter is checked by `SemanticFilter::matches()` in `semantic.rs`:
- `contains_call_to`: function must call at least one of these APIs (case-insensitive contains)
- `contains_import`: file must import from one of these packages
- `must_not_contain_call_to`: function must NOT call these
- `function_name_regex`: function name must match this regex
- `must_not_match_function_name`: function name must NOT match
- `must_not_match_file_path_pattern`: file path must NOT match

If ANY check fails, the pattern is skipped (no scoring).

## Source Text Extraction

The auto-filter reads source files to extract imports and call targets. The bundle builder
recursively searches all subdirectories of `corpus/targets/` for matching files:

- Positives stored under `pattern_id` key
- Negatives stored under `{pattern_id}_neg` key
- `extract_imports()`: scans for `from "..."` and `require("...")`
- `extract_call_targets()`: scans for `identifier(` → emits both full and short name
- `extract_node_types()`: keyword-based (crude), detects `return`, `if`, `for`, etc.

## FileContext Detection

`FileContext::extract()` in `frensense-engine/src/context/mod.rs` classifies each
scanned file by environment (Test, Mock, Config, RouteHandler, Utility). It uses
20+ content-based heuristics:

- Path contains `route`, `controller`, `handler`, `endpoint`, `api/`
- Code contains `(req, res)`, `app.get(`, `router.post(`, `res.json(`, `res.redirect(`
- Also checks `request, response`, `c.req`, `router.`

This works for any directory convention (routes/, handlers/, controllers/, pages/api/).

The `context_multiplier` in the scorer (line 381-400) penalizes mismatches:
- Pattern expects RouteHandler but file is Test/Utility → 0.5× penalty
- Pattern expects non-RouteHandler but file IS RouteHandler → 0.5× penalty

## Key Files

| File | Purpose |
|------|---------|
| `frensense-engine/src/auto_filter.rs` | `compute_auto_filters()`, `merge_filters()`, `extract_*()` |
| `frensense-engine/src/corpus/semantic.rs` | `SemanticFilter` struct, `matches()` method |
| `frensense-engine/src/corpus/loader.rs` | `parse_frensense_block()`, corpus file loading |
| `frensense-engine/src/context/mod.rs` | `FileContext::extract()` environment detection |
| `frensense-engine/src/corpus/bundle.rs` | Bundle serialization, `find_corpus_file()` search |
| `src/bin/corpus-quality.rs` | Quality scoring tool (0-100 per pattern) |
