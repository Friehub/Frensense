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
    #[allow(clippy::too_many_lines)]
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

        #[cfg(feature = "temporal")]
        if let Some(temp) = dsl.temporal {
            let (sequence, behavior) = crate::temporal::handler::compile_temporal_config(temp)?;
            flow_constraints.push(FlowConstraint::Temporal { sequence, behavior });
        }

        let use_query = dsl.use_query.unwrap_or_else(|| {
            // Default heuristic if not explicitly specified
            dsl.on_node.contains('(') || (!dsl.on_node.contains('|') && dsl.on_node.contains(' '))
        });

        let fix_pattern = if let Some(pat) = dsl.fix_pattern {
            Some(
                regex::Regex::new(&pat)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
            )
        } else {
            None
        };

        let exclude_scope = if let Some(scope) = dsl.exclude_scope {
            Some(
                regex::Regex::new(&scope)
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
            max_file_lines: dsl.max_file_lines,
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
            auto_fixable: dsl.auto_fixable,
            requires_human: dsl.requires_human,
            exclude_scope,
            skip_if_parent: dsl.skip_if_parent,
            body_query: dsl.body_query,
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

        let schema_contract_present = dsl.source_pattern.is_some()
            || dsl.source_ext.is_some()
            || dsl.source_file_glob.is_some()
            || dsl.schema_type.is_some()
            || dsl.schema_glob.is_some()
            || dsl.schema_extract.is_some();

        if schema_contract_present {
            let source_pattern = dsl.source_pattern.as_ref().ok_or_else(|| {
                crate::GenSenseError::Config(
                    "schema contract rules require source_pattern".to_string(),
                )
            })?;
            let schema_type = dsl.schema_type.ok_or_else(|| {
                crate::GenSenseError::Config(
                    "schema contract rules require schema_type".to_string(),
                )
            })?;
            let schema_glob = dsl.schema_glob.as_ref().ok_or_else(|| {
                crate::GenSenseError::Config(
                    "schema contract rules require schema_glob".to_string(),
                )
            })?;
            let schema_extract = dsl.schema_extract.ok_or_else(|| {
                crate::GenSenseError::Config(
                    "schema contract rules require schema_extract".to_string(),
                )
            })?;

            let source_file_glob = if let Some(glob) = dsl.source_file_glob.as_ref() {
                glob::Pattern::new(glob)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?
            } else if let Some(source_ext) = dsl.source_ext.as_ref() {
                let ext = source_ext.trim().trim_start_matches('.');
                glob::Pattern::new(&format!("**/*.{ext}"))
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?
            } else {
                return Err(crate::GenSenseError::Config(
                    "schema contract rules require either source_file_glob or source_ext"
                        .to_string(),
                ));
            };

            constraints.push(ProjectFlowConstraint::SchemaContract {
                source_capture_re: regex::Regex::new(source_pattern)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                source_file_glob,
                schema_type,
                schema_file_glob: glob::Pattern::new(schema_glob)
                    .map_err(|e| crate::GenSenseError::Pattern(e.to_string()))?,
                schema_extract,
            });
        }

        Ok(ProjectRuleIr {
            metadata: dsl.metadata,
            constraints,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_yaml_file_rule_compilation() {
        let content = std::fs::read_to_string("src/rules/definitions/rust/core.yml")
            .expect("YAML file should exist");
        let wrapper: crate::engine::auditor::common::RulesWrapper =
            serde_yaml::from_str(&content).expect("YAML should parse");
        let rule = wrapper
            .rules
            .iter()
            .find(|r| r.metadata.id == "RUST_LOCK_SLEEP");
        let rule = rule.expect("RUST_LOCK_SLEEP should be in core.yml");
        #[cfg(feature = "temporal")]
        assert!(
            rule.temporal.is_some(),
            "RUST_LOCK_SLEEP should have temporal config"
        );
        let compiled = crate::rules::compiler::RuleCompiler::compile(rule.clone());
        assert!(
            compiled.is_ok(),
            "RUST_LOCK_SLEEP should compile: {:?}",
            compiled.err()
        );
    }

    #[test]
    fn test_temporal_rule_compilation() {
        let yaml = r#"
rules:
- id: RUST_LOCK_SLEEP_TEST
  target_ext: rs
  on_node: function_item
  if_matches: lock
  temporal:
    sequence: ["^lock$", "^sleep$"]
    behavior: must_not_follow
  observation: test
  impact: test
  improvement: test
  name: test
  severity: Warning
  category: Concurrency
  tags: []
  confidence: 0.85
  precision: high
"#;
        let wrapper: crate::engine::auditor::common::RulesWrapper =
            serde_yaml::from_str(yaml).expect("test YAML should parse");
        assert_eq!(wrapper.rules.len(), 1);
        let rule = &wrapper.rules[0];
        assert_eq!(rule.metadata.id, "RUST_LOCK_SLEEP_TEST");
        #[cfg(feature = "temporal")]
        assert!(
            rule.temporal.is_some(),
            "temporal field should be Some when feature is enabled"
        );
        #[cfg(not(feature = "temporal"))]
        assert!(
            rule.temporal.is_none(),
            "temporal field should be None when feature is disabled"
        );
        let compiled = crate::rules::compiler::RuleCompiler::compile(rule.clone());
        assert!(
            compiled.is_ok(),
            "Rule compilation failed: {:?}",
            compiled.err()
        );
    }
}
