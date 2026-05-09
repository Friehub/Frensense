// SPDX-License-Identifier: MIT

use crate::{
    semantics::data_flow::{DataFlowAnalyzer, TaintRegistry},
    Advisory, GenSenseContext, GenSenseRule,
};
use regex::Regex;
use tree_sitter::Node;

/// Generic Declarative Rule: Configurable via YAML for a stress-free developer experience.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct CoreRule {
    pub id: String,
    pub domain: String,
    pub target_ext: String,
    pub observation: String, // Narrative: What we saw
    pub impact: String,      // Narrative: Why it matters
    pub improvement: String, // Narrative: How to refine it
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
    #[serde(default, with = "serde_regex_opt")]
    pub source_pattern: Option<Regex>,
    #[serde(default, with = "serde_regex_opt")]
    pub sink_pattern: Option<Regex>,
    pub severity: Option<crate::Severity>,
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl GenSenseRule for CoreRule {
    fn id(&self) -> &str {
        &self.id
    }
    fn description(&self) -> &str {
        &self.observation
    }
    fn category(&self) -> &str {
        self.category.as_deref().unwrap_or("General")
    }
    fn tags(&self) -> Vec<&str> {
        self.tags.iter().map(|s| s.as_str()).collect()
    }
    fn severity(&self) -> crate::Severity {
        self.severity.unwrap_or(crate::Severity::Warning)
    }
    fn impact(&self) -> &str {
        &self.impact
    }
    fn improvement(&self) -> &str {
        &self.improvement
    }

    fn query(&self) -> Option<&str> {
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
        if self.target_ext == "ts" && ext == "tsx" {
            return true;
        }
        if self.target_ext == "tsx" && ext == "ts" {
            return true;
        }
        self.target_ext == ext
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let ext = context
            .file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if !self.applies_to(ext) {
            return advisories;
        }

        let node_kind = node.kind();
        let target_kinds: Vec<String> = self
            .on_node
            .split('|')
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')' || c == ' ')
                    .split('@')
                    .next()
                    .unwrap_or(s)
                    .trim()
                    .to_string()
            })
            .collect();

        if target_kinds.iter().any(|k| k == node_kind) || self.on_node.contains(" ") {
            let code = &context.source_code[node.start_byte()..node.end_byte()];

            // --- Semantic Logic: Taint Tracking ---
            if let (Some(src_re), Some(sink_re)) = (&self.source_pattern, &self.sink_pattern) {
                let mut top = node;
                while let Some(parent) = top.parent() {
                    top = parent;
                }

                let analyzer = DataFlowAnalyzer::new(context, top);
                if node_kind == "function_declaration"
                    || node_kind == "function_item"
                    || node_kind == "arrow_function"
                    || node_kind == "method_definition"
                {
                    if let Some(body) = node.child_by_field_name("body") {
                        advisories.extend(analyzer.analyze_block(
                            body,
                            src_re,
                            sink_re,
                            self,
                            TaintRegistry::default(),
                        ));
                    }
                } else if node_kind == "lexical_declaration" || node_kind == "variable_declaration"
                {
                    advisories.extend(analyzer.analyze_block(
                        node,
                        src_re,
                        sink_re,
                        self,
                        TaintRegistry::default(),
                    ));
                }
                return advisories;
            }

            let matches_if = self
                .if_matches
                .as_ref()
                .map_or(true, |re| re.is_match(code));

            // 1. Metric-based violations
            if matches_if {
                let start_pos = node.start_position();
                let end_pos = node.end_position();
                let node_lines = end_pos.row - start_pos.row + 1;

                if let Some(max) = self.max_lines {
                    if node_lines > max {
                        advisories.push(self.new_advisory(
                            &node,
                            format!("Function size ({node_lines} lines) exceeds the target threshold of {max} lines."),
                            "Large functions can be more challenging to maintain and formally verify due to increased state space.".to_string(),
                            "Consider decomposing this function into smaller, specialized helper units or traits.".to_string(),
                        ));
                    }
                }

                if let Some(max) = self.max_depth {
                    let peak = self.calculate_peak_depth(node);
                    if peak > max {
                        advisories.push(self.new_advisory(
                            &node,
                            format!("Deep Nesting: logical depth ({peak}) exceeds standardized limit ({max} levels)."),
                            "Extreme indentation indicates complex, unmaintainable logic. Such paths are high risk for bugs.".to_string(),
                            "Extract inner logic or match-legs into helper functions.".to_string(),
                        ));
                    }
                }
            }

            // 2. Pattern-based violations (Conjunction Logic)
            // A violation only occurs if we have a reason to trigger (if_matches is true OR we have specific constraints)
            // AND the metrics were NOT the only thing this rule was about.
            if self.max_lines.is_none() && self.max_depth.is_none() {
                let matches_must_not = self
                    .must_not_contain
                    .as_ref()
                    .map_or(true, |re| re.is_match(code));

                let matches_must = if let Some(must_re) = &self.must_contain {
                    let mut found = must_re.is_match(code);
                    if !found {
                        let start_row = node.start_position().row;
                        let lines: Vec<&str> = context.source_code.lines().collect();
                        let search_start = start_row.saturating_sub(2);
                        for i in search_start..start_row {
                            if let Some(line) = lines.get(i) {
                                if must_re.is_match(line) {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    !found // must_contain violation if pattern NOT found
                } else {
                    false // Don't trigger by default if no must_contain specified
                };

                // Trigger if (if_matches matched) AND (must_not_contain matched OR must_contain was violated)
                // BUT only if this isn't JUST a metric rule.
                let mut should_trigger = false;
                if self.if_matches.is_some() && matches_if {
                    if self.must_not_contain.is_some() && matches_must_not {
                        should_trigger = true;
                    } else if self.must_contain.is_some() && matches_must {
                        should_trigger = true;
                    } else if self.must_not_contain.is_none() && self.must_contain.is_none() {
                        should_trigger = true;
                    }
                }

                if should_trigger {
                    let scope_matched = if let Some(scope_pattern) = &self.within_scope {
                        self.check_parent_scope(node, scope_pattern, context.source_code)
                    } else {
                        true
                    };

                    if scope_matched {
                        let mut adv = self.new_advisory(
                            &node,
                            self.observation.clone(),
                            self.impact.clone(),
                            self.improvement.clone(),
                        );
                        adv.original_content = code.to_string();
                        if let Some(fix) = &self.fix_with {
                            adv.proposed_replacement = Some(fix.clone());
                        }
                        advisories.push(adv);
                    }
                }
            }
        }

        advisories
    }
}

impl CoreRule {
    fn calculate_peak_depth(&self, node: Node) -> usize {
        let mut max_child_depth = 0;
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let d = self.calculate_peak_depth(child);
            if d > max_child_depth {
                max_child_depth = d;
            }
        }

        let kind = node.kind();
        // Nodes that increase logical depth
        let increases_depth = match kind {
            "if_statement" | "while_statement" | "for_statement" | "match_expression"
            | "if_expression" | "for_expression" | "while_expression" | "do_statement"
            | "try_statement" | "catch_clause" | "finally_clause" | "switch_statement"
            | "case_clause" => true,
            _ => false,
        };

        if increases_depth {
            max_child_depth + 1
        } else {
            max_child_depth
        }
    }

    fn check_parent_scope(&self, node: Node, scope_pattern: &str, source: &str) -> bool {
        let mut current = node;
        let scopes: Vec<&str> = scope_pattern.split('|').collect();

        while let Some(parent) = current.parent() {
            let kind = parent.kind();
            for scope in &scopes {
                if *scope == "async_fn" && kind == "function_item" {
                    let header = &source[parent.start_byte()
                        ..parent
                            .child_by_field_name("body")
                            .map_or(parent.end_byte(), |b| b.start_byte())];
                    if header.contains("async") {
                        return true;
                    }
                } else if kind == *scope {
                    return true;
                }
            }
            current = parent;
        }
        false
    }
}

mod serde_regex_opt {
    use regex::Regex;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(re_str) => Regex::new(&re_str)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
