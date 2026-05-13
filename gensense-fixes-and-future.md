# GenSense: Current Weaknesses, Fixes & Future Directions

**Branch:** `feature/multi-file-rules` | **Version:** 0.2.0 | **Date:** 2026-05-12

> This document covers three known weaknesses introduced or exposed by the
> `feature/multi-file-rules` branch, their concrete fixes, and the most
> valuable directions for future development. These are fixable bugs, not
> architectural problems — the underlying five-pass pipeline design is sound.

---

## Part 1: Current Weaknesses & Fixes

---

### Weakness 1 — BFS Visited Set Uses Name Instead of (Name, File)

**Affected files:** `src/rules/ir.rs` lines 263–280 (`MustHaveGuard`) and 327–350 (`CrossFileTaintFree`)

**Severity:** Medium — produces silent false negatives and false positives in cross-file rules on any real project.

#### What the code does today

Both `MustHaveGuard` and `CrossFileTaintFree` use a BFS to walk the call graph
outward from a source symbol. To avoid infinite loops the BFS tracks which nodes
it has already visited. The current visited key is the function **name string alone**:

```rust
// MustHaveGuard — src/rules/ir.rs:263-278
let mut visited = std::collections::HashSet::new();
let mut queue = std::collections::VecDeque::new();
queue.push_back(sym);
visited.insert(sym.name.clone()); // Simplification: name as ID for now

while let Some(current) = queue.pop_front() {
    // ...
    for callee in symbols.get_callees(current) {
        if visited.insert(callee.name.clone()) {  // ← bug: name only
            queue.push_back(callee);
        }
    }
}
```

```rust
// CrossFileTaintFree — src/rules/ir.rs:327-346
let mut visited = std::collections::HashSet::new();
queue.push_back(sym);
visited.insert(sym.name.clone());             // ← bug: name only

for callee in symbols.get_callees(current) {
    if visited.insert(callee.name.clone()) {  // ← bug: name only
        queue.push_back(callee);
    }
}
```

#### Why this breaks on real projects

Every real Rust project has multiple functions with the same name in different
files: `new`, `run`, `handle`, `process`, `init`, `parse`, `validate`, etc.
When the BFS visits the first `new` in `src/db.rs`, it inserts `"new"` into the
visited set and then silently skips every other `new` function regardless of
which file it lives in — including the one in `src/auth.rs` that might be the
security guard you are looking for.

This produces two failure modes simultaneously:
- **False negative** — `MustHaveGuard` reports a violation even though the guard
  exists, because the BFS skipped it after deduplicating on name.
- **False positive** — `CrossFileTaintFree` stops traversal early thinking it
  has visited a callee, missing the real taint path through a same-named
  function in another file.

Both failures are **silent** — no error is emitted, the wrong answer is returned
with full confidence.

#### The fix

Key the visited set on `(name, file_path)` — the same composite identity used
everywhere else in the symbol graph lookups (`SemanticGraph::find_node`,
`SymbolRegistry::find_at`, `add_call_edge`).

**`MustHaveGuard` fix:**

```rust
// Before
let mut visited = std::collections::HashSet::new();
visited.insert(sym.name.clone());
// ...
if visited.insert(callee.name.clone()) {

// After
let mut visited = std::collections::HashSet::<(String, String)>::new();
visited.insert((sym.name.clone(), sym.file_path.clone()));
// ...
if visited.insert((callee.name.clone(), callee.file_path.clone())) {
```

**`CrossFileTaintFree` fix** (identical pattern):

```rust
// Before
let mut visited = std::collections::HashSet::new();
visited.insert(sym.name.clone());
// ...
if visited.insert(callee.name.clone()) {

// After
let mut visited = std::collections::HashSet::<(String, String)>::new();
visited.insert((sym.name.clone(), sym.file_path.clone()));
// ...
if visited.insert((callee.name.clone(), callee.file_path.clone())) {
```

No other changes are needed. The `Symbol` struct already carries `file_path` as
a `String` field so this is a two-line change per site.

**Test to add** in `tests/project_rules_tests.rs`:

```rust
#[test]
fn test_bfs_does_not_deduplicate_across_files() {
    // Two functions both named "new" in different files.
    // The guard is the one in auth.rs — the BFS must not skip it
    // because it already visited db.rs::new.
    let guard = Symbol {
        name: "new".to_string(),
        file_path: "src/auth.rs".to_string(),
        // ...
    };
    let red_herring = Symbol {
        name: "new".to_string(),
        file_path: "src/db.rs".to_string(),
        // ...
    };
    // Connect handler → db::new → auth::new (guard)
    // With the old bug: BFS visits db::new, marks "new" visited,
    // never reaches auth::new, reports false violation.
    // With the fix: BFS visits both and finds the guard.
}
```

---

### Weakness 2 — `run_content` / JS `audit_content` Silently Skips All Project Rules

**Affected files:** `src/engine/project/mod.rs:266`, `src/js.rs:64`

**Severity:** Medium — every consumer of the Node.js API gets zero coverage from
`MustHaveGuard`, `MustBeInternal`, and `CrossFileTaintFree` with no indication
this is happening.

#### What the code does today

`run_content` is the single-file analysis path used by the Node.js
`audit_content` binding:

```rust
// src/engine/project/mod.rs:266
pub fn run_content(&self, file_path: &Path, content: &str) -> Result<Vec<Advisory>> {
    let mut registry = SourceRegistry::new();
    let id = registry.register(file_path, content.to_string());
    let mut symbols = SymbolRegistry::new();

    let (language, tree) = self.auditor.parse_source(file_path, content)?;
    let discovered = self.auditor.discover_symbols(file_path, content, &language, &tree)?;
    for sym in discovered { symbols.insert(sym); }
    let semantic_ops = self.auditor.extract_semantic_ops(file_path, content, &tree);

    let (advisories, _) = self.auditor.audit(
        id, file_path, content, &tree, &semantic_ops,
        &symbols, &self.enabled_categories, &self.enabled_tags, self.environment,
    )?;
    Ok(advisories)  // ← ProjectAuditor is never called
}
```

```rust
// src/js.rs:64 — this is what every Node.js caller hits
pub fn audit_content(&self, file_path: String, content: String) -> napi::Result<Vec<JsAdvisory>> {
    match self.inner.run_content(Path::new(&file_path), &content) {
```

The `audit_path` binding does call `run_detailed` (which includes Pass 5), but
`audit_path` is not documented and most editors and integrations use the simpler
`audit_content` per-file API.

#### The fix

**Step 1 — Add `audit_project` to the JS API** in `src/js.rs`:

```rust
/// Audit an entire project directory, including cross-file project rules.
/// Use this instead of `audit_content` when you need MustHaveGuard,
/// MustBeInternal, or CrossFileTaintFree rules to run.
#[napi]
pub fn audit_project(&mut self, root_dir: String) -> napi::Result<Vec<JsAdvisory>> {
    match self.inner.run(Path::new(&root_dir)) {
        Ok(advisories) => Ok(advisories.into_iter().map(to_js_advisory).collect()),
        Err(e) => Err(napi::Error::from_reason(format!("GenSense Engine Error: {e}"))),
    }
}
```

**Step 2 — Add a JSDoc comment to `audit_content`** making the limitation
explicit:

```rust
/// Analyse a single file in isolation. Per-file rules (style, security patterns,
/// AI artifacts) run in full. Cross-file project rules (MustHaveGuard,
/// MustBeInternal, CrossFileTaintFree) are NOT run — use `audit_project` for
/// those.
#[napi]
pub fn audit_content(&self, file_path: String, content: String) -> napi::Result<Vec<JsAdvisory>> {
```

**Step 3 — Add a Node.js integration test** in `tests/node/integration.test.js`:

```javascript
// Existing test covers audit_content (per-file rules).
// Add a test for audit_project (cross-file rules).
const engine = new GenSenseEngine();
const advisories = engine.auditProject('./tests/fixtures/project_with_guard_rule');
const ruleIds = advisories.map(a => a.ruleId);
assert(ruleIds.includes('GUARD_CHECK'), 'Project rules must fire via auditProject');
```

---

### Weakness 3 — No End-to-End Test for Project Rules Through the Full Engine

**Affected files:** `tests/e2e_tests.rs`, `tests/project_rules_tests.rs`

**Severity:** Low — the IR layer is tested in isolation, but no test verifies
that Pass 5 actually runs, that `disabled_rules` suppresses a project rule, or
that `severity_override` applies to a project advisory.

#### What the tests do today

`tests/project_rules_tests.rs` tests `ProjectRuleIr::check_project` directly,
bypassing the `Engine` entirely:

```rust
// Current approach — calls the rule directly, skips the engine pipeline
let p_rule = ProjectRuleCompiler::compile(p_rule_dsl).unwrap();
let advisories = p_rule.check_project(&symbols, &sources);
assert_eq!(advisories.len(), 1);
```

`tests/e2e_tests.rs` has three good full-engine tests but none of them mention
project rules:

```rust
fn test_e2e_user_yaml_rule_loaded()       // ← per-file custom rule only
fn test_e2e_suppress_file_respected()     // ← per-file suppression only
fn test_e2e_severity_override()           // ← per-file severity override only
```

#### The fix — add three e2e tests to `tests/e2e_tests.rs`

```rust
#[test]
fn test_e2e_project_rule_fires_via_engine() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Write two source files: a handler and a helper (no auth guard)
    fs::write(root.join("api.rs"),  "fn handle_request() { db_query(); }").unwrap();
    fs::write(root.join("db.rs"),   "fn db_query() {}").unwrap();

    // Write a project rule requiring handle_* to call check_auth
    let rules_dir = root.join(".gensense").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(rules_dir.join("guard.yml"), r#"
project_rules:
  - id: MUST_HAVE_AUTH
    name: "Auth Guard"
    severity: Critical
    observation: "Handler missing auth guard"
    category: Security
    impact: "Unauthenticated access"
    improvement: "Call check_auth"
    tags: ["security"]
    target_ext: "rs"
    must_have_guard:
      source_pattern: "handle_.*"
      guard_pattern: "check_auth"
      source_file_glob: "*"
      guard_file_glob: "*"
"#).unwrap();

    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    let advisories = engine.run(root).unwrap();

    assert!(
        advisories.iter().any(|a| a.rule_id == "MUST_HAVE_AUTH"),
        "Project rule should fire via full engine pipeline. Got: {advisories:?}"
    );
}

#[test]
fn test_e2e_project_rule_suppressed_by_disabled_rules() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("api.rs"), "fn handle_request() {}").unwrap();

    let config_dir = root.join(".gensense");
    fs::create_dir_all(&config_dir).unwrap();

    // Write the project rule
    fs::write(config_dir.join("rules").join("guard.yml"), /* same as above */"").unwrap();

    // Disable it via config
    fs::write(config_dir.join("config.yml"), r#"
disabled_rules:
  - MUST_HAVE_AUTH
"#).unwrap();

    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    let advisories = engine.run(root).unwrap();

    assert!(
        !advisories.iter().any(|a| a.rule_id == "MUST_HAVE_AUTH"),
        "Project rule should be suppressed by disabled_rules config"
    );
}

#[test]
fn test_e2e_project_rule_severity_override() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("api.rs"), "fn handle_request() {}").unwrap();

    let config_dir = root.join(".gensense");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("rules").join("guard.yml"), /* ... */"").unwrap();
    fs::write(config_dir.join("config.yml"), r#"
severity_override:
  MUST_HAVE_AUTH: Warning
"#).unwrap();

    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    let advisories = engine.run(root).unwrap();

    let adv = advisories
        .iter()
        .find(|a| a.rule_id == "MUST_HAVE_AUTH")
        .expect("Rule should fire");

    assert_eq!(
        adv.severity,
        gensense::Severity::Warning,
        "Severity should be overridden to Warning"
    );
}
```

---

## Part 2: Future Directions

---

### Direction 1 — LSP Server (`tower-lsp`)

**Impact:** High. Transforms GenSense from a batch CI scanner into a daily-driver
editor tool — advisories appear inline as you type in VS Code, Neovim, Helix, etc.

The existing `run_content` method has almost exactly the right signature for an
LSP `textDocument/publishDiagnostics` handler. The main work is wrapping it in a
`tower-lsp` server and translating `Advisory` to `lsp_types::Diagnostic`:

```rust
// Cargo.toml additions
tower-lsp = "0.20"
lsp-types = "0.95"
tokio = { version = "1", features = ["full"] }

// Advisory → LSP Diagnostic mapping
fn advisory_to_diagnostic(a: &Advisory) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: lsp_types::Range {
            start: lsp_types::Position { line: a.line - 1, character: a.column - 1 },
            end:   lsp_types::Position { line: a.line - 1, character: a.column + 20 },
        },
        severity: Some(match a.severity {
            Severity::Critical => lsp_types::DiagnosticSeverity::ERROR,
            Severity::Warning  => lsp_types::DiagnosticSeverity::WARNING,
            Severity::Info     => lsp_types::DiagnosticSeverity::INFORMATION,
        }),
        source: Some("gensense".to_string()),
        message: a.observation.clone(),
        ..Default::default()
    }
}
```

The `proposed_replacement` field on `Advisory` maps directly to a
`textDocument/codeAction` response, so auto-fix would work in the editor with no
additional analysis work.

---

### Direction 2 — Incremental Analysis

**Impact:** High. Required for editor integration to feel fast. Currently every
`run_detailed` call re-parses and re-analyses the entire project from scratch.

The `SourceRegistry` and `SymbolRegistry` are already the right data structures.
The addition needed is:

1. Store a content hash on `SourceFile`:
   ```rust
   pub struct SourceFile {
       pub id: FileId,
       pub path: PathBuf,
       pub content: Arc<str>,
       pub content_hash: u64,  // ← add this (FxHash or xxhash)
   }
   ```

2. Build a reverse-dependency index on `SymbolRegistry` — a map from symbol to
   the set of files that call it — so that when `src/auth.rs` changes, you can
   identify all files whose analysis might be invalidated.

3. In `Engine::run_detailed`, skip Pass 1 for files whose hash hasn't changed
   and reuse their cached snapshots.

This is an afternoon of work and would make LSP response times sub-100ms on large
projects.

---

### Direction 3 — Richer Temporal Rules

**Impact:** High. The `TemporalAnalyzer` exists but only supports simple call
sequences. The most valuable extension is expressing lock-acquire/await safety:

```yaml
# Declare a temporal rule in YAML
rules:
  - id: RUST_LOCK_ACROSS_AWAIT
    name: "Lock held across await"
    severity: Critical
    target_ext: rs
    on_node: function_item
    temporal:
      sequence: ["Mutex::lock", "await"]
      behavior: must_not_follow
      window: same_scope
```

This maps directly to the `TemporalBehavior::MustNotFollow` variant already in
`src/rules/ir.rs`. Adding a `window: same_scope` constraint would let the
analyzer check that no `await` appears between a lock acquisition and its drop
within the same lexical scope — the exact pattern that causes async deadlocks and
that `async_safety.rs` currently only partially catches.

---

### Direction 4 — SARIF Output

**Impact:** Medium-High. SARIF is the format GitHub's code scanning UI reads.
Outputting SARIF means GenSense findings appear as inline PR annotations in
GitHub without any extra tooling or configuration.

The `Advisory` struct maps cleanly to the SARIF schema:

```rust
// Advisory → SARIF Result
sarif::Result {
    rule_id: advisory.rule_id.clone(),
    message: sarif::Message { text: advisory.observation.clone() },
    locations: vec![sarif::Location {
        physical_location: sarif::PhysicalLocation {
            artifact_location: sarif::ArtifactLocation {
                uri: advisory.file_path.clone(),
            },
            region: sarif::Region {
                start_line: advisory.line,
                start_column: advisory.column,
            },
        },
    }],
    fixes: advisory.proposed_replacement.as_ref().map(|r| vec![
        sarif::Fix { description: "Apply suggested fix".into(), changes: vec![
            sarif::ArtifactChange {
                artifact_location: sarif::ArtifactLocation { uri: advisory.file_path.clone() },
                replacements: vec![sarif::Replacement {
                    deleted_region: sarif::Region { byte_offset: advisory.start_byte, byte_length: advisory.end_byte - advisory.start_byte },
                    inserted_content: sarif::ArtifactContent { text: r.clone() },
                }],
            }
        ]},
    ]),
}
```

Add `--output sarif` to the CLI and a `upload-sarif` step to `.github/workflows/ci.yml`.

---

### Direction 5 — Fix the `original_content` Gap in Project Advisories

**Impact:** Low (but easy). Every advisory from a project rule today has an empty
`original_content` field, breaking the patcher and `--fix` mode.

`ProjectRuleIr::new_advisory_for_symbol` already receives `sources: &SourceRegistry`.
The fix is a one-liner:

```rust
// src/rules/ir.rs:384 — current
original_content: String::new(), // Symbols don't store full content yet

// Fixed
original_content: sources
    .resolve_snippet(file_id, sym.start_byte as u32, sym.end_byte as u32)
    .unwrap_or_default(),
```

---

### Direction 6 — Expose Function Fingerprinting

**Impact:** Medium. The `FunctionFingerprint` infrastructure in
`src/engine/fingerprint.rs` is computed during every audit but never surfaced to
users. The most immediate application is **duplicate detection**: flagging when
the same function body (or a near-duplicate) appears in multiple files, which
usually indicates copy-paste that should be abstracted into a shared module.

Add a `--duplicates` CLI flag and a `find_duplicates` method on `Engine` that
groups fingerprints by hash and reports clusters of size ≥ 2.

---

## Summary

| # | Item | Type | Effort | Impact |
|---|------|------|--------|--------|
| W1 | BFS visited key bug | Fix | 30 min | Unblocks cross-file rules correctness |
| W2 | JS API skips project rules | Fix | 1 hour | Unblocks Node.js consumers |
| W3 | Missing e2e tests for project rules | Fix | 2 hours | Closes test coverage gap |
| F1 | LSP server | Future | 2–3 days | Transforms daily-driver utility |
| F2 | Incremental analysis | Future | 1–2 days | Required for fast LSP |
| F3 | Richer temporal rules | Future | 1 day | Catches async deadlock class |
| F4 | SARIF output | Future | 4 hours | GitHub PR annotations for free |
| F5 | Fix `original_content` in project advisories | Fix | 15 min | Unblocks patcher/`--fix` |
| F6 | Expose fingerprint duplicate detection | Future | 4 hours | High-value low-effort feature |
