// SPDX-License-Identifier: MIT

use crate::semantics::data_flow::DataFlowAnalyzer;
use crate::semantics::data_flow::TaintRegistry;
use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata};
use regex::Regex;
use tree_sitter::Node;

/// Tree-sitter query with capture.
#[derive(Debug, Clone)]
pub struct AstQuery {
    pub selector: String,
    #[allow(clippy::pub_underscore_fields)]
    pub _capture_name: String,
}

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
    ScopeConstraint { pattern: Regex },
}

#[derive(Debug, Clone)]
pub enum TemporalBehavior {
    MustFollow,
    MustNotFollow,
    ForbiddenBetween(Regex, Regex),
}

/// Compiled Internal Representation of a Core Rule.
#[derive(Debug, Clone)]
pub struct CoreRuleIr {
    pub metadata: RuleMetadata,
    pub match_queries: Vec<AstQuery>,
    pub flow_constraints: Vec<FlowConstraint>,
    pub if_matches: Option<Regex>,
    pub must_contain: Option<Regex>,
    pub must_not_contain: Option<Regex>,
    pub max_lines: Option<usize>,
    pub max_depth: Option<usize>,
    pub target_ext: String,
    pub target_kinds: Vec<String>,
    pub use_query: bool,
    pub fix_pattern: Option<Regex>,
    pub fix_template: Option<String>,
    pub inject_import: Option<String>,
    pub if_name_matches: Option<Regex>,
    pub body_must_contain: Option<Regex>,
    pub body_may_delegate_via: Option<Regex>,
}

impl GenSenseRule for CoreRuleIr {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn applies_to(&self, ext: &str) -> bool {
        self.target_ext == ext || self.target_ext == "*"
    }

    fn query(&self) -> Option<&str> {
        if self.use_query && !self.match_queries.is_empty() {
            Some(&self.match_queries[0].selector)
        } else {
            None
        }
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut top = node;
        while let Some(parent) = top.parent() {
            top = parent;
        }

        // If not using a query, we must filter by node kind manually in walk_tree context
        if !self.use_query && !self.target_kinds.is_empty() {
            let kind = node.kind();
            if !self.target_kinds.iter().any(|k| k == kind) {
                return Vec::new();
            }
        }

        // Taint Cache Check
        let file_path = context.file_path.to_string_lossy().to_string();
        let function_line = context
            .symbols
            .find_function_at(&file_path, node.start_position().row + 1)
            .and_then(|idx| context.symbols.graph().get_symbol(idx))
            .map_or(0, |s| s.line);

        let cache_key = (
            self.id().to_string(),
            file_path,
            if function_line == 0 {
                node.start_position().row
            } else {
                function_line
            },
        );

        {
            let cache = context.taint_cache.borrow();
            if cache.contains_key(&cache_key) {
                return Vec::new();
            }
        }

        if !self.check_regex_matching(node, context, &mut advisories) {
            return advisories;
        }

        if !self.check_structural_matching(node, context, &mut advisories) {
            return advisories;
        }

        self.check_content_constraints(node, context, &mut advisories);
        self.check_metric_constraints(node, context, &mut advisories);
        self.check_flow_constraints(node, context, top, &cache_key, &mut advisories);

        advisories
    }
}

impl CoreRuleIr {
    fn check_regex_matching<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        advisories: &mut Vec<Advisory>,
    ) -> bool {
        let code = &context.source_code[node.start_byte()..node.end_byte()];
        if let Some(re) = &self.if_matches {
            if !re.is_match(code) {
                return false;
            }

            if let Some(fix_re) = &self.fix_pattern {
                if let Some(template) = &self.fix_template {
                    if let Some(_caps) = fix_re.captures(code) {
                        let replacement = fix_re.replace_all(code, template).to_string();
                        if replacement == code {
                            return false;
                        }

                        let import = self.inject_import.as_ref().map(|import_template| {
                            let mut import_stmt = String::new();
                            let caps = fix_re.captures(code).expect(
                                "Regex captures should not fail since we just checked them",
                            );
                            caps.expand(import_template, &mut import_stmt);
                            import_stmt
                        });

                        advisories.push(self.new_remediated_advisory(
                            &node,
                            context,
                            self.metadata.observation.to_string(),
                            replacement,
                            import,
                        ));
                        return false;
                    }
                }
            }

            if self.flow_constraints.is_empty()
                && self.must_contain.is_none()
                && self.must_not_contain.is_none()
                && self.max_lines.is_none()
                && self.max_depth.is_none()
                && self.if_name_matches.is_none()
                && self.body_must_contain.is_none()
            {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    self.metadata.observation.to_string(),
                ));
            }
        }
        true
    }

    fn check_structural_matching<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        advisories: &mut Vec<Advisory>,
    ) -> bool {
        if let Some(re) = &self.if_name_matches {
            let name_node = node.child_by_field_name("name").or_else(|| {
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.kind().contains("identifier") {
                            return Some(child);
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                None
            });

            if let Some(name_node) = name_node {
                let name = &context.source_code[name_node.start_byte()..name_node.end_byte()];
                if !re.is_match(name) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(re) = &self.body_must_contain {
            let body_node = node.child_by_field_name("body").unwrap_or(node);
            let checker =
                crate::semantics::reachability::ReachabilityChecker::new(context.source_code);
            if !checker.any_reachable_path_contains(body_node, re) {
                let mut is_delegated = false;
                if let Some(delegation_re) = &self.body_may_delegate_via {
                    let code_in_body =
                        &context.source_code[body_node.start_byte()..body_node.end_byte()];
                    if delegation_re.is_match(code_in_body) {
                        is_delegated = true;
                    }
                }

                if !is_delegated {
                    advisories.push(self.new_advisory(
                        &node,
                        context,
                        format!(
                            "Function body has no reachable path containing '{}'.",
                            re.as_str()
                        ),
                    ));
                }
            }
        }
        true
    }

    fn check_content_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        advisories: &mut Vec<Advisory>,
    ) {
        let code = &context.source_code[node.start_byte()..node.end_byte()];
        if let Some(re) = &self.must_contain {
            if !re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Pattern '{}' was expected but not found.", re.as_str()),
                ));
            }
        }

        if let Some(re) = &self.must_not_contain {
            if re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Prohibited pattern '{}' was found.", re.as_str()),
                ));
            }
        }
    }

    fn check_metric_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        advisories: &mut Vec<Advisory>,
    ) {
        let node_lines = node.end_position().row - node.start_position().row + 1;
        if let Some(max) = self.max_lines {
            if node_lines > max {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Function size ({node_lines} lines) exceeds threshold of {max}."),
                ));
            }
        }

        if let Some(max) = self.max_depth {
            let mut depth = 0;
            let mut curr = node;
            while let Some(parent) = curr.parent() {
                depth += 1;
                curr = parent;
            }
            if depth > max {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Nesting depth ({depth}) exceeds threshold of {max}."),
                ));
            }
        }
    }

    fn check_flow_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        top: Node<'a>,
        cache_key: &(String, String, usize),
        advisories: &mut Vec<Advisory>,
    ) {
        for constraint in &self.flow_constraints {
            match constraint {
                FlowConstraint::TaintReached { source, sink } => {
                    let analyzer = DataFlowAnalyzer::new(context, top);
                    let mut registry = TaintRegistry::default();
                    analyzer.discover_symbols(&mut registry);
                    let target_node = node.child_by_field_name("body").unwrap_or(node);
                    let findings =
                        analyzer.analyze_block(target_node, source, sink, self, registry);
                    if !findings.is_empty() {
                        advisories.extend(findings);
                    }
                    let mut cache = context.taint_cache.borrow_mut();
                    cache.insert(cache_key.clone(), advisories.clone());
                }
                FlowConstraint::TaintForbidden { .. } | FlowConstraint::ScopeConstraint { .. } => {}
                FlowConstraint::Temporal { sequence, behavior } => {
                    let analyzer = crate::semantics::temporal::TemporalAnalyzer::new(context);
                    advisories.extend(analyzer.check_temporal(node, sequence, behavior, self));
                }
            }
        }
    }

    fn new_advisory<'a>(
        &self,
        node: &Node<'a>,
        context: &GenSenseContext<'a>,
        observation: String,
    ) -> Advisory {
        Advisory {
            rule_id: self.metadata.id.clone().into_owned(),
            file_id: context.file_id,
            file_path: context.file_path.to_string_lossy().to_string(),
            severity: self.metadata.severity,
            observation,
            impact: self.metadata.impact.to_string(),
            improvement: self.metadata.improvement.to_string(),
            line: u32::try_from(node.start_position().row + 1).unwrap_or(u32::MAX),
            column: u32::try_from(node.start_position().column + 1).unwrap_or(u32::MAX),
            start_byte: u32::try_from(node.start_byte()).unwrap_or(u32::MAX),
            end_byte: u32::try_from(node.end_byte()).unwrap_or(u32::MAX),
            original_content: context.source_code[node.start_byte()..node.end_byte()].to_string(),
            proposed_replacement: None,
            proposed_import: None,
            enclosing_symbol: context
                .symbols
                .find_function_at(
                    context.file_path.to_str().unwrap_or(""),
                    node.start_position().row + 1,
                )
                .and_then(|id| context.symbols.get_symbol(id))
                .map(|s| s.name.clone()),
            confidence: 1.0,
            fingerprint: String::new(),
        }
    }

    fn new_remediated_advisory<'a>(
        &self,
        node: &Node<'a>,
        context: &GenSenseContext<'a>,
        observation: String,
        replacement: String,
        import: Option<String>,
    ) -> Advisory {
        let mut adv = self.new_advisory(node, context, observation);
        adv.proposed_replacement = Some(replacement);
        adv.proposed_import = import;
        adv
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.metadata.id
    }
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
        _sources: &crate::SourceRegistry,
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
                ProjectFlowConstraint::GlobalDataFlow { .. } => {}
            }
        }

        advisories
    }
}

impl ProjectRuleIr {
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

        for source in sources {
            let mut covered = false;
            for guard in &guards {
                if symbols.graph().has_call_path(&source.name, &guard.name) {
                    covered = true;
                    break;
                }
            }

            if !covered {
                advisories.push(Advisory {
                    rule_id: self.metadata.id.to_string(),
                    file_id: source.file_id,
                    file_path: source.file_path.clone(),
                    line: u32::try_from(source.line).unwrap_or(0),
                    column: u32::try_from(source.column).unwrap_or(0),
                    severity: self.metadata.severity,
                    observation: format!(
                        "{}: missing a reachable security guard",
                        self.metadata.observation
                    ),
                    impact: self.metadata.impact.to_string(),
                    improvement: self.metadata.improvement.to_string(),
                    original_content: source.name.clone(),
                    proposed_replacement: None,
                    proposed_import: None,
                    enclosing_symbol: Some(source.name.clone()),
                    confidence: 1.0,
                    fingerprint: String::new(),
                    start_byte: u32::try_from(source.start_byte).unwrap_or(0),
                    end_byte: u32::try_from(source.end_byte).unwrap_or(0),
                });
            }
        }
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

        for target in targets {
            let callers = symbols.find_callers(&target.name);
            for caller in callers {
                if caller.file_path != target.file_path
                    && !glob.matches_with(&caller.file_path, options)
                {
                    advisories.push(Advisory {
                        rule_id: self.metadata.id.to_string(),
                        file_id: caller.file_id,
                        file_path: caller.file_path.clone(),
                        line: u32::try_from(caller.line).unwrap_or(0),
                        column: u32::try_from(caller.column).unwrap_or(0),
                        severity: self.metadata.severity,
                        observation: format!(
                            "{}: called from outside its file ({})",
                            self.metadata.observation, caller.file_path
                        ),
                        impact: self.metadata.impact.to_string(),
                        improvement: self.metadata.improvement.to_string(),
                        original_content: target.name.clone(),
                        proposed_replacement: None,
                        proposed_import: None,
                        enclosing_symbol: Some(caller.name.clone()),
                        confidence: 1.0,
                        fingerprint: String::new(),
                        start_byte: u32::try_from(caller.start_byte).unwrap_or(0),
                        end_byte: u32::try_from(caller.end_byte).unwrap_or(0),
                    });
                }
            }
        }
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

        for source in sources {
            for sink in &sinks {
                if source.file_path != sink.file_path
                    && symbols.graph().has_call_path(&source.name, &sink.name)
                {
                    advisories.push(Advisory {
                        rule_id: self.metadata.id.to_string(),
                        file_id: source.file_id,
                        file_path: source.file_path.clone(),
                        line: u32::try_from(source.line).unwrap_or(0),
                        column: u32::try_from(source.column).unwrap_or(0),
                        severity: self.metadata.severity,
                        observation: format!(
                            "{}: can reach sensitive sink",
                            self.metadata.observation
                        ),
                        impact: self.metadata.impact.to_string(),
                        improvement: self.metadata.improvement.to_string(),
                        original_content: source.name.clone(),
                        proposed_replacement: None,
                        proposed_import: None,
                        enclosing_symbol: Some(source.name.clone()),
                        confidence: 1.0,
                        fingerprint: String::new(),
                        start_byte: u32::try_from(source.start_byte).unwrap_or(0),
                        end_byte: u32::try_from(source.end_byte).unwrap_or(0),
                    });
                }
            }
        }
        advisories
    }
}
