// SPDX-License-Identifier: MIT

//! Decorator-based HTTP handler detection (NestJS, routing-controllers, tsoa, type-graphql).
//!
//! Frameworks like NestJS use class and method decorators instead of
//! traditional `req`/`res` parameters to declare HTTP handlers.
//! Without decorator detection, the engine misses entire controller classes.

use tree_sitter::Node;

use crate::data_flow::TaintOrigin;

/// Known parameter decorators that mark a parameter as user-controlled input.
///
/// NestJS and related frameworks use decorators like `@Body()`, `@Query()`,
/// `@Param()`, etc. to declare where a parameter's value comes from.
/// Any parameter with one of these decorators should be treated as a taint source.
pub static PARAM_TAINT_DECORATORS: &[(&str, TaintOrigin)] = &[
    // NestJS / routing-controllers body sources
    ("Body", TaintOrigin::UserInput),
    ("BodyParam", TaintOrigin::UserInput),
    ("RawBody", TaintOrigin::UserInput),
    // Query string
    ("Query", TaintOrigin::UserInput),
    ("Queries", TaintOrigin::UserInput),
    // Path parameters
    ("Param", TaintOrigin::UserInput),
    ("Params", TaintOrigin::UserInput),
    // Headers
    ("Headers", TaintOrigin::UserInput),
    ("Header", TaintOrigin::UserInput),
    // Raw request object
    ("Req", TaintOrigin::UserInput),
    // IP / Session / Cookies
    ("Ip", TaintOrigin::UserInput),
    ("IpAddress", TaintOrigin::UserInput),
    ("Session", TaintOrigin::UserInput),
    ("Cookies", TaintOrigin::UserInput),
    ("Cookie", TaintOrigin::UserInput),
    ("UploadedFile", TaintOrigin::UserInput),
    ("UploadedFiles", TaintOrigin::UserInput),
    ("HostParam", TaintOrigin::UserInput),
    ("Hostname", TaintOrigin::UserInput),
    // GraphQL
    ("Args", TaintOrigin::UserInput),
    ("Arg", TaintOrigin::UserInput),
    // WebSocket
    ("MessageBody", TaintOrigin::UserInput),
    ("ConnectedSocket", TaintOrigin::Network),
    // Injected services — NOT user input
    ("Inject", TaintOrigin::UserInput), // ambiguous, treat as user input
];

/// HTTP method types detected from decorators.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Any,
    GraphQL,
    Grpc,
    WebSocket,
}

/// Check if a function/method node has a routing decorator (e.g. `@Get`, `@Post`).
///
/// In tree-sitter TypeScript, a `method_definition` with decorators looks like:
/// ```text
/// class_body
///   decorator          ← sibling BEFORE method_definition
///     call_expression
///       function: identifier "Get"
///   method_definition
///     name: identifier "getUser"
///     ...
/// ```
///
/// This walks the parent's children looking for `decorator` nodes that precede
/// the given function node.
pub fn has_routing_decorator(fn_node: Node, source: &str) -> Option<HttpMethod> {
    let parent = fn_node.parent()?;
    // Decorators appear as siblings BEFORE the method in the parent's child list.
    for i in 0..parent.child_count() {
        let child = parent.child(i)?;
        if child == fn_node {
            break;
        }
        if child.kind() == "decorator" {
            if let Some(method) = decorator_to_http_method(child, source) {
                return Some(method);
            }
        }
    }
    None
}

/// Extract the decorator name from a parameter node (e.g. `"Body"` from `@Body()`).
/// Returns `None` if the parameter has no decorator or the decorator is unrecognized.
pub fn param_decorator_name(param_node: Node, source: &str) -> Option<&'static str> {
    extract_param_decorator(param_node, source)
}

/// Extract NestJS parameter decorator source information.
///
/// For `@Body() dto: CreateUserDto`, returns `Some(UserInput)`.
/// For `@Headers('authorization') token: string`, returns `Some(UserInput)`.
/// For `@Param('id') id: string`, returns `Some(UserInput)`.
/// For `@Query('page') page: number`, returns `Some(UserInput)`.
/// For `@Ip() ip: string`, returns `Some(UserInput)`.
///
/// In tree-sitter TypeScript, the `decorator` node is a CHILD of
/// `required_parameter` (not a sibling), so we walk the param_node's own children.
///
/// Uses the `PARAM_TAINT_DECORATORS` table for lookup.
pub fn classify_param_decorator(param_node: Node, source: &str) -> Option<TaintOrigin> {
    let decorator_name = param_decorator_name(param_node, source)?;
    PARAM_TAINT_DECORATORS
        .iter()
        .find(|(name, _)| *name == decorator_name)
        .map(|(_, origin)| origin.clone())
}

/// Extract decorator names (e.g. `"@Body"`, `"@Query"`) from a function node's
/// parameters for use in `type_usages` during fingerprint extraction.
///
/// Returns strings prefixed with `@` so they're distinguishable from regular type
/// annotations in the fingerprint (e.g. `"@Body"` vs `"CreateUserDto"`).
pub fn collect_param_decorator_types(fn_node: Node, source: &str) -> Vec<String> {
    let params_node = fn_node
        .child_by_field_name("parameters")
        .or_else(|| fn_node.child_by_field_name("formal_parameters"));
    let Some(params_node) = params_node else {
        return Vec::new();
    };

    let mut decorator_types = Vec::new();
    for i in 0..params_node.child_count() {
        let child = match params_node.child(i) {
            Some(c) => c,
            None => continue,
        };
        // Only look at named parameters (skip punctuation like `(`, `)`, `,`)
        if child.kind() != "required_parameter" && child.kind() != "optional_parameter" {
            continue;
        }
        if let Some(name) = extract_param_decorator(child, source) {
            decorator_types.push(format!("@{}", name));
        }
    }
    decorator_types
}

/// Internal: extract the bare decorator name (e.g. `"Body"`) from a parameter node.
fn extract_param_decorator(param_node: Node, source: &str) -> Option<&'static str> {
    for i in 0..param_node.child_count() {
        let child = param_node.child(i)?;
        if child.kind() != "decorator" {
            continue;
        }
        let call = child
            .child_by_field_name("expression")
            .or_else(|| child.named_child(0));
        let Some(call) = call else { continue };
        let name_node = match call.kind() {
            "call_expression" => call.child_by_field_name("function")?,
            _ => call,
        };
        let name = &source[name_node.start_byte()..name_node.end_byte()];
        // Look up from the static table to get a &'static str
        for (decorator_name, _) in PARAM_TAINT_DECORATORS {
            if *decorator_name == name {
                return Some(decorator_name);
            }
        }
        if name == "NextFunction" || name == "Next" {
            return Some("Next");
        }
    }
    None
}

fn decorator_to_http_method(decorator_node: Node, source: &str) -> Option<HttpMethod> {
    let call = decorator_node
        .child_by_field_name("expression")
        .or_else(|| decorator_node.named_child(0))?;

    let name_node = match call.kind() {
        "call_expression" => call.child_by_field_name("function")?,
        _ => call,
    };
    let name = &source[name_node.start_byte()..name_node.end_byte()];

    match name {
        // NestJS / routing-controllers / tsoa
        "Get" | "HttpGet" => Some(HttpMethod::Get),
        "Post" | "HttpPost" => Some(HttpMethod::Post),
        "Put" | "HttpPut" => Some(HttpMethod::Put),
        "Delete" | "HttpDelete" => Some(HttpMethod::Delete),
        "Patch" | "HttpPatch" => Some(HttpMethod::Patch),
        "All" | "Options" | "Head" | "Search" => Some(HttpMethod::Any),
        // type-graphql
        "Query" | "Mutation" | "Subscription" => Some(HttpMethod::GraphQL),
        // gRPC
        "GrpcMethod" | "GrpcStreamMethod" => Some(HttpMethod::Grpc),
        // WebSocket (NestJS)
        "SubscribeMessage" | "WebSocketGateway" => Some(HttpMethod::WebSocket),
        // routing-controllers
        "OnUndefined" | "OnNull" | "OnEmpty" | "UseBefore" | "UseAfter" | "UseInterceptor"
        | "Use" => None,
        _ => None,
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
    fn test_method_has_get_decorator() {
        let src = "class C { @Get() async get() {} }";
        let (tree, source) = parse_ts(src);
        let class = tree.root_node().child(0).unwrap();
        let body = class.child_by_field_name("body").unwrap();
        let mut method_node = None;
        let mut cursor = body.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "method_definition" {
                    method_node = Some(child);
                    break;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        let method = method_node.expect("method_definition not found");
        assert_eq!(
            has_routing_decorator(method, &source),
            Some(HttpMethod::Get)
        );
    }

    #[test]
    fn test_classify_body_param() {
        let src = "class C { createUser(@Body() dto: CreateUserDto) {} }";
        let (tree, source) = parse_ts(src);
        let class = tree.root_node().child(0).unwrap();
        let body = class.child_by_field_name("body").unwrap();
        // Find the method_definition
        let mut method_node = None;
        let mut cursor = body.walk();
        if cursor.goto_first_child() {
            loop {
                if cursor.node().kind() == "method_definition" {
                    method_node = Some(cursor.node());
                    break;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        let method = method_node.expect("method_definition not found");
        let params = method.child_by_field_name("parameters").unwrap();
        // param 0 is '(', param 1 is the required_parameter, param 2 is ')'
        let param = params.named_child(0).unwrap();
        assert!(classify_param_decorator(param, &source).is_some());
    }

    #[test]
    fn test_classify_non_decorated_param() {
        let src = "function createUser(dto: CreateUserDto) {}";
        let (tree, source) = parse_ts(src);
        let fn_node = tree.root_node().child(0).unwrap();
        let params = fn_node.child_by_field_name("parameters").unwrap();
        let param = params.named_child(0).unwrap();
        assert!(classify_param_decorator(param, &source).is_none());
    }
}
