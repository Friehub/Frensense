# GenSense Rule Detection Capabilities -- Complete Inventory

## 1. AST Query Matching (tree-sitter)
**Mechanism:** The engine compiles `on_node` into a tree-sitter query (stored in `AstQuery`). Each matched AST node triggers rule evaluation.

| Capability | How It Works | Where Defined |
|---|---|---|
| Simple AST node kind match | `on_node` like `"unsafe_block"` matches every `unsafe_block` syntax node. | `src/rules/compiler.rs:19-35` |
| Multi-kind OR match | `on_node` uses pipe: `"function_item\|function_declaration"` → query `[(function_item) (function_declaration)] @node`. | `src/rules/ir.rs:12-16` `AstQuery`, `compiler.rs:25-35` |
| Tree-sitter S-expression query | `on_node` is a full S-expression (contains `(` or space): e.g. `(call_expression function: (field_expression field: (field_identifier) @fn_name (#eq? @fn_name "unwrap"))) @node`. | `compiler.rs:19-23`, field `use_query: bool` |
| Explicit `use_query: true` | Forces interpretation as a tree-sitter query regardless of content. | `CoreRule.use_query` → `CoreRuleIr.use_query` |
| Body subtree tree-sitter query | `body_query` field: runs a standalone tree-sitter query against the `body` child of the matched node. | `CoreRuleIr.body_query`, `ir.rs:370-388` |
| `target_kinds` filter (non-query mode) | When `use_query=false`, filters by `node.kind()` against `target_kinds` derived from `on_node`. | `ir.rs:144-148` |

## 2. Regex Content Matching (text-level)
**Mechanism:** Regex patterns applied to the text of the matched node.

| Capability | Field Name | Behavior | Where |
|---|---|---|---|
| Node text must match | `if_matches: <regex>` | Node text must match the regex; if it doesn't, the node is skipped entirely. If it does and no other check remains, an advisory is emitted. | `CoreRule.if_matches` → `CoreRuleIr.if_matches`, `ir.rs:208-254` |
| Node text must contain | `must_contain: <regex>` | Node text must match the regex; failure produces advisory "Pattern was expected but not found." | `CoreRule.must_contain` → `CoreRuleIr.must_contain` |
| Node text must NOT contain | `must_not_contain: <regex>` | Walks reachable paths in the node's body; if any reachable path matches, produces "Prohibited pattern was found." | `CoreRule.must_not_contain` → `CoreRuleIr.must_not_contain` |
| Body must contain (reachability-aware) | `body_must_contain: <regex>` | Uses `ReachabilityChecker`; if no reachable path in the body contains the pattern (and delegation is not detected via `body_may_delegate_via`), produces advisory. | `CoreRule.body_must_contain` → `CoreRuleIr.body_must_contain` |
| Body must contain ANY OF | `body_must_contain_any_of: <regex>` | If ANY reachable path matches, and not bypassed by `must_not_contain`, advisory is raised. | `CoreRule.body_must_contain_any_of` → `CoreRuleIr.body_must_contain_any_of` |
| Body may delegate | `body_may_delegate_via: <regex>` | If delegation regex matches the body text, `body_must_contain` check is suppressed (delegation acknowledged). | `CoreRule.body_may_delegate_via` → `CoreRuleIr.body_may_delegate_via` |
| Node name must match (structural) | `if_name_matches: <regex>` | Looks up the name child (or first identifier child) of the node; if it doesn't match, the node is skipped. | `CoreRule.if_name_matches` → `CoreRuleIr.if_name_matches` |
| Sibling must be preceded by | `must_be_preceded_by: <node_kind>` | Walks previous siblings (skipping comments); if no sibling of the given AST kind is found, advisory fires. | `CoreRule.must_be_preceded_by` → `CoreRuleIr.must_be_preceded_by` |

## 3. Scope / Context Constraints
**Mechanism:** Checks about the node's surroundings — parent AST nodes, file path, special language constructs.

| Capability | Field / Mechanism | Behavior | Where |
|---|---|---|---|
| Exclude scope (file/ancestor regex) | `exclude_scope: <regex>` | If the file path OR any ancestor node's source text matches, the node is skipped entirely. | `CoreRule.exclude_scope` → `CoreRuleIr.exclude_scope`, `ir.rs:88-101` |
| Exclude `#[cfg(test)]` mod blocks | Hardcoded AST walk | Walks ancestors for `mod_item`; checks if sibling attribute is `#[cfg(test)]` or `#[test]`; if so, skip. | `ir.rs:104-127` |
| Skip if parent kind matches | `skip_if_parent: <node_kind>` | If the node's immediate parent has the given AST kind, the node is skipped. | `CoreRule.skip_if_parent` → `CoreRuleIr.skip_if_parent` |
| Must be within scope | `within_scope: <node_kind_regex>` → `FlowConstraint::ScopeConstraint { invert: false }` | Walks ancestors; if any ancestor's `node.kind()` matches the regex, the constraint fires (produces advisory). | `CoreRule.within_scope` → `compiler.rs:53-59`, `ir.rs:567-585` |
| Must be outside scope | `outside_scope: <node_kind_regex>` → `FlowConstraint::ScopeConstraint { invert: true }` | Same ancestor walk; fires if NO ancestor matches. | `CoreRule.outside_scope` → `compiler.rs:62-69`, `ir.rs:567-585` |

## 4. Metric / Size Constraints
**Mechanism:** Quantitative measurements on the matched node.

| Capability | Field | Behavior | Where |
|---|---|---|---|
| Max lines per function/node | `max_lines: <usize>` | If `node.end_position.row - node.start_position.row + 1 > max`, advisory raised. | `CoreRule.max_lines` → `CoreRuleIr.max_lines`, `ir.rs:420-429` |
| Max nesting depth | `max_depth: <usize>` | Counts ancestor control-flow depth (if/for/while/loop/match/switch/catch/block), if > max, advisory raised. | `CoreRule.max_depth` → `CoreRuleIr.max_depth` |
| Max file lines | `max_file_lines: <usize>` | In `file_check()`: if `source_code.lines().count() > max`, advisory raised at line 1. | `CoreRule.max_file_lines` → `CoreRuleIr.max_file_lines`, `ir.rs:166-197` |

## 5. Reachability Analysis
**Mechanism:** Structural AST walk that handles conditional dead branches.

| Capability | Description | Where |
|---|---|---|
| Reachable path pattern search | `ReachabilityChecker::any_reachable_path_contains(body, pattern)` — walks the AST but prunes dead branches of `if` statements when the condition is a constant (true/false/0/1). Skips comments. Stops at return/throw (siblings after are dead). | `src/semantics/reachability.rs` |
| Constant condition evaluation | Evaluates `if` conditions like `"true"`, `"false"`, `"0"`, `"1"`, `"!true"`, `"!false"` to determine dead branches. | `reachability.rs:102-111` |

## 6. Taint / Data Flow Analysis
**Mechanism:** Inter-procedural taint tracking from source patterns to sink patterns, triggered by `FlowConstraint::TaintReached` / `TaintForbidden`.

### 6.1 Semantic Operations Layer (Normalization Layer)
The engine decouples from raw AST by extracting `SemanticOp` values:

| Capability | Op | Description | Where |
|---|---|---|---|
| Variable binding extraction | `SemanticOp::Binding { name, value_range }` | `let x = ...` (Rust), `const x = ...` / `let x = ...` (TS) | `normalization.rs:23-25` |
| Assignment extraction | `SemanticOp::Assignment { target, value_range }` | `x = ...` | `normalization.rs:26-27` |
| Function/method call extraction | `SemanticOp::Call { function_name, args, range }` | Includes method calls (receiver appended as first arg), macro invocations. | `normalization.rs:28-33` |
| Block entry | `SemanticOp::EnterBlock(range)` | Function bodies, if/else blocks, etc. Enables recursion. | `normalization.rs:34-35` |
| Language support | Rust, TypeScript, JavaScript, TSX, JSX | `SemanticExtractor::extract` dispatches by file extension. | `normalization.rs:43-51` |
| Macro argument extraction | Rust `macro_invocation` handling | Extracts identifiers from `token_tree` for taint tracking. | `normalization.rs:263-303` |

### 6.2 Taint Registry (Variable-Level Taint Tracking)

| Capability | Description | Where |
|---|---|---|
| Variable taint | `taint(var, origin)` / `get_origin(var)` — scoped, last-wins | `TaintRegistry`, `mod.rs:110-130` |
| Field taint | `taint_field(var, field, origin)` / `get_field_origin(var, field)` — tracks `obj.prop` taint separately | `mod.rs:82-108` |
| Any field taint | `get_any_field_origin(var)` — returns taint if any field of `var` is tainted | `mod.rs:99-108` |
| Scope push/pop | `push_scope()` / `pop_scope()` — handles block scoping | `mod.rs:67-78` |
| Symbol registration | `register_symbol(name, node)` — maps variable name to definition AST node | `mod.rs:117-119` |
| Taint origins | `TaintOrigin::UserInput`, `Environment`, `Database`, `Network`, `FileSystem`, `Custom(String)` | `mod.rs:14-21` |

### 6.3 Data Flow Analyzer (Inter-Procedural Engine)

| Capability | Description | Where |
|---|---|---|
| Symbol discovery | Discovers all bindings in scope from `SemanticOp::Binding` ops | `tracking.rs:10-18` |
| Intra-procedural analysis | Processes bindings, assignments, calls, and block entries within a function | `tracking.rs:27-101` |
| Inter-procedural (cross-function) analysis | Follows call definitions: `find_definition` → `map_params` → create sub-analyzer on callee body | `tracking.rs:288-323`, `tracking.rs:530-577` |
| Cross-file analysis | `DataFlowAnalyzer::find_definition` looks up symbols across files via `SymbolRegistry`, also checks `file_trees` for other files | `lookup.rs:60-98` |
| Recursive depth limit | Configurable `taint_max_depth` (default 5) to prevent infinite recursion | `mod.rs:169-173` |
| Method chain taint resolution | Checks receiver of method calls for taint before resolving arguments | `tracking.rs:468-482` |
| Object literal taint propagation | For `object`/`object_expression`/`struct_expression` values, propagates taint to fields; handles spread elements | `tracking.rs:328-387` |
| Return value taint | Follows callee body return statements to determine return value taint | `tracking.rs:564-576` |
| Visited-set cycle detection | Prevents re-analysis of the same callee during cross-function taint resolution | `tracking.rs:534-538` |
| Taint result caching | Per-constraint caching via `taint_cache: HashMap` on `GenSenseContext` to avoid re-analyzing | `ir.rs:482-516` |
| Sink detection and reporting | If a tainted argument reaches a function/text matching `sink_re`, produces "Inter-procedural Leak" advisory | `tracking.rs:268-283` |

### 6.4 Taint Flow Constraints (from IR)

| Capability | `FlowConstraint` Variant | Description | Where |
|---|---|---|---|
| Taint Reached | `TaintReached { source: Regex, sink: Regex }` | Asserts that data DOES flow from a source pattern to a sink pattern. If flow is found, advisory fires. | `ir.rs:22`, `compiler.rs:37-42` |
| Taint Forbidden | `TaintForbidden { source: Regex, sink: Regex }` | Asserts that data NEVER flows from source to sink. If flow is found, advisory fires (different label). | `ir.rs:24`, `compiler.rs:44-51` |

## 7. Temporal Sequence Analysis
**Mechanism:** Event ordering within a function scope, gated behind `#[cfg(feature = "temporal")]`.

### 7.1 Event Extraction

| Event Type | Triggered By | Label | Where |
|---|---|---|---|
| Acquire | Calls to `lock`, `try_lock`, `acquire`, `wait` | The function name | `events.rs:105-106` |
| Release | Calls to `unlock`, `release`, `drop`, `signal` | The function name | `events.rs:108-109` |
| Call | Any other `call_expression` or `macro_invocation` | The function name (normalized) | `events.rs:111` |
| Assignment | `variable_declarator`, `assignment_expression`, `let_declaration` | The assigned variable name | `events.rs:117-127` |
| Await | `await_expression` | `".await"` | `events.rs:128-131` |
| Return | `return_statement` | `"return"` | `events.rs:132-135` |

### 7.2 Event Graph Structure

| Edge Kind | Between | Meaning | Where |
|---|---|---|---|
| `SequentiallyFollows` | Consecutive events in the same scope | Ordering of events | `events.rs:33-39` |
| `InScope` | Function symbol → event | Event occurs within a function's body | `events.rs:154-178` |
| `FlowsFrom` | Symbol/variable → event | Value flows into/out of an event | `events.rs:209-277` |

### 7.3 Temporal Behaviors

| Behavior | Meaning | Where |
|---|---|---|
| `MustNotFollow` | After event A is seen, event B must NOT appear before a Release event. | `analyzer.rs:50-114` |
| `MustFollow` | A given sequence of events must be completed (all steps matched). If partial match, advisory fires. | `analyzer.rs:116-149` |
| `ForbiddenBetween(start, end)` | Events matching the sequence patterns are forbidden between a start event and an end event. | `analyzer.rs:151-210` |

### 7.4 Temporal YAML Configuration

| Field | Description | Where |
|---|---|---|
| `temporal.sequence` | `Vec<String>` — ordered list of regex patterns matching event labels | `config.rs:7` |
| `temporal.behavior` | `"must_not_follow"`, `"must_follow"`, or `"forbidden_between"` | `config.rs:8`, `handler.rs:23-35` |

## 8. Project-Level Static Checks (Cross-File)
**Mechanism:** Run once over the entire project symbol graph via `ProjectRuleIr`.

| Constraint | Fields | What It Checks | Where |
|---|---|---|---|
| `MustHaveGuard` | `source_pattern`, `guard_pattern`, `source_glob`, `guard_glob` | Every symbol matching `source_pattern` within files matching `source_glob` must have a call-path reachable guard (a symbol matching `guard_pattern` in `guard_glob`). | `ir.rs:693-699`, `ir.rs:850-903` |
| `MustBeInternal` | `pattern`, `file_glob` | Symbols matching `pattern` must only be called from within the same file or from files matching `file_glob`. | `ir.rs:700-703`, `ir.rs:905-953` |
| `CrossFileTaintFree` | `source_pattern`, `sink_pattern` | If a call-path exists from a source-matching symbol to a sink-matching symbol in a different file, advisory fires. | `ir.rs:704-707`, `ir.rs:955-1000` |
| `GlobalDataFlow` | `source_pattern`, `sink_pattern` | If a call-path exists from any source symbol to any sink symbol (same or different file), advisory fires. | `ir.rs:708-711`, `ir.rs:1002-1048` |
| `SchemaContract` | `source_capture_re`, `source_file_glob`, `schema_type`, `schema_file_glob`, `schema_extract` | Matches `source_capture_re` against source files; validates the captured name exists in the schema (model names, field names, or enum values). | `ir.rs:712-718`, `ir.rs:1050-1135` |

## 9. Schema Contract Validation
**Mechanism:** Validates that source-code identifiers match a schema definition.

| Schema Type | Extract Type | What It Extracts | Where |
|---|---|---|---|
| Prisma | `ModelNames` | All model names from `*.prisma` files | `prisma_extractor.rs:126-143` |
| Prisma | `FieldNames` | All field names inside all models | `prisma_extractor.rs:147-164` |
| Prisma | `EnumValues` | All values inside all enum blocks | `prisma_extractor.rs:168-185` |
| OpenApi | — | Placeholder — deserializable but no extractor implemented | `ir.rs:689` |

The contract check: `source_capture_re` regex capture group 1 is extracted from source files; if not found in the valid schema set, advisory fires.

## 10. Symbol Graph & Graph Database
**Mechanism:** A Petgraph directed graph with typed nodes and edges.

### 10.1 Node Types

| Node Type | Description | Where |
|---|---|---|
| `SemanticNode::Declaration(Symbol)` | A declared symbol (function, struct, class, etc.) | `graph.rs:43-45` |
| `SemanticNode::Event(TemporalEvent)` | A temporal event (acquire, release, call, etc.) | `graph.rs:44` |

### 10.2 SymbolKind Variants

| Kind | Meaning |
|---|---|
| `Function` | Function, method |
| `Struct` | Struct or record |
| `Class` | Class (TS/JS) |
| `Interface` | Interface (TS) |
| `Enum` | Enumeration |
| `Constant` | Named constant |
| `Module` | Module/namespace |
| `Variable` | Mutable variable |
| `Parameter` | Function parameter |
| `Unknown` | Unresolved |

### 10.3 Edge Kinds

| Edge Kind | Direction | Meaning | Where |
|---|---|---|---|
| `Calls` | Caller → Callee | Function A calls function B | `graph.rs:11`, `symbols.rs:149` |
| `RefersTo` | (declared) | Reference relationship | `graph.rs:12` |
| `OwnedBy` | Child → Parent | Ownership/membership | `graph.rs:13` |
| `Inherits` | Child → Parent | Inheritance relationship | `graph.rs:14` |
| `Overrides` | Overrider → Base | Method/function override | `graph.rs:15` |
| `FlowsFrom` | Source → Target | Data flows from one node to another | `graph.rs:16`, `events.rs:210-277` |
| `SequentiallyFollows` | Earlier → Later | Event ordering in execution flow | `graph.rs:17`, `events.rs:33-39` |
| `InScope` | Scope (function) → Event | Event belongs to a scope | `graph.rs:18`, `events.rs:154-178` |
| `Parameter` | Function → Parameter | Parameter binding | `graph.rs:19`, `events.rs:263-265` |
| `TaintFlow` | Function → itself | Taint analysis found a source→sink path in this function | `graph.rs:22`, `ir.rs:246-260` |

### 10.4 Taint Flow Records

| Method | Purpose | Where |
|---|---|---|
| `record_taint_flow(record)` | Records a taint finding, materializes a `TaintFlow` edge | `graph.rs:244-260` |
| `taint_flows()` | Returns all recorded `TaintFlowRecord` entries | `graph.rs:263-265` |
| `has_taint_flow(func, file)` | Check if a function has any taint flow | `graph.rs:268-270` |
| `taint_flows_for(func, file)` | Get all taint flows for a function | `graph.rs:273-275` |

### 10.5 Graph Query Methods

| Method | Purpose | Where |
|---|---|---|
| `find_nodes(name)` | Lookup all nodes by name index | `graph.rs:82-90` |
| `has_call_path(from, to)` | DFS across `Calls` edges | `graph.rs:228-253` |
| `neighbors_of(id, kind)` | Outgoing neighbors by edge kind | `graph.rs:137-143` |
| `incoming_neighbors_of(id, kind)` | Incoming neighbors by edge kind | `graph.rs:146-152` |
| `ordered_events_in_scope(scope)` | Topologically sorted events for a function scope | `graph.rs:155-225` |

## 11. Automated Remediation (Auto-Fix)

| Capability | Field | Behavior | Where |
|---|---|---|---|
| Regex-based find-and-replace | `fix_pattern` + `fix_with` | If `if_matches` succeeds AND `fix_re.captures(code)` succeeds, the node text is replaced using `fix_re.replace_all(code, template)`; a new `remediated_advisory` is produced with `proposed_replacement`. | `ir.rs:213-238`, `compiler.rs:82-89` |
| Import injection | `inject_import` | A template expanded with regex captures; placed into `Advisory::proposed_import`. | `ir.rs:221-228`, `CoreRule.inject_import` |
| Auto-fixable flag | `auto_fixable: bool` | Propagated to advisory; default `true` when remediation data is present. | `CoreRule.auto_fixable`, `ir.rs:676` |
| Requires-human flag | `requires_human: bool` | Propagated to advisory. | `CoreRule.requires_human`, `ir.rs:677` |

## 12. Consistency Verification

| Capability | Description | Where |
|---|---|---|
| AST-vs-Graph consistency check | Compares advisories produced directly from AST against advisories produced via the graph path; detects divergence (missing or extra findings). | `consistency.rs:7-62` |
| Analysis artifact caching | `AnalysisRegistry` caches computed results per `(rule_id, scope_id)` key, with type-safe retrieval. | `registry.rs:10-46` |

## 13. Detection Capabilities Index (Quick Reference)

### Per-Node Checks (CoreRule / CoreRuleIr)

| # | Capability | YAML Field |
|---|---|---|
| 1 | AST S-expression match | `on_node` (as query) / `use_query: true` |
| 2 | AST node kind match | `on_node: "kind"` |
| 3 | Multi-kind OR match | `on_node: "kind1\|kind2"` |
| 4 | Node text regex match | `if_matches` |
| 5 | Body regex must exist | `must_contain` |
| 6 | Body regex must NOT exist | `must_not_contain` |
| 7 | Reachable path must contain | `body_must_contain` |
| 8 | Reachable path must contain any of | `body_must_contain_any_of` |
| 9 | Delegation bypass for body check | `body_may_delegate_via` |
| 10 | Node name must match | `if_name_matches` |
| 11 | Preceding sibling must be kind | `must_be_preceded_by` |
| 12 | Max lines | `max_lines` |
| 13 | Max nesting depth | `max_depth` |
| 14 | Max file lines | `max_file_lines` |
| 15 | Exclude scope (file/ancestor regex) | `exclude_scope` |
| 16 | Skip if parent kind | `skip_if_parent` |
| 17 | Within ancestor scope | `within_scope` |
| 18 | Outside of ancestor scope | `outside_scope` |
| 19 | Body subtree tree-sitter query | `body_query` |
| 20 | Taint reached (source→sink) | `source_pattern` + `sink_pattern` |
| 21 | Taint forbidden (source¬→sink) | `forbidden_source_pattern` + `forbidden_sink_pattern` |
| 22 | Temporal sequence must follow | `temporal.sequence` + `behavior: must_follow` |
| 23 | Temporal sequence must NOT follow | `temporal.sequence` + `behavior: must_not_follow` |
| 24 | Temporal forbidden between | `temporal.sequence[0..=1]` + `behavior: forbidden_between` |
| 25 | Regex auto-fix | `fix_pattern` + `fix_with` |
| 26 | Import injection | `inject_import` |
| 27 | Configurable taint depth | `taint_max_depth` |
| 28 | Composite: all sub-constraints must match | `all_of` |
| 29 | Composite: at least one sub-constraint matches | `any_of` |
| 30 | Composite: negates a sub-constraint | `not` |
| 31 | Composite: primary matches but exclusion doesn't | `without_constraint` + `without_exclusion` |
| 32 | Composite: taint path must cross a boundary | `across_boundary` (wraps `forbidden_source_pattern`/`forbidden_sink_pattern`) |

### Project-Level Checks (ProjectRuleIr / ProjectCoreRule)

| # | Capability | YAML Block |
|---|---|---|
| 33 | Every source symbol must have a guard call-path | `must_have_guard` |
| 34 | Symbol must be internal (same file / allowed glob) | `must_be_internal` |
| 35 | Cross-file taint must be free | `cross_file_taint_free` |
| 36 | Global data flow (any file) | `global_data_flow` |
| 37 | Schema contract validation | `schema_type`, `schema_glob`, `schema_extract`, `source_pattern` |

### Semantic Infrastructure (Supporting, not directly declarable)

| # | Capability | Module |
|---|---|---|
| 38 | Variable taint tracking (scoped, inter-procedural) | `data_flow/tracking.rs` |
| 39 | Field-level taint tracking | `data_flow/mod.rs:82-95` |
| 40 | Cross-file definition resolution | `data_flow/lookup.rs` |
| 41 | Method chain taint resolution | `data_flow/tracking.rs:468-482` |
| 42 | Return value taint propagation | `data_flow/tracking.rs:564-576` |
| 43 | Reachability-aware pattern search | `reachability.rs` |
| 44 | Event extraction (Acquire/Release/Call/Assignment/Await/Return) | `events.rs` |
| 45 | Event ordering graph (`SequentiallyFollows`) | `graph.rs` |
| 46 | Call graph traversal (`Calls` edges) | `graph.rs`, `symbols.rs` |
| 47 | Symbol query by name, location, and regex | `symbols.rs` |
| 48 | Consistency verification (AST vs Graph) | `consistency.rs` |
| 49 | Analyzed artifact caching | `registry.rs` |
| 50 | Prisma model/field/enum extraction | `prisma_extractor.rs` |
| 51 | Taint origin taxonomy (5 built-in + custom) | `data_flow/mod.rs:14-21` |
| 52 | Visited-set cycle detection | `data_flow/tracking.rs:534-538` |
| 53 | Match-arm and if-expression return propagation | `data_flow/tracking.rs:621-637` |
| 54 | `SemanticGraph` exposed in `GenSenseContext` for rule-level graph queries | `lib.rs:219`, `auditor/mod.rs` |
| 55 | Taint flow materialized as queryable `TaintFlow` graph edges | `graph.rs:20-22`, `project/mod.rs:654-667` |
| 56 | `FlowEvaluator` — recursive algebraic constraint tree evaluation | `ir.rs:1203-1345` |
| 57 | Graph exposed to `ProjectRule` via `SymbolRegistry.graph()` | `symbols.rs:47-49` |

## Language-Specific Event Classification

| Event Type | Recognized Function Names |
|---|---|
| Acquire | `lock`, `try_lock`, `acquire`, `wait` |
| Release | `unlock`, `release`, `drop`, `signal` |
| Call | Everything else (normalized, `!` suffix stripped) |
| Assignment | All variable declarations and assignments |
| Await | `.await` expressions |
| Return | `return` statements |

## Control-Flow Depth Tracked Kinds

| Category | AST Kinds |
|---|---|
| Rust | `if_expression`, `for_expression`, `while_expression`, `loop_expression`, `match_expression`, `match_arm` |
| General | `if_statement`, `for_statement`, `for_in_statement`, `while_statement`, `do_statement`, `switch_statement`, `catch_clause`, `block`, `compound_statement` |
