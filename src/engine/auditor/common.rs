// SPDX-License-Identifier: MIT

#[derive(serde::Deserialize)]
pub(crate) struct RulesWrapper {
    #[serde(default)]
    pub(crate) rules: Vec<crate::rules::core::CoreRule>,
    #[serde(default, alias = "schema_contracts")]
    pub(crate) project_rules: Vec<crate::rules::core::project::ProjectCoreRule>,
    /// Optional YAML format version. If absent, assumes latest (0.3.0).
    /// Supported: "0.3.0"
    #[serde(default)]
    pub(crate) version: Option<String>,
}

impl RulesWrapper {
    pub(crate) fn check_version(&self) {
        if let Some(ref ver) = self.version
            && ver != "0.3.0"
        {
            tracing::warn!(
                "[WARNING] Unknown rules format version '{}'. Assuming 0.3.0 compatibility. Supported versions: 0.3.0",
                ver
            );
        }
    }
}
