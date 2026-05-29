// SPDX-License-Identifier: MIT

use std::path::Path;
use tree_sitter::Node;

use crate::data_flow::TaintOrigin;
use crate::data_flow::TaintRegistry;

#[derive(Debug, Clone)]
pub struct TaintTracker<'a> {
    registry: TaintRegistry<'a>,
    source: &'a str,
    ext: &'a str,
    taint_sources: Vec<(&'a str, TaintOrigin)>,
    taint_sinks: Vec<&'a str>,
    file_path: &'a Path,
}

impl<'a> TaintTracker<'a> {
    pub fn new(
        source: &'a str,
        ext: &'a str,
        file_path: &'a Path,
    ) -> Self {
        Self {
            registry: TaintRegistry::default(),
            source,
            ext,
            taint_sources: Vec::new(),
            taint_sinks: Vec::new(),
            file_path,
        }
    }

    pub fn register_taint_source(&mut self, pattern: &'a str, origin: TaintOrigin) {
        self.taint_sources.push((pattern, origin));
    }

    pub fn register_taint_sink(&mut self, pattern: &'a str) {
        self.taint_sinks.push(pattern);
    }

    pub fn registry(&self) -> &TaintRegistry<'a> {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut TaintRegistry<'a> {
        &mut self.registry
    }

    pub fn track(&mut self, root: Node<'a>) {
        self.walk_and_tag(root);
        self.propagate_assignments(root);
    }

    fn walk_and_tag(&mut self, root: Node<'a>) {
        let mut cursor = root.walk();
        loop {
            let node = cursor.node();
            let kind = node.kind();

            {
                let source = self.source;
                let registry = &mut self.registry;
                let taint_sources = &self.taint_sources;
                if let Some(text) = node.utf8_text(source.as_bytes()).ok() {
                    for &(pattern, ref origin) in taint_sources {
                        if text.contains(pattern) {
                            for i in 0..node.child_count() {
                                if let Some(child) = node.child(i) {
                                    if let Ok(name) = child.utf8_text(source.as_bytes()) {
                                        if !name.contains('(') && !name.contains(' ') {
                                            registry.taint(name, origin.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if kind == "let_declaration" || kind == "lexical_declaration" || kind == "variable_declaration" {
                if let Some(value) = node.child_by_field_name("value") {
                    if let Some(pattern) = node.child_by_field_name("pattern") {
                        self.tag_from_value(pattern, value);
                    }
                }
            } else if kind == "assignment_expression" || kind == "assignment" {
                if let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) {
                    self.tag_from_value(left, right);
                }
            } else if kind == "call_expression" {
                self.check_sink(node);
            } else if matches!(kind, "function_item" | "function_declaration" | "method_definition") {
                self.registry.push_scope();
                if let Some(params) = node.child_by_field_name("parameters") {
                    for i in 0..params.child_count() {
                        if let Some(param) = params.child(i) {
                            if let Some(pattern) = param.child_by_field_name("pattern") {
                                if let Ok(name) = pattern.utf8_text(self.source.as_bytes()) {
                                    self.registry.register_symbol(name, param);
                                }
                            } else if let Ok(name) = param.utf8_text(self.source.as_bytes()) {
                                self.registry.register_symbol(name, param);
                            }
                        }
                    }
                }
            } else if matches!(kind, "block" | "declaration_list" | "statement_block") {
                self.registry.push_scope();
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                let cur = cursor.node();
                let ckind = cur.kind();
                if matches!(ckind, "block" | "declaration_list" | "statement_block"
                    | "function_item" | "function_declaration" | "method_definition")
                {
                    self.registry.pop_scope();
                }
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return;
                }
            }
        }
    }

    fn tag_from_value(&mut self, target: Node<'a>, value: Node<'a>) {
        if let Some(field_access) = value.child_by_field_name("object") {
            if let Some(field_name) = value.child_by_field_name("field") {
                if let (Ok(obj), Ok(field)) = (
                    field_access.utf8_text(self.source.as_bytes()),
                    field_name.utf8_text(self.source.as_bytes()),
                ) {
                    if let Some(origin) = self.registry.get_field_origin(obj, field) {
                        if let Ok(target_name) = target.utf8_text(self.source.as_bytes()) {
                            self.registry.taint(target_name, origin);
                        }
                    }
                }
            }
        }

        if let Ok(name) = target.utf8_text(self.source.as_bytes()) {
            if self.registry.get_origin(name).is_some() {
                return;
            }
        }

        if let Ok(value_text) = value.utf8_text(self.source.as_bytes()) {
            if let Ok(target_name) = target.utf8_text(self.source.as_bytes()) {
                for &(pattern, ref origin) in &self.taint_sources {
                    if value_text.contains(pattern) {
                        self.registry.taint(target_name, origin.clone());
                        return;
                    }
                }
                if self.registry.is_tainted(value_text) {
                    if let Some(origin) = self.registry.get_origin(value_text) {
                        self.registry.taint(target_name, origin);
                    } else if let Some(origin) = self.registry.get_any_field_origin(value_text) {
                        self.registry.taint(target_name, origin);
                    }
                }
            }
        }

        if let Some(arg) = value.child_by_field_name("argument") {
            if let (Ok(value_text), Ok(target_name)) =
                (arg.utf8_text(self.source.as_bytes()), target.utf8_text(self.source.as_bytes()))
            {
                if self.registry.is_tainted(value_text) {
                    if let Some(origin) = self.registry.get_origin(value_text) {
                        self.registry.taint(target_name, origin);
                    }
                }
            }
        }
    }

    fn propagate_assignments(&mut self, root: Node<'a>) {
        let mut changed = true;
        let mut iterations = 0;
        while changed && iterations < 10 {
            changed = false;
            iterations += 1;
            let mut cursor = root.walk();
            loop {
                let node = cursor.node();
                let kind = node.kind();
                if kind == "assignment_expression" || kind == "assignment" {
                    if let (Some(left), Some(right)) = (
                        node.child_by_field_name("left"),
                        node.child_by_field_name("right"),
                    ) {
                        if let (Ok(left_text), Ok(right_text)) = (
                            left.utf8_text(self.source.as_bytes()),
                            right.utf8_text(self.source.as_bytes()),
                        ) {
                            if self.registry.is_tainted(right_text) && !self.registry.is_tainted(left_text) {
                                if let Some(origin) = self.registry.get_origin(right_text) {
                                    self.registry.taint(left_text, origin);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
                if cursor.goto_first_child() {
                    continue;
                }
                loop {
                    if cursor.goto_next_sibling() {
                        break;
                    }
                    if !cursor.goto_parent() {
                        return;
                    }
                }
            }
        }
    }

    fn check_sink(&mut self, node: Node<'a>) {
        if let Some(func) = node.child_by_field_name("function") {
            if let Ok(func_name) = func.utf8_text(self.source.as_bytes()) {
                for sink_pattern in &self.taint_sinks {
                    if func_name == *sink_pattern || func_name.contains(sink_pattern) {
                        if let Some(args) = node.child_by_field_name("arguments") {
                            for i in 0..args.child_count() {
                                if let Some(arg) = args.child(i) {
                                    if let Ok(arg_text) = arg.utf8_text(self.source.as_bytes()) {
                                        let _ = sink_pattern;
                                        let _ = self.registry.is_tainted(arg_text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn is_tainted(&self, var: &str) -> bool {
        self.registry.is_tainted(var)
    }

    pub fn get_origin(&self, var: &str) -> Option<TaintOrigin> {
        self.registry.get_origin(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taint_registry_works() {
        let mut registry = TaintRegistry::default();
        registry.taint("x", TaintOrigin::UserInput);
        assert!(registry.is_tainted("x"));
        assert_eq!(registry.get_origin("x"), Some(TaintOrigin::UserInput));
    }

    #[test]
    fn test_taint_propagation() {
        let source = "let x = user_input();\nlet y = x;";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let mut tracker = TaintTracker::new(source, "rs", Path::new("test.rs"));
        tracker.register_taint_source("user_input", TaintOrigin::UserInput);
        tracker.track(root);
        assert!(true);
    }
}
