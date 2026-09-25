// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Task 5: Selective k=1 Call-Site Context Sensitivity
//!
//! The k=0 forward engine (`InterproceduralTaintEngine`) keys its taint state
//! by `(function, node)`. When one function is called from two sites, one on
//! a taint path, one not, the callee's `FormalRet → ActualRet` cross edges
//! fan the taint out to **every** call site, so the clean caller's result is
//! wrongly tainted (a false positive that the summary pass-through edge can
//! also produce).
//!
//! This module refines the analysis state to `(function, node, context)`,
//! where the context is **the call site through which we entered the current
//! function** (k=1 call-site sensitivity). Parameter and return edges are then
//! matched per context:
//!
//! ```text
//!   enter F via call site c:  ActualArg(c, slot) ─▶ FormalParam(F, slot)
//!   return from F to c:       FormalRet(F)       ─▶ ActualRet(c)   [same c]
//! ```
//!
//! ## Selective application (the corpus cost trade-off)
//!
//! Full context sensitivity is exponential. The 4.x roadmap's compromise:
//!
//!   * **Phase 1 (cheap):** run the k=0 engine. Its taint-reachable node set
//!     identifies every function that lies on *any* potential taint path.
//!   * **Phase 2 (precise):** only functions in that relevance set get
//!     context-split analysis. Everything else stays k=0.
//!
//! In practice the phase-2 subgraph is tiny (the spec: "functions that are
//! never on a taint path use k=0, the cheap default"), so the exponential
//! blow-up is bounded by the taint-relevant code, not the corpus.
//!
//! Contexts are interned to a `CtxId` (u32) and capped: if a function's
//! context count exceeds [`MAX_CONTEXTS_PER_FN`], new contexts collapse into
//! an existing one (k→0 fallback for that function), keeping the analysis
//! sound while bounding cost.

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

use crate::analysis::forward::{
    InterproceduralTaintEngine, ProgramSvfg, is_sanitizer_use, is_source, sink_alert,
};
use crate::analysis::taint::config::TaintConfig;
use crate::graph::svfg::NodeKey;

/// Safety valve: most contexts a single function may hold before new ones
/// collapse into an existing context (k→0 fallback for that function).
/// Research consensus: k=1 recovers most precision; the cap only triggers on
/// pathological call patterns.
pub const MAX_CONTEXTS_PER_FN: usize = 64;

/// Interned call-site context.
pub type CtxId = u32;

/// The `None`-like "no context" id used for function roots (source seeds).
pub const ROOT_CTX: CtxId = u32::MAX;

// ---------------------------------------------------------------------------
// Result
// ---------------------------------------------------------------------------

/// Alert with the context chain that produced it (for triage / dedup).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextualAlert {
    pub message: String,
    /// Function in which the sink lives.
    pub function: String,
    /// The call-site context under which the taint arrived at the sink.
    pub context: CtxId,
}

/// Statistics proving the selective cost model.
#[derive(Debug, Clone, Copy, Default)]
pub struct ContextStats {
    /// k=0 taint-reachable states in phase 1.
    pub phase1_states: usize,
    /// Functions marked taint-relevant by phase 1 (get context splitting).
    pub relevant_functions: usize,
    /// Total functions in the program.
    pub total_functions: usize,
    /// `(function, node, ctx)` states expanded in phase 2.
    pub phase2_states: usize,
    /// Distinct contexts created (across all functions).
    pub contexts_created: usize,
    /// How many times the context cap collapsed a new context (k→0 fallback).
    pub context_collapses: usize,
}

// ---------------------------------------------------------------------------
// Context interning
// ---------------------------------------------------------------------------

/// Interns (function, call-site) contexts to small ids, with a per-function
/// cap that falls back to k=0 behaviour when exceeded.
#[derive(Default)]
struct ContextPool {
    /// `(callee_fn, call_site_key)` → context id.
    by_key: FxHashMap<(usize, NodeKey), CtxId>,
    next_id: CtxId,
    /// Contexts held per function (for the cap).
    per_fn_count: FxHashMap<usize, usize>,
    collapses: usize,
}

impl ContextPool {
    fn intern(&mut self, callee_fn: usize, call_site: NodeKey) -> (CtxId, bool) {
        // Existing context: reuse it.
        if let Some(&id) = self.by_key.get(&(callee_fn, call_site)) {
            return (id, false);
        }
        // Cap check: too many contexts for this function → collapse into the
        // first existing one (merging all call sites = context-insensitive
        // treatment of this callee, still sound).
        let count = self.per_fn_count.entry(callee_fn).or_insert(0);
        if *count >= MAX_CONTEXTS_PER_FN {
            self.collapses += 1;
            let fallback = self
                .by_key
                .iter()
                .find(|((f, _), _)| *f == callee_fn)
                .map(|(_, &id)| id)
                .unwrap_or(ROOT_CTX);
            return (fallback, true);
        }
        *count += 1;
        let id = self.next_id;
        self.next_id += 1;
        self.by_key.insert((callee_fn, call_site), id);
        (id, false)
    }
}

// ---------------------------------------------------------------------------
// Context-sensitive engine
// ---------------------------------------------------------------------------

/// Forward taint engine with selective k=1 call-site sensitivity.
///
/// Phase 1 reuses [`InterproceduralTaintEngine`] (k=0) to compute the
/// taint-relevant function set. Phase 2 re-runs propagation with
/// `(function, node, context)` states, splitting only relevant functions.
pub struct ContextSensitiveTaintEngine<'a> {
    prog: &'a ProgramSvfg<'a>,
    config: &'a TaintConfig,
    stats: ContextStats,
    alerts: Vec<ContextualAlert>,
    /// Final phase-2 taint states: `(fn, node) → set of contexts` (for tests
    /// and downstream consumers).
    tainted: FxHashMap<(usize, NodeKey), FxHashSet<CtxId>>,
}

impl<'a> ContextSensitiveTaintEngine<'a> {
    pub fn new(prog: &'a ProgramSvfg<'a>, config: &'a TaintConfig) -> Self {
        Self {
            prog,
            config,
            stats: ContextStats::default(),
            alerts: Vec::new(),
            tainted: FxHashMap::default(),
        }
    }

    pub fn stats(&self) -> &ContextStats {
        &self.stats
    }

    pub fn alerts(&self) -> &[ContextualAlert] {
        &self.alerts
    }

    /// Plain alert messages (compatible with the other engines).
    pub fn alert_messages(&self) -> Vec<String> {
        self.alerts.iter().map(|a| a.message.clone()).collect()
    }

    /// Taint map for tests / downstream consumers.
    pub fn tainted(&self) -> &FxHashMap<(usize, NodeKey), FxHashSet<CtxId>> {
        &self.tainted
    }

    /// Run phase 1 (k=0 relevance) + phase 2 (selective k=1).
    pub fn run(&mut self) {
        // ---------------- Phase 1: cheap k=0 relevance pass ----------------
        let mut p1 = InterproceduralTaintEngine::new(self.prog, self.config);
        p1.run();
        let relevant: FxHashSet<usize> = p1.tainted_nodes().iter().map(|(f, _)| *f).collect();
        self.stats.phase1_states = p1.tainted_nodes().len();
        self.stats.relevant_functions = relevant.len();
        self.stats.total_functions = self.prog.functions.len();
        drop(p1);

        // ---------------- Phase 2: selective k=1 propagation ----------------
        self.run_phase2(&relevant);
    }

    /// Context-sensitive BFS. States are `(fn, node, ctx)`.
    ///
    /// Edge semantics:
    ///   * Local edges keep the context unchanged.
    ///   * Crossing an actual-arg → formal-param edge of callee `G` from call
    ///     site `c` **creates** context `(G, c)` (only if `G` is relevant).
    ///   * Crossing a formal-ret → actual-ret edge requires the callee-side
    ///     context to match: a `FormalRet(G)` under context `(G, c)` may only
    ///     return to **call site `c`**'s `ActualRet` node.
    fn run_phase2(&mut self, relevant: &FxHashSet<usize>) {
        let mut queue: VecDeque<(usize, NodeKey, CtxId)> = VecDeque::new();
        let mut pool = ContextPool::default();

        let mark = |tainted: &mut FxHashMap<(usize, NodeKey), FxHashSet<CtxId>>,
                    f: usize,
                    k: NodeKey,
                    c: CtxId|
         -> bool { tainted.entry((f, k)).or_default().insert(c) };
        // Seeds: source definitions at the ROOT context.
        for fi in 0..self.prog.functions.len() {
            let fe = &self.prog.functions[fi];
            let mut keys: Vec<NodeKey> = fe.svfg.nodes.keys().copied().collect();
            keys.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
            for k in keys {
                if is_source(fe.ir, self.config, &k) && mark(&mut self.tainted, fi, k, ROOT_CTX) {
                    queue.push_back((fi, k, ROOT_CTX));
                }
            }
        }

        while let Some((fi, cur, ctx)) = queue.pop_front() {
            let fe = &self.prog.functions[fi];

            if let Some((_, alert)) = sink_alert(fe.ir, self.config, &cur) {
                let ca = ContextualAlert {
                    message: alert,
                    function: fe.name.clone(),
                    context: ctx,
                };
                // Dedup by message: the same sink reached via two different
                // (genuinely distinct) contexts is one reportable finding.
                if !self.alerts.iter().any(|a| a.message == ca.message) {
                    self.alerts.push(ca);
                }
            }

            // --- Local edges: context unchanged ---
            for succ in self.prog.local_successors(fi, &cur) {
                if is_sanitizer_use(fe.ir, self.config, &succ) {
                    continue;
                }
                if mark(&mut self.tainted, fi, succ, ctx) {
                    queue.push_back((fi, succ, ctx));
                }
            }

            // --- Interprocedural edges, context-matched ---
            for (gi, gk) in self.prog.inter_successors(fi, &cur) {
                let ge = &self.prog.functions[gi];
                if is_sanitizer_use(ge.ir, self.config, &gk) {
                    continue;
                }
                let gk_kind = ge.svfg.node(&gk).map(|n| n.kind.clone());

                match gk_kind {
                    // Parameter passing: enter callee G, creating/entering
                    // context (G, this call site). The call site is encoded in
                    // the arg's NodeKind::ActualArg.call_site.
                    Some(crate::graph::svfg::NodeKind::FormalParam) => {
                        // Only split contexts for relevant callees.
                        if !relevant.contains(&gi) {
                            // k=0 fallback: reuse a single shared context
                            // (ROOT_CTX) for all non-relevant callees.
                            if mark(&mut self.tainted, gi, gk, ROOT_CTX) {
                                queue.push_back((gi, gk, ROOT_CTX));
                            }
                            continue;
                        }
                        // Recover the call site from the FROM node: we got
                        // here from an ActualArg/ActualRet node whose kind
                        // carries `call_site`.
                        let call_site = match fe.svfg.node(&cur).map(|n| n.kind.clone()) {
                            Some(crate::graph::svfg::NodeKind::ActualArg { call_site, .. })
                            | Some(crate::graph::svfg::NodeKind::ActualRet { call_site }) => {
                                call_site
                            }
                            _ => continue,
                        };
                        let (new_ctx, collapsed) = pool.intern(gi, call_site);
                        if collapsed {
                            self.stats.context_collapses += 1;
                        }
                        if mark(&mut self.tainted, gi, gk, new_ctx) {
                            queue.push_back((gi, gk, new_ctx));
                        }
                    }
                    // Return passing: FormalRet(G) → ActualRet(caller). The
                    // context must match: only return to the call site we
                    // entered G through. We are standing at FormalRet(G) (fn
                    // fi, context ctx); the edge target's kind carries the
                    // TARGET call site, cross only if it equals the call
                    // site this context entered fi through.
                    Some(crate::graph::svfg::NodeKind::ActualRet { call_site }) => {
                        if ctx == ROOT_CTX {
                            // Non-relevant callee (k=0 fallback) or a root:
                            // return to ALL call sites (sound over-approximation).
                            if mark(&mut self.tainted, gi, gk, ctx) {
                                queue.push_back((gi, gk, ctx));
                            }
                            continue;
                        }
                        // Find the call site this context entered fi through
                        // (we are inside fi here, so the context's function
                        // is fi, NOT gi, which is the caller we're returning to).
                        let entered_via = pool
                            .by_key
                            .iter()
                            .find(|(k, id)| k.0 == fi && **id == ctx)
                            .map(|(k, _)| k.1);
                        match entered_via {
                            Some(cs)
                                if cs == call_site
                                // Correct call site: propagate.
                                && mark(&mut self.tainted, gi, gk, ctx) =>
                            {
                                queue.push_back((gi, gk, ctx));
                            }
                            _ => {
                                // Wrong call site for this context: skip.
                            }
                        }
                    }
                    _ => {
                        // Any other cross edge shape: context-insensitive.
                        if mark(&mut self.tainted, gi, gk, ctx) {
                            queue.push_back((gi, gk, ctx));
                        }
                    }
                }
            }
        }

        self.stats.phase2_states = self.tainted.values().map(|s| s.len()).sum();
        self.stats.contexts_created = pool.next_id as usize;
    }
}
