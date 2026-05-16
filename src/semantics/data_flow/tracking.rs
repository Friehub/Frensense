// SPDX-License-Identifier: MIT

use super::normalization::SemanticOp;
use super::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a, '_> {
    pub fn discover_symbols(&self, registry: &mut TaintRegistry<'a>) {
        for op in self.context.semantic_ops {
            if let SemanticOp::Binding { name, value_range } = op {
                // Register symbol with its range
                let node = self.node_at(*value_range);
                registry.register_symbol(name, node);
            }
        }
    }

    fn node_at(&self, range: crate::semantics::data_flow::normalization::Range) -> Node<'a> {
        self.current_tree
            .root_node()
            .descendant_for_byte_range(range.start_byte, range.end_byte)
            .unwrap_or_else(|| self.current_tree.root_node())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn analyze_block(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        mut registry: TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let block_range = super::normalization::Range::from(node);

        for op in self.context.semantic_ops {
            // Only process ops within the current block's range
            match op {
                SemanticOp::Binding { name, value_range } => {
                    if value_range.start_byte >= block_range.start_byte
                        && value_range.end_byte <= block_range.end_byte
                    {
                        let v_node = self.node_at(*value_range);
                        registry.register_symbol(name, v_node);
                        let val_code = &self.current_source[v_node.start_byte()..v_node.end_byte()];

                        let origin = if source_re.is_match(name) || source_re.is_match(val_code) {
                            Some(super::TaintOrigin::UserInput)
                        } else {
                            self.resolve_taint(v_node, &registry)
                        };

                        if let Some(o) = origin {
                            if let Some((obj, prop)) = name.split_once('.') {
                                registry.taint_field(obj, prop, o);
                            } else {
                                registry.taint(name, o);
                            }
                        }

                        self.propagate_object_taint(name, v_node, &mut registry);
                    }
                }
                SemanticOp::Assignment {
                    target,
                    value_range,
                } => {
                    if value_range.start_byte >= block_range.start_byte
                        && value_range.end_byte <= block_range.end_byte
                    {
                        let v_node = self.node_at(*value_range);
                        let val_code = &self.current_source[v_node.start_byte()..v_node.end_byte()];

                        let origin = if source_re.is_match(target) || source_re.is_match(val_code) {
                            Some(super::TaintOrigin::UserInput)
                        } else {
                            self.resolve_taint(v_node, &registry)
                        };

                        if let Some(o) = origin {
                            if let Some((obj, prop)) = target.split_once('.') {
                                registry.taint_field(obj, prop, o);
                            } else {
                                registry.taint(target, o);
                            }
                        }

                        self.propagate_object_taint(target, v_node, &mut registry);
                    }
                }
                SemanticOp::Call {
                    function_name,
                    args,
                    range,
                } => {
                    if range.start_byte >= block_range.start_byte
                        && range.end_byte <= block_range.end_byte
                    {
                        let arg_nodes: Vec<Node> = args.iter().map(|r| self.node_at(*r)).collect();
                        let call_node = self.node_at(*range);
                        advisories.extend(self.analyze_call(
                            function_name,
                            &arg_nodes,
                            call_node,
                            source_re,
                            sink_re,
                            rule,
                            &mut registry,
                        ));
                    }
                }
                SemanticOp::EnterBlock(body_range) => {
                    if body_range.start_byte > block_range.start_byte
                        && body_range.end_byte < block_range.end_byte
                        && self.depth < self.max_depth
                    {
                        let body_node = self.node_at(*body_range);
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
                        );
                        advisories.extend(sub_analyzer.analyze_block(
                            body_node,
                            source_re,
                            sink_re,
                            rule,
                            registry.clone(),
                        ));
                        registry.pop_scope();
                    }
                }
            }
        }

        advisories
    }

    #[allow(clippy::too_many_arguments)]
    fn analyze_call(
        &self,
        fn_name: &'a str,
        args: &[Node<'a>],
        _call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut tainted_args = Vec::new();

        for (idx, arg) in args.iter().enumerate() {
            if source_re.is_match(fn_name) {
                let arg_name = &self.current_source[arg.start_byte()..arg.end_byte()];
                registry.taint(arg_name, super::TaintOrigin::UserInput);
            }
            if let Some(origin) = self.resolve_taint(*arg, registry) {
                tainted_args.push((idx, origin.clone()));
                if sink_re.is_match(fn_name) {
                    let arg_code = &self.current_source[arg.start_byte()..arg.end_byte()];
                    // Custom new_advisory that uses current file info
                    let mut advisory = rule.new_advisory(
                        arg,
                        self.context,
                        format!("Inter-procedural Leak: Tainted data from '{origin}' reached sink '{fn_name}' via variable '{arg_code}'."),
                    );
                    // Override file info for cross-file findings
                    advisory.file_id = self.current_file_id;
                    advisory.file_path = self.current_file_path.display().to_string();

                    // Differentiate confidence
                    let confidence = if self.depth > 0 { 0.80 } else { 0.90 };

                    advisories.push(rule.with_confidence(advisory, confidence));
                }
            }
        }

        if !tainted_args.is_empty() && self.depth < self.max_depth {
            if let Some((def_node, def_source, def_tree, def_id, def_path, def_ops)) =
                self.find_definition(fn_name, registry)
            {
                if let Some(next_registry) = self.map_params(def_node, def_source, &tainted_args) {
                    if let Some(body) = def_node.child_by_field_name("body") {
                        let new_context = crate::GenSenseContext {
                            file_id: def_id,
                            file_path: def_path,
                            source_code: def_source,
                            tree: def_tree,
                            symbols: self.context.symbols,
                            semantic_ops: def_ops,
                            taint_cache: self.context.taint_cache,
                            file_trees: self.context.file_trees,
                        };

                        let sub_analyzer = DataFlowAnalyzer::with_depth(
                            &new_context,
                            def_source,
                            def_tree,
                            def_path,
                            def_id,
                            body,
                            self.depth + 1,
                            self.max_depth,
                        );
                        advisories.extend(sub_analyzer.analyze_block(
                            body,
                            source_re,
                            sink_re,
                            rule,
                            next_registry,
                        ));
                    }
                }
            }
        }

        advisories
    }

    fn propagate_object_taint(
        &self,
        name: &'a str,
        v_node: Node<'a>,
        registry: &mut TaintRegistry<'a>,
    ) {
        let v_kind = v_node.kind();
        if v_kind == "object" || v_kind == "object_expression" || v_kind == "struct_expression" {
            let mut cursor = v_node.walk();
            for prop in v_node.children(&mut cursor) {
                if prop.kind() == "pair"
                    || prop.kind() == "shorthand_property_identifier"
                    || prop.kind() == "field_initializer"
                {
                    let key = prop
                        .child_by_field_name("key")
                        .or_else(|| prop.child_by_field_name("name"))
                        .or(Some(prop));
                    let val = prop.child_by_field_name("value");

                    if let (Some(k), Some(v)) = (key, val) {
                        let key_name = &self.current_source[k.start_byte()..k.end_byte()];
                        if let Some(prop_origin) = self.resolve_taint(v, registry) {
                            registry.taint_field(name, key_name, prop_origin);
                        }
                    }
                }
            }
        }
    }

    fn resolve_taint(
        &self,
        node: Node<'a>,
        registry: &TaintRegistry<'a>,
    ) -> Option<super::TaintOrigin> {
        match node.kind() {
            "identifier" => {
                let name = &self.current_source[node.start_byte()..node.end_byte()];
                registry.get_origin(name)
            }
            "member_expression" | "field_expression" => {
                // Handle user.password
                let object_node = node
                    .child_by_field_name("object")
                    .or_else(|| node.child(0))?;
                let property_node = node
                    .child_by_field_name("property")
                    .or_else(|| node.child_by_field_name("field"))
                    .or_else(|| node.child(2))?; // fallback for some tree-sitter grammars

                let obj_name =
                    &self.current_source[object_node.start_byte()..object_node.end_byte()];
                let prop_name =
                    &self.current_source[property_node.start_byte()..property_node.end_byte()];

                // Check specific field taint first
                if let Some(origin) = registry.get_field_origin(obj_name, prop_name) {
                    return Some(origin);
                }

                // Fallback: If the whole object is tainted, the field is tainted
                registry.get_origin(obj_name)
            }
            _ => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if let Some(origin) = self.resolve_taint(child, registry) {
                        return Some(origin);
                    }
                }
                None
            }
        }
    }
}
