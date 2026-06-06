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
        registry: &super::TaintRegistry,
    ) -> Option<(
        Node<'a>,
        &'a str,
        &'a tree_sitter::Tree,
        crate::FileId,
        &'a Path,
        &'a [crate::semantics::data_flow::normalization::SemanticOp],
    )> {
        let name = full_name.split("::").last().unwrap_or(full_name);

        let engine_file_trees: std::collections::HashMap<String, (&str, &tree_sitter::Tree)> = self
            .context
            .file_trees
            .iter()
            .map(|(k, (t, s, _))| (k.clone(), (s.as_str(), t)))
            .collect();

        let all_sym_entries: Vec<gensense_engine::data_flow::SymbolEntry> = self
            .context
            .symbols
            .query_all()
            .into_iter()
            .map(|s| gensense_engine::data_flow::SymbolEntry {
                name: s.name.clone(),
                file_path: s.file_path.clone(),
                start_byte: s.start_byte,
                end_byte: s.end_byte,
                line: s.line,
                end_line: s.end_line,
                file_id: s.file_id.0,
            })
            .collect();

        let resolved = gensense_engine::data_flow::resolve_fn_definition(
            name,
            &self.current_file_path.to_string_lossy(),
            self.root.start_position().row + 1,
            registry,
            self.current_tree.root_node(),
            self.current_source,
            &all_sym_entries,
            &engine_file_trees,
        )?;

        let is_current_file =
            resolved.file_path == self.current_file_path.to_string_lossy().as_ref();

        let (tree_ref, src_ref, ops_ref, path_key): (
            &tree_sitter::Tree,
            &str,
            &[crate::semantics::data_flow::normalization::SemanticOp],
            &str,
        ) = if is_current_file {
            (
                self.current_tree,
                self.current_source,
                self.context.semantic_ops,
                self.current_file_path.to_str()?,
            )
        } else {
            let (path, (tree, src, ops)) = self.context.file_trees.get_key_value(&resolved.file_path)?;
            (tree, src.as_str(), ops.as_slice(), path.as_str())
        };

        let resolved_path = Path::new(path_key);

        let node = tree_ref
            .root_node()
            .descendant_for_byte_range(resolved.byte_range.0, resolved.byte_range.1)?;

        let file_id = self
            .context
            .symbols
            .find_at(name, &resolved.file_path, 0)
            .map_or(self.context.file_id, |s| s.file_id);

        Some((node, src_ref, tree_ref, file_id, resolved_path, ops_ref))
    }

    #[must_use]
    pub fn map_params(
        &self,
        def_node: Node<'a>,
        def_source: &'a str,
        tainted_args: &[(usize, super::TaintOrigin)],
    ) -> Option<super::TaintRegistry> {
        gensense_engine::data_flow::map_call_args_to_params(def_node, def_source, tainted_args)
    }
}
