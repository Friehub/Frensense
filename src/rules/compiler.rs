// SPDX-License-Identifier: MIT

use crate::rules::core::CoreRule; // For now, we compile FROM the existing CoreRule struct (DSL)
use crate::rules::ir::{AstQuery, CoreRuleIr, DiagnosticTemplate, FlowConstraint};
use crate::Severity;

/// The GenSense Rule Compiler.
/// Transforms declarative DSL (YAML-backed CoreRule) into optimized CoreRuleIr.
pub struct RuleCompiler;

impl RuleCompiler {
    pub fn compile(dsl: CoreRule) -> CoreRuleIr {
        let mut match_queries = Vec::new();
        let mut flow_constraints = Vec::new();

        // 1. Compile Match Queries
        // If on_node is a query, use it directly. If it's a kind list, normalize it.
        if dsl.on_node.contains(" ") || dsl.on_node.contains("(") {
            match_queries.push(AstQuery {
                selector: dsl.on_node.clone(),
                capture_name: "node".to_string(), // Default capture
            });
        } else {
            // Normalize "function_item|method_definition" into a query
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

        // 2. Compile Flow Constraints
        if let (Some(src), Some(sink)) = (dsl.source_pattern, dsl.sink_pattern) {
            flow_constraints.push(FlowConstraint::TaintReached {
                source: src.as_str().to_string(),
                sink: sink.as_str().to_string(),
            });
        }

        if let Some(scope) = dsl.within_scope {
            flow_constraints.push(FlowConstraint::ScopeConstraint { pattern: scope });
        }

        // 3. Construct IR
        CoreRuleIr {
            id: dsl.id,
            match_queries,
            flow_constraints,
            if_matches: dsl.if_matches,
            must_contain: dsl.must_contain,
            must_not_contain: dsl.must_not_contain,
            max_lines: dsl.max_lines,
            max_depth: dsl.max_depth,
            severity: dsl.severity.unwrap_or(Severity::Warning),
            template: DiagnosticTemplate {
                observation: dsl.observation,
                impact: dsl.impact,
                improvement: dsl.improvement,
            },
            target_ext: dsl.target_ext,
            tags: dsl.tags,
            category: dsl.category.unwrap_or_else(|| "General".to_string()),
        }
    }
}
