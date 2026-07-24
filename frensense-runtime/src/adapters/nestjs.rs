use std::path::Path;

use crate::route_extractor::{Framework, InjectionPoint, ParameterLocation, RouteBinding};

use super::{extract_by_patterns, http_method_from_str, AuthConvention, FrameworkAdapter};

pub struct NestJsAdapter;

impl FrameworkAdapter for NestJsAdapter {
    fn name(&self) -> &'static str { "NestJS" }
    fn extensions(&self) -> &'static [&'static str] { &["ts"] }
    fn framework_enum(&self) -> Framework { Framework::NestJs }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let controller_re = regex::Regex::new(
            r#"@Controller\s*\(\s*['"`]?([^'"`\)]*?)['"`]?\s*\)"#
        ).unwrap();
        let method_re = regex::Regex::new(
            r#"@(Get|Post|Put|Delete|Patch)\s*\(\s*['"`]?([^'"`\)]*?)['"`]?\s*\)"#
        ).unwrap();

        let base_path = controller_re
            .captures(source)
            .map(|c| c[1].trim().to_string())
            .unwrap_or_default();

        method_re.captures_iter(source).map(|cap| {
            let method = http_method_from_str(&cap[1].to_lowercase());
            let sub_path = cap[2].trim().to_string();
            let full_path = format!("/{}/{}", base_path, sub_path)
                .replace("//", "/");
            RouteBinding {
                method,
                path_pattern: full_path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: String::new(),
                injection_points: Vec::new(),
                framework: Framework::NestJs,
            }
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(body, &[
            (r#"@Param\(['"](\w+)['"]"#,  ParameterLocation::PathParam),
            (r#"@Query\(['"](\w+)['"]"#,  ParameterLocation::Query),
            (r#"@Headers\(['"](\w+)['"]"#, ParameterLocation::Header),
            (r"body\.(\w+)",               ParameterLocation::Body),
        ]);
        if body.contains("@Body()") {
            points.push(super::body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "start:dev".to_string()])
    }

    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
