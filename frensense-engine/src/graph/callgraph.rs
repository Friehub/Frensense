// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Call-graph construction and dispatch resolution (§8.1)
//!
//! [`interprocedural::ProgramSvfg::discover_bindings`] resolves callees by
//! **exact string match** against a flat function-name index. That breaks on
//! the three constructs real code is made of:
//!
//! 1. **Aliased callees**, `const h = helper; h(x)`, re-exports. The call
//!    site says `h`, the function is named `helper`.
//! 2. **Method calls**, `db.query(...)` lowers to `CallVirtual { method:
//!    "query", receiver }`. The callee is `query` *on the receiver's class*,
//!    `query` alone matches every class in the program.
//! 3. **Higher-order calls**, `CallPointer { func_ptr: Var(v) }` where `v`
//!    holds a function reference or a callback parameter
//!    (`router.get(path, handler)` lowers the handler reference to a Var).
//!
//! This module resolves all three into [`CallTarget`]s per call site, using
//! only information in the lowered IRs:
//!
//! * **Alias chains**, SSA `Assign` chains over function-reference Vars.
//! * **Class method tables**, `Allocate { kind: ClassInstance(class) }` ties
//!   a receiver Var to its class; `this`-receivers resolve through the
//!   enclosing method's qualified name (`Class.method`).
//! * **Callback (higher-order) edges**, two-step Andersen-style treatment at
//!   function granularity: (a) find parameters that are *called* anywhere,
//!   (b) for each call site passing a function reference into such a
//!   parameter slot, add an edge caller → referenced function.
//!
//! Unresolved sites stay unresolved: the interprocedural layer keeps its
//! sound conservative pass-through edges for them. Resolution only ever adds
//! precision, it never removes the sound fallback.
//!
//! ## Determinism
//!
//! All iterations run over sorted keys; candidate lists are sorted+deduped.
//! The graph is byte-stable across runs (the corpus store hashes it).

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;

use crate::ir::function::*;

// ---------------------------------------------------------------------------
// Targets
// ---------------------------------------------------------------------------

/// A resolved callee. One call site may resolve to several targets (dynamic
/// dispatch, callbacks); the interprocedural layer installs cross edges for
/// every target, sound (any target may run) and precise enough for taint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CallTarget {
    /// A function in the analysed program, by its program-level key.
    Function(String),
    /// `Class.method` dispatch on a receiver of known class.
    Method { class: String, method: String },
    /// Callee outside the analysed program (library, builtin, unresolved
    /// indirect). The source/sink fact layer classifies these by name.
    External(String),
}

impl CallTarget {
    /// The program-function key this target resolves to, if internal.
    /// Method targets resolve under the `Class.method` key convention (see
    /// [`method_key`]).
    pub fn internal_key(&self) -> Option<String> {
        match self {
            CallTarget::Function(n) => Some(n.clone()),
            CallTarget::Method { class, method } => Some(method_key(class, method)),
            CallTarget::External(_) => None,
        }
    }

    pub fn is_external(&self) -> bool {
        matches!(self, CallTarget::External(_))
    }
}

/// Program-index key for a method implementation. The project lowering
/// registers methods qualified as `Class.method`; bare-named duplicates are
/// matched by the caller (interprocedural layer) as a fallback.
pub fn method_key(class: &str, method: &str) -> String {
    format!("{class}.{method}")
}

// ---------------------------------------------------------------------------
// The graph
// ---------------------------------------------------------------------------

/// Resolved call graph: per-call-site targets plus program-level edges.
#[derive(Debug, Default)]
pub struct CallGraph {
    /// Program functions, sorted (the IR map's keys).
    pub functions: Vec<String>,
    /// `(caller_fn, (block, instr_idx)) → resolved targets` for every call
    /// instruction encountered. Sites whose *only* target is external are
    /// recorded in [`CallGraph::unresolved`] instead.
    pub sites: FxHashMap<(String, (usize, usize)), Vec<CallTarget>>,
    /// Internal edges: caller → sorted callee keys.
    pub edges: FxHashMap<String, Vec<String>>,
    /// Reverse internal edges: callee → sorted callers.
    pub reverse_edges: FxHashMap<String, Vec<String>>,
    /// Sites with no internal target, mapped to their best-known external
    /// name (for the fact layer and diagnostics).
    pub unresolved: FxHashMap<(String, (usize, usize)), String>,
}

impl CallGraph {
    /// Direct internal callees of `fn` (sorted).
    pub fn callees_of(&self, fn_name: &str) -> &[String] {
        self.edges.get(fn_name).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Direct internal callers of `fn` (sorted).
    pub fn callers_of(&self, fn_name: &str) -> &[String] {
        self.reverse_edges
            .get(fn_name)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Targets for one call instruction inside `fn`.
    pub fn targets_at(&self, fn_name: &str, block: usize, instr_idx: usize) -> &[CallTarget] {
        self.sites
            .get(&(fn_name.to_string(), (block, instr_idx)))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// Per-function value facts gathered in pass 1.
#[derive(Default)]
struct FnFacts {
    /// `dest → src` copy assignments (alias chains).
    aliases: FxHashMap<VarId, VarId>,
    /// Vars known to hold a reference to a program function.
    fn_refs: FxHashMap<VarId, String>,
    /// Vars known to be instances of a class (`new Class()`).
    classes: FxHashMap<VarId, String>,
}

/// Input: every function's IR, keyed by program-level name (the same map the
/// `ProgramSvfg` builder consumes).
pub struct CallGraphBuilder<'a> {
    irs: &'a FxHashMap<String, &'a FunctionIR>,
}

impl<'a> CallGraphBuilder<'a> {
    pub fn new(irs: &'a FxHashMap<String, &'a FunctionIR>) -> Self {
        Self { irs }
    }

    pub fn build(&self) -> CallGraph {
        let mut functions: Vec<String> = self.irs.keys().cloned().collect();
        functions.sort();
        let fn_set: FxHashSet<&str> = functions.iter().map(|s| s.as_str()).collect();

        // ---- Pass 1: per-function value facts ------------------------------
        let mut facts: FxHashMap<&str, FnFacts> = FxHashMap::default();
        for (fname, ir) in self.irs {
            let mut f = FnFacts::default();
            for block in ir.blocks.values() {
                for instr in &block.instructions {
                    match instr {
                        Instruction::Assign { dest, src, .. } => match src {
                            Operand::Var(s) => {
                                f.aliases.insert(*dest, *s);
                            }
                            Operand::StringLiteral(name) if fn_set.contains(name.as_str()) => {
                                f.fn_refs.insert(*dest, name.clone());
                            }
                            _ => {}
                        },
                        Instruction::AddressOf { dest, src, .. } => {
                            // `&func`, the address-of a parameter can't be a
                            // function; a real fn-ref lowering path lands here
                            // when IR gains named function constants. Track it
                            // as an alias for now (conservative no-op unless
                            // src is later known to be a fn ref).
                            f.aliases.insert(*dest, *src);
                        }
                        Instruction::Allocate {
                            dest,
                            kind: AllocationKind::ClassInstance(class),
                            ..
                        } => {
                            f.classes.insert(*dest, class.clone());
                        }
                        _ => {}
                    }
                }
            }
            facts.insert(fname.as_str(), f);
        }

        // ---- Pass 2: which formal parameters are CALLED? -------------------
        // `(fn_name, param_slot)`, the two-step higher-order treatment.
        let mut called_params: FxHashSet<(String, usize)> = FxHashSet::default();
        for (fname, ir) in self.irs {
            for block in ir.blocks.values() {
                for instr in &block.instructions {
                    let func_ptr: Option<&Operand> = match instr {
                        Instruction::CallPointer { func_ptr, .. } => Some(func_ptr),
                        _ => None,
                    };
                    if let Some(Operand::Var(v)) = func_ptr {
                        for (slot, &pv) in ir.parameters.iter().enumerate() {
                            if pv == *v {
                                called_params.insert(((*fname).clone(), slot));
                            }
                        }
                    }
                }
            }
        }

        // ---- Pass 3: resolve every call site -------------------------------
        let mut graph = CallGraph {
            functions,
            ..CallGraph::default()
        };

        for (fname, ir) in self.irs {
            let f = &facts[fname.as_str()];

            for block in ir.blocks.values() {
                for (idx, instr) in block.instructions.iter().enumerate() {
                    let site_key = ((*fname).clone(), (block.id.0, idx));
                    let mut targets: Vec<CallTarget> = Vec::new();
                    let mut external_name: Option<String> = None;

                    match instr {
                        Instruction::CallStatic { func, .. } => {
                            if self.irs.contains_key(func) {
                                targets.push(CallTarget::Function(func.clone()));
                            } else {
                                external_name = Some(func.clone());
                            }
                        }

                        Instruction::CallVirtual {
                            method, receiver, ..
                        } => {
                            // Receiver → class:
                            let class: Option<String> = match receiver {
                                Operand::Var(r) => f.classes.get(r).cloned().or_else(|| {
                                    // Follow alias chains to an allocation.
                                    self.follow_alias_to_class(*r, f)
                                }),
                                _ => None,
                            };
                            // `this` receiver: the enclosing function's own
                            // name may be a qualified `Class.method`.
                            let class = class.or_else(|| this_class_of(fname));

                            match class {
                                Some(c) => {
                                    let key = method_key(&c, method);
                                    if self.irs.contains_key(&key) {
                                        targets.push(CallTarget::Method {
                                            class: c,
                                            method: method.clone(),
                                        });
                                    } else if self.irs.contains_key(method) {
                                        // Unqualified registration fallback.
                                        targets.push(CallTarget::Function(method.clone()));
                                        external_name = Some(method.clone());
                                    } else {
                                        external_name = Some(method.clone());
                                    }
                                }
                                None => {
                                    // Unknown receiver (library object or
                                    // unresolved import). External, but if a
                                    // program function shares the bare name,
                                    // keep it as a candidate too (sound).
                                    if self.irs.contains_key(method) {
                                        targets.push(CallTarget::Function(method.clone()));
                                    }
                                    external_name = Some(method.clone());
                                }
                            }
                        }

                        Instruction::CallPointer { func_ptr, .. } => {
                            match func_ptr {
                                Operand::Var(v) => {
                                    match self.follow_alias_to_fn(*v, f) {
                                        Some(fun) => targets.push(CallTarget::Function(fun)),
                                        None => {
                                            // Formal parameter called as a
                                            // function pointer: resolved by
                                            // the callback pass (step 4) when
                                            // callers pass fn refs into this
                                            // slot. Nothing to record here.
                                            let is_called_param =
                                                ir.parameters.iter().enumerate().any(
                                                    |(slot, &pv)| {
                                                        pv == *v
                                                            && called_params
                                                                .contains(&((*fname).clone(), slot))
                                                    },
                                                );
                                            if !is_called_param {
                                                external_name = Some("<indirect>".to_string());
                                            }
                                        }
                                    }
                                }
                                Operand::StringLiteral(name) => {
                                    if self.irs.contains_key(name) {
                                        targets.push(CallTarget::Function(name.clone()));
                                    } else {
                                        external_name = Some(name.clone());
                                    }
                                }
                                _ => {
                                    external_name = Some("<indirect>".to_string());
                                }
                            }
                        }

                        _ => continue,
                    }

                    // ---- Step 4 (callback edges): every call that passes a
                    // function reference into a CALLED parameter slot of an
                    // internal callee gains an edge to the referenced fn.
                    // Handled below in `install_callback_edges`.

                    targets.sort();
                    targets.dedup();

                    let has_internal = targets.iter().any(|t| !t.is_external());
                    match (has_internal, external_name) {
                        (true, _) => {
                            graph.sites.insert(site_key, targets);
                        }
                        (false, Some(name)) => {
                            graph.unresolved.insert(site_key, name);
                        }
                        (false, None) => {
                            graph.sites.insert(site_key, targets);
                        }
                    }
                }
            }
        }

        // ---- Step 4: callback (higher-order) edges --------------------------
        // For each internal call site passing a fn-ref Var at argument slot
        // `s`, where slot `s` of the callee is a called parameter, add an
        // edge to the referenced function, as an extra target on the site.
        let mut extra_targets: FxHashMap<(String, (usize, usize)), Vec<CallTarget>> =
            FxHashMap::default();

        for (fname, ir) in self.irs {
            let f = &facts[fname.as_str()];
            for block in ir.blocks.values() {
                for (idx, instr) in block.instructions.iter().enumerate() {
                    let (callee_name, args): (Option<String>, &[Operand]) = match instr {
                        Instruction::CallStatic { func, args, .. } => (Some(func.clone()), args),
                        Instruction::CallVirtual { method, args, .. } => {
                            (Some(method.clone()), args)
                        }
                        Instruction::CallPointer { args, .. } => (None, args),
                        _ => continue,
                    };

                    // Argument slots: no receiver here (receiver occupies
                    // slot 0 in the interprocedural layer, but the callee's
                    // *formal* slots line up with the explicit args only for
                    // static calls; virtual calls shift by one, handled by
                    // using explicit-arg position + 1 for virtual).
                    let is_virtual = matches!(instr, Instruction::CallVirtual { .. });
                    let slot_offset = if is_virtual { 1 } else { 0 };

                    for (pos, a) in args.iter().enumerate() {
                        // Args carry fn refs either directly (a string literal
                        // naming a program function) or via a Var whose alias
                        // chain resolves to one. Both shapes occur in lowered
                        // code; treat them identically.
                        let target_fn: String = match a {
                            Operand::Var(av) => match self.follow_alias_to_fn(*av, f) {
                                Some(t) => t,
                                None => continue,
                            },
                            Operand::StringLiteral(name) => {
                                if !self.irs.contains_key(name) {
                                    continue;
                                }
                                name.clone()
                            }
                            _ => continue,
                        };
                        // Which internal function will receive it?
                        let callee_key: Option<String> = match instr {
                            Instruction::CallStatic { func, .. } => {
                                if self.irs.contains_key(func) {
                                    Some(func.clone())
                                } else {
                                    None
                                }
                            }
                            Instruction::CallVirtual {
                                method, receiver, ..
                            } => {
                                let class = match receiver {
                                    Operand::Var(r) => f.classes.get(r).cloned(),
                                    _ => None,
                                }
                                .or_else(|| this_class_of(fname));
                                match class {
                                    Some(c) => {
                                        let k = method_key(&c, method);
                                        if self.irs.contains_key(&k) {
                                            Some(k)
                                        } else if self.irs.contains_key(method) {
                                            Some(method.clone())
                                        } else {
                                            None
                                        }
                                    }
                                    None => {
                                        if self.irs.contains_key(method) {
                                            Some(method.clone())
                                        } else {
                                            None
                                        }
                                    }
                                }
                            }
                            _ => None, // CallPointer target resolution above
                        };
                        let Some(callee_key) = callee_key else {
                            continue;
                        };
                        let formal_slot = pos + slot_offset;
                        if called_params.contains(&(callee_key.clone(), formal_slot)) {
                            extra_targets
                                .entry(((*fname).clone(), (block.id.0, idx)))
                                .or_default()
                                .push(CallTarget::Function(target_fn));
                        }
                    }
                    let _ = callee_name; // resolved inline above
                }
            }
        }

        for (site, mut tgts) in extra_targets {
            tgts.sort();
            tgts.dedup();
            let entry = graph.sites.entry(site).or_default();
            for t in tgts {
                if !entry.contains(&t) {
                    entry.push(t);
                }
            }
            entry.sort();
            entry.dedup();
        }

        // ---- Finalize edges --------------------------------------------------
        let mut edges: FxHashMap<String, BTreeSet<String>> = Default::default();
        let mut redges: FxHashMap<String, BTreeSet<String>> = Default::default();
        let fn_names: FxHashSet<&str> = graph.functions.iter().map(|s| s.as_str()).collect();
        for ((caller, _site), targets) in &graph.sites {
            for t in targets {
                if let Some(key) = t.internal_key()
                    && fn_names.contains(key.as_str())
                {
                    edges.entry(caller.clone()).or_default().insert(key.clone());
                    redges.entry(key).or_default().insert(caller.clone());
                }
            }
        }
        graph.edges = edges
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        graph.reverse_edges = redges
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        graph
    }

    /// Follow `v`'s alias chain to a function reference, if any.
    fn follow_alias_to_fn(&self, v: VarId, f: &FnFacts) -> Option<String> {
        let mut cur = v;
        let mut hops = 0;
        loop {
            if let Some(fun) = f.fn_refs.get(&cur) {
                return Some(fun.clone());
            }
            let next = f.aliases.get(&cur)?;
            if *next == cur || hops > 32 {
                return None; // cycle guard
            }
            cur = *next;
            hops += 1;
        }
    }

    /// Follow `v`'s alias chain to a class allocation, if any.
    fn follow_alias_to_class(&self, v: VarId, f: &FnFacts) -> Option<String> {
        let mut cur = v;
        let mut hops = 0;
        loop {
            if let Some(c) = f.classes.get(&cur) {
                return Some(c.clone());
            }
            let next = f.aliases.get(&cur)?;
            if *next == cur || hops > 32 {
                return None;
            }
            cur = *next;
            hops += 1;
        }
    }
}

/// If `fname` is a qualified method name (`Class.method`), return `Class`.
fn this_class_of(fname: &str) -> Option<String> {
    fname.rsplit_once('.').map(|(c, _)| c.to_string())
}
