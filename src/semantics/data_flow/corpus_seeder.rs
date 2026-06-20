// SPDX-License-Identifier: MIT

//! Corpus-driven taint seeding.
//!
//! Instead of regex rules, this module seeds taint based on corpus patterns.
//! When the corpus finds a function that matches a known bug pattern,
//! this seeder identifies which parameters are likely tainted based on
//! the pattern's structure.

use super::TaintRegistry;
use crate::semantics::data_flow::TaintOrigin;
use tree_sitter::Node;

/// Seed taint for a function based on corpus pattern matching.
///
/// The corpus found this function matches a known violation pattern.
/// This seeder identifies which parameters carry user-controlled data
/// by looking at the function's parameter types and usage patterns.
pub fn seed_from_corpus_match(
    fn_node: Node,
    source: &str,
    registry: &mut TaintRegistry,
) {
    // Strategy 1: Seed parameters with common framework types
    // This covers Express, Fastify, Axum, etc.
    seed_framework_params(fn_node, source, registry);

    // Strategy 2: Seed parameters named with common source patterns
    // This covers any function with input-like parameter names
    seed_named_params(fn_node, source, registry);
}

/// Seed parameters that match common framework entry point types.
fn seed_framework_params(fn_node: Node, source: &str, registry: &mut TaintRegistry) {
    let params_node = match fn_node
        .child_by_field_name("parameters")
        .or_else(|| fn_node.child_by_field_name("formal_parameters"))
    {
        Some(p) => p,
        None => return,
    };

    // Framework types that signal user-controlled input
    let framework_types = [
        "Request", "IncomingMessage", "FastifyRequest",
        "Context", "HttpContext", "ServletRequest",
        "Json", "Query", "Form", "Path", "Extension", "Multipart", "Bytes",
        "Body", "Query", "Path", "File", "Form",
    ];

    let mut cursor = params_node.walk();
    for param in params_node.children(&mut cursor) {
        if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
            continue;
        }

        let (param_name, param_type) = extract_param_info(param, source);
        if param_name.is_empty() {
            continue;
        }

        let clean_type = param_type.trim_start_matches(':').trim();

        // Check if type matches any framework type
        for framework_type in &framework_types {
            if clean_type.contains(framework_type) {
                registry.taint(&param_name, TaintOrigin::UserInput);
                return; // Found first tainted param, stop
            }
        }
    }
}

/// Seed parameters with common source-like names.
fn seed_named_params(fn_node: Node, source: &str, registry: &mut TaintRegistry) {
    let params_node = match fn_node
        .child_by_field_name("parameters")
        .or_else(|| fn_node.child_by_field_name("formal_parameters"))
    {
        Some(p) => p,
        None => return,
    };

    // Common names that signal user input
    let source_names = [
        "req", "request", "input", "body", "query", "params",
        "args", "argv", "data", "payload", "event", "ctx", "context",
    ];

    let mut cursor = params_node.walk();
    for param in params_node.children(&mut cursor) {
        if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
            continue;
        }

        let (param_name, _) = extract_param_info(param, source);
        if param_name.is_empty() {
            continue;
        }

        let lower_name = param_name.to_lowercase();
        for source_name in &source_names {
            if lower_name.contains(source_name) {
                registry.taint(&param_name, TaintOrigin::UserInput);
                return; // Found first tainted param, stop
            }
        }
    }
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
    fn test_seed_framework_params() {
        let source = "function handler(req: Request, res: Response) { }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        seed_from_corpus_match(fn_node, source, &mut registry);

        assert!(registry.is_tainted("req"), "req with Request type should be tainted");
    }

    #[test]
    fn test_seed_named_params() {
        let source = "function process(input: string) { }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        seed_from_corpus_match(fn_node, source, &mut registry);

        assert!(registry.is_tainted("input"), "input param should be tainted");
    }
}
