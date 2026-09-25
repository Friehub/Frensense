// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Phase 3: Points-To (Heap) Analysis
//!
//! This module implements Andersen's Points-To Analysis. It computes the set
//! of abstract memory locations that each variable can point to. This cleanly
//! tracks deeply nested object properties and handles array/dictionary elements.

use crate::graph::steensgaard::Steensgaard;
use crate::ir::function::*;
use rustc_hash::{FxHashMap, FxHashSet};

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

    /// Maps local variables to their abstract memory locations (for AddressOf logic).
    pub var_locs: FxHashMap<VarId, LocId>,

    next_loc_id: usize,
}

impl PointsToAnalysis {
    pub fn new() -> Self {
        Self {
            pts: FxHashMap::default(),
            fields: FxHashMap::default(),
            var_locs: FxHashMap::default(),
            next_loc_id: 1, // LocId(0) is reserved for globals
        }
    }

    /// R4 phase 2: run Andersen's fixed point restricted to the taint-
    /// relevant Steensgaard equivalence classes.
    ///
    /// Steensgaard (phase 1) over-approximates pointer equivalence in
    /// almost-linear time. This method uses that partition as a *gate*:
    /// constraint work whose variable is irrelevant, not reachable from
    /// any taint anchor, is collapsed to one-shot handling instead of
    /// participating in the O(n³)-worst-case fixed point. Variables in
    /// relevant classes get the full Andersen treatment, so precision on
    /// everything that can matter for taint is preserved by construction.
    ///
    /// The result is usable exactly like [`PointsToAnalysis::analyze`]:
    /// alias queries on relevant variables see the same sets; queries on
    /// irrelevant variables fall back to the conservative parameter-root
    /// answer at the call sites that consume this graph.
    pub fn analyze_two_phase(&mut self, ir: &FunctionIR) {
        // Phase 1: unify.
        let st = Steensgaard::analyze(ir);

        // Gate: a class is taint-relevant when any member is an anchor.
        // Anchors are structural (config-independent) so the cached heap
        // stays valid across taint-config changes:
        //   * formal parameters, sources flow through them, and sink
        //     arguments arrive through them;
        //   * variables involved in call argument positions, sink checks
        //     happen at calls, and pass-through taint moves arg → dest;
        //   * allocation-site variables, locally built objects (payload
        //     containers) can carry taint into field flows;
        //   * variables compared against or branched on? No, those have
        //     no taint channel, deliberately excluded.
        let mut relevant: FxHashSet<crate::graph::steensgaard::ClassId> = FxHashSet::default();
        let mark_relevant = |v: VarId, st: &Steensgaard, relevant: &mut FxHashSet<_>| {
            if let Some(c) = st.class_of(v) {
                relevant.insert(c);
            }
        };
        for &p in &ir.parameters {
            mark_relevant(p, &st, &mut relevant);
        }
        // Allocates are anchors ONLY when their class reaches a call (arg,
        // receiver, or dest), a locally built object that never crosses a
        // call boundary cannot carry taint anywhere the SVFG can observe,
        // so precise field tracking for it is pure cost. Class-level check:
        // Steensgaard has already unified assign-chains, so a shared class
        // means a shared anchor.
        let arg_dest_classes: FxHashSet<crate::graph::steensgaard::ClassId> = ir
            .blocks
            .values()
            .flat_map(|b| b.instructions.iter())
            .filter_map(|instr| match instr {
                Instruction::CallStatic { args, dest, .. } => {
                    let vars: Vec<VarId> = args
                        .iter()
                        .filter_map(|a| match a {
                            Operand::Var(v) => Some(*v),
                            _ => None,
                        })
                        .chain(dest.iter().copied())
                        .collect();
                    Some(vars)
                }
                Instruction::CallVirtual {
                    args,
                    dest,
                    receiver,
                    ..
                } => {
                    let mut vars: Vec<VarId> = args
                        .iter()
                        .filter_map(|a| match a {
                            Operand::Var(v) => Some(*v),
                            _ => None,
                        })
                        .chain(dest.iter().copied())
                        .collect();
                    if let Operand::Var(r) = receiver {
                        vars.push(*r);
                    }
                    Some(vars)
                }
                _ => None,
            })
            .flatten()
            .filter_map(|v| st.class_of(v))
            .collect();
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                match instr {
                    Instruction::Allocate { dest, .. } => {
                        if let Some(c) = st.class_of(*dest)
                            && arg_dest_classes.contains(&c)
                        {
                            relevant.insert(c);
                        }
                    }
                    Instruction::CallStatic { args, dest, .. } => {
                        for a in args {
                            if let Operand::Var(v) = a {
                                mark_relevant(*v, &st, &mut relevant);
                            }
                        }
                        if let Some(d) = dest {
                            mark_relevant(*d, &st, &mut relevant);
                        }
                    }
                    Instruction::CallVirtual {
                        args,
                        dest,
                        receiver,
                        ..
                    } => {
                        if let Operand::Var(r) = receiver {
                            mark_relevant(*r, &st, &mut relevant);
                        }
                        for a in args {
                            if let Operand::Var(v) = a {
                                mark_relevant(*v, &st, &mut relevant);
                            }
                        }
                        if let Some(d) = dest {
                            mark_relevant(*d, &st, &mut relevant);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Phase 2: Andersen's fixed point, skipping constraint *work* whose
        // source variable is in an irrelevant class. All vars not tracked by
        // Steensgaard (e.g. memory-state temporaries) are treated as
        // relevant, over-approximating the gate, never under.
        let class_of = |v: VarId| -> Option<crate::graph::steensgaard::ClassId> { st.class_of(v) };
        let is_relevant = |v: VarId| -> bool {
            match class_of(v) {
                Some(c) => relevant.contains(&c),
                None => true, // untracked: never exclude (sound direction)
            }
        };
        self.analyze_inner(ir, Some(&is_relevant));
    }

    /// Allocates a new abstract memory location.
    pub fn alloc(&mut self) -> LocId {
        let id = LocId(self.next_loc_id);
        self.next_loc_id += 1;
        id
    }

    /// Iteratively computes the Points-To constraints until a fixed-point is reached.
    pub fn analyze(&mut self, ir: &FunctionIR) {
        self.analyze_inner(ir, None);
    }

    /// The fixed-point engine. `relevance` (phase 2) prunes work: when
    /// `Some(f)`, constraint handling for a *source* variable the filter
    /// rejects is reduced to propagating already-computed sets (no field
    /// indexing), which bounds the expensive part to the relevant cluster.
    fn analyze_inner(&mut self, ir: &FunctionIR, relevance: Option<&dyn Fn(VarId) -> bool>) {
        let skip_work = |v: VarId| -> bool {
            match relevance {
                Some(f) => !f(v),
                None => false,
            }
        };
        // Phase-2 deferrals: loads whose base is excluded from precise
        // field tracking, resolved once after the fixed point converges.
        let mut excluded_loads: Vec<(VarId, VarId)> = Vec::new();

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
                        Instruction::Assign {
                            dest,
                            src: Operand::Var(src_var),
                        }
                        | Instruction::Cast {
                            dest,
                            src: Operand::Var(src_var),
                            ..
                        } => {
                            if let Some(src_pts) = self.pts.get(src_var).cloned()
                                && Self::merge_sets(&mut self.pts, *dest, &src_pts)
                            {
                                changed = true;
                            }
                        }

                        // base.field = src (Static)
                        Instruction::StoreField {
                            base, field, src, ..
                        } => {
                            // Phase-2 gate: an irrelevant base's field
                            // structure is not tracked precisely, its
                            // field sets stay empty, and loads from it
                            // (below) fall back to the src set directly.
                            if skip_work(*base) {
                                continue;
                            }
                            if let Some(base_pts) = self.pts.get(base).cloned() {
                                match src {
                                    Operand::Var(src_var) => {
                                        if let Some(src_pts) = self.pts.get(src_var).cloned() {
                                            for &loc in &base_pts {
                                                if Self::merge_field_sets(
                                                    &mut self.fields,
                                                    loc,
                                                    field.clone(),
                                                    &src_pts,
                                                ) {
                                                    changed = true;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        for &loc in &base_pts {
                                            if let Some(set) =
                                                self.fields.get_mut(&(loc, field.clone()))
                                                && !set.is_empty()
                                            {
                                                set.clear();
                                                changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // v_dest = base.field (Static)
                        Instruction::LoadField {
                            dest, base, field, ..
                        } => {
                            if let Some(base_pts) = self.pts.get(base).cloned() {
                                let mut field_pts = FxHashSet::default();
                                for &loc in &base_pts {
                                    if let Some(s) = self.fields.get(&(loc, field.clone())) {
                                        field_pts.extend(s.iter().copied());
                                    }
                                }
                                // Phase-2 gate: when the base was excluded
                                // from precise field tracking, record the
                                // (dest, base) pair, it is resolved ONCE
                                // after convergence (dest inherits the
                                // base's final pts), instead of churning in
                                // the fixed point. Sound: over-approximates,
                                // never drops a relevant location.
                                if field_pts.is_empty() && skip_work(*base) && relevance.is_some() {
                                    excluded_loads.push((*dest, *base));
                                    continue;
                                }
                                if Self::merge_sets(&mut self.pts, *dest, &field_pts) {
                                    changed = true;
                                }
                            }
                        }

                        // base[index] = src (Dynamic Arrays/Dicts)
                        Instruction::StoreElement {
                            base, index, src, ..
                        } => {
                            if skip_work(*base) {
                                continue;
                            }
                            if let Some(base_pts) = self.pts.get(base).cloned() {
                                let field_name = if let Operand::StringLiteral(s) = index {
                                    s.clone()
                                } else {
                                    "*".to_string()
                                };
                                match src {
                                    Operand::Var(src_var) => {
                                        if let Some(src_pts) = self.pts.get(src_var).cloned() {
                                            for &loc in &base_pts {
                                                if Self::merge_field_sets(
                                                    &mut self.fields,
                                                    loc,
                                                    field_name.clone(),
                                                    &src_pts,
                                                ) {
                                                    changed = true;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        // Only clear exact string indices. Wildcards (*) represent arrays which could
                                        // hold multiple elements, so we don't strong-update (clear) the whole array.
                                        if field_name != "*" {
                                            for &loc in &base_pts {
                                                if let Some(set) =
                                                    self.fields.get_mut(&(loc, field_name.clone()))
                                                    && !set.is_empty()
                                                {
                                                    set.clear();
                                                    changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // v_dest = base[index] (Dynamic Arrays/Dicts)
                        Instruction::LoadElement {
                            dest, base, index, ..
                        } => {
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
                                    if field_name != "*"
                                        && let Some(set) = self.fields.get(&(loc, "*".to_string()))
                                    {
                                        element_pts.extend(set.iter().copied());
                                    }
                                }
                                // Phase-2 gate: same treatment as
                                // LoadField, excluded-base loads are
                                // resolved once after convergence.
                                if element_pts.is_empty() && skip_work(*base) && relevance.is_some()
                                {
                                    excluded_loads.push((*dest, *base));
                                    continue;
                                }
                                if Self::merge_sets(&mut self.pts, *dest, &element_pts) {
                                    changed = true;
                                }
                            }
                        }

                        // Pointers (Rust/C++)
                        Instruction::AddressOf { dest, src: src_var } => {
                            let l_src = *self.var_locs.entry(*src_var).or_insert_with(|| {
                                let id = LocId(self.next_loc_id);
                                self.next_loc_id += 1;
                                id
                            });

                            let mut s = FxHashSet::default();
                            s.insert(l_src);
                            if Self::merge_sets(&mut self.pts, *dest, &s) {
                                changed = true;
                            }

                            if let Some(src_pts) = self.pts.get(src_var).cloned()
                                && Self::merge_field_sets(
                                    &mut self.fields,
                                    l_src,
                                    "*".to_string(),
                                    &src_pts,
                                )
                            {
                                changed = true;
                            }
                        }

                        Instruction::Dereference {
                            dest,
                            ptr: Operand::Var(ptr_var),
                            ..
                        } => {
                            if let Some(ptr_pts) = self.pts.get(ptr_var).cloned() {
                                let mut dest_locs = FxHashSet::default();
                                for &l in &ptr_pts {
                                    if let Some(field_locs) = self.fields.get(&(l, "*".to_string()))
                                    {
                                        for &f_l in field_locs {
                                            dest_locs.insert(f_l);
                                        }
                                    }
                                }
                                if Self::merge_sets(&mut self.pts, *dest, &dest_locs) {
                                    changed = true;
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        // 4. Phase-2 deferred resolution: excluded-base loads inherit the
        // base's FINAL points-to set, once. This replaces per-iteration
        // fallback churn with a single merge; it is conservative (a strict
        // over-approximation of precise field tracking) and only ever adds
        // locations to an already-computed set.
        if relevance.is_some() {
            for (dest, base) in &excluded_loads {
                if let Some(src_pts) = self.pts.get(base).cloned() {
                    Self::merge_sets(&mut self.pts, *dest, &src_pts);
                }
            }
        }
    }

    /// Helper: Merges new points-to locations into a VarId's set. Returns true if the set grew.
    fn merge_sets(
        map: &mut FxHashMap<VarId, FxHashSet<LocId>>,
        key: VarId,
        new_vals: &FxHashSet<LocId>,
    ) -> bool {
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
    fn merge_field_sets(
        map: &mut FxHashMap<(LocId, String), FxHashSet<LocId>>,
        loc: LocId,
        field: String,
        new_vals: &FxHashSet<LocId>,
    ) -> bool {
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
