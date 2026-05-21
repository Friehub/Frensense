Where it's weaker than it looks
The implementation has some meaningful limitations worth knowing:
The CSA rules use regex against the function body text, not actual control-flow analysis. So TS_CSA_VALIDATE_UNCONDITIONAL flags any validate_ function that doesn't contain the string throw or return false anywhere in its body — meaning a function with those strings in a dead branch or a comment would pass. It's a heuristic, not a proof.
The AI pattern detectors (tautological assert, useless test, dead result) are similarly string-match based. AI_TAUTOLOGICAL_ASSERT only catches assert!(true) and assert_eq!(1, 1) literally — it won't catch assert_eq!(x, x) or any non-trivial tautology.
The taint analysis is intra-procedural by default and the inter-procedural path (following taint into called functions) only works when the callee is defined in the same file. It also doesn't model aliasing or struct field propagation.


The gap to close
The FUTURE_ENHANCEMENTS_PLAN.md is candid about what's missing: no LSP server (so no editor integration), no SARIF output yet (blocking native GitHub PR annotations), confidence scoring not yet attached to findings, and the --fix mode has a known bug where original_content is empty for project-level advisories. Those are real gaps for production CI adoption, but all look tractable.
Overall it's a genuinely useful tool for its niche — especially for Rust/TypeScript codebases that generate a lot of AI-assisted code — but you should treat the CSA/taint findings as high-signal hints to investigate rather than definitive proof of a bug.




Fix 1 — Replace body_must_contain regex with a reachability walk
Where the problem lives: src/rules/core/mod.rs, the body_must_contain branch, and src/rules/definitions/csa.yml.
What to build: A ReachabilityChecker that walks the tree-sitter AST of a function body and returns the set of node kinds that appear on at least one path that can actually execute. Then the CSA rule checks that set instead of running a regex on raw text.
rust// src/semantics/reachability.rs  (new file)

use tree_sitter::Node;

pub struct ReachabilityChecker<'a> {
    source: &'a str,
}

impl<'a> ReachabilityChecker<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Returns true if at least one reachable path through `body`
    /// contains a node whose text matches `pattern`.
    pub fn any_reachable_path_contains(
        &self,
        body: Node<'a>,
        pattern: &regex::Regex,
    ) -> bool {
        self.walk_reachable(body, pattern, false)
    }

    fn walk_reachable(
        &self,
        node: Node<'a>,
        pattern: &regex::Regex,
        inside_dead_branch: bool,
    ) -> bool {
        // Prune known-dead branches
        let dead = inside_dead_branch || self.is_dead_branch(node);
        if dead {
            return false;
        }

        let text = &self.source[node.start_byte()..node.end_byte()];
        if pattern.is_match(text) {
            return true;
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if self.walk_reachable(child, pattern, dead) {
                return true;
            }
        }
        false
    }

    fn is_dead_branch(&self, node: Node) -> bool {
        // Detect `if (false) { ... }` and `if (true) { ... } else { ... }`
        if node.kind() == "if_statement" || node.kind() == "if_expression" {
            if let Some(cond) = node.child_by_field_name("condition") {
                let cond_text = &self.source[cond.start_byte()..cond.end_byte()];
                // Literal false condition — the consequence is dead
                if cond_text.trim() == "false" || cond_text.trim() == "0" {
                    return true;
                }
            }
        }
        // Detect unreachable code after unconditional return/panic
        // (handled by the caller tracking whether a Return event has been seen)
        false
    }
}
Then change the check in CoreRule:
rust// src/rules/core/mod.rs — replace the body_must_contain block

if let Some(re) = &self.body_must_contain {
    let body_node = node.child_by_field_name("body").unwrap_or(node);
    
    // Before: regex on raw text
    // let body_code = &context.source_code[body_node.start_byte()..body_node.end_byte()];
    // if !re.is_match(body_code) { ... }

    // After: reachability-aware walk
    let checker = crate::semantics::reachability::ReachabilityChecker::new(context.source_code);
    if !checker.any_reachable_path_contains(body_node, re) {
        advisories.push(self.new_advisory(
            &node,
            context,
            format!("Function body has no reachable path containing '{}'.", re.as_str()),
        ));
    }
}
This eliminates both false negative classes: strings in comments (comments are not AST expression nodes, so they won't match in the walk), and strings in dead if (false) branches (pruned by is_dead_branch). It costs one extra tree traversal per CSA-matched function, which is negligible since CSA rules only trigger on name-matched functions.

Fix 2 — Teach body_must_contain about delegation patterns
The false positive case — a correct validator that delegates to a library — needs a complementary DSL extension. Add a body_may_delegate_via field to CoreRule:
yaml# csa.yml
- id: "TS_CSA_VALIDATE_UNCONDITIONAL"
  if_name_matches: "(validate|verify|check)"
  body_must_contain: "return\\s+(false|null|undefined)|throw|Error"
  body_may_delegate_via: "safeParse|validate|verify|check|assert"
  # If the body contains a call to any of these, suppress the finding
  # even if body_must_contain doesn't match — the callee is assumed to handle rejection
In CoreRule::check:
rust// After body_must_contain fires a finding, check for delegation before emitting
if let Some(delegation_re) = &self.body_may_delegate_via {
    let body_code = &context.source_code[body_node.start_byte()..body_node.end_byte()];
    if delegation_re.is_match(body_code) {
        // Likely delegates rejection to a library — downgrade to Info, don't emit Critical
        continue; // or push Info-level advisory instead
    }
}
This directly addresses the schema.safeParse false positive and any similar pattern where a function wraps a library that handles the rejection contract.

Fix 3 — Promote AI pattern detectors from string matching to AST argument evaluation
TautologicalAssert — currently checks two hardcoded strings. Replace with argument node comparison:
rust// src/rules/global/ai_patterns/tautological_assert.rs

fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
    let mut advisories = Vec::new();

    let Some(macro_name_node) = node.child(0) else { return advisories };
    let macro_name = &context.source_code[macro_name_node.start_byte()..macro_name_node.end_byte()];

    if !matches!(macro_name, "assert" | "assert_eq" | "assert_ne") {
        return advisories;
    }

    // Find the token_tree (argument list) node
    let Some(token_tree) = node.child_by_field_name("token_tree")
        .or_else(|| node.children(&mut node.walk()).find(|c| c.kind() == "token_tree"))
    else { return advisories };

    // Collect non-punctuation argument nodes
    let args: Vec<Node> = token_tree.children(&mut token_tree.walk())
        .filter(|c| !matches!(c.kind(), "," | "!" | "(" | ")"))
        .collect();

    let is_tautology = match macro_name {
        "assert" => {
            // assert!(true) or assert!(false) — both are tautological (one always passes, one always fails)
            if let Some(arg) = args.first() {
                let text = &context.source_code[arg.start_byte()..arg.end_byte()].trim();
                *text == "true" || *text == "false"
            } else { false }
        }
        "assert_eq" | "assert_ne" => {
            if args.len() >= 2 {
                let lhs = &context.source_code[args[0].start_byte()..args[0].end_byte()].trim();
                let rhs = &context.source_code[args[1].start_byte()..args[1].end_byte()].trim();
                // Same text on both sides — assert_eq!(x, x) or assert_ne!(1, 1)
                lhs == rhs
            } else { false }
        }
        _ => false,
    };

    if is_tautology {
        advisories.push(self.new_advisory(&node, context,
            "Tautological assertion: both sides are identical or the condition is a literal.".to_string(),
        ));
    }

    advisories
}
This catches assert_eq!(x, x), assert_eq!(1, 1), assert!(true), and assert_ne!(x, x) — all patterns the current version misses — while remaining immune to false positives like assert_eq!(result, expected_value) because the two argument texts differ.
PlaceholderPanic — invert the filter entirely:
rust// src/rules/global/ai_patterns/placeholder_panic.rs

fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
    let mut advisories = Vec::new();
    let Some(macro_name_node) = node.child(0) else { return advisories };
    let macro_name = &context.source_code[macro_name_node.start_byte()..macro_name_node.end_byte()];

    if matches!(macro_name, "todo" | "unimplemented") {
        // Fire on ALL todo!/unimplemented! — no message filter
        // A bare todo!() is more dangerous than a documented one, not less
        advisories.push(self.new_advisory(&node, context,
            "Placeholder macro detected — this will panic unconditionally at runtime.".to_string(),
        ));
    }

    advisories
}
The suppression mechanism (.gensense-suppress.yml) already exists for cases where a todo!() is intentional. Let developers use that rather than relying on message content to gate the rule.
TsFloatingPromise — check parent node kind instead of string contents:
rust// src/rules/global/ai_patterns/ts_floating_promise.rs

fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
    let mut advisories = Vec::new();
    let code = &context.source_code[node.start_byte()..node.end_byte()];

    let promise_sinks = ["fetch(", "prisma.", "axios.", "db.", "supabase."];
    if !promise_sinks.iter().any(|&sink| code.contains(sink)) {
        return advisories;
    }

    // Instead of string-matching "let" or "=", check the AST parent node kind
    let is_handled = node.parent().map(|p| matches!(
        p.kind(),
        "await_expression"       // await fetch(...)
        | "return_statement"     // return fetch(...)
        | "variable_declarator"  // const x = fetch(...)
        | "assignment_expression"// x = fetch(...)
        | "lexical_declaration"  // let/const at declaration level
        | "arguments"            // passed as arg to another call — caller's responsibility
    )).unwrap_or(false);

    if !is_handled {
        advisories.push(self.new_advisory(&node, context,
            "Floating Promise: promise-returning call is not awaited, returned, or assigned.".to_string(),
        ));
    }

    advisories
}
The parent node kind is a structural fact, not a text scan. A const keyword in a comment or adjacent line can't interfere with it.

Fix 4 — Add a cross-file AST cache for inter-procedural taint
The structural problem: find_definition in src/semantics/data_flow/lookup.rs calls self.root.descendant_for_byte_range(...) — but self.root is the current file's tree. Definitions in other files are in the SymbolRegistry but their trees aren't accessible.
The fix: Add a TreeCache to GenSenseContext — a map of file path to (tree, source) pairs, populated during the multi-file discovery pass that already runs. The engine already loads all files; it just discards the trees after discovery.
rust// src/lib.rs — extend GenSenseContext

pub struct GenSenseContext<'a> {
    pub file_id: FileId,
    pub file_path: &'a Path,
    pub source_code: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub symbols: &'a SymbolRegistry,
    pub semantic_ops: &'a [crate::semantics::data_flow::normalization::SemanticOp],
    pub taint_cache: &'a TaintCache,
    // NEW: cross-file tree access
    pub file_trees: &'a HashMap<String, (tree_sitter::Tree, String)>,
}
Then in find_definition:
rust// src/semantics/data_flow/lookup.rs

pub fn find_definition(&self, name: &str, registry: &TaintRegistry<'a>) -> Option<Node<'a>> {
    // 1. Local scope (unchanged)
    if let Some(node) = registry.find_symbol(name) {
        return Some(node);
    }

    // 2. Current file symbols (unchanged)
    let file_path = self.context.file_path.to_string_lossy();
    let line = self.root.start_position().row + 1;
    if let Some(sym) = self.context.symbols.find_at(name, &file_path, line) {
        return self.root.descendant_for_byte_range(sym.start_byte, sym.end_byte);
    }

    // 3. NEW: Cross-file lookup
    let all_matches = self.context.symbols.find(name);
    for sym in all_matches {
        if sym.file_path == file_path.as_ref() { continue; } // already checked
        if let Some((tree, _src)) = self.context.file_trees.get(&sym.file_path) {
            if let Some(node) = tree.root_node()
                .descendant_for_byte_range(sym.start_byte, sym.end_byte) 
            {
                return Some(node);
            }
        }
    }

    None
}
This is the minimal change needed for cross-file taint. The trees are already parsed — the cost is retaining them in memory rather than dropping them after the discovery pass. For most codebases this is fine; for very large projects you'd want an LRU cache capped at N files.

Fix 5 — Add field-path taint propagation for struct/object aliasing
The TaintRegistry today maps var_name -> origin. Extend it to (var_name, field_path) -> origin:
rust// src/semantics/data_flow/mod.rs

#[derive(Debug, Clone)]
pub struct TaintRegistry<'a> {
    pub scopes: Vec<HashMap<&'a str, &'a str>>,
    pub symbols: Vec<HashMap<&'a str, Node<'a>>>,
    // NEW: field-path taint — key is (var, field), value is origin
    pub field_taint: Vec<HashMap<(&'a str, &'a str), &'a str>>,
}

impl<'a> TaintRegistry<'a> {
    pub fn taint_field(&mut self, var: &'a str, field: &'a str, origin: &'a str) {
        if let Some(scope) = self.field_taint.last_mut() {
            scope.insert((var, field), origin);
        }
    }

    pub fn get_field_origin(&self, var: &str, field: &str) -> Option<&'a str> {
        for scope in self.field_taint.iter().rev() {
            if let Some(origin) = scope.get(&(var, field)) {
                return Some(*origin);
            }
        }
        None
    }
}
Then in resolve_taint in tracking.rs, add a branch for member expression nodes:
rustfn resolve_taint(&self, node: Node<'a>, registry: &TaintRegistry<'a>) -> Option<&'a str> {
    if node.kind() == "identifier" {
        let name = &self.context.source_code[node.start_byte()..node.end_byte()];
        return registry.get_origin(name);
    }

    // NEW: handle member_expression (foo.bar) and field_expression (foo.bar in Rust)
    if node.kind() == "member_expression" || node.kind() == "field_expression" {
        if let (Some(obj), Some(field)) = (
            node.child_by_field_name("object"),
            node.child_by_field_name("field").or_else(|| node.child_by_field_name("property")),
        ) {
            let obj_name = &self.context.source_code[obj.start_byte()..obj.end_byte()];
            let field_name = &self.context.source_code[field.start_byte()..field.end_byte()];
            // Check field-specific taint first, then fall back to whole-object taint
            return registry.get_field_origin(obj_name, field_name)
                .or_else(|| registry.get_origin(obj_name));
        }
    }

    // Original recursive walk (unchanged)
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(origin) = self.resolve_taint(child, registry) {
            return Some(origin);
        }
    }
    None
}
And in analyze_block, when processing an object literal binding, propagate taint per property:
rust// When a Binding's value_range points to an object literal node,
// iterate its properties and taint each field if the value is tainted
SemanticOp::Binding { name, value_range } => {
    let v_node = self.node_at(*value_range);
    if v_node.kind() == "object" || v_node.kind() == "object_expression" {
        let mut cursor = v_node.walk();
        for prop in v_node.children(&mut cursor) {
            if prop.kind() == "pair" || prop.kind() == "shorthand_property_identifier" {
                let key = prop.child_by_field_name("key").or_else(|| Some(prop));
                let val = prop.child_by_field_name("value");
                if let (Some(k), Some(v)) = (key, val) {
                    let key_name = &self.context.source_code[k.start_byte()..k.end_byte()];
                    if let Some(origin) = self.resolve_taint(v, &registry) {
                        registry.taint_field(name, key_name, origin);
                    }
                }
            }
        }
    }
    // existing binding logic...
}

Fix 6 — Add a confidence field to Advisory and populate it
This is the practical expression of the "high-signal hint" problem. Instead of every finding having implicit equal weight, attach a confidence score derived from how the finding was produced:
rust// src/lib.rs

pub struct Advisory {
    // ... existing fields ...
    pub confidence: f32, // 0.0 – 1.0
}
Populate it by rule type at the point of emission:
Detection methodConfidenceExample rulesRegex on raw text0.55Current body_must_containReachability walk (Fix 1)0.75CSA after fixAST argument comparison (Fix 3)0.85TautologicalAssert after fixIntra-file taint, direct path0.90Taint in same functionCross-file taint (Fix 4)0.80Taint across module boundaryTemporal FSA (lock→await)0.92RUST_ASYNC_MUTEX_DEADLOCK
The CLI output and JSON can then surface this, so a CI gate can enforce --strict --min-confidence 0.75 rather than treating a regex heuristic the same as an AST-proven deadlock. This also makes the "high-signal hint vs. proof" distinction machine-readable, which is exactly what the gensense-agent-integration.md research doc was calling for.

Summary of what changes where
FileChangesrc/semantics/reachability.rsNew file — CFG-aware body walker (Fix 1)src/rules/core/mod.rsUse ReachabilityChecker for body_must_contain; add body_may_delegate_via (Fixes 1, 2)src/rules/definitions/csa.ymlAdd body_may_delegate_via to validator rules (Fix 2)src/rules/global/ai_patterns/tautological_assert.rsAST arg comparison (Fix 3)src/rules/global/ai_patterns/placeholder_panic.rsRemove message filter (Fix 3)src/rules/global/ai_patterns/ts_floating_promise.rsParent node kind check (Fix 3)src/lib.rsAdd file_trees to GenSenseContext, add confidence to Advisory (Fixes 4, 6)src/semantics/data_flow/lookup.rsCross-file find_definition (Fix 4)src/semantics/data_flow/mod.rsField-path taint in TaintRegistry (Fix 5)src/semantics/data_flow/tracking.rsmember_expression in resolve_taint, object property taint in analyze_block (Fix 5)
Fixes 1–3 are self-contained and can be shipped independently with no risk to existing passing tests. Fixes 4–5 touch the engine's core data structures and need corresponding test coverage for the cross-file and aliasing cases before merging. Fix 6 is additive and backward-compatible.