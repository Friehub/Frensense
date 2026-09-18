// SPDX-License-Identifier: MIT
//! JavaScript and TypeScript [`LanguageSpec`] implementations.
//!
//! Both share the same AST grammar (TypeScript is a superset of JavaScript in
//! tree-sitter-typescript). They differ only in the `tree_sitter_language()`
//! call and file extensions.

use tree_sitter::Node;

use crate::spec::{
    call_last_segment, node_text, Import, LanguageSpec, NodeRole, PackageCategory, PropagatorRule,
    SanitizerKind, TaintOrigin,
};

// ── Shared AST logic ──────────────────────────────────────────────────────────

fn classify_js(kind: &str) -> NodeRole {
    match kind {
        // ── Functions ────────────────────────────────────────────────────
        "function_declaration"
        | "function"
        | "async_function_declaration"
        | "generator_function_declaration" => NodeRole::Function {
            is_method: false,
            name_field: Some("name"),
            params_field: "parameters",
            body_field: "body",
        },
        "function_expression" | "async_function_expression" => NodeRole::Function {
            is_method: false,
            name_field: None,
            params_field: "parameters",
            body_field: "body",
        },
        "arrow_function" => NodeRole::Function {
            is_method: false,
            name_field: None,
            params_field: "parameters",
            body_field: "body",
        },
        "method_definition" => NodeRole::Function {
            is_method: true,
            name_field: Some("name"),
            params_field: "parameters",
            body_field: "body",
        },

        // ── Declarations / assignments ───────────────────────────────────
        "variable_declarator" | "lexical_declarator" => NodeRole::Declaration {
            name_field: "name",
            value_field: "value",
        },
        "assignment_expression" => NodeRole::Assignment {
            lhs_field: "left",
            rhs_field: "right",
        },

        // ── Calls ────────────────────────────────────────────────────────
        "call_expression" | "new_expression" => NodeRole::Call {
            callee_field: "function",
            args_field: "arguments",
        },
        "member_expression" => NodeRole::MemberAccess {
            object_field: "object",
            property_field: "property",
        },
        "subscript_expression" => NodeRole::MemberAccess {
            object_field: "object",
            property_field: "index",
        },

        // ── Control flow ─────────────────────────────────────────────────
        "if_statement" | "ternary_expression" | "switch_statement" => NodeRole::Branch,
        "for_statement" | "for_in_statement" | "for_of_statement" | "while_statement"
        | "do_statement" => NodeRole::Loop,
        "return_statement" => NodeRole::Return,
        "try_statement" => NodeRole::Try,
        "catch_clause" => NodeRole::Catch,
        "finally_clause" => NodeRole::Finally,
        "throw_statement" => NodeRole::Throw,
        "await_expression" => NodeRole::Await,

        // ── Structural ───────────────────────────────────────────────────
        "statement_block" | "object" => NodeRole::Block,
        "import_statement" => NodeRole::Import,
        "export_statement" => NodeRole::Export,
        "identifier" | "property_identifier" | "shorthand_property_identifier" => {
            NodeRole::Identifier
        }
        "string" | "template_string" | "number" | "true" | "false" | "null" | "undefined" => {
            NodeRole::Literal
        }

        // ── Supplementary structural roles ───────────────────────────────
        "formal_parameters" | "parameters" => NodeRole::Parameters,
        "arguments" => NodeRole::Arguments,
        "class_declaration" | "interface_declaration" | "enum_declaration" => NodeRole::ClassDef,
        "binary_expression" => NodeRole::BinaryOp,
        "unary_expression" | "update_expression" => NodeRole::UnaryOp,
        "switch_expression" => NodeRole::Match,

        _ => NodeRole::Other,
    }
}

fn extract_js_imports<'tree>(root: Node<'tree>, source: &str) -> Vec<Import> {
    let mut imports = Vec::new();
    let mut cursor = root.walk();

    'outer: loop {
        let node = cursor.node();

        if node.kind() == "import_statement" {
            // `import defaultExport from "module"`
            // `import { named, other as alias } from "module"`
            // `import * as ns from "module"`
            let package = find_string_child(node, source)
                .unwrap_or_default()
                .trim_matches(['"', '\''])
                .to_owned();

            for i in 0..node.named_child_count() {
                let child = node.named_child(i).unwrap();
                match child.kind() {
                    "identifier" => {
                        // default import
                        imports.push(Import {
                            local_name: node_text(child, source).to_owned(),
                            package: package.clone(),
                            symbol: None,
                        });
                    }
                    "import_clause" => {
                        extract_import_clause(child, source, &package, &mut imports);
                    }
                    _ => {}
                }
            }
        }

        if node.kind() == "call_expression" {
            // `require("module")` and `require("module").something`
            if let Some(callee) = node.child_by_field_name("function") {
                if node_text(callee, source) == "require" {
                    if let Some(args) = node.child_by_field_name("arguments") {
                        if let Some(str_node) = args.named_child(0) {
                            let pkg = node_text(str_node, source)
                                .trim_matches(['"', '\''])
                                .to_owned();
                            // The binding name comes from the outer variable_declarator
                            // We push a placeholder; the fingerprinter resolves it
                            // from the parent `variable_declarator.name` field.
                            imports.push(Import {
                                local_name: pkg.rsplit('/').next().unwrap_or(&pkg).to_owned(),
                                package: pkg,
                                symbol: None,
                            });
                        }
                    }
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
                break 'outer;
            }
        }
    }

    imports
}

fn extract_import_clause(node: Node<'_>, source: &str, package: &str, out: &mut Vec<Import>) {
    for i in 0..node.named_child_count() {
        let child = node.named_child(i).unwrap();
        match child.kind() {
            "identifier" => {
                // `import { Foo }` - no alias
                out.push(Import {
                    local_name: node_text(child, source).to_owned(),
                    package: package.to_owned(),
                    symbol: Some(node_text(child, source).to_owned()),
                });
            }
            "import_specifier" => {
                // `import { Foo as Bar }` → local=Bar, symbol=Foo
                let name = child
                    .child_by_field_name("name")
                    .map(|n| node_text(n, source))
                    .unwrap_or("");
                let alias = child
                    .child_by_field_name("alias")
                    .map(|n| node_text(n, source))
                    .unwrap_or(name);
                out.push(Import {
                    local_name: alias.to_owned(),
                    package: package.to_owned(),
                    symbol: Some(name.to_owned()),
                });
            }
            "namespace_import" => {
                // `import * as ns`
                if let Some(id) = child.named_child(0) {
                    out.push(Import {
                        local_name: node_text(id, source).to_owned(),
                        package: package.to_owned(),
                        symbol: None,
                    });
                }
            }
            _ => {}
        }
    }
}

fn find_string_child<'s>(node: Node<'_>, source: &'s str) -> Option<&'s str> {
    for i in 0..node.named_child_count() {
        let child = node.named_child(i)?;
        if matches!(child.kind(), "string" | "template_string") {
            return Some(node_text(child, source));
        }
    }
    None
}

// ── Package knowledge ─────────────────────────────────────────────────────────

fn js_package_category(pkg: &str) -> Option<PackageCategory> {
    // Normalize: strip @scope prefix for lookup, handle sub-paths
    let base = pkg.split('/').next().unwrap_or(pkg);
    match base {
        // HTTP frameworks
        "express" | "fastify" | "koa" | "hapi" | "@hono" | "hono" | "polka" | "h3" | "elysia"
        | "next" | "nuxt" | "@nestjs" | "nest" | "@adonisjs" | "bun" | "deno" | "@remix-run"
        | "astro" | "sveltekit" | "@sveltejs" | "trpc" | "@trpc" => {
            Some(PackageCategory::HttpFramework)
        }

        // GraphQL
        "graphql" | "apollo-server" | "@apollo" | "type-graphql" | "nexus" | "pothos-graphql"
        | "mercurius" => Some(PackageCategory::GraphQL),

        // WebSocket
        "ws" | "socket.io" | "uws" | "@fastify" => Some(PackageCategory::WebSocket),

        // Email Service
        "nodemailer" | "sendgrid" | "@sendgrid" | "mailgun" => Some(PackageCategory::EmailService),

        // SQL
        "pg" | "postgres" | "mysql" | "mysql2" | "mariadb" | "sqlite3" | "better-sqlite3"
        | "mssql" | "oracledb" | "sequelize" | "knex" | "typeorm" | "@prisma" | "prisma"
        | "slonik" | "drizzle-orm" => Some(PackageCategory::SqlDatabase),

        // NoSQL
        "mongodb" | "mongoose" | "redis" | "ioredis" | "cassandra-driver" | "couchdb"
        | "@elastic" => Some(PackageCategory::NoSqlDatabase),

        // Command execution
        "child_process" | "shelljs" | "execa" | "cross-spawn" | "node-pty" | "spawn-command" => {
            Some(PackageCategory::CommandExecution)
        }

        // HTTP clients (SSRF)
        "node-fetch" | "axios" | "got" | "superagent" | "undici" | "request" | "node:http"
        | "node:https" | "puppeteer" | "playwright" | "@playwright" => {
            Some(PackageCategory::HttpClient)
        }

        // File system (path traversal)
        "fs" | "node:fs" | "fs-extra" | "graceful-fs" | "recursive-readdir" | "glob" | "rimraf" => {
            Some(PackageCategory::FileSystem)
        }

        // Template engines (SSTI / XSS)
        "ejs" | "pug" | "handlebars" | "nunjucks" | "mustache" | "dot" | "art-template"
        | "consolidate" => Some(PackageCategory::TemplateEngine),

        // Unsafe deserialization
        "node-serialize"
        | "serialize-javascript"
        | "js-yaml"
        | "yaml"
        | "xml2js"
        | "fast-xml-parser"
        | "xml-js"
        | "libxmljs"
        | "saxjs" => Some(PackageCategory::Deserialization),

        // Testing and Validation
        "validator" | "joi" | "zod" | "yup" | "ajv" => Some(PackageCategory::Testing),

        // Crypto
        "crypto" | "node:crypto" | "bcrypt" | "bcryptjs" | "argon2" => {
            Some(PackageCategory::Crypto)
        }

        _ => None,
    }
}

fn js_classify_sanitizer(call: &str) -> Option<SanitizerKind> {
    match call_last_segment(call) {
        // HTML escape
        "escape" | "escapeHtml" | "escapeHTML" | "encodeHTML" | "sanitizeHtml" | "sanitize"
        | "clean" | "purify" | "stripTags" | "stripHtml" | "bleach" | "xssFilter" | "filterXSS"
        | "inHTMLData" | "inDoubleQuotedAttr" | "he.encode" => Some(SanitizerKind::HtmlEscape),

        // URL encode
        "encodeURIComponent" | "encodeURI" | "encode" => Some(SanitizerKind::UrlEncode),

        // Numeric coercion - input is definitely a number after this
        "parseInt" | "parseFloat" | "Number" | "BigInt" | "toFixed" | "toPrecision" => {
            Some(SanitizerKind::Full)
        }

        // Type narrowing and shell escapes
        "shellescape" | "shellQuote" | "escapeShellArg" | "quoteForShell" | "isUUID"
        | "isEmail" | "isAlphanumeric" | "isNumeric" | "isInt" | "isFloat" | "isISO8601"
        | "isValid" => Some(SanitizerKind::Full),

        // SQL parameterization (knex, sequelize, pg style)
        "sqlEscape" | "escapeId" | "format" | "literal" | "raw" => {
            Some(SanitizerKind::SqlParameterize)
        }

        // NoSQL parameterization
        "sanitizeFilter" | "mongoSanitize" | "sanitizeValue" => {
            Some(SanitizerKind::NoSqlParameterize)
        }

        // Path canonicalization
        "basename" | "realpath" => Some(SanitizerKind::PathNormalize),

        _ => None,
    }
}

static JS_PROPAGATORS: &[PropagatorRule] = &[
    // String methods - receiver taints return
    PropagatorRule {
        call: "concat",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "join",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "replace",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "replaceAll",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "slice",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "substring",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "trim",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "trimStart",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "trimEnd",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "toLowerCase",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "toUpperCase",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "split",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "toString",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "padStart",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "padEnd",
        tainted_arg: None,
        tainted_receiver: true,
    },
    // Array methods - receiver taints return
    PropagatorRule {
        call: "map",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "filter",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "flatMap",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "reduce",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "flat",
        tainted_arg: None,
        tainted_receiver: true,
    },
    // JSON
    PropagatorRule {
        call: "parse",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "stringify",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Buffer / encoding
    PropagatorRule {
        call: "from",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "toString",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "atob",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "btoa",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "decodeURIComponent",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "encodeURIComponent",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Template tags
    PropagatorRule {
        call: "format",
        tainted_arg: None,
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "template",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "render",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Path manipulation
    PropagatorRule {
        call: "join",
        tainted_arg: None,
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "resolve",
        tainted_arg: None,
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "normalize",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Modern String methods
    PropagatorRule {
        call: "charAt",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "charCodeAt",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "indexOf",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "lastIndexOf",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "includes",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "startsWith",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "endsWith",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "repeat",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "matchAll",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "at",
        tainted_receiver: true,
        tainted_arg: None,
    },
    // Template tags
    PropagatorRule {
        call: "interpolate",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "compile",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "tag",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Object spread
    PropagatorRule {
        call: "keys",
        tainted_arg: Some(0),
        tainted_receiver: true,
    }, // handles array keys() too
    PropagatorRule {
        call: "values",
        tainted_arg: Some(0),
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "entries",
        tainted_arg: Some(0),
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "assign",
        tainted_arg: Some(1),
        tainted_receiver: true,
    },
    PropagatorRule {
        call: "fromEntries",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "structuredClone",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "cloneDeep",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Array methods
    PropagatorRule {
        call: "find",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "findIndex",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "findLast",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "some",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "every",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "forEach",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "sort",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "reverse",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "fill",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "copyWithin",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "splice",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "push",
        tainted_receiver: true,
        tainted_arg: Some(0),
    },
    PropagatorRule {
        call: "unshift",
        tainted_receiver: true,
        tainted_arg: Some(0),
    },
    // Promise
    PropagatorRule {
        call: "then",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "catch",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "finally",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "all",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "allSettled",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "race",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    // Lodash
    PropagatorRule {
        call: "get",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "pick",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "omit",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "mapKeys",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "mapValues",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "flattenDeep",
        tainted_receiver: true,
        tainted_arg: None,
    },
    PropagatorRule {
        call: "groupBy",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "zip",
        tainted_arg: None,
        tainted_receiver: false,
    },
    PropagatorRule {
        call: "unzip",
        tainted_arg: Some(0),
        tainted_receiver: false,
    },
];

// ── TypeScript spec ───────────────────────────────────────────────────────────

pub struct TypeScriptSpec;

impl LanguageSpec for TypeScriptSpec {
    fn name(&self) -> &'static str {
        "typescript"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "tsx", "mts", "cts"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        #[cfg(feature = "typescript")]
        return tree_sitter_typescript::LANGUAGE_TSX.into();
        #[cfg(not(feature = "typescript"))]
        panic!("frensense-lang: 'typescript' feature not enabled");
    }

    fn classify(&self, kind: &str) -> NodeRole {
        classify_js(kind)
    }

    fn wrap_region(&self, code: &str) -> String {
        format!("function _region(): void {{\n{}\n}}", code)
    }

    fn import_node_kinds(&self) -> &'static [&'static str] {
        &["import_statement"]
    }

    fn extract_imports<'tree>(&self, root: Node<'tree>, source: &str) -> Vec<Import> {
        extract_js_imports(root, source)
    }

    fn symbol_query(&self) -> Option<&'static str> {
        Some(
            r#"
            (function_declaration name: (identifier) @name)
            (method_definition name: (property_identifier) @name)
            (class_declaration name: (type_identifier) @name)
            (interface_declaration name: (type_identifier) @name)
            (variable_declarator name: (identifier) @name)
            (call_expression arguments: (arguments (arrow_function) @name))
            (call_expression arguments: (arguments (function_expression) @name))
        "#,
        )
    }

    fn call_query(&self) -> Option<&'static str> {
        Some(
            r#"
            (function_declaration name: (identifier) @caller
                body: (statement_block
                    (expression_statement
                        (call_expression function: (identifier) @call))))
            (function_declaration name: (identifier) @caller
                body: (statement_block
                    (expression_statement
                        (call_expression
                            function: (member_expression
                                property: (property_identifier) @call)))))
            (method_definition name: (property_identifier) @caller
                body: (statement_block
                    (expression_statement
                        (call_expression function: (identifier) @call))))
        "#,
        )
    }

    fn package_category(&self, pkg: &str) -> Option<PackageCategory> {
        js_package_category(pkg)
    }

    fn classify_param_taint(
        &self,
        name: Option<&str>,
        type_annotation: Option<&str>,
    ) -> Option<TaintOrigin> {
        classify_js_param(name, type_annotation)
    }

    fn is_http_route_decorator(&self, decorator_name: &str) -> bool {
        matches!(
            decorator_name,
            "Get"
                | "Post"
                | "Put"
                | "Delete"
                | "Patch"
                | "All"
                | "Controller"
                | "Route"
                | "HttpGet"
                | "HttpPost"
                | "UseGuards"
                | "UseInterceptors"
        )
    }

    fn request_param_names(&self) -> &'static [&'static str] {
        &[
            "req", "request", "ctx", "context", "event", "c", "e", "r", "h", "app", "handler",
            "input", "args", "parent", "info",
        ]
    }

    fn known_sink_names(&self) -> &'static [(&'static str, crate::spec::SinkLabel)] {
        JS_SINK_NAMES
    }

    fn known_source_patterns(&self) -> &'static [&'static str] {
        JS_SOURCE_PATTERNS
    }

    fn propagator_rules(&self) -> &'static [PropagatorRule] {
        JS_PROPAGATORS
    }

    fn classify_sanitizer(&self, call: &str) -> Option<SanitizerKind> {
        js_classify_sanitizer(call)
    }

    fn route_context_hints(&self) -> &'static [&'static str] {
        &[
            "(req, res)",
            "app.get(",
            "router.get(",
            "app.post(",
            "router.post(",
            "res.send",
            "res.json",
            "res.status",
            "c.req",
            "c.json",
            "ctx.body",
            "ctx.response",
            "fastify.get(",
            "fastify.post(",
            "export async function GET(",
            "export async function POST(",
            "export async function PUT(",
            "export async function DELETE(",
            "export async function PATCH(",
            "export default function handler(",
            "publicProcedure.input(",
            "protectedProcedure.input(",
            "t.procedure",
            "Query: {",
            "Mutation: {",
            "Subscription: {",
            "resolve(",
            "export async function loader(",
            "export async function action(",
            "export async function load(",
            "export const GET = ",
            "export const POST = ",
            "io.on('connection'",
            "socket.on(",
            "Bun.serve(",
            "Deno.serve(",
            "Deno.serve({ handler",
        ]
    }

    fn test_context_hints(&self) -> &'static [&'static str] {
        &[
            "describe(",
            " it(",
            "test(",
            "expect(",
            "jest.",
            "vitest.",
            "chai.",
            "assert.",
        ]
    }

    fn response_method_names(&self) -> &'static [&'static str] {
        &[
            "json",
            "send",
            "redirect",
            "status",
            "render",
            "end",
            "write",
            "setHeader",
            "cookie",
            "clearCookie",
            "type",
            "format",
            "attachment",
        ]
    }

    fn db_api_method_names(&self) -> &'static [&'static str] {
        &[
            "query",
            "execute",
            "prepare",
            "raw",
            "find",
            "findOne",
            "findMany",
            "insert",
            "update",
            "delete",
            "create",
            "save",
            "select",
            "from",
            "where",
            "join",
            "aggregate",
            "count",
            "transaction",
            "commit",
            "rollback",
            "upsert",
        ]
    }

    fn shell_api_method_names(&self) -> &'static [&'static str] {
        &[
            "exec",
            "spawn",
            "execFile",
            "execSync",
            "spawnSync",
            "system",
        ]
    }

    fn route_registration_patterns(&self) -> &'static [&'static str] {
        &[
            "app.get(",
            "app.post(",
            "app.put(",
            "app.delete(",
            "app.patch(",
            "router.get(",
            "router.post(",
            "fastify.get(",
            "hono.get(",
        ]
    }

    fn known_semantic_categories(&self) -> &'static [(&'static str, &'static [&'static str])] {
        JS_SEMANTIC_CATEGORIES
    }
}

// ── JavaScript spec ────────────────────────────────────────────────────────────

pub struct JavaScriptSpec;

impl LanguageSpec for JavaScriptSpec {
    fn name(&self) -> &'static str {
        "javascript"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["js", "jsx", "mjs", "cjs"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        #[cfg(feature = "javascript")]
        return tree_sitter_javascript::LANGUAGE.into();
        #[cfg(not(feature = "javascript"))]
        panic!("frensense-lang: 'javascript' feature not enabled");
    }

    fn classify(&self, kind: &str) -> NodeRole {
        classify_js(kind)
    }

    fn wrap_region(&self, code: &str) -> String {
        format!("function _region() {{\n{}\n}}", code)
    }

    fn import_node_kinds(&self) -> &'static [&'static str] {
        &["import_statement"]
    }

    fn extract_imports<'tree>(&self, root: Node<'tree>, source: &str) -> Vec<Import> {
        extract_js_imports(root, source)
    }

    fn symbol_query(&self) -> Option<&'static str> {
        Some(
            r#"
            (function_declaration name: (identifier) @name)
            (method_definition name: (property_identifier) @name)
            (class_declaration name: (identifier) @name)
            (variable_declarator name: (identifier) @name)
            (call_expression arguments: (arguments (arrow_function) @name))
            (call_expression arguments: (arguments (function_expression) @name))
        "#,
        )
    }
    fn call_query(&self) -> Option<&'static str> {
        TypeScriptSpec.call_query()
    }

    fn package_category(&self, pkg: &str) -> Option<PackageCategory> {
        js_package_category(pkg)
    }

    fn classify_param_taint(
        &self,
        name: Option<&str>,
        type_annotation: Option<&str>,
    ) -> Option<TaintOrigin> {
        classify_js_param(name, type_annotation)
    }

    fn request_param_names(&self) -> &'static [&'static str] {
        TypeScriptSpec.request_param_names()
    }

    fn known_sink_names(&self) -> &'static [(&'static str, crate::spec::SinkLabel)] {
        JS_SINK_NAMES
    }

    fn known_source_patterns(&self) -> &'static [&'static str] {
        JS_SOURCE_PATTERNS
    }

    fn propagator_rules(&self) -> &'static [PropagatorRule] {
        JS_PROPAGATORS
    }

    fn classify_sanitizer(&self, call: &str) -> Option<SanitizerKind> {
        js_classify_sanitizer(call)
    }

    fn route_context_hints(&self) -> &'static [&'static str] {
        &[
            "(req, res)",
            "app.get(",
            "router.get(",
            "app.post(",
            "router.post(",
            "res.send",
            "res.json",
            "res.status",
            "c.req",
            "c.json",
            "ctx.body",
            "ctx.response",
            "fastify.get(",
            "fastify.post(",
            "export async function GET(",
            "export async function POST(",
            "export async function PUT(",
            "export async function DELETE(",
            "export async function PATCH(",
            "export default function handler(",
            "publicProcedure.input(",
            "protectedProcedure.input(",
            "t.procedure",
            "Query: {",
            "Mutation: {",
            "Subscription: {",
            "resolve(",
            "export async function loader(",
            "export async function action(",
            "export async function load(",
            "export const GET = ",
            "export const POST = ",
            "io.on('connection'",
            "socket.on(",
            "Bun.serve(",
            "Deno.serve(",
            "Deno.serve({ handler",
        ]
    }

    fn test_context_hints(&self) -> &'static [&'static str] {
        TypeScriptSpec.test_context_hints()
    }

    fn response_method_names(&self) -> &'static [&'static str] {
        TypeScriptSpec.response_method_names()
    }

    fn db_api_method_names(&self) -> &'static [&'static str] {
        TypeScriptSpec.db_api_method_names()
    }

    fn shell_api_method_names(&self) -> &'static [&'static str] {
        TypeScriptSpec.shell_api_method_names()
    }

    fn route_registration_patterns(&self) -> &'static [&'static str] {
        TypeScriptSpec.route_registration_patterns()
    }

    fn known_semantic_categories(&self) -> &'static [(&'static str, &'static [&'static str])] {
        JS_SEMANTIC_CATEGORIES
    }
}

// ── Shared JS/TS param classification ─────────────────────────────────────────

fn classify_js_param(name: Option<&str>, ann: Option<&str>) -> Option<TaintOrigin> {
    // Type annotation from an HTTP framework package → confirmed user input
    if let Some(a) = ann {
        let base = a
            .trim_start_matches(':')
            .trim()
            .split(['<', '[', ' '])
            .next()
            .unwrap_or(a);
        let base = base.rsplit('.').next().unwrap_or(base);
        if matches!(
            base,
            "Request"
                | "IncomingMessage"
                | "FastifyRequest"
                | "Context"
                | "HonoContext"
                | "KoaContext"
                | "APIGatewayProxyEvent"
                | "HttpRequest"
                | "NextApiRequest"
                | "NextRequest"
                | "ServerRequest"
                | "H3Event"
                | "ElysiaContext"
                | "AdonisRequest"
                | "ExpressRequest"
                | "SocketStream"
                | "CloudFrontRequest"
                | "APIGatewayProxyEventV2"
        ) {
            return Some(TaintOrigin::UserInput);
        }
    }
    // Name-based fallback for untyped code
    match name? {
        "req" | "request" | "ctx" | "context" | "event" | "c" | "e" | "socket" | "ws"
        | "incomingMsg" | "httpReq" => Some(TaintOrigin::UserInput),
        "env" | "config" | "settings" => Some(TaintOrigin::EnvVariable),
        "filePath" | "filepath" | "filename" | "fileName" | "dir" | "directory" => {
            Some(TaintOrigin::FileSystem)
        }
        "response" | "reply" => Some(TaintOrigin::ExternalService),
        _ => None,
    }
}

// ── Static sink/source tables ─────────────────────────────────────────────────

static JS_SINK_NAMES: &[(&'static str, crate::spec::SinkLabel)] = &[
    // Modern attack patterns
    ("insertAdjacentHTML", crate::spec::SinkLabel::XssDom),
    ("insertAdjacentElement", crate::spec::SinkLabel::XssDom),
    ("createContextualFragment", crate::spec::SinkLabel::XssDom),
    ("write", crate::spec::SinkLabel::XssDom),
    ("writeln", crate::spec::SinkLabel::XssDom),
    ("srcdoc", crate::spec::SinkLabel::XssDom),
    ("setAttribute", crate::spec::SinkLabel::XssDom),
    ("setAttributeNS", crate::spec::SinkLabel::XssDom),
    ("setHeader", crate::spec::SinkLabel::HeaderInjection),
    ("header", crate::spec::SinkLabel::HeaderInjection),
    ("append", crate::spec::SinkLabel::HeaderInjection),
    ("cookie", crate::spec::SinkLabel::CookiePoisoning),
    ("type", crate::spec::SinkLabel::ContentTypeInjection),
    ("appendFile", crate::spec::SinkLabel::PathTraversal),
    ("appendFileSync", crate::spec::SinkLabel::PathTraversal),
    ("copyFile", crate::spec::SinkLabel::PathTraversal),
    ("copyFileSync", crate::spec::SinkLabel::PathTraversal),
    ("mkdir", crate::spec::SinkLabel::PathTraversal),
    ("mkdirSync", crate::spec::SinkLabel::PathTraversal),
    ("rename", crate::spec::SinkLabel::PathTraversal),
    ("renameSync", crate::spec::SinkLabel::PathTraversal),
    ("rmdir", crate::spec::SinkLabel::PathTraversal),
    ("rmdirSync", crate::spec::SinkLabel::PathTraversal),
    ("lstat", crate::spec::SinkLabel::PathTraversal),
    ("lstatSync", crate::spec::SinkLabel::PathTraversal),
    ("chmod", crate::spec::SinkLabel::PathTraversal),
    ("chown", crate::spec::SinkLabel::PathTraversal),
    ("symlink", crate::spec::SinkLabel::PathTraversal),
    ("realpath", crate::spec::SinkLabel::PathTraversal),
    ("realpathSync", crate::spec::SinkLabel::PathTraversal),
    ("createWriteStream", crate::spec::SinkLabel::PathTraversal),
    ("openSync", crate::spec::SinkLabel::PathTraversal),
    ("fopen", crate::spec::SinkLabel::PathTraversal),
    ("readdir", crate::spec::SinkLabel::PathTraversal),
    ("readdirSync", crate::spec::SinkLabel::PathTraversal),
    ("axios.put", crate::spec::SinkLabel::Ssrf),
    ("axios.delete", crate::spec::SinkLabel::Ssrf),
    ("axios.patch", crate::spec::SinkLabel::Ssrf),
    ("axios.request", crate::spec::SinkLabel::Ssrf),
    ("axios.head", crate::spec::SinkLabel::Ssrf),
    ("axios.options", crate::spec::SinkLabel::Ssrf),
    ("got.get", crate::spec::SinkLabel::Ssrf),
    ("got.post", crate::spec::SinkLabel::Ssrf),
    ("got.put", crate::spec::SinkLabel::Ssrf),
    ("got.stream", crate::spec::SinkLabel::Ssrf),
    ("superagent.get", crate::spec::SinkLabel::Ssrf),
    ("superagent.post", crate::spec::SinkLabel::Ssrf),
    ("ky.get", crate::spec::SinkLabel::Ssrf),
    ("ky.post", crate::spec::SinkLabel::Ssrf),
    ("http.request", crate::spec::SinkLabel::Ssrf),
    ("https.request", crate::spec::SinkLabel::Ssrf),
    ("undici.fetch", crate::spec::SinkLabel::Ssrf),
    ("undici.request", crate::spec::SinkLabel::Ssrf),
    ("needle.get", crate::spec::SinkLabel::Ssrf),
    ("needle.post", crate::spec::SinkLabel::Ssrf),
    ("execaCommand", crate::spec::SinkLabel::CommandInjection),
    ("execaCommandSync", crate::spec::SinkLabel::CommandInjection),
    ("$", crate::spec::SinkLabel::CommandInjection),
    ("shell.exec", crate::spec::SinkLabel::CommandInjection),
    ("shell.run", crate::spec::SinkLabel::CommandInjection),
    ("cp.exec", crate::spec::SinkLabel::CommandInjection),
    ("aggregate", crate::spec::SinkLabel::NoSqlInjection),
    ("distinct", crate::spec::SinkLabel::NoSqlInjection),
    ("count", crate::spec::SinkLabel::NoSqlInjection),
    ("countDocuments", crate::spec::SinkLabel::NoSqlInjection),
    (
        "estimatedDocumentCount",
        crate::spec::SinkLabel::NoSqlInjection,
    ),
    ("findOneAndUpdate", crate::spec::SinkLabel::NoSqlInjection),
    ("findOneAndDelete", crate::spec::SinkLabel::NoSqlInjection),
    ("findOneAndReplace", crate::spec::SinkLabel::NoSqlInjection),
    ("replaceOne", crate::spec::SinkLabel::NoSqlInjection),
    ("bulkWrite", crate::spec::SinkLabel::NoSqlInjection),
    ("set", crate::spec::SinkLabel::NoSqlInjection),
    ("hget", crate::spec::SinkLabel::NoSqlInjection),
    ("hset", crate::spec::SinkLabel::NoSqlInjection),
    ("del", crate::spec::SinkLabel::NoSqlInjection),
    ("keys", crate::spec::SinkLabel::NoSqlInjection),
    ("deepMerge", crate::spec::SinkLabel::PrototypePollution),
    ("merge", crate::spec::SinkLabel::PrototypePollution),
    ("defaults", crate::spec::SinkLabel::PrototypePollution),
    ("extend", crate::spec::SinkLabel::PrototypePollution),
    ("assign", crate::spec::SinkLabel::PrototypePollution),
    ("deepExtend", crate::spec::SinkLabel::PrototypePollution),
    ("mixin", crate::spec::SinkLabel::PrototypePollution),
    ("cloneDeep", crate::spec::SinkLabel::PrototypePollution),
    ("sign", crate::spec::SinkLabel::JwtWeakAlgorithm),
    ("decode", crate::spec::SinkLabel::JwtUnsafeDecode),
    ("existsSync", crate::spec::SinkLabel::Toctou),
    ("exists", crate::spec::SinkLabel::Toctou),
    ("gql", crate::spec::SinkLabel::GraphqlInjection),
    ("buildSchema", crate::spec::SinkLabel::GraphqlInjection),
    ("graphql", crate::spec::SinkLabel::GraphqlInjection),
    (
        "makeExecutableSchema",
        crate::spec::SinkLabel::GraphqlInjection,
    ),
    ("cp.spawn", crate::spec::SinkLabel::CommandInjection),
    ("cp.execFile", crate::spec::SinkLabel::CommandInjection),
    ("proc.exec", crate::spec::SinkLabel::CommandInjection),
    (
        "childProcess.exec",
        crate::spec::SinkLabel::CommandInjection,
    ),
    // Code Execution
    ("eval", crate::spec::SinkLabel::CodeExecution),
    ("Function", crate::spec::SinkLabel::CodeExecution),
    ("setTimeout", crate::spec::SinkLabel::CodeExecution),
    ("setInterval", crate::spec::SinkLabel::CodeExecution),
    ("runInNewContext", crate::spec::SinkLabel::CodeExecution),
    ("runInThisContext", crate::spec::SinkLabel::CodeExecution),
    ("require", crate::spec::SinkLabel::CodeExecution),
    ("import", crate::spec::SinkLabel::CodeExecution),
    // Command Injection
    ("exec", crate::spec::SinkLabel::CommandInjection),
    ("execSync", crate::spec::SinkLabel::CommandInjection),
    ("spawn", crate::spec::SinkLabel::CommandInjection),
    ("spawnSync", crate::spec::SinkLabel::CommandInjection),
    ("execFile", crate::spec::SinkLabel::CommandInjection),
    ("execFileSync", crate::spec::SinkLabel::CommandInjection),
    ("shelljs.exec", crate::spec::SinkLabel::CommandInjection),
    ("execa", crate::spec::SinkLabel::CommandInjection),
    // SQL Injection
    ("query", crate::spec::SinkLabel::SqlInjection),
    ("execute", crate::spec::SinkLabel::SqlInjection),
    ("executeRaw", crate::spec::SinkLabel::SqlInjection),
    ("queryRaw", crate::spec::SinkLabel::SqlInjection),
    ("raw", crate::spec::SinkLabel::SqlInjection),
    ("prepare", crate::spec::SinkLabel::SqlInjection),
    // Path Traversal
    ("readFile", crate::spec::SinkLabel::PathTraversal),
    ("readFileSync", crate::spec::SinkLabel::PathTraversal),
    ("createReadStream", crate::spec::SinkLabel::PathTraversal),
    ("writeFile", crate::spec::SinkLabel::PathTraversal),
    ("join", crate::spec::SinkLabel::PathTraversal),
    ("unlink", crate::spec::SinkLabel::PathTraversal),
    ("stat", crate::spec::SinkLabel::PathTraversal),
    ("access", crate::spec::SinkLabel::PathTraversal),
    // SSRF
    ("fetch", crate::spec::SinkLabel::Ssrf),
    ("axios.get", crate::spec::SinkLabel::Ssrf),
    ("axios.post", crate::spec::SinkLabel::Ssrf),
    ("http.get", crate::spec::SinkLabel::Ssrf),
    ("https.get", crate::spec::SinkLabel::Ssrf),
    ("got", crate::spec::SinkLabel::Ssrf),
    ("get", crate::spec::SinkLabel::Ssrf),
    ("post", crate::spec::SinkLabel::Ssrf),
    ("request", crate::spec::SinkLabel::Ssrf),
    ("node-fetch", crate::spec::SinkLabel::Ssrf),
    // Open Redirect
    ("redirect", crate::spec::SinkLabel::OpenRedirect),
    ("location.href", crate::spec::SinkLabel::OpenRedirect),
    ("window.location", crate::spec::SinkLabel::OpenRedirect),
    // XSS
    ("innerHTML", crate::spec::SinkLabel::XssDom),
    ("outerHTML", crate::spec::SinkLabel::XssDom),
    ("document.write", crate::spec::SinkLabel::XssDom),
    ("document.writeln", crate::spec::SinkLabel::XssDom),
    ("dangerouslySetInnerHTML", crate::spec::SinkLabel::XssDom),
    // MongoDB / ORM
    ("update", crate::spec::SinkLabel::NoSqlInjection),
    ("updateOne", crate::spec::SinkLabel::NoSqlInjection),
    ("updateMany", crate::spec::SinkLabel::NoSqlInjection),
    ("insert", crate::spec::SinkLabel::NoSqlInjection),
    ("insertOne", crate::spec::SinkLabel::NoSqlInjection),
    ("insertMany", crate::spec::SinkLabel::NoSqlInjection),
    ("delete", crate::spec::SinkLabel::NoSqlInjection),
    ("deleteOne", crate::spec::SinkLabel::NoSqlInjection),
    ("deleteMany", crate::spec::SinkLabel::NoSqlInjection),
    ("find", crate::spec::SinkLabel::NoSqlInjection),
    ("findOne", crate::spec::SinkLabel::NoSqlInjection),
    ("findAll", crate::spec::SinkLabel::NoSqlInjection),
    // Storage Write
    ("put", crate::spec::SinkLabel::StorageWrite),
    ("setItem", crate::spec::SinkLabel::StorageWrite),
    // Log Leak
    ("log", crate::spec::SinkLabel::LogLeak),
    ("error", crate::spec::SinkLabel::LogLeak),
    ("info", crate::spec::SinkLabel::LogLeak),
    ("debug", crate::spec::SinkLabel::LogLeak),
    // SSTI - Template engine renders
    ("ejs.render", crate::spec::SinkLabel::TemplateSsti),
    ("ejs.renderFile", crate::spec::SinkLabel::TemplateSsti),
    ("pug.compile", crate::spec::SinkLabel::TemplateSsti),
    ("pug.render", crate::spec::SinkLabel::TemplateSsti),
    ("handlebars.compile", crate::spec::SinkLabel::TemplateSsti),
    ("handlebars.render", crate::spec::SinkLabel::TemplateSsti),
    ("nunjucks.render", crate::spec::SinkLabel::TemplateSsti),
    (
        "nunjucks.renderString",
        crate::spec::SinkLabel::TemplateSsti,
    ),
    ("nunjucks.renderFile", crate::spec::SinkLabel::TemplateSsti),
    ("marko.render", crate::spec::SinkLabel::TemplateSsti),
    ("eta.render", crate::spec::SinkLabel::TemplateSsti),
    ("swig.render", crate::spec::SinkLabel::TemplateSsti),
    ("liquid.render", crate::spec::SinkLabel::TemplateSsti),
    ("mustache.render", crate::spec::SinkLabel::TemplateSsti),
    ("jade.render", crate::spec::SinkLabel::TemplateSsti),
    (
        "react-dom/server.renderToString",
        crate::spec::SinkLabel::TemplateSsti,
    ),
    (
        "vue-server-renderer.renderToString",
        crate::spec::SinkLabel::TemplateSsti,
    ),
    // Insecure Deserialization
    ("serialize", crate::spec::SinkLabel::UnsafeDeserialize),
    ("deserialize", crate::spec::SinkLabel::UnsafeDeserialize),
    ("yaml.load", crate::spec::SinkLabel::UnsafeDeserialize),
    ("js-yaml.load", crate::spec::SinkLabel::UnsafeDeserialize),
    ("msgpack.decode", crate::spec::SinkLabel::UnsafeDeserialize),
    ("msgpack.unpack", crate::spec::SinkLabel::UnsafeDeserialize),
    // Prototype Pollution
    ("Object.assign", crate::spec::SinkLabel::PrototypePollution),
    ("_.merge", crate::spec::SinkLabel::PrototypePollution),
    ("lodash.merge", crate::spec::SinkLabel::PrototypePollution),
    ("_.defaultsDeep", crate::spec::SinkLabel::PrototypePollution),
    ("_.set", crate::spec::SinkLabel::PrototypePollution),
    ("$.extend", crate::spec::SinkLabel::PrototypePollution),
    ("jQuery.extend", crate::spec::SinkLabel::PrototypePollution),
    ("angular.merge", crate::spec::SinkLabel::PrototypePollution),
    ("setPrototypeOf", crate::spec::SinkLabel::PrototypePollution),
    // XXE
    ("DOMParser", crate::spec::SinkLabel::Xxe),
    // JWT
    ("jwt.verify", crate::spec::SinkLabel::Jwt),
    ("jwt.decode", crate::spec::SinkLabel::Jwt),
    ("jwt.sign", crate::spec::SinkLabel::Jwt),
    ("jsonwebtoken.verify", crate::spec::SinkLabel::Jwt),
    ("jsonwebtoken.decode", crate::spec::SinkLabel::Jwt),
    ("jsonwebtoken.sign", crate::spec::SinkLabel::Jwt),
    ("JWT.verify", crate::spec::SinkLabel::Jwt),
    ("JWT.decode", crate::spec::SinkLabel::Jwt),
    // Cloudflare Workers / Prisma
    ("c.redirect", crate::spec::SinkLabel::OpenRedirect),
    ("env.KV.put", crate::spec::SinkLabel::StorageWrite),
    ("KVNamespace.put", crate::spec::SinkLabel::StorageWrite),
    ("KVNamespace.delete", crate::spec::SinkLabel::StorageWrite),
    ("env.DB.prepare", crate::spec::SinkLabel::SqlInjection),
    ("res.send", crate::spec::SinkLabel::ResponseLeak),
    ("res.json", crate::spec::SinkLabel::ResponseLeak),
    ("res.redirect", crate::spec::SinkLabel::OpenRedirect),
    ("res.render", crate::spec::SinkLabel::TemplateSsti),
    ("revalidatePath", crate::spec::SinkLabel::StorageWrite),
    (
        "prisma.queryRawUnsafe",
        crate::spec::SinkLabel::SqlInjection,
    ),
    (
        "prisma.executeRawUnsafe",
        crate::spec::SinkLabel::SqlInjection,
    ),
    ("R2Bucket.put", crate::spec::SinkLabel::StorageWrite),
    ("D1Database.prepare", crate::spec::SinkLabel::SqlInjection),
    ("DurableObjectStub.fetch", crate::spec::SinkLabel::Ssrf),
    ("Queue.send", crate::spec::SinkLabel::Ssrf),
];

static JS_SOURCE_PATTERNS: &[&str] = &[
    "req.session",
    "req.session.userId",
    "req.session.user",
    "session.",
    "req.url",
    "req.path",
    "req.hostname",
    "req.ip",
    "req.protocol",
    "req.originalUrl",
    "req.subdomains",
    "ws.data",
    "socket.data",
    "msg.data",
    "message.data",
    "ctx.request.query",
    "ctx.request.headers",
    "ctx.request.url",
    "ctx.state",
    "c.req.header",
    "c.req.path",
    "c.req.url",
    "c.req.json",
    "c.req.text",
    "c.req.formData",
    "event.headers",
    "event.requestContext",
    "event.multiValueQueryStringParameters",
    "event.isBase64Encoded",
    "req.body",
    "req.query",
    "req.params",
    "req.headers",
    "req.cookies",
    "req.file",
    "req.files",
    "request.body",
    "request.query",
    "request.params",
    "ctx.request.body",
    "ctx.query",
    "ctx.params",
    "c.req.raw",
    "c.req.query",
    "c.req.param",
    "c.req",
    "event.body",
    "event.queryStringParameters",
    "event.pathParameters",
    "process.env",
    "process.argv",
    // Hardened for object destructuring: const { body, query, params } = req
    "body",
    "query",
    "params",
    "headers",
    "cookies",
    "file",
    "files",
];

static JS_SEMANTIC_CATEGORIES: &[(&str, &[&str])] = &[
    (
        "db_query",
        &[
            "query",
            "execute",
            "rawQuery",
            "sql_query",
            "executeQuery",
            "run",
            "all",
            "get",
            "find",
            "findOne",
            "findById",
            "aggregate",
            "count",
        ],
    ),
    (
        "db_write",
        &[
            "insert",
            "update",
            "upsert",
            "bulkWrite",
            "create",
            "delete",
            "remove",
            "save",
            "patch",
            "put",
        ],
    ),
    (
        "cmd_exec",
        &[
            "exec",
            "execSync",
            "spawn",
            "spawnSync",
            "execFile",
            "execFileSync",
            "system",
            "popen",
            "child_process",
            "fork",
        ],
    ),
    ("code_eval", &["eval", "Function", "new Function"]),
    (
        "file_read",
        &[
            "readFile",
            "readFileSync",
            "createReadStream",
            "readdir",
            "readdirSync",
        ],
    ),
    (
        "file_write",
        &[
            "writeFile",
            "writeFileSync",
            "createWriteStream",
            "appendFile",
            "appendFileSync",
        ],
    ),
    (
        "dom_xss",
        &[
            "innerHTML",
            "outerHTML",
            "document.write",
            "insertAdjacentHTML",
            "insertAdjacentText",
        ],
    ),
    (
        "http_request",
        &[
            "fetch",
            "axios",
            "request",
            "get",
            "post",
            "put",
            "delete",
            "patch",
            "superagent",
            "got",
            "undici",
            "node-fetch",
        ],
    ),
    ("url_redirect", &["redirect", "location"]),
    ("crypto_weak", &["md5", "sha1", "createHash", "createHmac"]),
    (
        "crypto_strong",
        &["sha256", "sha512", "bcrypt", "argon2", "scrypt"],
    ),
    (
        "deserialize",
        &["JSON.parse", "loads", "deserialize", "unmarshal", "decode"],
    ),
    (
        "sanitize",
        &["sanitize", "escape", "encode", "validate", "escapeHtml"],
    ),
    ("regex", &["new RegExp", "RegExp"]),
    ("process", &["exit", "kill", "process"]),
    (
        "auth_middleware",
        &[
            "verify",
            "decode",
            "verifyToken",
            "authenticate",
            "authorize",
        ],
    ),
    ("weak_random", &["random", "Math.random"]),
    (
        "financial_calc",
        &["price", "priceSnapshot", "total", "amount", "balance"],
    ),
];
