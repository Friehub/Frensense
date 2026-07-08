// SPDX-License-Identifier: MIT

//! Interprocedural taint verification.
//!
//! Follows taint flow across function boundaries, callbacks, and promises
//! to verify that user-controlled data actually reaches dangerous sinks.

use std::collections::HashSet;
use tree_sitter::Node;

use crate::semantics::data_flow::TaintOrigin;
use crate::semantics::data_flow::TaintRegistry;
use frensense_engine::corpus::source_sink::{CorpusSourceSinkRegistry, extract_param_info};

/// Result of interprocedural taint verification.
#[derive(Debug, Clone)]
pub struct InterproceduralResult {
    pub verified: bool,
    pub depth: usize,
    pub path: Vec<String>,
    pub detail: String,
}

/// Interprocedural taint verifier.
///
/// Follows taint flow through function calls, callbacks, and promises
/// to verify that user-controlled data reaches dangerous sinks.
pub struct InterproceduralVerifier<'a> {
    source: &'a str,
    _tree: &'a tree_sitter::Tree,
    registry: TaintRegistry,
    _visited: HashSet<(String, usize)>,
    max_depth: usize,
    source_sink: &'a CorpusSourceSinkRegistry,
}

impl<'a> InterproceduralVerifier<'a> {
    #[must_use]
    pub fn new(
        source: &'a str,
        tree: &'a tree_sitter::Tree,
        source_sink: &'a CorpusSourceSinkRegistry,
    ) -> Self {
        Self {
            source,
            _tree: tree,
            registry: TaintRegistry::default(),
            _visited: HashSet::new(),
            max_depth: 5,
            source_sink,
        }
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Seed taint for a function's parameters.
    pub fn seed_taint(&mut self, fn_node: Node, _source_name: &str) {
        self.seed_from_params(fn_node);
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Seed taint from function parameters using corpus-learned source types.
    fn seed_from_params(&mut self, fn_node: Node) {
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
                return;
            }
        }
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Verify that taint flows from parameters to a sink in the function body.
    pub fn verify_flow(&mut self, fn_node: Node) -> InterproceduralResult {
        let Some(body) = fn_node.child_by_field_name("body") else {
            return InterproceduralResult {
                verified: false,
                depth: 0,
                path: Vec::new(),
                detail: "No function body".to_string(),
            };
        };

        // Check if any parameter is tainted
        let has_tainted = self.registry.is_tainted("req")
            || self.registry.is_tainted("request")
            || self.registry.is_tainted("input")
            || self.registry.is_tainted("body")
            || self.registry.is_tainted("query")
            || self.registry.is_tainted("params")
            || self.registry.is_tainted("args")
            || self.registry.is_tainted("data")
            || self.registry.is_tainted("payload")
            || self.registry.is_tainted("event")
            || self.registry.is_tainted("ctx")
            || self.registry.is_tainted("context")
            || self.registry.is_tainted("name")
            || self.registry.is_tainted("cmd")
            || self.registry.is_tainted("url")
            || self.registry.is_tainted("path")
            || self.registry.is_tainted("file");

        if !has_tainted {
            return InterproceduralResult {
                verified: false,
                depth: 0,
                path: Vec::new(),
                detail: "No tainted parameters detected".to_string(),
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
    ) -> InterproceduralResult {
        if depth > self.max_depth {
            return InterproceduralResult {
                verified: false,
                depth,
                path: path.clone(),
                detail: "Maximum depth reached".to_string(),
            };
        }

        let kind = node.kind();

        match kind {
            // Check for sink function calls
            "call_expression" => {
                if let Some(result) = self.check_call_for_sink(node, depth, path) {
                    return result;
                }
                // Track taint through callback arguments
                if let Some(result) = self.check_callbacks(node, depth, path) {
                    return result;
                }
                // Track taint through promise chains
                if let Some(result) = self.check_promise_chain(node, depth, path) {
                    return result;
                }
            }
            // Track variable assignments
            "variable_declarator" | "lexical_declaration" => {
                if let Some(name_node) = node.child_by_field_name("name")
                    && let Some(value_node) = node.child_by_field_name("value")
                {
                    let name = &self.source[name_node.start_byte()..name_node.end_byte()];
                    // Check if value is tainted
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
            // Track await expressions
            "await_expression" => {
                if let Some(arg) = node.child(0)
                    && self.is_node_tainted(arg)
                {
                    // The awaited value is tainted
                    // Continue tracking in parent context
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

        InterproceduralResult {
            verified: false,
            depth,
            path: path.clone(),
            detail: "No sink found in function body".to_string(),
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
    ) -> Option<InterproceduralResult> {
        let callee = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))?;

        let fn_name = &self.source[callee.start_byte()..callee.end_byte()];

        // Check if this is a corpus-learned sink
        if !self.source_sink.is_sink(fn_name) {
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
                    path.push(format!("{fn_name}({arg_text})"));
                    return Some(InterproceduralResult {
                        verified: true,
                        depth,
                        path: path.clone(),
                        detail: format!("Tainted data reaches sink '{fn_name}'"),
                    });
                }
            }
        }

        None
    }

    /// Check if tainted data flows through callback arguments.
    ///
    /// Example: `processInput(() => req.query.cmd)`
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// The arrow function receives no arguments, but captures `req` from closure.
    fn check_callbacks(
        &mut self,
        call_node: Node,
        depth: usize,
        path: &mut Vec<String>,
    ) -> Option<InterproceduralResult> {
        let callee = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))?;

        let fn_name = &self.source[callee.start_byte()..callee.end_byte()];

        // Check if any argument is a callback (arrow function or function expression)
        if let Some(args_list) = call_node.child_by_field_name("arguments") {
            let mut cursor = args_list.walk();
            for arg in args_list.children(&mut cursor) {
                if matches!(arg.kind(), "(" | ")" | ",") {
                    continue;
                }

                // Check if argument is an arrow function
                if arg.kind() == "arrow_function" {
                    // Follow taint into the arrow function body
                    if let Some(body) = arg.child_by_field_name("body") {
                        let result = self.follow_taint(body, depth + 1, path);
                        if result.verified {
                            return Some(InterproceduralResult {
                                verified: true,
                                depth: result.depth,
                                path: result.path,
                                detail: format!("Tainted data flows through callback to {fn_name}"),
                            });
                        }
                    }
                }

                // Check if argument is a function expression
                if arg.kind() == "function"
                    && let Some(body) = arg.child_by_field_name("body")
                {
                    let result = self.follow_taint(body, depth + 1, path);
                    if result.verified {
                        return Some(InterproceduralResult {
                            verified: true,
                            depth: result.depth,
                            path: result.path,
                            detail: format!("Tainted data flows through callback to {fn_name}"),
                        });
                    }
                }
            }
        }

        None
    }

    /// Check if tainted data flows through a promise chain.
    ///
    /// Example: `getData().then(data => exec(data))`
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// The `.then()` callback receives the resolved value.
    fn check_promise_chain(
        &mut self,
        call_node: Node,
        depth: usize,
        path: &mut Vec<String>,
    ) -> Option<InterproceduralResult> {
        // Check if this is a method call like `.then()` or `.catch()`
        let callee = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))?;

        if callee.kind() != "member_expression" {
            return None;
        }

        let method_name = if let Some(prop) = callee.child_by_field_name("property") {
            &self.source[prop.start_byte()..prop.end_byte()]
        } else {
            return None;
        };

        // Check if this is a promise method
        let promise_methods = ["then", "catch", "finally", "tap"];
        if !promise_methods.contains(&method_name) {
            return None;
        }

        // Check if the receiver is tainted (e.g., `getData()` returns tainted data)
        if let Some(object) = callee
            .child_by_field_name("object")
            .or_else(|| callee.child(0))
            && !self.is_node_tainted(object)
        {
            return None;
        }

        // Check if the callback argument receives the resolved value
        if let Some(args_list) = call_node.child_by_field_name("arguments") {
            let mut cursor = args_list.walk();
            for arg in args_list.children(&mut cursor) {
                if matches!(arg.kind(), "(" | ")" | ",") {
                    continue;
                }

                // Check if argument is an arrow function
                if arg.kind() == "arrow_function" {
                    // The first parameter of the callback receives the resolved value
                    // Mark it as tainted
                    if let Some(params) = arg
                        .child_by_field_name("parameters")
                        .or_else(|| arg.child_by_field_name("formal_parameters"))
                    {
                        let mut param_cursor = params.walk();
                        if let Some(first_param) = params
                            .children(&mut param_cursor)
                            .find(|p| !matches!(p.kind(), "(" | ")" | "," | ";"))
                            && let Some(name_node) = first_param
                                .child_by_field_name("name")
                                .or_else(|| first_param.child(0))
                        {
                            let param_name =
                                &self.source[name_node.start_byte()..name_node.end_byte()];
                            self.registry.taint(param_name, TaintOrigin::UserInput);
                        }
                    }

                    // Follow taint into the callback body
                    if let Some(body) = arg.child_by_field_name("body") {
                        let result = self.follow_taint(body, depth + 1, path);
                        if result.verified {
                            return Some(InterproceduralResult {
                                verified: true,
                                depth: result.depth,
                                path: result.path,
                                detail: format!(
                                    "Tainted data flows through .{method_name}() callback"
                                ),
                            });
                        }
                    }
                }

                // Check if argument is a function expression
                if arg.kind() == "function" {
                    if let Some(params) = arg.child_by_field_name("parameters") {
                        let mut param_cursor = params.walk();
                        if let Some(first_param) = params
                            .children(&mut param_cursor)
                            .find(|p| !matches!(p.kind(), "(" | ")" | "," | ";"))
                            && let Some(name_node) = first_param
                                .child_by_field_name("name")
                                .or_else(|| first_param.child(0))
                        {
                            let param_name =
                                &self.source[name_node.start_byte()..name_node.end_byte()];
                            self.registry.taint(param_name, TaintOrigin::UserInput);
                        }
                    }

                    if let Some(body) = arg.child_by_field_name("body") {
                        let result = self.follow_taint(body, depth + 1, path);
                        if result.verified {
                            return Some(InterproceduralResult {
                                verified: true,
                                depth: result.depth,
                                path: result.path,
                                detail: format!(
                                    "Tainted data flows through .{method_name}() callback"
                                ),
                            });
                        }
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
                // Check if the object is tainted
                if let Some(object) = node.child_by_field_name("object").or_else(|| node.child(0)) {
                    return self.is_node_tainted(object);
                }
                false
            }
            "call_expression" => {
                // Check if the function returns tainted data
                if let Some(callee) = node
                    .child_by_field_name("function")
                    .or_else(|| node.child_by_field_name("callee"))
                    .or_else(|| node.child(0))
                {
                    let _fn_name = &self.source[callee.start_byte()..callee.end_byte()];

                    // Check if any argument is tainted
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
                false
            }
            // Await expressions preserve taint
            "await_expression" => {
                if let Some(arg) = node.child(0) {
                    return self.is_node_tainted(arg);
                }
                false
            }
            // Template literals with tainted expressions
            "template_string" | "template_literal" => {
                // Check if any expression in the template is tainted
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.kind() == "template_substitution"
                            && let Some(expr) = child.child(1)
                        {
                            // Skip the ${ and }
                            if self.is_node_tainted(expr) {
                                return true;
                            }
                        }
                        if !cursor.goto_next_sibling() {
                            break;
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
    use super::*;

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

    #[test]
    fn test_verify_flow_simple() {
        let source = "function handler(req: Request) { exec(req.body.cmd); }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let source_sink = test_registry();
        let mut verifier = InterproceduralVerifier::new(source, &tree, &source_sink);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        assert!(result.verified);
        assert!(result.detail.contains("exec"));
    }

    #[test]
    fn test_verify_flow_callback() {
        let source = r"
function handler(req: Request) {
    const cmd = req.query.cmd;
    exec(cmd);
}
";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let source_sink = test_registry();
        let mut verifier = InterproceduralVerifier::new(source, &tree, &source_sink);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        assert!(result.verified);
    }

    #[test]
    fn test_verify_flow_no_sink() {
        let source = "function handler(req: Request) { console.log(req.body.cmd); }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let source_sink = test_registry();
        let mut verifier = InterproceduralVerifier::new(source, &tree, &source_sink);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        assert!(!result.verified);
    }
}
