# Frensense Code Coverage Map
## What's Been Read vs What Remains

> Generated: 2026-06-20
> Total source files: 100 `.rs` files

---

## Coverage Legend

```
✅ READ     — Thoroughly read and documented
⚠️ PARTIAL  — Partially read or read via grep/search
❌ UNREAD   — Not yet read
```

---

## frensense-engine/src/ (Engine Layer)

```
frensense-engine/src/
├── lib.rs                          ✅ READ — exports, analyze_file, analyze_project API
├── ast_distance.rs                 ✅ READ — tree edit distance (used by scorer)
├── atomic_section.rs               ✅ READ — lock/unlock pairing, TOCTOU detection
├── cfg/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   └── def_use.rs                  ⚠️ PARTIAL — used by unused_variable finding
├── corpus/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── bundle.rs                   ⚠️ PARTIAL — FRC1 binary format
│   ├── loader.rs                   ✅ READ — load_corpus, sidecar TOML, multi-example
│   ├── registry.rs                 ✅ READ — PatternRegistry, scan_function, LSH index
│   └── semantic.rs                 ✅ READ — SemanticFilter, AST-level constraints
├── data_flow/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── alias.rs                    ✅ READ — transitive alias tracking
│   ├── confidence.rs               ✅ READ — CFG-based taint confidence adjustment
│   ├── cross_file.rs               ⚠️ PARTIAL — cross-file taint in engine
│   ├── engine.rs                   ✅ READ — DataFlowEngine with summary caching
│   ├── normalization.rs            ✅ READ — SemanticOp extraction (Binding/Assignment/Call/EnterBlock)
│   ├── resolver.rs                 ✅ READ — taint source seeding (regex-based)
│   └── taint_metrics.rs            ✅ READ — hollow validator detection (branch ratio < 0.2)
├── deps.rs                         ✅ READ — dependency resolver (Cargo.lock, package.json)
├── fingerprint.rs                  ✅ READ — 7-dimensional fingerprinting, IDF, positional ngrams
├── graph.rs                        ✅ READ — SemanticGraph, duplicate of src/semantics/graph.rs
├── lang/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── kinds.rs                    ✅ READ — AbstractKind taxonomy (32 kinds)
│   └── mapper.rs                   ✅ READ — per-language mapper (Rust, TS, C, Python)
├── minhash.rs                      ✅ READ — MinHash LSH, 16 bands × 8 rows
├── parser.rs                       ⚠️ PARTIAL — language detection, parser registry
├── pattern/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── canonical.rs                ✅ READ — canonical form, structural similarity
│   ├── compiler.rs                 ✅ READ — PatternNode compilation, wildcards
│   ├── matcher.rs                  ✅ READ — AST matching with captures
│   └── scorer.rs                   ✅ READ — 5-dimensional scoring, AST edit distance
├── profile.rs                      ✅ READ — ProjectProfile, style surprise detection
├── reachability.rs                 ⚠️ PARTIAL — used by dead_branch finding
├── secrets.rs                      ✅ READ — 9 secret patterns, entropy filtering
├── semantic_patterns/
│   ├── mod.rs                      ✅ READ — module declaration, PatternFinding
│   ├── check_then_act.rs           ✅ READ — CHECK_THEN_ACT_TOCTOU (Prisma-only)
│   ├── helpers.rs                  ✅ READ — is_db_read, is_db_write, is_inside_transaction
│   └── registry.rs                 ✅ READ — SemanticPattern trait, PatternRunner
├── symbols.rs                      ✅ READ — SymbolRegistry with call graph edges
└── temporal.rs                     ✅ READ — TemporalAnalyzer, 5 built-in rules
```

---

## src/ (CLI + Pipeline Layer)

```
src/
├── lib.rs                          ⚠️ PARTIAL — public API
├── parser.rs                       ❌ UNREAD — language detection
├── reporter.rs                     ✅ READ — Markdown + SARIF output (no JSON)
├── bin/
│   ├── frensense.rs                ❌ UNREAD — main CLI entry
│   ├── frensense-mcp.rs            ✅ READ — MCP server entry
│   ├── build-corpus-bundle.rs      ❌ UNREAD — bundle builder
│   └── retrain-calibration.rs      ❌ UNREAD — calibration retrain
├── cli/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── commands.rs                 ⚠️ PARTIAL — CLI commands
│   ├── options.rs                  ✅ READ — ALL 35+ CLI flags, defaults
│   ├── extras.rs                   ❌ UNREAD — extra CLI features
│   └── reporting.rs                ❌ UNREAD — CLI reporting
├── engine/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── ast_diff.rs                 ❌ UNREAD — AST diff
│   ├── clustering.rs               ❌ UNREAD — function clustering
│   ├── composition.rs              ✅ READ — Layer signal AND-gate composition
│   ├── confidence_calibration.rs   ✅ READ — Platt scaling with gradient descent
│   ├── fingerprint.rs              ❌ UNREAD — fingerprint extraction
│   ├── learn.rs                    ❌ UNREAD — learning from feedback
│   ├── per_category_calibration.rs ❌ UNREAD — per-category calibration
│   ├── profile.rs                  ❌ UNREAD — profile analysis
│   ├── source.rs                   ❌ UNREAD — source registry
│   ├── suppression.rs              ⚠️ PARTIAL — baseline suppression
│   ├── auditor/
│   │   ├── mod.rs                  ✅ READ — 3-phase audit, combined query
│   │   ├── discovery.rs            ✅ READ — symbol discovery
│   │   ├── events.rs               ✅ READ — temporal event discovery
│   │   ├── project_auditor.rs      ⚠️ PARTIAL — project-level rules
│   │   └── rules.rs                ✅ READ — default_rules() returns empty
│   ├── findings/
│   │   ├── mod.rs                  ✅ READ — FindingModule trait, registered_modules
│   │   ├── atomic_section.rs       ✅ READ — ATOMIC_SECTION_INCOMPLETE
│   │   ├── cross_file_taint.rs     ✅ READ — NO-OP (returns empty vec)
│   │   ├── dead_branch.rs          ✅ READ — uses ReachabilityChecker
│   │   ├── hallucinated_import.rs  ✅ READ — uses DependencyResolver
│   │   ├── semantic_patterns.rs    ✅ READ — runs PatternRunner
│   │   ├── temporal_violation.rs   ✅ READ — uses TemporalAnalyzer
│   │   └── unused_variable.rs      ✅ READ — uses CFG def-use chains
│   └── project/
│       ├── mod.rs                  ⚠️ PARTIAL — module declaration
│       ├── builder.rs              ❌ UNREAD — engine builder
│       ├── cache.rs                ⚠️ PARTIAL — file cache
│       ├── config.rs               ✅ READ — 3 YAML options only
│       ├── files.rs                ❌ UNREAD — file discovery
│       ├── helpers.rs              ❌ UNREAD — helper functions
│       └── runner.rs               ✅ READ — 8-phase pipeline (724 lines)
├── mcp/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── audit.rs                    ✅ READ — MCP audit, streaming, filters
│   ├── handler.rs                  ✅ READ — JSON-RPC 2.0 handler
│   └── protocol.rs                 ✅ READ — MCP protocol types
├── patcher/
│   └── mod.rs                      ✅ READ — atomic patching, import injection
├── semantics/
│   ├── mod.rs                      ⚠️ PARTIAL — module declaration
│   ├── consistency.rs              ✅ READ — cross-path consistency checking
│   ├── graph.rs                    ✅ READ — duplicate of engine graph
│   ├── reachability.rs             ⚠️ PARTIAL — used by dead_branch
│   ├── simple_taint.rs             ✅ READ — lightweight taint checker
│   ├── symbols.rs                  ❌ UNREAD — SymbolRegistry
│   └── data_flow/
│       ├── mod.rs                  ⚠️ PARTIAL — module declaration
│       ├── corpus_seeder.rs        ✅ READ — seeds taint from corpus matches
│       ├── cross_file.rs           ✅ READ — CrossFileVerifier
│       ├── handlers.rs             ❌ UNREAD — taint handlers
│       ├── interprocedural.rs      ✅ READ — InterproceduralVerifier
│       ├── lookup.rs               ❌ UNREAD — taint lookup
│       ├── normalization.rs        ❌ UNREAD — semantic op normalization
│       └── tracking.rs             ⚠️ PARTIAL — TaintTracker
└── temporal/
    ├── mod.rs                      ⚠️ PARTIAL — module declaration
    ├── analyzer.rs                 ❌ UNREAD — TemporalAnalyzer
    └── config.rs                   ✅ READ — temporal rule loading
```

---

## Summary Statistics

| Category | Total | Read | Partial | Unread |
|----------|-------|------|---------|--------|
| Engine (`frensense-engine/src/`) | 41 | 31 | 10 | 0 |
| CLI + Pipeline (`src/`) | 59 | 27 | 14 | 18 |
| **Total** | **100** | **58** | **24** | **18** |

---

## Priority Unread Files (High Impact)

These files likely contain important logic not yet documented:

| File | Why It Matters |
|------|----------------|
| `src/cli/options.rs` | ALL CLI flags — what users can configure |
| `src/engine/composition.rs` | How findings are composed/adjusted |
| `src/engine/confidence_calibration.rs` | Platt scaling implementation |
| `src/data_flow/engine.rs` | DataFlowEngine internals |
| `src/data_flow/confidence.rs` | How taint confidence is adjusted |
| `src/data_flow/alias.rs` | How variable aliases are tracked |
| `src/lang/kinds.rs` | AbstractKind taxonomy (32 kinds) |
| `src/lang/mapper.rs` | Per-language AST mapper |
| `src/symbols.rs` | SymbolRegistry implementation |
| `src/engine/clustering.rs` | Function clustering for near-duplicate detection |
| `src/engine/learn.rs` | Learning from feedback |
| `src/bin/frensense.rs` | Main CLI entry point |
| `src/bin/build-corpus-bundle.rs` | How bundles are built |

---

## How to Read Unread Files

```bash
# Read a specific unread file
cat src/cli/options.rs

# Or use the Read tool
Read(src/cli/options.rs)

# Search for specific patterns in unread files
grep -r "pub fn" src/engine/composition.rs
grep -r "struct " src/data_flow/engine.rs
```
