// SPDX-License-Identifier: MIT

pub mod helpers;
pub mod project;

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata};
use helpers::serde_regex_opt;
use regex::Regex;
use tree_sitter::Node;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct TemporalConfig {
    pub sequence: Vec<String>,
    pub behavior: String,
}

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
    pub within_scope: Option<String>,
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
}

impl GenSenseRule for CoreRule {
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

    fn check<'a>(&self, _node: Node<'a>, _context: &GenSenseContext<'a>) -> Vec<Advisory> {
        // CoreRule itself is just a template; it's compiled into CoreRuleIr for execution.
        Vec::new()
    }
}
