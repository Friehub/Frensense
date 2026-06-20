// SPDX-License-Identifier: MIT

use super::DataFlowAnalyzer;
use super::TaintRegistry;
use crate::Advisory;
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
        registry: &mut TaintRegistry,
        _advisories: &mut Vec<Advisory>,
    ) {
        if value_range.start_byte >= block_range.start_byte
            && value_range.end_byte <= block_range.end_byte
        {
            let v_node = self.node_at(value_range);
            registry.register_symbol(name, v_node.start_byte(), v_node.end_byte());

            self.record_alias_if_assign(name, v_node);
        }
    }

    pub(super) fn process_assignment(
        &self,
        target: &'a str,
        value_range: super::normalization::Range,
        block_range: super::normalization::Range,
        _registry: &mut TaintRegistry,
        _advisories: &mut Vec<Advisory>,
    ) {
        if value_range.start_byte >= block_range.start_byte
            && value_range.end_byte <= block_range.end_byte
        {
            let v_node = self.node_at(value_range);

            self.record_alias_if_assign(target, v_node);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn process_call(
        &self,
        _function_name: &'a str,
        _args: &[super::normalization::Range],
        _range: super::normalization::Range,
        _block_range: super::normalization::Range,
        _registry: &mut TaintRegistry,
    ) -> Option<Vec<Advisory>> {
        // Call processing retained for potential future corpus-based interprocedural analysis
        None
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn process_enter_block(
        &self,
        _body_range: super::normalization::Range,
        _block_range: super::normalization::Range,
        _registry: &mut TaintRegistry,
    ) -> Option<Vec<Advisory>> {
        // Block processing retained for potential future corpus-based interprocedural analysis
        None
    }
}
