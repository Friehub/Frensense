// SPDX-License-Identifier: MIT

use crate::{Advisory, FrensenseContext, FrensenseRule, RuleMetadata};
use tree_sitter::Node;

/// Behavior for temporal ordering constraints.
#[derive(Debug, Clone)]
pub enum TemporalBehavior {
    /// First event must be followed by second event.
    MustFollow,
    /// First event must NOT be followed by second event.
    MustNotFollow,
    /// Events are forbidden between start and end patterns.
    ForbiddenBetween(regex::Regex, regex::Regex),
}

pub struct TemporalAnalyzer<'a, 'ctx> {
    pub context: &'ctx FrensenseContext<'a>,
}

impl<'a, 'ctx> TemporalAnalyzer<'a, 'ctx> {
    #[must_use]
    pub const fn new(context: &'ctx FrensenseContext<'a>) -> Self {
        Self { context }
    }

    pub fn check_temporal(
        &self,
        scope: Node,
        sequence: &[regex::Regex],
        behavior: &TemporalBehavior,
        rule: &dyn FrensenseRule,
    ) -> Vec<Advisory> {
        let file_path = self.context.file_path.to_string_lossy();
        let line = scope.start_position().row + 1;

        let Some(scope_idx) = self.context.symbols.find_function_at(&file_path, line) else {
            return Vec::new();
        };

        let events = self
            .context
            .symbols
            .graph()
            .ordered_events_in_scope(scope_idx);
        let meta = rule.metadata();

        match behavior {
            TemporalBehavior::MustNotFollow => self.check_must_not_follow(&events, sequence, meta),
            TemporalBehavior::MustFollow => self.check_must_follow(&scope, &events, sequence, rule),
            TemporalBehavior::ForbiddenBetween(start_re, end_re) => {
                self.check_forbidden_between(&events, sequence, start_re, end_re, meta)
            }
        }
    }

    fn check_must_not_follow(
        &self,
        events: &[crate::semantics::graph::TemporalEvent],
        sequence: &[regex::Regex],
        meta: &RuleMetadata,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut found_first = false;

        for event in events {
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
                    let file_path = self.context.file_path.to_string_lossy().to_string();
                    let enclosing_symbol = self
                        .context
                        .symbols
                        .find_function_at(&file_path, event.line)
                        .and_then(|idx| self.context.symbols.graph().get_symbol(idx))
                        .map(|s| s.name.clone());

                    let mut adv = Advisory::bare(
                        meta.id.as_ref(),
                        meta.severity,
                        self.context.file_id,
                        self.context.file_path,
                        format!(
                            "Temporal Violation: '{}' must NOT follow '{}' in this scope.",
                            sequence[1].as_str(),
                            sequence[0].as_str()
                        ),
                    )
                    .with_confidence(0.92)
                    .with_line(u32::try_from(event.line).unwrap_or(u32::MAX))
                    .with_column(u32::try_from(event.column).unwrap_or(u32::MAX))
                    .with_content(&event.label)
                    .with_impact(meta.impact.as_ref())
                    .with_improvement(meta.improvement.as_ref());
                    adv.enclosing_symbol = enclosing_symbol;
                    adv.tags = meta.tags.iter().map(ToString::to_string).collect();
                    advisories.push(adv);
                }
            }

            if event.event_type == crate::semantics::graph::EventType::Release {
                found_first = false;
            }
        }
        advisories
    }

    fn check_must_follow(
        &self,
        scope: &Node,
        events: &[crate::semantics::graph::TemporalEvent],
        sequence: &[regex::Regex],
        rule: &dyn FrensenseRule,
    ) -> Vec<Advisory> {
        let mut current_step = 0;
        for event in events {
            if current_step < sequence.len() && sequence[current_step].is_match(&event.label) {
                current_step += 1;
            }
        }

        if current_step > 0 && current_step < sequence.len() {
            let mut adv = rule.new_advisory(
                scope,
                self.context,
                format!(
                    "Temporal Violation: Expected sequence [{}] was incomplete.",
                    sequence
                        .iter()
                        .map(regex::Regex::as_str)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            );
            adv.auto_fixable = false;
            adv.requires_human = true;
            vec![adv]
        } else {
            Vec::new()
        }
    }

    fn check_forbidden_between(
        &self,
        events: &[crate::semantics::graph::TemporalEvent],
        sequence: &[regex::Regex],
        start_re: &regex::Regex,
        end_re: &regex::Regex,
        meta: &RuleMetadata,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut in_forbidden_zone = false;

        for event in events {
            if start_re.is_match(&event.label) {
                in_forbidden_zone = true;
            } else if end_re.is_match(&event.label) {
                in_forbidden_zone = false;
            } else if in_forbidden_zone {
                for (i, re) in sequence.iter().enumerate().skip(2) {
                    if re.is_match(&event.label) {
                        let file_path = self.context.file_path.to_string_lossy().to_string();
                        let enclosing_symbol = self
                            .context
                            .symbols
                            .find_function_at(&file_path, event.line)
                            .and_then(|idx| self.context.symbols.graph().get_symbol(idx))
                            .map(|s| s.name.clone());

                        let mut adv = Advisory::bare(
                            meta.id.as_ref(),
                            meta.severity,
                            self.context.file_id,
                            self.context.file_path,
                            format!(
                                "Temporal Violation: '{}' found between '{}' and '{}', which is forbidden.",
                                sequence[i].as_str(),
                                start_re.as_str(),
                                end_re.as_str()
                            ),
                        )
                        .with_confidence(0.92)
                        .with_line(u32::try_from(event.line).unwrap_or(u32::MAX))
                        .with_column(u32::try_from(event.column).unwrap_or(u32::MAX))
                        .with_content(&event.label)
                        .with_impact(meta.impact.as_ref())
                        .with_improvement(meta.improvement.as_ref());
                        adv.enclosing_symbol = enclosing_symbol;
                        adv.tags = meta.tags.iter().map(ToString::to_string).collect();
                        advisories.push(adv);
                    }
                }
            }
        }
        advisories
    }
}
