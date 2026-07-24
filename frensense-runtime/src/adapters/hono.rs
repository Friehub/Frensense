use std::path::Path;

use crate::route_extractor::{Framework, InjectionPoint, ParameterLocation, RouteBinding};

use super::{extract_by_patterns, http_method_from_str, AuthConvention, FrameworkAdapter};

pub struct HonoAdapter;

impl FrameworkAdapter for HonoAdapter {
    fn name(&self) -> &'static str { "Hono" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "js"] }
    fn framework_enum(&self) -> Framework { Framework::Unknown }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let route_re = regex::Regex::new(
            r#"(?:app|router)\.(get|post|put|delete|patch)\s*\(\s*['"`]([^'"`]+)['"`]"#
        ).unwrap();
        route_re.captures_iter(source).map(|cap| {
            let method = http_method_from_str(&cap[1]);
            RouteBinding {
                method,
                path_pattern: cap[2].to_string(),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: String::new(),
                injection_points: Vec::new(),
                framework: Framework::Unknown,
            }
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(body, &[
            (r#"c\.req\.param\(['"](\w+)['"]"#,  ParameterLocation::PathParam),
            (r#"c\.req\.query\(['"](\w+)['"]"#,  ParameterLocation::Query),
            (r#"c\.req\.header\(['"]([^'"]+)['"]"#, ParameterLocation::Header),
        ]);
        if body.contains("c.req.json()") || body.contains("c.req.parseBody()") {
            points.push(super::body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "dev".to_string()])
    }

    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
