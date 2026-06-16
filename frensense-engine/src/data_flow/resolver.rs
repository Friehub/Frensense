// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;
use tree_sitter::Tree;

use crate::data_flow::{TaintOrigin, TaintRegistry};

#[derive(Debug, Clone)]
pub struct ResolvedFunction {
    pub file_path: String,
    pub source: String,
    pub byte_range: (usize, usize),
    pub body_range: Option<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub file_path: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub line: usize,
    pub end_line: usize,
    pub file_id: u32,
}

pub fn resolve_fn_definition(
    fn_name: &str,
    caller_file: &str,
    caller_line: usize,
    local_registry: &TaintRegistry,
    current_root: tree_sitter::Node,
    current_source: &str,
    all_symbols: &[SymbolEntry],
    file_trees: &HashMap<String, (&str, &Tree)>,
) -> Option<ResolvedFunction> {
    let name = fn_name.split("::").last().unwrap_or(fn_name);

    if let Some((start_byte, end_byte)) = local_registry.find_symbol_range(name)
        && let Some(node) = current_root.descendant_for_byte_range(start_byte, end_byte)
    {
        return Some(ResolvedFunction {
            file_path: caller_file.to_string(),
            source: current_source.to_string(),
            byte_range: (node.start_byte(), node.end_byte()),
            body_range: node
                .child_by_field_name("body")
                .map(|b| (b.start_byte(), b.end_byte())),
        });
    }

    let fp = Path::new(caller_file).to_string_lossy().to_string();

    if let Some(sym) = find_symbol_at(all_symbols, name, &fp, caller_line) {
        if let Some(node) = current_root.descendant_for_byte_range(sym.start_byte, sym.end_byte) {
            return Some(ResolvedFunction {
                file_path: fp,
                source: current_source.to_string(),
                byte_range: (node.start_byte(), node.end_byte()),
                body_range: node
                    .child_by_field_name("body")
                    .map(|b| (b.start_byte(), b.end_byte())),
            });
        }
    }

    let all_matches: Vec<_> = all_symbols.iter().filter(|s| s.name == name).collect();

    if let Some(sym) = all_matches.iter().find(|s| s.file_path == fp) {
        if let Some(node) = current_root.descendant_for_byte_range(sym.start_byte, sym.end_byte) {
            return Some(ResolvedFunction {
                file_path: fp,
                source: current_source.to_string(),
                byte_range: (node.start_byte(), node.end_byte()),
                body_range: node
                    .child_by_field_name("body")
                    .map(|b| (b.start_byte(), b.end_byte())),
            });
        }
    }

    for sym in all_matches {
        if sym.file_path == fp {
            continue;
        }
        if let Some((_path_str, (src, tree))) = file_trees.get_key_value(&sym.file_path) {
            if let Some(node) = tree
                .root_node()
                .descendant_for_byte_range(sym.start_byte, sym.end_byte)
            {
                return Some(ResolvedFunction {
                    file_path: sym.file_path.clone(),
                    source: (*src).to_string(),
                    byte_range: (node.start_byte(), node.end_byte()),
                    body_range: node
                        .child_by_field_name("body")
                        .map(|b| (b.start_byte(), b.end_byte())),
                });
            }
        }
    }

    None
}

fn find_symbol_at<'a>(
    symbols: &'a [SymbolEntry],
    name: &str,
    file: &str,
    line: usize,
) -> Option<&'a SymbolEntry> {
    symbols
        .iter()
        .filter(|s| s.name == name && s.file_path == file && line >= s.line && line <= s.end_line)
        .min_by_key(|s| s.end_line - s.line)
}

pub fn map_call_args_to_params(
    def_node: tree_sitter::Node,
    def_source: &str,
    tainted_arg_indices: &[(usize, TaintOrigin)],
) -> Option<TaintRegistry> {
    let params_node = def_node.child_by_field_name("parameters")?;
    let mut registry = TaintRegistry::default();
    let mut cursor = params_node.walk();
    let mut p_idx = 0;

    for param in params_node.children(&mut cursor) {
        if matches!(param.kind(), "(" | ")" | ",") {
            continue;
        }
        if let Some((_, origin)) = tainted_arg_indices.iter().find(|(idx, _)| *idx == p_idx) {
            let p_node = param.child_by_field_name("pattern").unwrap_or(param);
            let mut bindings = Vec::new();
            extract_parameter_bindings(p_node, def_source, &mut bindings);
            for name in bindings {
                registry.taint(&name, origin.clone());
            }
        }
        p_idx += 1;
    }
    Some(registry)
}

fn extract_parameter_bindings(node: tree_sitter::Node, source: &str, bindings: &mut Vec<String>) {
    match node.kind() {
        "identifier"
        | "shorthand_field_identifier"
        | "shorthand_property_identifier"
        | "shorthand_property_identifier_pattern"
        | "variable_declarator" => {
            let name = source[node.start_byte()..node.end_byte()].to_string();
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
                let name = source[node.start_byte()..node.end_byte()].to_string();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_flow::TaintRegistry;

    fn make_symbol_entry(name: &str, file: &str, start: usize, end: usize) -> SymbolEntry {
        SymbolEntry {
            name: name.to_string(),
            file_path: file.to_string(),
            start_byte: start,
            end_byte: end,
            line: 1,
            end_line: 10,
            file_id: 0,
        }
    }

    #[test]
    fn test_map_args_to_params_simple() {
        let source = "fn foo(x: i32, y: String) { }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let fn_node = root.child(0).unwrap();

        let result =
            map_call_args_to_params(fn_node, source, &[(0, TaintOrigin::UserInput)]).unwrap();

        assert!(result.is_tainted("x"));
        assert!(!result.is_tainted("y"));
        assert_eq!(result.get_origin("x"), Some(TaintOrigin::UserInput));
    }

    #[test]
    fn test_resolve_local_registry() {
        let source = "fn outer() { let handler = |x| x; handler(); }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let mut registry = TaintRegistry::default();
        registry.register_symbol("handler", 27, 34);

        let all_symbols: Vec<SymbolEntry> = vec![];
        let file_trees: HashMap<String, (&str, &Tree)> = HashMap::new();

        let result = resolve_fn_definition(
            "handler",
            "test.rs",
            1,
            &registry,
            root,
            source,
            &all_symbols,
            &file_trees,
        );

        assert!(result.is_some());
        assert_eq!(result.unwrap().file_path, "test.rs");
    }

    #[test]
    fn test_resolve_symbol_by_name() {
        let source = "fn target() { } fn caller() { target(); }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let all_symbols = vec![make_symbol_entry("target", "test.rs", 0, 15)];
        let file_trees: HashMap<String, (&str, &Tree)> = HashMap::new();
        let registry = TaintRegistry::default();

        let result = resolve_fn_definition(
            "target",
            "test.rs",
            4,
            &registry,
            root,
            source,
            &all_symbols,
            &file_trees,
        );

        assert!(result.is_some());
    }

    #[test]
    fn test_resolve_cross_file() {
        let source_a = "fn helper() { }";
        let source_b = "fn main() { helper(); }";

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();

        let tree_a = parser.parse(source_a, None).unwrap();
        let tree_b = parser.parse(source_b, None).unwrap();

        let mut file_trees: HashMap<String, (&str, &Tree)> = HashMap::new();
        file_trees.insert("a.rs".to_string(), (source_a, &tree_a));

        let all_symbols = vec![make_symbol_entry("helper", "a.rs", 0, 15)];
        let registry = TaintRegistry::default();

        let result = resolve_fn_definition(
            "helper",
            "b.rs",
            1,
            &registry,
            tree_b.root_node(),
            source_b,
            &all_symbols,
            &file_trees,
        );

        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.file_path, "a.rs");
        assert_eq!(r.source, source_a);
    }
}
