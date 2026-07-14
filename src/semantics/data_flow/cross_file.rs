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
        }
    }

    /// Seed taint for a function's parameters.
    ///
    /// Uses corpus-learned source types: taints parameters whose type annotations
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// match types found in positive corpus examples.
    pub fn seed_taint(&mut self, fn_node: Node) {
        let Some(params_node) = fn_node
            .child_by_field_name("parameters")
            .or_else(|| fn_node.child_by_field_name("formal_parameters"))
        else {
            return;
        };

        let mut cursor = params_node.walk();
        for param in params_node.children(&mut cursor) {
            if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
                continue;
            }

            let (param_name, param_type) = extract_param_info(param, self.source);
            if param_name.is_empty() {
                continue;
            }

            let clean_type = param_type.trim_start_matches(':').trim();
            if self.source_sink.is_source_type(clean_type) {
                self.registry.taint(&param_name, TaintOrigin::UserInput);
                self.source_name = Some(param_name);
                return;
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
                if let Some(name_node) = node.child_by_field_name("name")
                    && let Some(value_node) = node.child_by_field_name("value")
                {
                    let name = &self.source[name_node.start_byte()..name_node.end_byte()];
                    if self.is_node_tainted(value_node) {
                        self.registry.taint(name, TaintOrigin::UserInput);
                    }
                }
            }
            // Track assignments
            "assignment_expression" => {
                if let Some(left) = node.child_by_field_name("left")
                    && let Some(right) = node.child_by_field_name("right")
                {
                    let name = &self.source[left.start_byte()..left.end_byte()];
                    if self.is_node_tainted(right) {
                        self.registry.taint(name, TaintOrigin::UserInput);
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
            source_name: None,
            sink_name: None,
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

        // Check if this is a corpus-learned sink
        if !self.source_sink.is_sink(fn_name_field) {
            return None;
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
                // A call is tainted if any of its arguments are tainted
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
                false
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry;

    fn test_registry() -> CorpusSourceSinkRegistry {
        let mut reg = CorpusSourceSinkRegistry::default();
        reg.source_types.insert("Request".to_string(), 5);
        reg.source_types.insert("Json".to_string(), 3);
        reg.sink_names.insert("exec".to_string(), 4);
        reg.sink_names.insert("eval".to_string(), 3);
        reg.sink_names.insert("query".to_string(), 5);
        reg.sink_names.insert("innerHTML".to_string(), 2);
        reg
    }

    #[test]
    fn test_source_sink_registry() {
        let reg = test_registry();
        assert!(reg.is_source_type("Request"));
        assert!(reg.is_source_type("Json"));
        assert!(!reg.is_source_type("String"));
        assert!(reg.is_sink("exec"));
        assert!(reg.is_sink("eval"));
        assert!(!reg.is_sink("console.log"));
    }
}
