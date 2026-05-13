// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;

#[derive(serde::Deserialize, Default, Clone)]
pub struct GenSenseConfig {
    pub rules_dir: Option<String>,
    pub disabled_rules: Option<Vec<String>>,
    pub severity_override: Option<HashMap<String, crate::Severity>>,
}

pub fn load_config(root: &Path) -> GenSenseConfig {
    let config_path = root.join(".gensense").join("config.yml");
    if !config_path.exists() {
        return GenSenseConfig::default();
    }
    std::fs::read_to_string(&config_path)
        .ok()
        .and_then(|s| serde_yaml::from_str(&s).ok())
        .unwrap_or_default()
}
