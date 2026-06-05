// SPDX-License-Identifier: MIT

pub mod def_use;

use std::collections::{HashMap, HashSet};
use tree_sitter::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CFEdgeKind {
    Unconditional,
    Branch,
    Merge,
    BackEdge,
    Exception,
}

#[derive(Debug, Clone)]
pub struct BasicBlock<'a> {
    pub id: usize,
    pub start_byte: usize,
    pub end_byte: usize,
    pub kind: String,
    pub nodes: Vec<Node<'a>>,
    pub dominators: HashSet<usize>,
    pub successors: Vec<(usize, CFEdgeKind)>,
    pub predecessors: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct ControlFlowGraph<'a> {
    pub blocks: Vec<BasicBlock<'a>>,
    entry: usize,
    exit: usize,
    label_index: HashMap<String, usize>,
}

impl<'a> ControlFlowGraph<'a> {
    pub fn entry(&self) -> usize {
        self.entry
    }

    pub fn exit(&self) -> usize {
        self.exit
    }

    pub fn block(&self, id: usize) -> Option<&BasicBlock<'a>> {
        self.blocks.get(id)
    }

    pub fn successors(&self, id: usize) -> Vec<(usize, CFEdgeKind)> {
        self.blocks.get(id).map_or_else(Vec::new, |b| b.successors.clone())
    }

    pub fn predecessors(&self, id: usize) -> Vec<usize> {
        self.blocks.get(id).map_or_else(Vec::new, |b| b.predecessors.clone())
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn block_by_label(&self, label: &str) -> Option<usize> {
        self.label_index.get(label).copied()
    }

    pub fn is_reachable(&self, from: usize, to: usize) -> bool {
        if from >= self.blocks.len() || to >= self.blocks.len() {
            return false;
        }
        let mut visited = HashSet::new();
        let mut stack = vec![from];
        visited.insert(from);
        while let Some(block_id) = stack.pop() {
            if block_id == to {
                return true;
            }
            for &(succ, _) in &self.blocks[block_id].successors {
                if visited.insert(succ) {
                    stack.push(succ);
                }
            }
        }
        false
    }
}

#[allow(clippy::too_many_lines)]
pub fn build_cfg<'a>(root: Node<'a>, source: &'a str, _ext: &str) -> ControlFlowGraph<'a> {
    let mut blocks: Vec<BasicBlock<'a>> = Vec::new();
    let mut label_index: HashMap<String, usize> = HashMap::new();

    let entry = blocks.len();
    blocks.push(BasicBlock {
        id: entry,
        start_byte: root.start_byte(),
        end_byte: root.end_byte(),
        kind: "entry".to_string(),
        nodes: vec![root],
        dominators: HashSet::new(),
        successors: Vec::new(),
        predecessors: Vec::new(),
    });

    let mut cursor = root.walk();
    let mut parent_stack: Vec<usize> = Vec::new();
    let mut current_block = entry;

    loop {
        let node = cursor.node();
        let kind = node.kind();

        match kind {
            "if_statement" | "if_expression" | "ternary_expression" => {
                let branch_block = blocks.len();
                let merge_block = blocks.len() + 1;
                blocks.push(BasicBlock {
                    id: branch_block,
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    kind: "branch".to_string(),
                    nodes: vec![node],
                    dominators: HashSet::new(),
                    successors: vec![(merge_block, CFEdgeKind::Branch)],
                    predecessors: vec![current_block],
                });
                blocks[current_block].successors.push((branch_block, CFEdgeKind::Branch));
                blocks[current_block].successors.push((merge_block, CFEdgeKind::Branch));
                blocks.push(BasicBlock {
                    id: merge_block,
                    start_byte: node.end_byte(),
                    end_byte: node.end_byte(),
                    kind: "merge".to_string(),
                    nodes: Vec::new(),
                    dominators: HashSet::new(),
                    successors: Vec::new(),
                    predecessors: vec![branch_block, current_block],
                });
                blocks[branch_block].predecessors.push(current_block);
                blocks[current_block].successors.retain(|(id, _)| *id != branch_block || *id != merge_block);
                blocks[current_block].successors.push((branch_block, CFEdgeKind::Branch));
                blocks[current_block].successors.push((merge_block, CFEdgeKind::Branch));
                parent_stack.push(current_block);
                current_block = branch_block;
            }
            "loop_block" | "for_statement" | "while_statement" | "do_statement" => {
                let loop_body = blocks.len();
                let after_loop = blocks.len() + 1;
                blocks.push(BasicBlock {
                    id: loop_body,
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    kind: "loop_body".to_string(),
                    nodes: vec![node],
                    dominators: HashSet::new(),
                    successors: vec![(loop_body, CFEdgeKind::BackEdge), (after_loop, CFEdgeKind::Unconditional)],
                    predecessors: vec![current_block, loop_body],
                });
                blocks[current_block].successors.push((loop_body, CFEdgeKind::Unconditional));
                blocks.push(BasicBlock {
                    id: after_loop,
                    start_byte: node.end_byte(),
                    end_byte: node.end_byte(),
                    kind: "after_loop".to_string(),
                    nodes: Vec::new(),
                    dominators: HashSet::new(),
                    successors: Vec::new(),
                    predecessors: vec![loop_body],
                });
                parent_stack.push(current_block);
                current_block = loop_body;
            }
            "labeled_statement" => {
                if let Some(label) = node.child_by_field_name("label") {
                    let label_text = source[label.start_byte()..label.end_byte()].to_string();
                    label_index.insert(label_text, current_block);
                }
            }
            _ => {}
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                let exit = blocks.len();
                blocks.push(BasicBlock {
                    id: exit,
                    start_byte: root.end_byte(),
                    end_byte: root.end_byte(),
                    kind: "exit".to_string(),
                    nodes: Vec::new(),
                    dominators: HashSet::new(),
                    successors: Vec::new(),
                    predecessors: vec![current_block],
                });
                blocks[current_block].successors.push((exit, CFEdgeKind::Unconditional));
                return ControlFlowGraph {
                    blocks,
                    entry,
                    exit,
                    label_index,
                };
            }
            if let Some(parent_id) = parent_stack.pop() {
                current_block = parent_id;
            }
        }
    }
}

pub struct CFGWalkResult {
    pub reachable: HashSet<usize>,
    pub back_edges: Vec<(usize, usize)>,
}

pub fn walk_reachable(cfg: &ControlFlowGraph, from: usize) -> CFGWalkResult {
    let mut reachable = HashSet::new();
    let mut back_edges = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = vec![from];
    visited.insert(from);

    while let Some(block_id) = stack.pop() {
        reachable.insert(block_id);
        for &(succ, edge_kind) in &cfg.blocks[block_id].successors {
            if edge_kind == CFEdgeKind::BackEdge {
                back_edges.push((block_id, succ));
            }
            if visited.insert(succ) {
                stack.push(succ);
            }
        }
    }

    CFGWalkResult { reachable, back_edges }
}

pub fn compute_dominators(cfg: &mut ControlFlowGraph) {
    let n = cfg.blocks.len();
    if n == 0 {
        return;
    }

    for b in &mut cfg.blocks {
        b.dominators.clear();
    }
    cfg.blocks[cfg.entry].dominators.insert(cfg.entry);

    let all_blocks: HashSet<usize> = (0..n).collect();
    for i in 0..n {
        if i != cfg.entry {
            cfg.blocks[i].dominators.clone_from(&all_blocks);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for i in 1..n {
            let mut new_doms: HashSet<usize> = (0..n).collect();
            for pred in cfg.blocks[i].predecessors.clone() {
                new_doms = new_doms.intersection(&cfg.blocks[pred].dominators).copied().collect();
            }
            new_doms.insert(i);
            if new_doms != cfg.blocks[i].dominators {
                cfg.blocks[i].dominators = new_doms;
                changed = true;
            }
        }
    }
}

pub fn immediate_dominator(cfg: &ControlFlowGraph, block_id: usize) -> Option<usize> {
    if block_id == cfg.entry {
        return None;
    }
    let doms = &cfg.blocks[block_id].dominators;
    let mut idom = cfg.entry;
    for &d in doms {
        if d != block_id && cfg.blocks[d].dominators.contains(&idom) {
            idom = d;
        }
    }
    if idom == block_id {
        None
    } else {
        Some(idom)
    }
}

pub fn dominance_frontier(cfg: &ControlFlowGraph, block_id: usize) -> HashSet<usize> {
    let mut frontier = HashSet::new();
    let block = &cfg.blocks[block_id];
    for &(succ, _) in &block.successors {
        if let Some(idom) = immediate_dominator(cfg, block_id) {
            if !cfg.blocks[succ].dominators.contains(&idom) {
                frontier.insert(succ);
            }
        }
    }
    for other in 0..cfg.blocks.len() {
        if other == block_id {
            continue;
        }
        for &(succ, _) in &cfg.blocks[other].successors {
            if succ == block_id {
                if let Some(idom) = immediate_dominator(cfg, other) {
                    if !cfg.blocks[block_id].dominators.contains(&idom) {
                        frontier.insert(block_id);
                    }
                }
            }
        }
    }
    frontier
}
