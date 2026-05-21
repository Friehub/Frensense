// SPDX-License-Identifier: MIT
#![allow(clippy::type_complexity)]

use crate::{GenSenseRule, ProjectRule};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(serde::Deserialize)]
struct RulesWrapper {
    #[serde(default)]
    rules: Vec<crate::rules::core::CoreRule>,
    #[serde(default, alias = "schema_contracts")]
    project_rules: Vec<crate::rules::core::project::ProjectCoreRule>,
    /// Optional YAML format version. If absent, assumes latest (0.3.0).
    /// Supported: "0.3.0"
    #[serde(default)]
    version: Option<String>,
}

impl RulesWrapper {
    fn check_version(&self) {
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

#[allow(clippy::type_complexity)]
#[must_use]
pub fn load_user_rules(
    project_root: &Path,
    extra_dirs: &[PathBuf],
) -> (Vec<Box<dyn GenSenseRule>>, Vec<Box<dyn ProjectRule>>) {
    let mut rules = Vec::new();
    let mut project_rules = Vec::new();
    let mut dirs_to_check = Vec::new();

    // 1. Project-local rules: <project_root>/.gensense/rules/
    let local_rules_dir = project_root.join(".gensense").join("rules");
    if local_rules_dir.exists() && local_rules_dir.is_dir() {
        dirs_to_check.push(local_rules_dir);
    }

    // 2. Global user rules: ~/.gensense/rules/
    if let Some(home_dir) = dirs::home_dir() {
        let global_rules_dir = home_dir.join(".gensense").join("rules");
        if global_rules_dir.exists() && global_rules_dir.is_dir() {
            dirs_to_check.push(global_rules_dir);
        }
    }

    // 3. Extra directories specified via CLI (--rules-dir)
    for extra_dir in extra_dirs {
        if extra_dir.exists() && extra_dir.is_dir() {
            dirs_to_check.push(extra_dir.clone());
        } else {
            tracing::warn!(
                "[WARNING] Custom rules directory does not exist or is not a directory: {}",
                extra_dir.display()
            );
        }
    }

    for dir in dirs_to_check {
        for entry in WalkDir::new(&dir)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yml") {
                match std::fs::read_to_string(path) {
                    Ok(content) => match serde_yaml::from_str::<RulesWrapper>(&content) {
                        Ok(wrapper) => {
                            wrapper.check_version();
                            for rule in wrapper.rules {
                                match crate::rules::compiler::RuleCompiler::compile(rule) {
                                    Ok(compiled) => {
                                        rules.push(Box::new(compiled) as Box<dyn GenSenseRule>);
                                    }
                                    Err(e) => {
                                        tracing::warn!(
                                            "[WARNING] Failed to compile rule in {}: {}",
                                            path.display(),
                                            e
                                        );
                                    }
                                }
                            }
                            for p_rule in wrapper.project_rules {
                                match crate::rules::compiler::ProjectRuleCompiler::compile(p_rule) {
                                    Ok(compiled) => project_rules
                                        .push(Box::new(compiled) as Box<dyn ProjectRule>),
                                    Err(e) => {
                                        tracing::warn!(
                                            "[WARNING] Failed to compile project rule in {}: {}",
                                            path.display(),
                                            e
                                        );
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "[WARNING] Failed to parse YAML rules in {}: {}",
                                path.display(),
                                e
                            );
                        }
                    },
                    Err(e) => {
                        tracing::warn!(
                            "[WARNING] Failed to read rule file {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }
    }

    (rules, project_rules)
}
