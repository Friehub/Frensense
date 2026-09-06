use std::path::Path;

use crate::route_extractor::{
    Framework, HttpMethod, InjectionPoint, ParameterLocation, RouteBinding,
};

use super::{AuthConvention, FrameworkAdapter, extract_by_patterns};

pub struct RemixAdapter;

impl FrameworkAdapter for RemixAdapter {
    fn name(&self) -> &'static str {
        "Remix"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "tsx", "js"]
    }
    fn framework_enum(&self) -> Framework {
        Framework::Remix
    }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let path_str = file_path.to_string_lossy();
        let route_path = if let Some(pos) = path_str.find("/routes/") {
            let rel = &path_str[pos + 8..];
            let without_ext = rel
                .replace(".ts", "")
                .replace(".tsx", "")
                .replace(".js", "");
            format!("/{}", without_ext.replace('.', "/").replace('$', ":"))
        } else {
            return Vec::new();
        };

        let has_loader = source.contains("export async function loader")
            || source.contains("export function loader");
        let has_action = source.contains("export async function action")
            || source.contains("export function action");

        let mut routes = Vec::new();
        if has_loader {
            routes.push(RouteBinding {
                method: HttpMethod::Get,
                path_pattern: route_path.clone(),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: "loader".to_string(),
                injection_points: Vec::new(),
                framework: Framework::Remix,
            });
        }
        if has_action {
            routes.push(RouteBinding {
                method: HttpMethod::Post,
                path_pattern: route_path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: "action".to_string(),
                injection_points: Vec::new(),
                framework: Framework::Remix,
            });
        }
        routes
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        let mut points = extract_by_patterns(
            body,
            &[
                (
                    r#"searchParams\.get\(['"](\w+)['"]"#,
                    ParameterLocation::Query,
                ),
                (r"params\.(\w+)", ParameterLocation::PathParam),
                (
                    r#"formData\.get\(['"]\w+['"]\)"#,
                    ParameterLocation::FormData,
                ),
            ],
        );
        if body.contains("request.json()") {
            points.push(super::body_point());
        }
        if body.contains("request.formData()") {
            points.push(super::form_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec![
            "npx".to_string(),
            "remix".to_string(),
            "dev".to_string(),
        ])
    }

    fn auth_convention(&self) -> AuthConvention {
        AuthConvention::SessionCookie("__session".to_string())
    }
}
