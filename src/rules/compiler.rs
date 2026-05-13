// SPDX-License-Identifier: MIT

use crate::rules::core::CoreRule;
use crate::rules::ir::{AstQuery, CoreRuleIr, FlowConstraint};

/// The GenSense Rule Compiler.
pub struct RuleCompiler;

impl RuleCompiler {
    pub fn compile(dsl: CoreRule) -> crate::Result<CoreRuleIr> {
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

        if let (Some(src_re), Some(sink_re)) = (dsl.source_pattern, dsl.sink_pattern) {
            flow_constraints.push(FlowConstraint::TaintReached {
                source: src_re,
                sink: sink_re,
            });
        }

        if let Some(scope) = dsl.within_scope {
            flow_constraints.push(FlowConstraint::ScopeConstraint { pattern: scope });
        }

        if let Some(temp) = dsl.temporal {
            let mut sequence = Vec::new();
            for p in temp.sequence {
                sequence.push(
                    regex::Regex::new(&p)
                        .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                );
            }

            let behavior = match temp.behavior.as_str() {
                "must_follow" => crate::rules::ir::TemporalBehavior::MustFollow,
                "must_not_follow" => crate::rules::ir::TemporalBehavior::MustNotFollow,
                _ => crate::rules::ir::TemporalBehavior::MustFollow,
            };
            flow_constraints.push(FlowConstraint::Temporal { sequence, behavior });
        }

        let use_query = dsl.use_query.unwrap_or_else(|| {
            // Default heuristic if not explicitly specified
            !dsl.on_node.contains('|') && dsl.on_node.contains(' ')
        });

        Ok(CoreRuleIr {
            metadata: dsl.metadata,
            match_queries,
            flow_constraints,
            if_matches: dsl.if_matches,
            must_contain: dsl.must_contain,
            must_not_contain: dsl.must_not_contain,
            max_lines: dsl.max_lines,
            max_depth: dsl.max_depth,
            target_ext: dsl.target_ext,
            use_query,
        })
    }
}

use crate::rules::core::project::ProjectCoreRule;
use crate::rules::ir::{ProjectFlowConstraint, ProjectRuleIr};

pub struct ProjectRuleCompiler;

impl ProjectRuleCompiler {
    pub fn compile(dsl: ProjectCoreRule) -> crate::Result<ProjectRuleIr> {
        let mut constraints = Vec::new();

        if let Some(guard) = dsl.must_have_guard {
            constraints.push(ProjectFlowConstraint::MustHaveGuard {
                source_re: regex::Regex::new(&guard.source_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                guard_re: regex::Regex::new(&guard.guard_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                source_glob: glob::Pattern::new(&guard.source_file_glob)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                guard_glob: glob::Pattern::new(&guard.guard_file_glob)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
            });
        }

        if let Some(internal) = dsl.must_be_internal {
            constraints.push(ProjectFlowConstraint::MustBeInternal {
                re: regex::Regex::new(&internal.pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                glob: glob::Pattern::new(&internal.file_glob)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
            });
        }

        if let Some(taint) = dsl.cross_file_taint_free {
            constraints.push(ProjectFlowConstraint::CrossFileTaintFree {
                source_re: regex::Regex::new(&taint.source_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                sink_re: regex::Regex::new(&taint.sink_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
            });
        }

        Ok(ProjectRuleIr {
            metadata: dsl.metadata,
            constraints,
        })
    }
}
