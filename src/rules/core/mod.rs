// SPDX-License-Identifier: MIT

pub mod helpers;
pub mod project;

#[cfg(feature = "temporal")]
use crate::temporal::TemporalConfig;
use crate::{Advisory, FrensenseContext, FrensenseRule, RuleMetadata};
use helpers::serde_regex_opt;
use regex::Regex;
use tree_sitter::Node;

/// Generic Declarative Rule: Configurable via YAML.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct CoreRule {
    #[serde(flatten)]
    pub metadata: RuleMetadata,
    pub target_ext: String,
    pub on_node: String,
    #[serde(default, with = "serde_regex_opt")]
    pub if_matches: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub must_contain: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub must_not_contain: Option<Regex>,
    #[serde(default)]
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub max_depth: Option<usize>,
    #[serde(default)]
    pub max_file_lines: Option<usize>,
    #[serde(default)]
    pub within_scope: Option<String>,
    #[serde(default)]
    pub outside_scope: Option<String>,
    #[serde(default)]
    pub fix_with: Option<String>,
    #[serde(default)]
    pub fix_pattern: Option<String>,
    #[serde(default)]
    pub inject_import: Option<String>,
    #[serde(default, with = "serde_regex_opt")]
    pub source_pattern: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub sink_pattern: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub forbidden_source_pattern: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub forbidden_sink_pattern: Option<Regex>,
    #[cfg(feature = "temporal")]
    #[serde(default)]
    pub temporal: Option<TemporalConfig>,
    #[serde(default, with = "serde_regex_opt")]
    pub if_name_matches: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub body_must_contain: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub body_may_delegate_via: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub body_must_contain_any_of: Option<Regex>,
    #[serde(default)]
    pub must_be_preceded_by: Option<String>,
    #[serde(default)]
    pub use_query: Option<bool>,
    #[serde(default)]
    pub auto_fixable: Option<bool>,
    #[serde(default)]
    pub requires_human: Option<bool>,
    #[serde(default)]
    pub exclude_scope: Option<String>,
    #[serde(default)]
    pub skip_if_parent: Option<String>,
    #[serde(default)]
    pub body_query: Option<String>,
    #[serde(default)]
    pub taint_max_depth: Option<usize>,
    /// Composite constraint: shorthand for "taint forbidden path that must cross a boundary".
    /// Syntax: `across_boundary: "pattern"` combined with `forbidden_source_pattern`/`forbidden_sink_pattern`.
    #[serde(default)]
    pub across_boundary: Option<String>,
    /// Composite constraint: `all_of` wraps multiple sub-constraints that must all fire.
    /// Each element is itself a complete `CoreRule` (only constraint fields used).
    #[serde(default)]
    pub all_of: Option<Vec<CoreRule>>,
    /// Composite constraint: `any_of` fires if at least one sub-constraint fires.
    #[serde(default)]
    pub any_of: Option<Vec<CoreRule>>,
    /// Composite constraint: not fires when the sub-constraint does NOT fire.
    #[serde(default)]
    pub not: Option<Box<CoreRule>>,
    /// Composite constraint: without fires when `constraint` fires but `without` does not.
    #[serde(default)]
    pub without_constraint: Option<Box<CoreRule>>,
    #[serde(default)]
    pub without_exclusion: Option<Box<CoreRule>>,
}

impl FrensenseRule for CoreRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn query(&self) -> Option<&str> {
        if let Some(explicit) = self.use_query {
            if explicit {
                return Some(&self.on_node);
            }
            return None;
        }

        // Default heuristic
        if self.on_node.contains('|') || !self.on_node.contains(' ') {
            None
        } else {
            Some(&self.on_node)
        }
    }

    fn applies_to(&self, ext: &str) -> bool {
        if self.target_ext == "*" {
            return true;
        }
        self.target_ext == ext
    }

    fn check<'a>(&self, _node: Node<'a>, _context: &FrensenseContext<'a>) -> Vec<Advisory> {
        // CoreRule itself is just a template; it's compiled into CoreRuleIr for execution.
        Vec::new()
    }
}
