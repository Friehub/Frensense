// SPDX-License-Identifier: MIT

use std::path::Path;

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Environment {
    #[default]
    Unknown,
    Test,
    Mock,
    RouteHandler,
    Utility,
    Config,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataSensitivity {
    #[default]
    Unknown,
    Low,
    Medium,
    High,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileContext {
    pub environment: Environment,
    pub sensitivity: DataSensitivity,
    pub frameworks: Vec<String>,
}

impl FileContext {
    #[must_use]
    pub fn extract(file_path: &Path, content: &str) -> Self {
        let path_str = file_path.to_string_lossy().to_lowercase();

        let mut env = Environment::Unknown;
        let c = content.to_lowercase();

        if path_str.contains("test")
            || path_str.contains("spec")
            || path_str.contains("cypress")
            || path_str.contains("__tests__")
            || c.contains("describe(")
            || c.contains(" it(")
            || c.contains("\nit(")
        {
            env = Environment::Test;
        } else if path_str.contains("mock") || path_str.contains("stub") {
            env = Environment::Mock;
        } else if path_str.contains("config") || path_str.contains("settings") {
            env = Environment::Config;
        } else if path_str.contains("route")
            || path_str.contains("controller")
            || path_str.contains("handler")
            || path_str.contains("endpoint")
            || path_str.contains("api/")
            || c.contains("(req, res)")
            || c.contains("(req, res,")
            || c.contains("(req, res ")
            || c.contains("req: request")
            || c.contains("request, response")
            || c.contains("app.get(")
            || c.contains("app.post(")
            || c.contains("app.put(")
            || c.contains("app.delete(")
            || c.contains("app.patch(")
            || c.contains("router.get(")
            || c.contains("router.post(")
            || c.contains("router.put(")
            || c.contains("router.delete(")
            || c.contains("router.patch(")
            || c.contains("res.send")
            || c.contains("res.json")
            || c.contains("res.redirect")
            || c.contains("res.render")
            || c.contains("res.status")
            || c.contains("c.req")
            || c.contains("router.")
        {
            env = Environment::RouteHandler;
        } else if path_str.contains("lib")
            || path_str.contains("util")
            || path_str.contains("helper")
        {
            env = Environment::Utility;
        }

        let mut sensitivity = DataSensitivity::Unknown;
        if c.contains("password")
            || c.contains("secret")
            || c.contains("jwt")
            || c.contains("token")
            || c.contains("creditcard")
        {
            sensitivity = DataSensitivity::High;
        } else if c.contains("email") || c.contains("user") || c.contains("profile") {
            sensitivity = DataSensitivity::Medium;
        } else if c.contains("version") || c.contains("metric") || c.contains("telemetry") {
            sensitivity = DataSensitivity::Low;
        }

        let mut frameworks = Vec::new();
        if c.contains("express") || c.contains("req") && c.contains("res") {
            frameworks.push("Express".to_string());
        }
        if c.contains("react") || c.contains("jsx") {
            frameworks.push("React".to_string());
        }

        Self {
            environment: env,
            sensitivity,
            frameworks,
        }
    }
}
