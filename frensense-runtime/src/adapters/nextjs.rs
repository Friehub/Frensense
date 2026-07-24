use std::path::Path;

use crate::route_extractor::{Framework, HttpMethod, InjectionPoint, ParameterLocation, RouteBinding};

use super::{extract_by_patterns, http_method_from_str, AuthConvention, FrameworkAdapter};

pub struct NextJsAdapter;

impl FrameworkAdapter for NextJsAdapter {
    fn name(&self) -> &'static str { "Next.js" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "tsx", "js"] }
    fn framework_enum(&self) -> Framework { Framework::NextJs }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let path_str = file_path.to_string_lossy();
        let route_path = if let Some(pos) = path_str.find("/app/api/") {
            let rel = &path_str[pos + 4..];
            rel.replace("/route.ts", "").replace("/route.js", "")
               .replace("[", ":").replace("]", "")
        } else if let Some(pos) = path_str.find("/pages/api/") {
            let rel = &path_str[pos + 11..];
            format!("/api/{}", rel.replace(".ts", "").replace(".js", "")
                                  .replace("[", ":").replace("]", ""))
        } else {
            return Vec::new();
        };

        let method_re = regex::Regex::new(
            r"export\s+(?:async\s+)?function\s+(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)"
        ).unwrap();

        let mut routes: Vec<RouteBinding> = method_re.captures_iter(source).map(|cap| {
            RouteBinding {
                method: http_method_from_str(&cap[1].to_lowercase()),
                path_pattern: route_path.clone(),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: cap[1].to_string(),
                injection_points: Vec::new(),
                framework: Framework::NextJs,
            }
        }).collect();

        if routes.is_empty() && path_str.contains("/app/api/") {
            routes.push(RouteBinding {
                method: HttpMethod::Post,
                path_pattern: route_path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: "handler".to_string(),
                injection_points: Vec::new(),
                framework: Framework::NextJs,
            });
        }
        routes
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(body, &[
            (r"req\.body\.(\w+)",                        ParameterLocation::Body),
            (r"req\.query\.(\w+)",                       ParameterLocation::Query),
            (r#"searchParams\.get\(['"](\w+)['"]"#,      ParameterLocation::Query),
            (r"params\.(\w+)",                           ParameterLocation::PathParam),
        ]);
        if body.contains("request.json()") || body.contains("await req.json()") {
            points.push(InjectionPoint {
                location: ParameterLocation::Body,
                name: "_body".to_string(),
                taint_origin: Some("user_input".to_string()),
            });
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npx".to_string(), "next".to_string(), "dev".to_string()])
    }

    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
