// SPDX-License-Identifier: MIT

use super::TaintRegistry;
use super::{DataFlowAnalyzer, TaintOrigin};
use crate::{Advisory, FrensenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a, '_> {
    pub(super) fn node_at(
        &self,
        range: crate::semantics::data_flow::normalization::Range,
    ) -> Node<'a> {
        self.current_tree
            .root_node()
            .descendant_for_byte_range(range.start_byte, range.end_byte)
            .unwrap_or_else(|| self.current_tree.root_node())
    }

    fn record_alias_if_assign(&self, target: &str, value_node: Node<'a>) {
        let val_code = &self.current_source[value_node.start_byte()..value_node.end_byte()];
        if val_code == target {
            return;
        }
        let trimmed = val_code.trim();
        if !trimmed.is_empty()
            && !trimmed.contains(' ')
            && !trimmed.contains('(')
            && !trimmed.contains('+')
            && !trimmed.contains('-')
            && !trimmed.contains('*')
            && !trimmed.contains('/')
            && !trimmed.contains('.')
        {
            self.alias_tracker
                .borrow_mut()
                .record_alias(target, trimmed);
        }
    }

    pub(super) fn process_binding(
        &self,
        name: &'a str,
        value_range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn FrensenseRule,
        registry: &mut TaintRegistry,
        advisories: &mut Vec<Advisory>,
    ) {
        if value_range.start_byte >= block_range.start_byte
            && value_range.end_byte <= block_range.end_byte
        {
            let v_node = self.node_at(value_range);
            registry.register_symbol(name, v_node.start_byte(), v_node.end_byte());
            let val_code = &self.current_source[v_node.start_byte()..v_node.end_byte()];

            self.record_alias_if_assign(name, v_node);

            let origin = if source_re.is_match(name) || source_re.is_match(val_code) {
                Some(TaintOrigin::UserInput)
            } else {
                self.resolve_taint(v_node, source_re, sink_re, rule, registry, advisories)
            };

            if let Some(o) = origin {
                if let Some((obj, prop)) = name.split_once('.') {
                    registry.taint_field(obj, prop, o);
                } else {
                    registry.taint(name, o);
                }
            }

            self.propagate_object_taint(
                name, v_node, source_re, sink_re, rule, registry, advisories,
            );
        }
    }

    pub(super) fn process_assignment(
        &self,
        target: &'a str,
        value_range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn FrensenseRule,
        registry: &mut TaintRegistry,
        advisories: &mut Vec<Advisory>,
    ) {
        if value_range.start_byte >= block_range.start_byte
            && value_range.end_byte <= block_range.end_byte
        {
            let v_node = self.node_at(value_range);
            let val_code = &self.current_source[v_node.start_byte()..v_node.end_byte()];

            self.record_alias_if_assign(target, v_node);

            let origin = if source_re.is_match(target) || source_re.is_match(val_code) {
                Some(TaintOrigin::UserInput)
            } else {
                self.resolve_taint(v_node, source_re, sink_re, rule, registry, advisories)
            };

            if let Some(o) = origin {
                if let Some((obj, prop)) = target.split_once('.') {
                    registry.taint_field(obj, prop, o);
                } else {
                    registry.taint(target, o);
                }
            }

            self.propagate_object_taint(
                target, v_node, source_re, sink_re, rule, registry, advisories,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn process_call(
        &self,
        function_name: &'a str,
        args: &[super::normalization::Range],
        range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn FrensenseRule,
        registry: &mut TaintRegistry,
    ) -> Option<Vec<Advisory>> {
        if range.start_byte >= block_range.start_byte && range.end_byte <= block_range.end_byte {
            let arg_nodes: Vec<Node> = args.iter().map(|r| self.node_at(*r)).collect();
            let call_node = self.node_at(range);
            Some(self.analyze_call(
                function_name,
                &arg_nodes,
                call_node,
                source_re,
                sink_re,
                rule,
                registry,
            ))
        } else {
            None
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn process_enter_block(
        &self,
        body_range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn FrensenseRule,
        registry: &mut TaintRegistry,
    ) -> Option<Vec<Advisory>> {
        if body_range.start_byte > block_range.start_byte
            && body_range.end_byte < block_range.end_byte
            && self.depth < self.max_depth
        {
            let body_node = self.node_at(body_range);
            registry.push_scope();
            let sub_analyzer = DataFlowAnalyzer::with_depth(
                self.context,
                self.current_source,
                self.current_tree,
                self.current_file_path,
                self.current_file_id,
                body_node,
                self.depth + 1,
                self.max_depth,
            )
            .with_alias_tracker(self.alias_tracker.borrow().clone());
            let sub_advisories =
                sub_analyzer.analyze_block(body_node, source_re, sink_re, rule, registry);
            registry.pop_scope();
            Some(sub_advisories)
        } else {
            None
        }
    }
}
