// SPDX-License-Identifier: MIT

//! Corpus-driven taint seeding.
//!
//! Instead of regex rules, this module seeds taint based on corpus patterns.
//! When the corpus finds a function that matches a known bug pattern,
//! this seeder identifies which parameters are likely tainted based on
//! the parameter's type annotation, using the corpus-learned source registry.

use super::TaintRegistry;
use crate::semantics::data_flow::TaintOrigin;
use frensense_engine::corpus::source_sink::{CorpusSourceSinkRegistry, extract_param_info};
use tree_sitter::Node;

/// Walk a member-expression chain to find the base (leftmost) identifier.
fn base_identifier_of(node: Node, source: &str) -> Option<String> {
    match node.kind() {
        "identifier" => Some(source[node.start_byte()..node.end_byte()].to_string()),
        "member_expression" | "field_expression" => {
            let object = node
                .child_by_field_name("object")
                .or_else(|| node.child(0))?;
            base_identifier_of(object, source)
        }
        _ => None,
    }
}

/// Seed taint for a function based on corpus pattern matching.
///
/// Uses the corpus-learned source type registry: taints parameters whose
///
/// # Panics
/// May panic if internal assertions fail.
/// type annotations match types found in positive corpus examples.
pub fn seed_from_corpus_match(
    fn_node: Node,
    source: &str,
    registry: &mut TaintRegistry,
    source_sink: &CorpusSourceSinkRegistry,
) {
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

        let (mut param_name, param_type) = extract_param_info(param, source);
        // Fallback for bare JS identifiers not parsed as named children
        if param_name.is_empty() && param.kind() == "identifier" {
            param_name = source[param.start_byte()..param.end_byte()].to_string();
        }
        if param_name.is_empty() {
            continue;
        }

        let clean_type = param_type.trim_start_matches(':').trim();
        let origin = if source_sink.is_source_type(clean_type) {
            Some(TaintOrigin::UserInput)
        } else if let Some(decorator_origin) =
            frensense_engine::decorator::classify_param_decorator(param, source)
        {
            Some(decorator_origin)
        } else {
            classify_param_origin(&param_name)
        };

        if let Some(origin) = origin {
            registry.taint(&param_name, origin);
            return;
        }
    }
}

/// Recursively seed taint from AST expressions inside a function body.
/// This revitalizes environment, database, and network sources.
pub fn seed_from_ast_body(node: Node, source: &str, registry: &mut TaintRegistry) {
    match node.kind() {
        "member_expression" | "field_expression" => {
            if let Some(object) = node.child_by_field_name("object") {
                let obj_name = &source[object.start_byte()..object.end_byte()];
                if let Some(prop) = node
                    .child_by_field_name("property")
                    .or_else(|| node.child_by_field_name("field"))
                {
                    let prop_name = &source[prop.start_byte()..prop.end_byte()];

                    // Handle environment sources (process.env.*, Deno.env.*)
                    if obj_name == "process.env"
                        || obj_name == "env"
                        || obj_name == "CONFIG"
                        || obj_name == "__ENV__"
                    {
                        let full_name = &source[node.start_byte()..node.end_byte()];
                        registry.taint(full_name, TaintOrigin::Environment);
                    } else if obj_name == "Deno.env" && prop_name == "get" {
                        let full_name = &source[node.start_byte()..node.end_byte()];
                        registry.taint(full_name, TaintOrigin::Environment);
                    }

                    // Handle DOM sources
                    if obj_name.ends_with("querySelector") && prop_name == "value" {
                        let full_name = &source[node.start_byte()..node.end_byte()];
                        registry.taint(full_name, TaintOrigin::UserInput);
                    }

                    // KV, R2, and WebSockets/Message
                    if (obj_name.ends_with("KV") || obj_name.ends_with("R2"))
                        && (prop_name == "get" || prop_name == "list")
                    {
                        let full_name = &source[node.start_byte()..node.end_byte()];
                        registry.taint(full_name, TaintOrigin::Database);
                    } else if (obj_name == "socket" || obj_name == "ws" || obj_name == "message")
                        && (prop_name == "data" || prop_name == "message")
                    {
                        let full_name = &source[node.start_byte()..node.end_byte()];
                        registry.taint(full_name, TaintOrigin::UserInput);
                    }

                    // Framework specific sources (Hono, Express, Next.js)
                    if obj_name == "c.req"
                        || obj_name == "req"
                        || obj_name == "request"
                        || obj_name == "ctx.req"
                        || obj_name.ends_with("Request")
                    {
                        if matches!(
                            prop_name,
                            "json"
                                | "query"
                                | "header"
                                | "param"
                                | "text"
                                | "parseBody"
                                | "body"
                                | "params"
                                | "headers"
                                | "cookies"
                                | "files"
                                | "searchParams"
                                | "nextUrl.searchParams"
                        ) {
                            let full_name = &source[node.start_byte()..node.end_byte()];
                            registry.taint(full_name, TaintOrigin::UserInput);
                        }
                    }

                    // Taint the base identifier so follow_taint's recursive member_expression
                    // walk (is_node_tainted → member → … → identifier) finds it at module level
                    if let Some(base) = base_identifier_of(node, source) {
                        if !registry.is_tainted(&base) {
                            if let Some(origin) = classify_param_origin(&base) {
                                registry.taint(&base, origin);
                            }
                        }
                    }
                }
            }
        }
        "call_expression" => {
            if let Some(callee) = node
                .child_by_field_name("function")
                .or_else(|| node.child_by_field_name("callee"))
            {
                let callee_name = &source[callee.start_byte()..callee.end_byte()];

                // Network sources
                if callee_name == "fetch"
                    || callee_name == "axios.get"
                    || callee_name == "got"
                    || callee_name == "request"
                {
                    let full_name = &source[node.start_byte()..node.end_byte()];
                    registry.taint(full_name, TaintOrigin::Network);
                }

                // FileSystem sources
                if callee_name == "fs.readFileSync"
                    || callee_name == "fs.readFile"
                    || callee_name == "fsPromises.readFile"
                    || callee_name == "fs.createReadStream"
                    || callee_name == "readline.createInterface"
                {
                    let full_name = &source[node.start_byte()..node.end_byte()];
                    registry.taint(full_name, TaintOrigin::FileSystem);
                }

                // Database sources (simplified)
                if callee_name.ends_with(".findUnique")
                    || callee_name.ends_with(".findFirst")
                    || callee_name.ends_with(".findMany")
                    || callee_name.ends_with(".get")
                    || callee_name.ends_with(".first")
                    || callee_name.ends_with(".all")
                    || callee_name.ends_with(".hget")
                    || callee_name.ends_with(".lrange")
                    || callee_name.ends_with(".getItem")
                    || callee_name.ends_with(".findOne")
                {
                    let full_name = &source[node.start_byte()..node.end_byte()];
                    registry.taint(full_name, TaintOrigin::Database);
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            seed_from_ast_body(cursor.node(), source, registry);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Classify a bare parameter name into a `TaintOrigin` for untyped languages.
///
/// This duplicates the logic in `cross_file::classify_param_origin` to keep the
/// modules independently testable. Both tables must stay in sync.
fn classify_param_origin(name: &str) -> Option<TaintOrigin> {
    let lower = name.to_lowercase();
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
            | "searchparams"
            | "args"
            | "cmd"
            | "url"
            | "path"
            | "file"
    ) {
        return Some(TaintOrigin::UserInput);
    }
    if matches!(
        lower.as_str(),
        "env" | "config" | "settings" | "conf" | "options" | "opts" | "cfg"
    ) {
        return Some(TaintOrigin::Environment);
    }
    if matches!(
        lower.as_str(),
        "db" | "conn" | "connection" | "pool" | "row" | "record" | "result" | "results"
    ) {
        return Some(TaintOrigin::Database);
    }
    if matches!(
        lower.as_str(),
        "socket" | "ws" | "stream" | "client" | "server" | "tcp" | "udp" | "peer"
    ) {
        return Some(TaintOrigin::Network);
    }
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
    use super::*;

    fn test_source_sink() -> CorpusSourceSinkRegistry {
        let mut reg = CorpusSourceSinkRegistry::default();
        reg.source_types.insert("Request".to_string(), 5);
        reg.source_types.insert("Json".to_string(), 3);
        reg
    }

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
        let source_sink = test_source_sink();
        seed_from_corpus_match(fn_node, source, &mut registry, &source_sink);

        assert!(
            registry.is_tainted("req"),
            "req with Request type should be tainted"
        );
    }

    #[test]
    fn test_no_taint_without_framework_type() {
        let source = "function process(randomVar: string) { }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        let source_sink = test_source_sink();
        seed_from_corpus_match(fn_node, source, &mut registry, &source_sink);

        assert!(
            !registry.is_tainted("randomVar"),
            "randomVar without framework type should NOT be tainted"
        );
    }
}
