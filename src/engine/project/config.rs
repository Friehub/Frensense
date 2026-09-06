// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;

#[derive(serde::Deserialize, Default, Clone)]
pub struct FrensenseConfig {
    pub rules_dir: Option<String>,
    pub disabled_rules: Option<Vec<String>>,
    pub severity_override: Option<HashMap<String, crate::Severity>>,
}

#[must_use]
pub fn load_config(root: &Path) -> FrensenseConfig {
    let config_path = root.join(".frensense").join("config.yml");
    if !config_path.exists() {
        return FrensenseConfig::default();
    }
    std::fs::read_to_string(&config_path)
        .ok()
        .and_then(|s| serde_yaml::from_str(&s).ok())
        .unwrap_or_default()
}
