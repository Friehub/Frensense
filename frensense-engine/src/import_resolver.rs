// SPDX-License-Identifier: MIT

//! Per-file import map that resolves type names to their source packages.
//!
//! When a file has `import { Request } from 'express'`, this records
//! `"Request" → "express"`.  Without this, `Request` is ambiguous
//! (could be express.Request, node-fetch.Request, or a local type).
//! With the import map, any parameter typed `Request` is unambiguously
//! an Express request, so `is_http_handler` can use it as a signal.

use std::collections::HashMap;
use tree_sitter::Node;

/// Maps imported names (as they appear in type annotations) to their
/// originating package name.
///
/// Built once per file during `analyze_file`.  Used by `classify_role`
/// to resolve ambiguous type annotations like `Request` → `express`.
#[derive(Debug, Clone, Default)]
pub struct ImportMap {
    /// `name → package`, e.g. `"Request" → "express"`
    pub name_to_package: HashMap<String, String>,
}

impl ImportMap {
    pub fn new() -> Self {
        Self {
            name_to_package: HashMap::new(),
        }
    }

    /// Resolve an imported name to its source package.
    pub fn resolve(&self, type_name: &str) -> Option<&str> {
        self.name_to_package.get(type_name).map(|s| s.as_str())
    }

    /// Returns true when the given type name is known to be imported
    /// from the given package.
    pub fn is_imported_from(&self, type_name: &str, package: &str) -> bool {
        self.name_to_package
            .get(type_name)
            .is_some_and(|p| p == package)
    }

    /// Build the import map from a file's root AST node.
    ///
    /// Handles the following tree-sitter import patterns:
    /// - `import { A } from 'pkg'`            (named import)
    /// - `import { A as B } from 'pkg'`       (aliased named import → stores both `B` and `A`)
    /// - `import A from 'pkg'`                (default import → stores `A`)
    /// - `import * as A from 'pkg'`           (namespace import → stores `A`)
    /// - `import type { A } from 'pkg'`       (type import — same shape as named)
    /// - `import 'pkg'`                       (side-effect — no bindings, skipped)
    /// - `const A = require('pkg')`           (CommonJS default)
    /// - `const { A } = require('pkg')`       (CommonJS named)
    /// - `let A = require('pkg')`             (CommonJS with let/var)
    ///
    /// The function is language-agnostic within what tree-sitter grammars
    /// produce for `import_statement`, `call_expression` (require), etc.
    pub fn build_from_tree(source: &str, root: Node) -> Self {
        let mut map = Self::new();
        let mut cursor = root.walk();
        loop {
            let node = cursor.node();
            match node.kind() {
                "import_statement" => {
                    Self::process_import_statement(&mut map, node, source);
                }
                "lexical_declaration" | "variable_declaration" => {
                    Self::process_require_declaration(&mut map, node, source);
                }
                _ => {}
            }
            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return map;
                }
            }
        }
    }

    fn process_import_statement(map: &mut ImportMap, node: Node, source: &str) {
        let mut module_source: Option<String> = None;

        let mut cursor = node.walk();
        if !cursor.goto_first_child() {
            return;
        }
        loop {
            let child = cursor.node();
            match child.kind() {
                "string" | "string_fragment" => {
                    let raw = &source[child.start_byte()..child.end_byte()];
                    module_source = Some(raw.trim_matches(&['"', '\'', '`'] as &[_]).to_string());
                }
                "import_clause" => {
                    if let Some(ref pkg) = module_source {
                        Self::process_import_clause(map, child, source, pkg);
                    }
                }
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        // If we found a string source but no import_clause was matched above
        // (e.g. the string appears after the clause in the walk), try again
        if module_source.is_some() {
            return;
        }

        // Fallback: re-walk children to find both clause and source
        if cursor.goto_first_child() {
            let mut source_pkg: Option<String> = None;
            loop {
                let child = cursor.node();
                match child.kind() {
                    "string" | "string_fragment" => {
                        let raw = &source[child.start_byte()..child.end_byte()];
                        source_pkg = Some(raw.trim_matches(&['"', '\'', '`'] as &[_]).to_string());
                    }
                    "import_clause" => {
                        if let Some(ref pkg) = source_pkg {
                            Self::process_import_clause(map, child, source, pkg);
                        }
                    }
                    _ => {}
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    fn process_import_clause(map: &mut ImportMap, clause: Node, source: &str, package: &str) {
        let mut clause_cursor = clause.walk();
        if !clause_cursor.goto_first_child() {
            return;
        }
        loop {
            let child = clause_cursor.node();
            match child.kind() {
                // import A from 'pkg' → default import
                // tree-sitter may represent this as `identifier` directly inside import_clause
                "identifier" => {
                    let name = &source[child.start_byte()..child.end_byte()];
                    map.name_to_package
                        .insert(name.to_string(), package.to_string());
                }
                // namespace import: import * as A from 'pkg'
                "namespace_import" => {
                    if let Some(alias) = child.child_by_field_name("alias") {
                        let name = &source[alias.start_byte()..alias.end_byte()];
                        map.name_to_package
                            .insert(name.to_string(), package.to_string());
                    }
                }
                // Named import specifiers: { A, B as C }
                "named_imports" => {
                    let mut ncursor = child.walk();
                    if !ncursor.goto_first_child() {
                        break;
                    }
                    loop {
                        let spec = ncursor.node();
                        if spec.kind() == "import_specifier" {
                            let imported = spec
                                .child_by_field_name("name")
                                .or_else(|| spec.child_by_field_name("alias"));
                            let local = spec
                                .child_by_field_name("alias")
                                .or_else(|| spec.child_by_field_name("name"));
                            if let Some(l) = local {
                                let local_name =
                                    &source[l.start_byte()..l.end_byte()];
                                map.name_to_package
                                    .insert(local_name.to_string(), package.to_string());
                            }
                            if let Some(imp) = imported {
                                let imported_name =
                                    &source[imp.start_byte()..imp.end_byte()];
                                map.name_to_package
                                    .insert(imported_name.to_string(), package.to_string());
                            }
                        }
                        if !ncursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                _ => {}
            }
            if !clause_cursor.goto_next_sibling() {
                break;
            }
        }
    }

    fn process_require_declaration(map: &mut ImportMap, node: Node, source: &str) {
        let mut cursor = node.walk();
        if !cursor.goto_first_child() {
            return;
        }
        loop {
            let child = cursor.node();
            if child.kind() == "variable_declarator" {
                Self::process_require_declarator(map, child, source);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    fn process_require_declarator(map: &mut ImportMap, decl: Node, source: &str) {
        let value = match decl.child_by_field_name("value") {
            Some(v) => v,
            None => return,
        };
        if value.kind() != "call_expression" {
            return;
        }
        let callee = value
            .child_by_field_name("function")
            .or_else(|| value.child_by_field_name("callee"))
            .or_else(|| value.child(0));
        let callee_name = callee
            .map(|c| &source[c.start_byte()..c.end_byte()])
            .unwrap_or("");
        if callee_name != "require" {
            return;
        }

        // Extract the module name from require('...')
        let pkg = value
            .child_by_field_name("arguments")
            .and_then(|args| {
                let mut ac = args.walk();
                if ac.goto_first_child() {
                    let first = ac.node();
                    if first.kind() == "string" || first.kind() == "string_fragment" {
                        let raw = &source[first.start_byte()..first.end_byte()];
                        return Some(
                            raw.trim_matches(&['"', '\'', '`'] as &[_])
                                .to_string(),
                        );
                    }
                }
                None
            });

        let Some(package) = pkg else {
            return;
        };

        let name_node = decl.child_by_field_name("name").or_else(|| {
            // Fallback for destructured patterns: left side of assignment
            decl.child_by_field_name("pattern")
        });

        if let Some(nn) = name_node {
            match nn.kind() {
                "identifier" => {
                    let name = &source[nn.start_byte()..nn.end_byte()];
                    map.name_to_package
                        .insert(name.to_string(), package.clone());
                }
                "object_pattern" => {
                    let mut oc = nn.walk();
                    if oc.goto_first_child() {
                        loop {
                            let prop = oc.node();
                            if prop.kind() == "shorthand_property_identifier_pattern"
                                || prop.kind() == "pair_pattern"
                            {
                                let value_node = if prop.kind() == "pair_pattern" {
                                    prop.child_by_field_name("value")
                                } else {
                                    Some(prop)
                                };
                                if let Some(vn) = value_node {
                                    let name = &source[vn.start_byte()..vn.end_byte()];
                                    map.name_to_package
                                        .insert(name.to_string(), package.clone());
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let map = ImportMap::new();
        assert!(map.name_to_package.is_empty());
    }

    #[test]
    fn test_resolve_unknown() {
        let map = ImportMap::new();
        assert_eq!(map.resolve("Request"), None);
    }

    #[test]
    fn test_entry_point_not_imported() {
        let map = ImportMap::new();
        assert_eq!(map.classify_entry_point("Request"), EntryPointKind::Unknown);
    }
}
                            if !oc.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Categorizes what kind of entry point a function is based on its
/// imported parameter types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryPointKind {
    HttpRequestResponse,
    HttpRequestOnly,
    EventConsumer,
    QueueConsumer,
    WebhookReceiver,
    GrpcHandler,
    WebSocketHandler,
    Unknown,
}

static PACKAGE_HTTP_TYPES: &[(&str, &str, EntryPointKind)] = &[
    ("express",       "Request",           EntryPointKind::HttpRequestResponse),
    ("express",       "Response",          EntryPointKind::HttpRequestResponse),
    ("express",       "NextFunction",      EntryPointKind::HttpRequestResponse),
    ("fastify",       "FastifyRequest",    EntryPointKind::HttpRequestResponse),
    ("fastify",       "FastifyReply",      EntryPointKind::HttpRequestResponse),
    ("next/server",   "NextRequest",       EntryPointKind::HttpRequestOnly),
    ("next/server",   "NextResponse",      EntryPointKind::HttpRequestOnly),
    ("aws-lambda",    "APIGatewayProxyEvent",   EntryPointKind::HttpRequestResponse),
    ("aws-lambda",    "SQSEvent",               EntryPointKind::EventConsumer),
    ("aws-lambda",    "S3Event",                EntryPointKind::EventConsumer),
    ("@nestjs/common","ExecutionContext", EntryPointKind::HttpRequestResponse),
    ("hono",          "Context",           EntryPointKind::HttpRequestResponse),
    ("hono",          "HonoRequest",       EntryPointKind::HttpRequestOnly),
    ("koa",           "Context",           EntryPointKind::HttpRequestResponse),
    ("@grpc/grpc-js", "ServerUnaryCall",   EntryPointKind::GrpcHandler),
    ("ws",            "WebSocket",        EntryPointKind::WebSocketHandler),
    ("kafkajs",       "EachMessagePayload", EntryPointKind::QueueConsumer),
    ("stripe",        "Event",             EntryPointKind::WebhookReceiver),
];

impl ImportMap {
    pub fn classify_entry_point(&self, type_name: &str) -> EntryPointKind {
        let pkg = match self.name_to_package.get(type_name) {
            Some(p) => p.as_str(),
            None => return EntryPointKind::Unknown,
        };
        for &(table_pkg, table_type, ref kind) in PACKAGE_HTTP_TYPES {
            if pkg == table_pkg && type_name == table_type {
                return *kind;
            }
        }
        EntryPointKind::Unknown
    }

    pub fn has_http_entry_point(&self, type_usages: &[String]) -> bool {
        type_usages.iter().any(|t| {
            matches!(
                self.classify_entry_point(t),
                EntryPointKind::HttpRequestResponse | EntryPointKind::HttpRequestOnly
            )
        })
    }
}