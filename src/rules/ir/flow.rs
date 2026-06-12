// SPDX-License-Identifier: MIT

use crate::{Advisory, FrensenseContext};
use regex::Regex;
use tree_sitter::Node;

use super::core::CoreRuleIr;

/// Represents a dynamic flow constraint parsed from DSL.
#[derive(Debug, Clone)]
pub enum FlowConstraint {
    /// Asserts data flows from `source` to `sink`.
    TaintReached { source: Regex, sink: Regex },
    /// Asserts data NEVER flows from `source` to `sink`.
    TaintForbidden { source: Regex, sink: Regex },
    /// Asserts structural sequence.
    Temporal {
        sequence: Vec<Regex>,
        behavior: TemporalBehavior,
    },
    /// Asserts scope-level invariants (e.g. within transaction).
    ScopeConstraint { pattern: Regex, invert: bool },
    /// All sub-constraints must match.
    AllOf(Vec<FlowConstraint>),
    /// At least one sub-constraint matches.
    AnyOf(Vec<FlowConstraint>),
    /// Negation: fires when the sub-constraint does NOT match.
    Not(Box<FlowConstraint>),
    /// Sub-constraint must cross a boundary matching `boundary_re`.
    Across {
        constraint: Box<FlowConstraint>,
        boundary_re: Regex,
    },
    /// Fires when `constraint` matches but `exclusion` does not.
    Without {
        constraint: Box<FlowConstraint>,
        exclusion: Box<FlowConstraint>,
    },
    /// Fires when `source` reaches `sink` AND passes through `through`.
    Chain {
        source: Box<FlowConstraint>,
        through: Box<FlowConstraint>,
        sink: Box<FlowConstraint>,
    },
}

#[derive(Debug, Clone)]
pub enum TemporalBehavior {
    MustFollow,
    MustNotFollow,
    ForbiddenBetween(Regex, Regex),
}

/// Evaluates a tree of flow constraints using a recursive walk.
/// Each leaf constraint delegates to the appropriate analysis subsystem.
/// Combinators (`AllOf`, `AnyOf`, `Not`, etc.) recurse into children.
pub struct FlowEvaluator;

impl FlowEvaluator {
    /// Evaluate a single flow constraint against the given node and context.
    /// Returns advisories if the constraint is violated (or satisfied, for Not).
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn evaluate<'a>(
        constraint: &FlowConstraint,
        node: Node<'a>,
        context: &'a FrensenseContext<'a>,
        top: Node<'a>,
        rule: &CoreRuleIr,
        file_path: &str,
        func_or_node_line: usize,
    ) -> Vec<Advisory> {
        match constraint {
            FlowConstraint::AllOf(sub) => {
                let mut all_advisories = Vec::new();
                for sub_c in sub {
                    let results = Self::evaluate(
                        sub_c,
                        node,
                        context,
                        top,
                        rule,
                        file_path,
                        func_or_node_line,
                    );
                    if results.is_empty() {
                        return Vec::new();
                    }
                    all_advisories.extend(results);
                }
                all_advisories
            }
            FlowConstraint::AnyOf(sub) => {
                for sub_c in sub {
                    let results = Self::evaluate(
                        sub_c,
                        node,
                        context,
                        top,
                        rule,
                        file_path,
                        func_or_node_line,
                    );
                    if !results.is_empty() {
                        return results;
                    }
                }
                Vec::new()
            }
            FlowConstraint::Not(sub) => {
                let results =
                    Self::evaluate(sub, node, context, top, rule, file_path, func_or_node_line);
                if results.is_empty() {
                    vec![rule.new_advisory(
                        &node,
                        context,
                        "Negated constraint matched: rule should not have this pattern".to_string(),
                    )]
                } else {
                    Vec::new()
                }
            }
            FlowConstraint::Across {
                constraint,
                boundary_re,
            } => {
                let results = Self::evaluate(
                    constraint,
                    node,
                    context,
                    top,
                    rule,
                    file_path,
                    func_or_node_line,
                );
                if results.is_empty() {
                    return Vec::new();
                }
                let func_name = context
                    .symbols
                    .find_function_at(file_path, node.start_position().row + 1);
                if let Some(func_id) = func_name {
                    let events = context.symbols.graph().ordered_events_in_scope(func_id);
                    let crosses_boundary = events.iter().any(|ev| {
                        boundary_re.is_match(&ev.label)
                            || boundary_re.is_match(&format!("{:?}", ev.event_type))
                    });
                    if crosses_boundary {
                        return results;
                    }
                }
                Vec::new()
            }
            FlowConstraint::Without {
                constraint,
                exclusion,
            } => {
                let primary = Self::evaluate(
                    constraint,
                    node,
                    context,
                    top,
                    rule,
                    file_path,
                    func_or_node_line,
                );
                if primary.is_empty() {
                    return Vec::new();
                }
                let excluded = Self::evaluate(
                    exclusion,
                    node,
                    context,
                    top,
                    rule,
                    file_path,
                    func_or_node_line,
                );
                if excluded.is_empty() {
                    primary
                } else {
                    Vec::new()
                }
            }
            FlowConstraint::Chain {
                source,
                through,
                sink,
            } => {
                let source_results = Self::evaluate(
                    source,
                    node,
                    context,
                    top,
                    rule,
                    file_path,
                    func_or_node_line,
                );
                if source_results.is_empty() {
                    return Vec::new();
                }
                let sink_results =
                    Self::evaluate(sink, node, context, top, rule, file_path, func_or_node_line);
                if sink_results.is_empty() {
                    return Vec::new();
                }
                let through_results = Self::evaluate(
                    through,
                    node,
                    context,
                    top,
                    rule,
                    file_path,
                    func_or_node_line,
                );
                if through_results.is_empty() {
                    return Vec::new();
                }
                source_results
            }
            FlowConstraint::TaintReached { .. }
            | FlowConstraint::TaintForbidden { .. }
            | FlowConstraint::Temporal { .. }
            | FlowConstraint::ScopeConstraint { .. } => {
                let mut leaf_results = Vec::new();
                Self::evaluate_leaf(
                    constraint,
                    node,
                    context,
                    top,
                    rule,
                    file_path,
                    func_or_node_line,
                    &mut leaf_results,
                );
                leaf_results
            }
        }
    }

    /// Evaluate a leaf constraint directly, dispatching to the appropriate subsystem.
    pub fn evaluate_leaf<'a>(
        constraint: &FlowConstraint,
        node: Node<'a>,
        context: &'a FrensenseContext<'a>,
        top: Node<'a>,
        rule: &CoreRuleIr,
        file_path: &str,
        func_or_node_line: usize,
        advisories: &mut Vec<Advisory>,
    ) {
        match constraint {
            FlowConstraint::TaintReached { source, sink } => {
                rule.evaluate_taint_constraint(
                    node,
                    context,
                    top,
                    source,
                    sink,
                    "reached",
                    file_path,
                    func_or_node_line,
                    advisories,
                );
            }
            FlowConstraint::TaintForbidden { source, sink } => {
                rule.evaluate_taint_constraint(
                    node,
                    context,
                    top,
                    source,
                    sink,
                    "forbidden",
                    file_path,
                    func_or_node_line,
                    advisories,
                );
            }
            FlowConstraint::Temporal { sequence, behavior } => {
                #[cfg(feature = "temporal")]
                advisories.extend(crate::temporal::handler::check_temporal(
                    node, context, sequence, behavior, rule,
                ));
            }
            FlowConstraint::ScopeConstraint { pattern, invert } => {
                let mut current = node.parent();
                let mut matched = false;
                while let Some(p) = current {
                    if pattern.is_match(p.kind()) {
                        matched = true;
                        break;
                    }
                    current = p.parent();
                }
                let should_fire = if *invert { !matched } else { matched };
                if should_fire {
                    advisories.push(rule.new_advisory(
                        &node,
                        context,
                        rule.metadata.observation.to_string(),
                    ));
                }
            }
            _ => {}
        }
    }
}
