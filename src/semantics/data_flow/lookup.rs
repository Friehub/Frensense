// SPDX-License-Identifier: MIT

use super::DataFlowAnalyzer;
use tree_sitter::Node;

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    pub fn find_definition(
        &self,
        name: &str,
        registry: &super::TaintRegistry<'a>,
    ) -> Option<Node<'a>> {
        // 1. Check local lexical scopes (Active bindings in TaintRegistry)
        if let Some(node) = registry.find_symbol(name) {
            return Some(node);
        }

        // 2. Fallback to Hierarchical Index (Definitions discovered in previous passes)
        let file_path = self.context.file_path.to_string_lossy();
        let line = self.root.start_position().row + 1;

        if let Some(sym) = self.context.symbols.find_at(name, &file_path, line) {
            // Find the node in the current tree based on the symbol's byte range
            return self
                .root
                .descendant_for_byte_range(sym.start_byte, sym.end_byte);
        }

        None
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
