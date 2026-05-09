// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

pub struct TemporalAnalyzer<'a> {
    pub context: &'a GenSenseContext<'a>,
}

impl<'a> TemporalAnalyzer<'a> {
    pub fn new(context: &'a GenSenseContext<'a>) -> Self {
        Self { context }
    }

    pub fn check_temporal(
        &self,
        scope: Node,
        sequence: &[String],
        behavior: &crate::rules::ir::TemporalBehavior,
        rule: &dyn GenSenseRule,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let regexes: Vec<Regex> = sequence.iter().map(|p| Regex::new(p).unwrap()).collect();

        // 1. Find the function in the graph
        let file_path = self.context.file_path.to_string_lossy();
        let line = scope.start_position().row + 1;

        // Find enclosing function index
        let scope_idx = if let Some(idx) = self.context.symbols.find_function_at(&file_path, line) {
            Some(idx)
        } else {
            // If the node itself isn't the function, look for enclosing one
            let mut parent = scope.parent();
            let mut found_idx = None;
            while let Some(p) = parent {
                if matches!(
                    p.kind(),
                    "function_item" | "function_declaration" | "method_definition"
                ) {
                    let p_line = p.start_position().row + 1;
                    if let Some(idx) = self.context.symbols.find_function_at(&file_path, p_line) {
                        found_idx = Some(idx);
                        break;
                    }
                }
                parent = p.parent();
            }
            found_idx
        };

        let scope_idx = match scope_idx {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        // 2. Get ordered events from the graph
        let events = self
            .context
            .symbols
            .graph
            .ordered_events_in_scope(scope_idx);

        // 3. Find matches for each step in the sequence
        let mut matches = Vec::new();
        for event in &events {
            for (i, re) in regexes.iter().enumerate() {
                if re.is_match(&event.label) {
                    matches.push((event.clone(), i));
                }
            }
        }

        match behavior {
            crate::rules::ir::TemporalBehavior::MustNotFollow => {
                let mut found_first = false;
                for (event, p_idx) in matches {
                    if p_idx == 0 {
                        found_first = true;
                    } else if p_idx == 1 && found_first {
                        advisories.push(Advisory {
                            rule_id: rule.id().to_string(),
                            severity: rule.severity(),
                            observation: format!(
                                "Temporal Violation: '{}' must NOT follow '{}' in this scope.",
                                sequence[1], sequence[0]
                            ),
                            impact: rule.impact().to_string(),
                            improvement: rule.improvement().to_string(),
                            line: event.line,
                            column: event.column,
                            file_path: event.file_path.clone(),
                            original_content: String::new(),
                            proposed_replacement: None,
                        });
                    }
                }
            }
            crate::rules::ir::TemporalBehavior::MustFollow => {
                let mut current_step = 0;
                for (_event, p_idx) in &matches {
                    if *p_idx == current_step {
                        current_step += 1;
                        if current_step == sequence.len() {
                            break;
                        }
                    }
                }

                if current_step < sequence.len() {
                    advisories.push(Advisory {
                        rule_id: rule.id().to_string(),
                        severity: rule.severity(),
                        observation: format!(
                            "Temporal Violation: Expected sequence [{}] was incomplete (stopped at '{}').",
                            sequence.join(", "),
                            sequence[current_step.saturating_sub(1)]
                        ),
                        impact: rule.impact().to_string(),
                        improvement: rule.improvement().to_string(),
                        line: scope.start_position().row + 1,
                        column: scope.start_position().column + 1,
                        file_path: file_path.to_string(),
                        original_content: String::new(),
                        proposed_replacement: None,
                    });
                }
            }
            crate::rules::ir::TemporalBehavior::ForbiddenBetween(start_p, end_p) => {
                let start_re = Regex::new(start_p).unwrap();
                let end_re = Regex::new(end_p).unwrap();
                let mut in_forbidden_zone = false;

                for event in &events {
                    if start_re.is_match(&event.label) {
                        in_forbidden_zone = true;
                    } else if end_re.is_match(&event.label) {
                        in_forbidden_zone = false;
                    } else if in_forbidden_zone {
                        for (i, re) in regexes.iter().enumerate() {
                            if re.is_match(&event.label) {
                                advisories.push(Advisory {
                                    rule_id: rule.id().to_string(),
                                    severity: rule.severity(),
                                    observation: format!(
                                        "Temporal Violation: '{}' found between '{}' and '{}', which is forbidden.",
                                        sequence[i], start_p, end_p
                                    ),
                                    impact: rule.impact().to_string(),
                                    improvement: rule.improvement().to_string(),
                                    line: event.line,
                                    column: event.column,
                                    file_path: event.file_path.clone(),
                                    original_content: String::new(),
                                    proposed_replacement: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        advisories
    }
}
