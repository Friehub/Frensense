# GenSense v0.3.0+ Future Enhancements Plan

**Status**: Planning | **Created**: 2026-05-14 | **Version Baseline**: 0.2.1

This document outlines the complete roadmap for future enhancements following the v0.2.1 fixes. All 6 directions are broken into actionable tasks with priorities, effort estimates, and dependency tracking.

---

## Research Integration

This plan incorporates insights from three research documents that inform long-term strategy:

### From `gensense-agent-integration.md`
- GenSense should be a **shared ground truth** in multi-agent systems
- Agents need structured feedback to act autonomously
- Advisory struct needs `confidence` and `auto_fixable` fields
- Current JSON/SARIF output is already machine-readable; need contract clarity

**Implications for v0.3.0+:**
- Add confidence scoring (taint-based: 0.9+, heuristic: 0.6–0.8)
- Mark advisories that are safe for automatic fixing
- Document GenSense as agent-coordination tool

### From `gensense-algorithmic-grounding.md`
- GenSense already has strong mathematical foundations (taint lattices, temporal FSA, graph BFS)
- LSH/MinHash can detect LLM-generated structural duplicates (70-85% similarity)
- Datalog semantics can express complex reachability queries beyond manual BFS
- These extend existing infrastructure, don't replace it

**Implications for F6+ (Future):**
- Enhance fingerprinting with similarity scoring (pre-research)
- Express project rules in Datalog-like logic (post-v0.3.0)

### From `gensense-future-direction.md`
- New problem class: **Contract Surface Analysis (CSA)** for LLM-generated code
- LLM code fails structurally (incoherence between name/signature/body), not just mechanically
- GenSense can check name-based contracts (validate → must have conditional return)
- This is a v0.4+ strategic direction

**Implications for v0.4.0+ Strategy:**
- Add CSA rules: name-body coherence checking
- Foundation: Pattern `fn validate_*` → AST check for conditional falsy return
- Catch systematic LLM failures (tautological tests, placeholder panics, hollow implementations)

---

## Overview & Prioritization

### Tier 1: Quick Wins (Do First — v0.2.2)
These are low-effort, high-impact changes that unblock users immediately.

1. **F5 — Fix `original_content` Gap** | 15 min | Critical
2. **F4 — SARIF Output** | 4 hours | High

### Tier 2: Strategic Foundations (v0.3.0-alpha)
Prerequisite infrastructure for editor integration and agent systems.

3. **F2a — Agent-Ready Advisory Struct** | 2 hours | Medium (enhances F1/F4)
4. **F2 — Incremental Analysis** | 1-2 days | High (blocker for F1)
5. **F1 — LSP Server** | 2-3 days | High (depends on F2)

### Tier 3: High-Value Add-ons (v0.3.0+)
Can be done anytime; standalone features.

5. **F6 — Fingerprint Duplicates** | 4 hours | Medium
6. **F3 — Richer Temporal Rules** | 1 day | Medium

---

## TIER 1: QUICK WINS

### F5: Fix `original_content` Gap in Project Advisories

**Current State**: Project rule advisories have empty `original_content` field → breaks `--fix` mode

**Effort**: 15 minutes  
**Priority**: CRITICAL  
**Blocker**: None  
**Unblocks**: Patcher tool, `--fix` CLI flag

#### Acceptance Criteria
- [ ] Project rule advisories populate `original_content` 
- [ ] `--fix` mode works on project rule violations
- [ ] Existing tests still pass (no regressions)

#### Implementation Tasks

1. **Remove the placeholder** (1 min)
   - File: `src/rules/ir.rs:384`
   - Current: `original_content: String::new(),`
   - Change: Use `sources.resolve_snippet()` to extract actual content

2. **Test with patcher** (5 min)
   - Verify existing test `tests/correctness_tests.rs` no longer hits the empty case
   - Add assertion that `original_content` is non-empty for project rule advisories

3. **Update CLI help** (5 min)
   - Add note to `--fix` documentation that it now works with project rules
   - Example in README

#### Code Change Preview
```rust
// Before (src/rules/ir.rs:384)
original_content: String::new(),

// After
original_content: sources
    .resolve_snippet(file_id, sym.start_byte as u32, sym.end_byte as u32)
    .unwrap_or_default(),
```

#### Verification
```bash
cargo test tests/correctness_tests.rs --lib
# Should verify original_content is populated
```

---

### F4: SARIF Output Format

**Current State**: GenSense outputs JSON/stdout. GitHub code scanning wants SARIF.

**Effort**: 4 hours  
**Priority**: HIGH  
**Blocker**: None  
**Prerequisite**: None  
**Unblocks**: GitHub PR annotations without extra tooling

#### Acceptance Criteria
- [ ] `--output sarif` CLI flag works
- [ ] SARIF output is valid (passes `sarif-validator`)
- [ ] All advisory fields map cleanly
- [ ] Existing tests run with `--output json` (backward compat)

#### Implementation Tasks

1. **Add SARIF dependencies** (10 min)
   - `Cargo.toml`: Add `sarif = "0.1"` (or appropriate version)
   - `Cargo.toml`: Add `serde_json` if not present

2. **Add CLI flag** (15 min)
   - File: `src/bin/gensense.rs`
   - New enum: `OutputFormat::Json | Sarif`
   - Parse from `--output json|sarif`
   - Default: JSON (backward compat)

3. **Implement converter** (90 min)
   - File: New file `src/reporter/sarif.rs`
   - Function: `pub fn advisory_to_sarif_result(advisory: &Advisory) -> sarif::Result`
   - Handle all fields:
     - `rule_id` → `ruleId`
     - `observation` → `message.text`
     - `line`, `column` → `locations[0].region`
     - `file_path` → `locations[0].artifact_location.uri`
     - `proposed_replacement` → `fixes[0].changes[0].replacements`
     - `severity` → `level` (ERROR/WARNING/NOTE)

4. **Update reporter** (30 min)
   - File: `src/reporter.rs`
   - New function: `pub fn write_sarif(advisories: &[Advisory], out: &mut dyn Write)`
   - Wraps results in SARIF container object
   - Includes tool metadata (GenSense version, etc.)

5. **Add tests** (30 min)
   - `tests/sarif_output.rs` (new test file)
   - Test case: `test_sarif_output_format` — verify valid SARIF schema
   - Test case: `test_sarif_includes_all_fields` — spot check advisory fields
   - Test case: `test_sarif_backwards_compat_json` — JSON still works

6. **GitHub Actions integration** (15 min)
   - Add example `.github/workflows/gensense-sarif.yml`
   - Runs GenSense, outputs SARIF, uploads via `github/codeql-action/upload-sarif`

#### Code Structure Preview
```rust
// src/reporter/sarif.rs
use sarif::Result as SarifResult;

pub fn advisory_to_sarif_result(advisory: &Advisory) -> SarifResult {
    SarifResult {
        rule_id: Some(advisory.rule_id.clone()),
        message: sarif::Message {
            text: advisory.observation.clone(),
        },
        locations: vec![sarif::Location {
            physical_location: Some(sarif::PhysicalLocation {
                artifact_location: Some(sarif::ArtifactLocation {
                    uri: advisory.file_path.clone(),
                    ..Default::default()
                }),
                region: Some(sarif::Region {
                    start_line: Some(advisory.line as i64),
                    start_column: Some(advisory.column as i64),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }],
        level: Some(match advisory.severity {
            Severity::Critical => sarif::Level::Error,
            Severity::Warning => sarif::Level::Warning,
            Severity::Info => sarif::Level::Note,
        }),
        fixes: advisory.proposed_replacement.as_ref().map(|text| vec![
            sarif::Fix {
                description: Some(sarif::Message {
                    text: "Apply suggested fix".to_string(),
                }),
                changes: vec![sarif::ArtifactChange {
                    artifact_location: sarif::ArtifactLocation {
                        uri: advisory.file_path.clone(),
                        ..Default::default()
                    },
                    replacements: vec![sarif::Replacement {
                        deleted_region: Some(sarif::Region {
                            byte_offset: Some(advisory.start_byte as i64),
                            byte_length: Some((advisory.end_byte - advisory.start_byte) as i64),
                            ..Default::default()
                        }),
                        inserted_content: Some(sarif::ArtifactContent {
                            text: text.clone(),
                        }),
                        ..Default::default()
                    }],
                }],
            }
        ]),
        ..Default::default()
    }
}

pub fn advisories_to_sarif(advisories: &[Advisory]) -> sarif::Report {
    sarif::Report {
        version: "2.1.0".parse().unwrap(),
        runs: vec![sarif::Run {
            tool: sarif::Tool {
                driver: sarif::ToolComponent {
                    name: "gensense".to_string(),
                    version: Some(crate::GENSENSE_VERSION.to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            results: advisories.iter()
                .map(advisory_to_sarif_result)
                .collect(),
            ..Default::default()
        }],
        ..Default::default()
    }
}
```

#### Verification
```bash
# Build
cargo build --release

# Test SARIF output
./target/release/gensense --output sarif ./tests/samples > output.sarif
cat output.sarif | python -m json.tool > /dev/null  # validate JSON

# Optional: Install sarif-validator and check
npm install -g @microsoft/sarif-multitool
multitool validate output.sarif
```

---

## TIER 2: STRATEGIC FOUNDATIONS

> **Note**: F2a enhances advisories for multi-agent use. F2 must complete before F1. F1 depends on incremental analysis for sub-100ms response times.

### F2a: Agent-Ready Advisory Struct

**Current State**: Advisory struct has all fields humans need; agents need confidence scoring and auto-fix safety flags.

**Effort**: 2 hours  
**Priority**: MEDIUM  
**Blocker**: None  
**Enhances**: F1 (LSP), F4 (SARIF)  
**Source**: `gensense-agent-integration.md`  
**Unblocks**: Multi-agent system coordination

#### Acceptance Criteria
- [ ] Advisory struct adds `confidence: f32` (0.0–1.0)
- [ ] Advisory struct adds `auto_fixable: bool`
- [ ] Confidence scoring implemented (taint: 0.9+, heuristic: 0.6–0.8)
- [ ] Auto-fixable flags on rules with safe proposed_replacement
- [ ] JSON/SARIF output includes new fields
- [ ] Agents can parse and act on confidence scores

#### Implementation Tasks

1. **Extend Advisory struct** (15 min)
   - File: `src/lib.rs` (Advisory definition)
   - Add fields:
     ```rust
     pub confidence: f32,     // 0.0–1.0, higher = more certain
     pub auto_fixable: bool,  // safe for agent to apply fix without review
     ```

2. **Confidence scoring** (45 min)
   - File: New function `src/rules/confidence.rs`
   - Score by rule type:
     - Taint finds (high certainty): 0.95
     - Temporal violations: 0.90
     - Pattern matches (fingerprint-based): 0.85
     - Heuristic rules (naming patterns): 0.65
     - AI artifact detection: 0.75
   - Compute in auditor before returning Advisory

3. **Auto-fixable flag** (30 min)
   - File: `src/rules/metadata.rs`
   - Rules marked `auto_fixable: true` if:
     - Has `proposed_replacement`
     - No cross-file dependencies
     - IDE can safely apply (no side effects)
   - Examples: Import formatting (yes), guard check rewrites (no)

4. **JSON/SARIF serialization** (30 min)
   - Update `src/reporter.rs` to include new fields
   - Update SARIF output to map confidence → priority hints

#### Verification
```bash
# Test advisory structure
cargo test --lib
jq '.confidence' output.json  # verify field exists

# Test agent-ready output
./target/release/gensense --output json ./src | jq '.[] | {rule_id, confidence, auto_fixable}'
```

---

## TIER 2: STRATEGIC FOUNDATIONS

### F2 — Incremental Analysis

**Current State**: Every `run_detailed()` call re-parses and re-analyzes entire project from scratch.

**Effort**: 1-2 days  
**Priority**: HIGH  
**Blocker**: None  
**Blocked By**: None  
**Prerequisite For**: F1 (LSP Server)  
**Unblocks**: Sub-100ms LSP diagnostics

#### Acceptance Criteria
- [ ] Content hash stored on `SourceFile`
- [ ] Cache layer built (map: FileId → last_hash, last_symbols)
- [ ] Modified files re-analyzed, unchanged files reuse cached symbols
- [ ] Performance: 10x faster on large projects with few changes
- [ ] Benchmarks show sub-100ms on typical 10k-LOC project

#### Implementation Tasks

1. **Add content hash tracking** (30 min)
   - File: `src/engine/source.rs`
   - Add field to `SourceFile`: `pub content_hash: u64`
   - Compute hash on register: `use std::collections::hash_map::DefaultHasher;`
   - Hash algorithm: `DefaultHasher` (or `xxhash2` for speed)

2. **Build reverse-dependency index** (1 hour)
   - File: `src/engine/symbols.rs` (extend `SymbolRegistry`)
   - New method: `pub fn get_reverse_deps(&self, sym: &Symbol) -> Vec<FileId>`
   - During symbol registration, track: "which files call this file's symbols"
   - Data structure: `file_id → Vec<FileId>` (callers)

3. **Implement cache layer** (1.5 hours)
   - File: New file `src/engine/cache.rs`
   - Struct: `AnalysisCache { file_id → (hash, symbols, semantic_ops) }`
   - Methods:
     - `pub fn is_valid(&self, file_id: FileId, current_hash: u64) -> bool`
     - `pub fn get_symbols(&self, file_id: FileId) -> Option<Vec<Symbol>>`
     - `pub fn invalidate_dependents(&mut self, file_id: FileId)`
     - `pub fn store(&mut self, file_id: FileId, hash: u64, symbols: Vec<Symbol>, ops: Vec<SemanticOp>)`

4. **Integrate into Engine** (1 hour)
   - File: `src/engine/project/mod.rs`
   - Before Pass 1 (parse): `if cache.is_valid(file_id, new_hash) { reuse symbols; skip parsing }`
   - After Pass 1 (parse): `cache invalidate_dependents(file_id)` if hash changed
   - Result: Only changed files + their dependents re-analyzed

5. **Add benchmarks** (30 min)
   - File: `benches/incremental.rs` (new benchmark)
   - Scenario 1: No changes (10 files) — should be near-instant
   - Scenario 2: One file changes (10 files) — should reanalyze that file + dependents only
   - Target: 50ms for scenario 2 on modern hardware

6. **Tests** (30 min)
   - `tests/incremental_analysis_tests.rs` (new test file)
   - Test: `test_cache_hit_skips_parsing`
   - Test: `test_cache_invalidation_on_change`
   - Test: `test_reverse_deps_computed_correctly`

#### Algorithm Overview
```
Before (Parse Everything):
  for each file:
    parse() → AST
    discover_symbols() → symbols
    extract_ops() → semantic ops
  Total: O(n) where n = all files

After (Incremental):
  for each file:
    current_hash = hash(content)
    if cache.is_valid(file_id, current_hash):
      symbols = cache.get_symbols(file_id)  // ← skip parsing!
    else:
      parse() → AST
      discover_symbols() → symbols
      if hash_changed:
        invalidate_dependents(file_id)      // ← invalidate only affected files
  Total: O(c) where c = changed files + dependents (~1-5 files in practice)
```

#### Verification
```bash
# Build
cargo build --all --release

# Run incremental benchmark
cargo bench --bench incremental

# Expected output:
# no_changes (reuse all)    ... 5ms
# one_change (10% analyzed) ... 45ms
```

---

### F1: LSP Server (`tower-lsp`)

**Current State**: GenSense is batch CLI tool. No real-time editor integration.

**Effort**: 2-3 days  
**Priority**: HIGH  
**Blocker**: F2 must be complete  
**Unblocks**: VS Code, Neovim, Helix, Emacs integration (any LSP client)

#### Acceptance Criteria
- [ ] LSP server binary starts and listens on stdio
- [ ] `textDocument/didOpen` triggers analysis
- [ ] `textDocument/didChange` triggers incremental re-analysis
- [ ] `textDocument/publishDiagnostics` sends results to client
- [ ] `textDocument/codeAction` returns proposed fixes
- [ ] Works with VS Code and at least one other editor (Neovim)

#### Implementation Tasks

1. **Add dependencies** (5 min)
   - `Cargo.toml`:
     ```toml
     tower-lsp = "0.20"
     lsp-types = "0.95"
     tokio = { version = "1", features = ["full"] }
     ```

2. **Create LSP service struct** (1 hour)
   - File: New file `src/lsp/service.rs`
   - Struct: `GenSenseLsp { engine: Engine, project_root: PathBuf }`
   - Implement `tower_lsp::LanguageServer` trait:
     - `initialize()`
     - `initialized()`
     - `did_open()`
     - `did_change()`
     - `did_save()`
     - `execute_command()` (for code actions)

3. **Implement diagnostics handler** (1 hour)
   - Convert `Advisory` → `lsp_types::Diagnostic`
   - Map severity: Critical→Error, Warning→Warning, Info→Information
   - Map location: `line`, `column` → LSP range (0-indexed)
   - Send via `client.publish_diagnostics()`

4. **Implement code action handler** (1 hour)
   - On `textDocument/codeAction`:
     - Find advisories at cursor position
     - For each with `proposed_replacement`:
       - Create `CodeAction` with `TextEdit`
     - Return array of actions
   - User can apply fix directly in editor

5. **Create LSP entrypoint binary** (30 min)
   - File: New file `src/bin/gensense-lsp.rs`
   - Main function:
     ```rust
     #[tokio::main]
     async fn main() {
         let stdin = tokio::io::stdin();
         let stdout = tokio::io::stdout();
         let (service, socket) = LspService::new(GenSenseLsp::new());
         Server::new(stdin, stdout, socket).run(service).await;
     }
     ```

6. **VS Code extension scaffold** (1.5 hours)
   - File: New directory `editors/vscode/`
   - Create minimal extension that:
     - Launches `gensense-lsp` binary
     - Registers for Rust, TypeScript, Solidity files
     - Shows diagnostics inline as you type
   - Publish to VS Code Marketplace (optional, can be local install)

7. **Integration tests** (1 hour)
   - File: `tests/lsp_tests.rs` (new test file)
   - Test: `test_lsp_publish_diagnostics_on_open`
   - Test: `test_lsp_code_action_returns_fixes`
   - Test: `test_lsp_ignore_suppressed_rules`
   - Mock LSP client to verify messages

8. **Documentation** (30 min)
   - Add `docs/lsp.md` with setup instructions
   - Update README with editor integration section
   - Add VS Code extension README

#### LSP Service Skeleton
```rust
// src/lsp/service.rs
use tower_lsp::*;

pub struct GenSenseLsp {
    client: Client,
    engine: Engine,
    documents: DashMap<Url, String>,
    project_root: PathBuf,
}

#[tower_lsp::async_trait]
impl LanguageServer for GenSenseLsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.project_root = params.root_uri
            .and_then(|uri| uri.to_file_path().ok())
            .unwrap_or_default();
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        self.documents.insert(uri.clone(), content.clone());
        
        // Run analysis
        let file_path = uri.to_file_path().unwrap_or_default();
        if let Ok(advisories) = self.engine.run_content(&file_path, &content) {
            let diagnostics = advisories.into_iter()
                .map(|a| advisory_to_diagnostic(&a))
                .collect();
            self.client.publish_diagnostics(uri, diagnostics, None).await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(mut text) = self.documents.get_mut(&uri) {
            for change in params.content_changes {
                *text = change.text;
            }
        }
        // Trigger re-analysis (leverages F2 incremental analysis for speed)
        // ... same as did_open
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        match params.command.as_str() {
            "gensense.fix" => {
                // Apply code action fix
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

fn advisory_to_diagnostic(advisory: &Advisory) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position { line: advisory.line - 1, character: advisory.column - 1 },
            end: Position { line: advisory.line - 1, character: advisory.column + 20 },
        },
        severity: Some(match advisory.severity {
            Severity::Critical => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
        }),
        source: Some("gensense".to_string()),
        message: advisory.observation.clone(),
        code: Some(NumberOrString::String(advisory.rule_id.clone())),
        ..Default::default()
    }
}
```

#### Verification
```bash
# Build LSP binary
cargo build --release --bin gensense-lsp

# Test with VS Code or Neovim
# VS Code: Install extension from ./editors/vscode
# Neovim: Configure with nvim-lspconfig

# Manual test with nc (netcat) for debugging
# (See docs/lsp.md for debug session setup)
```

---

## TIER 3: HIGH-VALUE ADD-ONS

> These are standalone features that don't block anything else. Can be done in any order.

### F6: Expose Function Fingerprinting — Duplicate Detection

**Current State**: `FunctionFingerprint` computed during audit but never exposed to users.

**Effort**: 4 hours  
**Priority**: MEDIUM  
**Blocker**: None  
**Unblocks**: Copy-paste detection, code quality insights

#### Acceptance Criteria
- [ ] `--duplicates` CLI flag works
- [ ] Groups fingerprints by hash
- [ ] Reports clusters of 2+ functions with identical bodies
- [ ] Integrates with JSON output (`--output json` includes duplicates section)

#### Implementation Tasks

1. **Add CLI flag** (15 min)
   - File: `src/bin/gensense.rs`
   - New flag: `--duplicates` (bool)
   - Default: false (don't compute to avoid overhead)

2. **Extend Engine** (1 hour)
   - File: `src/engine/project/mod.rs`
   - New method: `pub fn find_duplicates(&self) -> Vec<DuplicateCluster>`
   - Struct: `DuplicateCluster { hash: u64, functions: Vec<Symbol>, body_hash: String }`
   - Returns clusters of size >= 2

3. **Output formatting** (1 hour)
   - File: `src/reporter.rs`
   - New struct: `DuplicateReport { clusters: Vec<DuplicateCluster> }`
   - JSON output: Add `"duplicates"` section if `--duplicates` flag set
   - CLI output: Pretty-print "Found 3 duplicate functions across 2 files"

4. **Integration** (1 hour)
   - File: `src/engine/fingerprint.rs`
   - Add method to group fingerprints by hash
   - Track which functions have same body hash
   - Return as structured report

5. **Tests & Docs** (1.5 hours)
   - Test: `tests/duplicate_detection_tests.rs` (new)
   - Test: `test_finds_copy_pasted_functions`
   - Test: `test_ignores_similar_but_different_functions`
   - Docs: Add `docs/duplicate-detection.md`
   - Example: Show how to find copy-paste in real projects

#### Example Output
```
$ gensense --duplicates ./src

GenSense v0.2.1 Duplicate Detection Report
===========================================

Found 2 duplicate function clusters:

1. Hash: a1b2c3d4e5f6
   Found 3 identical functions:
   - src/db.rs::query_user() (45 bytes)
   - src/api.rs::query_user() (45 bytes)
   - src/cache.rs::query_user() (45 bytes)
   
   Suggestion: Extract to src/common/query.rs and import everywhere

2. Hash: x7y8z9w1v2u3
   Found 2 identical functions:
   - src/validate.rs::check_email() (28 bytes)
   - src/handlers.rs::check_email() (28 bytes)
```

---

### F3: Richer Temporal Rules — Async Safety Patterns

**Current State**: `TemporalAnalyzer` exists but only supports simple call sequences.

**Effort**: 1 day  
**Priority**: MEDIUM  
**Blocker**: None  
**Unblocks**: Async deadlock detection, lock safety patterns

#### Acceptance Criteria
- [ ] New `window: same_scope` constraint in temporal rules
- [ ] Detects lock-across-await patterns
- [ ] Catches `Mutex::lock() ... await` in same lexical scope
- [ ] Reduces false positives in async code

#### Implementation Tasks

1. **Extend temporal rule DSL** (1 hour)
   - File: `src/rules/ir.rs`
   - New enum variant: `TemporalWindow::SameScope | AnyScope | CallDepth(N)`
   - Parse YAML: `window: same_scope`

2. **Implement scope analysis** (2 hours)
   - File: `src/semantics/temporal.rs`
   - New function: `check_temporal_sequence_in_scope()`
   - Track lexical scope depth (brace nesting in AST)
   - Verify sequence doesn't cross scope boundary

3. **Add YAML test rules** (30 min)
   - File: `tests/fixtures/temporal_rules.yml`
   - Example: `RUST_LOCK_ACROSS_AWAIT` rule using new `window: same_scope`
   - Example: `RUST_UNSAFE_LIFETIME` rule

4. **Test cases** (1.5 hours)
   - File: `tests/temporal_scope_tests.rs` (new)
   - Test: `test_detects_lock_across_await_same_scope`
   - Test: `test_ignores_lock_across_await_different_scope`
   - Test: `test_temporal_window_constraint_applied`
   - Test fixtures: Rust async code samples

5. **Documentation** (30 min)
   - File: `docs/temporal-rules.md`
   - Explain new `window` constraint
   - Provide example rules for common async patterns
   - Reference MSRV (Minimum Supported Rust Version) compatibility

#### Example Rule
```yaml
project_rules:
  - id: RUST_LOCK_ACROSS_AWAIT
    name: "Lock Held Across Await Point"
    severity: Critical
    target_ext: rs
    temporal:
      # Detect: Mutex::lock ... await in same scope
      sequence: ["Mutex::lock", ".await"]
      behavior: must_not_follow
      window: same_scope  # ← NEW: must be in same block scope
      reason: "Lock held across await can deadlock or stall other tasks"
      improvement: "Drop lock before await point"
```

#### Verification
```bash
# Test temporal rules
cargo test --test temporal_scope_tests

# Run on real code
cargo build --release
./target/release/gensense ./tests/samples/async_code/

# Should find RUST_LOCK_ACROSS_AWAIT violations
```

---

## Implementation Timeline & Sequencing

### Phase 1: Quick Wins (v0.2.2) — 1-2 days
```
Week 1
├─ Mon: F5 (15 min) + F4 intro (30 min)
├─ Tue: F4 SARIF impl (3 hours) + tests (1 hour)
├─ Wed: F4 GHA integration (1 hour) + PR review
└─ Done → v0.2.2 release
```

### Phase 2: Strategic Foundations (v0.3.0-alpha) — 1 sprint
```
Week 2-3
├─ Mon-Tue: F2 incremental analysis (1.5 days)
├─ Wed-Fri: F1 LSP server (2.5 days)
│   ├─ Thu: VS Code extension scaffold
│   └─ Fri: Integration tests & docs
└─ Done → v0.3.0-alpha release
```

### Phase 3: Add-ons (v0.3.0) — Parallel, 2-4 days
```
Week 4
├─ Mon: F6 duplicate detection (4 hours)
├─ Tue: F3 temporal rules (1 day) [can be parallel with F6]
├─ Wed: Polish, docs, benchmarks
└─ Done → v0.3.0 release
```

**Total Engineering**: ~1-2 sprints (2-3 weeks)

---

## Implementation Checklist Template

### Feature: [Name]

**Status**: ⏳ Not Started | 🔄 In Progress | ✅ Complete

- [ ] Acceptance criteria met
- [ ] Code implemented and reviewed
- [ ] Tests written (unit + integration)
- [ ] Benchmarks run (if applicable)
- [ ] Documentation updated (README, docs/, examples/)
- [ ] Example scripts/fixtures added
- [ ] Changelog entry added
- [ ] PR merged to main
- [ ] Release notes prepared

---

## Success Metrics

| Feature | Metric | Target | Notes |
|---------|--------|--------|-------|
| F5 | `original_content` populated | 100% | Must be non-empty for all project rules |
| F4 | SARIF output valid | 100% | Passes sarif-validator, GitHub recognizes |
| F2 | Incremental analysis speed | <100ms | For 10k LOC project with 1 file changed |
| F1 | LSP diagnostics latency | <250ms | From file change to diagnostics published |
| F1 | Code action fix application | 100% | All fixed tests in original tests still pass |
| F6 | Duplicate cluster detection | 100% | Finds all real duplicates in test suite |
| F3 | Temporal rule precision | >95% | False positive rate < 5% |

---

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|-----------|
| LSP perf too slow without incremental | HIGH | Complete F2 before F1; add benchmarks |
| SARIF output breaks existing workflows | MEDIUM | Keep JSON default, SARIF opt-in with flag |
| Temporal scope analysis misses edge cases | MEDIUM | Add extensive fuzzing, real-world tests |
| LSP VS Code extension complex | LOW | Start with minimal feature set; extend later |
| Fingerprint hash collisions | LOW | Use cryptographic hash (SHA256), not FxHash |

---

## Dependency Graph

```
F2a (Agent-Ready Advisory)
  └─ enhances F1 (LSP) and F4 (SARIF)

F1 (LSP)
  ├─ requires F2 (Incremental Analysis)
  ├─ depends on F5 (original_content)
  ├─ enhanced by F2a (Agent-Ready Advisory)
  └─ can use F4 (SARIF for language server protocol)

F2 (Incremental Analysis)
  └─ no external dependencies

F3 (Temporal Rules)
  └─ standalone, can be done anytime

F4 (SARIF)
  ├─ enhanced by F2a (Agent-Ready Advisory in SARIF)
  └─ standalone, can be done anytime

F5 (original_content fix)
  └─ standalone, should do first

F6 (Duplicates)
  └─ standalone, can be done anytime

Implementation Order (respecting dependencies):
1. Do F5, F4, F6, F3, F2a in parallel (no blockers)
2. Complete F2 (no blocker, enables F1)
3. Do F1 last (depends on F2)
```

---

## Future Research Directions (v0.4.0+)

The research documents outline strategic directions beyond the v0.3 feature roadmap:

### Contract Surface Analysis (CSA) — v0.4.0

**Source**: `gensense-future-direction.md`  
**Impact**: Catch systematic LLM-generated failures (contradiction between name and implementation)  
**Effort**: 2-3 days research + implementation  

**Core Idea**: Check coherence between function name/signature and implementation body.

**Examples**:
- `fn validate_*` must have conditional that can return false/error
- `fn sanitize_*` must transform input, not return unmodified
- `fn find_*` must have code path returning None/empty
- `fn create_*` must return new value, not reuse input

**Mathematical Basis**: Already in codebase
- AST pattern matching (existing rule IR)
- Name hashing and comparison
- Control flow graph reachability

**Research Phase**: Formalize name-contract grammar, validate on LLM code corpus

---

### Algorithmic Enhancements (v0.4.0+)

**Source**: `gensense-algorithmic-grounding.md`

#### Similarity Scoring with MinHash (v0.4.0)
- Extend fingerprinting with Locality-Sensitive Hashing
- Detect functions that are 70-85% structurally similar
- Useful for LLM-generated variants of the same function
- **Effort**: 1 day | **Prerequisite**: F6 (duplicate detection foundation)

#### Datalog Semantics for Project Rules (v0.4.0+)
- Express complex reachability queries beyond manual BFS
- Replace hand-written `ProjectRule` BFS with declarative rules
- Example: "Variable x flows from source S to sink T through at most 3 hops"
- **Effort**: 2-3 days research | **Prerequisite**: F2 (incremental analysis for performance)

---

## Getting Started

1. **Pick a feature from Tier 1** (F5 or F4)
2. **Create a branch**: `git checkout -b feature/f5-original-content` or `feature/f4-sarif`
3. **Break it into PRs**: One per "Implementation Task" section
4. **Reference this document**: Include section link in PR description
5. **Update this file**: Mark tasks as 🔄 In Progress when starting

---

**Document Version**: 1.1 | **Last Updated**: 2026-05-14 | **Research Sources**: 3 | **Next Review**: Before F5 starts
