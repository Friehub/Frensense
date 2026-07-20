// SPDX-License-Identifier: MIT

//! Cross-file taint verification.
//!
//! Follows taint flow across file boundaries to verify that
//! user-controlled data reaches dangerous sinks through imports and exports.

use std::collections::{HashMap, HashSet};
use tree_sitter::Node;

use crate::semantics::data_flow::TaintOrigin;
use crate::semantics::data_flow::TaintRegistry;
use crate::semantics::symbols::SymbolRegistry;
use frensense_engine::corpus::source_sink::{CorpusSourceSinkRegistry, extract_param_info};
use frensense_engine::data_flow::DataFlowEngine;

/// Result of cross-file taint verification.
#[derive(Debug, Clone)]
pub struct CrossFileResult {
    pub verified: bool,
    pub depth: usize,
    pub path: Vec<String>,
    pub detail: String,
    pub source_name: Option<String>,
    pub sink_name: Option<String>,
}

fn extract_binding_names<'a>(node: Node<'a>, source: &'a str) -> Vec<String> {
    let mut names = Vec::new();
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        match n.kind() {
            "identifier" | "shorthand_property_identifier_pattern" => {
                names.push(source[n.start_byte()..n.end_byte()].to_string());
            }
            "pair_pattern" => {
                if let Some(value) = n.child_by_field_name("value") {
                    stack.push(value);
                }
            }
            "object_pattern" | "array_pattern" | "struct_pattern" | "tuple_pattern" => {
                let mut cursor = n.walk();
                for child in n.children(&mut cursor) {
                    stack.push(child);
                }
            }
            _ => {}
        }
    }
    names
}

/// Cross-file taint verifier.
///
/// Follows taint flow through imports/exports and function calls
/// across multiple files to verify that user-controlled data
/// reaches dangerous sinks.
pub struct CrossFileVerifier<'a> {
    source: &'a str,
    _tree: &'a tree_sitter::Tree,
    _file_path: String,
    registry: TaintRegistry,
    _symbols: &'a SymbolRegistry,
    _data_flow: &'a DataFlowEngine,
    _file_trees: &'a HashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
    _visited: HashSet<(String, usize)>,
    max_depth: usize,
    source_sink: &'a CorpusSourceSinkRegistry,
    deps: &'a std::collections::HashSet<String>,
    pub source_name: Option<String>,
    pub potential_sink_name: Option<String>,
    pub sanitizers: frensense_engine::data_flow::SanitizerRegistry,
    pub propagators: frensense_engine::data_flow::propagators::PropagatorRegistry,
    cfg: Option<frensense_engine::cfg::ControlFlowGraph<'a>>,
}

impl<'a> CrossFileVerifier<'a> {
    #[must_use]
    pub fn new(
        source: &'a str,
        tree: &'a tree_sitter::Tree,
        file_path: &str,
        symbols: &'a SymbolRegistry,
        data_flow: &'a DataFlowEngine,
        file_trees: &'a HashMap<
            String,
            (
                tree_sitter::Tree,
                String,
                Vec<crate::semantics::data_flow::normalization::SemanticOp>,
            ),
        >,
        source_sink: &'a CorpusSourceSinkRegistry,
        deps: &'a std::collections::HashSet<String>,
    ) -> Self {
        Self {
            source,
            _tree: tree,
            _file_path: file_path.to_string(),
            registry: TaintRegistry::default(),
            _symbols: symbols,
            _data_flow: data_flow,
            _file_trees: file_trees,
            _visited: HashSet::new(),
            max_depth: 5,
            source_sink,
            deps,
            source_name: None,
            potential_sink_name: None,
            sanitizers: frensense_engine::data_flow::SanitizerRegistry::default_combined(),
            propagators: frensense_engine::data_flow::propagators::PropagatorRegistry::new(),
            cfg: None,
        }
    }

    #[must_use]
    pub fn with_cfg(mut self, cfg: frensense_engine::cfg::ControlFlowGraph<'a>) -> Self {
        self.cfg = Some(cfg);
        self
    }

    /// Seed taint for a function's parameters.
    ///
    /// Uses corpus-learned source types: taints parameters whose type annotations
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// match types found in positive corpus examples.
    pub fn seed_taint(&mut self, fn_node: Node) {
        self.seed_taint_recursive(fn_node);
        if let Some(body) = fn_node.child_by_field_name("body") {
            crate::semantics::data_flow::corpus_seeder::seed_from_ast_body(
                body,
                self.source,
                &mut self.registry,
            );
        }
    }

    fn seed_taint_recursive(&mut self, node: Node) {
        // Seed parameters of this node if it's a function
        if let Some(params_node) = node
            .child_by_field_name("parameters")
            .or_else(|| node.child_by_field_name("formal_parameters"))
        {
            let mut cursor = params_node.walk();
            for param in params_node.children(&mut cursor) {
                if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
                    continue;
                }

                let (mut param_name, param_type) = extract_param_info(param, self.source);
                if param_name.is_empty() && param.kind() == "identifier" {
                    param_name = self.source[param.start_byte()..param.end_byte()].to_string();
                }
                if param_name.is_empty() {
                    continue;
                }

                let clean_type = param_type.trim_start_matches(':').trim();

                // Determine the taint origin from the corpus-learned type or
                // a heuristic name match for untyped languages (JavaScript, Python, etc.)
                let origin = if self.source_sink.is_source_type(clean_type) {
                    Some(TaintOrigin::UserInput)
                } else {
                    classify_param_origin(&param_name)
                };

                if let Some(origin) = origin {
                    self.registry.taint(&param_name, origin);
                    if self.source_name.is_none() {
                        self.source_name = Some(param_name);
                    }
                }
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                self.seed_taint_recursive(cursor.node());
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Verify that taint flows from parameters to a sink across files.
    pub fn verify_flow(&mut self, fn_node: Node) -> CrossFileResult {
        let Some(body) = fn_node.child_by_field_name("body") else {
            return CrossFileResult {
                verified: false,
                depth: 0,
                path: Vec::new(),
                detail: "No function body".to_string(),
                source_name: None,
                sink_name: None,
            };
        };

        // Check if any parameter is tainted (AST-seeded, not name-based)
        if !self.registry.has_any_tainted() {
            return CrossFileResult {
                verified: false,
                depth: 0,
                path: Vec::new(),
                detail: "No tainted parameters detected".to_string(),
                source_name: None,
                sink_name: None,
            };
        }

        // Follow taint through the function body
        self.follow_taint(body, 0, &mut Vec::new())
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Recursively follow taint through a code block.
    fn follow_taint(
        &mut self,
        node: Node,
        depth: usize,
        path: &mut Vec<String>,
    ) -> CrossFileResult {
        if depth > self.max_depth {
            return CrossFileResult {
                verified: false,
                depth,
                path: path.clone(),
                detail: "Maximum depth reached".to_string(),
                source_name: None,
                sink_name: None,
            };
        }

        let kind = node.kind();

        match kind {
            // Check for sink function calls
            "call_expression" => {
                if let Some(result) = self.check_call_for_sink(node, depth, path) {
                    return result;
                }
            }
            // Track variable assignments
            "variable_declarator" | "lexical_declaration" => {
                if let Some(name_node) = node
                    .child_by_field_name("name")
                    .or_else(|| node.child_by_field_name("pattern"))
                    && let Some(value_node) = node.child_by_field_name("value")
                {
                    let names = extract_binding_names(name_node, self.source);
                    if self.is_node_tainted(value_node) {
                        let mut is_sanitized = false;
                        if value_node.kind() == "call_expression" {
                            if let Some(callee) = value_node
                                .child_by_field_name("function")
                                .or_else(|| value_node.child_by_field_name("callee"))
                            {
                                let callee_name =
                                    &self.source[callee.start_byte()..callee.end_byte()];
                                let short_name =
                                    callee_name.rsplit('.').next().unwrap_or(callee_name).trim();
                                if self.sanitizers.is_full_sanitizer(short_name)
                                    || self.sanitizers.is_full_sanitizer(callee_name)
                                {
                                    is_sanitized = true;
                                }
                            }
                        }

                        if !is_sanitized {
                            let value_text =
                                &self.source[value_node.start_byte()..value_node.end_byte()];
                            if !self.source_sink.is_sanitizer_call(value_text) {
                                for name in names {
                                    self.registry.taint(&name, TaintOrigin::UserInput);
                                }
                            }
                        }
                    } else if self.is_node_environment_source(value_node) {
                        for name in names {
                            self.registry.taint(&name, TaintOrigin::Environment);
                        }
                    }
                }
            }
            // Track assignments
            "assignment_expression" => {
                if let Some(left) = node.child_by_field_name("left")
                    && let Some(right) = node.child_by_field_name("right")
                {
                    let names = extract_binding_names(left, self.source);
                    if self.is_node_tainted(right) {
                        let mut is_sanitized = false;
                        if right.kind() == "call_expression" {
                            if let Some(callee) = right
                                .child_by_field_name("function")
                                .or_else(|| right.child_by_field_name("callee"))
                            {
                                let callee_name =
                                    &self.source[callee.start_byte()..callee.end_byte()];
                                let short_name =
                                    callee_name.rsplit('.').next().unwrap_or(callee_name).trim();
                                if self.sanitizers.is_full_sanitizer(short_name)
                                    || self.sanitizers.is_full_sanitizer(callee_name)
                                {
                                    is_sanitized = true;
                                }
                            }
                        }

                        if is_sanitized {
                            for name in &names {
                                self.registry.untaint(name);
                            }
                        } else {
                            let value_text = &self.source[right.start_byte()..right.end_byte()];
                            if !self.source_sink.is_sanitizer_call(value_text) {
                                for name in names {
                                    self.registry.taint(&name, TaintOrigin::UserInput);
                                }
                            }
                        }
                    } else if self.is_node_environment_source(right) {
                        for name in names {
                            self.registry.taint(&name, TaintOrigin::Environment);
                        }
                    } else {
                        // Clear taint on reassignment to a non-tainted value
                        for name in &names {
                            self.registry.untaint(name);
                        }
                    }
                }
            }
            _ => {}
        }

        // Recurse into children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                let result = self.follow_taint(child, depth + 1, path);
                if result.verified {
                    return result;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        CrossFileResult {
            verified: false,
            depth,
            path: path.clone(),
            detail: "No sink found in function body".to_string(),
            source_name: self.source_name.clone(),
            sink_name: self.potential_sink_name.clone(),
        }
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Check if a function call is a sink and if tainted data reaches it.
    fn check_call_for_sink(
        &mut self,
        call_node: Node,
        depth: usize,
        path: &mut Vec<String>,
    ) -> Option<CrossFileResult> {
        let callee = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))?;

        let fn_name_full = &self.source[callee.start_byte()..callee.end_byte()];

        let mut fn_name_field = fn_name_full;
        if callee.kind() == "member_expression" || callee.kind() == "field_expression" {
            if let Some(field) = callee.child_by_field_name("field") {
                fn_name_field = &self.source[field.start_byte()..field.end_byte()];
            }
        }

        // Apply safe-base filtering to avoid false positives on native objects
        if fn_name_full.starts_with("Object.")
            || fn_name_full.starts_with("Array.")
            || fn_name_full.starts_with("String.")
            || fn_name_full.starts_with("Math.")
            || fn_name_full.starts_with("JSON.")
            || fn_name_full.starts_with("console.")
            || fn_name_full.starts_with("process.")
        {
            return None;
        }

        // Check if any argument is tainted
        if let Some(args_list) = call_node.child_by_field_name("arguments") {
            let mut cursor = args_list.walk();
            for arg in args_list.children(&mut cursor) {
                if matches!(arg.kind(), "(" | ")" | ",") {
                    continue;
                }
                if self.is_node_tainted(arg) {
                    // Record as a potential sink since tainted data reached it
                    self.potential_sink_name = Some(fn_name_full.to_string());

                    // Use is_sink_expr for context-aware matching (qualified + suffix)
                    if self.source_sink.is_sink_expr(fn_name_full).is_some() {
                        if let Some(cfg) = &self.cfg {
                            if let Some(sink_block) =
                                frensense_engine::cfg::block_for_byte(cfg, call_node.start_byte())
                            {
                                if frensense_engine::cfg::has_auth_guard_dominator(
                                    cfg,
                                    sink_block,
                                    self.source,
                                ) {
                                    return None;
                                }
                            }
                        }

                        let arg_text = &self.source[arg.start_byte()..arg.end_byte()];
                        path.push(format!("{fn_name_full}({arg_text})"));
                        return Some(CrossFileResult {
                            verified: true,
                            depth,
                            path: path.clone(),
                            detail: format!("Tainted data reaches sink '{fn_name_full}'"),
                            source_name: self.source_name.clone(),
                            sink_name: Some(fn_name_full.to_string()),
                        });
                    }
                }
            }
        }

        None
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Check if a node is tainted (variable reference or member expression).
    fn is_node_tainted(&self, node: Node) -> bool {
        match node.kind() {
            "identifier" => {
                let name = &self.source[node.start_byte()..node.end_byte()];
                self.registry.is_tainted(name)
            }
            "member_expression" | "field_expression" => {
                if let Some(object) = node.child_by_field_name("object").or_else(|| node.child(0)) {
                    return self.is_node_tainted(object);
                }
                false
            }
            "call_expression" => {
                let callee = node
                    .child_by_field_name("function")
                    .or_else(|| node.child_by_field_name("callee"))
                    .or_else(|| node.child(0));

                if let Some(c) = callee {
                    let fn_name = &self.source[c.start_byte()..c.end_byte()];
                    let mut short_name = fn_name;
                    let mut is_member = false;

                    if matches!(c.kind(), "member_expression" | "field_expression") {
                        is_member = true;
                        if let Some(prop) = c
                            .child_by_field_name("property")
                            .or_else(|| c.child_by_field_name("field"))
                        {
                            short_name = &self.source[prop.start_byte()..prop.end_byte()];
                        }
                    }

                    if let Some(rule) = self
                        .propagators
                        .get_rule(fn_name)
                        .or_else(|| self.propagators.get_rule(short_name))
                    {
                        if rule.tainted_receiver && is_member {
                            if let Some(object) = c.child_by_field_name("object") {
                                if self.is_node_tainted(object) {
                                    return true;
                                }
                            }
                        }

                        if let Some(target_idx) = rule.tainted_arg {
                            if let Some(args_list) = node.child_by_field_name("arguments") {
                                let mut cursor = args_list.walk();
                                let mut arg_idx = 0;
                                for arg in args_list.children(&mut cursor) {
                                    if matches!(arg.kind(), "(" | ")" | ",") {
                                        continue;
                                    }
                                    if arg_idx == target_idx && self.is_node_tainted(arg) {
                                        return true;
                                    }
                                    arg_idx += 1;
                                }
                            }
                        }
                    } else {
                        // Fallback logic for unknown functions
                        if self.is_node_tainted(c) {
                            return true;
                        }

                        if let Some(args_list) = node.child_by_field_name("arguments") {
                            let mut cursor = args_list.walk();
                            for arg in args_list.children(&mut cursor) {
                                if matches!(arg.kind(), "(" | ")" | ",") {
                                    continue;
                                }
                                if self.is_node_tainted(arg) {
                                    return true;
                                }
                            }
                        }
                    }

                    // Return-value taint propagation from known source methods
                    if is_member {
                        if let Some(object) = c.child_by_field_name("object") {
                            if matches!(
                                short_name,
                                "json"
                                    | "text"
                                    | "formData"
                                    | "query"
                                    | "header"
                                    | "param"
                                    | "body"
                                    | "arrayBuffer"
                                    | "first"
                                    | "all"
                                    | "get"
                            ) {
                                if self.is_node_tainted(object) {
                                    return true;
                                }
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Check if a node is an environment variable source.
    fn is_node_environment_source(&self, node: Node) -> bool {
        match node.kind() {
            "member_expression" | "field_expression" => {
                let text = &self.source[node.start_byte()..node.end_byte()];
                text.starts_with("process.env.")
                    || text.starts_with("env.")
                    || text == "process.env"
                    || text == "env"
            }
            "identifier" => {
                let text = &self.source[node.start_byte()..node.end_byte()];
                text == "env" || text == "process.env"
            }
            _ => false,
        }
    }
}

/// Classify a parameter name into a `TaintOrigin` based on naming conventions.
///
/// Used for untyped languages (JavaScript, Python, etc.) where type annotations
/// are absent. Returns `None` if the name does not match any known source pattern.
fn classify_param_origin(name: &str) -> Option<TaintOrigin> {
    let lower = name.to_lowercase();
    // User-controlled HTTP input
    if matches!(
        lower.as_str(),
        "req"
            | "request"
            | "event"
            | "ctx"
            | "context"
            | "payload"
            | "input"
            | "body"
            | "query"
            | "params"
            | "args"
            | "data"
            | "cmd"
            | "url"
            | "path"
            | "file"
            | "name"
    ) {
        return Some(TaintOrigin::UserInput);
    }
    // Environment / configuration
    if lower == "env" {
        return Some(TaintOrigin::Environment);
    }
    // Database records
    if matches!(
        lower.as_str(),
        "db" | "conn" | "connection" | "pool" | "row" | "record" | "result" | "results"
    ) {
        return Some(TaintOrigin::Database);
    }
    // Network sources
    if matches!(
        lower.as_str(),
        "socket" | "ws" | "stream" | "client" | "server" | "tcp" | "udp" | "peer"
    ) {
        return Some(TaintOrigin::Network);
    }
    // File-system sources
    if matches!(
        lower.as_str(),
        "fd" | "filepath" | "filename" | "buf" | "reader" | "content" | "src"
    ) {
        return Some(TaintOrigin::FileSystem);
    }
    None
}

#[cfg(test)]
mod tests {

    use frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry;

    fn test_registry() -> CorpusSourceSinkRegistry {
        let mut reg = CorpusSourceSinkRegistry::default();
        reg.source_types.insert("Request".to_string(), 5);
        reg.source_types.insert("Json".to_string(), 3);
        reg.sink_names.insert(
            "exec".to_string(),
            (
                frensense_engine::corpus::source_sink::SinkCategory::CodeExecution,
                4,
            ),
        );
        reg.sink_names.insert(
            "eval".to_string(),
            (
                frensense_engine::corpus::source_sink::SinkCategory::CodeExecution,
                3,
            ),
        );
        reg.sink_names.insert(
            "query".to_string(),
            (
                frensense_engine::corpus::source_sink::SinkCategory::SqlInjection,
                5,
            ),
        );
        reg.sink_names.insert(
            "innerHTML".to_string(),
            (frensense_engine::corpus::source_sink::SinkCategory::Xss, 2),
        );
        reg
    }

    #[test]
    fn test_source_sink_registry() {
        let reg = test_registry();
        assert!(reg.is_source_type("Request"));
        assert!(reg.is_source_type("Json"));
        assert!(!reg.is_source_type("String"));
        assert!(reg.is_sink("exec").is_some());
        assert!(reg.is_sink("eval").is_some());
        assert!(reg.is_sink("Math.random").is_none());
    }
}
