// SPDX-License-Identifier: MIT

use crate::RuleMetadata;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct MustHaveGuard {
    pub source_pattern: String,
    pub guard_pattern: String,
    pub source_file_glob: String,
    pub guard_file_glob: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct MustBeInternal {
    pub pattern: String,
    pub file_glob: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CrossFileTaintFree {
    pub source_pattern: String,
    pub sink_pattern: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct GlobalDataFlow {
    pub source_pattern: String,
    pub sink_pattern: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ProjectCoreRule {
    #[serde(flatten)]
    pub metadata: RuleMetadata,
    #[serde(default)]
    pub target_ext: String,
    pub must_have_guard: Option<MustHaveGuard>,
    pub must_be_internal: Option<MustBeInternal>,
    pub cross_file_taint_free: Option<CrossFileTaintFree>,
    pub global_data_flow: Option<GlobalDataFlow>,
    #[serde(default)]
    pub source_ext: Option<String>,
    #[serde(default)]
    pub source_pattern: Option<String>,
    #[serde(default)]
    pub source_file_glob: Option<String>,
    #[serde(default)]
    pub schema_type: Option<crate::rules::ir::SchemaType>,
    #[serde(default)]
    pub schema_glob: Option<String>,
    #[serde(default)]
    pub schema_extract: Option<crate::rules::ir::SchemaExtract>,
}
