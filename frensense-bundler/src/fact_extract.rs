// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Fact extraction from positive/negative corpus pairs (§9).
//!
//! Replaces the old shape/fingerprint pipeline. For every corpus family:
//!
//! 1. Scan positives and negatives through the **shared engine harness**
//!    (`engine::data_flow::scan`) under the built-in fact table.
//! 2. Compute the delta: sink/sanitizer candidates from how taint moves (or
//!    fails to move) through each variant.
//! 3. **Replay gate**: apply candidate facts, re-scan the family; publish a
//!    fact only if the family separates (positives alert, negatives don't)
//!    and no other family's negatives newly alert.
//! 4. Publish with `support` = number of variants that voted. support ≥ 2 is
//!    `confirmed`, otherwise `provisional`.

use std::collections::BTreeSet;
use std::path::Path;

use frensense_engine::analysis::taint::config::TaintConfig;
use frensense_engine::analysis::taint::facts::{
    FactTable, LearnedCheckFact, LearnedFactEntry, SanitizerFact, SinkSignature,
};
use frensense_engine::scan;
use rustc_hash::FxHashMap;

/// A published learned fact with its provenance.
#[derive(Debug, Clone)]
pub struct LearnedFact {
    /// The fact itself, bundle-ready.
    pub entry: LearnedFactEntry,
    /// Number of corpus variants that voted for this fact.
    pub support: u32,
    /// `confirmed` (support ≥ 2) or `provisional` (support = 1).
    pub status: String,
    /// Families that voted for this fact.
    pub families: Vec<String>,
}

/// One corpus family: id + variant files.
pub struct Family {
    pub id: String,
    /// (file name, source text, ext)
    pub positives: Vec<(String, String, String)>,
    pub negatives: Vec<(String, String, String)>,
    /// Declared check trigger from the positive's `[frensense]` metadata
    /// (`check-call: name`). When present, Check proposals are restricted
    /// to this call, the family authors declare which call is the
    /// privileged action; the gate still validates the claim.
    pub declared_check_call: Option<String>,
}

/// Group a corpus directory into families by filename convention
/// (`<family>_positive*.ts`, `<family>_negative*.ts`).
pub fn group_families(corpus_dir: &Path) -> Result<Vec<Family>, String> {
    let mut families: FxHashMap<String, Family> = FxHashMap::default();

    // Recursive walk: the corpus is organized as a manifest tree
    // (`<lang>/<CWE>/<family>/<stem>_<variant>.<ext>`), with flat files
    // still supported at the root of `corpus_dir`. Family identity is the
    // file stem before `_positive`/`_negative`; directory structure is
    // metadata (recorded in each family's manifest.json), never part of
    // the id, so moving a family between CWE directories preserves its
    // learned facts.
    let mut stack = vec![corpus_dir.to_path_buf()];
    let mut all_paths: Vec<std::path::PathBuf> = Vec::new();
    while let Some(dir) = stack.pop() {
        let entries =
            std::fs::read_dir(&dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                all_paths.push(path);
            }
        }
    }
    all_paths.sort();

    for path in all_paths {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        if !matches!(
            ext.as_str(),
            "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "rs"
        ) {
            continue;
        }
        let (family, variant) = if let Some(i) = name.find("_positive") {
            (name[..i].to_string(), "positive")
        } else if let Some(i) = name.find("_negative") {
            (name[..i].to_string(), "negative")
        } else {
            continue;
        };
        let source = std::fs::read_to_string(&path).unwrap_or_default();
        // Declared check trigger: `// check-call: <name>` inside the
        // `[frensense]` metadata block of a positive variant.
        let declared_check_call = if variant == "positive" {
            source
                .lines()
                .take(30)
                .find_map(|l| l.trim().strip_prefix("// check-call:"))
                .map(|s| s.trim().to_string())
        } else {
            None
        };
        let f = families.entry(family.clone()).or_insert_with(|| Family {
            id: family.clone(),
            positives: Vec::new(),
            negatives: Vec::new(),
            declared_check_call: None,
        });
        if declared_check_call.is_some() {
            f.declared_check_call = declared_check_call;
        }
        let slot = match variant {
            "positive" => &mut f.positives,
            _ => &mut f.negatives,
        };
        slot.push((name.to_string(), source, ext));
    }

    let mut out: Vec<Family> = families.into_values().collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

/// A candidate fact proposed by delta analysis.
#[derive(Debug, Clone)]
enum Candidate {
    /// Call is a dangerous sink at these arg slots.
    Sink {
        call: String,
        dangerous: BTreeSet<usize>,
    },
    /// Call sanitizes (guard-style or transform).
    Sanitizer { call: String, guard: bool },
    /// Call is a policy violation per a corpus-verified non-dataflow rule
    /// (positive contains the call and alerts-or-should; negative lacks it,
    /// or enforces it with a guard the positive lacks).
    Check {
        rule: String,
        call: String,
        message: String,
        /// Fire only when this guard is absent (shape-b families).
        unless_guard: Option<String>,
        /// Fire only when the trigger arg has NO literal range comparison
        /// (inline enforcement modality).
        unless_range_check: Option<Vec<String>>,
    },
}

/// Taint-relevant calls observed in one variant, with arg-slot detail.
/// Uses the engine's scan; the deltas come from comparing what taint reached.
fn scan_variant(
    files: &[(String, String, String)],
    config: &TaintConfig,
    facts: &FactTable,
) -> frensense_engine::scan::ScanResult {
    scan::scan(files, config, facts)
}

/// One family's variants, lowered once. The gate re-scans families per
/// candidate fact; without pre-lowering the gate is O(candidates × families)
/// lowerings and times out at corpus scale.
struct PreparedFamily {
    id: String,
    pos: frensense_engine::scan::PreparedProgram,
    neg: frensense_engine::scan::PreparedProgram,
    /// All call names appearing in any variant, used to skip gate trials
    /// for facts that cannot affect this family.
    calls: std::collections::BTreeSet<String>,
}

impl PreparedFamily {
    fn new(f: &Family) -> Result<Self, String> {
        let pos = frensense_engine::scan::prepare(&f.positives)?;
        let neg = frensense_engine::scan::prepare(&f.negatives)?;
        let mut calls = std::collections::BTreeSet::new();
        calls.extend(
            collect_calls(&f.positives)
                .keys()
                .cloned()
                .chain(collect_calls(&f.negatives).keys().cloned()),
        );
        Ok(Self {
            id: f.id.clone(),
            pos,
            neg,
            calls,
        })
    }

    /// Family separation under a fact table. Reuses the lowered IRs;
    /// each call re-runs the taint engine + checker only.
    fn separates(&self, config: &TaintConfig, facts: &FactTable) -> bool {
        let pos = scan::scan_prepared(&self.pos, config, facts);
        let neg = scan::scan_prepared(&self.neg, config, facts);
        pos.has_alert() && !neg.has_alert()
    }
}

/// Propose candidate facts for one family from the pos/neg delta.
///
/// Signal extraction per family:
/// * If positives alert and negatives don't under the built-in table, the
///   family *confirms* current handling (no new fact needed).
/// * If positives do NOT alert (a real flow the engine missed), look for
///   calls in positives that the built-in table doesn't know → propose them
///   as sinks (all-args; the replay gate validates).
/// * If negatives contain guard/transform calls between the source and the
///   sink that positives lack → propose sanitizer facts.
fn propose(family: &Family, config: &TaintConfig, builtin: &FactTable) -> Vec<Candidate> {
    let mut candidates = Vec::new();

    let pos = scan_variant(&family.positives, config, builtin);
    let neg = scan_variant(&family.negatives, config, builtin);
    let pos_alerts = pos.has_alert();
    let neg_alerts = neg.has_alert();

    if pos_alerts && !neg_alerts {
        // Family already separates, no new fact needed. But if the NEGATIVE
        // showed `Unknown` verdicts, the negative may be safe only by luck.
        // Still no fact proposal (conservative).
        return candidates;
    }

    // Guard/transform calls present in negatives but not in positives,
    // these are likely the *reason* the negative is safe.
    let pos_calls = collect_calls(&family.positives);
    let neg_calls = collect_calls(&family.negatives);
    for (call, shape) in &neg_calls {
        if pos_calls.contains_key(call) || builtin.sanitizer_fact(call).is_some() {
            continue;
        }
        if !looks_taint_relevant(call) {
            continue;
        }
        candidates.push(Candidate::Sanitizer {
            call: call.clone(),
            guard: *shape == CallShape::Predicate,
        });
    }

    // Calls in positives that no table knows and no candidate yet → sink
    // proposals (validated by replay).
    if !pos_alerts {
        for call in pos_calls.keys() {
            if builtin.sink_signature(call).is_some() || builtin.sanitizer_fact(call).is_some() {
                continue;
            }
            if !looks_taint_relevant(call) {
                continue;
            }
            if candidates
                .iter()
                .any(|c| matches!(c, Candidate::Sanitizer { call: c2, .. } if c2 == call))
            {
                continue;
            }
            candidates.push(Candidate::Sink {
                call: call.clone(),
                dangerous: BTreeSet::new(), // all args; replay validates
            });
        }
    }

    // Non-dataflow policy deltas: the two family shapes that express
    // "trigger without enforcement":
    //
    // (a) trigger call present in positives, ABSENT from negatives, the
    //     call itself is the violation (presence-only check);
    // (b) trigger call present in BOTH, but negatives contain a guard call
    //     (clamp/validate/allowlist helper) that positives lack, the
    //     violation is executing the trigger WITHOUT the guard. The fact
    //     carries `unless_guard` so the engine fires only when the guard
    //     is missing. This is the chatbot/privileged-tool shape.
    for call in pos_calls.keys() {
        if builtin.learned_checks.iter().any(|c| c.call == *call) {
            continue; // already learned
        }
        // Family-declared trigger restricts Check proposals to the declared
        // call; undeclared families keep the delta-driven path.
        if let Some(declared) = &family.declared_check_call {
            if call != declared {
                continue;
            }
        } else if !looks_taint_relevant(call) {
            continue;
        }
        let rule = format!("policy_{call}");
        let message = format!(
            "Corpus-verified policy violation: `{call}` (learned from family {})",
            family.id
        );
        // Enforcement modalities the negatives demonstrate. A negative can
        // enforce via a named helper, an inline literal range check, or
        // both; the fact records every modality observed and the engine
        // stays silent when ANY of them matches. When the negatives show
        // no enforcement at all, the fact is presence-only.
        let mut unless_guard: Option<String> = None;
        let mut unless_range_check: Option<Vec<String>> = None;
        if neg_calls.contains_key(call) {
            // Helper modality: absent from every positive, present in at
            // least one negative (prefer a helper common to all negatives).
            // Each negative need only be suppressed by ONE modality, the
            // gate validates the combination end-to-end.
            let pos_has = |g: &str| pos_calls.contains_key(g);
            let all_negs_have = |g: &str| {
                family
                    .negatives
                    .iter()
                    .map(|v| collect_calls(std::slice::from_ref(v)))
                    .all(|calls| calls.contains_key(g))
            };
            unless_guard = neg_calls
                .keys()
                .find(|g| !pos_has(g) && !builtin.sanitizer_fact(g).is_some() && all_negs_have(g))
                .or_else(|| {
                    neg_calls
                        .keys()
                        .find(|g| !pos_has(g) && !builtin.sanitizer_fact(g).is_some())
                })
                .cloned();
            // Inline modality: any negative compares a trigger-argument var
            // against a literal bound.
            if family
                .negatives
                .iter()
                .any(|v| variant_has_range_check_on_call(&[v.clone()], call))
            {
                unless_range_check = Some(vec!["<".into(), ">".into(), "<=".into(), ">=".into()]);
            }
            if unless_guard.is_none() && unless_range_check.is_none() {
                // Negatives contain the trigger but demonstrate no
                // recognizable enforcement: no check fact, the family
                // doesn't yet teach a suppressible difference.
                continue;
            }
        }
        candidates.push(Candidate::Check {
            rule,
            call: call.clone(),
            message,
            unless_guard,
            unless_range_check,
        });
    }

    candidates
}

/// True when any argument var passed to `call` is compared against a
/// literal with a range operator (<, >, <=, >=) in the variant's IR,
/// inline range enforcement.
fn variant_has_range_check_on_call(files: &[(String, String, String)], call: &str) -> bool {
    use frensense_engine::ir::function::{Instruction, Operand};
    const RANGE_OPS: &[&str] = &["<", ">", "<=", ">="];
    let Some((path, source, ext)) = files.first() else {
        return false;
    };
    let Ok(irs) = frensense_engine::harness::lower_source(path, source, ext) else {
        return false;
    };
    for ir in irs.values() {
        // Vars passed as arguments to the trigger call.
        let mut arg_vars: Vec<_> = Vec::new();
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                let is_call = match instr {
                    Instruction::CallStatic { func, .. } => func.rsplit('.').next() == Some(call),
                    Instruction::CallVirtual { method, .. } => {
                        method.rsplit('.').next() == Some(call)
                    }
                    _ => false,
                };
                let args = match instr {
                    Instruction::CallStatic { args, .. }
                    | Instruction::CallVirtual { args, .. } => Some(args),
                    _ => None,
                };
                if is_call && args.is_some() {
                    for a in args.unwrap() {
                        if let Operand::Var(v) = a {
                            arg_vars.push(*v);
                        }
                    }
                }
            }
        }
        // Any literal comparison on those vars?
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                let op = match instr {
                    Instruction::BinaryOp { op, .. } => op,
                    _ => continue,
                };
                if !RANGE_OPS.contains(&op.as_str()) {
                    continue;
                }
                if let Instruction::BinaryOp { lhs, rhs, .. } = instr {
                    let (a, b) = (lhs, rhs);
                    let var_side = matches!(a, Operand::Var(v) if arg_vars.contains(v))
                        || matches!(b, Operand::Var(v) if arg_vars.contains(v));
                    let lit_side = matches!(
                        b,
                        Operand::StringLiteral(_)
                            | Operand::IntLiteral(_)
                            | Operand::FloatLiteral(_)
                    ) || matches!(
                        a,
                        Operand::StringLiteral(_)
                            | Operand::IntLiteral(_)
                            | Operand::FloatLiteral(_)
                    );
                    if var_side && lit_side {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CallShape {
    Predicate,
    Transform,
    Other,
}

/// Collect call names + coarse shapes from lowered IR text (cheap: reuses the
/// harness lowering). Shape detection: a call whose result feeds only a
/// Branch/condition is predicate-shaped; a call whose result is re-stored or
/// passed on is transform-shaped.
fn collect_calls(files: &[(String, String, String)]) -> FxHashMap<String, CallShape> {
    let mut calls: FxHashMap<String, CallShape> = FxHashMap::default();
    use frensense_engine::ir::function::{Instruction, Operand, Terminator, VarId};

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

    for (path, source, ext) in files {
        let Ok(irs) = frensense_engine::harness::lower_source(path, source, ext) else {
            continue;
        };
        for ir in irs.values() {
            // Vars that flow into a branch condition (directly or via a
            // short def chain, captures `!x.test(y)`).
            let mut cond_vars: FxHashMap<VarId, ()> = FxHashMap::default();
            for block in ir.blocks.values() {
                if let Terminator::Branch { cond, .. } = &block.terminator {
                    if let Operand::Var(c) = cond {
                        cond_vars.insert(*c, ());
                        // one hop up the def chain (unary ! / binary &&)
                        for b2 in ir.blocks.values() {
                            for i in &b2.instructions {
                                match i {
                                    Instruction::UnaryOp {
                                        dest,
                                        src: Operand::Var(s),
                                        ..
                                    }
                                    | Instruction::BinaryOp {
                                        dest,
                                        lhs: Operand::Var(s),
                                        ..
                                    }
                                    | Instruction::BinaryOp {
                                        dest,
                                        rhs: Operand::Var(s),
                                        ..
                                    } if *dest == *c => {
                                        cond_vars.insert(*s, ());
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            for block in ir.blocks.values() {
                for instr in &block.instructions {
                    let (name, dest) = match instr {
                        Instruction::CallStatic { func, dest, .. } => (Some(func.clone()), *dest),
                        Instruction::CallVirtual { method, dest, .. } => {
                            (Some(method.clone()), *dest)
                        }
                        _ => (None, None),
                    };
                    if let Some(name) = name {
                        let shape = if dest.map(|d| cond_vars.contains_key(&d)).unwrap_or(false) {
                            CallShape::Predicate
                        } else {
                            CallShape::Other
                        };
                        // Predicate classification wins over Other.
                        calls
                            .entry(name)
                            .and_modify(|e| {
                                if shape == CallShape::Predicate {
                                    *e = shape.clone();
                                }
                            })
                            .or_insert(shape);
                    }
                }
            }
        }
    }
    calls
}

/// Names that plausibly participate in taint boundaries. This is a *bundler*
/// heuristic (build-time, recall-oriented); the replay gate provides the
/// precision.
/// Decides whether a call name is worth proposing as a candidate fact.
///
/// The corpus, not a hardcoded list, is the source of truth: a call is
/// taint-relevant if the FAMILY ITSELF discriminates on it. We keep only a
/// minimal syntactic exclusion (property lookups / obviously pure names
/// that would explode the candidate space) and let the replay gate reject
/// wrong proposals, that is what the gate is for.
fn looks_taint_relevant(call: &str) -> bool {
    // Accessor/universal-method noise: these appear in nearly every
    // variant and are structurally not policy/sink calls.
    const NOISE: &[&str] = &[
        "get",
        "set",
        "has",
        "then",
        "catch",
        "finally",
        "toString",
        "valueOf",
        "push",
        "pop",
        "map",
        "filter",
        "reduce",
        "forEach",
        "join",
        "split",
        "len",
        "length",
        "keys",
        "values",
        "entries",
        "stringify",
        "parse",
    ];
    !NOISE.contains(&call) && !call.is_empty()
}

/// Apply a candidate to a fact table (for replay).
fn apply_candidate(table: &mut FactTable, c: &Candidate) {
    match c {
        Candidate::Sink { call, dangerous } => {
            let sig = if dangerous.is_empty() {
                SinkSignature::all_args(call)
            } else {
                SinkSignature::with_args(call, &dangerous.iter().copied().collect::<Vec<_>>())
            };
            table.sink_signatures.insert(call.clone(), sig);
        }
        Candidate::Sanitizer { call, guard } => {
            table.sanitizer_facts.insert(
                call.clone(),
                SanitizerFact {
                    call: call.clone(),
                    kind: if *guard {
                        "allowlist".into()
                    } else {
                        "encode".into()
                    },
                    sanitizes_args: Default::default(),
                    guard_style: *guard,
                },
            );
        }
        Candidate::Check {
            rule,
            call,
            message,
            unless_guard,
            unless_range_check,
        } => {
            table.learned_checks.push(LearnedCheckFact {
                rule: rule.clone(),
                call: call.clone(),
                message: message.clone(),
                severity: "warning".into(),
                unless_guard: unless_guard.clone(),
                unless_range_check: unless_range_check.clone(),
            });
        }
    }
}

/// Family separation check under a fact table (single-shot variant; the
/// gate uses the pre-lowered [`PreparedFamily::separates`] instead).
fn separates(family: &Family, config: &TaintConfig, facts: &FactTable) -> bool {
    let pos = scan_variant(&family.positives, config, facts);
    let neg = scan_variant(&family.negatives, config, facts);
    pos.has_alert() && !neg.has_alert()
}

fn fact_key(e: &LearnedFactEntry) -> (String, String) {
    match e {
        LearnedFactEntry::Sink { call, .. } => ("sink".into(), call.clone()),
        LearnedFactEntry::Sanitizer { call, .. } => ("san".into(), call.clone()),
        LearnedFactEntry::Check { rule, call, .. } => ("check".into(), format!("{rule}:{call}")),
    }
}

/// Full extraction: propose per family → replay-gate → merged learned table.
pub fn extract_facts(
    families: &[Family],
    config: &TaintConfig,
    builtin: &FactTable,
) -> (FactTable, Vec<LearnedFact>) {
    // votes: candidate-key → (support, families)
    let mut votes: FxHashMap<String, (Candidate, u32, Vec<String>)> = FxHashMap::default();

    // Pass 1: proposals per family (accumulate votes).
    for f in families {
        for c in propose(f, config, builtin) {
            let key = match &c {
                Candidate::Sink { call, .. } => format!("sink:{call}"),
                Candidate::Sanitizer { call, guard } => format!("san:{call}:{guard}"),
                Candidate::Check { rule, call, .. } => format!("check:{rule}:{call}"),
            };
            let entry = votes.entry(key).or_insert_with(|| (c, 0, Vec::new()));
            entry.1 += 1;
            if !entry.2.contains(&f.id) {
                entry.2.push(f.id.clone());
            }
        }
    }

    // Pass 2: replay gate. Every family is lowered once up front; each
    // candidate trial then re-runs only the taint engine + checker per
    // family (no re-lowering). Baseline separation is measured under the
    // built-in table; a candidate fact must (a) make (or keep) its voting
    // families separate and (b) NOT break any family that already separates
    // at baseline. Families that never separate under any table (corrupted
    // positives, see docs/E2E_REPORT.md §1b) are excluded from the
    // regression check: they can't be "broken" further, and gating on them
    // would reject every fact.
    let prepared: Vec<PreparedFamily> = families
        .iter()
        .filter_map(|f| match PreparedFamily::new(f) {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("[facts] skip family {}: {e}", f.id);
                None
            }
        })
        .collect();

    let baseline: FxHashMap<String, bool> = prepared
        .iter()
        .map(|p| (p.id.clone(), p.separates(config, builtin)))
        .collect();

    let mut learned = FactTable::default();
    let mut published: Vec<LearnedFact> = Vec::new();

    for (key, (cand, support, fams)) in votes {
        let mut trial = builtin.clone();
        apply_candidate(&mut trial, &cand);

        // The fact must make each voting family separate (or keep it
        // separate), and must not break any family that separated at
        // baseline.
        if std::env::var("FXDBG").is_ok() {
            eprintln!("[gate] candidate key={} support={}", key, support);
        }
        // Candidate call names (trigger + guard), a family whose variants
        // never call any of them cannot change separation under this fact,
        // so keep its baseline verdict without re-scanning.
        let cand_calls: Vec<&str> = match &cand {
            Candidate::Sink { call, .. } => vec![call.as_str()],
            Candidate::Sanitizer { call, .. } => vec![call.as_str()],
            Candidate::Check {
                call, unless_guard, ..
            } => match unless_guard {
                Some(g) => vec![call.as_str(), g.as_str()],
                None => vec![call.as_str()],
            },
        };
        let mut ok = true;
        for p in &prepared {
            let voted = fams.contains(&p.id);
            let relevant = voted || cand_calls.iter().any(|c| p.calls.contains(*c));
            let sep = if relevant {
                p.separates(config, &trial)
            } else {
                baseline.get(&p.id).copied().unwrap_or(false)
            };
            if voted && !sep {
                ok = false; // its own vote failed
                break;
            }
            if !voted && baseline.get(&p.id).copied().unwrap_or(false) && !sep {
                // Cross-family regression: the fact broke a previously
                // separating family. Reject the fact (shape-thinking creep).
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }

        apply_candidate(&mut learned, &cand);
        let entry = match &cand {
            Candidate::Sink { call, dangerous } => LearnedFactEntry::Sink {
                call: call.clone(),
                dangerous_args: dangerous.clone(),
                binding_args_safe: false,
            },
            Candidate::Sanitizer { call, guard } => LearnedFactEntry::Sanitizer {
                call: call.clone(),
                kind: if *guard {
                    "allowlist".into()
                } else {
                    "encode".into()
                },
                guard_style: *guard,
            },
            Candidate::Check {
                rule,
                call,
                message,
                unless_guard,
                unless_range_check,
            } => LearnedFactEntry::Check {
                rule: rule.clone(),
                call: call.clone(),
                message: message.clone(),
                severity: "warning".into(),
                unless_guard: unless_guard.clone(),
                unless_range_check: unless_range_check.clone(),
            },
        };
        published.push(LearnedFact {
            entry,
            support,
            status: if support >= 2 {
                "confirmed"
            } else {
                "provisional"
            }
            .into(),
            families: fams,
        });
        let _ = key;
    }

    published.sort_by(|a, b| {
        let ka = fact_key(&a.entry);
        let kb = fact_key(&b.entry);
        ka.cmp(&kb)
    });
    (learned, published)
}

#[cfg(test)]
mod slot_regression_tests {
    //! Corpus-level regression pins for per-slot sink awareness (see
    //! `docs/JUICESHOP_BASELINE.md` and the FP-reduction work): the replay
    //! gate must never publish a fact that (a) re-alerts the parameterized
    //! query binding channel, (b) suppresses the dangerous SQL string slot,
    //! or (c) re-promotes validator APIs (`jwt.verify`) to sinks.

    use super::*;
    use frensense_engine::analysis::taint::facts::{config_from_spec, fact_table_from_spec};

    /// Built-in config + fact table exactly as the bundler CLI builds them.
    fn builtin() -> (TaintConfig, FactTable) {
        let mut config = TaintConfig::default();
        let mut table = FactTable::default();
        for spec in frensense_lang::all_specs() {
            let c = config_from_spec(spec);
            config.sources.extend(c.sources);
            config.sinks.extend(c.sinks);
            config.sanitizers.extend(c.sanitizers);
            table.merge(&fact_table_from_spec(spec));
        }
        (config, table)
    }

    fn ts_file(name: &str, source: &str) -> (String, String, String) {
        (name.to_string(), source.to_string(), "ts".to_string())
    }

    /// Taint lands in slot 0 (the SQL string): must alert.
    const PARAM_POSITIVE: &str = r#"
import { Pool } from "pg";
const pool = new Pool();
export function getUser (req: any) {
  const id = req.body.id;
  return pool.query("SELECT * FROM users WHERE id = " + id)
}
"#;

    /// Taint lands in slot 1 (the params binding channel): must NOT alert.
    const PARAM_NEGATIVE: &str = r#"
import { Pool } from "pg";
const pool = new Pool();
export function getUser (req: any) {
  const id = req.body.id;
  return pool.query("SELECT * FROM users WHERE id = $1", [id])
}
"#;

    /// `jwt.verify(taintedToken, secret)`: validation by design, not a sink.
    const JWT_VERIFY: &str = r#"
import jwt from "jsonwebtoken";
const JWT_SECRET = "dev-secret";
export function auth (req: any) {
  const token: string = req.cookies.token;
  return jwt.verify(token, JWT_SECRET)
}
"#;

    /// Control: a real injection flow through slot 0 must keep alerting,
    /// guards against someone later over-suppressing the whole sink.
    const CONTROL_SINK: &str = r#"
import { Pool } from "pg";
const pool = new Pool();
export function badSearch (req: any) {
  const q = req.query.q;
  return pool.query("SELECT * FROM items WHERE name = '" + q + "'")
}
"#;

    #[test]
    fn builtin_table_is_slot_aware() {
        let (_, table) = builtin();

        let q = table
            .sink_signature("query")
            .expect("query must be a known sink");
        assert!(q.is_dangerous(0), "SQL string slot is dangerous");
        assert!(
            !q.is_dangerous(1),
            "params slot is the safe binding channel"
        );
        assert!(q.is_binding(1));

        let d = table
            .sink_signature("decrypt")
            .expect("decrypt must be a known sink");
        assert!(d.is_dangerous(0), "ciphertext slot is dangerous");
        assert!(!d.is_dangerous(1), "key slot must not alert");
    }

    #[test]
    fn jwt_verify_is_a_validator_not_a_sink() {
        let (config, table) = builtin();
        assert!(
            !config.sinks.contains("verify"),
            "verify must not be a configured sink"
        );
        assert!(table.sink_signature("verify").is_none());

        let res = scan::scan(&[ts_file("jwt.ts", JWT_VERIFY)], &config, &table);
        assert!(
            !res.has_alert(),
            "jwt.verify(token, secret) must not alert; got {:?}",
            res.vulnerable().collect::<Vec<_>>()
        );

        // Control: real sinks still alert through the same harness.
        let ctrl = scan::scan(&[ts_file("ctrl.ts", CONTROL_SINK)], &config, &table);
        assert!(ctrl.has_alert(), "control injection flow must still alert");
    }

    #[test]
    fn parameterized_query_family_separates() {
        let (config, table) = builtin();
        let pos = scan::scan(
            &[ts_file("pq_positive.ts", PARAM_POSITIVE)],
            &config,
            &table,
        );
        let neg = scan::scan(
            &[ts_file("pq_negative.ts", PARAM_NEGATIVE)],
            &config,
            &table,
        );
        assert!(pos.has_alert(), "taint in the SQL slot must alert");
        assert!(
            !neg.has_alert(),
            "taint in the params binding channel must not alert"
        );
    }

    #[test]
    fn replay_gate_cannot_override_builtin_slot_facts() {
        let (config, table) = builtin();

        let param = Family {
            id: "parameterized_query".into(),
            positives: vec![ts_file("parameterized_query_positive.ts", PARAM_POSITIVE)],
            negatives: vec![ts_file("parameterized_query_negative.ts", PARAM_NEGATIVE)],
            declared_check_call: None,
        };
        let jwt = Family {
            id: "jwt_validator".into(),
            positives: vec![ts_file("jwt_validator_positive.ts", JWT_VERIFY)],
            negatives: vec![ts_file("jwt_validator_negative.ts", CONTROL_SINK)],
            declared_check_call: None,
        };

        let (learned, published) = extract_facts(&[param, jwt], &config, &table);

        // The gate must never publish a `query` sink fact: any such fact
        // would override the built-in slot-restricted signature on merge.
        let query_fact = published.iter().any(
            |f| matches!(&f.entry, LearnedFactEntry::Sink { call, .. } if call.as_str() == "query"),
        );
        assert!(
            !query_fact,
            "gate must not publish a query sink fact over the built-in slot signature"
        );

        // Slot awareness must survive merging whatever the gate published.
        let mut merged = table.clone();
        merged.merge(&learned);
        let pos = scan::scan(
            &[ts_file("pq_positive.ts", PARAM_POSITIVE)],
            &config,
            &merged,
        );
        let neg = scan::scan(
            &[ts_file("pq_negative.ts", PARAM_NEGATIVE)],
            &config,
            &merged,
        );
        assert!(pos.has_alert(), "SQL-slot danger must survive fact merge");
        assert!(
            !neg.has_alert(),
            "binding-channel safety must survive fact merge"
        );
    }
}
