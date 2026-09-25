// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Task 4.2: Bottom-Up Interprocedural Value-Flow (CFTaint-style)
//!
//! This module wires many per-function [`Svfg`]s into one program-level
//! value-flow structure with **real interprocedural edges**:
//!
//! ```text
//!   caller ActualArg  ──▶ callee FormalParam     (parameter passing)
//!   callee FormalRet  ──▶ caller ActualRet       (return value passing)
//! ```
//!
//! ## Bottom-up analysis order
//!
//! The call graph is topologically sorted **leaves first** (DFS post-order).
//! Each function is processed exactly once, in that order:
//!
//! 1. Analyse the function's own SVFG, treating callees as black boxes.
//! 2. Where a callee already has a summary, apply it (relational: "if argument
//!    slot `s` is tainted, the return value becomes tainted").
//! 3. Produce a [`TaintSummary`] and cache it.
//!
//! Because callees are summarised before callers, every function is analysed
//! exactly once, for a corpus with shared utility libraries, the summary is
//! computed once and applied at every call site (the key corpus-scale
//! optimisation from the 4.2 spec).
//!
//! ## Pass-through suppression
//!
//! The single-function SVFG contains conservative `arg-use → dest-def`
//! pass-through edges for *every* call (sound when the callee is unknown).
//! When a callee is known **and** its summary is available, those edges are
//! suppressed for that call site and replaced by the callee's summary, this
//! is strictly more precise (a sanitising callee no longer leaks taint through
//! a local pass-through edge) and matches "apply the callee's summary rather
//! than inlining its analysis".
//!
//! For callees inside reference cycles (where no summary exists yet), the
//! local pass-through edges are kept: the analysis stays sound (may
//! over-approximate) while the SVFG cross-edges still carry exact flow.
//!
//! ## Result artifacts
//!
//! * [`ProgramSvfg::cross_edges`], the explicit interprocedural edge list
//!   (deterministic, serialisable, feeds task 4.5's corpus graph store).
//! * [`ProgramSvfg::summary_registry`], legacy [`FunctionSummary`]s populated
//!   from the relational summaries, plus the richer [`TaintSummary`]s.
//! * [`InterproceduralTaintEngine`], BFS over local + cross edges; alerts are
//!   reported with the function in which the sink lives.

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

use crate::analysis::summary::{FunctionSummary, SummaryRegistry};
use crate::analysis::taint::config::TaintConfig;
use crate::analysis::taint::facts::FactTable;
use crate::graph::callgraph::{CallGraph, CallGraphBuilder};
use crate::graph::svfg::{NodeKey, NodeKind, Svfg, SvfgBuilder};
use crate::ir::function::*;

// ---------------------------------------------------------------------------
// Serialisable structure parts (task 4.5)
// ---------------------------------------------------------------------------

/// Plain-data mirror of [`FunctionEntry`] containing only what the corpus
/// graph store persists: everything derived from source, nothing derived from
/// the rule set.
#[cfg(feature = "serialize")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionParts {
    pub name: String,
    /// A complete copy of the function's IR. At load time the caller-supplied
    /// IR takes precedence; this copy serves structural validation and makes
    /// the store self-contained.
    pub ir: FunctionIR,
    pub svfg: Svfg,
    /// VarId → defining node key. Serialises as a sorted pair list
    /// (`FxHashMap<VarId, _>` is not serde_json-key-compatible).
    #[cfg_attr(feature = "serialize", serde(with = "var_keyed_map"))]
    pub def_site: FxHashMap<VarId, NodeKey>,
    /// Resolved call-site bindings (callee indices are store-stable: they
    /// refer to positions in `functions`, which serialises in order).
    pub bindings: Vec<CallBinding>,
}

/// Plain-data mirror of the program-level graph (see [`ProgramSvfg::to_parts`]).
#[cfg(feature = "serialize")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramSvfgParts {
    /// Functions in store order (deterministic: sorted by name at build time).
    pub functions: Vec<FunctionParts>,
    /// DFS post-order (leaves first) over the call graph, as function indices.
    pub topological_order: Vec<usize>,
    /// `(from_fn, from_node) → [(to_fn, to_node)]`, stored as a sorted pair
    /// list (serde_json needs string map keys, and a sorted Vec is byte-
    /// deterministic for cache hashing). Keys via [`SerKey`].
    pub cross_edges: Vec<(SerKey, Vec<(usize, NodeKey)>)>,
}

/// Serde-friendly key for the cross-edge map: `(function index, node key)`.
#[cfg(feature = "serialize")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SerKey {
    pub f: usize,
    pub k: NodeKey,
}

/// Serde with-helper for `FxHashMap<VarId, V>`: round-trips through a sorted
/// `Vec<(VarId, V)>` (VarId is a newtype over usize; serde_json cannot use it
/// as a map key directly). Sorted = byte-deterministic serialisation.
#[cfg(feature = "serialize")]
mod var_keyed_map {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        map: &FxHashMap<VarId, NodeKey>,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        let mut v: Vec<(VarId, NodeKey)> = map.iter().map(|(k, val)| (*k, *val)).collect();
        v.sort_by_key(|(k, _)| k.0);
        v.serialize(ser)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        de: D,
    ) -> Result<FxHashMap<VarId, NodeKey>, D::Error> {
        Ok(FxHashMap::from_iter(Vec::<(VarId, NodeKey)>::deserialize(
            de,
        )?))
    }
}

// ---------------------------------------------------------------------------
// Relational taint summary
// ---------------------------------------------------------------------------

/// CFTaint-style relational summary: *not* "is this function tainted?" but
/// "if argument slot `s` is tainted, which outputs become tainted?".
///
/// Applied differently at each call site, this gives context sensitivity for
/// free at the summary-application step.
#[derive(Debug, Clone, Default)]
pub struct TaintSummary {
    pub function_name: String,
    /// Indexed by formal parameter slot. `true` if taint introduced at that
    /// parameter can reach any of the function's return values.
    pub param_taints_return: Vec<bool>,
    /// Indexed by formal parameter slot. `true` if taint introduced at that
    /// parameter can reach a configured sink inside this function.
    pub param_reaches_sink: Vec<bool>,
}

impl TaintSummary {
    /// True if taint at parameter slot `slot` flows to the return value.
    pub fn taints_return(&self, slot: usize) -> bool {
        self.param_taints_return.get(slot).copied().unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Call-site bindings
// ---------------------------------------------------------------------------

/// One resolved call site inside a function.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct CallBinding {
    /// Sentinel key identifying the call instruction (var = `VarId(usize::MAX)`).
    pub call_site: NodeKey,
    /// Indices of resolved callees in [`ProgramSvfg::functions`], sorted and
    /// deduped. Empty when every target is external/unresolved, dynamic
    /// dispatch and callbacks may resolve to several callees at once; the
    /// cross-edge installer links all of them (sound: any may run).
    /// Resolution comes from [`callgraph`] (aliases, receiver classes,
    /// higher-order callbacks); bare-name matching is the fallback.
    pub callees: Vec<usize>,
    /// Best-known callee name for diagnostics and external fact lookup
    /// (source/sink classification); `<indirect>` when unresolved.
    pub callee_name: String,
    /// `(arg use-node key, formal parameter slot)`. For virtual calls the
    /// receiver occupies slot 0 and explicit args start at slot 1.
    pub args: Vec<(NodeKey, usize)>,
    /// The caller's `ActualRet` def node, if the call has a destination.
    pub ret_node: Option<NodeKey>,
}

// ---------------------------------------------------------------------------
// Per-function entry
// ---------------------------------------------------------------------------

/// Everything the interprocedural layer knows about one function.
pub struct FunctionEntry<'a> {
    pub name: String,
    pub ir: &'a FunctionIR,
    pub svfg: Svfg,
    /// Maps each VarId to the SVFG node that defines it.
    pub def_site: FxHashMap<VarId, NodeKey>,
    /// Resolved call sites in this function (discovery order is deterministic:
    /// blocks sorted by id, instructions by index).
    pub bindings: Vec<CallBinding>,
    /// Arg use-node key → `(binding index, formal slot)`.
    pub arg_slots: FxHashMap<NodeKey, (usize, usize)>,
    /// Filled bottom-up during [`ProgramSvfg::compute_summaries`].
    pub summary: Option<TaintSummary>,
}

// ---------------------------------------------------------------------------
// Program-level SVFG
// ---------------------------------------------------------------------------

/// The whole-program value-flow structure: per-function SVFGs linked by
/// explicit interprocedural edges, with bottom-up summaries cached per fn.
pub struct ProgramSvfg<'a> {
    /// Deterministic order: sorted by function name.
    pub functions: Vec<FunctionEntry<'a>>,
    func_index: FxHashMap<String, usize>,
    /// DFS post-order (leaves first) over the call graph.
    pub topological_order: Vec<usize>,
    /// `(caller_fn, from_node) → [(callee_fn, to_node)]`, sorted & deduped.
    /// Parameter edges: ActualArg → FormalParam. Return edges: FormalRet → ActualRet.
    pub cross_edges: FxHashMap<(usize, NodeKey), Vec<(usize, NodeKey)>>,
    /// Local pass-through edges `(fn, from, to)` suppressed because a summary
    /// replaced them. Consulted by both the summary BFS and the taint engine.
    suppressed: FxHashSet<(usize, NodeKey, NodeKey)>,
    /// Legacy summaries (from `summary.rs`) populated from `TaintSummary`s.
    pub summary_registry: SummaryRegistry,
}

impl<'a> ProgramSvfg<'a> {
    /// Build the whole-program structure from a set of SSA-converted
    /// `FunctionIR`s. `config` is needed at build time because summaries must
    /// know which calls are sinks.
    pub fn new(irs: &FxHashMap<String, &'a FunctionIR>, config: &TaintConfig) -> Self {
        // 1. Build per-function SVFGs (deterministic name order).
        let mut names: Vec<&String> = irs.keys().collect();
        names.sort();
        let functions: Vec<FunctionEntry<'a>> = names
            .iter()
            .map(|n| {
                let ir = irs[*n];
                let (svfg, def_site) = SvfgBuilder::new(ir).build();
                FunctionEntry {
                    name: (*n).clone(),
                    ir,
                    svfg,
                    def_site,
                    bindings: Vec::new(),
                    arg_slots: FxHashMap::default(),
                    summary: None,
                }
            })
            .collect();
        let func_index: FxHashMap<String, usize> = functions
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name.clone(), i))
            .collect();

        let mut prog = Self {
            functions,
            func_index,
            topological_order: Vec::new(),
            cross_edges: FxHashMap::default(),
            suppressed: FxHashSet::default(),
            summary_registry: SummaryRegistry::new(),
        };

        // Resolve the call graph once (aliases, receiver-class methods,
        // higher-order callbacks) and use it for all binding discovery.
        let mut borrowed: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        for (name, ir) in irs.iter() {
            borrowed.insert(name.clone(), ir);
        }
        let callgraph = CallGraphBuilder::new(&borrowed).build();

        prog.discover_bindings(&callgraph);
        prog.compute_topological_order();
        prog.compute_summaries(config);
        prog.install_cross_edges();
        prog.install_heap_cross_edges();
        prog
    }

    /// Look up a function's index by name.
    pub fn function_index(&self, name: &str) -> Option<usize> {
        self.func_index.get(name).copied()
    }

    // -----------------------------------------------------------------------
    // Step 1: discover call-site bindings
    // -----------------------------------------------------------------------

    fn discover_bindings(&mut self, callgraph: &CallGraph) {
        for fi in 0..self.functions.len() {
            let ir = self.functions[fi].ir;
            let fname = self.functions[fi].name.clone();
            let mut bindings: Vec<CallBinding> = Vec::new();
            let mut arg_slots: FxHashMap<NodeKey, (usize, usize)> = FxHashMap::default();

            let mut block_ids: Vec<BlockId> = ir.blocks.keys().copied().collect();
            block_ids.sort_by_key(|b| b.0);

            for &block in &block_ids {
                let bdata = &ir.blocks[&block];
                for (idx, instr) in bdata.instructions.iter().enumerate() {
                    // Call-shape extraction. `CallPointer` sites participate
                    // when the callgraph resolved them (aliases, callbacks);
                    // unresolved ones keep the sound local pass-through
                    // fallback instead of a binding.
                    let (callee_name, receiver, args, dest): (
                        String,
                        Option<&Operand>,
                        &Vec<Operand>,
                        Option<VarId>,
                    ) = match instr {
                        Instruction::CallStatic {
                            func, args, dest, ..
                        } => (func.clone(), None, args, *dest),
                        Instruction::CallVirtual {
                            method,
                            receiver,
                            args,
                            dest,
                            ..
                        } => (method.clone(), Some(receiver), args, *dest),
                        Instruction::CallPointer { args, dest, .. } => {
                            let targets = callgraph.targets_at(&fname, block.0, idx);
                            if targets.is_empty() {
                                continue; // unresolved indirect → sound fallback
                            }
                            let name = targets
                                .iter()
                                .find_map(|t| t.internal_key())
                                .unwrap_or_else(|| "<indirect>".to_string());
                            (name, None, args, *dest)
                        }
                        _ => continue,
                    };

                    let call_site = NodeKey::instr(block, idx, VarId(usize::MAX));
                    let mut bargs: Vec<(NodeKey, usize)> = Vec::new();
                    let mut slot = 0usize;
                    if let Some(Operand::Var(r)) = receiver {
                        // Virtual receiver occupies formal slot 0.
                        bargs.push((NodeKey::instr(block, idx, *r), 0));
                        slot = 1;
                    }
                    for a in args {
                        if let Operand::Var(v) = a {
                            bargs.push((NodeKey::instr(block, idx, *v), slot));
                        }
                        slot += 1;
                    }
                    let ret_node = dest.map(|d| NodeKey::instr(block, idx, d));

                    // Resolve targets: callgraph first (aliases, class-
                    // qualified methods, callbacks), bare-name fallback.
                    let mut callees: Vec<usize> = callgraph
                        .targets_at(&fname, block.0, idx)
                        .iter()
                        .filter_map(|t| t.internal_key())
                        .filter_map(|k| self.func_index.get(&k).copied())
                        .collect();
                    if callees.is_empty()
                        && let Some(gi) = self.func_index.get(&callee_name).copied()
                    {
                        callees.push(gi);
                    }
                    callees.sort_unstable();
                    callees.dedup();

                    bindings.push(CallBinding {
                        call_site,
                        callees,
                        callee_name,
                        args: bargs,
                        ret_node,
                    });
                    let bi = bindings.len() - 1;
                    for (k, s) in &bindings[bi].args {
                        arg_slots.insert(*k, (bi, *s));
                    }
                }
            }

            self.functions[fi].bindings = bindings;
            self.functions[fi].arg_slots = arg_slots;
        }
    }

    // -----------------------------------------------------------------------
    // Step 2: bottom-up topological order (leaves first)
    // -----------------------------------------------------------------------

    fn compute_topological_order(&mut self) {
        let n = self.functions.len();
        let mut state = vec![0u8; n]; // 0 = unvisited, 1 = in progress, 2 = done
        let mut order = Vec::with_capacity(n);

        for fi in 0..n {
            self.dfs_topo(fi, &mut state, &mut order);
        }
        self.topological_order = order;
    }

    fn dfs_topo(&self, fi: usize, state: &mut [u8], order: &mut Vec<usize>) {
        if state[fi] != 0 {
            return; // done, or back-edge into an in-progress function (cycle)
        }
        state[fi] = 1;
        // Visit callees in deterministic order.
        let mut callees: Vec<usize> = self.functions[fi]
            .bindings
            .iter()
            .flat_map(|b| b.callees.iter().copied())
            .collect();
        callees.sort_unstable();
        callees.dedup();
        for gi in callees {
            self.dfs_topo(gi, state, order);
        }
        state[fi] = 2;
        order.push(fi);
    }

    // -----------------------------------------------------------------------
    // Step 3: bottom-up summaries (one analysis per function)
    // -----------------------------------------------------------------------

    fn compute_summaries(&mut self, config: &TaintConfig) {
        for &fi in &self.topological_order {
            // Suppress local pass-through edges for callees that already have
            // summaries (their relational summary replaces the local edge).
            let suppressible: Vec<(NodeKey, NodeKey)> = self.functions[fi]
                .bindings
                .iter()
                .filter(|b| {
                    b.callees
                        .iter()
                        .any(|&gi| self.functions[gi].summary.is_some())
                })
                .filter_map(|b| b.ret_node.map(|r| (b, r)))
                .flat_map(|(b, r)| b.args.iter().map(move |(a, _)| (*a, r)))
                .collect();
            for (from, to) in suppressible {
                self.suppressed.insert((fi, from, to));
            }

            let summary = self.compute_one_summary(fi, config);
            let legacy = FunctionSummary {
                function_name: summary.function_name.clone(),
                returns_tainted_by_params: summary
                    .param_taints_return
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| **t)
                    .map(|(i, _)| i)
                    .collect(),
                mutates_param_fields: FxHashMap::default(),
                sinks_params: summary
                    .param_reaches_sink
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| **s)
                    .map(|(i, _)| i)
                    .collect(),
            };
            self.summary_registry.register(legacy);
            self.functions[fi].summary = Some(summary);
        }
    }

    /// Compositional summary for one function: BFS restricted to the
    /// function's *own* graph; at call sites with summarised callees, apply
    /// the callee summary instead of crossing into it.
    fn compute_one_summary(&self, fi: usize, config: &TaintConfig) -> TaintSummary {
        let fe = &self.functions[fi];
        let nparams = fe.ir.parameters.len();
        let mut param_taints_return = vec![false; nparams];
        let mut param_reaches_sink = vec![false; nparams];

        for p in 0..nparams {
            let Some(&seed) = fe.def_site.get(&fe.ir.parameters[p]) else {
                continue;
            };
            let mut visited: FxHashSet<NodeKey> = FxHashSet::default();
            visited.insert(seed);
            let mut queue: VecDeque<NodeKey> = VecDeque::new();
            queue.push_back(seed);

            while let Some(cur) = queue.pop_front() {
                let node = match fe.svfg.node(&cur) {
                    Some(n) => n,
                    None => continue,
                };

                if node.kind == NodeKind::FormalRet {
                    param_taints_return[p] = true;
                }
                if sink_alert(fe.ir, config, &cur).is_some() {
                    param_reaches_sink[p] = true;
                }

                // Apply a summarised callee at an ActualArg node instead of
                // crossing into the callee graph (compositional step).
                if let Some(&(bi, slot)) = fe.arg_slots.get(&cur) {
                    let binding = &fe.bindings[bi];
                    for &gi in &binding.callees {
                        if let Some(sum) = &self.functions[gi].summary
                            && sum.taints_return(slot)
                            && let Some(ret) = binding.ret_node
                            && visited.insert(ret)
                        {
                            queue.push_back(ret);
                            break; // ret already queued
                        }
                    }
                }

                for succ in node.successors() {
                    if self.suppressed.contains(&(fi, cur, succ)) {
                        continue;
                    }
                    if is_sanitizer_use(fe.ir, config, &succ) {
                        continue;
                    }
                    if visited.insert(succ) {
                        queue.push_back(succ);
                    }
                }
            }
        }

        TaintSummary {
            function_name: fe.name.clone(),
            param_taints_return,
            param_reaches_sink,
        }
    }

    // -----------------------------------------------------------------------
    // Step 4: explicit interprocedural edges
    // -----------------------------------------------------------------------

    fn install_cross_edges(&mut self) {
        for fi in 0..self.functions.len() {
            let bindings = self.functions[fi].bindings.clone();
            for b in &bindings {
                for &gi in &b.callees {
                    let ge = &self.functions[gi];

                    // Parameter passing: ActualArg(use node) → FormalParam(slot).
                    for (arg_key, slot) in &b.args {
                        if let Some(&param_var) = ge.ir.parameters.get(*slot)
                            && let Some(&param_key) = ge.def_site.get(&param_var)
                        {
                            self.cross_edges
                                .entry((fi, *arg_key))
                                .or_default()
                                .push((gi, param_key));
                        }
                    }

                    // Return passing: every FormalRet node of the callee feeds the
                    // caller's ActualRet def node. Keyed by the CALLEE (the from-node
                    // lives there) so the map always stores edges in value-flow
                    // direction: FormalRet → ActualRet.
                    if let Some(ret) = b.ret_node {
                        let mut formal_rets: Vec<NodeKey> = ge
                            .svfg
                            .nodes
                            .values()
                            .filter(|n| n.kind == NodeKind::FormalRet)
                            .map(|n| n.key)
                            .collect();
                        formal_rets.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
                        for fr in formal_rets {
                            self.cross_edges
                                .entry((gi, fr))
                                .or_default()
                                .push((fi, ret));
                        }
                    }
                }
            }
        }

        // Deterministic edge lists (sorted + deduped).
        for edges in self.cross_edges.values_mut() {
            edges.sort_by_key(|(f, k)| (*f, k.block.0, k.instr_idx, k.var.0));
            edges.dedup();
        }
    }

    /// Local successors of a node, minus suppressed pass-through edges.
    pub fn local_successors(&self, fi: usize, key: &NodeKey) -> Vec<NodeKey> {
        match self.functions[fi].svfg.node(key) {
            Some(n) => n
                .successors()
                .into_iter()
                .filter(|s| !self.suppressed.contains(&(fi, *key, *s)))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Local predecessors of a node, minus suppressed pass-through edges
    /// (mirror of [`ProgramSvfg::local_successors`] for backward traversal).
    pub fn local_predecessors(&self, fi: usize, key: &NodeKey) -> Vec<NodeKey> {
        match self.functions[fi].svfg.node(key) {
            Some(n) => n
                .predecessors()
                .into_iter()
                .filter(|p| !self.suppressed.contains(&(fi, *p, *key)))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Interprocedural successors of a node (cross edges out).
    pub fn inter_successors(&self, fi: usize, key: &NodeKey) -> Vec<(usize, NodeKey)> {
        self.cross_edges
            .get(&(fi, *key))
            .cloned()
            .unwrap_or_default()
    }

    // -----------------------------------------------------------------------
    // Step 4b: field-sensitive cross-function heap edges
    // -----------------------------------------------------------------------

    /// Install heap value-flow edges *across* call boundaries:
    ///
    /// ```text
    ///   caller: obj.data = src          callee: return p.data
    ///           call f(obj)      ⟶            (load of p.data)
    /// ```
    ///
    /// Scalar parameter/return edges (Step 4) carry direct values, but a
    /// field *written into an object* before a call and *read from it*
    /// (before or during the call) has no edge, the classic missed bug
    /// `obj.data = source(); sink(obj.data)` split across two functions.
    ///
    /// Two directions are installed per call site, per matching field:
    ///
    ///  1. **caller-store → callee-load**: a `StoreField` in the caller whose
    ///     base is the same object as an actual argument feeds every
    ///     `LoadField` in the callee whose base is that callee parameter.
    ///  2. **callee-store → caller-load**: a `StoreField` in the callee on
    ///     its parameter object feeds every `LoadField` in the caller whose
    ///     base is the corresponding actual argument (covers callee-side
    ///     mutation the caller reads afterwards).
    ///  3. **callee-store → sibling-load**: two callees that receive the
    ///     *same argument object* (same arg variable at the same call-site
    ///     root) are linked: a store in one feeds loads of the same field in
    ///     the other. This is the write-then-read pipeline,
    ///     `handler(obj, input)` calling `writer(obj, input)` then
    ///     `reader(obj)`, where neither the caller nor either callee alone
    ///     completes the flow.
    ///
    /// Object identity is anchored on SSA roots (`object_roots`): parameters
    /// and allocation sites. A field matches on exact name, or when either
    /// side is the element wildcard `"*"`. Edges enter the regular
    /// `cross_edges` map, so both the forward engine and the demand-driven
    /// backward engine (via its reverse index) traverse them unchanged.
    fn install_heap_cross_edges(&mut self) {
        // Per-function caches, computed lazily.
        let mut roots: Vec<Option<FxHashMap<VarId, ObjectRoot>>> = vec![None; self.functions.len()];
        let mut field_ops: Vec<Option<(Vec<FieldStore>, Vec<FieldLoad>)>> =
            vec![None; self.functions.len()];

        let mut new_edges: Vec<((usize, NodeKey), (usize, NodeKey))> = Vec::new();

        for fi in 0..self.functions.len() {
            let (c_stores, c_loads) = {
                if field_ops[fi].is_none() {
                    field_ops[fi] = Some(collect_field_ops(self.functions[fi].ir));
                }
                field_ops[fi].as_ref().unwrap().clone()
            };
            // Sibling heap edges (direction 3) need only the callees' field
            // ops, so an empty caller set must not skip the whole loop:
            // directions 1-2 iterate empty vectors harmlessly.
            if self.functions[fi].bindings.is_empty() {
                continue;
            }
            let bindings = self.functions[fi].bindings.clone();
            for b in &bindings {
                for &gi in &b.callees {
                    let (g_stores, g_loads) = {
                        if field_ops[gi].is_none() {
                            field_ops[gi] = Some(collect_field_ops(self.functions[gi].ir));
                        }
                        field_ops[gi].as_ref().unwrap().clone()
                    };

                    for (arg_key, slot) in &b.args {
                        let arg_var = arg_key.var;
                        if roots[fi].is_none() {
                            roots[fi] = Some(object_roots(self.functions[fi].ir));
                        }
                        let arg_root = match roots[fi].as_ref().unwrap().get(&arg_var).copied() {
                            Some(r) => r,
                            None => continue,
                        };
                        let param_root = ObjectRoot::Param(*slot);

                        // 1. caller store → callee load.
                        for st in &c_stores {
                            if roots[fi].as_ref().unwrap().get(&st.base).copied() != Some(arg_root)
                            {
                                continue;
                            }
                            for ld in &g_loads {
                                if roots[gi].is_none() {
                                    roots[gi] = Some(object_roots(self.functions[gi].ir));
                                }
                                if roots[gi].as_ref().unwrap().get(&ld.base).copied()
                                    != Some(param_root)
                                {
                                    continue;
                                }
                                if !fields_match(st.field.as_str(), ld.field.as_str()) {
                                    continue;
                                }
                                new_edges.push(((fi, st.src_use), (gi, ld.dest)));
                            }
                        }

                        // 2. callee store → caller load.
                        for st in &g_stores {
                            if roots[gi].is_none() {
                                roots[gi] = Some(object_roots(self.functions[gi].ir));
                            }
                            if roots[gi].as_ref().unwrap().get(&st.base).copied()
                                != Some(param_root)
                            {
                                continue;
                            }
                            for ld in &c_loads {
                                if roots[fi].as_ref().unwrap().get(&ld.base).copied()
                                    != Some(arg_root)
                                {
                                    continue;
                                }
                                if !fields_match(st.field.as_str(), ld.field.as_str()) {
                                    continue;
                                }
                                new_edges.push(((gi, st.src_use), (fi, ld.dest)));
                            }
                        }

                        // 3. callee store → sibling callee load. Every OTHER
                        // binding of this call site whose arg at the SAME
                        // slot carries the same object root receives the
                        // stores this callee makes on that parameter object.
                        // Pairwise across all bindings of `fi`, so both
                        // directions emerge from the (writer, reader) loop.
                        for bj in self.functions[fi].bindings.iter() {
                            if bj.call_site == b.call_site {
                                continue; // same binding: directions 1+2 cover it
                            }
                            let Some(&(_, sj)) = bj
                                .args
                                .iter()
                                .find(|(k, s)| *s == *slot && k.var == arg_var)
                            else {
                                continue; // sibling does not bind this object here
                            };
                            for &gj in &bj.callees {
                                if gj == gi {
                                    continue;
                                }
                                if field_ops[gj].is_none() {
                                    field_ops[gj] = Some(collect_field_ops(self.functions[gj].ir));
                                }
                                if roots[gj].is_none() {
                                    roots[gj] = Some(object_roots(self.functions[gj].ir));
                                }
                                let sibling_root = ObjectRoot::Param(sj);
                                for st in &g_stores {
                                    if roots[gi].as_ref().unwrap().get(&st.base).copied()
                                        != Some(param_root)
                                    {
                                        continue;
                                    }
                                    for ld in &field_ops[gj].as_ref().unwrap().1 {
                                        if roots[gj].as_ref().unwrap().get(&ld.base).copied()
                                            != Some(sibling_root)
                                        {
                                            continue;
                                        }
                                        if !fields_match(st.field.as_str(), ld.field.as_str()) {
                                            continue;
                                        }
                                        new_edges.push(((gi, st.src_use), (gj, ld.dest)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for ((ff, fk), (tf, tk)) in new_edges {
            self.cross_edges.entry((ff, fk)).or_default().push((tf, tk));
        }

        // Re-establish determinism over the whole map (heap edges join the
        // parameter/return edges installed by Step 4).
        for edges in self.cross_edges.values_mut() {
            edges.sort_by_key(|(f, k)| (*f, k.block.0, k.instr_idx, k.var.0));
            edges.dedup();
        }
    }

    // -----------------------------------------------------------------------
    // Serialisation support (task 4.5): structure ↔ parts
    // -----------------------------------------------------------------------

    /// Extract the *config-independent* structure of this program graph as
    /// plain data, ready for serialisation (task 4.5's corpus graph store).
    ///
    /// What is captured: per-function SVFGs (nodes, kinds, edges), call-site
    /// bindings, cross edges and the bottom-up topological order, everything
    /// determined solely by the source code.
    ///
    /// What is deliberately NOT captured: [`TaintSummary`]s and the
    /// pass-through suppression set. Both depend on the **rule set**
    /// (sources/sinks/sanitizers), not the source, they are recomputed at
    /// load time from the freshly applied config. This is precisely what
    /// lets a new sink configuration re-scan a cached repo without re-running
    /// lower → SSA → SVFG build.
    #[cfg(feature = "serialize")]
    pub fn to_parts(&self) -> ProgramSvfgParts {
        ProgramSvfgParts {
            functions: self
                .functions
                .iter()
                .map(|f| FunctionParts {
                    name: f.name.clone(),
                    ir: f.ir.clone(),
                    svfg: f.svfg.clone(),
                    def_site: f.def_site.clone(),
                    bindings: f.bindings.clone(),
                })
                .collect(),
            topological_order: self.topological_order.clone(),
            cross_edges: {
                let mut v: Vec<(SerKey, Vec<(usize, NodeKey)>)> = self
                    .cross_edges
                    .iter()
                    .map(|((f, k), edges)| (SerKey { f: *f, k: *k }, edges.clone()))
                    .collect();
                v.sort_by_key(|(sk, _)| (sk.f, sk.k.block.0, sk.k.instr_idx, sk.k.var.0));
                v
            },
        }
    }

    /// Rebuild a [`ProgramSvfg`] from serialised parts plus a **new** taint
    /// config. Summaries and pass-through suppression are recomputed from the
    /// supplied config (not restored from disk), so the returned graph answers
    /// queries under the new rule set.
    #[cfg(feature = "serialize")]
    pub fn from_parts(
        parts: ProgramSvfgParts,
        irs: &FxHashMap<String, &'a FunctionIR>,
        config: &TaintConfig,
    ) -> Self {
        let ProgramSvfgParts {
            functions,
            topological_order,
            cross_edges,
        } = parts;

        // Rebuild function entries. The SVFG/bindings/def-sites come from the
        // store; the IR reference must point at the caller-supplied IRs (the
        // store's copies are structurally identical, but predicates like
        // `is_source` consult the live IR).
        //
        // Functions absent from `irs` fall back to their stored IR copy, leaked
        // to satisfy the `&'a` lifetime (program graphs are long-lived by
        // design; the leak is one copy per function per cache load, and the
        // self-contained-cache path has no live IR to hold anyway).
        let functions: Vec<FunctionEntry> = functions
            .into_iter()
            .map(|fp| {
                let name = fp.name.clone();
                let ir: &FunctionIR = match irs.get(&name) {
                    Some(&live) => live,
                    None => Box::leak(Box::new(fp.ir)),
                };
                let arg_slots: FxHashMap<NodeKey, (usize, usize)> = fp
                    .bindings
                    .iter()
                    .enumerate()
                    .flat_map(|(bi, b)| b.args.iter().map(move |(k, s)| (*k, (bi, *s))))
                    .collect();
                FunctionEntry {
                    name,
                    ir,
                    svfg: fp.svfg,
                    def_site: fp.def_site,
                    bindings: fp.bindings,
                    arg_slots,
                    summary: None,
                }
            })
            .collect();

        // Re-derive the name → index map (store order is preserved exactly).
        let func_index: FxHashMap<String, usize> = functions
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name.clone(), i))
            .collect();

        let mut prog = Self {
            functions,
            func_index,
            topological_order,
            cross_edges: cross_edges
                .into_iter()
                .map(|(sk, edges)| ((sk.f, sk.k), edges))
                .collect(),
            suppressed: FxHashSet::default(),
            summary_registry: SummaryRegistry::new(),
        };

        // Recompute the config-dependent state under the NEW rule set.
        prog.compute_summaries(config);
        prog
    }
}

// ---------------------------------------------------------------------------
// Field-sensitive cross-function heap modeling
// ---------------------------------------------------------------------------

/// SSA-anchored identity of an object: where it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ObjectRoot {
    /// A formal parameter (slot order = parameter index).
    Param(usize),
    /// An `Allocate` instruction (block, idx), a locally created object.
    Alloc(BlockId, usize),
    /// The initial/global memory state (module-level objects).
    Global,
}

/// A field write: `base.field = src` (or `base[field] = src`).
#[derive(Debug, Clone)]
pub(crate) struct FieldStore {
    pub base: VarId,
    pub field: String,
    /// The SVFG *use* node of the stored value inside the store instruction,
    /// the point where value-flow leaves the writer.
    pub src_use: NodeKey,
}

/// A field read: `dest = base.field` (or `dest = base[field]`).
#[derive(Debug, Clone)]
pub(crate) struct FieldLoad {
    pub base: VarId,
    pub field: String,
    /// The SVFG *def* node of the destination, the point where value-flow
    /// enters the reader.
    pub dest: NodeKey,
}

/// Do two field names denote the same heap slot? Exact match, or either side
/// is the dynamic-element wildcard `"*"` (an unknown index can hit any slot).
pub(crate) fn fields_match(a: &str, b: &str) -> bool {
    a == b || a == "*" || b == "*"
}

/// Map every variable to the SSA root of the object it denotes (parameter,
/// allocation site, or global state). Follows `Assign`/`Cast` chains; nothing
/// else propagates object identity (a `LoadField` result is a *field value*,
/// not the object itself, treating it as such would be unsound in the other
/// direction only for receiver-chaining, which `member_access_path` already
/// handles for sources).
pub(crate) fn object_roots(ir: &FunctionIR) -> FxHashMap<VarId, ObjectRoot> {
    let mut map: FxHashMap<VarId, ObjectRoot> = FxHashMap::default();

    for (slot, &p) in ir.parameters.iter().enumerate() {
        map.insert(p, ObjectRoot::Param(slot));
    }
    map.insert(ir.initial_memory_state, ObjectRoot::Global);

    let mut block_ids: Vec<BlockId> = ir.blocks.keys().copied().collect();
    block_ids.sort_by_key(|b| b.0);
    for &b in &block_ids {
        let blk = &ir.blocks[&b];
        for (idx, instr) in blk.instructions.iter().enumerate() {
            match instr {
                Instruction::Allocate { dest, .. } => {
                    map.insert(*dest, ObjectRoot::Alloc(b, idx));
                }
                Instruction::Assign {
                    dest,
                    src: Operand::Var(s),
                }
                | Instruction::Cast {
                    dest,
                    src: Operand::Var(s),
                    ..
                } => {
                    if let Some(r) = map.get(s).copied() {
                        // First writer wins (SSA: one def per var; multiple
                        // entries can only come from the parameter seeding,
                        // which is authoritative).
                        map.entry(*dest).or_insert(r);
                    }
                }
                // An object literal `{}` / `[]` lowers to `dest = Unknown`
                // (or a literal) with no Allocate, it is still a *fresh*
                // object. Anchor it as its own allocation site so field
                // stores into it participate in heap flow.
                Instruction::Assign {
                    dest,
                    src: Operand::Unknown,
                }
                | Instruction::Assign {
                    dest,
                    src: Operand::Null,
                }
                | Instruction::Assign {
                    dest,
                    src: Operand::StringLiteral(_),
                }
                | Instruction::Assign {
                    dest,
                    src: Operand::IntLiteral(_),
                }
                | Instruction::Assign {
                    dest,
                    src: Operand::BoolLiteral(_),
                } => {
                    map.entry(*dest).or_insert(ObjectRoot::Alloc(b, idx));
                }
                _ => {}
            }
        }
    }
    map
}

/// Collect all field stores/loads of a function with their SVFG attachment
/// points (store: the src *use* node; load: the dest *def* node).
pub(crate) fn collect_field_ops(ir: &FunctionIR) -> (Vec<FieldStore>, Vec<FieldLoad>) {
    let mut stores = Vec::new();
    let mut loads = Vec::new();

    let mut block_ids: Vec<BlockId> = ir.blocks.keys().copied().collect();
    block_ids.sort_by_key(|b| b.0);
    for &b in &block_ids {
        let blk = &ir.blocks[&b];
        for (idx, instr) in blk.instructions.iter().enumerate() {
            match instr {
                Instruction::StoreField {
                    base,
                    field,
                    src: Operand::Var(sv),
                    ..
                } => {
                    stores.push(FieldStore {
                        base: *base,
                        field: field.clone(),
                        src_use: NodeKey::instr(b, idx, *sv),
                    });
                }
                Instruction::StoreElement {
                    base, index, src, ..
                } => {
                    if let (Operand::Var(sv), Some(field)) = (src, elem_field(index)) {
                        stores.push(FieldStore {
                            base: *base,
                            field,
                            src_use: NodeKey::instr(b, idx, *sv),
                        });
                    }
                }
                Instruction::LoadField {
                    dest, base, field, ..
                } => {
                    loads.push(FieldLoad {
                        base: *base,
                        field: field.clone(),
                        dest: NodeKey::instr(b, idx, *dest),
                    });
                }
                Instruction::LoadElement {
                    dest, base, index, ..
                } => {
                    if let Some(field) = elem_field(index) {
                        loads.push(FieldLoad {
                            base: *base,
                            field,
                            dest: NodeKey::instr(b, idx, *dest),
                        });
                    }
                }
                _ => {}
            }
        }
    }
    (stores, loads)
}

/// Field name for an element index operand: string literal verbatim,
/// anything dynamic is the wildcard.
fn elem_field(index: &Operand) -> Option<String> {
    Some(match index {
        Operand::StringLiteral(s) => s.clone(),
        _ => "*".to_string(),
    })
}

// ---------------------------------------------------------------------------
// Shared IR predicates (same semantics as the single-function engine)
// ---------------------------------------------------------------------------

/// True if the node is a definition whose instruction is a configured source.
pub(crate) fn is_source(ir: &FunctionIR, config: &TaintConfig, key: &NodeKey) -> bool {
    let NodeKey {
        block,
        instr_idx: Some(idx),
        var,
    } = *key
    else {
        return false;
    };
    let Some(b) = ir.blocks.get(&block) else {
        return false;
    };
    if idx >= b.instructions.len() {
        // Sentinel node: function parameter. A parameter whose conventional
        // name is a configured source (e.g. "req", "input", "body") is itself
        // a taint source, the language spec's request_param_names.
        if let Some(meta) = ir.var_metadata.get(&var)
            && let Some(name) = &meta.source_name
        {
            return config.sources.contains(name);
        }
        return false;
    }
    match &b.instructions[idx] {
        Instruction::CallStatic {
            func,
            dest: Some(d),
            ..
        } => config.sources.contains(func) && *d == var,
        Instruction::CallVirtual {
            method,
            dest: Some(d),
            ..
        } => config.sources.contains(method) && *d == var,
        Instruction::LoadField { base, field, .. } => {
            // Member-expression source: `req.body` lowers to LoadField
            // chains, the *dest* of each link has no source_name, so the
            // access path must be reconstructed by walking the base chain
            // through the IR until a named root var is reached:
            //   LoadField(dest2, base1, "args"); LoadField(dest1, req, "body")
            // reconstructs "req.body.args".
            let path = member_access_path(ir, *base, field);
            let root = path.split('.').next().unwrap_or("");
            #[allow(clippy::disallowed_methods)]
            {
                if std::env::var("FRENSdbg_IS_SOURCE").is_ok() {
                    eprintln!(
                        "[is_source] path={path:?} hit={}",
                        config.sources.contains(&path)
                    );
                }
            }
            config.sources.contains(&path)
                || config.sources.contains(root)
                // Prefix semantics: a configured source that is a path
                // prefix ("req.body" covers "req.body.args") taints the
                // whole subtree.
                || config
                    .sources
                    .iter()
                    .any(|s| path.starts_with(s.as_str()) && path.as_bytes().get(s.len()) == Some(&b'.'))
        }
        _ => false,
    }
}

/// Reconstruct a member-access path like `req.body.args` by walking
/// LoadField base chains backwards to a named root variable. Bounded to
/// prevent pathological IR from looping (alias chains are acyclic by SSA
/// construction, but the bound is cheap insurance).
pub(crate) fn member_access_path(ir: &FunctionIR, mut base: VarId, last_field: &str) -> String {
    let mut segments = vec![last_field.to_string()];
    for _ in 0..16 {
        match ir
            .var_metadata
            .get(&base)
            .and_then(|m| m.source_name.clone())
        {
            Some(name) => {
                segments.push(name);
                break;
            }
            None => {
                // Find the instruction that defines `base`.
                let mut found = None;
                'outer: for b in ir.blocks.values() {
                    for (idx, instr) in b.instructions.iter().enumerate() {
                        if let Instruction::LoadField {
                            dest: d,
                            base: b2,
                            field,
                            ..
                        } = instr
                            && *d == base
                        {
                            found = Some((*b2, field.clone(), idx));
                            break 'outer;
                        }
                    }
                }
                match found {
                    Some((b2, field, _)) => {
                        segments.push(field);
                        base = b2;
                    }
                    None => break,
                }
            }
        }
    }
    segments.reverse();
    // Drop leading mem-state artifacts if any leaked in.
    while segments.len() > 1 && segments[0] == "InitialHeapState" {
        segments.remove(0);
    }
    segments.join(".")
}

/// True if the node is a use inside a configured sanitizer call.
pub(crate) fn is_sanitizer_use(ir: &FunctionIR, config: &TaintConfig, key: &NodeKey) -> bool {
    let NodeKey {
        block,
        instr_idx: Some(idx),
        ..
    } = *key
    else {
        return false;
    };
    let Some(b) = ir.blocks.get(&block) else {
        return false;
    };
    if idx >= b.instructions.len() {
        return false;
    }
    match &b.instructions[idx] {
        Instruction::CallStatic { func, .. } => config.sanitizers.contains(func),
        Instruction::CallVirtual { method, .. } => config.sanitizers.contains(method),
        _ => false,
    }
}

/// Fact-table-aware sanitizer check: configured sanitizers PLUS any call with
/// a [`SanitizerFact`] in the table (e.g. `.replace()`, `.test()` guard
/// methods learned from `frensense-lang` tables or a bundle).
pub(crate) fn is_sanitizer_use_with_facts(
    ir: &FunctionIR,
    config: &TaintConfig,
    facts: &FactTable,
    key: &NodeKey,
) -> bool {
    if is_sanitizer_use(ir, config, key) {
        return true;
    }
    let NodeKey {
        block,
        instr_idx: Some(idx),
        ..
    } = *key
    else {
        return false;
    };
    let Some(b) = ir.blocks.get(&block) else {
        return false;
    };
    if idx >= b.instructions.len() {
        return false;
    }
    match &b.instructions[idx] {
        Instruction::CallStatic { func, .. } => facts.sanitizer_fact(func).is_some(),
        Instruction::CallVirtual {
            method, receiver, ..
        } => {
            if facts.sanitizer_fact(method).is_some() {
                return true;
            }
            // Session-store accessor trust: `store.get(token)` returns a
            // server-issued session object (undefined for unknown tokens),
            // so values derived from its result are not attacker-controlled.
            // Receiver-aware: only declared session roots qualify.
            if let Operand::Var(r) = receiver {
                let path = FactTable::receiver_access_path(ir, *r);
                return facts.is_session_path(method, path.as_deref());
            }
            false
        }
        _ => false,
    }
}

/// Alert message if the node is a tainted use inside a configured sink call.
/// Honours [`FactTable`] sink signatures when one matches: a tainted arg in a
/// *non-dangerous* slot (e.g. a parameterized query's binding array) does not
/// alert.
pub(crate) fn sink_alert(
    ir: &FunctionIR,
    config: &TaintConfig,
    key: &NodeKey,
) -> Option<(crate::analysis::taint::engine::FindingClass, String)> {
    sink_alert_with_facts(ir, config, &FactTable::from_config(config), key)
}

/// Like [`sink_alert`] but consults a pre-built [`FactTable`] (no per-node
/// rebuild). Callers looping over many nodes should build the table once.
pub(crate) fn sink_alert_with_facts(
    ir: &FunctionIR,
    config: &TaintConfig,
    facts: &FactTable,
    key: &NodeKey,
) -> Option<(crate::analysis::taint::engine::FindingClass, String)> {
    let NodeKey {
        block,
        instr_idx: Some(idx),
        var,
    } = *key
    else {
        return None;
    };
    let b = ir.blocks.get(&block)?;
    if idx >= b.instructions.len() {
        return None;
    }
    let slot_and_name = match &b.instructions[idx] {
        Instruction::CallStatic { func, args, .. } if config.sinks.contains(func) => args
            .iter()
            .position(|a| *a == Operand::Var(var))
            .map(|pos| (pos, func)),
        Instruction::CallVirtual {
            method,
            args,
            receiver,
            ..
        } if config.sinks.contains(method) || {
            let root = match receiver {
                Operand::Var(r) => FactTable::receiver_root(ir, *r),
                _ => None,
            };
            facts.is_sink_call(method, root.as_deref())
        } =>
        {
            if receiver == &Operand::Var(var) {
                Some((usize::MAX, method)) // receiver slot
            } else {
                args.iter()
                    .position(|a| *a == Operand::Var(var))
                    .map(|pos| (pos, method))
            }
        }
        _ => None,
    };
    let (slot, name) = slot_and_name?;
    // Source-accessor precedence (RECEIVER-TAINT ALERTS ONLY): when the
    // alert comes from a tainted *receiver* and the method name is the last
    // segment of a configured dotted source pattern, the call READS taint
    // rather than consuming it, Go's canonical `r.URL.Query().Get("id")`
    // (both `Query` and `Get` are also sink names) must not alert just
    // because its receiver carries taint.
    //
    // This must NOT apply to argument-slot alerts: `req.query` is a source
    // pattern, so a blanket rule would suppress `pool.query(sql)`, a real
    // sink, because the sink name matches a source-pattern segment.
    if slot == usize::MAX {
        if is_source(ir, config, key) {
            return None;
        }
        #[allow(clippy::disallowed_methods)]
        {
            let is_source_accessor = config
                .sources
                .iter()
                .any(|s| s.rsplit('.').next() == Some(name));
            if is_source_accessor {
                return None;
            }
        }
    }
    // Receiver-taint rule: for a slot-restricted sink (dangerous_args
    // non-empty), a tainted RECEIVER is the object being read (the DB
    // handle, the request object), not data being consumed, no alert.
    // Only all-args sinks keep receiver taint dangerous.
    if slot == usize::MAX
        && let Some(sig) = facts.sink_signature(name)
        && !sig.dangerous_args.is_empty()
    {
        return None;
    }
    // Sink-signature check: dangerous slot (or receiver) only.
    if let Some(sig) = facts.sink_signature(name)
        && slot != usize::MAX
        && !sig.is_dangerous(slot)
    {
        return None; // safe binding channel, no alert
    }
    let where_ = if slot == usize::MAX {
        "at receiver".to_string()
    } else {
        format!("at argument {}", slot)
    };
    // Arg-shape classification: when the tainted argument is (or resolves
    // through an Assign chain to) an object literal carrying IDOR keys
    // (`where`, `id`, `owner`, ...), the sink call is an access-control
    // query, not an injection, the driver parameterizes object values.
    // Reported with the Idor class so consumers rank it below Critical.
    let class = if let Some(sig) = facts.sink_signature(name)
        && !sig.idor_keys.is_empty()
        && slot != usize::MAX
        && arg_is_idor_shape(ir, var, &sig.idor_keys)
    {
        crate::analysis::taint::engine::FindingClass::Idor
    } else {
        crate::analysis::taint::engine::FindingClass::Injection
    };
    let message = if class == crate::analysis::taint::engine::FindingClass::Idor {
        format!(
            "IDOR-CLASS: Tainted data controls a query-object field at sink '{}' {} [in {}]",
            name, where_, ir.name
        )
    } else {
        format!(
            "CRITICAL VULNERABILITY: Tainted data reached sink '{}' {} [in {}]",
            name, where_, ir.name
        )
    };
    Some((class, message))
}

/// Does the tainted var originate from an object literal whose pair keys
/// intersect `idor_keys`? Follows plain `Assign` chains (composite merge
/// vars, destructuring temps) but stops at anything else.
fn arg_is_idor_shape(ir: &FunctionIR, mut var: VarId, idor_keys: &[String]) -> bool {
    for _ in 0..8 {
        if let Some(meta) = ir.var_metadata.get(&var)
            && meta
                .object_keys
                .iter()
                .any(|k| idor_keys.iter().any(|ik| ik == k))
        {
            return true;
        }
        // Follow one Assign hop back (composite merge chain).
        let mut next = None;
        for b in ir.blocks.values() {
            for instr in &b.instructions {
                if let Instruction::Assign { dest, src } = instr
                    && *dest == var
                    && let Operand::Var(v) = src
                {
                    next = Some(*v);
                }
            }
        }
        match next {
            Some(v) => var = v,
            None => return false,
        }
    }
    false
}

/// Object-literal keys that mark a tainted argument as an IDOR-class query
/// payload. Shared with the JS spec's sink-signature table via
/// `idor_keys`; this default covers common finders when a spec provides
/// none of its own.
pub const DEFAULT_IDOR_KEYS: &[&str] = &["where", "id", "owner", "userId", "user", "_id"];

/// Sink names whose tainted object-literal arguments are classified as
/// IDOR-class (access-control) findings rather than injection.
pub const IDOR_FINDER_SINKS: &[&str] = &[
    "findOne",
    "findOneAndUpdate",
    "findOneAndDelete",
    "findOneAndReplace",
    "findByIdAndUpdate",
    "findByIdAndDelete",
    "find",
    "findAll",
    "update",
    "updateOne",
    "updateMany",
    "deleteOne",
    "deleteMany",
    "destroy",
    "count",
];

// ---------------------------------------------------------------------------
// Interprocedural taint engine
// ---------------------------------------------------------------------------

/// BFS taint propagation over the whole-program value-flow structure.
///
/// Cost remains proportional to the taint-reachable subgraph: local successors
/// plus explicit cross edges, no fixed-point loop, no visit to functions or
/// variables off every taint path.
pub struct InterproceduralTaintEngine<'a> {
    prog: &'a ProgramSvfg<'a>,
    config: &'a TaintConfig,
    tainted: FxHashSet<(usize, NodeKey)>,
    pub alerts: Vec<String>,
}

impl<'a> InterproceduralTaintEngine<'a> {
    pub fn new(prog: &'a ProgramSvfg<'a>, config: &'a TaintConfig) -> Self {
        Self {
            prog,
            config,
            tainted: FxHashSet::default(),
            alerts: Vec::new(),
        }
    }

    /// Run the BFS from all source nodes across all functions.
    pub fn run(&mut self) {
        let mut queue: VecDeque<(usize, NodeKey)> = VecDeque::new();

        // Seeds: source definitions in every function (deterministic order).
        for fi in 0..self.prog.functions.len() {
            let fe = &self.prog.functions[fi];
            let mut keys: Vec<NodeKey> = fe.svfg.nodes.keys().copied().collect();
            keys.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
            for k in keys {
                if is_source(fe.ir, self.config, &k) {
                    self.tainted.insert((fi, k));
                    queue.push_back((fi, k));
                }
            }
        }

        while let Some((fi, cur)) = queue.pop_front() {
            let fe = &self.prog.functions[fi];

            // Sink check at every visited node.
            if let Some((_, alert)) = sink_alert(fe.ir, self.config, &cur)
                && !self.alerts.contains(&alert)
            {
                self.alerts.push(alert);
            }

            if fe.svfg.node(&cur).is_none() {
                continue;
            }

            // Local edges (suppressed pass-through edges removed).
            for succ in self.prog.local_successors(fi, &cur) {
                if is_sanitizer_use(fe.ir, self.config, &succ) {
                    continue;
                }
                self.visit(fi, succ, &mut queue);
            }

            // Interprocedural edges (actual-arg → formal-param,
            // formal-ret → actual-ret).
            for (gi, gk) in self.prog.inter_successors(fi, &cur) {
                let ge = &self.prog.functions[gi];
                if is_sanitizer_use(ge.ir, self.config, &gk) {
                    continue;
                }
                self.visit(gi, gk, &mut queue);
            }
        }
    }

    fn visit(&mut self, fi: usize, key: NodeKey, queue: &mut VecDeque<(usize, NodeKey)>) {
        if self.tainted.insert((fi, key)) {
            queue.push_back((fi, key));
        }
    }

    /// The set of tainted `(function, node)` pairs, for debugging/tests.
    pub fn tainted_nodes(&self) -> &FxHashSet<(usize, NodeKey)> {
        &self.tainted
    }
}
