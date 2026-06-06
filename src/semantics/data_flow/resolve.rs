// SPDX-License-Identifier: MIT

use super::TaintRegistry;
use super::{DataFlowAnalyzer, TaintOrigin};
use crate::{Advisory, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a, '_> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn analyze_call(
        &self,
        fn_name: &'a str,
        args: &[Node<'a>],
        call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut tainted_args = Vec::new();

        // Full call expression text (e.g., "console.log(payload)") for sink matching
        let call_text = &self.current_source[call_node.start_byte()..call_node.end_byte()];

        for (idx, arg) in args.iter().enumerate() {
            if source_re.is_match(fn_name) || source_re.is_match(call_text) {
                let arg_name = &self.current_source[arg.start_byte()..arg.end_byte()];
                registry.taint(arg_name, TaintOrigin::UserInput);
            }
            if let Some(origin) =
                self.resolve_taint(*arg, source_re, sink_re, rule, registry, &mut advisories)
            {
                tainted_args.push((idx, origin.clone()));
                if sink_re.is_match(fn_name) || sink_re.is_match(call_text) {
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
                    let confidence = if self.depth > 0 {
                        self.context.taint_confidence_interprocedural
                    } else {
                        self.context.taint_confidence_intraprocedural
                    };

                    advisories.push(rule.with_confidence(advisory, confidence));
                }
            }
        }

        if !tainted_args.is_empty()
            && self.depth < self.max_depth
            && let Some((def_node, def_source, def_tree, def_id, def_path, def_ops)) =
                self.find_definition(fn_name, registry)
            && let Some(mut next_registry) = self.map_params(def_node, def_source, &tainted_args)
            && let Some(body) = def_node.child_by_field_name("body")
        {
            let new_context = crate::GenSenseContext {
                file_id: def_id,
                file_path: def_path,
                source_code: def_source,
                tree: def_tree,
                symbols: self.context.symbols,
                graph: self.context.graph,
                semantic_ops: def_ops,
                taint_cache: self.context.taint_cache,
                file_trees: self.context.file_trees,
                taint_confidence_interprocedural: self.context.taint_confidence_interprocedural,
                taint_confidence_intraprocedural: self.context.taint_confidence_intraprocedural,
                default_taint_max_depth: self.context.default_taint_max_depth,
                ngram_window_size: self.context.ngram_window_size,
            };

            let sub_analyzer = if let Some(engine) = self.data_flow_engine {
                DataFlowAnalyzer::with_depth_and_engine(
                    &new_context,
                    def_source,
                    def_tree,
                    def_path,
                    def_id,
                    body,
                    self.depth + 1,
                    self.max_depth,
                    engine,
                )
            } else {
                DataFlowAnalyzer::with_depth(
                    &new_context,
                    def_source,
                    def_tree,
                    def_path,
                    def_id,
                    body,
                    self.depth + 1,
                    self.max_depth,
                )
            };
            advisories.extend(sub_analyzer.analyze_block(
                body,
                source_re,
                sink_re,
                rule,
                &mut next_registry,
            ));
        }

        advisories
    }

    pub(super) fn propagate_object_taint(
        &self,
        name: &'a str,
        v_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry,
        advisories: &mut Vec<Advisory>,
    ) {
        let v_kind = v_node.kind();
        if v_kind == "object" || v_kind == "object_expression" || v_kind == "struct_expression" {
            // First pass: resolve taint on spread_element children.
            // If any spread source is tainted, explicit properties become overwritable.
            let mut spread_origin: Option<TaintOrigin> = None;
            {
                let mut cursor = v_node.walk();
                for prop in v_node.children(&mut cursor) {
                    if prop.kind() == "spread_element" {
                        // The spread expression is the first named child (skipping `...` syntax).
                        if let Some(val) = prop.named_child(0)
                            && let Some(origin) = self
                                .resolve_taint(val, source_re, sink_re, rule, registry, advisories)
                        {
                            spread_origin = Some(origin);
                            break;
                        }
                    }
                }
            }

            // Second pass: process explicit properties.
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
                        if let Some(prop_origin) =
                            self.resolve_taint(v, source_re, sink_re, rule, registry, advisories)
                        {
                            registry.taint_field(name, key_name, prop_origin);
                        } else if let Some(ref origin) = spread_origin {
                            // Spread element may override this explicit property.
                            registry.taint_field(name, key_name, origin.clone());
                        }
                    }
                }
            }
        }
    }

    pub(super) fn resolve_taint(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &TaintRegistry,
        advisories: &mut Vec<Advisory>,
    ) -> Option<TaintOrigin> {
        let mut cursor = node.walk();
        let mut stack = vec![node];

        while let Some(current) = stack.pop() {
            match current.kind() {
                "identifier" => {
                    let name = &self.current_source[current.start_byte()..current.end_byte()];
                    if let Some(origin) = registry.get_origin(name) {
                        return Some(origin);
                    }
                    if let Some(origin) = registry.get_any_field_origin(name) {
                        return Some(origin);
                    }
                }
                "member_expression" | "field_expression" => {
                    if let Some(object_node) = current
                        .child_by_field_name("object")
                        .or_else(|| current.child(0))
                        && let Some(property_node) = current
                            .child_by_field_name("property")
                            .or_else(|| current.child_by_field_name("field"))
                            .or_else(|| current.child(2))
                    {
                        let obj_name =
                            &self.current_source[object_node.start_byte()..object_node.end_byte()];
                        let prop_name = &self.current_source
                            [property_node.start_byte()..property_node.end_byte()];

                        if let Some(origin) = registry.get_field_origin(obj_name, prop_name) {
                            return Some(origin);
                        }

                        if let Some(origin) = registry.get_origin(obj_name) {
                            return Some(origin);
                        }
                    }
                }
                "call_expression" | "call" => {
                    if let Some(origin) = self
                        .resolve_call_taint(current, source_re, sink_re, rule, registry, advisories)
                    {
                        return Some(origin);
                    }
                }
                _ => {
                    cursor.reset(current);
                    if cursor.goto_first_child() {
                        loop {
                            stack.push(cursor.node());
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                }
            }
        }
        None
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_call_taint(
        &self,
        call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &TaintRegistry,
        advisories: &mut Vec<Advisory>,
    ) -> Option<TaintOrigin> {
        // Method chain receiver check
        if let Some(callee_node) = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))
            && (callee_node.kind() == "member_expression"
                || callee_node.kind() == "field_expression")
            && let Some(receiver) = callee_node
                .child_by_field_name("object")
                .or_else(|| callee_node.child(0))
            && let Some(receiver_origin) =
                self.resolve_taint(receiver, source_re, sink_re, rule, registry, advisories)
        {
            return Some(receiver_origin);
        }

        // Get function name
        let callee = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))?;
        let fn_name = &self.current_source[callee.start_byte()..callee.end_byte()];

        if source_re.is_match(fn_name) {
            return Some(TaintOrigin::UserInput);
        }

        // Extract argument nodes
        let mut args = Vec::new();
        if let Some(args_list) = call_node.child_by_field_name("arguments") {
            let mut cursor = args_list.walk();
            for child in args_list.children(&mut cursor) {
                if !matches!(child.kind(), "(" | ")" | ",") {
                    args.push(child);
                }
            }
        } else {
            // Fallback: look for a child with kind containing "arguments" or walk siblings
            let mut cursor = call_node.walk();
            for child in call_node.children(&mut cursor) {
                if child.kind().contains("arguments") {
                    let mut c2 = child.walk();
                    for grandchild in child.children(&mut c2) {
                        if !matches!(grandchild.kind(), "(" | ")" | ",") {
                            args.push(grandchild);
                        }
                    }
                }
            }
        }

        // Resolve argument taints
        let mut tainted_args = Vec::new();
        for (idx, arg) in args.iter().enumerate() {
            if let Some(origin) =
                self.resolve_taint(*arg, source_re, sink_re, rule, registry, advisories)
            {
                tainted_args.push((idx, origin));
            }
        }

        // Check if there is a definition
        if self.depth < self.max_depth
            && let Some((def_node, def_source, def_tree, def_id, def_path, def_ops)) =
                self.find_definition(fn_name, registry)
        {
            let def_key = (
                def_path.to_string_lossy().to_string(),
                def_node.start_byte(),
            );
            let mut visited = self.visited.borrow_mut();
            if !visited.insert(def_key) {
                return None;
            }
            drop(visited);

            if let Some(mut next_registry) = self.map_params(def_node, def_source, &tainted_args)
                && let Some(body) = def_node.child_by_field_name("body")
            {
                let new_context = crate::GenSenseContext {
                    file_id: def_id,
                    file_path: def_path,
                    source_code: def_source,
                    tree: def_tree,
                    symbols: self.context.symbols,
                    graph: self.context.graph,
                    semantic_ops: def_ops,
                    taint_cache: self.context.taint_cache,
                    file_trees: self.context.file_trees,
                    taint_confidence_interprocedural: self.context.taint_confidence_interprocedural,
                    taint_confidence_intraprocedural: self.context.taint_confidence_intraprocedural,
                    default_taint_max_depth: self.context.default_taint_max_depth,
                    ngram_window_size: self.context.ngram_window_size,
                };

                let sub_analyzer = if let Some(engine) = self.data_flow_engine {
                    DataFlowAnalyzer::with_depth_and_engine(
                        &new_context,
                        def_source,
                        def_tree,
                        def_path,
                        def_id,
                        body,
                        self.depth + 1,
                        self.max_depth,
                        engine,
                    )
                } else {
                    DataFlowAnalyzer::with_depth(
                        &new_context,
                        def_source,
                        def_tree,
                        def_path,
                        def_id,
                        body,
                        self.depth + 1,
                        self.max_depth,
                    )
                };

                sub_analyzer.discover_symbols(&mut next_registry);

                let sub_advisories =
                    sub_analyzer.analyze_block(body, source_re, sink_re, rule, &mut next_registry);
                advisories.extend(sub_advisories);

                let return_nodes = get_callee_returns(body);
                for ret_node in return_nodes {
                    if let Some(ret_origin) = sub_analyzer.resolve_taint(
                        ret_node,
                        source_re,
                        sink_re,
                        rule,
                        &next_registry,
                        advisories,
                    ) {
                        return Some(ret_origin);
                    }
                }
            }
        }

        None
    }
}

fn find_returns<'a>(node: Node<'a>, returns: &mut Vec<Node<'a>>) {
    match node.kind() {
        "return_statement" | "return_expression" => {
            let val_node = node.child_by_field_name("value").or_else(|| {
                if node.child_count() > 1 {
                    let mut last_idx = node.child_count() - 1;
                    if node.child(last_idx).is_some_and(|c| c.kind() == ";") {
                        last_idx = last_idx.saturating_sub(1);
                    }
                    node.child(last_idx)
                } else {
                    None
                }
            });
            if let Some(v) = val_node {
                returns.push(v);
            }
        }
        "block" | "block_expression" | "compound_statement" | "statement_block" => {
            if (node.kind() == "block" || node.kind() == "block_expression")
                && let Some(last_child) = node.child(node.child_count().saturating_sub(2))
            {
                let kind = last_child.kind();
                if kind != ";" && !kind.contains("statement") && kind != "}" && kind != "{" {
                    returns.push(last_child);
                }
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                find_returns(child, returns);
            }
        }
        "if_expression" => {
            if let Some(consequence) = node.child_by_field_name("consequence") {
                returns.push(consequence);
            }
            if let Some(alternative) = node.child_by_field_name("alternative") {
                returns.push(alternative);
            }
        }
        "match_expression" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "match_arm"
                    && let Some(value) = child.child_by_field_name("value")
                {
                    returns.push(value);
                }
            }
        }
        "function_declaration" | "function_item" | "arrow_function" | "method_definition" => {}
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                find_returns(child, returns);
            }
        }
    }
}

fn get_callee_returns(body: Node<'_>) -> Vec<Node<'_>> {
    let mut returns = Vec::new();
    if body.kind() != "block"
        && body.kind() != "block_expression"
        && body.kind() != "statement_block"
    {
        returns.push(body);
    } else {
        find_returns(body, &mut returns);
    }
    returns
}
