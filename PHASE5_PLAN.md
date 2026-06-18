# Phase 5: Taint Precision — Implementation Plan

> Replace regex-on-identifier source seeding with AST-based typed entry-point detection.
> Target: 585 false positives on Axum → ~10-20.

---

## Problem

In `src/semantics/data_flow/resolve.rs` line 29:
```rust
if source_re.is_match(arg_text) {
    registry.taint(arg_text, TaintOrigin::UserInput);
}
```

Any argument named `request`, `body`, `input`, `header` is immediately tainted — regardless of whether it came from user input. This causes 585 false positives on Axum's own source code.

## Solution

Replace regex-on-identifier seeding with **typed entry-point seeding**: only taint function parameters whose type annotations match known user-input types (e.g., `Json<T>`, `Query<T>`, `Path<T>` in Axum; `req.body`, `req.query` in Express).

---

## Step 1: `taint_entry_points.toml` — Entry Point Definitions

**New file:** `taint_entry_points.toml` (repo root)

```toml
# Rust / Axum — parameters with extractor types are taint sources
[[entry_points]]
language = "rust"
type_pattern = "Json|Query|Form|Path|Extension|Multipart|Bytes"
param_index = null
rule_ids = []

# TypeScript / Express — req.body, req.query, req.params, req.headers
[[entry_points]]
language = "typescript"
type_pattern = "Request|IncomingMessage"
param_index = 0
rule_ids = []

# TypeScript / Fastify
[[entry_points]]
language = "typescript"
type_pattern = "FastifyRequest"
param_index = 0
rule_ids = []

# Python / FastAPI
[[entry_points]]
language = "python"
type_pattern = "Body|Query|Path|Form|File|Request"
param_index = null
rule_ids = []
```

---

## Step 2: `src/engine/taint_entry_points.rs` — Loader

**New file:** ~80 lines

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TaintEntryPoint {
    pub language: String,
    pub type_pattern: Option<String>,
    pub param_index: Option<usize>,
    #[serde(default)]
    pub rule_ids: Vec<String>,
}

static ENTRY_POINTS_FILE: &str = include_str!("../../taint_entry_points.toml");

pub static BUILTIN_ENTRY_POINTS: LazyLock<Vec<TaintEntryPoint>> = LazyLock::new(|| {
    load_entry_points_from_str(ENTRY_POINTS_FILE)
});

pub fn load_entry_points_from_str(content: &str) -> Vec<TaintEntryPoint> { ... }
pub fn load_entry_points(extra_dirs: &[PathBuf]) -> Vec<TaintEntryPoint> { ... }
```

Same pattern as `taint_rules.rs`: embed built-in via `include_str!`, merge with user dirs at runtime.

---

## Step 3: `src/engine/taint_seeder.rs` — AST-Based Seeder

**New file:** ~120 lines

The `TaintSeeder` walks a function's parameter list in the AST and seeds the `TaintRegistry` with parameters that match a `TaintEntryPoint`.

### Core logic

```rust
pub struct TaintSeeder<'a> {
    entry_points: &'a [TaintEntryPoint],
    language: &'a str,
}

impl<'a> TaintSeeder<'a> {
    pub fn seed_from_function_params(
        &self,
        fn_node: tree_sitter::Node<'_>,
        source: &str,
        registry: &mut TaintRegistry,
    ) {
        let params_node = fn_node.child_by_field_name("parameters");
        // Walk children, extract (name, type) per language
        // Match against entry points, taint matching params
    }
}
```

### Language-specific parameter extraction

| Language | AST node kind | Name field | Type field |
|----------|--------------|------------|------------|
| Rust | `parameter` | child(0) pattern text | `type` child text |
| TypeScript | `required_parameter` | `name` field | `type` annotation child |
| Python | `typed_parameter` | `name` child | `type` child after `:` |

### Axum-specific detection

For Rust, also check if the file imports `axum::extract::*`. If it does, any parameter whose type text contains `Json`, `Query`, `Form`, `Path`, `Extension`, `Multipart`, or `Bytes` is a taint source — even without matching the entry point type pattern exactly.

---

## Step 4: `sanitizers.toml` — Built-in Sanitizer List

**New file:** `sanitizers.toml` (repo root)

```toml
[[sanitizers]]
language = "rust"
functions = ["validate", "sanitize", "escape", "encode", "hash", "bcrypt", "argon2", "hmac", "sha256", "from_str", "parse"]

[[sanitizers]]
language = "typescript"
functions = ["escape", "sanitize", "encode", "validate", "parse", "z.parse", "zod.parse", "joi.validate"]

[[sanitizers]]
language = "python"
functions = ["escape", "sanitize", "validate", "parse", "model_validate", "BaseModel.parse_obj"]
```

Build sanitizer regex from this list at startup. Pass to `DataFlowAnalyzer::with_sanitizers()` (already exists).

---

## Step 5: Wire Into `DataFlowAnalyzer`

**File:** `src/semantics/data_flow/mod.rs`

Add field:
```rust
pub(crate) seeder: Option<TaintSeeder<'a>>,
```

Add builder:
```rust
pub fn with_seeder(mut self, seeder: TaintSeeder<'a>) -> Self {
    self.seeder = Some(seeder);
    self
}
```

---

## Step 6: Replace Regex Seeding in `tracking.rs`

**File:** `src/semantics/data_flow/tracking.rs`

At the beginning of `analyze_block`, before iterating ops:
```rust
if let Some(ref seeder) = self.seeder {
    seeder.seed_from_function_params(self.root, self.current_source, registry);
}
```

The existing `source_re` parameter stays but its role changes:
- **Before:** `source_re.is_match(arg_text)` taints any identifier matching the regex
- **After:** `source_re` is only used to identify **return-taint from known source functions** (functions whose return value is always tainted, like `req.body()`, `body.bytes()`)

---

## Step 7: Demote `source_re` in `resolve.rs`

**File:** `src/semantics/data_flow/resolve.rs`

Remove lines 29-31:
```rust
// REMOVE:
if source_re.is_match(arg_text) {
    registry.taint(arg_text, TaintOrigin::UserInput);
}
```

Keep `source_re` parameter for return-taint detection in `resolve_call_taint` only.

---

## Step 8: Wire Entry Points Into Taint Rules

**File:** `src/engine/taint_rules.rs`

Add `source_functions` field to `TaintRuleToml`:
```rust
#[serde(default)]
pub source_functions: Vec<String>,
```

Add to `TaintRule`:
```rust
pub source_functions: Vec<String>,
```

In `run_taint_analysis` (runner.rs), after loading rules, build the seeder:
```rust
let entry_points = taint_entry_points::load_entry_points(&self.extra_taint_rule_dirs);
let seeder = TaintSeeder::new(&entry_points, file_lang);
analyzer = analyzer.with_seeder(seeder);
```

---

## Step 9: Handler Detection (Deferred)

Steps 6-7 in AUDIT.md (Axum handler detection via router pattern matching) are deferred to a follow-up. The typed entry-point seeding (Steps 1-5) already handles the common case: function parameters with extractor types. Handler detection adds the ability to seed taint on functions that are registered as route handlers but don't have extractor types in their signature. This is a smaller win and can come later.

---

## Files Changed

| File | Action | Lines |
|------|--------|-------|
| `taint_entry_points.toml` | **New** | ~30 |
| `sanitizers.toml` | **New** | ~20 |
| `src/engine/taint_entry_points.rs` | **New** | ~80 |
| `src/engine/taint_seeder.rs` | **New** | ~120 |
| `src/semantics/data_flow/mod.rs` | Edit | ~15 |
| `src/semantics/data_flow/tracking.rs` | Edit | ~10 |
| `src/semantics/data_flow/resolve.rs` | Edit | ~5 |
| `src/engine/taint_rules.rs` | Edit | ~15 |
| `src/engine/project/runner.rs` | Edit | ~20 |
| **Total** | | **~315 lines** |

---

## Verification

After implementation:
1. `cargo check` — must compile clean
2. `cargo test -p frensense-engine` — all 69+ tests pass
3. Run on Axum source: `cargo run --release -- . --severity warning --json` — measure FP count
4. Target: <20 taint findings on Axum's own source (down from 585)
5. Run on a real web API with known vulnerabilities — verify TP detection still works

---

## Build Order

```
Step 1: taint_entry_points.toml + taint_entry_points.rs    (TOML + loader)
Step 2: sanitizers.toml                                      (TOML only)
Step 3: taint_seeder.rs                                      (AST walker)
Step 4: Wire seeder into DataFlowAnalyzer                    (mod.rs)
Step 5: Replace regex seeding in tracking.rs                 (remove source_re auto-taint)
Step 6: Demote source_re in resolve.rs                       (remove arg tainting)
Step 7: Add source_functions to TaintRuleToml                (taint_rules.rs)
Step 8: Wire entry points into runner.rs                     (build seeder per file)
Step 9: Test on Axum + real web API
```

Steps 1-3 are independent and can be built in parallel. Steps 4-8 depend on Steps 1-3.
