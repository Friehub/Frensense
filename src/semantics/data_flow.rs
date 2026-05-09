// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, Symbol};
use regex::Regex;
use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Debug, Clone, Default)]
pub struct TaintRegistry {
    pub tainted_vars: HashMap<String, String>, // var_name -> source_expr
}

impl TaintRegistry {
    pub fn get_origin(&self, name: &str) -> String {
        self.tainted_vars
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string())
    }
}

pub struct DataFlowAnalyzer<'a> {
    pub context: &'a GenSenseContext<'a>,
    pub root: Node<'a>,
    pub depth: usize,
}

const MAX_TAINT_DEPTH: usize = 3;

impl<'a> DataFlowAnalyzer<'a> {
    pub fn new(context: &'a GenSenseContext<'a>, root: Node<'a>) -> Self {
        Self {
            context,
            root,
            depth: 0,
        }
    }

    pub fn with_depth(context: &'a GenSenseContext<'a>, root: Node<'a>, depth: usize) -> Self {
        Self {
            context,
            root,
            depth,
        }
    }

    pub fn analyze_block(
        &self,
        node: Node,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        initial_taint: TaintRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut registry = initial_taint;

        let mut cursor = node.walk();
        self.traverse_for_taint(
            node,
            &mut cursor,
            source_re,
            sink_re,
            &mut registry,
            &mut advisories,
            rule,
        );

        advisories
    }

    fn traverse_for_taint(
        &self,
        node: Node,
        cursor: &mut tree_sitter::TreeCursor,
        source_re: &Regex,
        sink_re: &Regex,
        registry: &mut TaintRegistry,
        advisories: &mut Vec<Advisory>,
        rule: &dyn GenSenseRule,
    ) {
        let mut node = node;
        loop {
            let kind = node.kind();

            match kind {
                "variable_declarator" | "assignment_expression" => {
                    self.handle_assignment(node, source_re, registry);
                }
                "call_expression" => {
                    self.handle_call(node, source_re, sink_re, registry, advisories, rule);
                }
                _ => {
                    // Recurse into children if they exist
                    if cursor.goto_first_child() {
                        self.traverse_for_taint(
                            cursor.node(),
                            cursor,
                            source_re,
                            sink_re,
                            registry,
                            advisories,
                            rule,
                        );
                    }
                }
            }

            if !cursor.goto_next_sibling() {
                break;
            }
            node = cursor.node();
        }
        cursor.goto_parent();
    }
    fn node_src(&self, node: Node) -> String {
        self.context.source_code[node.start_byte()..node.end_byte()].to_string()
    }

    fn handle_assignment(&self, node: Node, source_re: &Regex, registry: &mut TaintRegistry) {
        let kind = node.kind();
        let (name_node, value_node) = if kind == "variable_declarator" {
            (
                node.child_by_field_name("name"),
                node.child_by_field_name("value"),
            )
        } else {
            (
                node.child_by_field_name("left"),
                node.child_by_field_name("right"),
            )
        };

        if let (Some(n), Some(v)) = (name_node, value_node) {
            let val_code = self.node_src(v);
            let is_source_val =
                source_re.is_match(&val_code) || registry.tainted_vars.contains_key(&val_code);

            if n.kind() == "object_pattern" {
                let mut p_cursor = n.walk();
                for part in n.children(&mut p_cursor) {
                    if let Some(p_name) = self.extract_ident(part) {
                        if source_re.is_match(&p_name) || is_source_val {
                            let origin = if source_re.is_match(&p_name) {
                                p_name.clone()
                            } else {
                                registry.get_origin(&val_code)
                            };
                            registry.tainted_vars.insert(p_name, origin);
                        }
                    }
                }
            } else {
                let name = self.node_src(n);
                if is_source_val || source_re.is_match(&name) {
                    let origin = if source_re.is_match(&name) {
                        name.clone()
                    } else {
                        registry.get_origin(&val_code)
                    };
                    registry.tainted_vars.insert(name, origin);
                }
            }
        }
    }

    fn handle_call(
        &self,
        node: Node,
        source_re: &Regex,
        sink_re: &Regex,
        registry: &mut TaintRegistry,
        advisories: &mut Vec<Advisory>,
        rule: &dyn GenSenseRule,
    ) {
        if let Some(fn_node) = node.child_by_field_name("function") {
            let fn_name = self.node_src(fn_node);
            let tainted_args = self.get_tainted_args(node, registry);

            if sink_re.is_match(&fn_name) {
                for (_idx, arg_code, origin) in &tainted_args {
                    advisories.push(rule.new_advisory(
                        &node,
                        format!("Inter-procedural Leak: Tainted data from '{origin}' reached sink '{fn_name}' via variable '{arg_code}'."),
                        rule.impact().to_string(),
                        rule.improvement().to_string(),
                    ));
                }
            } else if !tainted_args.is_empty() && self.depth < MAX_TAINT_DEPTH {
                if let Some(def_node) = self.find_definition(&fn_name) {
                    if let Some(next_registry) = self.map_params(def_node, &tainted_args) {
                        if let Some(body) = def_node.child_by_field_name("body") {
                            let sub_analyzer = DataFlowAnalyzer::with_depth(
                                self.context,
                                self.root,
                                self.depth + 1,
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
        }
    }

    fn extract_ident(&self, node: Node) -> Option<String> {
        let kind = node.kind();
        if kind == "identifier"
            || kind == "shorthand_property_identifier"
            || kind == "shorthand_property_identifier_pattern"
        {
            Some(self.node_src(node))
        } else if kind == "pair" || kind == "pair_pattern" {
            node.child_by_field_name("value").map(|v| self.node_src(v))
        } else {
            None
        }
    }

    fn get_tainted_args(
        &self,
        node: Node,
        registry: &TaintRegistry,
    ) -> Vec<(usize, String, String)> {
        let mut tainted = Vec::new();
        if let Some(args) = node.child_by_field_name("arguments") {
            let mut cursor = args.walk();
            let mut index = 0;
            for arg in args.children(&mut cursor) {
                if matches!(arg.kind(), "(" | ")" | "," | "[") {
                    continue;
                }
                let code = self.node_src(arg);
                if let Some(origin) = registry.tainted_vars.get(&code) {
                    tainted.push((index, code, origin.clone()));
                }
                index += 1;
            }
        }
        tainted
    }

    fn find_definition(&self, name: &str) -> Option<Node<'a>> {
        if let Some(node) = self.find_definition_in_current_scope(name) {
            return Some(node);
        }
        let candidates = self.context.symbols.find(name);
        for sym in candidates {
            if sym.file_path == self.context.file_path.to_string_lossy() {
                let mut cursor = self.root.walk();
                return self.find_node_at_pos(self.root, &mut cursor, sym.line - 1, sym.column - 1);
            }
        }
        None
    }

    fn map_params(
        &self,
        def_node: Node,
        tainted_args: &[(usize, String, String)],
    ) -> Option<TaintRegistry> {
        let params = def_node.child_by_field_name("parameters")?;
        let mut registry = TaintRegistry::default();
        let mut cursor = params.walk();
        let mut p_idx = 0;

        for p in params.children(&mut cursor) {
            if matches!(p.kind(), "(" | ")" | "," | "[") {
                continue;
            }
            if let Some(p_name) = self.extract_ident(p) {
                for (arg_idx, _, origin) in tainted_args {
                    if *arg_idx == p_idx {
                        registry.tainted_vars.insert(p_name.clone(), origin.clone());
                    }
                }
                p_idx += 1;
            }
        }
        if registry.tainted_vars.is_empty() {
            None
        } else {
            Some(registry)
        }
    }

    fn find_definition_in_current_scope(&self, name: &str) -> Option<Node<'a>> {
        let candidates = self.context.symbols.find(name);
        for sym in candidates {
            if sym.file_path == self.context.file_path.to_string_lossy() {
                let mut cursor = self.root.walk();
                return self.find_node_at_pos(self.root, &mut cursor, sym.line - 1, sym.column - 1);
            }
        }

        // Fallback to manual search if not indexed (unlikely but safe)
        let mut cursor = self.root.walk();
        self.search_node(self.root, &mut cursor, name)
    }

    pub fn resolve_symbol_in_graph(&self, name: &str) -> Option<&Symbol> {
        let candidates = self.context.symbols.find(name);
        if !candidates.is_empty() {
            // Prefer current file, then any
            candidates
                .iter()
                .find(|s| s.file_path == self.context.file_path.to_string_lossy())
                .or_else(|| candidates.first())
                .copied()
        } else {
            None
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn find_node_at_pos<'b>(
        &self,
        node: Node<'b>,
        cursor: &mut tree_sitter::TreeCursor<'b>,
        row: usize,
        _col: usize,
    ) -> Option<Node<'b>> {
        let start = node.start_position();
        let end = node.end_position();

        if row < start.row || row > end.row {
            return None;
        }

        let kind = node.kind();
        if (kind == "function_declaration"
            || kind == "method_definition"
            || kind == "function_item")
            && start.row == row
        {
            return Some(node);
        }

        if cursor.goto_first_child() {
            loop {
                if let Some(found) = self.find_node_at_pos(cursor.node(), cursor, row, _col) {
                    return Some(found);
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
        None
    }

    fn search_node<'b>(
        &self,
        node: Node<'b>,
        cursor: &mut tree_sitter::TreeCursor<'b>,
        name: &str,
    ) -> Option<Node<'b>> {
        let kind = node.kind();
        if kind == "function_declaration" || kind == "method_definition" || kind == "function_item"
        {
            if let Some(n) = node.child_by_field_name("name") {
                let node_name = &self.context.source_code[n.start_byte()..n.end_byte()];
                if node_name == name {
                    return Some(node);
                }
            }
        }

        if cursor.goto_first_child() {
            loop {
                if let Some(found) = self.search_node(cursor.node(), cursor, name) {
                    return Some(found);
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
        None
    }
}
