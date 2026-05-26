// SPDX-License-Identifier: MIT

use crate::{Advisory, RuleMetadata};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Prisma,
    OpenApi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaExtract {
    ModelNames,
    FieldNames,
    EnumValues,
}

/// Project-wide flow constraint.
#[derive(Debug, Clone)]
pub enum ProjectFlowConstraint {
    MustHaveGuard {
        source_re: Regex,
        guard_re: Regex,
        source_glob: glob::Pattern,
        guard_glob: glob::Pattern,
    },
    MustBeInternal {
        re: Regex,
        glob: glob::Pattern,
    },
    CrossFileTaintFree {
        source_re: Regex,
        sink_re: Regex,
    },
    GlobalDataFlow {
        source_pattern: Regex,
        sink_pattern: Regex,
    },
    SchemaContract {
        source_capture_re: Regex,
        source_file_glob: glob::Pattern,
        schema_type: SchemaType,
        schema_file_glob: glob::Pattern,
        schema_extract: SchemaExtract,
    },
}

#[derive(Debug, Clone)]
pub struct ProjectRuleIr {
    pub metadata: RuleMetadata,
    pub constraints: Vec<ProjectFlowConstraint>,
}

impl crate::ProjectRule for ProjectRuleIr {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn check_project(
        &self,
        symbols: &crate::semantics::symbols::SymbolRegistry,
        sources: &crate::SourceRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        for constraint in &self.constraints {
            match constraint {
                ProjectFlowConstraint::MustHaveGuard {
                    source_re,
                    guard_re,
                    source_glob,
                    guard_glob,
                } => {
                    advisories.extend(self.check_must_have_guard(
                        symbols,
                        source_re,
                        guard_re,
                        source_glob,
                        guard_glob,
                    ));
                }
                ProjectFlowConstraint::MustBeInternal { re, glob } => {
                    advisories.extend(self.check_must_be_internal(symbols, re, glob));
                }
                ProjectFlowConstraint::CrossFileTaintFree { source_re, sink_re } => {
                    advisories
                        .extend(self.check_cross_file_taint_free(symbols, source_re, sink_re));
                }
                ProjectFlowConstraint::GlobalDataFlow {
                    source_pattern,
                    sink_pattern,
                } => {
                    advisories.extend(self.check_global_data_flow(
                        symbols,
                        source_pattern,
                        sink_pattern,
                    ));
                }
                ProjectFlowConstraint::SchemaContract {
                    source_capture_re,
                    source_file_glob,
                    schema_type,
                    schema_file_glob,
                    schema_extract,
                } => {
                    advisories.extend(self.check_schema_contract(
                        sources,
                        source_capture_re,
                        source_file_glob,
                        *schema_type,
                        schema_file_glob,
                        *schema_extract,
                    ));
                }
            }
        }

        advisories
    }
}

impl ProjectRuleIr {
    #[allow(clippy::too_many_arguments)]
    fn new_advisory(
        &self,
        file_id: crate::FileId,
        file_path: String,
        line: u32,
        column: u32,
        observation: String,
        original_content: String,
        enclosing_symbol: Option<String>,
        start_byte: u32,
        end_byte: u32,
    ) -> Advisory {
        let rule_id = self.metadata.id.to_string();
        let fingerprint = {
            let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
            let input = format!(
                "{}:{}:{}:{}",
                rule_id,
                file_path,
                enclosing_symbol.as_deref().unwrap_or(""),
                original_content
            );
            for byte in input.bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x100_0000_01b3);
            }
            format!("{hash:016x}")
        };

        Advisory {
            rule_id,
            file_id,
            file_path,
            line,
            column,
            severity: self.metadata.severity,
            observation,
            impact: self.metadata.impact.to_string(),
            improvement: self.metadata.improvement.to_string(),
            original_content,
            proposed_replacement: None,
            proposed_import: None,
            enclosing_symbol,
            confidence: self.metadata.confidence,
            fingerprint,
            start_byte,
            end_byte,
            auto_fixable: false,
            requires_human: true,
            tags: self.metadata.tags.iter().map(ToString::to_string).collect(),
        }
    }

    fn check_must_have_guard(
        &self,
        symbols: &crate::semantics::symbols::SymbolRegistry,
        source_re: &Regex,
        guard_re: &Regex,
        source_glob: &glob::Pattern,
        guard_glob: &glob::Pattern,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let all_symbols = symbols.query_all();

        let sources: Vec<_> = all_symbols
            .iter()
            .filter(|s| source_re.is_match(&s.name) && source_glob.matches(&s.file_path))
            .collect();

        let guards: Vec<_> = all_symbols
            .iter()
            .filter(|s| guard_re.is_match(&s.name) && guard_glob.matches(&s.file_path))
            .collect();

        let new_advisories = sources.into_iter().filter_map(|source| {
            let mut covered = false;
            let source_nodes = symbols.graph().find_nodes(&source.name);
            for guard in &guards {
                let guard_nodes = symbols.graph().find_nodes(&guard.name);
                if symbols.graph().has_call_path(&source_nodes, &guard_nodes) {
                    covered = true;
                    break;
                }
            }

            if covered {
                None
            } else {
                Some(self.new_advisory(
                    source.file_id,
                    source.file_path.clone(),
                    u32::try_from(source.line).unwrap_or(0),
                    u32::try_from(source.column).unwrap_or(0),
                    format!(
                        "{}: missing a reachable security guard",
                        self.metadata.observation
                    ),
                    source.name.clone(),
                    Some(source.name.clone()),
                    u32::try_from(source.start_byte).unwrap_or(0),
                    u32::try_from(source.end_byte).unwrap_or(0),
                ))
            }
        });
        advisories.extend(new_advisories);
        advisories
    }

    fn check_must_be_internal(
        &self,
        symbols: &crate::semantics::symbols::SymbolRegistry,
        re: &Regex,
        glob: &glob::Pattern,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let all_symbols = symbols.query_all();
        let options = glob::MatchOptions {
            case_sensitive: true,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };

        let targets: Vec<_> = all_symbols
            .iter()
            .filter(|s| re.is_match(&s.name))
            .collect();

        let new_advisories = targets.iter().flat_map(|target| {
            symbols
                .find_callers(&target.name)
                .into_iter()
                .filter_map(move |caller| {
                    if caller.file_path != target.file_path
                        && !glob.matches_with(&caller.file_path, options)
                    {
                        Some(self.new_advisory(
                            caller.file_id,
                            caller.file_path.clone(),
                            u32::try_from(caller.line).unwrap_or(0),
                            u32::try_from(caller.column).unwrap_or(0),
                            format!(
                                "{}: called from outside its file ({})",
                                self.metadata.observation, caller.file_path
                            ),
                            target.name.clone(),
                            Some(caller.name.clone()),
                            u32::try_from(caller.start_byte).unwrap_or(0),
                            u32::try_from(caller.end_byte).unwrap_or(0),
                        ))
                    } else {
                        None
                    }
                })
        });
        advisories.extend(new_advisories);
        advisories
    }

    fn check_cross_file_taint_free(
        &self,
        symbols: &crate::semantics::symbols::SymbolRegistry,
        source_re: &Regex,
        sink_re: &Regex,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let all_symbols = symbols.query_all();

        let sources: Vec<_> = all_symbols
            .iter()
            .filter(|s| source_re.is_match(&s.name))
            .collect();
        let sinks: Vec<_> = all_symbols
            .iter()
            .filter(|s| sink_re.is_match(&s.name))
            .collect();

        let mut violations = Vec::new();
        for source in sources {
            let source_nodes = symbols.graph().find_nodes(&source.name);
            for sink in &sinks {
                let sink_nodes = symbols.graph().find_nodes(&sink.name);
                if source.file_path != sink.file_path
                    && symbols.graph().has_call_path(&source_nodes, &sink_nodes)
                {
                    violations.push(source);
                }
            }
        }

        advisories.extend(violations.iter().map(|source| {
            self.new_advisory(
                source.file_id,
                source.file_path.clone(),
                u32::try_from(source.line).unwrap_or(0),
                u32::try_from(source.column).unwrap_or(0),
                format!("{}: can reach sensitive sink", self.metadata.observation),
                source.name.clone(),
                Some(source.name.clone()),
                u32::try_from(source.start_byte).unwrap_or(0),
                u32::try_from(source.end_byte).unwrap_or(0),
            )
        }));
        advisories
    }

    fn check_global_data_flow(
        &self,
        symbols: &crate::semantics::symbols::SymbolRegistry,
        source_re: &Regex,
        sink_re: &Regex,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let all_symbols = symbols.query_all();

        let sources: Vec<_> = all_symbols
            .iter()
            .filter(|s| source_re.is_match(&s.name))
            .collect();
        let sinks: Vec<_> = all_symbols
            .iter()
            .filter(|s| sink_re.is_match(&s.name))
            .collect();

        let mut violations = Vec::new();
        for source in sources {
            let source_nodes = symbols.graph().find_nodes(&source.name);
            for sink in &sinks {
                let sink_nodes = symbols.graph().find_nodes(&sink.name);
                if symbols.graph().has_call_path(&source_nodes, &sink_nodes) {
                    violations.push(source);
                }
            }
        }

        advisories.extend(violations.iter().map(|source| {
            self.new_advisory(
                source.file_id,
                source.file_path.clone(),
                u32::try_from(source.line).unwrap_or(0),
                u32::try_from(source.column).unwrap_or(0),
                format!(
                    "{}: global reachability: source reached sensitive sink",
                    self.metadata.observation
                ),
                source.name.clone(),
                Some(source.name.clone()),
                u32::try_from(source.start_byte).unwrap_or(0),
                u32::try_from(source.end_byte).unwrap_or(0),
            )
        }));
        advisories
    }

    fn check_schema_contract(
        &self,
        sources: &crate::SourceRegistry,
        source_capture_re: &Regex,
        source_file_glob: &glob::Pattern,
        schema_type: SchemaType,
        schema_file_glob: &glob::Pattern,
        schema_extract: SchemaExtract,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let root = super::find_project_root(sources);

        let valid_names = match (schema_type, schema_extract) {
            (SchemaType::Prisma, SchemaExtract::ModelNames) => {
                crate::rules::schema_contract::prisma_extractor::PrismaExtractor::extract_model_names(
                    schema_file_glob,
                    &root,
                )
            }
            (SchemaType::Prisma, SchemaExtract::FieldNames) => {
                crate::rules::schema_contract::prisma_extractor::PrismaExtractor::extract_field_names(
                    schema_file_glob,
                    &root,
                )
            }
            (SchemaType::Prisma, SchemaExtract::EnumValues) => {
                crate::rules::schema_contract::prisma_extractor::PrismaExtractor::extract_enum_values(
                    schema_file_glob,
                    &root,
                )
            }
            _ => std::collections::HashSet::new(),
        };

        let options = glob::MatchOptions {
            case_sensitive: true,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };

        for file in sources.all_files() {
            let rel_path = file.path.strip_prefix(&root).unwrap_or(&file.path);
            if !source_file_glob.matches_with(rel_path.to_str().unwrap_or(""), options) {
                continue;
            }

            for cap in source_capture_re.captures_iter(&file.content) {
                if let Some(matched_group) = cap.get(1) {
                    let matched_str = matched_group.as_str();
                    if !valid_names.contains(matched_str) {
                        let start_byte = matched_group.start();
                        let end_byte = matched_group.end();

                        let mut line = 1;
                        let mut column = 1;
                        for ch in file.content[..start_byte].chars() {
                            if ch == '\n' {
                                line += 1;
                                column = 1;
                            } else {
                                column += 1;
                            }
                        }

                        let advisory = self.new_advisory(
                            file.id,
                            file.path.to_string_lossy().to_string(),
                            line,
                            column,
                            format!(
                                "{}: '{}' not found in schema",
                                self.metadata.observation, matched_str
                            ),
                            matched_group.as_str().to_string(),
                            None,
                            u32::try_from(start_byte).unwrap_or(u32::MAX),
                            u32::try_from(end_byte).unwrap_or(u32::MAX),
                        );
                        advisories.push(advisory);
                    }
                }
            }
        }

        advisories
    }
}
