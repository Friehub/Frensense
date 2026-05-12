// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TemporalAnalyzer<'a, 'ctx> {
    pub context: &'ctx GenSenseContext<'a>,
}

impl<'a, 'ctx> TemporalAnalyzer<'a, 'ctx> {
    pub fn new(context: &'ctx GenSenseContext<'a>) -> Self {
        Self { context }
    }

    pub fn check_temporal(
        &self,
        scope: Node,
        sequence: &[regex::Regex],
        behavior: &crate::rules::ir::TemporalBehavior,
        rule: &dyn GenSenseRule,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let file_path = self.context.file_path.to_string_lossy();
        let line = scope.start_position().row + 1;

        let scope_idx = self.context.symbols.find_function_at(&file_path, line);
        let scope_idx = match scope_idx {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        let events = self
            .context
            .symbols
            .graph
            .ordered_events_in_scope(scope_idx);
        let meta = rule.metadata();

        match behavior {
            crate::rules::ir::TemporalBehavior::MustNotFollow => {
                let mut found_first = false;
                for event in &events {
                    let mut matched_p = None;
                    for (i, re) in sequence.iter().enumerate() {
                        if re.is_match(&event.label) {
                            matched_p = Some(i);
                            break;
                        }
                    }

                    if let Some(p_idx) = matched_p {
                        if p_idx == 0 {
                            found_first = true;
                        } else if p_idx == 1 && found_first {
                            advisories.push(Advisory {
                                rule_id: meta.id.to_string(),
                                file_id: self.context.file_id,
                                file_path: self.context.file_path.to_string_lossy().to_string(),
                                severity: meta.severity,
                                observation: format!(
                                    "Temporal Violation: '{}' must NOT follow '{}' in this scope.",
                                    sequence[1].as_str(),
                                    sequence[0].as_str()
                                ),
                                impact: meta.impact.to_string(),
                                improvement: meta.improvement.to_string(),
                                line: event.line as u32,
                                column: event.column as u32,
                                start_byte: 0,
                                end_byte: 0,
                                original_content: String::new(),
                                proposed_replacement: None,
                            });
                        }
                    }

                    if event.event_type == crate::semantics::graph::EventType::Release {
                        found_first = false;
                    }
                }
            }
            crate::rules::ir::TemporalBehavior::MustFollow => {
                let mut current_step = 0;
                for event in &events {
                    if current_step < sequence.len()
                        && sequence[current_step].is_match(&event.label)
                    {
                        current_step += 1;
                    }
                }

                if current_step < sequence.len() {
                    advisories.push(rule.new_advisory(
                        &scope,
                        self.context,
                        format!(
                            "Temporal Violation: Expected sequence [{}] was incomplete.",
                            sequence
                                .iter()
                                .map(|re| re.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    ));
                }
            }
            crate::rules::ir::TemporalBehavior::ForbiddenBetween(start_re, end_re) => {
                let mut in_forbidden_zone = false;

                for event in &events {
                    if start_re.is_match(&event.label) {
                        in_forbidden_zone = true;
                    } else if end_re.is_match(&event.label) {
                        in_forbidden_zone = false;
                    } else if in_forbidden_zone {
                        for (i, re) in sequence.iter().enumerate() {
                            if re.is_match(&event.label) {
                                advisories.push(Advisory {
                                    rule_id: meta.id.to_string(),
                                    file_id: self.context.file_id,
                                    file_path: self.context.file_path.to_string_lossy().to_string(),
                                    severity: meta.severity,
                                    observation: format!(
                                        "Temporal Violation: '{}' found between '{}' and '{}', which is forbidden.",
                                        sequence[i].as_str(),
                                        start_re.as_str(),
                                        end_re.as_str()
                                    ),
                                    impact: meta.impact.to_string(),
                                    improvement: meta.improvement.to_string(),
                                    line: event.line as u32,
                                    column: event.column as u32,
                                    start_byte: 0,
                                    end_byte: 0,
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
