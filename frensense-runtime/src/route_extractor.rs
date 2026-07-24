use std::path::Path;
use tree_sitter::{Language, Node, Parser};

use frensense::Advisory;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteBinding {
    pub method: HttpMethod,
    pub path_pattern: String,
    pub handler_file: String,
    pub handler_function: String,
    pub injection_points: Vec<InjectionPoint>,
    pub framework: Framework,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InjectionPoint {
    pub location: ParameterLocation,
    pub name: String,
    pub taint_origin: Option<String>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ParameterLocation {
    Body,
    Query,
    PathParam,
    Header,
    Cookie,
    FormData,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Framework {
    Express,
    Fastify,
    Koa,
    NestJs,
    NextJs,
    Remix,
    SvelteKit,
    Hono,
    Astro,
    Trpc,
    Actix,
    AxumRust,
    Rocket,
    Warp,
    Tonic,
    GoNetHttp,
    GoGin,
    GoEcho,
    GoChi,
    GorillaMux,
    GoFiber,
    Unknown,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Patch => write!(f, "PATCH"),
            Self::All => write!(f, "ALL"),
        }
    }
}

pub fn extract_routes(file_path: &Path, source: &str, lang: Language) -> Vec<RouteBinding> {
    let mut parser = Parser::new();
    parser.set_language(&lang).ok();
    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let mut routes = Vec::new();
    extract_routes_recursive(tree.root_node(), source, &mut routes, file_path);
    routes
}

fn extract_routes_recursive(
    node: Node<'_>,
    source: &str,
    routes: &mut Vec<RouteBinding>,
    file: &Path,
) {
    if node.kind() == "call_expression" {
        if let Some(func) = node.child_by_field_name("function") {
            let callee = &source[func.start_byte()..func.end_byte()];
            if let Some((method, framework)) = detect_route_registration(callee) {
                if let Some(binding) =
                    extract_express_route(node, source, file, method, framework)
                {
                    routes.push(binding);
                    return;
                }
            }
        }
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_routes_recursive(cursor.node(), source, routes, file);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn detect_route_registration(callee: &str) -> Option<(HttpMethod, Framework)> {
    let segments: Vec<&str> = callee.splitn(2, '.').collect();
    if segments.len() == 2 {
        let method = match segments[1] {
            "get" => Some(HttpMethod::Get),
            "post" => Some(HttpMethod::Post),
            "put" => Some(HttpMethod::Put),
            "delete" => Some(HttpMethod::Delete),
            "patch" => Some(HttpMethod::Patch),
            "all" => Some(HttpMethod::All),
            "route" => Some(HttpMethod::Post),
            _ => None,
        };
        if let Some(m) = method {
            let fw = detect_framework(segments[0]);
            return Some((m, fw));
        }
    }
    if callee == "http.HandleFunc" || callee.ends_with(".HandleFunc") {
        return Some((HttpMethod::All, Framework::GoNetHttp));
    }
    None
}

fn detect_framework(receiver: &str) -> Framework {
    match receiver {
        "app" | "router" | "express" => Framework::Express,
        "fastify" | "server" => Framework::Fastify,
        "r" | "engine" => Framework::GoGin,
        "e" => Framework::GoEcho,
        _ => Framework::Unknown,
    }
}

fn extract_express_route(
    node: Node<'_>,
    source: &str,
    file: &Path,
    method: HttpMethod,
    framework: Framework,
) -> Option<RouteBinding> {
    let args = node.child_by_field_name("arguments")?;
    let mut arg_nodes: Vec<Node<'_>> = Vec::new();
    let mut cursor = args.walk();
    if cursor.goto_first_child() {
        loop {
            let n = cursor.node();
            if n.kind() != "," {
                arg_nodes.push(n);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    if arg_nodes.len() < 2 {
        return None;
    }
    let path_pattern = source[arg_nodes[0].start_byte()..arg_nodes[0].end_byte()]
        .trim_matches('"')
        .to_string();
    let handler_node = arg_nodes[1];
    let handler_function = source[handler_node.start_byte()..handler_node.end_byte()].to_string();
    let handler_file = file.display().to_string();

    Some(RouteBinding {
        method,
        path_pattern,
        handler_file,
        handler_function,
        injection_points: vec![],
        framework,
    })
}

pub fn match_finding_to_route<'a>(
    advisory: &Advisory,
    routes: &'a [RouteBinding],
) -> Option<&'a RouteBinding> {
    routes
        .iter()
        .find(|r| {
            let file_match = r.handler_file.ends_with(&advisory.file_path)
                || advisory.file_path.ends_with(&r.handler_file);
            let fn_match = advisory.enclosing_symbol.as_deref().map_or(false, |sym| {
                sym == r.handler_function || r.handler_function.contains(sym)
            });
            file_match && fn_match
        })
        .or_else(|| {
            routes.iter().find(|r| r.handler_file.ends_with(&advisory.file_path))
        })
}

pub fn extract_injection_points_from_advisory(advisory: &Advisory) -> Vec<InjectionPoint> {
    let mut points = Vec::new();
    let content = &advisory.original_content;

    let patterns: &[(&str, ParameterLocation)] = &[
        ("req.body.", ParameterLocation::Body),
        ("req.query.", ParameterLocation::Query),
        ("req.params.", ParameterLocation::PathParam),
        ("req.headers.", ParameterLocation::Header),
        ("req.cookies.", ParameterLocation::Cookie),
        ("ctx.request.", ParameterLocation::Body),
        ("r.URL.Query()", ParameterLocation::Query),
        ("r.FormValue(", ParameterLocation::FormData),
        ("c.Query(", ParameterLocation::Query),
        ("c.PostForm(", ParameterLocation::Body),
    ];

    for (prefix, location) in patterns {
        let mut rest = content.as_str();
        while let Some(pos) = rest.find(prefix) {
            let after = &rest[pos + prefix.len()..];
            let name_end = after
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after.len());
            let name = after[..name_end].to_string();
            if !name.is_empty() && !points.iter().any(|p: &InjectionPoint| p.name == name) {
                points.push(InjectionPoint {
                    location: *location,
                    name,
                    taint_origin: Some("user_input".to_string()),
                });
            }
            rest = &rest[pos + prefix.len()..];
        }
    }
    points
}
