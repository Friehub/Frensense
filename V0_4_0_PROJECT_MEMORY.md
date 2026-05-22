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

## Appendix B — SRI Diff-Only Baselines

### Problem

Every linter re-scans the entire codebase on every CI run. Teams accumulate hundreds or thousands of pre-existing issues. When a PR adds 3 new issues among 500 pre-existing ones, nobody notices — the signal is buried in noise. Developers learn to ignore the linter entirely.

The standard workaround is to maintain a baseline file (e.g., `.clippy_baseline`) that lists known issues. But baselines are:
- **Line-based** — a single formatting change shifts every line, producing false positives
- **Manual** — someone has to regenerate the baseline after every PR
- **Ignored** — the baseline file grows forever, nobody audits it

### Solution

GenSense already has SRI (Symbol-Relative Identity) — fingerprints anchored to logical symbols, not line numbers. Use SRI to filter advisories to **only symbols that changed** in the current branch vs main.

### Architecture

#### Phase 1: Branch-Aware Symbol Registry

Extend `SymbolRegistry` to track which git branch each symbol belongs to:

```rust
pub struct SymbolEntry {
    pub name: String,
    pub file_path: String,
    pub line: usize,
    pub fingerprint: String,          // SRI: "handler.rs/process_request/L42"
    pub git_blob_oid: Option<String>,  // git hash of the file content at scan time
    pub first_seen_branch: Option<String>,
    pub last_modified_branch: Option<String>,
}
```

When scanning, for each symbol, check `git diff main...HEAD` to determine if the symbol's file + line range was modified. Symbols in unchanged files are excluded from the advisory output.

#### Phase 2: SRI-Anchored Baseline

On `main` branch, run a full scan and serialize the SRI fingerprints of all advisories to `.gensense/baseline.json`:

```json
{
  "version": "0.4.0",
  "generated_at": "2026-06-01T00:00:00Z",
  "baselines": [
    {
      "fingerprint": "sri://order_service.rs/createFromCart/L15",
      "rule_id": "REDUNDANT_BOILERPLATE",
      "severity": "Warning",
      "first_seen": "2026-05-20T00:00:00Z"
    }
  ]
}
```

On feature branches, advisories whose SRI fingerprint matches a baseline entry are **suppressed**. Only **new** advisories (fingerprint not in baseline) are reported.

This is git-aware: if a symbol's file hasn't changed in `main..HEAD`, skip it entirely. If the file changed but the specific symbol's line range didn't, skip it. Only report new advisories in actually-changed code.

#### Phase 3: `--diff-only` CLI Flag

```bash
# Scan only changed symbols vs main
gensense . --diff-only

# Scan only changed symbols vs a specific branch
gensense . --diff-only --diff-base origin/main

# Auto-refresh baseline after merge to main
gensense . --update-baseline
```

#### Phase 4: Baseline Auto-Update CI

```yaml
# After merging to main, regenerate baseline
- name: Update advisory baseline
  run: |
    gensense . --json > .gensense/baseline.json
    if git diff --quiet .gensense/baseline.json; then
      echo "No baseline changes"
    else
      git add .gensense/baseline.json
      git commit -m "chore: update advisory baseline [skip ci]"
      git push
    fi
```

### Why SRI Matters Here

A line-based baseline breaks when:
- A function moves up/down by 5 lines (every line number shifts)
- A file is renamed (every file path changes)
- A comment changes (line count unaffected, but blob hash changes)

SRI is resilient to all of these because the fingerprint is:
```
sri://<file_path>/<symbol_name>/<line>
```

But the key is the **symbol name** + **relative position** within the symbol, not the absolute line number. If `createFromCart` moves from line 15 to line 20, the SRI fingerprint stays the same as long as the symbol name and file path are unchanged.

### Advisory Output

```
Rule: RUST_DEADLOCK_RISK
Severity: Critical
File: src/handler.rs:42
Status: NEW (not in baseline)          ← only possible with git-aware diff
Fingerprint: sri://handler.rs/process_request/L42
```

```
Rule: RUST_DEADLOCK_RISK
Severity: Warning
File: src/legacy.rs:103
Status: BASELINED (present in 1,247 baseline entries)  ← suppressed from output
```

### Effort

| Step | Time |
|------|------|
| Git diff detection for changed symbols | 2h |
| `.gensense/baseline.json` format + serialization | 1h |
| SRI fingerprint matching for suppression | 1h |
| `--diff-only` and `--diff-base` CLI flags | 1h |
| `--update-baseline` CI integration | 30m |
| Tests (git-aware fixtures with branch switching) | 1.5h |
| Benchmark (scan with 10k baseline entries) | 30m |

**Total:** ~7.5 hours

---

## Future Work (v0.4.1+)

- **Temporal decay:** Old functions contribute less to the profile (project conventions evolve)
- **Multi-repo profiles:** Cross-project style comparison ("does this code look like it belongs to this org?")
- **Profile diff:** When reviewing a PR, diff the PR branch's profile vs main to surface what the PR changes stylistically
- **Auto-remediation:** Suggest replacements for off-style patterns based on the profile (e.g., `any` → the most common type at that position)

---

## v0.5.0 Roadmap

### AI Hallucination Detection

**Problem:** LLMs routinely generate code that imports or calls functions that don't exist. The code compiles in the LLM's training distribution but not in the user's project. Examples:

```rust
use nonexistent_crate::AdvancedProcessor;  // crate doesn't exist
let result = process_data(input).unwrap();  // process_data returns ()
import { magicSdk } from '@company/sdk';    // package not in dependencies
```

No existing tool catches this — the compiler catches it too late (after a failed build), and linters don't resolve imports against the actual dependency tree.

**Solution:** Build a **dependency symbol table** by walking:
- `Cargo.toml` + lockfile for Rust crates and their public API surface
- `package.json` + lockfile for npm packages and their exports
- Filesystem for intra-project symbols

For every `use` / `import` / `require` / `use` statement, resolve the imported symbol against the table. If the symbol doesn't exist in any reachable dependency or source file, flag it.

**Edge cases:**
- Dynamic imports (require with variable) — skip, can't resolve statically
- Re-exports through intermediate modules — follow the re-export chain
- Cargo workspace members — check all workspace crates, not just the current one

**Advisory:**
```
Rule: HALLUCINATED_IMPORT
Severity: Critical
File: src/main.rs:1
Observation: Import of 'nonexistent_crate' resolves to nothing.
  Crate 'nonexistent_crate' is not in Cargo.toml dependencies
  (92 crates in lockfile, none match this name).
Impact: Code will fail to build. LLMs often hallucinate crate names
  that look plausible but don't exist.
Improvement: Install the crate or replace with an existing alternative.
  Similar crates in this project: serde, tokio, reqwest.
```

**Effort:** ~6 hours (dependency resolution for Cargo + npm + filesystem)

---

### Secrets with AST Context

**Problem:** Entropy-based secret scanners flag anything random-looking — UUIDs, hashes, base64 encoded data, long hex strings. A codebase with 100 UUIDs gets 100 false positives. Teams disable the scanner.

Real secrets often have **low entropy** (API keys like `sk_live_1234`) and are missed by entropy-only approaches.

**Solution:** Use AST context to determine *how* a string literal is used:

```rust
// High-confidence secret (flagged)
let api_key = "sk_live_abc123def456";
client.auth(api_key);

// Low-confidence secret (suppressed)
let user_id = "550e8400-e29b-41d4-a716-446655440000";
let order = db.find_order(user_id);
```

**Heuristics:**
- Variable name matches `{api,secret,token,auth,password,key}_{key,secret,str}` — flag
- String is passed to a known auth function (`Authorization:` header, `auth()`, `authenticate()`) — flag
- String is assigned to a known credential field (`.password`, `.api_key`, `.secret`) — flag
- String is used in a SQL query, entity ID comparison, or logging — suppress
- String is a well-known UUID/GUID format — suppress

**Advisory:**
```
Rule: LEAKED_CREDENTIAL
Severity: Critical
File: src/config.rs:15
Observation: High-entropy string used as an authentication credential.
  Variable 'api_key' assigned a literal string, passed to 'client.auth()'.
Impact: Hardcoded credentials in source code can be leaked through
  version control, CI logs, or compiled artifacts.
Improvement: Use environment variables or a secrets manager.
```

**Effort:** ~4 hours (AST context matching + variable naming patterns + auth function signatures)

---

### Performance Anti-Patterns

**Problem:** Certain code patterns are correct but unnecessarily slow. No linter catches them because they don't violate type or safety rules.

**Patterns:**

- **N+1 queries in ORMs:** Loop calling `prisma.user.findUnique()` or `User::find()` inside a loop where `findMany` with `include` is appropriate
  - Detect: `for` / `for_each` body containing ORM `findUnique` / `findFirst` with different arguments each iteration
- **Unnecessary `clone()` in Rust:** Data that is only read (never mutated) but is cloned before use
  - Detect: `foo.clone()` where `foo` is not used mutably afterwards and is `Copy` or the lifetime can be borrowed
- **Arc<Mutex<T>> in async code:** Using `std::sync::Mutex` inside `async fn` where `tokio::sync::Mutex` is appropriate for long-held locks
  - Detect: `std::sync::Mutex` lock held across an `.await` point (existing deadlock rule partially covers this)

**Advisory (N+1 example):**
```
Rule: PERF_N_PLUS_ONE_QUERY
Severity: Warning
File: src/users.ts:42
Observation: Database query called inside a loop — potential N+1.
  Line 42: for (const id of userIds) {
  Line 43:   prisma.user.findUnique({ where: { id } })
  Pattern: findUnique inside a for-of loop. Use findMany with 'in' clause.
Improvement: const users = await prisma.user.findMany({
  where: { id: { in: userIds } }
});
```

**Effort:** ~5 hours (loop body analysis + ORM call pattern matching + async mutex detection expansion)

---

### v0.5.0 Effort Summary

| Feature | Time |
|---------|------|
| AI hallucination detection | 6h |
| Secrets with AST context | 4h |
| Performance anti-patterns | 5h |
| Integration tests + benchmarks | 2h |

**Total:** ~17 hours
