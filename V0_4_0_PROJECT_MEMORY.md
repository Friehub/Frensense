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

---

## Appendix A — CSA Expansion: `AtomicSection` Constraint

### Problem

The sleeping barber race condition shows a pattern that no existing CSA constraint catches:

```c
void *customer(void *arg) {
    if (waiting == chairs) return NULL;    // READ outside mutex ← BUG
    pthread_mutex_lock(&mutex);             // lock
    waiting++;                               // WRITE inside mutex
    pthread_cond_signal(&customer_ready);
    pthread_cond_wait(&barber_ready, &mutex);
    pthread_mutex_unlock(&mutex);
    return NULL;
}
```

The `if (waiting == chairs)` check reads a mutex-protected variable **outside the mutex**. Between the read and the lock, the barber can change `waiting`. This allows two customers to both pass the `waiting == chairs` gate, both increment past capacity, and deadlock waiting on `barber_ready`.

**The invariant:** `waiting` is protected by `mutex` — every write (`waiting++`, `waiting--`) happens inside a critical section. But the read in `if (waiting == chairs)` violates the invariant. Any code path that reads a mutex-protected variable without holding the mutex is a potential TOCTOU race.

### Proposed Constraint: `AtomicSection`

A new `ProjectFlowConstraint` variant that declares "these operations must execute atomically (under the same lock)":

```yaml
constraints:
  - AtomicSection:
      name: "capacity_check"
      shared_variable: "waiting"
      guard_mutex: "mutex"
      description: >
        The capacity check and increment must be atomic.
        Reading 'waiting' outside the mutex allows a TOCTOU race
        where two threads both pass the capacity gate.
```

In YAML rules:

```yaml
rules:
  - id: "RACE_CAPACITY_CHECK"
    domain: "concurrency"
    target_ext: "c"
    constraints:
      - AtomicSection:
          shared_variable: "{expr}"
          guard_mutex: "{expr}"
```

### Implementation

#### Phase 1: Lock-Set Construction (AST pass)

Build a map of `{variable → set of mutexes that protect it}`:

```rust
struct LockSet {
    // For each mutex-guarded variable, which mutex(es) protect it
    protected_by: HashMap<VarId, HashSet<MutexId>>,
    // All lock/unlock sites
    mutex_ops: Vec<MutexOp>,
}

struct MutexOp {
    kind: LockOp,       // Lock | Unlock
    mutex: MutexId,
    location: SourceLoc,
    enclosing_function: FunctionId,
}
```

Walk the AST and collect:
- All `pthread_mutex_lock(&m)` / `pthread_mutex_unlock(&m)` calls
- All `pthread_cond_wait(&cond, &m)` calls (which atomically unlock m and wait)
- All reads and writes of shared variables

Infer protection: if every write to variable `v` is inside a `lock(m)`...`unlock(m)` span, then `v` is protected by `m`.

#### Phase 2: Read-Outside-Lock Detection

For every read of a protected variable, check if it occurs within the lock/unlock span of its protecting mutex:

```rust
fn check_atomic_section(engine: &Engine, constraint: &AtomicSection) -> Vec<Advisory> {
    let var = &constraint.shared_variable;
    let mutex = &constraint.guard_mutex;

    engine.source_registry
        .functions()
        .flat_map(|func| {
            let reads = func.reads_of(var);
            let lock_held = |loc| func.is_inside_mutex_span(loc, mutex);
            reads
                .filter(|r| !lock_held(r.location))
                .map(|r| advisory_for_read_outside_lock(r, var, mutex))
        })
        .collect()
}
```

#### Phase 3: Condition Variable Pairing

For `pthread_cond_signal`/`pthread_cond_wait` pairs, verify:

1. The signal is always called **while holding** the associated mutex
2. The wait is always called **while holding** the associated mutex (by POSIX spec)
3. The predicate (`waiting > 0` for barber, `waiting < chairs` for customer) is checked **after** reacquiring the mutex on wake

This catches "lost wakeup" patterns where `signal()` is called when nobody is waiting on the condition variable — not a direct CSA constraint, but a temporal invariant that the condition variable graph can verify.

### Advisory Output

```
Rule: RACE_CAPACITY_CHECK
Severity: Critical
File: sleeping_barber.c:8
Observation: Read of 'waiting' outside mutex 'mutex'
  Variable 'waiting' is protected by 'mutex' — every write (lines 11, 28)
  occurs inside pthread_mutex_lock/unlock. But line 8 reads it without
  holding the mutex, allowing a TOCTOU race.
Impact: Two threads can both pass the capacity check, increment past
  capacity, and deadlock waiting on a condition signal.
Improvement: Move the capacity check inside the mutex:
    pthread_mutex_lock(&mutex);
    if (waiting == chairs) { pthread_mutex_unlock(&mutex); return NULL; }
    waiting++;
    ...rest of critical section...
```

### Effort

| Step | Time |
|------|------|
| Lock-set construction AST pass (Phase 1) | 3h |
| Read-outside-lock detection (Phase 2) | 2h |
| Condition variable pairing verification (Phase 3) | 2h |
| YAML constraint DSL + parsing | 1h |
| C target support (tree-sitter-c grammar) | 2h |
| Integration tests (sleeping barber fixture) | 1h |
| Benchmark (lock-set on rustc-sized codebase) | 30m |

**Total:** ~11.5 hours

### Relation to Style Profile

The `AtomicSection` constraint is **complementary** to the style-anomaly profile:

- **Style profile** catches *"this doesn't look like our code"* — unwritten conventions, LLM tells
- **AtomicSection** catches *"this is demonstrably wrong"* — race conditions, TOCTOU, lost wakeups

Both are part of v0.4.0. The style profile is the headline feature; `AtomicSection` is the depth feature that catches real bugs the style profile would miss.

---

## Future Work (v0.4.1+)

- **Temporal decay:** Old functions contribute less to the profile (project conventions evolve)
- **Multi-repo profiles:** Cross-project style comparison ("does this code look like it belongs to this org?")
- **Profile diff:** When reviewing a PR, diff the PR branch's profile vs main to surface what the PR changes stylistically
- **Auto-remediation:** Suggest replacements for off-style patterns based on the profile (e.g., `any` → the most common type at that position)
- **Model checking (v0.5.x):** Full state-space exploration for small critical sections — enumerate reachable states of {shared vars, locks, condition queues} and prove absence of deadlock.
