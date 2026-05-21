// SPDX-License-Identifier: MIT
#![allow(clippy::type_complexity)]

use super::GenSenseAuditor;
use crate::{EMBEDDED_RULES_DIR, GenSenseRule, ProjectRule};
use std::collections::HashSet;
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

    fn collect_yml_files<'a>(
        dir: &'a include_dir::Dir<'a>,
        files: &mut Vec<include_dir::File<'a>>,
    ) {
        for entry in dir.entries() {
            match entry {
                include_dir::DirEntry::Dir(d) => Self::collect_yml_files(d, files),
                include_dir::DirEntry::File(f) => {
                    if f.path().extension().and_then(|s| s.to_str()) == Some("yml") {
                        files.push(f.clone());
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn default_rules() -> (Vec<Box<dyn GenSenseRule>>, Vec<Box<dyn ProjectRule>>) {
        let mut rules: Vec<Box<dyn GenSenseRule>> = Vec::new();
        let mut project_rules: Vec<Box<dyn ProjectRule>> = Vec::new();

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

            // AI Patterns
            rules.push(Box::new(
                crate::rules::global::ai_patterns::tautological_assert::TautologicalAssert,
            ));
            rules.push(Box::new(
                crate::rules::global::ai_patterns::placeholder_panic::PlaceholderPanic,
            ));
        }

        let yaml_rules_loaded = Self::load_yaml_rules_from_disk(&mut rules, &mut project_rules);

        // Load embedded YAML rules if local rules were not loaded
        if !yaml_rules_loaded {
            Self::load_embedded_yaml_rules(&mut rules, &mut project_rules);
        }

        (rules, project_rules)
    }

    fn load_yaml_rules_from_disk(
        rules: &mut Vec<Box<dyn GenSenseRule>>,
        project_rules: &mut Vec<Box<dyn ProjectRule>>,
    ) -> bool {
        let mut yaml_rules_loaded = false;
        let local_rules_path = Path::new("src/rules/definitions");
        if local_rules_path.exists() && local_rules_path.is_dir() {
            for e in WalkDir::new(local_rules_path).into_iter().flatten() {
                if e.path().extension().and_then(|s| s.to_str()) == Some("yml")
                    && let Ok(content) = std::fs::read_to_string(e.path())
                    && let Ok(wrapper) = serde_yaml::from_str::<RulesWrapper>(&content)
                {
                    wrapper.check_version();
                    for dsl_rule in wrapper.rules {
                        if let Ok(compiled) =
                            crate::rules::compiler::RuleCompiler::compile(dsl_rule)
                        {
                            rules.push(Box::new(compiled));
                        }
                    }
                    for p_rule in wrapper.project_rules {
                        if let Ok(compiled) =
                            crate::rules::compiler::ProjectRuleCompiler::compile(p_rule)
                        {
                            project_rules.push(Box::new(compiled));
                        }
                    }
                    yaml_rules_loaded = true;
                }
            }
        }
        yaml_rules_loaded
    }

    fn load_embedded_yaml_rules(
        rules: &mut Vec<Box<dyn GenSenseRule>>,
        project_rules: &mut Vec<Box<dyn ProjectRule>>,
    ) {
        let mut rule_files = Vec::new();
        Self::collect_yml_files(&EMBEDDED_RULES_DIR, &mut rule_files);
        for file in rule_files {
            if let Some(rules_yml) = file.contents_utf8() {
                match serde_yaml::from_str::<RulesWrapper>(rules_yml) {
                    Ok(wrapper) => {
                        wrapper.check_version();
                        for dsl_rule in wrapper.rules {
                            if let Ok(compiled) =
                                crate::rules::compiler::RuleCompiler::compile(dsl_rule)
                            {
                                rules.push(Box::new(compiled));
                            }
                        }
                        for p_rule in wrapper.project_rules {
                            if let Ok(compiled) =
                                crate::rules::compiler::ProjectRuleCompiler::compile(p_rule)
                            {
                                project_rules.push(Box::new(compiled));
                            }
                        }
                    }
                    _ => {
                        tracing::error!(
                            "[ERROR] Failed to parse YAML rules in {}: failure",
                            file.path().display()
                        );
                    }
                }
            }
        }
    }

    #[allow(clippy::type_complexity)]
    #[must_use]
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
