// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Phase 2: Static Single Assignment (SSA) Construction
//!
//! This module converts our "unstructured" FunctionIR into strict SSA form.
//! It seamlessly versions standard variables AND Memory States to produce
//! a perfectly flow-sensitive graph.

use crate::ir::function::*;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct SSABuilder {
    ir: FunctionIR,
    /// Phi rename bookkeeping: `new_dest → original variable`. Filled while
    /// renaming a block's phis; consulted when predecessors fill
    /// `incoming` so the stack lookup uses the ORIGINAL variable even after
    /// the phi's dest has been renamed (the successor-fill for a later
    /// predecessor must find the same stack the rename pushed onto).
    phi_origins: FxHashMap<VarId, VarId>,
}

impl SSABuilder {
    pub fn new(ir: FunctionIR) -> Self {
        Self {
            ir,
            phi_origins: FxHashMap::default(),
        }
    }

    pub fn build(mut self) -> FunctionIR {
        let idoms = self.compute_immediate_dominators();
        let df = self.compute_dominance_frontiers(&idoms);

        self.place_phi_nodes(&df);
        self.rename_variables(&idoms);

        self.ir
    }

    fn compute_immediate_dominators(&self) -> FxHashMap<BlockId, BlockId> {
        let mut doms: FxHashMap<BlockId, FxHashSet<BlockId>> = FxHashMap::default();
        let all_blocks: FxHashSet<BlockId> = self.ir.blocks.keys().copied().collect();

        for &b in &all_blocks {
            if b == self.ir.entry_block {
                let mut s = FxHashSet::default();
                s.insert(b);
                doms.insert(b, s);
            } else {
                doms.insert(b, all_blocks.clone());
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            for (&b_id, block) in &self.ir.blocks {
                if b_id == self.ir.entry_block {
                    continue;
                }

                let mut new_dom = all_blocks.clone();
                for &pred in &block.predecessors {
                    if let Some(pred_doms) = doms.get(&pred) {
                        new_dom = new_dom.intersection(pred_doms).copied().collect();
                    }
                }
                new_dom.insert(b_id);

                if doms.get(&b_id) != Some(&new_dom) {
                    doms.insert(b_id, new_dom);
                    changed = true;
                }
            }
        }

        let mut idoms = FxHashMap::default();
        for (&b_id, dom_set) in &doms {
            if b_id == self.ir.entry_block {
                continue;
            }
            let strict_doms: FxHashSet<BlockId> =
                dom_set.iter().copied().filter(|&d| d != b_id).collect();
            for &candidate in &strict_doms {
                let candidate_strict_doms: FxHashSet<BlockId> = doms[&candidate]
                    .iter()
                    .copied()
                    .filter(|&d| d != candidate)
                    .collect();
                let is_idom = strict_doms
                    .iter()
                    .all(|&other| other == candidate || candidate_strict_doms.contains(&other));
                if is_idom {
                    idoms.insert(b_id, candidate);
                    break;
                }
            }
        }
        idoms
    }

    fn compute_dominance_frontiers(
        &self,
        idoms: &FxHashMap<BlockId, BlockId>,
    ) -> FxHashMap<BlockId, FxHashSet<BlockId>> {
        let mut df: FxHashMap<BlockId, FxHashSet<BlockId>> = FxHashMap::default();
        for &b_id in self.ir.blocks.keys() {
            df.insert(b_id, FxHashSet::default());
        }

        for (&b_id, block) in &self.ir.blocks {
            if block.predecessors.len() >= 2 {
                for &pred in &block.predecessors {
                    let mut runner = pred;
                    while runner != *idoms.get(&b_id).unwrap_or(&self.ir.entry_block) {
                        df.get_mut(&runner).unwrap().insert(b_id);
                        if let Some(&next) = idoms.get(&runner) {
                            runner = next;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        df
    }

    fn place_phi_nodes(&mut self, df: &FxHashMap<BlockId, FxHashSet<BlockId>>) {
        let mut def_blocks: FxHashMap<VarId, FxHashSet<BlockId>> = FxHashMap::default();

        for (b_id, block) in &self.ir.blocks {
            for instr in &block.instructions {
                for dest in Self::get_instruction_dests(instr) {
                    def_blocks.entry(dest).or_default().insert(*b_id);
                }
            }
        }

        let mut phi_placements: FxHashMap<BlockId, FxHashSet<VarId>> = FxHashMap::default();
        for (&var, blocks) in &def_blocks {
            let mut worklist: Vec<BlockId> = blocks.iter().copied().collect();
            let mut added_to_worklist = FxHashSet::default();

            while let Some(n) = worklist.pop() {
                if let Some(frontiers) = df.get(&n) {
                    for &f in frontiers {
                        let placed = phi_placements.entry(f).or_default();
                        if !placed.contains(&var) {
                            placed.insert(var);
                            self.ir.blocks.get_mut(&f).unwrap().phis.push(Phi {
                                dest: var,
                                incoming: Vec::new(),
                            });

                            if added_to_worklist.insert(f) {
                                worklist.push(f);
                            }
                        }
                    }
                }
            }
        }
    }

    fn rename_variables(&mut self, idoms: &FxHashMap<BlockId, BlockId>) {
        let mut dom_tree: FxHashMap<BlockId, Vec<BlockId>> = FxHashMap::default();
        for (&node, &idom) in idoms {
            dom_tree.entry(idom).or_default().push(node);
        }

        let mut counts: FxHashMap<VarId, usize> = FxHashMap::default();
        let mut stacks: FxHashMap<VarId, Vec<VarId>> = FxHashMap::default();

        for &param in &self.ir.parameters {
            stacks.entry(param).or_default().push(param);
        }

        stacks
            .entry(self.ir.initial_memory_state)
            .or_default()
            .push(self.ir.initial_memory_state);

        self.rename_block(self.ir.entry_block, &dom_tree, &mut counts, &mut stacks);
    }

    fn rename_block(
        &mut self,
        block_id: BlockId,
        dom_tree: &FxHashMap<BlockId, Vec<BlockId>>,
        counts: &mut FxHashMap<VarId, usize>,
        stacks: &mut FxHashMap<VarId, Vec<VarId>>,
    ) {
        let mut pushed_vars = Vec::new();

        let mut phis = std::mem::take(&mut self.ir.blocks.get_mut(&block_id).unwrap().phis);
        for phi in &mut phis {
            let orig = phi.dest;
            let new_id = self.generate_new_name(orig, counts, stacks);
            self.phi_origins.insert(new_id, orig);
            phi.dest = new_id;
            pushed_vars.push(orig);
        }
        self.ir.blocks.get_mut(&block_id).unwrap().phis = phis;

        let mut instructions =
            std::mem::take(&mut self.ir.blocks.get_mut(&block_id).unwrap().instructions);
        for instr in &mut instructions {
            Self::replace_uses(instr, stacks);
            self.rename_instruction_defs(instr, counts, stacks, &mut pushed_vars);
        }
        self.ir.blocks.get_mut(&block_id).unwrap().instructions = instructions;

        Self::replace_uses_in_terminator(
            &mut self.ir.blocks.get_mut(&block_id).unwrap().terminator,
            stacks,
        );

        let successors = self.ir.blocks.get(&block_id).unwrap().successors.clone();
        for succ_id in successors {
            if let Some(succ) = self.ir.blocks.get_mut(&succ_id) {
                for phi in &mut succ.phis {
                    // Resolve the phi's ORIGINAL variable (pre-rename): if a
                    // dom-tree child already renamed this phi, `phi.dest` is
                    // the new id whose stack is empty. Without the origin
                    // mapping, every predecessor visited after the merge
                    // block's rename silently contributes no incoming edge.
                    let orig = *self.phi_origins.get(&phi.dest).unwrap_or(&phi.dest);
                    if let Some(stack) = stacks.get(&orig)
                        && let Some(&active_var) = stack.last()
                    {
                        phi.incoming.push((block_id, active_var));
                    }
                }
            }
        }

        if let Some(children) = dom_tree.get(&block_id) {
            for &child in children {
                self.rename_block(child, dom_tree, counts, stacks);
            }
        }

        for orig in pushed_vars {
            stacks.get_mut(&orig).unwrap().pop();
        }
    }

    fn generate_new_name(
        &mut self,
        orig: VarId,
        counts: &mut FxHashMap<VarId, usize>,
        stacks: &mut FxHashMap<VarId, Vec<VarId>>,
    ) -> VarId {
        *counts.entry(orig).or_insert(0) += 1;
        let meta = self
            .ir
            .var_metadata
            .get(&orig)
            .cloned()
            .unwrap_or(VarMetadata {
                source_name: None,
                type_name: None,
                byte_range: None,
                is_memory_state: false,
                object_keys: Vec::new(),
            });
        let new_id = self.ir.new_var(meta);
        stacks.entry(orig).or_default().push(new_id);
        new_id
    }

    fn replace_uses(instr: &mut Instruction, stacks: &FxHashMap<VarId, Vec<VarId>>) {
        let replace_var = |v: &mut VarId| {
            if let Some(stack) = stacks.get(v)
                && let Some(&top) = stack.last()
            {
                *v = top;
            }
        };
        let replace_op = |op: &mut Operand| {
            if let Operand::Var(v) = op {
                replace_var(v);
            }
        };

        match instr {
            Instruction::Assign { src, .. } => replace_op(src),

            Instruction::LoadField { mem_in, base, .. } => {
                replace_var(mem_in);
                replace_var(base);
            }
            Instruction::StoreField {
                mem_in, base, src, ..
            } => {
                replace_var(mem_in);
                replace_var(base);
                replace_op(src);
            }

            Instruction::LoadElement {
                mem_in,
                base,
                index,
                ..
            } => {
                replace_var(mem_in);
                replace_var(base);
                replace_op(index);
            }
            Instruction::StoreElement {
                mem_in,
                base,
                index,
                src,
                ..
            } => {
                replace_var(mem_in);
                replace_var(base);
                replace_op(index);
                replace_op(src);
            }

            Instruction::LoadGlobal { mem_in, .. } => {
                replace_var(mem_in);
            }
            Instruction::StoreGlobal { mem_in, src, .. } => {
                replace_var(mem_in);
                replace_op(src);
            }

            Instruction::Allocate { mem_in, .. } => replace_var(mem_in),
            Instruction::AddressOf { src, .. } => replace_var(src),
            Instruction::Dereference { mem_in, ptr, .. } => {
                replace_var(mem_in);
                replace_op(ptr);
            }

            Instruction::CallStatic { mem_in, args, .. } => {
                replace_var(mem_in);
                args.iter_mut().for_each(replace_op);
            }
            Instruction::CallVirtual {
                mem_in,
                receiver,
                args,
                ..
            } => {
                replace_var(mem_in);
                replace_op(receiver);
                args.iter_mut().for_each(replace_op);
            }
            Instruction::CallPointer {
                mem_in,
                func_ptr,
                args,
                ..
            } => {
                replace_var(mem_in);
                replace_op(func_ptr);
                args.iter_mut().for_each(replace_op);
            }

            Instruction::BinaryOp { lhs, rhs, .. } => {
                replace_op(lhs);
                replace_op(rhs);
            }
            Instruction::UnaryOp { src, .. } => replace_op(src),
            Instruction::Cast { src, .. } => replace_op(src),
            Instruction::ExtractValue { tuple, .. } => replace_op(tuple),

            Instruction::Await {
                mem_in, promise, ..
            } => {
                replace_var(mem_in);
                replace_op(promise);
            }
            Instruction::Yield { mem_in, src, .. } => {
                replace_var(mem_in);
                if let Some(s) = src {
                    replace_op(s);
                }
            }
        }
    }

    fn replace_uses_in_terminator(term: &mut Terminator, stacks: &FxHashMap<VarId, Vec<VarId>>) {
        let replace_op = |op: &mut Operand| {
            if let Operand::Var(v) = op
                && let Some(stack) = stacks.get(v)
                && let Some(&top) = stack.last()
            {
                *v = top;
            }
        };

        match term {
            Terminator::Branch { cond, .. } => replace_op(cond),
            Terminator::Return { src: Some(src) } => replace_op(src),
            Terminator::Throw { src } => replace_op(src),
            Terminator::Switch { cond, cases, .. } => {
                replace_op(cond);
                for (case_op, _) in cases {
                    replace_op(case_op);
                }
            }
            _ => {}
        }
    }

    fn get_instruction_dests(instr: &Instruction) -> Vec<VarId> {
        let mut dests = Vec::new();
        match instr {
            Instruction::Assign { dest, .. }
            | Instruction::LoadField { dest, .. }
            | Instruction::LoadElement { dest, .. }
            | Instruction::LoadGlobal { dest, .. }
            | Instruction::AddressOf { dest, .. }
            | Instruction::Dereference { dest, .. }
            | Instruction::Cast { dest, .. }
            | Instruction::ExtractValue { dest, .. }
            | Instruction::BinaryOp { dest, .. }
            | Instruction::UnaryOp { dest, .. } => dests.push(*dest),

            Instruction::StoreField { mem_out, .. }
            | Instruction::StoreElement { mem_out, .. }
            | Instruction::StoreGlobal { mem_out, .. } => dests.push(*mem_out),

            Instruction::Allocate { dest, mem_out, .. }
            | Instruction::Await { dest, mem_out, .. } => {
                dests.push(*dest);
                dests.push(*mem_out);
            }

            Instruction::CallStatic { dest, mem_out, .. }
            | Instruction::CallVirtual { dest, mem_out, .. }
            | Instruction::CallPointer { dest, mem_out, .. }
            | Instruction::Yield { dest, mem_out, .. } => {
                if let Some(d) = dest {
                    dests.push(*d);
                }
                dests.push(*mem_out);
            }
        }
        dests
    }

    fn rename_instruction_defs(
        &mut self,
        instr: &mut Instruction,
        counts: &mut FxHashMap<VarId, usize>,
        stacks: &mut FxHashMap<VarId, Vec<VarId>>,
        pushed_vars: &mut Vec<VarId>,
    ) {
        let mut process = |dest: &mut VarId| {
            let orig = *dest;
            let new_id = self.generate_new_name(orig, counts, stacks);
            *dest = new_id;
            pushed_vars.push(orig);
        };

        match instr {
            Instruction::Assign { dest, .. }
            | Instruction::LoadField { dest, .. }
            | Instruction::LoadElement { dest, .. }
            | Instruction::LoadGlobal { dest, .. }
            | Instruction::AddressOf { dest, .. }
            | Instruction::Dereference { dest, .. }
            | Instruction::Cast { dest, .. }
            | Instruction::ExtractValue { dest, .. }
            | Instruction::BinaryOp { dest, .. }
            | Instruction::UnaryOp { dest, .. } => process(dest),

            Instruction::StoreField { mem_out, .. }
            | Instruction::StoreElement { mem_out, .. }
            | Instruction::StoreGlobal { mem_out, .. } => process(mem_out),

            Instruction::Allocate { dest, mem_out, .. }
            | Instruction::Await { dest, mem_out, .. } => {
                process(dest);
                process(mem_out);
            }

            Instruction::CallStatic { dest, mem_out, .. }
            | Instruction::CallVirtual { dest, mem_out, .. }
            | Instruction::CallPointer { dest, mem_out, .. }
            | Instruction::Yield { dest, mem_out, .. } => {
                if let Some(d) = dest {
                    process(d);
                }
                process(mem_out);
            }
        }
    }
}
