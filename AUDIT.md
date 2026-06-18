# Frensense — Production Audit

> Written after a full read of: `src/lib.rs`, `src/temporal/`, `src/engine/`,
> `src/engine/auditor/`, `src/engine/taint_rules.rs`, `Cargo.toml`, `taint_rules.toml`,
> and the `corpus/` directory structure.
> All findings reference exact file and line. All fixes are scoped and actionable.

---

## Phase 1: Critical Architectural Gap — Temporal Cannot Use Corpus

### Issue 1.1: `TemporalConfig` Is Hardwired to YAML, Not Corpus

**File:** `src/temporal/config.rs` — full file
**File:** `src/temporal/handler.rs` — line 14 comment

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct TemporalConfig {
    pub sequence: Vec<String>,
    pub behavior: String,
}
```

The comment on `handler.rs` line 14 reads: "Compile a `TemporalConfig` (from YAML)". The struct derives `Deserialize` with no format constraint, but the field names `sequence` and `behavior` match the old YAML rule format. When the project migrated to corpus (`.frc` bundle), all rule definitions moved to the corpus format. Temporal rules are the only rule type that still expects the old serialization path — there is no corpus loader that reads temporal sequence/behavior definitions from the `.frc` bundle and constructs `TemporalConfig` structs.

The result: temporal rules that were defined as YAML are silently dropped after the corpus migration. No temporal checks fire. No error is raised. The `temporal` feature flag is still active in `Cargo.toml` line 65, but its rules have no data source.

**Root Cause:**
The corpus bundle (`frensense-corpus.frc`) stores rule definitions. The taint rules load from `taint_rules.toml` via `include_str!` at compile time (`taint_rules.rs` line 88). Temporal rules have no equivalent loading path — neither from the corpus bundle nor from a compiled-in TOML.

**Fix (3 steps):**

Step 1: Define a `TemporalRuleToml` struct parallel to `TaintRuleToml`:

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TemporalRuleToml {
    pub id: String,
    pub sequence: Vec<String>,
    pub behavior: String,    // "must_not_follow" | "must_follow" | "forbidden_between"
    pub severity: String,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
```

Step 2: Create `temporal_rules.toml` at the repo root (alongside `taint_rules.toml`) with the existing temporal rule definitions encoded in TOML. Add `include_str!("../../temporal_rules.toml")` in a new `static BUILTIN_TEMPORAL_RULES` using `LazyLock`, mirroring `taint_rules.rs` exactly.

Step 3: Add a `load_all_temporal_rules(extra_dirs: &[PathBuf]) -> Vec<CompiledTemporalRule>` function that compiles the TOML entries into `(Vec<regex::Regex>, TemporalBehavior)` pairs. The `compile_temporal_config` function in `handler.rs` already does this compilation — call it per entry.

This approach is zero new dependencies and zero breaking changes. The temporal feature becomes corpus-adjacent: TOML-driven, compile-time embedded, with optional override from extra dirs (same pattern as taint rules).

---

### Issue 1.2: `is_rule_enabled` in `FrensenseAuditor` Always Returns `true`

**File:** `src/engine/auditor/mod.rs` — lines 146–157

```rust
#[allow(clippy::unused_self, clippy::too_many_arguments)]
fn is_rule_enabled(...) -> bool {
    true
}
```

The category filter, tag filter, suite, environment, and severity filter parameters are all prefixed with `_` and completely ignored. The `allow(clippy::unused_self)` suppression confirms this is intentional dead code. The CLI exposes `--category`, `--tag`, `--suite`, `--env`, and `--severity` flags that populate these filter sets, but none of them have any effect on rule execution.

This is not a crash bug, but it means all filtering advertised in the CLI is silently broken. A user running `frensense scan --category security` gets the same results as running with no filter.

**Fix:** Implement the filtering logic. The rule metadata has `category`, `tags`, `precision` (maps to suite), and `severity`. The implementation is:

```rust
fn is_rule_enabled(&self, rule: &dyn FrensenseRule, category_filter: &HashSet<String>,
    tag_filter: &HashSet<String>, suite: Suite, env: FrensenseEnvironment,
    severity_filter: Option<Severity>) -> bool {
    let meta = rule.metadata();
    if !category_filter.is_empty() && !category_filter.contains(meta.category.as_ref()) {
        return false;
    }
    if !tag_filter.is_empty() && !meta.tags.iter().any(|t| tag_filter.contains(t.as_ref())) {
        return false;
    }
    if !meta.meets_suite(suite) { return false; }
    if let Some(threshold) = severity_filter {
        if !meta.severity.meets_threshold(threshold) { return false; }
    }
    true
}
```

The `meets_suite` and `meets_threshold` methods already exist on `RuleMetadata` and `Severity`. This is approximately 15 lines of code to make all filtering functional.

---

## Phase 2: Taint Analysis Gaps

### Issue 2.1: `COMBINED_SOURCE_RE` and `COMBINED_SINK_RE` Only Cover Built-in Rules

**File:** `src/engine/taint_rules.rs` — lines 110–119

```rust
pub static COMBINED_SOURCE_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    let patterns: Vec<&str> = BUILTIN_RULES.iter().map(|r| r.source_re.as_str()).collect();
    regex::Regex::new(&patterns.join("|")).expect("valid combined source regex")
});
```

The combined regexes are built at program startup from `BUILTIN_RULES` only. Rules loaded via `--extra-taint-rules <dir>` are added to the runtime `Vec<TaintRule>` returned by `load_all_taint_rules`, but they are never incorporated into `COMBINED_SOURCE_RE` or `COMBINED_SINK_RE`. Any taint analysis pipeline that uses these static regexes for pre-filtering will miss sources and sinks defined in user-supplied rules.

**Fix:** `COMBINED_SOURCE_RE` and `COMBINED_SINK_RE` must not be static statics. They need to be built from the complete rule set at analysis time. Either:

Option A: Pass the full rule list to a `build_combined_regexes(rules: &[TaintRule])` function that returns `(Regex, Regex)`. Build once per scan session after `load_all_taint_rules` returns.

Option B: Expose a `TaintRuleRegistry` struct that owns the full rule list and the two compiled regexes, built together. The scanner creates one registry per run.

Option A is smaller. Option B is cleaner for future extension.

---

### Issue 2.2: Taint Rules Have No `language` Field — All Rules Apply to All Languages

**File:** `src/engine/taint_rules.toml` — all entries
**File:** `src/engine/taint_rules.rs` — `TaintRuleToml` struct

The `TaintRuleToml` struct and the TOML schema have no `language` field. Every taint rule fires on every language. `TAINT_CREDENTIAL_TO_DB` matches the sink pattern `insert|update|create|upsert|db\.|database\.|query|execute` regardless of whether the file is Rust, TypeScript, Python, or Solidity. This causes false positives: in Python, `list.update()` triggers `TAINT_CREDENTIAL_TO_DB` if a variable named `token` is in scope nearby.

**Fix:** Add an optional `language` field to `TaintRuleToml`:

```toml
[[rules]]
id = "TAINT_CREDENTIAL_TO_DB"
language = ["typescript", "javascript", "python", "rust"]  # or omit for all
source = "password|secret|token|credential|key|api_key"
sink = "insert|update|..."
```

In `TaintRuleToml`:
```rust
#[serde(default)]
pub languages: Vec<String>,  // empty = all languages
```

In the taint runner, check `rule.languages.is_empty() || rule.languages.contains(&current_language)` before applying the rule.

---

### Issue 2.3: Missing Taint Rules for Common Sink Patterns

**File:** `taint_rules.toml`

The six built-in rules cover important flows but are missing several high-frequency attack surfaces:

| Missing Rule | Description |
|---|---|
| `TAINT_INPUT_TO_SQL` | Raw string interpolation into SQL — separate from DB sink |
| `TAINT_INPUT_TO_DESERIALIZE` | Untrusted input to `serde_json::from_str`, `JSON.parse`, `pickle.loads` |
| `TAINT_INPUT_TO_REGEX` | Untrusted input as a regex pattern (ReDoS) |
| `TAINT_PII_TO_RESPONSE` | Email, phone, SSN in HTTP response body |
| `TAINT_INPUT_TO_ENV` | Untrusted input used in `std::env::set_var` or `process.env` assignment |

None of these require new infrastructure — they are pure TOML entries. The source/sink regex patterns follow the exact same format as existing rules.

**What to add to `taint_rules.toml`:**

```toml
[[rules]]
id = "TAINT_INPUT_TO_SQL"
source = "input|body|param|query|request|user|header|cookie"
sink = "raw_query|execute_raw|format_sql|interpolate|sql_query|raw\\(|format!.*SELECT|format!.*INSERT|format!.*UPDATE|format!.*DELETE"
severity = "critical"
observation = "Untrusted input may be interpolated into a raw SQL string."
impact = "SQL injection allows data exfiltration and arbitrary database modification."
improvement = "Use parameterized queries or an ORM. Never interpolate user input into SQL strings."

[[rules]]
id = "TAINT_INPUT_TO_DESERIALIZE"
source = "input|body|param|request|user|header|cookie"
sink = "from_str|from_slice|loads|parse|deserialize|JSON\\.parse|json\\.loads"
severity = "warning"
observation = "Untrusted input passed to a deserialization function."
impact = "Malicious payloads can trigger deserialization vulnerabilities or panic the process."
improvement = "Validate input schema before deserializing. Use size limits and type assertions."

[[rules]]
id = "TAINT_INPUT_TO_REGEX"
source = "input|body|param|query|user"
sink = "Regex::new|new RegExp|re\\.compile|regex\\.compile"
severity = "warning"
observation = "Untrusted input used as a regex pattern — potential ReDoS."
impact = "A crafted input can cause catastrophic backtracking and hang the process."
improvement = "Sanitize or escape input before using as a regex. Use a regex timeout or bounded engine."
```

---

## Phase 3: Corpus Architecture — Accurate State After Full Code Read

### What Is Actually Built

**`frensense-engine/src/corpus/` — fully implemented, not empty.**

- `bundle.rs` — FRC1 binary format. `BundleHeader` (magic `b"FRC1"`, version, blake3 checksum),
  `BundlePattern` (`id`, `positives`, `negatives: Vec<FunctionFingerprint>`), `build_bundle()`,
  `load_bundle()` with version rejection and checksum verification. Tested with roundtrip, version
  check, and invalid magic tests.
- `loader.rs` — `load_corpus(dir)` reads `corpus/targets/`, identifies `_positive.*` / `_negative.*`
  pairs, parses with tree-sitter, calls `extract_fingerprints`, returns `Vec<CorpusPattern>`.
  Supports `.rs`, `.ts`, `.tsx`, `.js`, `.jsx`. Python `.py` is silently skipped.
- `registry.rs` — `PatternRegistry` with `load_from_bundle()` and threshold-based `scan_function()`.

**`runner.rs` lines 532–613** — corpus is wired in `run_detailed`. Tries the embedded
`frensense-corpus.frc` bundle first (via `self.corpus_bundle`), falls back to `--corpus <dir>`
source files. Findings emitted as `CORPUS_{PATTERN_ID}` advisories with match score as confidence.

**The FRC1 corpus pipeline is production-ready and working.** It is not a gap.

---

### Issue 3.1: Temporal Rules Are Hardcoded Rust Structs — No TOML Exists

**File:** `frensense-engine/src/temporal.rs` — `add_default_rules()` lines 91–141
**File:** `src/engine/findings/temporal_violation.rs` — lines 3–5

The engine-level `TemporalAnalyzer::add_default_rules()` defines all five temporal rules as
inline Rust struct literals (lock/unlock, acquire/release, open/close, connect/disconnect,
lock+sleep). There is no `temporal_rules.toml` anywhere in the repository. The only user-facing
TOML for rules is `taint_rules.toml`.

`temporal_violation::find()` calls `add_default_rules()` directly, bypassing the `src/temporal/`
consumer layer (`TemporalConfig`, `compile_temporal_config`, `check_temporal`) entirely.
The consumer layer is dead code from the perspective of the running engine. Adding a temporal
rule requires editing Rust source and recompiling. There is no external data path.

**Fix:** Create `temporal_rules.toml` at the repo root:

```toml
[[rules]]
id = "LOCK_UNLOCK"
before = "lock"
after = "unlock"
description = "Every lock() must be followed by unlock()"
severity = "error"

[[rules]]
id = "ACQUIRE_RELEASE"
before = "acquire"
after = "release"
description = "Every acquire() must be followed by release()"
severity = "error"

[[rules]]
id = "OPEN_CLOSE"
before = "open"
after = "close"
description = "Every open() must be followed by close()"
severity = "warning"

[[rules]]
id = "CONNECT_DISCONNECT"
before = "connect"
after = "disconnect"
description = "Every connect() must be followed by disconnect()"
severity = "warning"

[[rules]]
id = "RUST_LOCK_SLEEP"
before = "lock"
after = "sleep"
description = "Holding a lock while sleeping may cause deadlock"
severity = "error"
```

Add a `TemporalRuleToml` struct to `frensense-engine/src/temporal.rs`, a
`BUILTIN_TEMPORAL: LazyLock<Vec<TemporalRule>>` using `include_str!("../../temporal_rules.toml")`,
and call the loaded rules in `temporal_violation::find()` alongside or replacing `add_default_rules()`.
Wire `--extra-temporal-rules <dir>` CLI flag for user-supplied temporal rules, same pattern as
`--extra-taint-rules`.

---

### Issue 3.2: `FrensenseAuditor::default_rules()` Returns Empty Vec

**File:** `src/engine/auditor/rules.rs` — lines 8–10

```rust
pub fn default_rules() -> Vec<Box<dyn FrensenseRule>> {
    Vec::new()
}
```

**Status: By-design.** The `FrensenseRule` trait system is not used for detection. All findings
come from module-level `find()` functions called directly by the runner:
- `temporal_violation::find` (W1)
- `dead_branch::find` (W2)
- `unused_variable::find` (W3)
- `cross_file_taint::find` (W4)
- `hallucinated_import::find` (W7)
- corpus fingerprint matching (L1)
- taint rules (L2)
- secret scanning

The `FrensenseRule` trait, `is_rule_enabled` filter, and `AuditOptions` struct exist for
future extensibility but are not currently wired. The `is_rule_enabled` filter has been
fixed (Issue 1.2 above) so it's ready when rules are registered. No action needed now.

---

### Issue 3.3: Corpus Loader Silently Drops Python Files

**File:** `frensense-engine/src/corpus/loader.rs` — lines 38–43

**Status: Fixed.** Added `eprintln!` warning for unsupported extensions. Contributors adding
`.py` files now see `corpus: skipping unsupported extension 'py' in '...'`. Python tree-sitter
support is behind the `python` feature flag — the warning directs users to enable it.

---

### Issue 3.4: `config.rules_dir` Field Is Loaded but Never Read

**File:** `src/engine/project/config.rs` — `rules_dir: Option<String>`
**File:** `src/engine/project/runner.rs` — `initialize_auditor_and_config()`

**Status: Fixed.** `config.rules_dir` is now wired to `extra_taint_rule_dirs` in
`initialize_auditor_and_config`. Users who set `rules_dir: "./my-rules"` in
`.frensense/config.yml` get their taint rules loaded automatically.

---

### Issue 3.5: Mixed Config Formats — YAML Config, YAML Suppress, TOML Rules

**File:** `src/engine/project/runner.rs` — line 773, `.frensense-suppress.yml`
**File:** `src/engine/project/config.rs` — `.frensense/config.yml`
**File:** `taint_rules.toml`

Three configuration surfaces, two formats. Low priority but worth tracking for the contributor
guide. Do not change formats now — the migration cost outweighs the benefit.

---


## Phase 4: What to Build Next — Ordered Fix List

This is the agent's build order. Do not deviate without auditor review.

| Priority | Item | File(s) | Est. Lines | Blocks | Status |
|---|---|---|---|---|---|
| 1 | Create `temporal_rules.toml` with existing rules | repo root | 40 | Temporal running | **Done** (W1) |
| 2 | Add `load_all_temporal_rules()` function | `src/temporal/handler.rs` or new `src/temporal/loader.rs` | 60 | Temporal corpus path | **Done** (W1) |
| 3 | Implement `is_rule_enabled` properly | `src/engine/auditor/mod.rs` | 15 | All CLI filters | **Done** |
| 4 | Add `language` field to `TaintRuleToml` | `src/engine/taint_rules.rs` + `taint_rules.toml` | 10 | False positive reduction | **Done** |
| 5 | Fix `COMBINED_SOURCE_RE` / `COMBINED_SINK_RE` to include user rules | `src/engine/taint_rules.rs` | 20 | `--extra-taint-rules` correctness | **Done** |
| 6 | Add 3 missing taint rules to `taint_rules.toml` | `taint_rules.toml` | 30 lines TOML | Coverage | **Done** |
| 7 | Create `src/engine/corpus/mod.rs` with `CorpusBundle` struct | new file | 60 | Corpus integration | **Done** (S2-S4) |
| 8 | Add `corpus: Option<&CorpusBundle>` to `FrensenseContext` | `src/lib.rs` | 2 | Corpus API | **Done** (S3) |
| 9 | Add corpus target files (see list above) | `corpus/targets/` | N/A | Ground truth | **Done** (89 patterns) |
| 10 | Add `corpus/ground_truth/labels.json` | new file | 30 lines JSON | Precision benchmarks | **Done** (F8) |
| 11 | Wire `config.rules_dir` to extra taint rule dirs | `src/engine/project/runner.rs` | 4 | Config completeness | **Done** |
| 12 | Add warning for unsupported corpus extensions | `frensense-engine/src/corpus/loader.rs` | 3 | Corpus UX | **Done** |

### Scope Ceiling — Do Not Build

- **Do not add a new language parser** before temporal rules are fixed. The temporal/corpus gap affects Rust and TypeScript — the two languages you already support. Expanding to Java or Go before the core analysis pipeline is correct makes the problem worse.

- **Do not replace `taint_rules.toml` with a corpus format.** The TOML format for taint and temporal rules is correct and maintainable. The corpus bundle is for pattern fingerprints and ground truth examples, not for rule metadata. Mixing these formats creates ambiguity.

- **Do not add an AI/LLM layer to Frensense.** The engine is a static analysis tool. Its value is deterministic, high-precision, zero-network-call output. Any AI augmentation belongs in a separate reporting layer, not in the engine itself.

- **Do not implement a web UI.** The MCP server (`src/bin/frensense-mcp.rs`) plus CLI is the correct interface surface. A web UI is a separate product.

---

## Phase 5: Option B — Proper Interprocedural Taint With Flow Edges

> This is the agent's primary engineering mandate after completing the fixes in Phase 4.
> All work in this phase builds on existing infrastructure. No new crates are required.

### What Already Exists

After reading all of `src/semantics/data_flow/`, the situation is better than it appears from
the 0% precision report. The interprocedural engine is substantially implemented:

- `DataFlowAnalyzer` (`data_flow/mod.rs`) — exists, has `depth`, `max_depth`, `visited`,
  `alias_tracker`, and `sanitize_re` fields.
- `resolve_taint` (`resolve.rs` lines 186–261) — walks the AST and resolves taint through
  identifiers, member expressions, and call chains.
- `analyze_call` (`resolve.rs` lines 11–124) — follows tainted arguments across function
  boundaries up to `max_depth`, creates a new `FrensenseContext` per callee, recurses.
- `resolve_call_taint` (`resolve.rs` lines 264–428) — handles method chains, sanitizer
  short-circuit, callee return value taint propagation.
- `SemanticOp` — `Binding`, `Assignment`, `Call`, `EnterBlock` cover the statement types.
- `EdgeKind::TaintFlow` — exists on `SemanticGraph`, `record_taint_flow` exists.
- `TaintCache` — LRU cache for taint results, capacity 1024.

**The problem is not the interprocedural engine. The problem is where taint originates.**

In `resolve.rs` line 29–30, the source check reads:
```rust
if source_re.is_match(arg_text) {
    registry.taint(arg_text, TaintOrigin::UserInput);
}
```

`arg_text` is the raw identifier string at a call site. So any argument named `request`,
`body`, `input`, `header` is immediately tainted — before any flow analysis — regardless of
whether that variable came from user input or is a local framework struct. This is where the
585 false positives come from. The Axum source code passes variables named `request` (which
are `axum::http::Request<B>` internal types) to HTTP method functions like `get`, `post`,
`put` (routing registrations). Both the identifier name and the function name match the
regex, so every routing call in Axum fires as a taint hit.

The fix is to replace regex-on-identifier-text source seeding with **typed entry-point
seeding**. Everything downstream of that (the flow engine, the interprocedural recursion,
alias tracking, sanitizer short-circuit) is already correct.

---

### Step 1: Define `TaintEntryPoint` — The Source of Truth for Taint Origins

**New file:** `src/engine/taint_entry_points.rs`

A `TaintEntryPoint` defines where taint actually enters a program — at a specific function
parameter position with a specific type annotation, not at any identifier matching a regex.

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TaintEntryPoint {
    /// The fully-qualified function name or pattern (regex).
    /// For Axum handlers: any function referenced by a router `.route()` call.
    pub function_pattern: String,

    /// The parameter index (0-based) that carries user-controlled data.
    /// `None` = all parameters are tainted entry points.
    pub param_index: Option<usize>,

    /// Optional type annotation filter. If set, only taint parameters
    /// whose declared type matches this pattern.
    /// e.g. "Json|Query|Form|Path|Extension|axum::extract"
    pub type_pattern: Option<String>,

    /// The rule IDs this entry point applies to.
    /// Empty = applies to all rules.
    pub rule_ids: Vec<String>,

    /// Language this entry point applies to.
    pub language: String,
}
```

Ship built-in entry points for the three supported languages in a new
`taint_entry_points.toml` at the repo root:

```toml
# Rust / Axum
[[entry_points]]
language = "rust"
function_pattern = ".*"              # any handler
type_pattern = "Json|Query|Form|Path|Extension|Multipart|Bytes"
param_index = null                   # any param with matching type
rule_ids = []                        # apply to all rules

# TypeScript / Express
[[entry_points]]
language = "typescript"
function_pattern = ".*"
type_pattern = "Request|IncomingMessage"
param_index = 0
rule_ids = []

# TypeScript / Fastify
[[entry_points]]
language = "typescript"
function_pattern = ".*"
type_pattern = "FastifyRequest"
param_index = 0
rule_ids = []

# Python / FastAPI
[[entry_points]]
language = "python"
function_pattern = ".*"
type_pattern = "Body|Query|Path|Form|File|Request"
param_index = null
rule_ids = []
```

Embed with `include_str!("../../taint_entry_points.toml")` in the same pattern as
`taint_rules.rs`.

---

### Step 2: AST-Based Entry Point Detection

**New file:** `src/engine/taint_seeder.rs`

The `TaintSeeder` replaces the regex-on-identifier source check. Its job is to walk a
function's parameter list in the AST and seed the `TaintRegistry` with parameters that match
a `TaintEntryPoint`.

```rust
pub struct TaintSeeder<'a> {
    entry_points: &'a [TaintEntryPoint],
    language: &'a str,
}

impl<'a> TaintSeeder<'a> {
    /// Walk the function_item/function_declaration node's `parameters` child.
    /// For each parameter that matches an entry point, call registry.taint(name).
    pub fn seed_from_function_params(
        &self,
        fn_node: tree_sitter::Node<'_>,
        source: &str,
        registry: &mut TaintRegistry,
    ) {
        let params_node = fn_node.child_by_field_name("parameters").unwrap_or(fn_node);
        let mut cursor = params_node.walk();
        for (idx, param) in params_node.children(&mut cursor)
            .filter(|c| !matches!(c.kind(), "(" | ")" | "," | "self"))
            .enumerate()
        {
            let (param_name, param_type) = extract_param_name_and_type(param, source, self.language);
            for ep in self.entry_points {
                if ep.language != self.language { continue; }
                if let Some(pi) = ep.param_index {
                    if pi != idx { continue; }
                }
                if let Some(ref tp) = ep.type_pattern {
                    let type_re = regex::Regex::new(tp).unwrap();
                    if !type_re.is_match(&param_type) { continue; }
                }
                registry.taint(&param_name, TaintOrigin::UserInput);
                break;
            }
        }
    }
}
```

`extract_param_name_and_type` is language-specific:
- **Rust:** In a `parameter` node, the pattern child is the name, the `type` child is the
  type annotation. For `Json<CreateUser>`, the type text includes `Json`.
- **TypeScript:** In a `required_parameter`, `name` field is the name, `type` annotation
  child (if present) gives the type. Axum-equivalent is Express `Request`.
- **Python:** In a `typed_parameter`, `name` is the name, `type` is the annotation after `:`.

For Rust specifically, also detect axum extractor parameters via the `axum::extract` import
in scope — if the function file imports `axum::extract::Json`, any `Json<T>` parameter is
a taint source regardless of the type pattern string comparison.

---

### Step 3: Replace Regex Source Seeding with `TaintSeeder`

**File:** `src/semantics/data_flow/resolve.rs` — lines 27–31
**File:** `src/semantics/data_flow/tracking.rs` — `analyze_block`

Remove:
```rust
if source_re.is_match(arg_text) {
    registry.taint(arg_text, TaintOrigin::UserInput);
}
```

Replace with: at the beginning of `analyze_block`, before iterating ops, call:
```rust
seeder.seed_from_function_params(fn_node, source, registry);
```

Where `fn_node` is the enclosing function item. The `DataFlowAnalyzer` needs to know its
enclosing function node — add `fn_node: Option<Node<'a>>` to the struct. Set it when
`with_depth` creates a sub-analyzer using the `def_node` from `find_definition`.

The `source_re` parameter in `analyze_call`, `resolve_taint`, `analyze_block` is kept but its
role changes: it is no longer used to taint new identifiers at call sites. It is used only to
identify **return-taint from known source functions** — i.e. functions whose return value is
always tainted regardless of parameters (e.g. `read_to_string`, `body.bytes()`,
`req.body()`). These are a short, curated list, not a broad keyword regex.

---

### Step 4: Curate Source-Function Allowlist

The existing `source` regex in `taint_rules.toml` is what causes the 520 false positives for
`TAINT_INPUT_TO_HTTP`. Replace it with a **named source function list** instead of a keyword
regex. Add a new field `source_functions` to `TaintRuleToml`:

```toml
[[rules]]
id = "TAINT_INPUT_TO_HTTP"
language = ["rust", "typescript", "python"]
source_functions = [
  # Rust/Axum body extractors (return value is user-controlled)
  "body", "bytes", "text", "json", "form", "multipart",
  # Express
  "req.body", "req.query", "req.params", "req.headers",
  # Python
  "request.body", "request.json", "request.form", "request.args",
]
sink = "fetch|reqwest|ureq|http::Client|HttpClient|axios|aiohttp"
severity = "warning"
observation = "User-controlled data may reach an outbound HTTP request (SSRF)."
impact = "SSRF allows the server to make requests to internal services or exfiltrate data."
improvement = "Validate URLs against an allowlist. Block RFC-1918 address ranges."
```

The `source_functions` list is specific enough that `list.update()` in Python will not match.
The regex `source` field stays for backward compatibility but is demoted: it only fires when
no `source_functions` list is present on the rule.

---

### Step 5: Integrate Sanitizer Recognition

The `DataFlowAnalyzer` already has `sanitize_re: Option<Regex>`. It is already checked in
`resolve_call_taint` at line 300–304 — if the function name matches, taint is cleared.

What is missing is a built-in sanitizer list. Add `sanitizers.toml` at the repo root:

```toml
[[sanitizers]]
language = "rust"
functions = [
  "validate", "sanitize", "escape", "encode", "hash",
  "bcrypt", "argon2", "hmac", "sha256",
  "from_str",     # typed parse clears taint — user-controlled string becomes typed value
  "parse",        # same
]

[[sanitizers]]
language = "typescript"
functions = [
  "escape", "sanitize", "encode", "validate", "parse",
  "z.parse", "zod.parse", "joi.validate",
]

[[sanitizers]]
language = "python"
functions = [
  "escape", "sanitize", "validate", "parse", "model_validate",
  "BaseModel.parse_obj",
]
```

Build the sanitizer regex from this list at analysis startup (same `LazyLock` pattern as
`COMBINED_SOURCE_RE`). Pass it to `DataFlowAnalyzer::with_sanitizers`.

---

### Step 6: Axum-Specific Handler Detection

Axum registers handlers via `.route("/path", get(handler).post(other_handler))`. The handler
function is passed as a function reference — it is not called directly. The `TaintSeeder`
needs to know which functions are handler entry points so it seeds their parameters.

This requires a project-level rule (`ProjectRule` trait, already exists) that:
1. Scans all files for `Router::new().route(...)` or `.get(fn_ref)` patterns.
2. Collects function names passed as routing arguments.
3. Marks those functions in the `SymbolRegistry` with a `handler_entry_point: true` flag.

The `DataFlowAnalyzer` checks this flag before seeding: if the function is a known handler
entry point, seed all parameters that match the `TaintEntryPoint` type pattern.

For Express/Fastify, the same detection reads `app.get("/path", handler)` calls. For Python
FastAPI, it reads `@app.get("/path")` decorators.

---

### Step 7: Expected Precision After These Changes

Based on the Axum false positive breakdown:

| Rule | Current FP | Expected After Fix | Reason |
|---|---|---|---|
| `TAINT_INPUT_TO_HTTP` | 520 | ~5–10 | Source seeding becomes typed, not regex on names |
| `TAINT_INPUT_TO_FS` | 28 | ~2–5 | Same — `path` params only tainted from typed extractors |
| `TAINT_CREDENTIAL_TO_HTTP` | 20 | ~1–3 | `password`/`token` only tainted from user-typed params |
| `TAINT_CREDENTIAL_TO_DB` | 12 | ~0–2 | Framework-internal DB calls no longer flagged |
| `TAINT_INPUT_TO_EXEC` | 5 | ~0–1 | Exec sinks not present in Axum core |

Total expected FP reduction: from 585 to approximately 10–20 on Axum's own source. That
remaining set should be genuine findings worth human review.

---

### Build Order for This Phase

| Step | File | Description | Est. Lines |
|---|---|---|---|
| 1 | `taint_entry_points.toml` (new) | TOML entry point definitions for Rust/TS/Python | 40 |
| 2 | `src/engine/taint_entry_points.rs` (new) | `TaintEntryPoint` struct + loader + `LazyLock` | 80 |
| 3 | `src/engine/taint_seeder.rs` (new) | `TaintSeeder` with per-language param extractor | 120 |
| 4 | `sanitizers.toml` (new) | Built-in sanitizer function list | 30 |
| 5 | `src/engine/taint_rules.rs` | Add `source_functions` field, demote `source` regex | 20 |
| 6 | `src/semantics/data_flow/mod.rs` | Add `fn_node` and `seeder` to `DataFlowAnalyzer` | 15 |
| 7 | `src/semantics/data_flow/tracking.rs` | Replace regex source seeding with `TaintSeeder` | 10 |
| 8 | `src/semantics/data_flow/resolve.rs` | Demote `source_re` role, remove auto-taint at call sites | 15 |
| 9 | `src/engine/auditor/` new `ProjectRule` | Axum/Express/FastAPI handler detection | 100 |
| 10 | `src/engine/mod.rs` | Wire entry point loader and seeder into engine init | 20 |

**Total: approximately 450 lines of new/modified code.**

Do not implement steps 6–10 before steps 1–5 are complete and the build passes. The seeder
depends on the entry point structs. The auditor modification depends on the seeder. Work
linearly.
