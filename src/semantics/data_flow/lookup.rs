// SPDX-License-Identifier: MIT

use super::DataFlowAnalyzer;
use std::path::Path;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a, '_> {
    #[allow(clippy::type_complexity)]
    #[must_use]
    pub fn find_definition(
        &self,
        full_name: &str,
        registry: &super::TaintRegistry<'a>,
    ) -> Option<(
        Node<'a>,
        &'a str,
        &'a tree_sitter::Tree,
        crate::FileId,
        &'a Path,
        &'a [crate::semantics::data_flow::normalization::SemanticOp],
    )> {
        // Handle qualified names (e.g., crate::mod::fn) by taking the last part
        let name = full_name.split("::").last().unwrap_or(full_name);

        // 1. Check local lexical scopes (Active bindings in TaintRegistry)
        if let Some(node) = registry.find_symbol(name) {
            return Some((
                node,
                self.current_source,
                self.current_tree,
                self.current_file_id,
                self.current_file_path,
                self.context.semantic_ops,
            ));
        }

        // 2. Fallback to Hierarchical Index (Definitions discovered in previous passes)
        let file_path = self.context.file_path.to_string_lossy();
        let line = self.root.start_position().row + 1;

        if let Some(sym) = self.context.symbols.find_at(name, &file_path, line) {
            // Find the node in the current tree based on the symbol's byte range
            return self
                .root
                .descendant_for_byte_range(sym.start_byte, sym.end_byte)
                .map(|node| {
                    (
                        node,
                        self.current_source,
                        self.current_tree,
                        self.current_file_id,
                        self.current_file_path,
                        self.context.semantic_ops,
                    )
                });
        }

        // 3. Global lookup (Symbols in any file, prioritizing current file)
        let all_matches = self.context.symbols.find(name);

        // Try current file first (but any symbol, not just containing ones)
        if let Some(sym) = all_matches
            .iter()
            .find(|s| s.file_path == file_path.as_ref())
        {
            return self
                .current_tree
                .root_node()
                .descendant_for_byte_range(sym.start_byte, sym.end_byte)
                .map(|node| {
                    (
                        node,
                        self.current_source,
                        self.current_tree,
                        self.current_file_id,
                        self.current_file_path,
                        self.context.semantic_ops,
                    )
                });
        }

        // Then try other files
        for sym in all_matches {
            if sym.file_path == file_path.as_ref() {
                continue;
            }
            if let Some((tree, src, ops)) = self.context.file_trees.get(&sym.file_path) {
                if let Some(node) = tree
                    .root_node()
                    .descendant_for_byte_range(sym.start_byte, sym.end_byte)
                {
                    let path =
                        Box::leak(std::path::PathBuf::from(&sym.file_path).into_boxed_path());
                    return Some((node, src, tree, sym.file_id, path, ops));
                }
            }
        }

        None
    }

    #[must_use]
    pub fn map_params(
        &self,
        def_node: Node<'a>,
        def_source: &'a str,
        tainted_args: &[(usize, super::TaintOrigin)],
    ) -> Option<super::TaintRegistry<'a>> {
        let params_node = def_node.child_by_field_name("parameters")?;
        let mut registry = super::TaintRegistry::default();
        let mut cursor = params_node.walk();
        let mut p_idx = 0;

        for param in params_node.children(&mut cursor) {
            if matches!(param.kind(), "(" | ")" | ",") {
                continue;
            }
            if let Some((_, origin)) = tainted_args.iter().find(|(idx, _)| *idx == p_idx) {
                // In Rust, parameter is often (parameter pattern: (identifier) type: (type_identifier))
                let p_node = param.child_by_field_name("pattern").unwrap_or(param);
                let p_name = &def_source[p_node.start_byte()..p_node.end_byte()];
                registry.taint(p_name, origin.clone());
            }
            p_idx += 1;
        }
        Some(registry)
    }
}
