// SPDX-License-Identifier: MIT

use crate::parser::ParserRegistry;
use crate::semantics::data_flow::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, FrensenseContext};
use tree_sitter::Node;

use super::core::CoreRuleIr;
use super::flow::{FlowConstraint, FlowEvaluator};

impl CoreRuleIr {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.metadata.id
    }

    pub(crate) fn new_advisory<'a>(
        &self,
        node: &Node<'a>,
        context: &FrensenseContext<'a>,
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
                hash = hash.wrapping_mul(0x100_0000_01b3);
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

    pub(super) fn new_remediated_advisory<'a>(
        &self,
        node: &Node<'a>,
        context: &FrensenseContext<'a>,
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

    pub(super) fn check_regex_matching<'a>(
        &self,
        node: Node<'a>,
        context: &FrensenseContext<'a>,
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

                let import = self.inject_import.as_ref().and_then(|import_template| {
                    let mut import_stmt = String::new();
                    let caps = fix_re.captures(code)?;
                    caps.expand(import_template, &mut import_stmt);
                    Some(import_stmt)
                });

                advisories.push(self.new_remediated_advisory(
                    &node,
                    context,
                    self.metadata.observation.to_string(),
                    replacement,
                    import,
                ));
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

    pub(super) fn check_structural_matching<'a>(
        &self,
        node: Node<'a>,
        context: &FrensenseContext<'a>,
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
                let is_bypassed = if let Some(bypass_re) = &self.must_not_contain {
                    let code_in_body =
                        &context.source_code[body_node.start_byte()..body_node.end_byte()];
                    bypass_re.is_match(code_in_body)
                } else {
                    false
                };

                if !is_bypassed {
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
        }
        true
    }

    pub(super) fn check_content_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &FrensenseContext<'a>,
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
        } else if self.body_must_contain.is_none()
            && let Some(re) = &self.must_not_contain
        {
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

    pub(super) fn check_metric_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &FrensenseContext<'a>,
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

    // frensense-ignore RUST_LOCK_IO
    pub(crate) fn evaluate_taint_constraint<'a>(
        &self,
        node: Node<'a>,
        context: &FrensenseContext<'a>,
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

        let cached_findings = context.taint_cache.get(&constraint_cache_key);

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
            let max_depth = self.taint_max_depth.unwrap_or(5);
            let mut analyzer = DataFlowAnalyzer::with_depth(
                context,
                context.source_code,
                context.tree,
                context.file_path,
                context.file_id,
                top,
                0,
                max_depth,
            );
            if let Some(ref re) = self.sanitize_pattern {
                analyzer = analyzer.with_sanitizers(re.clone());
            }
            let mut registry = TaintRegistry::default();
            analyzer.discover_symbols(&mut registry);
            let target_node = node.child_by_field_name("body").unwrap_or(node);
            let mut findings = analyzer.analyze_block(target_node, source, sink, self, &mut registry);

            for adv in &mut findings {
                let adjusted = frensense_engine::data_flow::confidence::TaintConfidenceAdjuster::adjust_confidence(
                    context.source_code,
                    context.file_path,
                    adv.line,
                    &adv.original_content,
                    adv.confidence,
                );
                adv.confidence = adjusted;
            }

            context.taint_cache.insert(constraint_cache_key, findings.clone());

            advisories.extend(findings);
        }
    }

    pub(super) fn check_flow_constraints<'a>(
        &self,
        node: Node<'a>,
        context: &FrensenseContext<'a>,
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
                    #[cfg(feature = "temporal")]
                    advisories.extend(crate::temporal::handler::check_temporal(
                        node, context, sequence, behavior, self,
                    ));
                }
                FlowConstraint::AllOf(_)
                | FlowConstraint::AnyOf(_)
                | FlowConstraint::Not(_)
                | FlowConstraint::Across { .. }
                | FlowConstraint::Without { .. }
                | FlowConstraint::Chain { .. } => {
                    let results = FlowEvaluator::evaluate(
                        constraint,
                        node,
                        context,
                        top,
                        self,
                        &file_path,
                        func_or_node_line,
                    );
                    advisories.extend(results);
                }
            }
        }
    }
}
