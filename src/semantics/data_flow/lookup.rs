// SPDX-License-Identifier: MIT

use super::DataFlowAnalyzer;
use std::path::Path;
use tree_sitter::Node;

// intern_path has been removed as we can safely borrow paths from GenSenseContext.

impl<'a> DataFlowAnalyzer<'a, '_> {
    #[allow(clippy::type_complexity)]
    #[must_use]
    pub fn find_definition(
        &self,
        full_name: &str,
        registry: &super::TaintRegistry,
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
        if let Some((start_byte, end_byte)) = registry.find_symbol_range(name)
            && let Some(node) = self.current_tree.root_node().descendant_for_byte_range(start_byte, end_byte)
        {
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
            if let Some((path_str, (tree, src, ops))) =
                self.context.file_trees.get_key_value(&sym.file_path)
                && let Some(node) = tree
                    .root_node()
                    .descendant_for_byte_range(sym.start_byte, sym.end_byte)
            {
                let path = Path::new(path_str);
                return Some((node, src, tree, sym.file_id, path, ops));
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
    ) -> Option<super::TaintRegistry> {
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
                let mut bindings = Vec::new();
                extract_parameter_bindings(p_node, def_source, &mut bindings);
                for name in bindings {
                    registry.taint(name, origin.clone());
                }
            }
            p_idx += 1;
        }
        Some(registry)
    }
}

fn extract_parameter_bindings<'a>(node: Node<'a>, source: &'a str, bindings: &mut Vec<&'a str>) {
    match node.kind() {
        "identifier"
        | "shorthand_field_identifier"
        | "shorthand_property_identifier"
        | "shorthand_property_identifier_pattern"
        | "variable_declarator" => {
            let name = &source[node.start_byte()..node.end_byte()];
            bindings.push(name);
        }
        "tuple_pattern"
        | "array_pattern"
        | "object_pattern"
        | "struct_pattern"
        | "tuple_struct_pattern"
        | "pair"
        | "property" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if (child.kind() == "pair" || child.kind() == "property")
                    && let Some(val_node) = child.child_by_field_name("value")
                {
                    extract_parameter_bindings(val_node, source, bindings);
                } else if child.kind() != ":"
                    && child.kind() != ","
                    && child.kind() != "{"
                    && child.kind() != "}"
                {
                    extract_parameter_bindings(child, source, bindings);
                }
            }
        }
        _ => {
            if node.child_count() == 0
                && (node.kind().contains("identifier")
                    || node.kind().contains("pattern")
                    || node.kind() == "variable_declarator")
                && node.kind() != "type_identifier"
            {
                let name = &source[node.start_byte()..node.end_byte()];
                bindings.push(name);
            } else {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() != "("
                        && child.kind() != ")"
                        && child.kind() != ","
                        && child.kind() != ":"
                        && child.kind() != "type_identifier"
                    {
                        extract_parameter_bindings(child, source, bindings);
                    }
                }
            }
        }
    }
}
