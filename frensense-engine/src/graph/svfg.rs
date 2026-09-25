// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Phase 3.5: Sparse Value-Flow Graph (SVFG)
//!
//! The SVFG is a directed graph `G = (N, E)` where:
//!   - Each node represents an SSA instruction that defines or uses a variable.
//!   - A directed edge `A → B` means: a value defined at `A` is used at `B`.
//!
//! Because our IR is already in SSA form, every variable has exactly one static
//! definition site. Def-use chains are therefore explicit and exact, the SVFG
//! is simply those chains made into a first-class graph structure.
//!
//! ## Why this matters for taint analysis
//!
//! The current `TaintEngine` runs a fixed-point loop over *all* instructions in
//! *all* blocks, repeating until nothing changes. On a large codebase this visits
//! huge amounts of irrelevant IR. The SVFG changes the cost model:
//!
//!   * Taint propagation = BFS/DFS from **source nodes**, following edges.
//!   * No fixed-point loop. No visiting blocks that aren't reachable from a source.
//!   * Cost is proportional to the number of actual taint paths, not program size.
//!
//! ## Node kinds
//!
//! Each `SvfgNode` carries a `NodeKind` that describes what kind of IR event it
//! represents:
//!
//! | Kind              | Meaning                                                   |
//! |-------------------|-----------------------------------------------------------|
//! | `Phi`             | SSA φ-node merging two or more incoming values            |
//! | `InstrDef`        | Instruction that **defines** a variable                   |
//! | `InstrUse`        | Instruction that **uses** a variable as an operand        |
//! | `FormalParam`     | Formal parameter of this function                         |
//! | `ActualArg`       | Argument passed at a call site (interprocedural attachment point) |
//! | `ActualRet`       | The destination variable that receives a call's return    |
//! | `FormalRet`       | The source of a `Return` terminator (interprocedural stub)|
//!
//! ## Edge model (the critical part)
//!
//! For an instruction `J` that reads `v1` and writes `v2`, we model **three**
//! nodes and two edges:
//!
//! ```text
//!   def(v1) ──▶ use(J, v1) ──▶ def(J, v2)
//! ```
//!
//! The `use(J, v1) → def(J, v2)` edge is the *intra-instruction flow* edge: it
//! is what makes taint survive an assignment, a binary op, or a call
//! destination. Without it, BFS dies at the first use node. This mirrors
//! exactly what `TaintEngine::process_instruction` does when it moves taint
//! from operands to dests, the graph just makes it explicit and reusable for
//! demand-driven backward traversal later (task 4.4).
//!
//! Memory states flow the same way: every instruction consuming `mem_in` and
//! producing `mem_out` gets a `mem_in-use → mem_out-def` edge, so memory taint
//! (from a `StoreField` whose `src` was tainted) keeps flowing forward through
//! the mem-state chain even though the SVFG engine currently tracks taint on
//! plain variables only.

use crate::graph::heap::PointsToAnalysis;
use crate::ir::function::*;
use rustc_hash::{FxHashMap, FxHashSet};

// ---------------------------------------------------------------------------
// Node key
// ---------------------------------------------------------------------------

/// A stable, cheap key identifying a node in the SVFG.
///
/// We use the *instruction index* within a block rather than a pointer so the
/// graph is fully serialisable and deterministic (important for task 4.5, the
/// corpus graph store).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeKey {
    pub block: BlockId,
    /// `None` for Phi nodes, `Some(idx)` for regular instructions.
    pub instr_idx: Option<usize>,
    pub var: VarId,
}

impl NodeKey {
    pub(crate) fn phi(block: BlockId, var: VarId) -> Self {
        Self {
            block,
            instr_idx: None,
            var,
        }
    }

    pub(crate) fn instr(block: BlockId, idx: usize, var: VarId) -> Self {
        Self {
            block,
            instr_idx: Some(idx),
            var,
        }
    }
}

// ---------------------------------------------------------------------------
// Node kind
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKind {
    /// SSA φ-function merging incoming values for `var`.
    Phi,
    /// This instruction *defines* `var` (dest or mem_out).
    InstrDef,
    /// This instruction *uses* `var` as an input operand or `mem_in`.
    InstrUse,
    /// `var` is a formal parameter of the enclosing function.
    FormalParam,
    /// `var` is passed as an argument at a call site (interprocedural attachment point).
    ActualArg {
        call_site: NodeKey,
        arg_index: usize,
    },
    /// `var` receives the return value of a call (interprocedural attachment point).
    ActualRet { call_site: NodeKey },
    /// `var` is the value returned by a `Return` terminator.
    FormalRet,
}

// ---------------------------------------------------------------------------
// Node
// ---------------------------------------------------------------------------

/// A single node in the SVFG.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SvfgNode {
    pub key: NodeKey,
    pub kind: NodeKind,
    /// Edges leaving this node (i.e. nodes that use the value defined here).
    successors: FxHashSet<NodeKey>,
    /// Edges entering this node (for reverse traversal / debugging).
    predecessors: FxHashSet<NodeKey>,
}

impl SvfgNode {
    /// Successor keys in deterministic (sorted) order.
    pub fn successors(&self) -> Vec<NodeKey> {
        let mut v: Vec<NodeKey> = self.successors.iter().copied().collect();
        v.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
        v
    }

    /// Predecessor keys in deterministic (sorted) order.
    pub fn predecessors(&self) -> Vec<NodeKey> {
        let mut v: Vec<NodeKey> = self.predecessors.iter().copied().collect();
        v.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
        v
    }
}

// ---------------------------------------------------------------------------
// Graph
// ---------------------------------------------------------------------------

/// The Sparse Value-Flow Graph for a single function.
///
/// Build it with [`SvfgBuilder::build`], then query it with the helper
/// methods or use [`SvfgTaintEngine`] for source-to-sink analysis.
#[derive(Debug, Default, Clone)]
pub struct Svfg {
    pub nodes: FxHashMap<NodeKey, SvfgNode>,
}

// Custom serde impls (task 4.5): serde_json cannot use struct-typed map keys
// ("key must be a string"), so the node map serialises as a *sorted* list of
// `(NodeKey, SvfgNode)` pairs, which is also byte-deterministic for the
// corpus store's content hashing.
#[cfg(feature = "serialize")]
impl serde::Serialize for Svfg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut v: Vec<(&NodeKey, &SvfgNode)> = self.nodes.iter().collect();
        v.sort_by_key(|(k, _)| (k.block.0, k.instr_idx, k.var.0));
        v.serialize(serializer)
    }
}

#[cfg(feature = "serialize")]
impl<'de> serde::Deserialize<'de> for Svfg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = Vec::<(NodeKey, SvfgNode)>::deserialize(deserializer)?;
        Ok(Self {
            nodes: v.into_iter().collect(),
        })
    }
}

impl Svfg {
    /// Returns the node for the given key, if it exists.
    #[inline]
    pub fn node(&self, key: &NodeKey) -> Option<&SvfgNode> {
        self.nodes.get(key)
    }

    /// All nodes whose `kind` matches a predicate.
    pub fn nodes_where<F: Fn(&SvfgNode) -> bool>(&self, pred: F) -> Vec<&SvfgNode> {
        self.nodes.values().filter(|n| pred(n)).collect()
    }

    /// Every node that is a `FormalParam`, convenient as BFS seeds for
    /// parameter-taint analysis.
    pub fn param_nodes(&self) -> Vec<&SvfgNode> {
        self.nodes_where(|n| n.kind == NodeKind::FormalParam)
    }

    /// Every node that is a definition (`InstrDef`, `Phi`, or `FormalParam`).
    pub fn def_nodes(&self) -> Vec<&SvfgNode> {
        self.nodes_where(|n| {
            matches!(
                n.kind,
                NodeKind::InstrDef | NodeKind::Phi | NodeKind::FormalParam
            )
        })
    }

    /// Add a directed edge `from → to`. O(1); dedup via hash set.
    fn add_edge(&mut self, from: NodeKey, to: NodeKey) {
        if let Some(f) = self.nodes.get_mut(&from) {
            f.successors.insert(to);
        }
        if let Some(t) = self.nodes.get_mut(&to) {
            t.predecessors.insert(from);
        }
    }

    /// Ensure a node exists; insert it if missing. Returns `true` if a *new*
    /// node was created (callers use this to decide whether to overwrite the
    /// default kind of a pre-existing node).
    fn ensure_node(&mut self, key: NodeKey, kind: NodeKind) -> bool {
        match self.nodes.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(SvfgNode {
                    key,
                    kind,
                    successors: FxHashSet::default(),
                    predecessors: FxHashSet::default(),
                });
                true
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builds a `Svfg` from an already-SSA-converted `FunctionIR`.
///
/// The algorithm is a direct implementation of the spec:
///
/// ```text
/// 1. For every instruction I that defines variable v  → create node(I, v)
/// 2. For every instruction J that uses variable v     → create node(J, v)
/// 3. For every use of v in J where v was defined at I → add edge node(I,v) → node(J,v)
///    3b. Intra-instruction flow: use(J, v) → def(J, w) for every w that J defines
/// 4. Call sites: actual-arg → formal-param edges, formal-ret → actual-ret edges
///    (attachment points created here; cross-function linking lands in task 4.2)
/// ```
///
/// Step 3 is O(1) per use because SSA guarantees exactly one definition for
/// every variable, we just look up the pre-built `def_site` table.
pub struct SvfgBuilder<'a> {
    ir: &'a FunctionIR,
    graph: Svfg,
    /// Maps each VarId to the SVFG node that *defines* it.
    def_site: FxHashMap<VarId, NodeKey>,
    /// Shared points-to analysis for this function, built lazily on first
    /// alias query and reused for the rest of the build (R4: one heap per
    /// function instead of one per query).
    heap_cache: Option<std::rc::Rc<PointsToAnalysis>>,
}

impl<'a> SvfgBuilder<'a> {
    pub fn new(ir: &'a FunctionIR) -> Self {
        Self {
            ir,
            graph: Svfg::default(),
            def_site: FxHashMap::default(),
            heap_cache: None,
        }
    }

    /// Consume the builder and return the finished graph plus the def-site table.
    pub fn build(mut self) -> (Svfg, FxHashMap<VarId, NodeKey>) {
        // -----------------------------------------------------------------
        // Pass 0: Register formal parameters as def nodes.
        // -----------------------------------------------------------------
        // Parameters are not "inside" any instruction, so we give them their
        // own node kind and a synthetic key. We reserve instr_idx positions
        // that can never collide with real instructions: real instructions
        // occupy [0, n), terminators use MAX-1, so parameters live at
        // MAX - 2 - i (one distinct slot per parameter).
        let entry = self.ir.entry_block;
        let param_slot_base = usize::MAX - 2;

        for (idx, &param_var) in self.ir.parameters.iter().enumerate() {
            let key = NodeKey::instr(entry, param_slot_base - idx, param_var);
            self.graph.ensure_node(key, NodeKind::FormalParam);
            self.def_site.insert(param_var, key);
        }

        // The initial memory state is also a "parameter" of sorts.
        {
            let mem = self.ir.initial_memory_state;
            let key = NodeKey::instr(entry, param_slot_base, mem);
            self.graph.ensure_node(key, NodeKind::FormalParam);
            self.def_site.insert(mem, key);
        }

        // -----------------------------------------------------------------
        // Pass 1: Walk every block and register def nodes + def sites.
        // -----------------------------------------------------------------
        // We walk blocks in a stable order so the graph is deterministic.
        let mut block_ids: Vec<BlockId> = self.ir.blocks.keys().copied().collect();
        block_ids.sort_by_key(|b| b.0);

        for &block_id in &block_ids {
            let block = &self.ir.blocks[&block_id];

            // --- Phi nodes ---
            for phi in &block.phis {
                let key = NodeKey::phi(block_id, phi.dest);
                self.graph.ensure_node(key, NodeKind::Phi);
                self.def_site.insert(phi.dest, key);
            }

            // --- Regular instructions ---
            for (idx, instr) in block.instructions.iter().enumerate() {
                for def_var in Self::instruction_defs(instr) {
                    let key = NodeKey::instr(block_id, idx, def_var);
                    self.graph.ensure_node(key, NodeKind::InstrDef);
                    self.def_site.insert(def_var, key);
                }
            }
        }

        // -----------------------------------------------------------------
        // Pass 2: Walk again to create use nodes and connect edges.
        // -----------------------------------------------------------------
        for &block_id in &block_ids {
            let block = &self.ir.blocks[&block_id];

            // --- Phi uses: each incoming (pred_block, incoming_var) is a use ---
            for phi in &block.phis {
                let phi_def_key = NodeKey::phi(block_id, phi.dest);
                for &(_, inc_var) in &phi.incoming {
                    // def(inc_var) → Phi(inc_var), value flows into the phi...
                    self.connect_def_to_use(inc_var, phi_def_key);
                }
            }

            // --- Regular instructions ---
            for (idx, instr) in block.instructions.iter().enumerate() {
                let defs = Self::instruction_defs(instr);

                // Memory-state handling: mem_in is consumed and (if the
                // instruction produces mem_out) the memory flows onward.
                let uses_mem = Self::instruction_uses_mem(instr);
                let has_mem_out = defs.iter().any(|v| {
                    self.ir
                        .var_metadata
                        .get(v)
                        .is_some_and(|m| m.is_memory_state)
                });

                // a) Operand uses: def(v) → use(J, v) → def(J, w) for each defined w.
                for use_var in Self::instruction_uses(instr) {
                    let use_key = NodeKey::instr(block_id, idx, use_var);
                    self.graph.ensure_node(use_key, NodeKind::InstrUse);
                    self.connect_def_to_use(use_var, use_key);

                    // Intra-instruction flow: the consumed value contributes to
                    // everything this instruction defines. This is the edge that
                    // lets taint cross Assign / BinaryOp / Cast / Call-dest etc.
                    for &def_var in &defs {
                        let def_key = NodeKey::instr(block_id, idx, def_var);
                        self.graph.add_edge(use_key, def_key);
                    }
                }

                // b) Memory flow: mem_in-use → mem_out-def (pass-through /
                //    weak update semantics at the graph level).
                if uses_mem && has_mem_out {
                    let mem_out = *defs
                        .iter()
                        .find(|v| {
                            self.ir
                                .var_metadata
                                .get(v)
                                .is_some_and(|m| m.is_memory_state)
                        })
                        .unwrap();
                    let mem_in = Self::instruction_mem_in(instr).expect("uses_mem implies mem_in");
                    let mem_in_key = NodeKey::instr(block_id, idx, mem_in);
                    self.graph.ensure_node(mem_in_key, NodeKind::InstrUse);
                    self.connect_def_to_use(mem_in, mem_in_key);
                    let mem_out_key = NodeKey::instr(block_id, idx, mem_out);
                    self.graph.add_edge(mem_in_key, mem_out_key);
                }

                // c) Call-site interprocedural attachment points.
                self.register_call_stubs(block_id, idx, instr);
            }

            // --- Terminator uses (Return / Throw / Branch) ---
            for use_var in Self::terminator_uses(&block.terminator) {
                // Terminators don't define variables. We reserve `usize::MAX - 1`
                // as the terminator's instr_idx (parameters live at MAX-2-i and
                // below, so there is no collision).
                let use_key = NodeKey::instr(block_id, usize::MAX - 1, use_var);
                let kind = if matches!(&block.terminator, Terminator::Return { .. }) {
                    NodeKind::FormalRet
                } else {
                    NodeKind::InstrUse
                };
                self.graph.ensure_node(use_key, kind);
                self.connect_def_to_use(use_var, use_key);
            }
        }

        // -----------------------------------------------------------------
        // Pass 3: field-sensitive heap edges (store.src → load.dest).
        // -----------------------------------------------------------------
        self.connect_heap_edges(&block_ids);

        (self.graph, self.def_site)
    }

    /// Pass 3: field-sensitive heap edges.
    ///
    /// `StoreField { base, field, src }` writes a value into an object field;
    /// `LoadField { base, field, dest }` reads it back. The generic Pass 2
    /// edges only connect `src → store` and `base → load`, so taint written
    /// into `obj.data` dies in the opaque memory state and a later
    /// `x = obj.data` reads clean, the classic missed-bug shape
    /// `obj.data = source(); sink(obj.data)`.
    ///
    /// This pass links them explicitly:
    ///
    /// ```text
    ///   def(src) ──▶ use(store.src) ──▶ def(load.dest)
    /// ```
    ///
    /// whenever the store's and load's *bases* may denote the same object
    /// (checked with the points-to analysis: intersecting base location
    /// sets, or both sides resolving to the same named parameter) and the
    /// field names match. `StoreElement`/`LoadElement` participate through
    /// the same mechanism with the wildcard field `"*"`.
    ///
    /// The pass runs on the SSA form, so a strong store kills the flow from
    /// earlier same-field stores only implicitly: the load simply also links
    /// to the later store, and both edges exist (over-approximation, never
    /// under-approximation, sound for taint).
    fn connect_heap_edges(&mut self, block_ids: &[BlockId]) {
        // Collect (block, idx, base, field, src_use_key) for every field store.
        let mut stores: Vec<(VarId, String, NodeKey)> = Vec::new();
        for &block_id in block_ids {
            let block = &self.ir.blocks[&block_id];
            for (idx, instr) in block.instructions.iter().enumerate() {
                let (base, field, src) = match instr {
                    Instruction::StoreField {
                        base, field, src, ..
                    } => (*base, field.clone(), src),
                    Instruction::StoreElement {
                        base, index, src, ..
                    } => (
                        *base,
                        match index {
                            Operand::StringLiteral(s) => s.clone(),
                            _ => "*".to_string(),
                        },
                        src,
                    ),
                    _ => continue,
                };
                if let Operand::Var(sv) = src {
                    stores.push((base, field, NodeKey::instr(block_id, idx, *sv)));
                }
            }
        }
        if stores.is_empty() {
            return;
        }

        // For every field load, link to every matching store.
        for &block_id in block_ids {
            let block = &self.ir.blocks[&block_id];
            for (idx, instr) in block.instructions.iter().enumerate() {
                let (base, field) = match instr {
                    Instruction::LoadField { base, field, .. } => (*base, field.clone()),
                    Instruction::LoadElement { base, index, .. } => (
                        *base,
                        match index {
                            Operand::StringLiteral(s) => s.clone(),
                            _ => "*".to_string(),
                        },
                    ),
                    _ => continue,
                };
                let load_dest_key = NodeKey::instr(
                    block_id,
                    idx,
                    match instr {
                        Instruction::LoadField { dest, .. }
                        | Instruction::LoadElement { dest, .. } => *dest,
                        _ => unreachable!(),
                    },
                );
                for (sbase, sfield, src_use_key) in &stores {
                    if sfield != &field && field != "*" && sfield != "*" {
                        continue; // field-sensitive: exact match or a wildcard side
                    }
                    if !self.may_alias_cached(*sbase, base) {
                        continue;
                    }
                    self.graph.add_edge(*src_use_key, load_dest_key);
                }
            }
        }
    }

    /// Alias query against the per-function cached heap analysis (R4:
    /// analyze once per function, not once per query; phase 2: the cached
    /// analysis is the *two-phase* Andersen restricted to taint-relevant
    /// Steensgaard classes, so its cost tracks the relevant cluster size).
    fn may_alias_cached(&mut self, a: VarId, b: VarId) -> bool {
        if a == b {
            return true;
        }
        // Fill the cache on first use.
        if self.heap_cache.is_none() {
            let mut heap = PointsToAnalysis::new();
            // Two-phase pays off only when the fixed point has real work:
            // below the threshold, plain Andersen is faster than paying for
            // Steensgaard + the relevance gate.
            const TWO_PHASE_MIN_INSTRS: usize = 4096;
            let instr_count: usize = self
                .ir
                .blocks
                .values()
                .map(|b| b.instructions.len() + b.phis.len())
                .sum();
            if instr_count >= TWO_PHASE_MIN_INSTRS {
                heap.analyze_two_phase(self.ir);
            } else {
                heap.analyze(self.ir);
            }
            self.heap_cache = Some(std::rc::Rc::new(heap));
        }
        let heap = self.heap_cache.as_ref().unwrap();
        let pa = heap.pts.get(&a).cloned().unwrap_or_default();
        let pb = heap.pts.get(&b).cloned().unwrap_or_default();
        if !pa.is_empty() && !pb.is_empty() {
            return !pa.is_disjoint(&pb);
        }
        Self::same_param_root(self.ir, a, b)
    }

    /// True if `a` and `b` both trace back (through Assign/Cast SSA chains)
    /// to the same formal parameter or the same `Allocate` site.
    fn same_param_root(ir: &FunctionIR, a: VarId, b: VarId) -> bool {
        fn root_of(ir: &FunctionIR, mut v: VarId) -> Option<VarId> {
            for _ in 0..32 {
                let mut found = None;
                'outer: for blk in ir.blocks.values() {
                    for instr in &blk.instructions {
                        match instr {
                            Instruction::Assign {
                                dest,
                                src: Operand::Var(s),
                            }
                            | Instruction::Cast {
                                dest,
                                src: Operand::Var(s),
                                ..
                            } if *dest == v => {
                                found = Some(*s);
                                break 'outer;
                            }
                            Instruction::Allocate { dest, .. } if *dest == v => {
                                return Some(v);
                            }
                            _ => {}
                        }
                    }
                }
                match found {
                    Some(prev) => v = prev,
                    None => return None,
                }
            }
            None
        }
        match (root_of(ir, a), root_of(ir, b)) {
            (Some(ra), Some(rb)) => ra == rb,
            _ => false,
        }
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Connect the definition site of `var` to a node that *uses* `var`.
    fn connect_def_to_use(&mut self, var: VarId, use_key: NodeKey) {
        if let Some(def_key) = self.def_site.get(&var).copied()
            && def_key != use_key
        {
            self.graph.add_edge(def_key, use_key);
        }
        // If `var` has no def site it's an external / undefined variable.
        // We silently skip, this can happen for globals referenced without
        // a `LoadGlobal` in the current function scope.
    }

    /// Create `ActualArg` and `ActualRet` attachment nodes for call instructions.
    ///
    /// These run *after* the generic use-node creation within the same
    /// instruction, so we can't collide with `InstrUse` nodes of the same
    /// `(block, idx, var)` key, instead of re-creating them we *upgrade* the
    /// existing node's kind in place (the node is the same value-flow point
    /// either way; the kind is just metadata for later interprocedural linking).
    fn register_call_stubs(&mut self, block: BlockId, idx: usize, instr: &Instruction) {
        let call_site_key = NodeKey::instr(block, idx, VarId(usize::MAX));

        let (args, receiver, dest): (&Vec<Operand>, Option<&Operand>, Option<VarId>) = match instr {
            Instruction::CallStatic { dest, args, .. } => (args, None, *dest),
            Instruction::CallVirtual {
                dest,
                receiver,
                args,
                ..
            } => (args, Some(receiver), *dest),
            Instruction::CallPointer { dest, args, .. } => (args, None, *dest),
            _ => return,
        };

        // Actual-arg attachment points (upgrade kind if the node already
        // exists as an InstrUse).
        for (arg_idx, arg) in args.iter().enumerate() {
            if let Operand::Var(use_var) = arg {
                let arg_key = NodeKey::instr(block, idx, *use_var);
                self.upgrade_node_kind(arg_key, || NodeKind::ActualArg {
                    call_site: call_site_key,
                    arg_index: arg_idx,
                });
            }
        }
        if let Some(Operand::Var(recv_var)) = receiver {
            let arg_key = NodeKey::instr(block, idx, *recv_var);
            self.upgrade_node_kind(arg_key, || NodeKind::ActualArg {
                call_site: call_site_key,
                arg_index: usize::MAX,
            });
        }

        // Actual-ret attachment point: the call's dest variable, if any. Its
        // def node already exists from Pass 1; just tag it.
        if let Some(ret_var) = dest {
            let ret_key = NodeKey::instr(block, idx, ret_var);
            self.upgrade_node_kind(ret_key, || NodeKind::ActualRet {
                call_site: call_site_key,
            });
        }
    }

    /// If `key` exists, overwrite its kind with `make_kind()`; if not, create
    /// the node with that kind directly.
    fn upgrade_node_kind<F: FnOnce() -> NodeKind>(&mut self, key: NodeKey, make_kind: F) {
        if let Some(node) = self.graph.nodes.get_mut(&key) {
            node.kind = make_kind();
        } else {
            self.graph.ensure_node(key, make_kind());
        }
    }

    // -----------------------------------------------------------------------
    // Pure functions: extract defs / uses from each instruction variant.
    // These mirror `SSABuilder::get_instruction_dests` / `replace_uses` but
    // return owned lists rather than mutating in place.
    // -----------------------------------------------------------------------

    fn instruction_defs(instr: &Instruction) -> Vec<VarId> {
        let mut v = Vec::with_capacity(2);
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
            | Instruction::UnaryOp { dest, .. } => v.push(*dest),

            Instruction::StoreField { mem_out, .. }
            | Instruction::StoreElement { mem_out, .. }
            | Instruction::StoreGlobal { mem_out, .. } => v.push(*mem_out),

            Instruction::Allocate { dest, mem_out, .. }
            | Instruction::Await { dest, mem_out, .. } => {
                v.push(*dest);
                v.push(*mem_out);
            }

            Instruction::CallStatic { dest, mem_out, .. }
            | Instruction::CallVirtual { dest, mem_out, .. }
            | Instruction::CallPointer { dest, mem_out, .. }
            | Instruction::Yield { dest, mem_out, .. } => {
                if let Some(d) = dest {
                    v.push(*d);
                }
                v.push(*mem_out);
            }
        }
        v
    }

    fn instruction_uses(instr: &Instruction) -> Vec<VarId> {
        let mut v: Vec<VarId> = Vec::with_capacity(4);

        let push_op = |op: &Operand, v: &mut Vec<VarId>| {
            if let Operand::Var(var) = op {
                v.push(*var);
            }
        };

        match instr {
            Instruction::Assign { src, .. }
            | Instruction::Cast { src, .. }
            | Instruction::UnaryOp { src, .. } => push_op(src, &mut v),

            Instruction::BinaryOp { lhs, rhs, .. } => {
                push_op(lhs, &mut v);
                push_op(rhs, &mut v);
            }

            Instruction::LoadField {
                mem_in: _, base, ..
            } => {
                v.push(*base);
            }

            Instruction::StoreField { base, src, .. } => {
                v.push(*base);
                push_op(src, &mut v);
            }

            Instruction::LoadElement { base, index, .. } => {
                v.push(*base);
                push_op(index, &mut v);
            }

            Instruction::StoreElement {
                base, index, src, ..
            } => {
                v.push(*base);
                push_op(index, &mut v);
                push_op(src, &mut v);
            }

            Instruction::LoadGlobal { .. } => {}

            Instruction::StoreGlobal { src, .. } => push_op(src, &mut v),

            Instruction::CallStatic { args, .. } => {
                args.iter().for_each(|a| push_op(a, &mut v));
            }

            Instruction::CallVirtual { receiver, args, .. } => {
                push_op(receiver, &mut v);
                args.iter().for_each(|a| push_op(a, &mut v));
            }

            Instruction::CallPointer { func_ptr, args, .. } => {
                push_op(func_ptr, &mut v);
                args.iter().for_each(|a| push_op(a, &mut v));
            }

            Instruction::Allocate { .. } => {}

            Instruction::AddressOf { src, .. } => v.push(*src),

            Instruction::Dereference { ptr, .. } => push_op(ptr, &mut v),

            Instruction::ExtractValue { tuple, .. } => push_op(tuple, &mut v),

            Instruction::Await { promise, .. } => push_op(promise, &mut v),

            Instruction::Yield { src, .. } => {
                if let Some(s) = src {
                    push_op(s, &mut v);
                }
            }
        }
        v
    }

    /// Returns the `mem_in` VarId if this instruction consumes a memory state.
    fn instruction_mem_in(instr: &Instruction) -> Option<VarId> {
        match instr {
            Instruction::LoadField { mem_in, .. }
            | Instruction::StoreField { mem_in, .. }
            | Instruction::LoadElement { mem_in, .. }
            | Instruction::StoreElement { mem_in, .. }
            | Instruction::LoadGlobal { mem_in, .. }
            | Instruction::StoreGlobal { mem_in, .. }
            | Instruction::CallStatic { mem_in, .. }
            | Instruction::CallVirtual { mem_in, .. }
            | Instruction::CallPointer { mem_in, .. }
            | Instruction::Allocate { mem_in, .. }
            | Instruction::Dereference { mem_in, .. }
            | Instruction::Await { mem_in, .. }
            | Instruction::Yield { mem_in, .. } => Some(*mem_in),
            _ => None,
        }
    }

    /// Alias kept for readability at the call site.
    fn instruction_uses_mem(instr: &Instruction) -> bool {
        Self::instruction_mem_in(instr).is_some()
    }

    fn terminator_uses(term: &Terminator) -> Vec<VarId> {
        let mut v = Vec::new();
        let push_op = |op: &Operand, v: &mut Vec<VarId>| {
            if let Operand::Var(var) = op {
                v.push(*var);
            }
        };
        match term {
            Terminator::Return { src: Some(src) } => push_op(src, &mut v),
            Terminator::Throw { src } => push_op(src, &mut v),
            Terminator::Branch { cond, .. } => push_op(cond, &mut v),
            Terminator::Switch { cond, cases, .. } => {
                push_op(cond, &mut v);
                for (case_op, _) in cases {
                    push_op(case_op, &mut v);
                }
            }
            _ => {}
        }
        v
    }
}

// ---------------------------------------------------------------------------
// Graph-traversal taint engine
// ---------------------------------------------------------------------------

use crate::analysis::taint::config::TaintConfig;

/// A taint engine that operates purely on the SVFG via BFS.
///
/// Unlike the fixed-point `TaintEngine`, this one:
///   * Seeds the worklist with nodes whose defining instruction is a **source**.
///   * Follows SVFG edges forward (def → use → def → …).
///   * Checks each reached node against **sinks**.
///   * Stops at **sanitizer** call sites.
///   * Applies call pass-through (non-source, non-sanitizer call with a tainted
///     arg taints the call's dest), matching `TaintEngine` semantics.
///
/// Because we follow only edges that exist in the graph, we naturally visit
/// only variables reachable from a source, exactly the "5% of the program"
/// mentioned in the spec.
pub struct SvfgTaintEngine<'a> {
    ir: &'a FunctionIR,
    #[allow(dead_code)] // reserved for 4.3 (two-phase points-to integration)
    heap: &'a PointsToAnalysis,
    config: &'a TaintConfig,
    graph: &'a Svfg,

    /// Nodes that have been confirmed tainted.
    tainted: FxHashSet<NodeKey>,

    pub alerts: Vec<String>,
}

impl<'a> SvfgTaintEngine<'a> {
    pub fn new(
        ir: &'a FunctionIR,
        heap: &'a PointsToAnalysis,
        config: &'a TaintConfig,
        graph: &'a Svfg,
        _def_site: &'a FxHashMap<VarId, NodeKey>,
    ) -> Self {
        Self {
            ir,
            heap,
            config,
            graph,
            tainted: FxHashSet::default(),
            alerts: Vec::new(),
        }
    }

    /// Run the BFS-based taint propagation.
    ///
    /// # Complexity
    ///
    /// O(|reachable nodes from sources|), proportional to taint paths,
    /// not program size.
    pub fn run(&mut self) {
        let mut worklist: std::collections::VecDeque<NodeKey> = std::collections::VecDeque::new();

        // Seed: find all InstrDef nodes whose instruction is a configured source.
        // We iterate in deterministic key order so alerts come out stable.
        let mut seed_keys: Vec<NodeKey> = self.graph.nodes.keys().copied().collect();
        seed_keys.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
        for key in seed_keys {
            if self.node_is_source(&key) {
                self.tainted.insert(key);
                worklist.push_back(key);
            }
        }

        // BFS
        while let Some(current) = worklist.pop_front() {
            let node = match self.graph.node(&current) {
                Some(n) => n,
                None => continue,
            };

            for succ_key in node.successors() {
                // Check for sanitizer: if the successor node represents a
                // sanitizer call-site use, stop propagation along this edge.
                if self.node_is_sanitizer_use(&succ_key) {
                    continue;
                }

                // Check for sink
                self.check_sink(&succ_key);

                if !self.tainted.contains(&succ_key) {
                    self.tainted.insert(succ_key);
                    worklist.push_back(succ_key);
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Returns `true` if the node at `key` is a definition that is a taint
    /// source (i.e. the defining instruction calls a configured source function).
    fn node_is_source(&self, key: &NodeKey) -> bool {
        let NodeKey {
            block,
            instr_idx: Some(idx),
            var,
        } = *key
        else {
            return false;
        };

        let block_data = match self.ir.blocks.get(&block) {
            Some(b) => b,
            None => return false,
        };

        // Guard: instruction index must be valid (not a sentinel)
        if idx >= block_data.instructions.len() {
            return false;
        }

        match &block_data.instructions[idx] {
            Instruction::CallStatic {
                func,
                dest: Some(dest_var),
                ..
            } => self.config.sources.contains(func) && *dest_var == var,
            Instruction::CallVirtual {
                method,
                dest: Some(dest_var),
                ..
            } => self.config.sources.contains(method) && *dest_var == var,
            _ => false,
        }
    }

    /// Returns `true` if the node at `key` is a *use* inside a sanitizer call.
    fn node_is_sanitizer_use(&self, key: &NodeKey) -> bool {
        let NodeKey {
            block,
            instr_idx: Some(idx),
            ..
        } = *key
        else {
            return false;
        };

        let block_data = match self.ir.blocks.get(&block) {
            Some(b) => b,
            None => return false,
        };

        if idx >= block_data.instructions.len() {
            return false;
        }

        match &block_data.instructions[idx] {
            Instruction::CallStatic { func, .. } => self.config.sanitizers.contains(func),
            Instruction::CallVirtual { method, .. } => self.config.sanitizers.contains(method),
            _ => false,
        }
    }

    /// Emits an alert if `key` is a use inside a configured sink.
    fn check_sink(&mut self, key: &NodeKey) {
        let NodeKey {
            block,
            instr_idx: Some(idx),
            var,
        } = *key
        else {
            return;
        };

        let block_data = match self.ir.blocks.get(&block) {
            Some(b) => b,
            None => return,
        };

        if idx >= block_data.instructions.len() {
            return;
        }

        let alert = match &block_data.instructions[idx] {
            Instruction::CallStatic { func, args, .. } if self.config.sinks.contains(func) => args
                .iter()
                .position(|a| a == &Operand::Var(var))
                .map(|pos| {
                    format!(
                        "CRITICAL VULNERABILITY: Tainted data reached sink '{}' at argument {}",
                        func, pos
                    )
                }),
            Instruction::CallVirtual {
                method,
                args,
                receiver,
                ..
            } if self.config.sinks.contains(method) => {
                if receiver == &Operand::Var(var) {
                    Some(format!(
                        "CRITICAL VULNERABILITY: Tainted data reached sink '{}' at receiver",
                        method
                    ))
                } else {
                    args.iter().position(|a| a == &Operand::Var(var)).map(|pos| {
                        format!(
                            "CRITICAL VULNERABILITY: Tainted data reached sink '{}' at argument {}",
                            method, pos
                        )
                    })
                }
            }
            _ => None,
        };

        if let Some(alert) = alert
            && !self.alerts.contains(&alert)
        {
            self.alerts.push(alert);
        }
    }
}

// ---------------------------------------------------------------------------
// Statistics / Debug helpers
// ---------------------------------------------------------------------------

impl Svfg {
    /// Returns a summary of the graph for debugging.
    pub fn stats(&self) -> SvfgStats {
        let mut def_count = 0;
        let mut use_count = 0;
        let mut phi_count = 0;
        let mut param_count = 0;
        let mut edge_count = 0;

        for node in self.nodes.values() {
            edge_count += node.successors.len();
            match node.kind {
                NodeKind::InstrDef => def_count += 1,
                NodeKind::InstrUse => use_count += 1,
                NodeKind::Phi => phi_count += 1,
                NodeKind::FormalParam => param_count += 1,
                _ => {}
            }
        }

        SvfgStats {
            node_count: self.nodes.len(),
            def_count,
            use_count,
            phi_count,
            param_count,
            edge_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SvfgStats {
    pub node_count: usize,
    pub def_count: usize,
    pub use_count: usize,
    pub phi_count: usize,
    pub param_count: usize,
    pub edge_count: usize,
}

impl std::fmt::Display for SvfgStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SVFG {{ nodes: {}, defs: {}, uses: {}, phis: {}, params: {}, edges: {} }}",
            self.node_count,
            self.def_count,
            self.use_count,
            self.phi_count,
            self.param_count,
            self.edge_count,
        )
    }
}
