# v0.4.0 — Project Memory (Style-Anomaly Detection)

## Summary

GenSense currently detects duplicate boilerplate via n-gram Jaccard similarity (pairwise function comparison). v0.4.0 extends this into a **project-style profile** — a learned statistical model of "what code in this project looks like" — to flag LLM-generated or off-style code that violates unwritten team conventions.

## Motivation

Teams accumulate dozens of implicit conventions that no linter or style guide captures:

- All services use `export const serviceName = { async method(...) }` — never classes
- Monetary values use `Decimal`, never `number`
- Parameters are always typed — `any` has zero occurrences
- Method names are `camelCase`, function-local variables are `snake_case`
- Database access goes through a shared module, never injected via constructor

These aren't style choices — they're **project-specific invariants**. LLMs routinely violate them because no rule says "this project doesn't use classes." A statistical profile catches these without any rule-writing.

## Architecture

### Phase 1: Token Extraction (expand existing fingerprinting)

**Current:** `extract_fingerprints` tokenizes function bodies by whitespace into 5-gram `FxHashSet<u64>`.

**v0.4.0:** Extract richer features from each function — not just body n-grams:

| Feature | Source | Example |
|---------|--------|---------|
| **Body n-grams** | Whitespace-split tokens (existing) | `["async", "fn", "name", "(", "params"]` |
| **Signature n-grams** | Function declaration tokens | `["export", "const", "name", "=", "{"]` vs `["export", "class", "Name", "{"]` |
| **Parameter type n-grams** | Type annotations in params | `["userId:", "string", "cartId:", "string"]` |
| **Method name segments** | CamelCase/PascalCase boundaries | `createFromCart` → `["create", "From", "Cart"]` |
| **Structural markers** | AST node kinds in body | `["variable_declarator", "call_expression", "return"]` |
| **Type usage** | Type annotation occurrences | `["string", "number", "Decimal", "any"]` |
| **Comment density** | Comment bytes / total bytes | `0.02` (ratio) |

Each feature contributes to a **per-function fingerprint** stored as frequency maps rather than presence sets.

### Phase 2: Project Profile (the "memory")

After scanning the entire project, aggregate all fingerprints into a language-aware **project profile**:

```rust
pub struct ProjectProfile {
    version: String,
    language_profiles: HashMap<String, LanguageProfile>,
    generated_at: String,
    file_count: u32,
}

pub struct LanguageProfile {
    // Frequency distribution of all n-grams across the project
    ngram_frequencies: HashMap<u64, usize>,
    // Per-file profiles (for file-level anomaly scoring)
    file_profiles: Vec<FileProfile>,
    // Total token count for probability calculations
    total_ngrams: usize,
}
```

The profile is serialized to `.gensense/profile.json` and committed to the repo. This is the portable "memory" — CI can compare new code against it.

### Phase 3: Anomaly Scoring

For each function, compute a **style-surprise score** — the fraction of its n-grams that are rare or unseen in the project profile:

```rust
fn style_surprise(function: &FunctionFingerprint, profile: &ProjectProfile) -> f64 {
    let unseen = function.ngram_hashes
        .iter()
        .filter(|h| profile.ngram_frequencies.get(h).map_or(true, |&c| c < 2))
        .count();
    unseen as f64 / function.ngram_hashes.len() as f64
}
```

A score of `0.0` means every n-gram in the function is common in the project. A score of `0.8` means 80% of its n-grams are rare or never seen.

**Threshold:** Flag at `> 0.5` (strict) or `> 0.7` (default), configurable via CLI/API.

### Phase 4: Advisory Reporting

New rule: `STYLE_ANOMALY` (severity: `Warning`)

```
Observation: Function 'ProcessPayment' has 80% unfamiliar token patterns.
  This project uses camelCase for methods (seen 1,247 times).
  'ProcessPayment' uses PascalCase — seen 0 times in project.
  'any' type used — seen 0 times (project uses typed params).
Impact: Code that doesn't follow project conventions increases
  cognitive load and maintenance cost over time.
Improvement: Match the project's established patterns:
  - Use camelCase method names (createFromCart, cancelOrder)
  - Avoid 'any' — use proper types for all parameters
  - Use const service pattern instead of class
```

## File-Level Profiles (Noise Mitigation)

A project-global profile would flag test files (which have different conventions) as anomalous. Solution: maintain per-file-type and per-directory profiles alongside the global one.

- Separate profiles for `src/`, `tests/`, `scripts/`
- A function in `tests/` is compared against the test profile, not the src profile
- If a new directory appears, it gets scored against the closest matching profile

## CLI & API

```bash
# Build/refresh the project profile
gensense --learn-profile

# Audit with profile-based anomaly detection
gensense . --check-profile

# Audit with profile, scoring only new/changed files
gensense . --check-profile --diff-only

# View profile stats
gensense --profile-stats
```

```rust
// Rust API
let engine = Engine::with_profile(ProjectProfile::load(".gensense/profile.json")?)?;
let advisories = engine.run_with_profile(Path::new("./src"))?;
```

## Acceptance Criteria

1. `--learn-profile` scans the project, builds `.gensense/profile.json` with n-gram frequencies per language per file
2. An LLM-generated function with `any` types, PascalCase methods, or class syntax in a const-service project scores `> 0.5` surprise
3. A normal project function scores `< 0.3` surprise
4. Profile is deterministic — same project produces same hash (ignoring timestamps)
5. No false positives for test files when using file-level profiles
6. CI integration: `gensense . --check-profile --strict` fails if any function exceeds the threshold

## Effort Estimate

| Step | Time |
|------|------|
| Expand fingerprint extraction (signatures, types, structure) | 2h |
| Build `ProjectProfile` struct + serialization | 1h |
| Implement `style_surprise` scoring | 1h |
| File-level profile isolation | 1h |
| `--learn-profile` CLI command | 30m |
| `--check-profile` CLI integration | 30m |
| `STYLE_ANOMALY` rule + advisory message templates | 1h |
| `.gensense/profile.json` git integration | 15m |
| CI workflow integration | 15m |
| Tests (fixtures with known LLM-generated code) | 1h |

**Total:** ~8.5 hours

## Future Work (v0.4.1+)

- **Temporal decay:** Old functions contribute less to the profile (project conventions evolve)
- **Multi-repo profiles:** Cross-project style comparison ("does this code look like it belongs to this org?")
- **Profile diff:** When reviewing a PR, diff the PR branch's profile vs main to surface what the PR changes stylistically
- **Auto-remediation:** Suggest replacements for off-style patterns based on the profile (e.g., `any` → the most common type at that position)
