// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, CoreRule, GenSenseContext, GenSenseRule};
use regex::Regex;
use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Debug, Clone, Default)]
pub struct TaintRegistry {
    pub tainted_vars: HashMap<String, String>, // var_name -> source_expr
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
        rule: &CoreRule,
        initial_taint: TaintRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut registry = initial_taint;

        // Perform a linear traversal of the body to track flow
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

    #[allow(clippy::too_many_arguments)]
    fn traverse_for_taint(
        &self,
        node: Node,
        cursor: &mut tree_sitter::TreeCursor,
        source_re: &Regex,
        sink_re: &Regex,
        registry: &mut TaintRegistry,
        advisories: &mut Vec<Advisory>,
        rule: &CoreRule,
    ) {
        let kind = node.kind();
        let _code = &self.context.source_code[node.start_byte()..node.end_byte()];

        // 1. Assignment Logic (let x = ... or x = ...)
        if kind == "variable_declarator" || kind == "assignment_expression" {
            let mut name = String::new();
            let mut value_code = String::new();

            // Extract name and value based on language structure
            if kind == "variable_declarator" {
                let name_node = node.child_by_field_name("name");
                let value_node = node.child_by_field_name("value");

                if let (Some(n), Some(v)) = (name_node, value_node) {
                    let val_code =
                        self.context.source_code[v.start_byte()..v.end_byte()].to_string();
                    let is_source_val = source_re.is_match(&val_code)
                        || registry.tainted_vars.contains_key(&val_code);

                    if n.kind() == "object_pattern" {
                        let mut pattern_cursor = n.walk();
                        for part in n.children(&mut pattern_cursor) {
                            let part_kind = part.kind();
                            // Support both JS and TS pattern kinds
                            if part_kind == "shorthand_property_identifier"
                                || part_kind == "shorthand_property_identifier_pattern"
                                || part_kind == "identifier"
                                || part_kind == "pair"
                                || part_kind == "pair_pattern"
                            {
                                let part_code = self.context.source_code
                                    [part.start_byte()..part.end_byte()]
                                    .to_string();

                                // Professional Logic: Taint if property name is sensitive OR if the RHS is a known source
                                if source_re.is_match(&part_code) || is_source_val {
                                    let origin_to_record = if source_re.is_match(&part_code) {
                                        part_code.clone()
                                    } else {
                                        registry
                                            .tainted_vars
                                            .get(&val_code)
                                            .cloned()
                                            .unwrap_or(val_code.clone())
                                    };

                                    if part_kind == "pair" || part_kind == "pair_pattern" {
                                        if let Some(alias) = part.child_by_field_name("value") {
                                            let alias_name = self.context.source_code
                                                [alias.start_byte()..alias.end_byte()]
                                                .to_string();
                                            registry
                                                .tainted_vars
                                                .insert(alias_name, origin_to_record);
                                        }
                                    } else {
                                        registry.tainted_vars.insert(part_code, origin_to_record);
                                    }
                                }
                            }
                        }
                    } else {
                        let name =
                            self.context.source_code[n.start_byte()..n.end_byte()].to_string();
                        if is_source_val || source_re.is_match(&name) {
                            let origin_to_record = if source_re.is_match(&name) {
                                name.clone()
                            } else {
                                registry
                                    .tainted_vars
                                    .get(&val_code)
                                    .cloned()
                                    .unwrap_or(val_code.clone())
                            };
                            registry.tainted_vars.insert(name, origin_to_record);
                        }
                    }
                }
            } else if kind == "assignment_expression" {
                if let Some(left) = node.child_by_field_name("left") {
                    name = self.context.source_code[left.start_byte()..left.end_byte()].to_string();
                }
                if let Some(right) = node.child_by_field_name("right") {
                    value_code =
                        self.context.source_code[right.start_byte()..right.end_byte()].to_string();
                }
            }

            if !name.is_empty() && !value_code.is_empty() {
                // If value is a SOURCE, mark name as TAINTED
                if source_re.is_match(&value_code) {
                    registry
                        .tainted_vars
                        .insert(name.clone(), value_code.clone());
                }
                // If value is a TAINTED variable, spread the taint
                else if let Some(source_origin) = registry.tainted_vars.get(&value_code).cloned()
                {
                    registry.tainted_vars.insert(name.clone(), source_origin);
                }
            }
        }

        // 2. Sink Logic (console.log(x))
        if kind == "call_expression" {
            if let Some(fn_node) = node.child_by_field_name("function") {
                let fn_name =
                    self.context.source_code[fn_node.start_byte()..fn_node.end_byte()].to_string();

                // Get arguments once
                let mut tainted_args = Vec::new();
                if let Some(args) = node.child_by_field_name("arguments") {
                    let mut arg_cursor = args.walk();
                    let mut index = 0;
                    for arg in args.children(&mut arg_cursor) {
                        let arg_code =
                            self.context.source_code[arg.start_byte()..arg.end_byte()].to_string();
                        if let Some(origin) = registry.tainted_vars.get(&arg_code) {
                            tainted_args.push((index, arg_code.clone(), origin.clone()));
                        }
                        if arg.kind() != "(" && arg.kind() != ")" && arg.kind() != "," {
                            index += 1;
                        }
                    }
                }

                // A. Check if it's a known SINK
                if sink_re.is_match(&fn_name) {
                    for (_idx, arg_code, origin) in &tainted_args {
                        advisories.push(rule.new_advisory(
                            &node,
                            format!("Inter-procedural Leak: Tainted data from '{origin}' reached sink '{fn_name}' via variable '{arg_code}'."),
                            rule.impact.clone(),
                            rule.improvement.clone(),
                        ));
                    }
                }
                // B. Check if it's a known Function to "Jump" into (Inter-procedural)
                else if !tainted_args.is_empty() && self.depth < MAX_TAINT_DEPTH {
                    if let Some(_symbols) = self.context.symbols.symbols.get(&fn_name) {
                        if let Some(def_node) = self.find_definition_in_current_scope(&fn_name) {
                            if let Some(params) = def_node.child_by_field_name("parameters") {
                                let mut next_registry = TaintRegistry::default();
                                let mut p_cursor = params.walk();
                                let mut p_idx = 0;
                                for p in params.children(&mut p_cursor) {
                                    let p_kind = p.kind();
                                    if p_kind == "identifier"
                                        || p_kind == "parameter"
                                        || p_kind == "required_parameter"
                                    {
                                        for (arg_idx, _arg_code, origin) in &tainted_args {
                                            if *arg_idx == p_idx {
                                                let p_name = self.context.source_code
                                                    [p.start_byte()..p.end_byte()]
                                                    .to_string();
                                                next_registry
                                                    .tainted_vars
                                                    .insert(p_name, origin.clone());
                                            }
                                        }
                                        p_idx += 1;
                                    }
                                }

                                if !next_registry.tainted_vars.is_empty() {
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

                // C. Check if the function itself is tainted (Function Aliasing)
                if sink_re.as_str() == "<EXECUTE>" {
                    if let Some(origin) = registry.tainted_vars.get(&fn_name) {
                        if source_re.is_match(origin) {
                            advisories.push(rule.new_advisory(
                                &node,
                                format!(
                                    "Dynamic Execution Leak: Aliased execution of '{origin}' via '{fn_name}'."
                                ),
                                rule.impact.clone(),
                                rule.improvement.clone(),
                            ));
                        }
                    }
                }
            }
        }

        // Recurse children
        if cursor.goto_first_child() {
            loop {
                self.traverse_for_taint(
                    cursor.node(),
                    cursor,
                    source_re,
                    sink_re,
                    registry,
                    advisories,
                    rule,
                );
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    fn find_definition_in_current_scope(&self, name: &str) -> Option<Node<'a>> {
        // Use the SymbolRegistry to find the location first
        if let Some(symbols) = self.context.symbols.symbols.get(name) {
            // Find a symbol in the current file
            for sym in symbols {
                if sym.file_path == self.context.file_path.to_string_lossy() {
                    // Find the node at this position
                    let mut cursor = self.root.walk();
                    return self.find_node_at_pos(
                        self.root,
                        &mut cursor,
                        sym.line - 1,
                        sym.column - 1,
                    );
                }
            }
        }

        // Fallback to manual search if not indexed (unlikely but safe)
        let mut cursor = self.root.walk();
        self.search_node(self.root, &mut cursor, name)
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
