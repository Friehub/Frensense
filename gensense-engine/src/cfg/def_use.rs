// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use tree_sitter::Node;

use crate::cfg::{build_cfg, BasicBlock, ControlFlowGraph};

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub block_id: usize,
    pub node: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone)]
pub struct Use {
    pub name: String,
    pub block_id: usize,
    pub node: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, Default)]
pub struct DefUseChain {
    pub definitions: Vec<Definition>,
    pub uses: Vec<Use>,
    pub def_for_use: HashMap<usize, Vec<usize>>,
    pub use_for_def: HashMap<usize, Vec<usize>>,
    pub reaching_defs: HashMap<usize, HashSet<usize>>,
}

impl DefUseChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn defs_for(&self, name: &str) -> Vec<&Definition> {
        self.definitions.iter().filter(|d| d.name == name).collect()
    }

    pub fn uses_of(&self, name: &str) -> Vec<&Use> {
        self.uses.iter().filter(|u| u.name == name).collect()
    }

    pub fn uses_of_def(&self, def_index: usize) -> Vec<&Use> {
        self.use_for_def
            .get(&def_index)
            .map(|indices| indices.iter().filter_map(|&i| self.uses.get(i)).collect())
            .unwrap_or_default()
    }

    pub fn defs_reaching(&self, use_index: usize) -> Vec<&Definition> {
        self.def_for_use
            .get(&use_index)
            .map(|indices| indices.iter().filter_map(|&i| self.definitions.get(i)).collect())
            .unwrap_or_default()
    }
}

fn find_var_name(node: Node, source: &str) -> Option<String> {
    match node.kind() {
        "identifier" | "variable_declarator" => {
            let name = &source[node.start_byte()..node.end_byte()];
            if !name.is_empty() {
                Some(name.to_string())
            } else {
                None
            }
        }
        "shorthand_property_identifier_pattern" | "assignment_pattern" => {
            if let Some(key) = node.child_by_field_name("key") {
                find_var_name(key, source)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn scan_node_def_uses<'a>(
    node: Node<'a>,
    block_id: usize,
    source: &'a str,
    definitions: &mut Vec<Definition>,
    uses: &mut Vec<Use>,
    node_counter: &mut usize,
) {
    let kind = node.kind();
    match kind {
        "let_declaration" | "lexical_declaration" | "variable_declaration" => {
            if let Some(value) = node.child_by_field_name("value") {
                let value_text = source[value.start_byte()..value.end_byte()].to_string();
                if let Some(pattern) = node.child_by_field_name("pattern") {
                    if let Some(name) = find_var_name(pattern, source) {
                        definitions.push(Definition {
                            name,
                            block_id,
                            node: *node_counter,
                            start_byte: pattern.start_byte(),
                            end_byte: pattern.end_byte(),
                        });
                        *node_counter += 1;
                        uses.push(Use {
                            name: value_text,
                            block_id,
                            node: *node_counter,
                            start_byte: value.start_byte(),
                            end_byte: value.end_byte(),
                        });
                        *node_counter += 1;
                    }
                }
            } else if let Some(pattern) = node.child_by_field_name("pattern") {
                if let Some(name) = find_var_name(pattern, source) {
                    definitions.push(Definition {
                        name,
                        block_id,
                        node: *node_counter,
                        start_byte: pattern.start_byte(),
                        end_byte: pattern.end_byte(),
                    });
                    *node_counter += 1;
                }
            }
        }
        "assignment_expression" | "assignment" => {
            if let (Some(left), Some(right)) = (
                node.child_by_field_name("left"),
                node.child_by_field_name("right"),
            ) {
                let left_name = source[left.start_byte()..left.end_byte()].to_string();
                let right_name = source[right.start_byte()..right.end_byte()].to_string();
                definitions.push(Definition {
                    name: left_name,
                    block_id,
                    node: *node_counter,
                    start_byte: left.start_byte(),
                    end_byte: left.end_byte(),
                });
                *node_counter += 1;
                uses.push(Use {
                    name: right_name,
                    block_id,
                    node: *node_counter,
                    start_byte: right.start_byte(),
                    end_byte: right.end_byte(),
                });
                *node_counter += 1;
            }
        }
        "call_expression" => {
            if let Some(func) = node.child_by_field_name("function") {
                let func_name = source[func.start_byte()..func.end_byte()].to_string();
                uses.push(Use {
                    name: func_name,
                    block_id,
                    node: *node_counter,
                    start_byte: func.start_byte(),
                    end_byte: func.end_byte(),
                });
                *node_counter += 1;
            }
            if let Some(args) = node.child_by_field_name("arguments") {
                for i in 0..args.child_count() {
                    if let Some(arg) = args.child(i) {
                        let arg_name = source[arg.start_byte()..arg.end_byte()].to_string();
                        uses.push(Use {
                            name: arg_name,
                            block_id,
                            node: *node_counter,
                            start_byte: arg.start_byte(),
                            end_byte: arg.end_byte(),
                        });
                        *node_counter += 1;
                    }
                }
            }
        }
        "return_statement" | "return_expression" => {
            if let Some(value) = node.child_by_field_name("value") {
                let value_text = source[value.start_byte()..value.end_byte()].to_string();
                uses.push(Use {
                    name: value_text,
                    block_id,
                    node: *node_counter,
                    start_byte: value.start_byte(),
                    end_byte: value.end_byte(),
                });
                *node_counter += 1;
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    loop {
        if cursor.goto_first_child() {
            scan_node_def_uses(cursor.node(), block_id, source, definitions, uses, node_counter);
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                scan_node_def_uses(cursor.node(), block_id, source, definitions, uses, node_counter);
                break;
            }
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}

fn scan_block_def_uses<'a>(
    block: &BasicBlock<'a>,
    source: &'a str,
    definitions: &mut Vec<Definition>,
    uses: &mut Vec<Use>,
    node_counter: &mut usize,
) {
    for &node in &block.nodes {
        scan_node_def_uses(node, block.id, source, definitions, uses, node_counter);
    }
}

fn compute_reaching_defs(cfg: &ControlFlowGraph, chains: &mut DefUseChain) {
    let n = cfg.blocks.len();
    for i in 0..n {
        chains.reaching_defs.entry(i).or_insert_with(HashSet::new);
    }

    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..n {
            let mut incoming: HashSet<usize> = HashSet::new();
            for &pred in &cfg.blocks[i].predecessors {
                if let Some(rd) = chains.reaching_defs.get(&pred) {
                    incoming.extend(rd);
                }
            }

            let block_defs: HashSet<usize> = chains
                .definitions
                .iter()
                .enumerate()
                .filter(|(_, d)| d.block_id == i)
                .map(|(idx, _)| idx)
                .collect();

            let def_names: HashSet<&str> = block_defs
                .iter()
                .filter_map(|&idx| chains.definitions.get(idx))
                .map(|d| d.name.as_str())
                .collect();

            let mut new_rd: HashSet<usize> = incoming
                .into_iter()
                .filter(|&idx| {
                    chains
                        .definitions
                        .get(idx)
                        .map_or(true, |d| !def_names.contains(d.name.as_str()))
                })
                .collect();
            new_rd.extend(&block_defs);

            if let Some(existing) = chains.reaching_defs.get(&i) {
                if *existing != new_rd {
                    chains.reaching_defs.insert(i, new_rd);
                    changed = true;
                }
            } else {
                chains.reaching_defs.insert(i, new_rd);
                changed = true;
            }
        }
    }
}

pub fn compute_def_use<'a>(cfg: &ControlFlowGraph<'a>, source: &'a str) -> DefUseChain {
    let mut chains = DefUseChain::new();
    let mut node_counter = 0usize;

    for block in &cfg.blocks {
        scan_block_def_uses(block, source, &mut chains.definitions, &mut chains.uses, &mut node_counter);
    }

    compute_reaching_defs(cfg, &mut chains);

    for (use_idx, use_) in chains.uses.iter().enumerate() {
        let mut reaching_defs: Vec<usize> = Vec::new();
        if let Some(rd) = chains.reaching_defs.get(&use_.block_id) {
            for &def_idx in rd {
                if let Some(def) = chains.definitions.get(def_idx) {
                    if def.name == use_.name {
                        reaching_defs.push(def_idx);
                        chains.use_for_def.entry(def_idx).or_default().push(use_idx);
                    }
                }
            }
        }
        chains.def_for_use.insert(use_idx, reaching_defs);
    }

    chains
}

pub fn build_def_use<'a>(root: Node<'a>, source: &'a str, ext: &str) -> DefUseChain {
    let cfg = build_cfg(root, source, ext);
    compute_def_use(&cfg, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def_use_simple_let() {
        let source = "fn foo() { let x = 1; let y = x + 1; }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let chain = build_def_use(root, source, "rs");
        assert!(!chain.definitions.is_empty(), "should have definitions");
        assert!(!chain.uses.is_empty(), "should have uses");
    }
}
