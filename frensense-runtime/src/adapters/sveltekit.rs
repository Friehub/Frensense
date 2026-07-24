use std::path::Path;

use crate::route_extractor::{Framework, InjectionPoint, ParameterLocation, RouteBinding};

use super::{extract_by_patterns, http_method_from_str, AuthConvention, FrameworkAdapter};

pub struct SvelteKitAdapter;

impl FrameworkAdapter for SvelteKitAdapter {
    fn name(&self) -> &'static str { "SvelteKit" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "js"] }
    fn framework_enum(&self) -> Framework { Framework::SvelteKit }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let path_str = file_path.to_string_lossy();
        let route_path = if let Some(pos) = path_str.find("/routes/") {
            let rel = &path_str[pos + 8..];
            rel.replace("+server.ts", "").replace("+server.js", "")
               .replace("[", ":").replace("]", "")
               .trim_end_matches('/').to_string()
        } else {
            return Vec::new();
        };

        let method_re = regex::Regex::new(
            r"export\s+(?:const|async function)\s+(GET|POST|PUT|DELETE|PATCH)"
        ).unwrap();

        method_re.captures_iter(source).map(|cap| {
            let method = http_method_from_str(&cap[1].to_lowercase());
            RouteBinding {
                method,
                path_pattern: format!("/{}", route_path),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: cap[1].to_string(),
                injection_points: Vec::new(),
                framework: Framework::SvelteKit,
            }
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(body, &[
            (r#"(?:event\.)?url\.searchParams\.get\(['"](\w+)['"]"#, ParameterLocation::Query),
            (r"(?:event\.)?params\.(\w+)", ParameterLocation::PathParam),
        ]);
        if body.contains("event.request.json()") || body.contains("request.json()") {
            points.push(super::body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "dev".to_string()])
    }

    fn auth_convention(&self) -> AuthConvention {
        AuthConvention::SessionCookie("session".to_string())
    }
}
