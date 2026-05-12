// SPDX-License-Identifier: MIT
#![allow(clippy::type_complexity)]

use super::GenSenseAuditor;
use crate::{GenSenseRule, ProjectRule, EMBEDDED_RULES_DIR};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(serde::Deserialize)]
struct RulesWrapper {
    #[serde(default)]
    rules: Vec<crate::rules::core::CoreRule>,
    #[serde(default)]
    project_rules: Vec<crate::rules::core::project::ProjectCoreRule>,
}

impl GenSenseAuditor {
    pub fn is_rule_enabled(
        &self,
        rule: &dyn GenSenseRule,
        cat_filter: &HashSet<String>,
        tag_filter: &HashSet<String>,
        env: crate::GenSenseEnvironment,
    ) -> bool {
        let meta = rule.metadata();

        if env == crate::GenSenseEnvironment::Production && meta.tags.iter().any(|t| t == "beta") {
            return false;
        }

        if cat_filter.is_empty() && tag_filter.is_empty() {
            return true;
        }

        if !cat_filter.is_empty() && cat_filter.contains(meta.category.as_ref()) {
            return true;
        }

        if !tag_filter.is_empty() {
            for tag in &meta.tags {
                if tag_filter.contains(tag.as_ref()) {
                    return true;
                }
            }
        }

        false
    }

    pub fn default_rules() -> (Vec<Box<dyn GenSenseRule>>, Vec<Box<dyn ProjectRule>>) {
        let mut rules: Vec<Box<dyn GenSenseRule>> = Vec::new();
        let mut project_rules: Vec<Box<dyn ProjectRule>> = Vec::new();

        // Track whether YAML rules were successfully loaded
        let mut yaml_rules_loaded = false;

        #[cfg(feature = "rust")]
        {
            rules.push(Box::new(crate::rules::rust::deadlock_guard::DeadlockGuard));
            rules.push(Box::new(
                crate::rules::rust::blocking_io::BlockingIoDetector,
            ));
            rules.push(Box::new(crate::rules::rust::async_safety::AsyncPanicSafety));
            rules.push(Box::new(crate::rules::rust::timeout_guard::TimeoutGuard));
            rules.push(Box::new(crate::rules::rust::tracing_guard::TracingGuard));
            rules.push(Box::new(crate::rules::rust::fake_async::FakeAsyncDetector));
        }

        fn collect_yml_files<'a>(
            dir: &'a include_dir::Dir<'a>,
            files: &mut Vec<include_dir::File<'a>>,
        ) {
            for entry in dir.entries() {
                match entry {
                    include_dir::DirEntry::Dir(d) => collect_yml_files(d, files),
                    include_dir::DirEntry::File(f) => {
                        if f.path().extension().and_then(|s| s.to_str()) == Some("yml") {
                            files.push(f.clone());
                        }
                    }
                }
            }
        }

        let mut rule_files = Vec::new();
        let local_rules_path = Path::new("src/rules/definitions");
        if local_rules_path.exists() && local_rules_path.is_dir() {
            for e in WalkDir::new(local_rules_path).into_iter().flatten() {
                if e.path().extension().and_then(|s| s.to_str()) == Some("yml") {
                    if let Ok(content) = std::fs::read_to_string(e.path()) {
                        match serde_yaml::from_str::<RulesWrapper>(&content) {
                            Ok(wrapper) => {
                                for dsl_rule in wrapper.rules {
                                    match crate::rules::compiler::RuleCompiler::compile(dsl_rule) {
                                        Ok(compiled) => rules.push(Box::new(compiled)),
                                        Err(e) => {
                                            eprintln!("[ERROR] Failed to compile rule: {e}");
                                        }
                                    }
                                }
                                for p_rule in wrapper.project_rules {
                                    match crate::rules::compiler::ProjectRuleCompiler::compile(
                                        p_rule,
                                    ) {
                                        Ok(compiled) => project_rules.push(Box::new(compiled)),
                                        Err(e) => {
                                            eprintln!(
                                                "[ERROR] Failed to compile project rule: {e}"
                                            );
                                        }
                                    }
                                }
                                yaml_rules_loaded = true;
                            }
                            Err(err) => {
                                eprintln!(
                                    "[ERROR] Dev Mode: Failed to parse {}: {}",
                                    e.path().display(),
                                    err
                                );
                            }
                        }
                    }
                }
            }
        }

        // Load embedded YAML rules if local rules were not loaded
        if !yaml_rules_loaded {
            collect_yml_files(&EMBEDDED_RULES_DIR, &mut rule_files);
            for file in rule_files {
                if let Some(rules_yml) = file.contents_utf8() {
                    match serde_yaml::from_str::<RulesWrapper>(rules_yml) {
                        Ok(wrapper) => {
                            for dsl_rule in wrapper.rules {
                                match crate::rules::compiler::RuleCompiler::compile(dsl_rule) {
                                    Ok(compiled) => rules.push(Box::new(compiled)),
                                    Err(e) => {
                                        eprintln!("[ERROR] Failed to compile rule: {e}");
                                    }
                                }
                            }
                            for p_rule in wrapper.project_rules {
                                match crate::rules::compiler::ProjectRuleCompiler::compile(p_rule) {
                                    Ok(compiled) => project_rules.push(Box::new(compiled)),
                                    Err(e) => {
                                        eprintln!("[ERROR] Failed to compile project rule: {e}");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "[ERROR] Failed to parse YAML rules in {:?}: {}",
                                file.path(),
                                e
                            );
                        }
                    }
                }
            }
        }

        (rules, project_rules)
    }

    #[allow(clippy::type_complexity)]
    pub fn build_rule_set(
        project_root: &Path,
        extra_dirs: &[PathBuf],
        no_builtin_rules: bool,
    ) -> (Vec<Box<dyn GenSenseRule>>, Vec<Box<dyn ProjectRule>>) {
        let (mut rules, mut project_rules) = if no_builtin_rules {
            (Vec::new(), Vec::new())
        } else {
            Self::default_rules()
        };

        let (user_rules, user_project_rules) =
            super::user_rules::load_user_rules(project_root, extra_dirs);

        let user_ids: HashSet<&str> = user_rules.iter().map(|r| r.id()).collect();
        rules.retain(|r| !user_ids.contains(r.id()));
        rules.extend(user_rules);

        let user_project_ids: HashSet<&str> = user_project_rules.iter().map(|r| r.id()).collect();
        project_rules.retain(|r| !user_project_ids.contains(r.id()));
        project_rules.extend(user_project_rules);

        (rules, project_rules)
    }
}
