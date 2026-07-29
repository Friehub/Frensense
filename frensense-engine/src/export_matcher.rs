// SPDX-License-Identifier: MIT

//! File-level export pattern matching.
//!
//! Frameworks like Next.js (App + Pages Router), SvelteKit, Cloudflare
//! Workers, and AWS Lambda use file conventions and named/default exports
//! to declare HTTP handlers — no explicit route registration needed.
//!
//! This module walks top-level export statements and classifies exported
//! functions/variables against a table of framework-specific rules keyed
//! by file path patterns and export names.

use tree_sitter::Node;

use crate::decorator::HttpMethod;

/// Kinds of HTTP handlers detected from file-level exports.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportHandlerKind {
    /// Named HTTP method export: `export async function GET(request) {}`
    /// Triggers when the export name matches an HTTP method AND the file
    /// path matches a framework convention (e.g. `api/` + `route.ts`).
    HttpHandlerByName(HttpMethod),
    /// Default function export: `export default function handler(...) {}`
    DefaultHandler,
    /// Cloudflare Worker: `export default { async fetch(request, env) {} }`
    WorkerFetch,
    /// AWS Lambda: `export const handler: Handler = async (...) => {}`
    LambdaHandler,
    /// SvelteKit: `export const GET: RequestHandler = ...` (same as HttpHandlerByName)
    SvelteKitHandler,
}

/// Framework-specific export handler rules.
///
/// Each entry: (file_path_substring, export_name, ExportHandlerKind)
/// An empty `file_path_substring` matches any file.
static EXPORT_HANDLER_RULES: &[(&str, &str, ExportHandlerKind)] = &[
    // SvelteKit: +server files (check BEFORE api/ so +server files under api/ match correctly)
    ("+server", "GET",    ExportHandlerKind::SvelteKitHandler),
    ("+server", "POST",   ExportHandlerKind::SvelteKitHandler),
    ("+server", "PUT",    ExportHandlerKind::SvelteKitHandler),
    ("+server", "DELETE", ExportHandlerKind::SvelteKitHandler),
    ("+server", "PATCH",  ExportHandlerKind::SvelteKitHandler),
    // Next.js App Router: files under api/ with any name, HTTP method exports
    ("api/", "GET",    ExportHandlerKind::HttpHandlerByName(HttpMethod::Get)),
    ("api/", "POST",   ExportHandlerKind::HttpHandlerByName(HttpMethod::Post)),
    ("api/", "PUT",    ExportHandlerKind::HttpHandlerByName(HttpMethod::Put)),
    ("api/", "DELETE", ExportHandlerKind::HttpHandlerByName(HttpMethod::Delete)),
    ("api/", "PATCH",  ExportHandlerKind::HttpHandlerByName(HttpMethod::Patch)),
    // Next.js Pages Router: files under pages/api/, default export
    ("pages/api/", "", ExportHandlerKind::DefaultHandler),
    // AWS Lambda: any file, named handler or lambdaHandler
    ("", "handler",       ExportHandlerKind::LambdaHandler),
    ("", "lambdaHandler", ExportHandlerKind::LambdaHandler),
    // Cloudflare Worker: any file, default export of an object with fetch
    ("", "", ExportHandlerKind::WorkerFetch), // checked separately
];

/// Try to classify a function node as an exported HTTP handler.
///
/// Checks whether the function is inside an `export_statement` and whether
/// the export matches a known framework pattern given the file path.
pub fn classify_exported_handler(
    fn_node: Node,
    source: &str,
    file_path: &str,
) -> Option<ExportHandlerKind> {
    // Walk up to find an export_statement ancestor
    let mut export_node: Option<Node> = None;
    let mut is_default = false;
    let mut current = fn_node;
    loop {
        if let Some(parent) = current.parent() {
            if parent.kind() == "export_statement" {
                export_node = Some(parent);
                for i in 0..parent.child_count() {
                    if let Some(child) = parent.child(i) {
                        if child.kind() == "default" {
                            is_default = true;
                            break;
                        }
                    }
                }
                break;
            }
            current = parent;
        } else {
            break;
        }
    }

    let export_node = export_node?;

    // For default exports of objects, check if it has a fetch method → Worker.
    // This runs BEFORE extract_exported_name because objects don't have a name.
    if is_default {
        let value = export_node
            .child_by_field_name("value")
            .or_else(|| {
                for i in 0..export_node.child_count() {
                    let child = export_node.child(i)?;
                    if child.kind() != "export" && child.kind() != "default" && child.kind() != ";" {
                        return Some(child);
                    }
                }
                None
            });
        if let Some(value_node) = value {
            if value_node.kind() == "object" || value_node.kind() == "object_literal" {
                for j in 0..value_node.child_count() {
                    if let Some(member) = value_node.child(j) {
                        if member.kind() == "method_definition" {
                            if let Some(name) = member.child_by_field_name("name") {
                                let method_name = &source[name.start_byte()..name.end_byte()];
                                if method_name == "fetch" {
                                    return Some(ExportHandlerKind::WorkerFetch);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let exported_name = extract_exported_name(fn_node, source)?;

    // Match against the rules table
    for (path_pattern, name_pattern, kind) in EXPORT_HANDLER_RULES {
        if matches!(kind, ExportHandlerKind::WorkerFetch) {
            continue;
        }

        if matches!(kind, ExportHandlerKind::DefaultHandler) {
            if is_default && file_path.contains(path_pattern) {
                return Some(*kind);
            }
            continue;
        }

        let name_matches = if name_pattern.is_empty() {
            true
        } else {
            exported_name == *name_pattern
        };

        let path_matches = if path_pattern.is_empty() {
            true
        } else {
            file_path.contains(path_pattern)
        };

        if name_matches && path_matches {
            return Some(*kind);
        }
    }

    None
}

/// Extract the exported name from a function/variable declaration node.
fn extract_exported_name(fn_node: Node, source: &str) -> Option<String> {
    if let Some(name_node) = fn_node.child_by_field_name("name") {
        return Some(source[name_node.start_byte()..name_node.end_byte()].to_string());
    }

    if fn_node.kind() == "lexical_declaration" || fn_node.kind() == "variable_declaration" {
        for i in 0..fn_node.child_count() {
            let child = fn_node.child(i)?;
            if child.kind() == "variable_declarator" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    return Some(source[name_node.start_byte()..name_node.end_byte()].to_string());
                }
            }
        }
    }

    if let Some(export_parent) = fn_node.parent() {
        if export_parent.kind() == "export_statement" {
            for i in 0..export_parent.child_count() {
                let child = export_parent.child(i)?;
                if child.kind() == "lexical_declaration" || child.kind() == "variable_declaration" {
                    for j in 0..child.child_count() {
                        let gc = child.child(j)?;
                        if gc.kind() == "variable_declarator" {
                            if let Some(name_node) = gc.child_by_field_name("name") {
                                return Some(
                                    source[name_node.start_byte()..name_node.end_byte()]
                                        .to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Extract the route path from a file path, matching known framework conventions.
///
/// Examples:
/// - `app/api/users/[id]/route.ts` → `/api/users/:id`
/// - `src/routes/api/orders/+server.ts` → `/api/orders`
/// - `pages/api/products/[slug].ts` → `/api/products/:slug`
pub fn extract_route_path_from_file(file_path: &str) -> Option<String> {
    let path = std::path::Path::new(file_path);

    let stem = path.file_stem()?.to_str()?;
    let mut full = path.with_extension("").to_string_lossy().to_string();

    if stem == "route" || stem == "+server" {
        if let Some(parent) = path.parent() {
            full = parent.with_extension("").to_string_lossy().to_string();
        }
    }

    let route = if let Some(idx) = full.find("app/") {
        &full[idx + 4..]
    } else if let Some(idx) = full.find("pages/") {
        &full[idx + 6..]
    } else if let Some(idx) = full.find("src/routes/") {
        &full[idx + 11..]
    } else if let Some(idx) = full.find("routes/") {
        &full[idx + 7..]
    } else {
        return None;
    };

    let route = route.replace('[', ":").replace(']', "");
    Some(format!("/{}", route))
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
    fn test_nextjs_app_router_get() {
        let src = "export async function GET(request: Request) { return Response.json({}); }";
        let (tree, source) = parse_ts(src);
        let export_stmt = tree.root_node().child(0).unwrap();
        let fn_node = export_stmt.child(1).unwrap(); // 0=export, 1=function_declaration
        assert_eq!(fn_node.kind(), "function_declaration");
        let result = classify_exported_handler(fn_node, &source, "app/api/users/route.ts");
        assert_eq!(result, Some(ExportHandlerKind::HttpHandlerByName(HttpMethod::Get)));
    }

    #[test]
    fn test_nextjs_pages_router_default() {
        let src = "export default function handler(req: NextApiRequest, res: NextApiResponse) {}";
        let (tree, source) = parse_ts(src);
        let export_stmt = tree.root_node().child(0).unwrap();
        let fn_node = export_stmt.child(2).unwrap();
        assert_eq!(fn_node.kind(), "function_declaration");
        let result = classify_exported_handler(fn_node, &source, "pages/api/users/[id].ts");
        assert_eq!(result, Some(ExportHandlerKind::DefaultHandler));
    }

    #[test]
    fn test_cloudflare_worker_fetch() {
        let src = "export default { async fetch(request: Request, env: Env): Promise<Response> { return new Response('ok'); } };";
        let (tree, source) = parse_ts(src);
        let export_stmt = tree.root_node().child(0).unwrap();
        let obj = export_stmt.child(2).unwrap();
        assert_eq!(obj.kind(), "object");
        let result = classify_exported_handler(obj, &source, "src/worker.ts");
        assert_eq!(result, Some(ExportHandlerKind::WorkerFetch));
    }

    #[test]
    fn test_sveltekit_server() {
        let src = "export const GET: RequestHandler = async ({ params }) => { return new Response('ok'); };";
        let (tree, source) = parse_ts(src);
        let export_stmt = tree.root_node().child(0).unwrap();
        for i in 0..export_stmt.child_count() {
            let child = export_stmt.child(i).unwrap();
            if child.kind() == "lexical_declaration" {
                let result = classify_exported_handler(child, &source, "src/routes/api/orders/+server.ts");
                assert_eq!(result, Some(ExportHandlerKind::SvelteKitHandler));
                return;
            }
        }
        panic!("lexical_declaration not found");
    }

    #[test]
    fn test_aws_lambda_handler() {
        let src = "export const handler: Handler = async (event, context) => {};";
        let (tree, source) = parse_ts(src);
        let export_stmt = tree.root_node().child(0).unwrap();
        for i in 0..export_stmt.child_count() {
            let child = export_stmt.child(i).unwrap();
            if child.kind() == "lexical_declaration" {
                let result = classify_exported_handler(child, &source, "src/lambda.ts");
                assert_eq!(result, Some(ExportHandlerKind::LambdaHandler));
                return;
            }
        }
        panic!("lexical_declaration not found");
    }

    #[test]
    fn test_non_export_not_matched() {
        let src = "function helper(x: number) { return x; }";
        let (tree, source) = parse_ts(src);
        let fn_node = tree.root_node().child(0).unwrap();
        let result = classify_exported_handler(fn_node, &source, "src/util.ts");
        assert!(result.is_none());
    }

    #[test]
    fn test_route_path_app_router() {
        let path = "app/api/users/[id]/route.ts";
        let route = extract_route_path_from_file(path);
        assert_eq!(route, Some("/api/users/:id".to_string()));
    }

    #[test]
    fn test_route_path_pages_router() {
        let path = "pages/api/products/[slug].ts";
        let route = extract_route_path_from_file(path);
        assert_eq!(route, Some("/api/products/:slug".to_string()));
    }

    #[test]
    fn test_route_path_sveltekit() {
        let path = "src/routes/api/orders/+server.ts";
        let route = extract_route_path_from_file(path);
        assert_eq!(route, Some("/api/orders".to_string()));
    }

    #[test]
    fn test_route_path_no_match() {
        let path = "src/lib/utils.ts";
        let route = extract_route_path_from_file(path);
        assert!(route.is_none());
    }
}