// SPDX-License-Identifier: MIT

//! Route registration backtracking.
//!
//! Express, Fastify, Koa, and Hono register HTTP handlers by passing
//! a function reference to a router method. The registration itself
//! is definitive evidence that the function is an HttpHandler — no
//! type annotations or response-call signals needed.
//!
//! This module walks the AST looking for call expressions that match
//! known router registration patterns (e.g. `app.get('/path', fn)`),
//! extracts the handler name, and builds a project-wide registry.

use std::collections::HashMap;
use tree_sitter::Node;

use crate::data_flow::TaintOrigin;

/// Where to find the handler argument in a registration call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandlerPosition {
    /// Handler is the last non-punctuation argument.
    /// Express: `app.get(path, handler)` — handler is the last arg.
    /// Middleware: `app.use(handler)` — handler is the last arg.
    LastArg,
    /// Handler is a field inside an options object.
    /// Fastify: `fastify.get(path, { handler: fn, schema: ... })` — `handler` field.
    ObjectField(&'static str),
}

/// One discovered route registration.
#[derive(Debug, Clone)]
pub struct RouteRegistration {
    /// HTTP method (Get, Post, etc.) or Any for `.use()`.
    pub method: super::decorator::HttpMethod,
    /// Route path string (empty if not a literal).
    pub path: String,
    /// Name of the handler function.
    pub handler_name: String,
    /// Source file path.
    pub file: String,
    /// Line number in source file.
    pub line: usize,
}

/// Project-wide registry of all discovered route registrations.
#[derive(Debug, Clone, Default)]
pub struct HandlerRegistry {
    /// function name → route registration(s).
    handlers: HashMap<String, Vec<RouteRegistration>>,
}

impl HandlerRegistry {
    pub fn is_registered_handler(&self, function_name: &str) -> bool {
        self.handlers.contains_key(function_name)
    }

    pub fn registrations_for(&self, name: &str) -> &[RouteRegistration] {
        self.handlers.get(name).map_or(&[], Vec::as_slice)
    }

    /// Merge registrations from another registry.
    pub fn merge(&mut self, other: HandlerRegistry) {
        for (name, regs) in other.handlers {
            self.handlers.entry(name).or_default().extend(regs);
        }
    }

    /// Insert a single registration.
    pub fn register(&mut self, reg: RouteRegistration) {
        self.handlers
            .entry(reg.handler_name.clone())
            .or_default()
            .push(reg);
    }
}

/// Known router method suffixes and their handler position.
///
/// Each entry maps: (method_suffix, HttpMethod, HandlerPosition)
static REGISTRATION_PATTERNS: &[(&str, super::decorator::HttpMethod, HandlerPosition)] = &[
    (
        "get",
        super::decorator::HttpMethod::Get,
        HandlerPosition::LastArg,
    ),
    (
        "post",
        super::decorator::HttpMethod::Post,
        HandlerPosition::LastArg,
    ),
    (
        "put",
        super::decorator::HttpMethod::Put,
        HandlerPosition::LastArg,
    ),
    (
        "delete",
        super::decorator::HttpMethod::Delete,
        HandlerPosition::LastArg,
    ),
    (
        "patch",
        super::decorator::HttpMethod::Patch,
        HandlerPosition::LastArg,
    ),
    (
        "all",
        super::decorator::HttpMethod::Any,
        HandlerPosition::LastArg,
    ),
    (
        "use",
        super::decorator::HttpMethod::Any,
        HandlerPosition::LastArg,
    ),
    (
        "options",
        super::decorator::HttpMethod::Any,
        HandlerPosition::LastArg,
    ),
    (
        "head",
        super::decorator::HttpMethod::Any,
        HandlerPosition::LastArg,
    ),
    // Fastify route object: `fastify.route({ method: 'POST', url: '/', handler: fn })`
    (
        "route",
        super::decorator::HttpMethod::Any,
        HandlerPosition::ObjectField("handler"),
    ),
];

/// Known router variable names (receiver of the method call).
/// E.g. `app`, `router`, `fastify`, `hono`, `server`, `route`.
/// We check the callee receiver against these to avoid false positives
/// from unrelated `.get()` / `.post()` calls.
static ROUTER_NAMES: &[&str] = &[
    "app",
    "router",
    "fastify",
    "hono",
    "server",
    "route",
    "mainRouter",
    "apiRouter",
    "adminRouter",
    "authRouter",
];

/// Build a `HandlerRegistry` from a file's AST.
///
/// Walks the entire tree looking for call expressions where:
/// 1. The callee is a member expression (e.g. `app.get`)
/// 2. The method suffix matches a registration pattern
/// 3. The receiver is a known router variable name
///
/// Extracts the handler function name from the arguments.
pub fn build_handler_registry(root: Node, source: &str, file_path: &str) -> HandlerRegistry {
    let mut registry = HandlerRegistry::default();
    let mut cursor = root.walk();

    loop {
        let node = cursor.node();
        if node.kind() == "call_expression" {
            if let Some(reg) = extract_registration(node, source, file_path) {
                registry.register(reg);
            }
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return registry;
            }
        }
    }
}

/// Try to extract a route registration from a call expression.
fn extract_registration(
    call_node: Node,
    source: &str,
    file_path: &str,
) -> Option<RouteRegistration> {
    let callee = call_node.child_by_field_name("function")?;
    if callee.kind() != "member_expression" && callee.kind() != "field_expression" {
        return None;
    }

    let object = callee.child_by_field_name("object")?;
    let obj_name = &source[object.start_byte()..object.end_byte()];

    // Check if receiver is a known router variable
    if !ROUTER_NAMES.iter().any(|r| obj_name == *r) {
        return None;
    }

    let property = callee
        .child_by_field_name("property")
        .or_else(|| callee.child_by_field_name("field"))?;
    let method_name = &source[property.start_byte()..property.end_byte()];

    // Match against known registration patterns
    let (_, http_method, position) = REGISTRATION_PATTERNS
        .iter()
        .find(|(name, _, _)| *name == method_name)?;

    let (handler_name, path) = extract_handler_and_path(call_node, source, position)?;

    Some(RouteRegistration {
        method: *http_method,
        path,
        handler_name,
        file: file_path.to_string(),
        line: call_node.start_position().row + 1,
    })
}

/// Extract the handler function name and route path from a registration call.
fn extract_handler_and_path(
    call_node: Node,
    source: &str,
    position: &HandlerPosition,
) -> Option<(String, String)> {
    let args = call_node.child_by_field_name("arguments")?;

    let handler_name = match position {
        HandlerPosition::LastArg => extract_last_named_arg(args, source),
        HandlerPosition::ObjectField(field) => extract_object_field_value(args, source, field),
    }?;

    // Try to extract the path (first string argument)
    let path = extract_first_string_arg(args, source).unwrap_or_default();

    Some((handler_name, path))
}

/// Extract the last named child of an arguments node.
/// If the last arg is an object literal with a `handler` field, extract that field's value
/// (Fastify options-style: `fastify.get(path, { handler: fn, ... })`).
fn extract_last_named_arg(args: Node, source: &str) -> Option<String> {
    let count = args.named_child_count();
    if count == 0 {
        return None;
    }
    let last = args.named_child(count - 1)?;
    let kind = last.kind();
    // Fastify options-style: last arg is an object with { handler: fn }
    if kind == "object" || kind == "object_literal" {
        if let Some(handler) = find_object_field_value_raw(last, source, "handler") {
            return Some(handler);
        }
    }
    // For member expressions (e.g. `sessionHandler.displayWelcomePage`),
    // extract just the property name (last segment)
    if kind == "member_expression" || kind == "field_expression" {
        if let Some(prop) = last
            .child_by_field_name("property")
            .or_else(|| last.child_by_field_name("field"))
        {
            return Some(source[prop.start_byte()..prop.end_byte()].to_string());
        }
    }
    let name = &source[last.start_byte()..last.end_byte()];
    Some(name.to_string())
}

/// Extract the first string literal argument (route path).
fn extract_first_string_arg(args: Node, source: &str) -> Option<String> {
    for i in 0..args.named_child_count() {
        let child = args.named_child(i)?;
        let kind = child.kind();
        if kind == "string" || kind == "string_fragment" || kind == "template_string" {
            let raw = &source[child.start_byte()..child.end_byte()];
            return Some(raw.trim_matches(&['"', '\'', '`'] as &[_]).to_string());
        }
    }
    None
}

/// Walk an object literal looking for `{ field: value }` and return the value text.
fn extract_object_field_value(args: Node, source: &str, field: &str) -> Option<String> {
    // Look through all args for an object literal
    for i in 0..args.named_child_count() {
        let child = args.named_child(i)?;
        if child.kind() != "object" && child.kind() != "object_literal" {
            continue;
        }
        return find_object_field_value_raw(child, source, field);
    }
    None
}

/// Given an object literal node, find a pair with the given key and return its value text.
fn find_object_field_value_raw(obj_node: Node, source: &str, field: &str) -> Option<String> {
    for j in 0..obj_node.named_child_count() {
        let pair = obj_node.named_child(j)?;
        if pair.kind() != "pair" {
            continue;
        }
        let key = pair.child_by_field_name("key")?;
        let key_name = &source[key.start_byte()..key.end_byte()];
        if key_name != field {
            continue;
        }
        let value = pair.child_by_field_name("value")?;
        let value_text = &source[value.start_byte()..value.end_byte()];
        return Some(value_text.to_string());
    }
    None
}

/// Check if an arrow function is passed as a direct argument to a router registration.
/// If so, the inline function IS the handler — classify it as HttpHandler without a name lookup.
pub fn is_inline_registered_handler(fn_node: Node, source: &str) -> bool {
    if fn_node.kind() != "arrow_function" {
        return false;
    }
    let Some(parent) = fn_node.parent() else {
        return false;
    };
    if parent.kind() != "arguments" {
        return false;
    }
    let Some(call) = parent.parent() else {
        return false;
    };
    if call.kind() != "call_expression" {
        return false;
    }
    extract_registration(call, source, "").is_some()
}

/// Try to infer a function's effective name when it's assigned to a property.
///
/// Handles patterns like:
/// - `this.displayWelcomePage = (req, res) => { ... }` → "displayWelcomePage"
/// - `SessionHandler.prototype.displayWelcomePage = function(req, res) { ... }` → "displayWelcomePage"
/// - `exports.handler = (req, res) => { ... }` → "handler"
/// - `module.exports = { handler: (req, res) => { ... } }` → "handler" (via pair key)
/// - `const foo = (req, res) => { ... }` → "foo"
///
/// Walks upward from `fn_node` to find the enclosing binary expression or
/// variable declarator, then extracts the property name.
pub fn infer_function_name(fn_node: Node, source: &str) -> Option<String> {
    let parent = fn_node.parent()?;
    let kind = parent.kind();

    // Case 1: expression_statement > assignment_expression > arrow_function
    // `this.foo = (req, res) => { ... }`
    if kind == "assignment_expression" || kind == "assignment" {
        let left = parent.child_by_field_name("left")?;
        return extract_property_name(left, source);
    }

    // Case 2: variable_declarator with arrow/value
    // `const foo = (req, res) => { ... }` → use variable name
    if kind == "variable_declarator" || kind == "assignment_pattern" {
        if let Some(name_node) = parent.child_by_field_name("name") {
            return Some(source[name_node.start_byte()..name_node.end_byte()].to_string());
        }
    }

    // Case 3: pair value (inside object literal)
    // `{ handler: (req, res) => { ... } }`
    if kind == "pair" {
        if let Some(key) = parent.child_by_field_name("key") {
            return Some(source[key.start_byte()..key.end_byte()].to_string());
        }
    }

    // Case 4: method_definition
    // `{ foo(req, res) { ... } }`
    if kind == "method_definition" || kind == "property_definition" {
        if let Some(name_node) = parent.child_by_field_name("name") {
            return Some(source[name_node.start_byte()..name_node.end_byte()].to_string());
        }
    }

    // Case 5: declaration — `function foo(req, res) { ... }`
    if kind == "function_declaration" || kind == "lexical_declaration" {
        if let Some(name_node) = parent.child_by_field_name("name") {
            return Some(source[name_node.start_byte()..name_node.end_byte()].to_string());
        }
    }

    None
}

/// Extract the property name from a member expression or field expression.
/// `this.displayWelcomePage` → "displayWelcomePage"
fn extract_property_name(node: Node, source: &str) -> Option<String> {
    let kind = node.kind();
    if kind == "member_expression" || kind == "field_expression" {
        if let Some(prop) = node
            .child_by_field_name("property")
            .or_else(|| node.child_by_field_name("field"))
        {
            return Some(source[prop.start_byte()..prop.end_byte()].to_string());
        }
    }
    // Plain identifier
    let text = &source[node.start_byte()..node.end_byte()];
    Some(text.to_string())
}

/// Check if a function or anonymous arrow is referenced as a handler
/// in the file's route registrations.
/// This is a same-file check used during fingerprint extraction.
///
/// Supports:
/// - Named functions: `function displayWelcomePage(req, res) { ... }`
/// - Anonymous arrows assigned to properties: `this.displayWelcomePage = (req, res) => { ... }`
/// - Anonymous arrows assigned to variables: `const displayWelcomePage = (req, res) => { ... }`
///
/// Walks upward from `fn_node` to find the tree root, then walks the full tree
/// looking for registration calls that reference this function's name.
pub fn is_function_registered_in_file(fn_node: Node, source: &str, _file_path: &str) -> bool {
    // First try to get the function's declared name (e.g., `function foo()`)
    let fn_name = if let Some(name_node) = fn_node.child_by_field_name("name") {
        source[name_node.start_byte()..name_node.end_byte()].to_string()
    } else if let Some(inferred) = infer_function_name(fn_node, source) {
        inferred
    } else {
        return false;
    };

    // Walk up to the root
    let mut root = fn_node;
    while let Some(parent) = root.parent() {
        root = parent;
    }

    let mut cursor = root.walk();
    loop {
        let node = cursor.node();
        if node.kind() == "call_expression" {
            if let Some(reg) = extract_registration(node, source, "") {
                if reg.handler_name == fn_name {
                    return true;
                }
            }
        }
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ts(source: &str) -> (tree_sitter::Tree, String) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        (tree, source.to_string())
    }

    #[test]
    fn test_build_registry_express_get() {
        let src = "app.get('/users', getUsers);";
        let (tree, source) = parse_ts(src);
        let registry = build_handler_registry(tree.root_node(), &source, "test.ts");
        assert!(registry.is_registered_handler("getUsers"));
        let regs = registry.registrations_for("getUsers");
        assert_eq!(regs.len(), 1);
        assert_eq!(regs[0].path, "/users");
    }

    #[test]
    fn test_build_registry_express_post() {
        let src = "router.post('/orders', createOrder);";
        let (tree, source) = parse_ts(src);
        let registry = build_handler_registry(tree.root_node(), &source, "test.ts");
        assert!(registry.is_registered_handler("createOrder"));
    }

    #[test]
    fn test_build_registry_fastify_options() {
        let src = r#"fastify.get('/users', { handler: getUsers, schema: {} });"#;
        let (tree, source) = parse_ts(src);
        let registry = build_handler_registry(tree.root_node(), &source, "test.ts");
        assert!(registry.is_registered_handler("getUsers"));
    }

    #[test]
    fn test_build_registry_middleware() {
        let src = "app.use('/admin', adminRouter);";
        let (tree, source) = parse_ts(src);
        let registry = build_handler_registry(tree.root_node(), &source, "test.ts");
        assert!(registry.is_registered_handler("adminRouter"));
    }

    #[test]
    fn test_not_registered_unrelated_call() {
        let src = "console.log('hello');";
        let (tree, source) = parse_ts(src);
        let registry = build_handler_registry(tree.root_node(), &source, "test.ts");
        assert!(!registry.is_registered_handler("hello"));

        let src2 = "user.get('hello');";
        let (tree2, source2) = parse_ts(src2);
        let registry2 = build_handler_registry(tree2.root_node(), &source2, "test.ts");
        assert!(!registry2.is_registered_handler("hello"));
    }

    #[test]
    fn test_merge_registries() {
        let src1 = "app.get('/a', fnA);";
        let src2 = "app.get('/b', fnB);";
        let (tree1, source1) = parse_ts(src1);
        let (tree2, source2) = parse_ts(src2);
        let reg1 = build_handler_registry(tree1.root_node(), &source1, "a.ts");
        let reg2 = build_handler_registry(tree2.root_node(), &source2, "b.ts");
        let mut merged = reg1.clone();
        merged.merge(reg2);
        assert!(merged.is_registered_handler("fnA"));
        assert!(merged.is_registered_handler("fnB"));
    }

    #[test]
    fn test_inline_arrow_handler() {
        let src = "app.get('/users', (req, res) => { res.send('ok'); });";
        let (tree, source) = parse_ts(src);
        let stmt = tree.root_node().child(0).unwrap();
        let call = stmt.child(0).unwrap();
        let args = call.child_by_field_name("arguments").unwrap();
        let arrow = args.named_child(1).unwrap();
        assert_eq!(arrow.kind(), "arrow_function");
        assert!(is_inline_registered_handler(arrow, &source));
    }

    #[test]
    fn test_non_inline_arrow_is_not_handler() {
        let src = "const f = (x) => x + 1;";
        let (tree, source) = parse_ts(src);
        let decl = tree.root_node().child(0).unwrap();
        let declarator = decl.child(1).unwrap();
        let arrow = declarator.child_by_field_name("value").unwrap();
        assert_eq!(arrow.kind(), "arrow_function");
        assert!(!is_inline_registered_handler(arrow, &source));
    }

    #[test]
    fn test_is_function_registered_via_this_assignment() {
        // NodeGoat-style handler: `this.displayWelcomePage = (req, res) => { ... }`
        // registered as `app.get("/", sessionHandler.displayWelcomePage)`
        let src = r#"
function SessionHandler(db) {
    this.displayWelcomePage = (req, res) => {
        res.render("welcome");
    };
}
const sessionHandler = new SessionHandler(db);
app.get("/", sessionHandler.displayWelcomePage);
"#;
        let (tree, source) = parse_ts(src);
        // Find the arrow function node
        let fn_node = tree.root_node();
        let mut cursor = fn_node.walk();
        let mut arrow_node = None;
        loop {
            let node = cursor.node();
            if node.kind() == "arrow_function" {
                arrow_node = Some(node);
                // Find the one inside the assignment
                let text = &source[node.start_byte()..node.end_byte()];
                if text.contains("res.render") {
                    arrow_node = Some(node);
                    break;
                }
            }
            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    break;
                }
            }
            if cursor.node().kind() == "program" && !cursor.goto_first_child() {
                break;
            }
        }
        let arrow = arrow_node.expect("should find arrow function");
        assert!(is_function_registered_in_file(arrow, &source, "test.ts"));
    }

    #[test]
    fn test_hono_style_inline() {
        let src = "app.get('/users/:id', (c) => c.json({}));";
        let (tree, source) = parse_ts(src);
        let stmt = tree.root_node().child(0).unwrap();
        let call = stmt.child(0).unwrap();
        let args = call.child_by_field_name("arguments").unwrap();
        let arrow = args.named_child(1).unwrap();
        assert_eq!(arrow.kind(), "arrow_function");
        assert!(is_inline_registered_handler(arrow, &source));
    }
}
