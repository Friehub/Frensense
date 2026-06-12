// SPDX-License-Identifier: MIT

use super::Engine;
use crate::FrensenseEnvironment;
use std::path::PathBuf;

impl Engine {
    #[cfg(feature = "fingerprinting")]
    #[must_use]
    pub fn with_profile(mut self, profile: crate::engine::profile::ProjectProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    #[cfg(feature = "fingerprinting")]
    pub const fn set_profile_threshold(&mut self, threshold: f64) {
        self.profile_threshold = threshold;
    }

    #[cfg(feature = "fingerprinting")]
    #[must_use]
    pub fn profile(&self) -> Option<&crate::engine::profile::ProjectProfile> {
        self.profile.as_ref()
    }

    pub const fn set_jaccard_threshold(&mut self, val: f64) {
        self.jaccard_threshold = val;
    }

    pub const fn set_confidence_boost_rate(&mut self, val: f32) {
        self.confidence_boost_rate = val;
    }

    pub const fn set_confidence_boost_max(&mut self, val: f32) {
        self.confidence_boost_max = val;
    }

    pub const fn set_max_source_lines(&mut self, val: usize) {
        self.max_source_lines = Some(val);
    }

    pub const fn set_ngram_window_size(&mut self, val: usize) {
        self.ngram_window_size = val;
    }

    pub const fn set_min_ngram_count(&mut self, val: usize) {
        self.min_ngram_count = val;
    }

    pub const fn set_taint_confidence_interprocedural(&mut self, val: f32) {
        self.taint_confidence_interprocedural = val;
    }

    pub const fn set_taint_confidence_intraprocedural(&mut self, val: f32) {
        self.taint_confidence_intraprocedural = val;
    }

    pub const fn set_default_taint_max_depth(&mut self, val: usize) {
        self.default_taint_max_depth = val;
    }

    pub fn add_disabled_rule(&mut self, rule_id: &str) {
        self.disabled_rule_ids.push(rule_id.to_string());
    }

    pub fn add_severity_override(&mut self, rule_id: &str, severity: crate::Severity) {
        self.severity_overrides
            .insert(rule_id.to_string(), severity);
    }

    pub const fn set_min_confidence(&mut self, val: f32) {
        self.min_confidence = val;
    }

    pub const fn set_environment(&mut self, env: FrensenseEnvironment) {
        self.environment = env;
    }

    pub fn set_rules(&mut self, rules: Vec<Box<dyn crate::FrensenseRule>>) {
        self.auditor.set_rules(rules);
    }

    #[must_use]
    pub const fn auditor(&self) -> &crate::FrensenseAuditor {
        &self.auditor
    }

    #[must_use]
    pub fn project_rules(&self) -> &[Box<dyn crate::ProjectRule>] {
        &self.project_rules
    }

    #[must_use]
    pub fn list_rules(&self) -> Vec<(String, String, String)> {
        let mut rules = Vec::new();
        for r in self.auditor.rules() {
            let meta = r.metadata();
            rules.push((
                meta.id.to_string(),
                meta.name.to_string(),
                format!("{:?}", meta.severity),
            ));
        }
        for r in &self.project_rules {
            let meta = r.metadata();
            rules.push((
                meta.id.to_string(),
                meta.name.to_string(),
                format!("{:?}", meta.severity),
            ));
        }
        rules
    }

    pub fn enable_tag(&mut self, tag: &str) {
        self.enabled_tags.insert(tag.to_string());
    }

    pub fn enable_category(&mut self, category: &str) {
        self.enabled_categories.insert(category.to_string());
    }

    pub fn add_rule_dir<P: Into<PathBuf>>(&mut self, path: P) {
        self.extra_rule_dirs.push(path.into());
    }

    pub const fn set_no_builtin_rules(&mut self, val: bool) {
        self.no_builtin_rules = val;
    }

    pub const fn set_isolate_rules(&mut self, val: bool) {
        self.isolate_rules = val;
    }

    pub const fn set_severity_filter(&mut self, severity: Option<crate::Severity>) {
        self.severity_filter = severity;
    }

    pub fn set_language_filter(&mut self, extensions: &[&'static str]) {
        self.language_filter = Some(extensions.to_vec());
    }

    pub fn set_suite(&mut self, suite: crate::Suite) {
        self.suite = suite;
    }
}
