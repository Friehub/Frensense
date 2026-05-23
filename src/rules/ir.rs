// SPDX-License-Identifier: MIT

use crate::parser::ParserRegistry;
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
    ScopeConstraint { pattern: Regex, invert: bool },
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
    pub max_file_lines: Option<usize>,
    pub target_ext: String,
    pub target_kinds: Vec<String>,
    pub use_query: bool,
    pub fix_pattern: Option<Regex>,
    pub fix_template: Option<String>,
    pub inject_import: Option<String>,
    pub if_name_matches: Option<Regex>,
    pub body_must_contain: Option<Regex>,
    pub body_may_delegate_via: Option<Regex>,
    pub body_must_contain_any_of: Option<Regex>,
    pub must_be_preceded_by: Option<String>,
    pub auto_fixable: Option<bool>,
    pub requires_human: Option<bool>,
    pub exclude_scope: Option<Regex>,
    pub skip_if_parent: Option<String>,
    pub body_query: Option<String>,
}

impl GenSenseRule for CoreRuleIr {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn applies_to(&self, ext: &str) -> bool {
        self.target_ext == "*" || self.target_ext.split('|').any(|e| e.trim() == ext)
    }

    fn query(&self) -> Option<&str> {
        self.match_queries.first().map(|q| q.selector.as_str())
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // Exclude scope check: skip if file path or any ancestor's source text matches
        if let Some(re) = &self.exclude_scope {
            let file_path = context.file_path.to_string_lossy();
            if re.is_match(&file_path) {
                return Vec::new();
            }
            let mut current = node.parent();
            while let Some(ancestor) = current {
                let text = &context.source_code[ancestor.start_byte()..ancestor.end_byte()];
                if re.is_match(text) {
                    return Vec::new();
                }
                current = ancestor.parent();
            }
        }

        // Skip if parent node kind matches skip_if_parent
        if let Some(kind) = &self.skip_if_parent
            && let Some(parent) = node.parent()
            && parent.kind() == kind.as_str()
        {
            return Vec::new();
        }

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

        if !self.check_regex_matching(node, context, &mut advisories) {
            return advisories;
        }

        if !self.check_structural_matching(node, context, &mut advisories) {
            return advisories;
        }

        self.check_content_constraints(node, context, &mut advisories);
        self.check_metric_constraints(node, context, &mut advisories);
        self.check_flow_constraints(node, context, top, &mut advisories);

        advisories
    }

    fn file_check(&self, context: &GenSenseContext<'_>) -> Vec<Advisory> {
        if let Some(max) = self.max_file_lines {
            let line_count = context.source_code.lines().count();
            if line_count > max {
                let meta = self.metadata();
                return vec![Advisory {
                    rule_id: meta.id.to_string(),
                    file_id: context.file_id,
                    file_path: context.file_path.to_string_lossy().to_string(),
                    severity: meta.severity,
                    confidence: meta.confidence,
                    observation: format!("File length ({line_count} lines) exceeds threshold of {max}."),
                    impact: meta.impact.to_string(),
                    improvement: meta.improvement.to_string(),
                    line: 1,
                    column: 1,
                    start_byte: 0,
                    end_byte: 0,
                    original_content: String::new(),
                    proposed_replacement: None,
                    proposed_import: None,
                    enclosing_symbol: None,
                    fingerprint: String::new(),
                    auto_fixable: false,
                    requires_human: false,
                    tags: meta.tags.iter().map(|t| t.to_string()).collect(),
                }];
            }
        }
        Vec::new()
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

            if let (Some(fix_re), Some(template)) = (&self.fix_pattern, &self.fix_template)
                && let Some(_caps) = fix_re.captures(code)
            {
                let replacement = fix_re.replace_all(code, template).to_string();
                if replacement == code {
                    return false;
                }

                let import = self.inject_import.as_ref().map(|import_template| {
                    let mut import_stmt = String::new();
                    let caps = fix_re
                        .captures(code)
                        .expect("Regex captures should not fail since we just checked them");
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
                // Removed early return to allow flow constraints to run
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
        if let Some(re) = &self.must_contain
            && !re.is_match(code)
        {
            advisories.push(self.new_advisory(
                &node,
                context,
                format!("Pattern '{}' was expected but not found.", re.as_str()),
            ));
        }

        if let Some(re) = &self.body_must_contain_any_of {
            let body_node = node.child_by_field_name("body").unwrap_or(node);
            let checker =
                crate::semantics::reachability::ReachabilityChecker::new(context.source_code);
            if checker.any_reachable_path_contains(body_node, re) {
                let is_bypassed = if let Some(bypass_re) = &self.must_not_contain {
                    checker.any_reachable_path_contains(body_node, bypass_re)
                } else {
                    false
                };

                if !is_bypassed {
                    advisories.push(self.new_advisory(
                        &node,
                        context,
                        self.metadata.observation.to_string(),
                    ));
                }
            }
        } else if let Some(re) = &self.must_not_contain {
            let body_node = node.child_by_field_name("body").unwrap_or(node);
            let checker =
                crate::semantics::reachability::ReachabilityChecker::new(context.source_code);
            if checker.any_reachable_path_contains(body_node, re) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Prohibited pattern '{}' was found.", re.as_str()),
                ));
            }
        }

        // Body tree-sitter query: run query within the body subtree
        if let Some(query_str) = &self.body_query {
            let body_node = node.child_by_field_name("body").unwrap_or(node);
            if let Ok(language) = ParserRegistry::get_language(context.file_path)
                && let Ok(query) = tree_sitter::Query::new(&language, query_str)
            {
                let mut cursor = tree_sitter::QueryCursor::new();
                let has_match = cursor
                    .matches(&query, body_node, context.source_code.as_bytes())
                    .next()
                    .is_some();
                if has_match {
                    advisories.push(self.new_advisory(
                        &node,
                        context,
                        self.metadata.observation.to_string(),
                    ));
                }
            }
        }

        if let Some(kind) = &self.must_be_preceded_by {
            let mut prev = node.prev_sibling();
            let mut found = false;
            while let Some(p) = prev {
                if p.kind() == kind {
                    found = true;
                    break;
                }
                if p.kind() == "line_comment" || p.kind() == "block_comment" {
                    prev = p.prev_sibling();
                    continue;
                }
                break;
            }
            if !found {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    self.metadata.observation.to_string(),
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
        if let Some(max) = self.max_lines
            && node_lines > max
        {
            advisories.push(self.new_advisory(
                &node,
                context,
                format!("Function size ({node_lines} lines) exceeds threshold of {max}."),
            ));
        }

        if let Some(max) = self.max_depth {
            let mut depth = 0;
            let mut curr = node;
            while let Some(parent) = curr.parent() {
                let kind = parent.kind();
                let is_control_flow = matches!(
                    kind,
                    "if_expression"
                        | "for_expression"
                        | "while_expression"
                        | "loop_expression"
                        | "match_expression"
                        | "match_arm"
                        | "if_statement"
                        | "for_statement"
                        | "for_in_statement"
                        | "while_statement"
                        | "do_statement"
                        | "switch_statement"
                        | "catch_clause"
                        | "block"
                        | "compound_statement"
                );
                if is_control_flow {
                    depth += 1;
                }
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

    // gensense-ignore RUST_LOCK_IO
    fn evaluate_taint_constraint<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        top: Node<'a>,
        source: &regex::Regex,
        sink: &regex::Regex,
        constraint_type: &str,
        file_path: &str,
        func_or_node_line: usize,
        advisories: &mut Vec<Advisory>,
    ) {
        let constraint_cache_key = (
            constraint_type.to_string(),
            source.as_str().to_string(),
            sink.as_str().to_string(),
            file_path.to_string(),
            func_or_node_line,
        );

        let cached_findings = {
            let cache = context.taint_cache.borrow();
            cache.get(&constraint_cache_key).cloned()
        };

        if let Some(mut findings) = cached_findings {
            for a in &mut findings {
                a.rule_id = self.metadata.id.to_string();
                a.severity = self.metadata.severity;
                a.observation = self.metadata.observation.to_string();
                a.impact = self.metadata.impact.to_string();
                a.improvement = self.metadata.improvement.to_string();
                a.confidence = self.metadata.confidence;
            }
            advisories.extend(findings);
        } else {
            let analyzer = DataFlowAnalyzer::new(context, top);
            let mut registry = TaintRegistry::default();
            analyzer.discover_symbols(&mut registry);
            let target_node = node.child_by_field_name("body").unwrap_or(node);
            let findings = analyzer.analyze_block(target_node, source, sink, self, &mut registry);

            let mut cache = context.taint_cache.borrow_mut();
            cache.insert(constraint_cache_key, findings.clone());

            advisories.extend(findings);
        }
    }

    fn check_flow_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
        top: Node<'a>,
        advisories: &mut Vec<Advisory>,
    ) {
        let file_path = context.file_path.to_string_lossy().to_string();
        let function_line = context
            .symbols
            .find_function_at(&file_path, node.start_position().row + 1)
            .and_then(|idx| context.symbols.graph().get_symbol(idx))
            .map_or(0, |s| s.line);

        let func_or_node_line = if function_line == 0 {
            node.start_position().row
        } else {
            function_line
        };

        for constraint in &self.flow_constraints {
            match constraint {
                FlowConstraint::TaintReached { source, sink } => {
                    self.evaluate_taint_constraint(
                        node,
                        context,
                        top,
                        source,
                        sink,
                        "reached",
                        &file_path,
                        func_or_node_line,
                        advisories,
                    );
                }
                FlowConstraint::TaintForbidden { source, sink } => {
                    self.evaluate_taint_constraint(
                        node,
                        context,
                        top,
                        source,
                        sink,
                        "forbidden",
                        &file_path,
                        func_or_node_line,
                        advisories,
                    );
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
                        advisories.push(self.new_advisory(
                            &node,
                            context,
                            self.metadata.observation.to_string(),
                        ));
                    }
                }
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
        let rule_id = self.metadata.id.clone().into_owned();
        let file_path = context.file_path.to_string_lossy().to_string();
        let enclosing_symbol = context
            .symbols
            .find_function_at(
                context.file_path.to_str().unwrap_or(""),
                node.start_position().row + 1,
            )
            .and_then(|id| context.symbols.get_symbol(id))
            .map(|s| s.name.clone());
        let original_content = context.source_code[node.start_byte()..node.end_byte()].to_string();

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
                hash = hash.wrapping_mul(0x10_0000_01b3);
            }
            format!("{hash:016x}")
        };

        Advisory {
            rule_id,
            file_id: context.file_id,
            file_path,
            severity: self.metadata.severity,
            observation,
            impact: self.metadata.impact.to_string(),
            improvement: self.metadata.improvement.to_string(),
            line: u32::try_from(node.start_position().row + 1).unwrap_or(u32::MAX),
            column: u32::try_from(node.start_position().column + 1).unwrap_or(u32::MAX),
            start_byte: u32::try_from(node.start_byte()).unwrap_or(u32::MAX),
            end_byte: u32::try_from(node.end_byte()).unwrap_or(u32::MAX),
            original_content,
            proposed_replacement: None,
            proposed_import: None,
            enclosing_symbol,
            confidence: self.metadata.confidence,
            fingerprint,
            auto_fixable: self.auto_fixable.unwrap_or(false),
            requires_human: self.requires_human.unwrap_or(false),
            tags: self.metadata.tags.iter().map(ToString::to_string).collect(),
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
        adv.auto_fixable = self.auto_fixable.unwrap_or(true);
        adv.requires_human = self.requires_human.unwrap_or(false);
        adv
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.metadata.id
    }
}

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
                hash = hash.wrapping_mul(0x10_0000_01b3);
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
        let root = find_project_root(sources);

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

fn find_project_root(sources: &crate::SourceRegistry) -> std::path::PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        return cwd;
    }

    // Fallback (cwd unavailable — rare): compute the longest common prefix
    // of all registered source files.
    let mut files = sources.all_files().peekable();
    if files.peek().is_none() {
        return std::path::PathBuf::from(".");
    }

    let mut common_path: Option<std::path::PathBuf> = None;
    for file in files {
        if let Some(ref current) = common_path {
            let mut common = std::path::PathBuf::new();
            let current_comps: Vec<_> = current.components().collect();
            let file_comps: Vec<_> = file.path.components().collect();
            for (c1, c2) in current_comps.into_iter().zip(file_comps) {
                if c1 == c2 {
                    common.push(c1);
                } else {
                    break;
                }
            }
            common_path = Some(common);
        } else {
            common_path = Some(file.path.parent().map_or_else(
                || std::path::PathBuf::from("."),
                std::path::Path::to_path_buf,
            ));
        }
    }

    common_path.unwrap_or_else(|| std::path::PathBuf::from("."))
}
