// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! R4 phase 1: Steensgaard-style unification points-to (near-linear).
//!
//! Steensgaard's algorithm treats every assignment as an *equality*
//! constraint and merges the pointees' equivalence classes with union-find.
//! It is an over-approximation of Andersen's inclusion-based result (some
//! sets that are separate will be merged) but runs in almost-linear time,
//! the standard industrial first phase for scaling points-to to large
//! programs.
//!
//! The output is a partition of variables into **pointer equivalence
//! classes**. Consumers (two-phase pipeline) use it to decide where the
//! expensive Andersen phase must run at full precision: only classes that
//! can contain taint-relevant variables. Classes that cannot matter keep
//! the cheap unified answer.

use rustc_hash::FxHashMap;

use crate::ir::function::{FunctionIR, Instruction, Operand, VarId};

/// A pointer-equivalence class produced by unification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassId(pub usize);

/// Union-find over variable ids with path compression + union by size.
#[derive(Debug, Default)]
struct UnionFind {
    parent: FxHashMap<usize, usize>,
    size: FxHashMap<usize, usize>,
}

impl UnionFind {
    fn find(&mut self, v: usize) -> usize {
        // Make sure the node exists.
        self.parent.entry(v).or_insert(v);
        self.size.entry(v).or_insert(1);
        // Path compression (iterative).
        let mut root = v;
        while self.parent[&root] != root {
            root = self.parent[&root];
        }
        let mut cur = v;
        while self.parent[&cur] != cur {
            let next = self.parent[&cur];
            self.parent.insert(cur, root);
            cur = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        let (small, large) = if self.size[&ra] < self.size[&rb] {
            (ra, rb)
        } else {
            (rb, ra)
        };
        self.parent.insert(small, large);
        *self.size.get_mut(&large).unwrap() += self.size[&small];
    }
}

/// The result of Steensgaard unification over one function: variable →
/// equivalence class, plus the class membership lists.
#[derive(Debug, Default)]
pub struct Steensgaard {
    /// Class of each participating variable.
    pub class_of: FxHashMap<VarId, ClassId>,
    /// Members per class (for phase-2 restricted Andersen).
    pub members: FxHashMap<ClassId, Vec<VarId>>,
}

impl Steensgaard {
    /// Unify over one function's assignment-like constraints.
    ///
    /// Constraints collected (all equalities, the Steensgaard premise):
    /// - `Assign { dest, src: Var }`, `Cast { dest, src: Var }`
    /// - Phi incoming edges (`dest = incoming`)
    /// - `LoadField { dest, base, field }` unifies dest with the
    ///   representative of `base.field` via a synthetic field-var class so
    ///   field structure survives: two loads of the same base.field land in
    ///   the same class, loads of different fields stay separate at this
    ///   stage (phase 2 refines).
    pub fn analyze(ir: &FunctionIR) -> Self {
        let mut uf = UnionFind::default();
        let mut field_var: FxHashMap<(VarId, String), VarId> = FxHashMap::default();
        let mut next_synthetic = ir
            .var_metadata
            .keys()
            .copied()
            .map(|v| v.0)
            .max()
            .unwrap_or(0)
            + 1000;

        // Represent a (base, field) access by a synthetic variable so
        // equality constraints on field reads/writes unify correctly.
        let field_rep = |uf: &mut UnionFind,
                         field_var: &mut FxHashMap<(VarId, String), VarId>,
                         next: &mut usize,
                         base: VarId,
                         field: &str|
         -> VarId {
            let key = (base, field.to_string());
            if let Some(&v) = field_var.get(&key) {
                return v;
            }
            let v = VarId(*next);
            *next += 1;
            field_var.insert(key, v);
            uf.find(v.0);
            // The field access points into the base's class neighborhood:
            // unify with nothing by default; constraints drive merging.
            v
        };

        // Seed: every parameter and allocated var is its own class root.
        for &p in &ir.parameters {
            uf.find(p.0);
        }
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                if let Instruction::Allocate { dest, .. } = instr {
                    uf.find(dest.0);
                }
            }
        }

        // Collect equality constraints.
        for block in ir.blocks.values() {
            for phi in &block.phis {
                for &(_, inc) in &phi.incoming {
                    uf.union(phi.dest.0, inc.0);
                }
            }
            for instr in &block.instructions {
                match instr {
                    Instruction::Assign {
                        dest,
                        src: Operand::Var(src),
                    }
                    | Instruction::Cast {
                        dest,
                        src: Operand::Var(src),
                        ..
                    } => uf.union(dest.0, src.0),
                    Instruction::StoreField {
                        base,
                        field,
                        src: Operand::Var(sv),
                        ..
                    } => {
                        let rep =
                            field_rep(&mut uf, &mut field_var, &mut next_synthetic, *base, field);
                        uf.union(rep.0, sv.0);
                    }
                    Instruction::LoadField {
                        dest, base, field, ..
                    } => {
                        let rep =
                            field_rep(&mut uf, &mut field_var, &mut next_synthetic, *base, field);
                        uf.union(dest.0, rep.0);
                    }
                    Instruction::StoreElement {
                        base, index, src, ..
                    } => {
                        let f = match index {
                            Operand::StringLiteral(s) => s.clone(),
                            _ => "*".to_string(),
                        };
                        if let Operand::Var(sv) = src {
                            let rep =
                                field_rep(&mut uf, &mut field_var, &mut next_synthetic, *base, &f);
                            uf.union(rep.0, sv.0);
                        }
                    }
                    Instruction::LoadElement {
                        dest, base, index, ..
                    } => {
                        let f = match index {
                            Operand::StringLiteral(s) => s.clone(),
                            _ => "*".to_string(),
                        };
                        let rep =
                            field_rep(&mut uf, &mut field_var, &mut next_synthetic, *base, &f);
                        uf.union(dest.0, rep.0);
                    }
                    _ => {}
                }
            }
        }

        // Materialize classes.
        let mut class_of: FxHashMap<VarId, ClassId> = FxHashMap::default();
        let mut members: FxHashMap<ClassId, Vec<VarId>> = FxHashMap::default();
        let mut root_to_class: FxHashMap<usize, ClassId> = FxHashMap::default();
        let mut next_class = 0usize;

        let mut roots: Vec<usize> = uf
            .parent
            .keys()
            .copied()
            .filter(|v| !is_synthetic(*v, next_synthetic))
            .collect();
        roots.sort_unstable();
        for v in roots {
            let root = uf.find(v);
            let class = *root_to_class.entry(root).or_insert_with(|| {
                let c = ClassId(next_class);
                next_class += 1;
                c
            });
            let vid = VarId(v);
            class_of.insert(vid, class);
            members.entry(class).or_default().push(vid);
        }

        Steensgaard { class_of, members }
    }

    /// Equivalence class of `v`, if it participates in any constraint.
    pub fn class_of(&self, v: VarId) -> Option<ClassId> {
        self.class_of.get(&v).copied()
    }
}

/// Synthetic field-representation vars live above the real var-id space;
/// they never appear in the emitted classes.
fn is_synthetic(v: usize, synthetic_base: usize) -> bool {
    v >= synthetic_base
}
