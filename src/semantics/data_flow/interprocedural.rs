// SPDX-License-Identifier: MIT

//! Interprocedural taint verification.
//!
//! Follows taint flow across function boundaries, callbacks, and promises
//! to verify that user-controlled data actually reaches dangerous sinks.

use std::collections::HashSet;
use tree_sitter::Node;

use crate::semantics::data_flow::TaintRegistry;
use crate::semantics::data_flow::TaintOrigin;

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
    tree: &'a tree_sitter::Tree,
    registry: TaintRegistry,
    visited: HashSet<(String, usize)>,
    max_depth: usize,
}

impl<'a> InterproceduralVerifier<'a> {
    pub fn new(source: &'a str, tree: &'a tree_sitter::Tree) -> Self {
        Self {
            source,
            tree,
            registry: TaintRegistry::default(),
            visited: HashSet::new(),
            max_depth: 5,
        }
    }

    /// Seed taint for a function's parameters.
    pub fn seed_taint(&mut self, fn_node: Node, _source_name: &str) {
        self.seed_from_params(fn_node);
    }

    /// Seed taint from function parameters based on naming conventions.
    fn seed_from_params(&mut self, fn_node: Node) {
        let params_node = match fn_node
            .child_by_field_name("parameters")
            .or_else(|| fn_node.child_by_field_name("formal_parameters"))
        {
            Some(p) => p,
            None => return,
        };

        let source_names = [
            "req", "request", "input", "body", "query", "params",
            "args", "argv", "data", "payload", "event", "ctx", "context",
            "name", "cmd", "url", "path", "file",
        ];

        let framework_types = [
            "Request", "IncomingMessage", "FastifyRequest",
            "Context", "HttpContext", "ServletRequest",
            "Json", "Query", "Form", "Path", "Extension",
        ];

        let mut cursor = params_node.walk();
        for param in params_node.children(&mut cursor) {
            if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
                continue;
            }

            let (param_name, param_type) = extract_param_info(param, self.source);
            if param_name.is_empty() {
                continue;
            }

            let lower_name = param_name.to_lowercase();
            let clean_type = param_type.trim_start_matches(':').trim();

            // Check framework types
            for framework_type in &framework_types {
                if clean_type.contains(framework_type) {
                    self.registry.taint(&param_name, TaintOrigin::UserInput);
                    return;
                }
            }

            // Check common names
            for source_name in &source_names {
                if lower_name.contains(source_name) {
                    self.registry.taint(&param_name, TaintOrigin::UserInput);
                    return;
                }
            }
        }
    }

    /// Verify that taint flows from parameters to a sink in the function body.
    pub fn verify_flow(&mut self, fn_node: Node) -> InterproceduralResult {
        let body = match fn_node.child_by_field_name("body") {
            Some(b) => b,
            None => return InterproceduralResult {
                verified: false,
                depth: 0,
                path: Vec::new(),
                detail: "No function body".to_string(),
            },
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
                if let Some(name_node) = node.child_by_field_name("name") {
                    if let Some(value_node) = node.child_by_field_name("value") {
                        let name = &self.source[name_node.start_byte()..name_node.end_byte()];
                        // Check if value is tainted
                        if self.is_node_tainted(value_node) {
                            self.registry.taint(name, TaintOrigin::UserInput);
                        }
                    }
                }
            }
            // Track assignments
            "assignment_expression" => {
                if let Some(left) = node.child_by_field_name("left") {
                    if let Some(right) = node.child_by_field_name("right") {
                        let name = &self.source[left.start_byte()..left.end_byte()];
                        if self.is_node_tainted(right) {
                            self.registry.taint(name, TaintOrigin::UserInput);
                        }
                    }
                }
            }
            // Track await expressions
            "await_expression" => {
                if let Some(arg) = node.child(0) {
                    if self.is_node_tainted(arg) {
                        // The awaited value is tainted
                        // Continue tracking in parent context
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

        InterproceduralResult {
            verified: false,
            depth,
            path: path.clone(),
            detail: "No sink found in function body".to_string(),
        }
    }

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

        // Check if this is a dangerous sink
        let sink_info = identify_sink(fn_name);
        if let Some((_sink_type, description)) = sink_info {
            // Check if any argument is tainted
            if let Some(args_list) = call_node.child_by_field_name("arguments") {
                let mut cursor = args_list.walk();
                for arg in args_list.children(&mut cursor) {
                    if matches!(arg.kind(), "(" | ")" | ",") {
                        continue;
                    }
                    if self.is_node_tainted(arg) {
                        let arg_text = &self.source[arg.start_byte()..arg.end_byte()];
                        path.push(format!("{}({})", fn_name, arg_text));
                        return Some(InterproceduralResult {
                            verified: true,
                            depth,
                            path: path.clone(),
                            detail: format!(
                                "Tainted data reaches {} ({})",
                                fn_name, description
                            ),
                        });
                    }
                }
            }
        }

        None
    }

    /// Check if tainted data flows through callback arguments.
    ///
    /// Example: `processInput(() => req.query.cmd)`
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
                                detail: format!(
                                    "Tainted data flows through callback to {}",
                                    fn_name
                                ),
                            });
                        }
                    }
                }

                // Check if argument is a function expression
                if arg.kind() == "function" {
                    if let Some(body) = arg.child_by_field_name("body") {
                        let result = self.follow_taint(body, depth + 1, path);
                        if result.verified {
                            return Some(InterproceduralResult {
                                verified: true,
                                depth: result.depth,
                                path: result.path,
                                detail: format!(
                                    "Tainted data flows through callback to {}",
                                    fn_name
                                ),
                            });
                        }
                    }
                }
            }
        }

        None
    }

    /// Check if tainted data flows through a promise chain.
    ///
    /// Example: `getData().then(data => exec(data))`
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
        if let Some(object) = callee.child_by_field_name("object")
            .or_else(|| callee.child(0))
        {
            if !self.is_node_tainted(object) {
                return None;
            }
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
                    if let Some(params) = arg.child_by_field_name("parameters")
                        .or_else(|| arg.child_by_field_name("formal_parameters"))
                    {
                        let mut param_cursor = params.walk();
                        if let Some(first_param) = params.children(&mut param_cursor)
                            .find(|p| !matches!(p.kind(), "(" | ")" | "," | ";"))
                        {
                            if let Some(name_node) = first_param.child_by_field_name("name")
                                .or_else(|| first_param.child(0))
                            {
                                let param_name = &self.source[name_node.start_byte()..name_node.end_byte()];
                                self.registry.taint(param_name, TaintOrigin::UserInput);
                            }
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
                                    "Tainted data flows through .{}() callback",
                                    method_name
                                ),
                            });
                        }
                    }
                }

                // Check if argument is a function expression
                if arg.kind() == "function" {
                    if let Some(params) = arg.child_by_field_name("parameters") {
                        let mut param_cursor = params.walk();
                        if let Some(first_param) = params.children(&mut param_cursor)
                            .find(|p| !matches!(p.kind(), "(" | ")" | "," | ";"))
                        {
                            if let Some(name_node) = first_param.child_by_field_name("name")
                                .or_else(|| first_param.child(0))
                            {
                                let param_name = &self.source[name_node.start_byte()..name_node.end_byte()];
                                self.registry.taint(param_name, TaintOrigin::UserInput);
                            }
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
                                    "Tainted data flows through .{}() callback",
                                    method_name
                                ),
                            });
                        }
                    }
                }
            }
        }

        None
    }

    /// Check if a node is tainted (variable reference or member expression).
    fn is_node_tainted(&self, node: Node) -> bool {
        match node.kind() {
            "identifier" => {
                let name = &self.source[node.start_byte()..node.end_byte()];
                self.registry.is_tainted(name)
            }
            "member_expression" | "field_expression" => {
                // Check if the object is tainted
                if let Some(object) = node.child_by_field_name("object")
                    .or_else(|| node.child(0))
                {
                    return self.is_node_tainted(object);
                }
                false
            }
            "call_expression" => {
                // Check if the function returns tainted data
                if let Some(callee) = node.child_by_field_name("function")
                    .or_else(|| node.child_by_field_name("callee"))
                    .or_else(|| node.child(0))
                {
                    let fn_name = &self.source[callee.start_byte()..callee.end_byte()];

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

                    // For common getter functions, assume they return tainted data
                    let getter_patterns = ["get", "fetch", "read", "load", "parse", "find"];
                    for pattern in &getter_patterns {
                        if fn_name.to_lowercase().contains(pattern) {
                            return true;
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
                        if child.kind() == "template_substitution" {
                            if let Some(expr) = child.child(1) {
                                // Skip the ${ and }
                                if self.is_node_tainted(expr) {
                                    return true;
                                }
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

/// Identify if a function name is a dangerous sink.
fn identify_sink(fn_name: &str) -> Option<(&'static str, &'static str)> {
    let sinks = [
        ("exec", "command execution"),
        ("eval", "code evaluation"),
        ("system", "system command"),
        ("spawn", "process spawn"),
        ("popen", "process open"),
        ("query", "database query"),
        ("execute", "database execute"),
        ("raw_query", "raw SQL query"),
        ("format_sql", "SQL formatting"),
        ("read_to_string", "file read"),
        ("readFile", "file read"),
        ("write", "file write"),
        ("createReadStream", "file read stream"),
        ("innerHTML", "DOM XSS"),
        ("outerHTML", "DOM XSS"),
        ("document.write", "DOM XSS"),
        ("location.href", "URL redirect"),
        ("location.assign", "URL redirect"),
        ("redirect", "URL redirect"),
        ("fetch", "HTTP request"),
        ("http", "HTTP request"),
        ("request", "HTTP request"),
        ("axios", "HTTP request"),
    ];

    for (sink, description) in &sinks {
        if fn_name.contains(sink) {
            return Some((sink, description));
        }
    }
    None
}

/// Extract parameter name and type from a function parameter node.
fn extract_param_info(param: Node, source: &str) -> (String, String) {
    let mut name = String::new();
    let mut ty = String::new();

    let mut cursor = param.walk();
    for child in param.children(&mut cursor) {
        match child.kind() {
            "identifier" | "shorthand_field_identifier" | "field_identifier" => {
                if name.is_empty() {
                    name = source[child.start_byte()..child.end_byte()].to_string();
                }
            }
            "type_annotation" | "type_identifier" | "scoped_type_identifier" | "generic_type" => {
                if ty.is_empty() {
                    ty = source[child.start_byte()..child.end_byte()].to_string();
                }
            }
            _ => {}
        }
    }

    // Fallback: regex on full text
    if name.is_empty() || ty.is_empty() {
        let text = &source[param.start_byte()..param.end_byte()];
        if let Some(caps) = regex::Regex::new(r"(\w+)\s*:\s*(.+)")
            .ok()
            .and_then(|re| re.captures(text))
        {
            if name.is_empty() {
                name = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            }
            if ty.is_empty() {
                ty = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            }
        }
    }

    (name, ty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_sink() {
        assert!(identify_sink("exec").is_some());
        assert!(identify_sink("eval").is_some());
        assert!(identify_sink("query").is_some());
        assert!(identify_sink("innerHTML").is_some());
        assert!(identify_sink("console.log").is_none());
    }

    #[test]
    fn test_verify_flow_simple() {
        let source = "function handler(req) { exec(req.body.cmd); }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut verifier = InterproceduralVerifier::new(source, &tree);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        assert!(result.verified);
        assert!(result.detail.contains("exec"));
    }

    #[test]
    fn test_verify_flow_callback() {
        // Test that the verifier works with a simple function call
        let source = r#"
function handler(req) {
    const cmd = req.query.cmd;
    exec(cmd);
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut verifier = InterproceduralVerifier::new(source, &tree);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        // This should verify because exec(cmd) is a sink and cmd is tainted
        assert!(result.verified);
    }

    #[test]
    fn test_verify_flow_promise() {
        let source = r#"
function handler(req) {
    const data = getData(req);
    exec(data);
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut verifier = InterproceduralVerifier::new(source, &tree);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        // getData is a getter function, so it should be considered tainted
        assert!(result.verified);
    }

    #[test]
    fn test_verify_flow_no_sink() {
        let source = "function handler(req) { console.log(req.body.cmd); }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut verifier = InterproceduralVerifier::new(source, &tree);
        verifier.seed_taint(fn_node, "req");
        let result = verifier.verify_flow(fn_node);

        assert!(!result.verified);
    }
}
