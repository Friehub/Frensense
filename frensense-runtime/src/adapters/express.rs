use std::path::Path;

use crate::route_extractor::{Framework, InjectionPoint, ParameterLocation, RouteBinding};

use super::{AuthConvention, FrameworkAdapter, extract_by_patterns, http_method_from_str};

pub struct ExpressAdapter;
pub struct FastifyAdapter;

impl FrameworkAdapter for ExpressAdapter {
    fn name(&self) -> &'static str {
        "Express"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "js"]
    }
    fn framework_enum(&self) -> Framework {
        Framework::Express
    }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let route_re = regex::Regex::new(
            r#"(?:app|router|server)\.(get|post|put|delete|patch|all)\s*\(\s*['"`]([^'"`]+)['"`]"#,
        )
        .unwrap();
        route_re
            .captures_iter(source)
            .map(|cap| {
                let method = http_method_from_str(&cap[1]);
                RouteBinding {
                    method,
                    path_pattern: cap[2].to_string(),
                    handler_file: file_path.to_string_lossy().to_string(),
                    handler_function: String::new(),
                    injection_points: Vec::new(),
                    framework: Framework::Express,
                }
            })
            .collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(
            body,
            &[
                (r"req\.body\.(\w+)", ParameterLocation::Body),
                (r"req\.query\.(\w+)", ParameterLocation::Query),
                (r"req\.params\.(\w+)", ParameterLocation::PathParam),
                (
                    r#"req\.headers\[?['"]?(\w[\w-]*)['"]?\]?"#,
                    ParameterLocation::Header,
                ),
                (r"req\.cookies\.(\w+)", ParameterLocation::Cookie),
            ],
        );
        if body.contains("req.body") {
            points.push(super::body_point());
        }
        points
    }

    fn startup_command(&self, root: &Path) -> Option<Vec<String>> {
        for entry in &[
            "src/index.ts",
            "src/app.ts",
            "index.ts",
            "app.ts",
            "server.ts",
        ] {
            if root.join(entry).exists() {
                return Some(vec![
                    "npx".to_string(),
                    "ts-node".to_string(),
                    entry.to_string(),
                ]);
            }
        }
        Some(vec![
            "npm".to_string(),
            "run".to_string(),
            "start".to_string(),
        ])
    }

    fn auth_convention(&self) -> AuthConvention {
        AuthConvention::BearerToken
    }
}

impl FrameworkAdapter for FastifyAdapter {
    fn name(&self) -> &'static str {
        "Fastify"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "js"]
    }
    fn framework_enum(&self) -> Framework {
        Framework::Fastify
    }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let route_re = regex::Regex::new(
            r#"(?:fastify|server|app)\.(get|post|put|delete|patch)\s*\(\s*['"`]([^'"`]+)['"`]"#,
        )
        .unwrap();
        route_re
            .captures_iter(source)
            .map(|cap| {
                let method = http_method_from_str(&cap[1]);
                RouteBinding {
                    method,
                    path_pattern: cap[2].to_string(),
                    handler_file: file_path.to_string_lossy().to_string(),
                    handler_function: String::new(),
                    injection_points: Vec::new(),
                    framework: Framework::Fastify,
                }
            })
            .collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(
            body,
            &[
                (r"request\.body\.(\w+)", ParameterLocation::Body),
                (r"request\.query\.(\w+)", ParameterLocation::Query),
                (r"request\.params\.(\w+)", ParameterLocation::PathParam),
                (r"request\.headers\.(\w+)", ParameterLocation::Header),
            ],
        );
        if body.contains("request.body") || body.contains("request.json()") {
            points.push(super::body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec![
            "npm".to_string(),
            "run".to_string(),
            "start".to_string(),
        ])
    }

    fn auth_convention(&self) -> AuthConvention {
        AuthConvention::BearerToken
    }
}
