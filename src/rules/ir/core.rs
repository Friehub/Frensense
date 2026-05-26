// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata};
use regex::Regex;
use tree_sitter::Node;

use super::flow::FlowConstraint;

/// Tree-sitter query with capture.
#[derive(Debug, Clone)]
pub struct AstQuery {
    pub selector: String,
    #[allow(clippy::pub_underscore_fields)]
    pub _capture_name: String,
}

/// Compiled Internal Representation of a Core Rule.
#[derive(Debug, Clone)]
pub struct CoreRuleIr {
    pub metadata: RuleMetadata,
    pub match_queries: Vec<AstQuery>,
    pub flow_constraints: Vec<FlowConstraint>,
    pub if_matches: Option<Regex>,
    pub must_contain: Option<Regex>,
    pub must_not_contain: Option<Regex>,
    pub max_lines: Option<usize>,
    pub max_depth: Option<usize>,
    pub max_file_lines: Option<usize>,
    pub target_ext: String,
    pub target_kinds: Vec<String>,
    pub use_query: bool,
    pub fix_pattern: Option<Regex>,
    pub fix_template: Option<String>,
    pub inject_import: Option<String>,
    pub if_name_matches: Option<Regex>,
    pub body_must_contain: Option<Regex>,
    pub body_may_delegate_via: Option<Regex>,
    pub body_must_contain_any_of: Option<Regex>,
    pub must_be_preceded_by: Option<String>,
    pub auto_fixable: Option<bool>,
    pub requires_human: Option<bool>,
    pub exclude_scope: Option<Regex>,
    pub skip_if_parent: Option<String>,
    pub body_query: Option<String>,
    pub taint_max_depth: Option<usize>,
}

impl GenSenseRule for CoreRuleIr {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn applies_to(&self, ext: &str) -> bool {
        self.target_ext == "*" || self.target_ext.split('|').any(|e| e.trim() == ext)
    }

    fn query(&self) -> Option<&str> {
        self.match_queries.first().map(|q| q.selector.as_str())
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        if let Some(re) = &self.exclude_scope {
            let file_path = context.file_path.to_string_lossy();
            if re.is_match(&file_path) {
                return Vec::new();
            }
            let mut current = node.parent();
            while let Some(ancestor) = current {
                let text = &context.source_code[ancestor.start_byte()..ancestor.end_byte()];
                if re.is_match(text) {
                    return Vec::new();
                }
                current = ancestor.parent();
            }
        }

        {
            let mut cur = node;
            while let Some(parent) = cur.parent() {
                if parent.kind() == "mod_item" {
                    let mut prev = parent.prev_sibling();
                    while let Some(sib) = prev {
                        if sib.kind() == "attribute_item" {
                            let text = &context.source_code[sib.start_byte()..sib.end_byte()];
                            if text.contains("#[cfg(test)]") || text.contains("#[test]") {
                                return Vec::new();
                            }
                            break;
                        }
                        if sib.kind() != "line_comment" && sib.kind() != "block_comment" {
                            break;
                        }
                        prev = sib.prev_sibling();
                    }
                }
                cur = parent;
            }
        }

        if let Some(kind) = &self.skip_if_parent
            && let Some(parent) = node.parent()
            && parent.kind() == kind.as_str()
        {
            return Vec::new();
        }

        let mut top = node;
        while let Some(parent) = top.parent() {
            top = parent;
        }

        if !self.use_query && !self.target_kinds.is_empty() {
            let kind = node.kind();
            if !self.target_kinds.iter().any(|k| k == kind) {
                return Vec::new();
            }
        }

        if !self.check_regex_matching(node, context, &mut advisories) {
            return advisories;
        }

        if !self.check_structural_matching(node, context, &mut advisories) {
            return advisories;
        }

        self.check_content_constraints(node, context, &mut advisories);
        self.check_metric_constraints(node, context, &mut advisories);
        self.check_flow_constraints(node, context, top, &mut advisories);

        advisories
    }

    fn file_check(&self, context: &GenSenseContext<'_>) -> Vec<Advisory> {
        if let Some(max) = self.max_file_lines {
            let line_count = context.source_code.lines().count();
            if line_count > max {
                let meta = self.metadata();
                return vec![Advisory {
                    rule_id: meta.id.to_string(),
                    file_id: context.file_id,
                    file_path: context.file_path.to_string_lossy().to_string(),
                    severity: meta.severity,
                    confidence: meta.confidence,
                    observation: format!(
                        "File length ({line_count} lines) exceeds threshold of {max}."
                    ),
                    impact: meta.impact.to_string(),
                    improvement: meta.improvement.to_string(),
                    line: 1,
                    column: 1,
                    start_byte: 0,
                    end_byte: 0,
                    original_content: String::new(),
                    proposed_replacement: None,
                    proposed_import: None,
                    enclosing_symbol: None,
                    fingerprint: String::new(),
                    auto_fixable: false,
                    requires_human: false,
                    tags: meta.tags.iter().map(ToString::to_string).collect(),
                }];
            }
        }
        Vec::new()
    }
}
