// SPDX-License-Identifier: MIT

pub mod helpers;
pub mod project;

use crate::{
    semantics::data_flow::{DataFlowAnalyzer, TaintRegistry},
    Advisory, GenSenseContext, GenSenseRule, RuleMetadata,
};
use helpers::serde_regex_opt;
use regex::Regex;
use tree_sitter::Node;
// use std::borrow::Cow;

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
            } else {
                return None;
            }
        }

        // Default heuristic
        if self.on_node.contains("|") || !self.on_node.contains(" ") {
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

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let mut top = node;
        while let Some(parent) = top.parent() {
            top = parent;
        }

        let file_path = context.file_path.to_string_lossy().to_string();
        let function_line = context
            .symbols
            .find_function_at(&file_path, node.start_position().row + 1)
            .and_then(|idx| context.symbols.graph.get_symbol(idx))
            .map(|s| s.line)
            .unwrap_or(0);

        let cache_key = (
            self.id().to_string(),
            file_path.clone(),
            if function_line == 0 {
                node.start_position().row
            } else {
                function_line
            },
        );

        {
            let cache = context.taint_cache.borrow();
            if cache.contains_key(&cache_key) {
                return Vec::new();
            }
        }

        let code = &context.source_code[node.start_byte()..node.end_byte()];
        let src_re = self.source_pattern.as_ref();
        let sink_re = self.sink_pattern.as_ref();

        if let (Some(src_re), Some(sink_re)) = (src_re, sink_re) {
            let analyzer = DataFlowAnalyzer::new(context, top);
            let mut registry = TaintRegistry::default();
            analyzer.discover_symbols(&mut registry);

            let target_node = node.child_by_field_name("body").unwrap_or(node);
            advisories.extend(analyzer.analyze_block(target_node, src_re, sink_re, self, registry));

            let mut cache = context.taint_cache.borrow_mut();
            cache.insert(cache_key, advisories.clone());
            return advisories;
        }

        if let Some(re) = &self.if_matches {
            if !re.is_match(code) {
                return Vec::new();
            }

            if let (Some(fix_pat_str), Some(template)) = (&self.fix_pattern, &self.fix_with) {
                if let Ok(fix_re) = Regex::new(fix_pat_str) {
                    if let Some(_caps) = fix_re.captures(code) {
                        let replacement = fix_re.replace_all(code, template).to_string();
                        if replacement != code {
                            let import = self.inject_import.as_ref().map(|import_template| {
                                fix_re.replace_all(code, import_template).to_string()
                            });
                            advisories.push(self.new_remediated_advisory(
                                &node,
                                context,
                                self.metadata.observation.to_string(),
                                replacement,
                                import,
                            ));
                            return advisories;
                        }
                    }
                }
            }
        }

        // CSA: Name matching
        if let Some(re) = &self.if_name_matches {
            let name_node = node.child_by_field_name("name").or_else(|| {
                node.children(&mut node.walk())
                    .find(|c| c.kind().contains("identifier"))
            });

            if let Some(name_node) = name_node {
                let name = &context.source_code[name_node.start_byte()..name_node.end_byte()];
                if !re.is_match(name) {
                    return Vec::new();
                }
            } else {
                return Vec::new();
            }
        }

        // CSA: Body content matching
        if let Some(re) = &self.body_must_contain {
            let body_node = node.child_by_field_name("body").unwrap_or(node);
            let body_code = &context.source_code[body_node.start_byte()..body_node.end_byte()];
            if !re.is_match(body_code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!(
                        "Function body is missing required structural element for '{}'.",
                        re.as_str()
                    ),
                ));
            }
        }

        // Metrics
        if let Some(max) = self.max_lines {
            let lines = node.end_position().row - node.start_position().row + 1;
            if lines > max {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Size violation: {lines} > {max} lines."),
                ));
            }
        }

        if let Some(max) = self.max_depth {
            let depth = helpers::calculate_peak_depth(node);
            if depth > max {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Depth violation: {depth} > {max} levels."),
                ));
            }
        }

        // Patterns
        if let Some(re) = &self.must_contain {
            if !re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Missing expected pattern '{}'.", re.as_str()),
                ));
            }
        }

        if let Some(re) = &self.must_not_contain {
            if re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Prohibited pattern '{}' found.", re.as_str()),
                ));
            }
        }

        advisories
    }
}
