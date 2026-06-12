// SPDX-License-Identifier: MIT

use crate::temporal::analyzer::TemporalAnalyzer;
use crate::temporal::config::TemporalConfig;
use crate::{Advisory, FrensenseContext, FrensenseRule};

#[derive(Debug, Clone)]
pub enum TemporalBehavior {
    MustNotFollow,
    MustFollow,
    ForbiddenBetween(regex::Regex, regex::Regex),
}

/// Compile a `TemporalConfig` (from YAML) into the sequence + behavior
/// needed to construct a `FlowConstraint::Temporal`.
///
/// # Errors
/// Returns `FrensenseError::Pattern` if a regex in the sequence is invalid
/// or if `forbidden_between` does not have exactly 2 elements.
pub fn compile_temporal_config(
    config: TemporalConfig,
) -> Result<(Vec<regex::Regex>, TemporalBehavior), crate::FrensenseError> {
    let mut sequence = Vec::new();
    for p in config.sequence {
        sequence
            .push(regex::Regex::new(&p).map_err(|e| crate::FrensenseError::Pattern(e.to_string()))?);
    }

    let behavior = match config.behavior.as_str() {
        "must_not_follow" => TemporalBehavior::MustNotFollow,
        "forbidden_between" => {
            if sequence.len() >= 2 {
                TemporalBehavior::ForbiddenBetween(sequence[0].clone(), sequence[1].clone())
            } else {
                return Err(crate::FrensenseError::Pattern(
                    "forbidden_between requires at least 2 elements in sequence (start, end, + forbidden patterns)".to_string(),
                ));
            }
        }
        _ => TemporalBehavior::MustFollow,
    };

    Ok((sequence, behavior))
}

/// Execute a temporal constraint: build the analyzer, run `check_temporal`,
/// and return advisories.
pub fn check_temporal<'a>(
    node: tree_sitter::Node<'a>,
    context: &FrensenseContext<'a>,
    sequence: &[regex::Regex],
    behavior: &TemporalBehavior,
    rule: &dyn FrensenseRule,
) -> Vec<Advisory> {
    let analyzer = TemporalAnalyzer::new(context);
    analyzer.check_temporal(node, sequence, behavior, rule)
}
