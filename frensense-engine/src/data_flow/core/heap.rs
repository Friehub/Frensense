// SPDX-License-Identifier: MIT

//! Phase 3: Points-To (Heap) Analysis
//!
//! This module implements Andersen's Points-To Analysis. It computes the set
//! of abstract memory locations that each variable can point to. This cleanly
//! tracks deeply nested object properties and handles array/dictionary elements.

use rustc_hash::{FxHashMap, FxHashSet};
use crate::data_flow::ir::*;

/// A unique identifier for an abstract memory allocation in the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocId(pub usize);

/// Computes and stores the Points-To graph for the function.
#[derive(Debug, Default)]
pub struct PointsToAnalysis {
    /// Maps a variable to the set of memory locations it could point to.
    /// Example: `v1 -> {Loc_1, Loc_2}`
    pub pts: FxHashMap<VarId, FxHashSet<LocId>>,
    
    /// Maps (Base Location, Field Name) to a set of memory locations.
    /// Example: `Loc_1.user -> {Loc_5}`
    /// Note: The field `"*"` represents dynamic array/element wildcard access.
    pub fields: FxHashMap<(LocId, String), FxHashSet<LocId>>,
    
    next_loc_id: usize,
}

impl PointsToAnalysis {
    pub fn new() -> Self {
        Self {
            pts: FxHashMap::default(),
            fields: FxHashMap::default(),
            next_loc_id: 1,
        }
    }

    /// Allocates a new abstract memory location.
    fn alloc(&mut self) -> LocId {
        let id = LocId(self.next_loc_id);
        self.next_loc_id += 1;
        id
    }

    /// Iteratively computes the Points-To constraints until a fixed-point is reached.
    pub fn analyze(&mut self, ir: &FunctionIR) {
        // 1. Initial Pass: Allocate memory for function parameters
        // Without this, inputs like `req` would have no memory to attach properties to!
        for &param_var in &ir.parameters {
            let loc = self.alloc();
            self.pts.entry(param_var).or_default().insert(loc);
        }

        // 2. Initial Pass: Allocate locations for explicit `Allocate` instructions
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                if let Instruction::Allocate { dest, .. } = instr {
                    let loc = self.alloc();
                    self.pts.entry(*dest).or_default().insert(loc);
                }
            }
        }

        // 3. Fixed-Point Iteration (Andersen's Algorithm)
        let mut changed = true;
        while changed {
            changed = false;

            for block in ir.blocks.values() {
                // A. Handle SSA Phi Nodes (v_dest = v_1 | v_2)
                for phi in &block.phis {
                    let dest = phi.dest;
                    let mut incoming_pts = FxHashSet::default();
                    
                    for &(_, inc_var) in &phi.incoming {
                        if let Some(set) = self.pts.get(&inc_var) {
                            incoming_pts.extend(set.iter().copied());
                        }
                    }
                    
                    if Self::merge_sets(&mut self.pts, dest, &incoming_pts) {
                        changed = true;
                    }
                }

                // B. Handle Data Flow Instructions
                for instr in &block.instructions {
                    match instr {
                        // v_dest = v_src
                        Instruction::Assign { dest, src: Operand::Var(src_var) } |
                        Instruction::Cast { dest, src: Operand::Var(src_var), .. } => {
                            if let Some(src_pts) = self.pts.get(src_var).cloned() {
                                if Self::merge_sets(&mut self.pts, *dest, &src_pts) {
                                    changed = true;
                                }
                            }
                        }
                        
                        // base.field = src (Static)
                        Instruction::StoreField { base, field, src: Operand::Var(src_var) } => {
                            if let (Some(base_pts), Some(src_pts)) = (self.pts.get(base).cloned(), self.pts.get(src_var).cloned()) {
                                for &loc in &base_pts {
                                    if Self::merge_field_sets(&mut self.fields, loc, field.clone(), &src_pts) {
                                        changed = true;
                                    }
                                }
                            }
                        }
                        
                        // v_dest = base.field (Static)
                        Instruction::LoadField { dest, base, field } => {
                            if let Some(base_pts) = self.pts.get(base).cloned() {
                                let mut field_pts = FxHashSet::default();
                                for &loc in &base_pts {
                                    if let Some(set) = self.fields.get(&(loc, field.clone())) {
                                        field_pts.extend(set.iter().copied());
                                    }
                                }
                                if Self::merge_sets(&mut self.pts, *dest, &field_pts) {
                                    changed = true;
                                }
                            }
                        }
                        
                        // base[index] = src (Dynamic Arrays/Dicts)
                        Instruction::StoreElement { base, index, src: Operand::Var(src_var) } => {
                            if let (Some(base_pts), Some(src_pts)) = (self.pts.get(base).cloned(), self.pts.get(src_var).cloned()) {
                                // If index is a known string (e.g. obj["name"]), treat it statically.
                                // Otherwise, use the wildcard "*" to represent dynamic array population.
                                let field_name = if let Operand::StringLiteral(s) = index {
                                    s.clone()
                                } else {
                                    "*".to_string()
                                };
                                
                                for &loc in &base_pts {
                                    if Self::merge_field_sets(&mut self.fields, loc, field_name.clone(), &src_pts) {
                                        changed = true;
                                    }
                                }
                            }
                        }
                        
                        // v_dest = base[index] (Dynamic Arrays/Dicts)
                        Instruction::LoadElement { dest, base, index } => {
                            if let Some(base_pts) = self.pts.get(base).cloned() {
                                let field_name = if let Operand::StringLiteral(s) = index {
                                    s.clone()
                                } else {
                                    "*".to_string()
                                };
                                
                                let mut element_pts = FxHashSet::default();
                                for &loc in &base_pts {
                                    // Try exact match
                                    if let Some(set) = self.fields.get(&(loc, field_name.clone())) {
                                        element_pts.extend(set.iter().copied());
                                    }
                                    // Always mix in the wildcard '*' since dynamic writes could have put taint anywhere
                                    if field_name != "*" {
                                        if let Some(set) = self.fields.get(&(loc, "*".to_string())) {
                                            element_pts.extend(set.iter().copied());
                                        }
                                    }
                                }
                                if Self::merge_sets(&mut self.pts, *dest, &element_pts) {
                                    changed = true;
                                }
                            }
                        }

                        // Pointers (Rust/C++)
                        Instruction::AddressOf { dest, src } => {
                            if let Some(src_pts) = self.pts.get(src).cloned() {
                                if Self::merge_sets(&mut self.pts, *dest, &src_pts) {
                                    changed = true;
                                }
                            }
                        }
                        Instruction::Dereference { dest, ptr: Operand::Var(ptr_var) } => {
                            if let Some(ptr_pts) = self.pts.get(ptr_var).cloned() {
                                if Self::merge_sets(&mut self.pts, *dest, &ptr_pts) {
                                    changed = true;
                                }
                            }
                        }
                        
                        _ => {}
                    }
                }
            }
        }
    }

    /// Helper: Merges new points-to locations into a VarId's set. Returns true if the set grew.
    fn merge_sets(map: &mut FxHashMap<VarId, FxHashSet<LocId>>, key: VarId, new_vals: &FxHashSet<LocId>) -> bool {
        let entry = map.entry(key).or_default();
        let mut changed = false;
        for &val in new_vals {
            if entry.insert(val) {
                changed = true;
            }
        }
        changed
    }

    /// Helper: Merges new points-to locations into an Object Field's set. Returns true if the set grew.
    fn merge_field_sets(map: &mut FxHashMap<(LocId, String), FxHashSet<LocId>>, loc: LocId, field: String, new_vals: &FxHashSet<LocId>) -> bool {
        let entry = map.entry((loc, field)).or_default();
        let mut changed = false;
        for &val in new_vals {
            if entry.insert(val) {
                changed = true;
            }
        }
        changed
    }
}
