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

    pub fn analyze_block(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let block_range = super::normalization::Range::from(node);

        for op in self.context.semantic_ops {
            match op {
                SemanticOp::Binding { name, value_range } => {
                    self.process_binding(
                        name,
                        *value_range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                        &mut advisories,
                    );
                }
                SemanticOp::Assignment {
                    target,
                    value_range,
                } => {
                    self.process_assignment(
                        target,
                        *value_range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                        &mut advisories,
                    );
                }
                SemanticOp::Call {
                    function_name,
                    args,
                    range,
                } => {
                    if let Some(call_advisories) = self.process_call(
                        function_name,
                        args,
                        *range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                    ) {
                        advisories.extend(call_advisories);
                    }
                }
                SemanticOp::EnterBlock(body_range) => {
                    if let Some(sub_advisories) = self.process_enter_block(
                        *body_range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                    ) {
                        advisories.extend(sub_advisories);
                    }
                }
            }
        }

        advisories
    }

    fn process_binding(
        &self,
        name: &'a str,
        value_range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
        advisories: &mut Vec<Advisory>,
    ) {
        if value_range.start_byte >= block_range.start_byte
            && value_range.end_byte <= block_range.end_byte
        {
            let v_node = self.node_at(value_range);
            registry.register_symbol(name, v_node);
            let val_code = &self.current_source[v_node.start_byte()..v_node.end_byte()];

            let origin = if source_re.is_match(name) || source_re.is_match(val_code) {
                Some(super::TaintOrigin::UserInput)
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

    fn process_assignment(
        &self,
        target: &'a str,
        value_range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
        advisories: &mut Vec<Advisory>,
    ) {
        if value_range.start_byte >= block_range.start_byte
            && value_range.end_byte <= block_range.end_byte
        {
            let v_node = self.node_at(value_range);
            let val_code = &self.current_source[v_node.start_byte()..v_node.end_byte()];

            let origin = if source_re.is_match(target) || source_re.is_match(val_code) {
                Some(super::TaintOrigin::UserInput)
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
    fn process_call(
        &self,
        function_name: &'a str,
        args: &[super::normalization::Range],
        range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
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
    fn process_enter_block(
        &self,
        body_range: super::normalization::Range,
        block_range: super::normalization::Range,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
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
            );
            let sub_advisories =
                sub_analyzer.analyze_block(body_node, source_re, sink_re, rule, registry);
            registry.pop_scope();
            Some(sub_advisories)
        } else {
            None
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn analyze_call(
        &self,
        fn_name: &'a str,
        args: &[Node<'a>],
        call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut tainted_args = Vec::new();

        // Full call expression text (e.g., "console.log(payload)") for sink matching
        let call_text = &self.current_source[call_node.start_byte()..call_node.end_byte()];

        for (idx, arg) in args.iter().enumerate() {
            if source_re.is_match(fn_name) || source_re.is_match(call_text) {
                let arg_name = &self.current_source[arg.start_byte()..arg.end_byte()];
                registry.taint(arg_name, super::TaintOrigin::UserInput);
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
                    let confidence = if self.depth > 0 { 0.80 } else { 0.90 };

                    advisories.push(rule.with_confidence(advisory, confidence));
                }
            }
        }

        if !tainted_args.is_empty() && self.depth < self.max_depth {
            if let Some((def_node, def_source, def_tree, def_id, def_path, def_ops)) =
                self.find_definition(fn_name, registry)
            {
                if let Some(mut next_registry) =
                    self.map_params(def_node, def_source, &tainted_args)
                {
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
                            &mut next_registry,
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
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
        advisories: &mut Vec<Advisory>,
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
                        if let Some(prop_origin) =
                            self.resolve_taint(v, source_re, sink_re, rule, registry, advisories)
                        {
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
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &TaintRegistry<'a>,
        advisories: &mut Vec<Advisory>,
    ) -> Option<super::TaintOrigin> {
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
                    {
                        if let Some(property_node) = current
                            .child_by_field_name("property")
                            .or_else(|| current.child_by_field_name("field"))
                            .or_else(|| current.child(2))
                        {
                            let obj_name = &self.current_source
                                [object_node.start_byte()..object_node.end_byte()];
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
    fn resolve_call_taint(
        &self,
        call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &TaintRegistry<'a>,
        advisories: &mut Vec<Advisory>,
    ) -> Option<super::TaintOrigin> {
        // Method chain receiver check
        if let Some(callee_node) = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))
        {
            if callee_node.kind() == "member_expression" || callee_node.kind() == "field_expression"
            {
                if let Some(receiver) = callee_node
                    .child_by_field_name("object")
                    .or_else(|| callee_node.child(0))
                {
                    if let Some(receiver_origin) =
                        self.resolve_taint(receiver, source_re, sink_re, rule, registry, advisories)
                    {
                        return Some(receiver_origin);
                    }
                }
            }
        }

        // Get function name
        let callee = call_node
            .child_by_field_name("function")
            .or_else(|| call_node.child_by_field_name("callee"))
            .or_else(|| call_node.child(0))?;
        let fn_name = &self.current_source[callee.start_byte()..callee.end_byte()];

        if source_re.is_match(fn_name) {
            return Some(super::TaintOrigin::UserInput);
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
        if self.depth < self.max_depth {
            if let Some((def_node, def_source, def_tree, def_id, def_path, def_ops)) =
                self.find_definition(fn_name, registry)
            {
                if let Some(mut next_registry) =
                    self.map_params(def_node, def_source, &tainted_args)
                {
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

                        sub_analyzer.discover_symbols(&mut next_registry);

                        let sub_advisories = sub_analyzer.analyze_block(
                            body,
                            source_re,
                            sink_re,
                            rule,
                            &mut next_registry,
                        );
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
            if node.kind() == "block" || node.kind() == "block_expression" {
                if let Some(last_child) = node.child(node.child_count().saturating_sub(2)) {
                    let kind = last_child.kind();
                    if kind != ";" && !kind.contains("statement") && kind != "}" && kind != "{" {
                        returns.push(last_child);
                    }
                }
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                find_returns(child, returns);
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
