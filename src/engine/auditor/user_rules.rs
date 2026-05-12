// SPDX-License-Identifier: MIT
#![allow(clippy::type_complexity)]

use crate::{GenSenseRule, ProjectRule};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(serde::Deserialize)]
struct RulesWrapper {
    #[serde(default)]
    rules: Vec<crate::rules::core::CoreRule>,
    #[serde(default)]
    project_rules: Vec<crate::rules::core::project::ProjectCoreRule>,
}

#[allow(clippy::type_complexity)]
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
            eprintln!(
                "[WARNING] Custom rules directory does not exist or is not a directory: {}",
                extra_dir.display()
            );
        }
    }

    for dir in dirs_to_check {
        for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yml") {
                match std::fs::read_to_string(path) {
                    Ok(content) => match serde_yaml::from_str::<RulesWrapper>(&content) {
                        Ok(wrapper) => {
                            for rule in wrapper.rules {
                                let compiled = crate::rules::compiler::RuleCompiler::compile(rule);
                                rules.push(Box::new(compiled) as Box<dyn GenSenseRule>);
                            }
                            for p_rule in wrapper.project_rules {
                                let compiled =
                                    crate::rules::compiler::ProjectRuleCompiler::compile(p_rule);
                                project_rules.push(Box::new(compiled) as Box<dyn ProjectRule>);
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "[WARNING] Failed to parse YAML rules in {}: {}",
                                path.display(),
                                e
                            );
                        }
                    },
                    Err(e) => {
                        eprintln!(
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
