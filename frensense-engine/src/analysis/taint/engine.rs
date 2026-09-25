// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Task 4.4: Demand-Driven Backward Taint Analysis
//!
//! The forward engines seed at **sources** and push taint through everything
//! reachable. For corpus analysis that is often wasteful: a repository may
//! have 10,000 functions but only 12 that touch a configured sink. This
//! module inverts the direction (Oracle Labs / Allen et al. 2021 style):
//!
//! ```text
//! 1. Start at each configured **sink** (its argument use-nodes).
//! 2. Walk BACKWARD along value-flow edges: which definitions could feed
//!    this sink argument?
//! 3. For each such definition, walk backward further: what feeds IT?
//! 4. Stop conditions:
//!      - a configured **source**   → real vulnerability path → alert
//!      - a configured **sanitizer** → this branch is clean → prune
//!      - a node with no further predecessors:
//!          - an unlinked formal param (callee never resolved) → **unknown**
//!          - anything else (literal, external, allocate) → clean root
//! ```
//!
//! Only functions actually on some sink's backward-reachable subgraph are
//! ever visited, the other 9,988 are untouched. There is no pre-computed
//! whole-program call graph requirement: cross edges are consulted lazily
//! per node via a reverse index built on demand.
//!
//! ## Soundness note
//!
//! Backward analysis is *unsound* by nature when pass-through edges are
//! suppressed (a sanitizing callee makes the local arg→dest edge vanish), so
//! this engine deliberately runs over the **unsuppressed** predecessor view:
//! it asks "could tainted data reach here?" rather than "does summary-tainted
//! data reach here?". Call it with [`BackwardEngine::with_suppression`] to
//! honour the summary-driven suppression instead (more precise, matches the
//! forward engine's alerts exactly).
//!
//! ## Verdicts
//!
//! Each explored sink argument gets a [`BackwardVerdict`]:
//! `Vulnerable` (source reached), `Sanitized` (only sanitizer-cut branches),
//! `Unknown` (unresolvable definitions on the path), or `Clean`.

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

use crate::analysis::forward::{
    ProgramSvfg, is_sanitizer_use_with_facts, is_source, member_access_path, sink_alert_with_facts,
};
use crate::analysis::taint::config::TaintConfig;
use crate::analysis::taint::facts::FactTable;
use crate::analysis::taint::path::{PathStep, TaintPath};
use crate::graph::svfg::NodeKey;
use crate::ir::function::*;

// ---------------------------------------------------------------------------
// Verdicts
// ---------------------------------------------------------------------------

/// Outcome of the backward exploration of one sink argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackwardVerdict {
    /// A configured source reaches this sink argument, real alert.
    Vulnerable,
    /// All backward paths are cut by sanitizers, safe.
    Sanitized,
    /// The backward walk hit definitions that cannot be resolved within the
    /// analysed program (unlinked params of external callees, globals with no
    /// local def, pointer sources). Manual review or a wider config needed.
    Unknown,
    /// No definitions at all feed this argument (pure literal / constant).
    Clean,
}

/// Shape classification of a finding, derived from the sink-call argument
/// shape (object-literal query payload vs raw value).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FindingClass {
    /// Taint reaches a raw injection channel (SQL string, exec, redirect...).
    #[default]
    Injection,
    /// Taint controls a field of an object-literal query payload
    /// (`findOne({ where: { id: taint } })`), an access-control concern.
    Idor,
}

/// One explored sink argument with its verdict.
#[derive(Debug, Clone)]
pub struct SinkFinding {
    /// Function in which the sink call lives.
    pub function: String,
    pub sink: String,
    /// Formal slot of the tainted argument (receiver = slot 0 on virtual calls).
    pub arg_slot: usize,
    /// Message identical in shape to the forward engine's alert.
    pub alert: Option<String>,
    pub verdict: BackwardVerdict,
    /// Shape classification of the finding (`"idor"` for access-control
    /// query payloads, `"injection"` for everything else). Consumers rank
    /// idor findings below Critical.
    pub finding_class: FindingClass,
    /// What the sink does with its input, the primary severity signal.
    pub role: crate::analysis::taint::role::SinkRole,
    /// Human description of the taint origin (the source access path,
    /// e.g. `req.body.target`), populated for `Vulnerable` verdicts.
    pub source_desc: Option<String>,
    /// Byte range of the sink call instruction in the source file,
    /// when the lowering recorded spans.
    pub sink_span: Option<(usize, usize)>,
    /// Reconstructed source→sink chain (vulnerable findings only; empty
    /// otherwise, and empty when path capture is disabled).
    pub path: TaintPath,
}

/// Traversal statistics, proof of the demand-driven cost model.
#[derive(Debug, Clone, Copy, Default)]
pub struct BackwardStats {
    /// `(function, node)` pairs visited during backward exploration.
    pub nodes_visited: usize,
    /// Distinct functions entered (should be ≪ total functions on big repos).
    pub functions_visited: usize,
    /// Number of sink arguments explored.
    pub sink_args_explored: usize,
    /// Per-verdict counts.
    pub vulnerable: usize,
    pub sanitized: usize,
    pub unknown: usize,
    pub clean: usize,
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

/// Demand-driven backward taint engine over the whole-program SVFG.
pub struct BackwardTaintEngine<'a> {
    prog: &'a ProgramSvfg<'a>,
    config: &'a TaintConfig,
    /// Merged fact table (sink signatures + sanitizer facts), built once.
    facts: FactTable,
    /// If `true` (the default), honour summary-driven pass-through
    /// suppression, this is what makes "stop when you hit a sanitizer →
    /// safe" work across call boundaries: a sanitizing callee's summary
    /// removes the local arg→dest leak edge, so the backward walk proceeds
    /// through the callee's FormalRet (past its sanitizer) instead.
    honour_suppression: bool,
    /// Reverse cross-edge index, built lazily on first use:
    /// `(callee_fn, to_node) → [(caller_fn, from_node)]`.
    reverse_cross: FxHashMap<(usize, NodeKey), Vec<(usize, NodeKey)>>,

    pub findings: Vec<SinkFinding>,
    pub stats: BackwardStats,
    /// Per-function guard maps, built lazily on first entry into the fn.
    guard_maps: FxHashMap<usize, GuardMap>,
    /// Function-name → source-file mapping for path step locations
    /// (populated via [`with_fn_file`]; empty in tests without files).
    fn_file: FxHashMap<String, String>,
    /// When `true`, reconstruct the walked source→sink chain per vulnerable
    /// finding (small extra bookkeeping; path reporting needs it).
    capture_paths: bool,
    /// BFS parent of each visited node for the *current* root exploration,
    /// consumed right after `explore_from` in `explore_call_site`.
    current_parents: FxHashMap<(usize, NodeKey), (usize, NodeKey)>,
}

/// Per-root exploration state (not per-node: a node can be re-explored from
/// another sink root with a different outcome, that's the point of verdicts).
#[derive(Default)]
struct ExploreState {
    saw_source: bool,
    /// The first source node reached (for path reconstruction).
    source_node: Option<(usize, NodeKey)>,
    /// Human description of the first source found (access path / call name).
    source_desc: Option<String>,
    saw_sanitized_root_only: bool,
    saw_unknown: bool,
    /// Nodes whose local predecessors were fully expanded for this root.
    visited: FxHashSet<(usize, NodeKey)>,
}

/// Human-readable description of the taint origin at a source node:
/// the source call name (`.getQuery()`) or the member access path
/// (`req.body.args`). Returns `None` when the node is not a source.
pub(crate) fn source_description(
    ir: &FunctionIR,
    config: &TaintConfig,
    key: &NodeKey,
) -> Option<String> {
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
        // Parameter sentinel: the source is the parameter's own name.
        return ir
            .var_metadata
            .get(&var)
            .and_then(|m| m.source_name.clone())
            .filter(|n| config.sources.contains(n));
    }
    match &b.instructions[idx] {
        Instruction::CallStatic {
            func,
            dest: Some(d),
            ..
        } if config.sources.contains(func) && *d == var => Some(func.clone()),
        Instruction::CallVirtual {
            method,
            dest: Some(d),
            ..
        } if config.sources.contains(method) && *d == var => Some(method.clone()),
        Instruction::LoadField { base, field, .. } => {
            let path = member_access_path(ir, *base, field);
            let root = path.split('.').next().unwrap_or("");
            if config.sources.contains(&path)
                || config.sources.contains(root)
                || config.sources.iter().any(|s| {
                    path.starts_with(s.as_str()) && path.as_bytes().get(s.len()) == Some(&b'.')
                })
            {
                Some(path)
            } else {
                None
            }
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Guard map (task 8.3, minimal form)
// ---------------------------------------------------------------------------

/// Per-function guard analysis: which variables are *checked* by a
/// guard-style call (`.test(x)`, `includes(x)`, allowlist `has(x)`) in which
/// blocks, plus dominator sets.
///
/// A value-flow graph alone cannot see guards: `if (SAFE.test(x)) sink(x)`
/// has the guard on a *sibling* use of `x`, so a backward walk from the sink
/// never passes through it. The fix is structural: the branch condition is
/// derived from a guard call on `x`; if that branch dominates the block the
/// sink lives in, any `x` value flowing there was checked.
#[derive(Default)]
struct GuardMap {
    /// var → blocks whose terminator branch is derived from a guard-style
    /// call on that var.
    guards: FxHashMap<VarId, Vec<BlockId>>,
    /// block → set of blocks that dominate it (reflexive), computed once.
    dominators: FxHashMap<BlockId, FxHashSet<BlockId>>,
}

impl GuardMap {
    fn build(ir: &FunctionIR, facts: &FactTable, config: &TaintConfig) -> Self {
        let mut guards: FxHashMap<VarId, Vec<BlockId>> = FxHashMap::default();

        // Find guard calls and the vars they check.
        // A guard call is a CallVirtual/CallStatic whose method has a
        // guard-style SanitizerFact. The checked var is the first Var arg
        // (predicate on an argument) or the receiver (predicate on the
        // receiver, e.g. `file.includes('..')`).
        let checked_vars = |instr: &Instruction| -> Vec<VarId> {
            let (method, receiver, args) = match instr {
                Instruction::CallVirtual {
                    method,
                    receiver,
                    args,
                    ..
                } => (method, Some(receiver), args),
                Instruction::CallStatic { func, args, .. } => (func, None, args),
                _ => return Vec::new(),
            };
            let Some(f) = facts.sanitizer_fact(method) else {
                return Vec::new();
            };
            if !f.guard_style {
                return Vec::new();
            }
            // The call is guard-style; also confirm the method really is one
            // of the configured/known guards (fact table already ensures
            // this). Collect checked vars.
            let mut out = Vec::new();
            if let Some(Operand::Var(r)) = receiver {
                out.push(*r);
            }
            for a in args {
                if let Operand::Var(v) = a {
                    out.push(*v);
                }
            }
            out
        };

        // Walk: cond var → def instruction, up to 3 hops, collecting
        // guard-call vars along the way (handles `!ALLOWED.has(x)`,
        // `x != null && SAFE.test(x)` shape fragments).
        fn defs_of(instr: &Instruction) -> Vec<VarId> {
            let mut v = Vec::with_capacity(2);
            match instr {
                Instruction::Assign { dest, .. }
                | Instruction::LoadField { dest, .. }
                | Instruction::LoadElement { dest, .. }
                | Instruction::LoadGlobal { dest, .. }
                | Instruction::Cast { dest, .. }
                | Instruction::ExtractValue { dest, .. }
                | Instruction::BinaryOp { dest, .. }
                | Instruction::UnaryOp { dest, .. } => v.push(*dest),
                Instruction::CallStatic { dest, .. }
                | Instruction::CallVirtual { dest, .. }
                | Instruction::CallPointer { dest, .. } => {
                    if let Some(d) = dest {
                        v.push(*d);
                    }
                }
                _ => {}
            }
            v
        }
        fn def_site_of(ir: &FunctionIR, v: VarId) -> Option<&Instruction> {
            for b in ir.blocks.values() {
                for i in &b.instructions {
                    if defs_of(i).contains(&v) {
                        return Some(i);
                    }
                }
            }
            None
        }

        let mut block_ids: Vec<BlockId> = ir.blocks.keys().copied().collect();
        block_ids.sort_by_key(|b| b.0);
        for &bid in &block_ids {
            let blk = &ir.blocks[&bid];
            let cond_var = match &blk.terminator {
                Terminator::Branch {
                    cond: Operand::Var(c),
                    ..
                } => Some(*c),
                Terminator::Branch { .. } => None,
                _ => continue,
            };
            let Some(cond_var) = cond_var else { continue };
            // Hop 0..3 up the def chain from the condition.
            let mut frontier = vec![cond_var];
            for _hop in 0..3 {
                let mut next = Vec::new();
                for v in &frontier {
                    let Some(instr) = def_site_of(ir, *v) else {
                        continue;
                    };
                    match instr {
                        Instruction::CallStatic { .. } | Instruction::CallVirtual { .. } => {
                            for cv in checked_vars(instr) {
                                guards.entry(cv).or_default().push(bid);
                            }
                        }
                        Instruction::UnaryOp {
                            src: Operand::Var(u),
                            ..
                        } => {
                            next.push(*u);
                        }
                        Instruction::BinaryOp { lhs, rhs, .. } => {
                            for op in [lhs, rhs] {
                                if let Operand::Var(u) = op {
                                    next.push(*u);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if next.is_empty() {
                    break;
                }
                frontier = next;
            }
        }
        let _ = config;

        // Dominators (Cooper-Harvey-Kennedy, small graphs).
        let mut preds: FxHashMap<BlockId, Vec<BlockId>> = FxHashMap::default();
        let mut succs: FxHashMap<BlockId, Vec<BlockId>> = FxHashMap::default();
        for (&b, blk) in &ir.blocks {
            for s in &blk.successors {
                succs.entry(b).or_default().push(*s);
                preds.entry(*s).or_default().push(b);
            }
        }
        // RPO from entry.
        let entry = ir.entry_block;
        let mut po = Vec::new();
        let mut seen = FxHashSet::default();
        let mut stack = vec![(entry, 0usize)];
        while let Some((b, i)) = stack.pop() {
            if i == 0 {
                if !seen.insert(b) {
                    continue;
                }
                stack.push((b, 1));
                if let Some(ss) = succs.get(&b) {
                    for &s in ss {
                        stack.push((s, 0));
                    }
                }
            } else {
                po.push(b);
            }
        }
        let rpo: Vec<BlockId> = po.iter().rev().copied().collect();
        let mut idom: FxHashMap<BlockId, BlockId> = FxHashMap::default();
        idom.insert(entry, entry);
        let mut changed = true;
        while changed {
            changed = false;
            for &b in &rpo {
                if b == entry {
                    continue;
                }
                let Some(ps) = preds.get(&b) else { continue };
                let mut new_idom: Option<BlockId> = None;
                for &p in ps {
                    if idom.contains_key(&p) {
                        new_idom = Some(match new_idom {
                            None => p,
                            Some(cur) => {
                                // intersect (Cooper-Harvey-Kennedy)
                                let pos = |x: BlockId| po.iter().position(|&y| y == x).unwrap_or(0);
                                let mut f1 = cur;
                                let mut f2 = p;
                                while f1 != f2 {
                                    while pos(f1) < pos(f2) {
                                        f1 = *idom.get(&f1).unwrap_or(&f1);
                                    }
                                    while pos(f2) < pos(f1) {
                                        f2 = *idom.get(&f2).unwrap_or(&f2);
                                    }
                                }
                                f1
                            }
                        });
                    }
                }
                if let Some(n) = new_idom
                    && idom.get(&b) != Some(&n)
                {
                    idom.insert(b, n);
                    changed = true;
                }
            }
        }
        // Dominator sets via idom chains.
        let mut dominators: FxHashMap<BlockId, FxHashSet<BlockId>> = FxHashMap::default();
        for &b in &rpo {
            let mut set = FxHashSet::default();
            set.insert(b);
            let mut cur = b;
            while let Some(&p) = idom.get(&cur) {
                if p == cur {
                    break;
                }
                set.insert(p);
                cur = p;
            }
            dominators.insert(b, set);
        }

        Self { guards, dominators }
    }

    /// Is `var` checked by a guard that dominates `sink_block`?
    fn is_guarded(&self, var: VarId, sink_block: BlockId) -> bool {
        let Some(gs) = self.guards.get(&var) else {
            return false;
        };
        let Some(doms) = self.dominators.get(&sink_block) else {
            return false;
        };
        gs.iter().any(|g| doms.contains(g))
    }
}

impl<'a> BackwardTaintEngine<'a> {
    pub fn new(prog: &'a ProgramSvfg<'a>, config: &'a TaintConfig) -> Self {
        Self {
            prog,
            config,
            facts: FactTable::from_config(config),
            honour_suppression: true,
            reverse_cross: FxHashMap::default(),
            guard_maps: FxHashMap::default(),
            capture_paths: true,
            current_parents: FxHashMap::default(),
            fn_file: FxHashMap::default(),
            findings: Vec::new(),
            stats: BackwardStats::default(),
        }
    }

    /// Merge additional (e.g. bundle-learned) facts over the built-in table.
    pub fn with_fact_table(mut self, facts: &FactTable) -> Self {
        self.facts.merge(facts);
        self
    }

    /// Provide the function-name → source-file mapping so captured taint
    /// paths carry real file locations per step (cross-function and
    /// cross-file flows otherwise report the function name as the file).
    pub fn with_fn_file(mut self, fn_file: &FxHashMap<String, String>) -> Self {
        self.fn_file = fn_file.clone();
        self
    }

    /// Disable summary-driven pass-through suppression (over-approximating
    /// mode: any caller-side arg→dest edge leaks the walk past sanitizing
    /// callees, so verdicts may be `Vulnerable` where the precise mode says
    /// `Sanitized`). Useful as a sound upper bound.
    pub fn without_suppression(mut self) -> Self {
        self.honour_suppression = false;
        self
    }

    /// Toggle taint-path capture (on by default). Turning it off saves the
    /// per-root parent map when path reporting is not needed.
    pub fn capture_paths(mut self, yes: bool) -> Self {
        self.capture_paths = yes;
        self
    }

    /// Build the reverse cross-edge index once.
    fn ensure_reverse_index(&mut self) {
        if !self.reverse_cross.is_empty() {
            return;
        }
        for ((from_f, from_k), edges) in &self.prog.cross_edges {
            for (to_f, to_k) in edges {
                self.reverse_cross
                    .entry((*to_f, *to_k))
                    .or_default()
                    .push((*from_f, *from_k));
            }
        }
        for v in self.reverse_cross.values_mut() {
            v.sort_by_key(|(f, k)| (*f, k.block.0, k.instr_idx, k.var.0));
            v.dedup();
        }
    }

    /// Run the demand-driven analysis over every configured sink argument.
    pub fn run(&mut self) {
        self.ensure_reverse_index();

        for fi in 0..self.prog.functions.len() {
            let ir = self.prog.functions[fi].ir;
            // Guard map per function (task 8.3): guards + dominators.
            self.guard_maps
                .entry(fi)
                .or_insert_with(|| GuardMap::build(ir, &self.facts, self.config));
            let mut block_ids: Vec<BlockId> = ir.blocks.keys().copied().collect();
            block_ids.sort_by_key(|b| b.0);

            for &block in &block_ids {
                let bdata = &ir.blocks[&block];
                for (idx, instr) in bdata.instructions.iter().enumerate() {
                    self.explore_call_site(fi, block, idx, instr);
                }
            }
        }
    }

    /// Explore one call site if it is a configured sink.
    fn explore_call_site(&mut self, fi: usize, block: BlockId, idx: usize, instr: &Instruction) {
        let (name, receiver, args, call_dest): (
            &str,
            Option<&Operand>,
            &Vec<Operand>,
            Option<VarId>,
        ) = match instr {
            Instruction::CallStatic {
                func, args, dest, ..
            } => (func, None, args, dest.as_ref().copied()),
            Instruction::CallVirtual {
                method,
                receiver,
                args,
                dest,
                ..
            } => (method, Some(receiver), args, dest.as_ref().copied()),
            _ => return,
        };
        let ir = self.prog.functions[fi].ir;
        // Receiver-aware sink match: verb-named sinks (get/post/...) from
        // dotted client entries only fire when the receiver root is a known
        // client, `map.get(t)` is a Map accessor, `got.get(t)` is SSRF.
        let receiver_root = match receiver {
            Some(Operand::Var(r)) => FactTable::receiver_root(ir, *r),
            _ => None,
        };
        if !self.facts.is_sink_call(name, receiver_root.as_deref()) {
            return;
        }

        // Collect (slot, var, arg-node) triplets. For virtual calls the
        // receiver is slot 0 (matching the forward engine's cross-edge slots).
        let mut triplets: Vec<(usize, VarId, NodeKey)> = Vec::new();
        let mut slot = 0usize;
        if let Some(Operand::Var(r)) = receiver {
            triplets.push((0, *r, NodeKey::instr(block, idx, *r)));
            slot = 1;
        }
        for a in args {
            if let Operand::Var(v) = a {
                triplets.push((slot, *v, NodeKey::instr(block, idx, *v)));
            }
            slot += 1;
        }
        if triplets.is_empty() {
            return;
        }

        self.stats.sink_args_explored += triplets.len();

        for (arg_slot, var, arg_node) in triplets {
            let sink_eval = sink_alert_with_facts(ir, self.config, &self.facts, &arg_node);
            let finding_class = sink_eval
                .as_ref()
                .map(|(c, _)| *c)
                .unwrap_or(FindingClass::Injection);
            let alert = sink_eval.map(|(_, m)| m);
            // Receiver-aware role: `kv.put(t)` resolves Storage (dotted
            // KVNamespace.put) while `axios.put(t)` stays Ssrf, the
            // receiver root is the disambiguator, mirroring verb-sink
            // matching above.
            let role = self
                .facts
                .role_for_call(name, receiver_root.as_deref())
                .unwrap_or(crate::analysis::taint::role::SinkRole::Other);
            let mut state = ExploreState::default();
            state.visited.insert((fi, arg_node));
            self.current_parents.clear();
            self.explore_from(fi, arg_node, &mut state);

            let verdict = if state.saw_source {
                BackwardVerdict::Vulnerable
            } else if state.saw_unknown {
                BackwardVerdict::Unknown
            } else if state.saw_sanitized_root_only {
                BackwardVerdict::Sanitized
            } else {
                BackwardVerdict::Clean
            };

            // Bookkeeping for stats.
            self.stats.nodes_visited += state.visited.len();
            let mut fns: FxHashSet<usize> = FxHashSet::default();
            for (f, _) in state.visited.iter() {
                fns.insert(*f);
            }
            self.stats.functions_visited += fns.len();

            match verdict {
                BackwardVerdict::Vulnerable => self.stats.vulnerable += 1,
                BackwardVerdict::Sanitized => self.stats.sanitized += 1,
                BackwardVerdict::Unknown => self.stats.unknown += 1,
                BackwardVerdict::Clean => self.stats.clean += 1,
            }

            let alert = if verdict == BackwardVerdict::Vulnerable {
                alert
            } else {
                None
            };

            // Reconstruct the source→sink chain for vulnerable findings.
            let path = if verdict == BackwardVerdict::Vulnerable && self.capture_paths {
                self.reconstruct_path(state.source_node, state.source_desc.clone())
            } else {
                TaintPath::default()
            };

            self.findings.push(SinkFinding {
                function: ir.name.clone(),
                sink: name.to_string(),
                arg_slot,
                alert,
                verdict,
                finding_class,
                role,
                source_desc: state.source_desc.clone(),
                path,
                // Span of the SINK CALL itself, not the tainted operand's
                // definition: reports must point at the line the user fixes.
                // (Reporting the operand's def site put `search.ts` SQLi at
                // line 18 instead of the vulnerable `query(...)` on line 21.)
                sink_span: call_dest
                    .and_then(|d| ir.var_metadata.get(&d))
                    .and_then(|m| m.byte_range)
                    .or_else(|| ir.var_metadata.get(&var).and_then(|m| m.byte_range)),
            });
            let _ = var;
        }
    }

    /// Classify one walked node as a [`PathStep`] for path reporting.
    fn step_of(&self, cf: usize, key: &NodeKey) -> PathStep {
        let fe = &self.prog.functions[cf];
        let ir = fe.ir;
        let fname = ir.name.clone();
        let Some(node) = fe.svfg.node(key) else {
            return PathStep::Assignment {
                function: fname,
                variable: key.var.0 as u32,
            };
        };
        use crate::graph::svfg::NodeKind;
        match &node.kind {
            NodeKind::Phi => PathStep::Phi {
                function: fname,
                variable: key.var.0 as u32,
            },
            NodeKind::FormalParam => {
                let name = ir
                    .var_metadata
                    .get(&key.var)
                    .and_then(|m| m.source_name.clone())
                    .unwrap_or_else(|| format!("v{}", key.var.0));
                PathStep::FormalParam {
                    function: fname,
                    param: name,
                }
            }
            NodeKind::ActualArg {
                call_site,
                arg_index,
            } => PathStep::CallArgument {
                function: fname,
                callee: self.callee_display(ir, call_site),
                slot: *arg_index,
            },
            NodeKind::ActualRet { call_site } => {
                let callee = self.callee_display(ir, call_site);
                let external = !self.prog.functions.iter().any(|f| f.ir.name == callee);
                PathStep::CallReturn {
                    function: fname,
                    callee,
                    external,
                }
            }
            NodeKind::FormalRet => PathStep::ReturnToCaller { caller: fname },
            NodeKind::InstrDef | NodeKind::InstrUse => self.instr_step(ir, key, fname),
        }
    }

    /// Display name of the callee at a call-site node key.
    fn callee_display(&self, ir: &FunctionIR, call_site: &NodeKey) -> String {
        let Some(idx) = call_site.instr_idx else {
            return "?".into();
        };
        match ir
            .blocks
            .get(&call_site.block)
            .and_then(|b| b.instructions.get(idx))
        {
            Some(Instruction::CallStatic { func, .. }) => func.clone(),
            Some(Instruction::CallVirtual { method, .. }) => method.clone(),
            _ => "?".into(),
        }
    }

    /// Instruction-level step: field loads become `FieldLoad`, everything
    /// else a generic `Assignment` (variable-level flow).
    fn instr_step(&self, ir: &FunctionIR, key: &NodeKey, fname: String) -> PathStep {
        let Some(idx) = key.instr_idx else {
            return PathStep::Assignment {
                function: fname,
                variable: key.var.0 as u32,
            };
        };
        let Some(b) = ir.blocks.get(&key.block) else {
            return PathStep::Assignment {
                function: fname,
                variable: key.var.0 as u32,
            };
        };
        match b.instructions.get(idx) {
            Some(Instruction::LoadField { base, field, .. }) => {
                let base_name = ir
                    .var_metadata
                    .get(base)
                    .and_then(|m| m.source_name.clone())
                    .unwrap_or_else(|| format!("v{}", base.0));
                PathStep::FieldLoad {
                    function: fname,
                    base: base_name,
                    field: field.clone(),
                }
            }
            _ => PathStep::Assignment {
                function: fname,
                variable: key.var.0 as u32,
            },
        }
    }

    /// Walk the BFS parent map from the sink argument back to the source and
    /// build the source→sink [`TaintPath`].
    fn reconstruct_path(
        &self,
        source_node: Option<(usize, NodeKey)>,
        source_desc: Option<String>,
    ) -> TaintPath {
        let mut chain: Vec<(String, FunctionIR, NodeKey, PathStep)> = Vec::new();
        // The parent map maps each visited node → its BFS parent (the node
        // one step closer to the sink). The chain therefore runs from the
        // source (which has no parent entry, BFS stopped there) to the node
        // just before the sink argument. The sink node itself is not a step:
        // the finding already reports it.
        let Some((sf, skey)) = source_node else {
            return TaintPath::new(chain, source_desc, &self.fn_file);
        };
        let mut cur = (sf, skey);
        for _ in 0..self.current_parents.len() + 1 {
            let (cf, key) = cur;
            let ir = self.prog.functions[cf].ir.clone();
            let step = self.step_of(cf, &key);
            chain.push((ir.name.clone(), ir, key, step));
            match self.current_parents.get(&cur) {
                Some(&parent) => cur = parent,
                None => break,
            }
        }
        TaintPath::new(chain, source_desc, &self.fn_file)
    }

    /// Backward BFS from `root` within one root's exploration state.
    fn explore_from(&mut self, fi: usize, root: NodeKey, state: &mut ExploreState) {
        let mut queue: VecDeque<(usize, NodeKey)> = VecDeque::new();
        queue.push_back((fi, root));

        while let Some((cf, cur)) = queue.pop_front() {
            #[allow(clippy::disallowed_methods)]
            if std::env::var("FRENSdbg_WALK").is_ok() {
                let kind = self.prog.functions[cf]
                    .svfg
                    .node(&cur)
                    .map(|n| format!("{:?}", n.kind))
                    .unwrap_or("NONE".into());
                eprintln!("[walk] cf={cf} key={cur:?} kind={kind}");
            }
            let ir = self.prog.functions[cf].ir;

            // Stop: guard check. If this node's variable is checked by a
            // guard-style call (`.test(x)` / `includes(x)`) whose branch
            // dominates the block where the value is (re)used, the value was
            // validated on every path reaching here, sanitized (task 8.3
            // guard map). Pure value-flow cannot see this: the guard is a
            // sibling use, not a link in the chain.
            if let Some(gm) = self.guard_maps.get(&cf)
                && let Some(def_block) = self.def_block_of(cf, &cur)
                && gm.is_guarded(cur.var, def_block)
            {
                state.saw_sanitized_root_only = true;
                continue;
            }

            // Stop: source reached, vulnerable path confirmed.
            if is_source(ir, self.config, &cur) {
                state.saw_source = true;
                if state.source_node.is_none() {
                    state.source_node = Some((cf, cur));
                    state.source_desc = source_description(ir, self.config, &cur);
                }
                continue;
            }
            // Stop: sanitizer use, this branch is clean. Fact-table aware:
            // includes learned methods like .replace()/.test() guards.
            if is_sanitizer_use_with_facts(ir, self.config, &self.facts, &cur) {
                state.saw_sanitized_root_only = true;
                continue;
            }

            // Local predecessors (optionally honouring suppression).
            let local_preds: Vec<NodeKey> = if self.honour_suppression {
                self.prog.local_predecessors(cf, &cur)
            } else {
                match self.prog.functions[cf].svfg.node(&cur) {
                    Some(n) => n.predecessors(),
                    None => Vec::new(),
                }
            };

            // Cross predecessors (reverse index): who feeds this node from
            // another function? For a FormalParam that means actual-args at
            // caller call sites; for an ActualRet that means the callee's
            // FormalRet nodes.
            let cross_preds: Vec<(usize, NodeKey)> = self
                .reverse_cross
                .get(&(cf, cur))
                .cloned()
                .unwrap_or_default();

            if local_preds.is_empty() && cross_preds.is_empty() {
                #[allow(clippy::disallowed_methods)]
                if std::env::var("FRENSdbg_DEADEND").is_ok() {
                    let kind = self.prog.functions[cf]
                        .svfg
                        .node(&cur)
                        .map(|n| format!("{:?}", n.kind))
                        .unwrap_or("NONE".into());
                    eprintln!("[deadend] cf={cf} key={cur:?} kind={kind}");
                }
                // Dead end. Classify: unresolvable roots (formal params never
                // fed by an analysed call site, or environment-defined values
                // like LoadGlobal / Dereference / CallPointer) are "unknown",
                // we can't prove them clean. Everything else (literals,
                // allocations) is a genuinely clean root.
                if self.node_is_unresolvable(cf, &cur) {
                    state.saw_unknown = true;
                }
                // Otherwise: clean root, stop.
                continue;
            }

            for (pf, pk) in cross_preds {
                if state.visited.insert((pf, pk)) {
                    if self.capture_paths {
                        self.current_parents.insert((pf, pk), (cf, cur));
                    }
                    queue.push_back((pf, pk));
                }
            }
            for pk in local_preds {
                if state.visited.insert((cf, pk)) {
                    if self.capture_paths {
                        self.current_parents.insert((cf, pk), (cf, cur));
                    }
                    queue.push_back((cf, pk));
                }
            }
        }
    }

    /// A node is unresolvable if we cannot prove its value clean within the
    /// analysed program:
    ///   * a `FormalParam` with no cross-edge feeders (external entry point,
    ///     or called only with literal args), or
    ///   * a def produced by an environment-read instruction (`LoadGlobal`,
    ///     `Dereference`, `CallPointer`), its value comes from outside the
    ///     local value-flow graph.
    fn node_is_unresolvable(&self, fi: usize, key: &NodeKey) -> bool {
        let fe = &self.prog.functions[fi];
        let Some(node) = fe.svfg.node(key) else {
            return false;
        };
        if node.kind == crate::graph::svfg::NodeKind::FormalParam {
            // Unresolvable iff there are no cross edges feeding it.
            return !self.reverse_cross.contains_key(&(fi, *key));
        }
        // A call result with no cross-edge feeding it is a call to an
        // unknown/external callee (library API): its semantics are unknown,
        // so "clean" would be unsound, classify as unknown. Resolved
        // internal callees have ActualRet→FormalRet reverse edges and never
        // reach this dead-end classification.
        if matches!(node.kind, crate::graph::svfg::NodeKind::ActualRet { .. }) {
            #[allow(clippy::disallowed_methods)]
            if std::env::var("FRENSdbg_UNRES").is_ok() {
                eprintln!(
                    "[unresolvable] ActualRet key={key:?} has_cross={} block={:?} idx={:?}",
                    self.reverse_cross.contains_key(&(fi, *key)),
                    key.block,
                    key.instr_idx
                );
            }
            let NodeKey {
                block,
                instr_idx: Some(idx),
                var,
            } = *key
            else {
                return false;
            };
            if let Some(b) = fe.ir.blocks.get(&block)
                && idx < b.instructions.len()
                && matches!(
                    &b.instructions[idx],
                    Instruction::CallStatic { dest: Some(d), .. }
                        | Instruction::CallVirtual { dest: Some(d), .. }
                        | Instruction::CallPointer { dest: Some(d), .. }
                    if *d == var
                )
            {
                return true;
            }
            return false;
        }
        if node.kind == crate::graph::svfg::NodeKind::InstrDef {
            let NodeKey {
                block,
                instr_idx: Some(idx),
                ..
            } = *key
            else {
                return false;
            };
            let Some(b) = fe.ir.blocks.get(&block) else {
                return false;
            };
            if idx >= b.instructions.len() {
                return false;
            }
            if matches!(
                &b.instructions[idx],
                Instruction::LoadGlobal { .. }
                    | Instruction::Dereference { .. }
                    | Instruction::CallPointer { .. }
            ) {
                return true;
            }
            // A call result with no cross-edge feeding it is a call to an
            // unknown/external callee (library API): its semantics are
            // unknown, so "clean" would be unsound, classify as unknown.
            // Resolved internal callees have ActualRet→FormalRet reverse
            // edges and never reach this dead-end classification.
            if matches!(node.kind, crate::graph::svfg::NodeKind::ActualRet { .. })
                && matches!(
                    &b.instructions[idx],
                    Instruction::CallStatic { dest: Some(d), .. }
                        | Instruction::CallVirtual { dest: Some(d), .. }
                    if *d == key.var
                )
            {
                return true;
            }
        }
        false
    }

    /// Block containing the node's defining instruction (guards are
    /// block-anchored). Sentinel instr_idxs (params/terminator) map to their
    /// block too.
    fn def_block_of(&self, fi: usize, key: &NodeKey) -> Option<BlockId> {
        let fe = &self.prog.functions[fi];
        fe.svfg.node(key).map(|_| key.block)
    }

    /// Convenience: just the vulnerable alerts (compatible with forward engines).
    pub fn alerts(&self) -> Vec<String> {
        self.findings
            .iter()
            .filter_map(|f| f.alert.clone())
            .collect()
    }
}
