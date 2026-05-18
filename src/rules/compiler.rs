// SPDX-License-Identifier: MIT

use crate::rules::core::CoreRule;
use crate::rules::ir::{AstQuery, CoreRuleIr, FlowConstraint};

/// The `GenSense` Rule Compiler.
pub struct RuleCompiler;

impl RuleCompiler {
    /// Compiles a DSL rule into its Internal Representation (IR).
    ///
    /// # Errors
    /// Returns an error if the regex patterns in the rule are invalid.
    pub fn compile(dsl: CoreRule) -> crate::Result<CoreRuleIr> {
        let mut match_queries = Vec::new();
        let mut flow_constraints = Vec::new();

        if dsl.on_node.contains(' ') || dsl.on_node.contains('(') {
            match_queries.push(AstQuery {
                selector: dsl.on_node.clone(),
                _capture_name: "node".to_string(),
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
                _capture_name: "node".to_string(),
            });
        }

        if let (Some(src_re), Some(sink_re)) = (dsl.source_pattern, dsl.sink_pattern) {
            flow_constraints.push(FlowConstraint::TaintReached {
                source: src_re,
                sink: sink_re,
            });
        }

        if let (Some(src_re), Some(sink_re)) =
            (dsl.forbidden_source_pattern, dsl.forbidden_sink_pattern)
        {
            flow_constraints.push(FlowConstraint::TaintForbidden {
                source: src_re,
                sink: sink_re,
            });
        }

        if let Some(scope) = dsl.within_scope {
            let re = regex::Regex::new(&scope)
                .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?;
            flow_constraints.push(FlowConstraint::ScopeConstraint {
                pattern: re,
                invert: false,
            });
        }

        if let Some(scope) = dsl.outside_scope {
            let re = regex::Regex::new(&scope)
                .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?;
            flow_constraints.push(FlowConstraint::ScopeConstraint {
                pattern: re,
                invert: true,
            });
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
                "must_not_follow" => crate::rules::ir::TemporalBehavior::MustNotFollow,
                "forbidden_between" => {
                    if sequence.len() == 2 {
                        crate::rules::ir::TemporalBehavior::ForbiddenBetween(
                            sequence[0].clone(),
                            sequence[1].clone(),
                        )
                    } else {
                        return Err(crate::GenSenseError::Pattern(
                            "forbidden_between requires exactly 2 elements in sequence".to_string(),
                        ));
                    }
                }
                _ => crate::rules::ir::TemporalBehavior::MustFollow,
            };
            flow_constraints.push(FlowConstraint::Temporal { sequence, behavior });
        }

        let use_query = dsl.use_query.unwrap_or_else(|| {
            // Default heuristic if not explicitly specified
            !dsl.on_node.contains('|') && dsl.on_node.contains(' ')
        });

        let fix_pattern = if let Some(pat) = dsl.fix_pattern {
            Some(
                regex::Regex::new(&pat)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
            )
        } else {
            None
        };

        let target_kinds: Vec<String> = dsl
            .on_node
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();

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
            target_kinds,
            use_query,
            fix_pattern,
            fix_template: dsl.fix_with,
            inject_import: dsl.inject_import,
            if_name_matches: dsl.if_name_matches,
            body_must_contain: dsl.body_must_contain,
            body_may_delegate_via: dsl.body_may_delegate_via,
            body_must_contain_any_of: dsl.body_must_contain_any_of,
            must_be_preceded_by: dsl.must_be_preceded_by,
        })
    }
}

use crate::rules::core::project::ProjectCoreRule;
use crate::rules::ir::{ProjectFlowConstraint, ProjectRuleIr};

pub struct ProjectRuleCompiler;

impl ProjectRuleCompiler {
    /// Compiles a project-level DSL rule into its Internal Representation (IR).
    ///
    /// # Errors
    /// Returns an error if the regex patterns or file globs in the rule are invalid.
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

        if let Some(taint) = dsl.global_data_flow {
            constraints.push(ProjectFlowConstraint::GlobalDataFlow {
                source_pattern: regex::Regex::new(&taint.source_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                sink_pattern: regex::Regex::new(&taint.sink_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
            });
        }

        Ok(ProjectRuleIr {
            metadata: dsl.metadata,
            constraints,
        })
    }
}
