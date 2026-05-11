// SPDX-License-Identifier: MIT

use super::DataFlowAnalyzer;
use tree_sitter::Node;

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    pub fn find_definition(
        &self,
        name: &str,
        registry: &super::TaintRegistry<'a>,
    ) -> Option<Node<'a>> {
        registry.find_symbol(name)
    }

    pub fn map_params(
        &self,
        def_node: Node<'a>,
        tainted_args: &[usize],
    ) -> Option<super::TaintRegistry<'a>> {
        let params_node = def_node.child_by_field_name("parameters")?;
        let mut registry = super::TaintRegistry::default();
        let mut cursor = params_node.walk();
        let mut p_idx = 0;

        for param in params_node.children(&mut cursor) {
            if matches!(param.kind(), "(" | ")" | ",") {
                continue;
            }
            if tainted_args.contains(&p_idx) {
                let p_name = &self.context.source_code[param.start_byte()..param.end_byte()];
                registry.taint(p_name, "parameter");
            }
            p_idx += 1;
        }
        Some(registry)
    }
}
