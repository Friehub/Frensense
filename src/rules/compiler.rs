// SPDX-License-Identifier: MIT

use crate::rules::core::CoreRule;
use crate::rules::ir::{AstQuery, CoreRuleIr, FlowConstraint};

/// The GenSense Rule Compiler.
pub struct RuleCompiler;

impl RuleCompiler {
    pub fn compile(dsl: CoreRule) -> CoreRuleIr {
        let mut match_queries = Vec::new();
        let mut flow_constraints = Vec::new();

        if dsl.on_node.contains(" ") || dsl.on_node.contains("(") {
            match_queries.push(AstQuery {
                selector: dsl.on_node.clone(),
                capture_name: "node".to_string(),
            });
        } else {
            let kinds: Vec<String> = dsl.on_node.split('|').map(|s| format!("({s})")).collect();
            let query = if kinds.len() > 1 {
                format!("[{}] @node", kinds.join(" "))
            } else {
                format!("{} @node", kinds[0])
            };
            match_queries.push(AstQuery {
                selector: query,
                capture_name: "node".to_string(),
            });
        }

        if let (Some(src), Some(sink)) = (dsl.source_pattern, dsl.sink_pattern) {
            flow_constraints.push(FlowConstraint::TaintReached {
                source: src.as_str().to_string(),
                sink: sink.as_str().to_string(),
            });
        }

        if let Some(scope) = dsl.within_scope {
            flow_constraints.push(FlowConstraint::ScopeConstraint { pattern: scope });
        }

        if let Some(temp) = dsl.temporal {
            let behavior = match temp.behavior.as_str() {
                "must_follow" => crate::rules::ir::TemporalBehavior::MustFollow,
                "must_not_follow" => crate::rules::ir::TemporalBehavior::MustNotFollow,
                _ => crate::rules::ir::TemporalBehavior::MustFollow,
            };
            flow_constraints.push(FlowConstraint::Temporal {
                sequence: temp.sequence,
                behavior,
            });
        }

        CoreRuleIr {
            metadata: dsl.metadata,
            match_queries,
            flow_constraints,
            if_matches: dsl.if_matches,
            must_contain: dsl.must_contain,
            must_not_contain: dsl.must_not_contain,
            max_lines: dsl.max_lines,
            max_depth: dsl.max_depth,
            target_ext: dsl.target_ext,
        }
    }
}
