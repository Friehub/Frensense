pub mod detector;
pub mod express;
pub mod hono;
pub mod nestjs;
pub mod nextjs;
pub mod remix;
pub mod sveltekit;

use std::path::Path;

use crate::route_extractor::{Framework, InjectionPoint, ParameterLocation, RouteBinding};
use base64::Engine;

pub trait FrameworkAdapter: Send + Sync {
    fn name(&self) -> &'static str;

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding>;

    fn extract_injection_points(&self, function_body: &str) -> Vec<InjectionPoint>;

    fn startup_command(&self, project_root: &Path) -> Option<Vec<String>>;

    fn auth_convention(&self) -> AuthConvention;

    fn extensions(&self) -> &'static [&'static str];

    fn framework_enum(&self) -> Framework;
}

#[derive(Debug, Clone)]
pub enum AuthConvention {
    BearerToken,
    SessionCookie(String),
    ApiKeyHeader(String),
    BasicAuth,
    None,
}

impl AuthConvention {
    pub fn apply_auth(
        &self,
        request: reqwest::RequestBuilder,
        token: &str,
    ) -> reqwest::RequestBuilder {
        match self {
            AuthConvention::BearerToken => {
                request.header("Authorization", format!("Bearer {token}"))
            }
            AuthConvention::SessionCookie(name) => {
                request.header("Cookie", format!("{name}={token}"))
            }
            AuthConvention::ApiKeyHeader(name) => request.header(name.as_str(), token),
            AuthConvention::BasicAuth => {
                let encoded = base64::engine::general_purpose::STANDARD.encode(format!(":{token}"));
                request.header("Authorization", format!("Basic {encoded}"))
            }
            AuthConvention::None => request,
        }
    }
}

pub struct UnknownAdapter;
impl FrameworkAdapter for UnknownAdapter {
    fn name(&self) -> &'static str {
        "Unknown"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &[]
    }
    fn extract_routes(&self, _file_path: &Path, _source: &str) -> Vec<RouteBinding> {
        Vec::new()
    }
    fn extract_injection_points(&self, _body: &str) -> Vec<InjectionPoint> {
        Vec::new()
    }
    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        None
    }
    fn auth_convention(&self) -> AuthConvention {
        AuthConvention::None
    }
    fn framework_enum(&self) -> Framework {
        Framework::Unknown
    }
}

pub fn http_method_from_str(s: &str) -> crate::route_extractor::HttpMethod {
    use crate::route_extractor::HttpMethod;
    match s.to_lowercase().as_str() {
        "get" => HttpMethod::Get,
        "post" => HttpMethod::Post,
        "put" => HttpMethod::Put,
        "delete" => HttpMethod::Delete,
        "patch" => HttpMethod::Patch,
        "all" | "any" => HttpMethod::All,
        _ => HttpMethod::Post,
    }
}

pub fn body_point() -> InjectionPoint {
    InjectionPoint {
        location: ParameterLocation::Body,
        name: "_body".to_string(),
        taint_origin: Some("user_input".to_string()),
    }
}

pub fn query_point() -> InjectionPoint {
    InjectionPoint {
        location: ParameterLocation::Query,
        name: "_query".to_string(),
        taint_origin: Some("user_input".to_string()),
    }
}

pub fn path_point() -> InjectionPoint {
    InjectionPoint {
        location: ParameterLocation::PathParam,
        name: "_path".to_string(),
        taint_origin: Some("user_input".to_string()),
    }
}

pub fn form_point() -> InjectionPoint {
    InjectionPoint {
        location: ParameterLocation::FormData,
        name: "_form".to_string(),
        taint_origin: Some("user_input".to_string()),
    }
}

pub fn extract_by_patterns(
    body: &str,
    patterns: &[(&str, ParameterLocation)],
) -> Vec<InjectionPoint> {
    let mut points = Vec::new();
    for (pattern, location) in patterns {
        let re = regex::Regex::new(pattern).unwrap();
        for cap in re.captures_iter(body) {
            let name = cap[1].to_string();
            if !points
                .iter()
                .any(|p: &InjectionPoint| p.name == name && p.location == *location)
            {
                points.push(InjectionPoint {
                    location: *location,
                    name,
                    taint_origin: Some("user_input".to_string()),
                });
            }
        }
    }
    points
}
