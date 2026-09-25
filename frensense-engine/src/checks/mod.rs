// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Non-dataflow checks: policy assertions over the lowered IR that need no
//! taint analysis.
//!
//! Two tiers, deliberately separated:
//!
//! **Built-in seed checks** (`weak_hash` and future structural rules) supply
//! only *mechanism*, how to look at the IR. They are the bootstrapping
//! oracle, kept minimal so shape-thinking cannot creep back in.
//!
//! **Corpus-learned checks** ([`LearnedCheckFact`] in the fact table) supply
//! the *conclusions*, which calls are violations, with bundle-authored
//! advisory text. They arrive through `.frc` bundles, were verified by the
//! bundler's replay gate (fire on positives, silent on negatives), and are
//! the scale path: nobody writes 45k rules.
//!
//! Line drawn: **built-in = how to look; learned = what to conclude.**

pub mod guard_bypass;
pub mod schema_policy;
pub mod weak_hash;

#[cfg(test)]
mod guard_bypass_tests;
#[cfg(test)]
mod schema_policy_tests;
#[cfg(test)]
mod weak_hash_tests;

use crate::analysis::taint::facts::FactTable;
use crate::ir::function::FunctionIR;
use rustc_hash::FxHashSet;

/// One non-dataflow policy finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckerFinding {
    /// Function containing the violating instruction.
    pub function: String,
    /// Stable rule id (e.g. `"weak_hash_md5"`, or a bundle's rule id).
    /// Built-in rules use `&'static str` ids; learned rules carry their
    /// bundle-supplied id, interned here.
    pub rule: String,
    /// Short human description of the violation.
    pub message: String,
    /// Byte range of the violating call in the source file, when known.
    pub span: Option<(usize, usize)>,
    /// `true` when this finding comes from a corpus-learned rule in the
    /// fact table rather than a built-in seed check.
    pub learned: bool,
}

/// Run every registered check over every function, deduped and ordered by
/// (function, span) so output is deterministic.
///
/// `facts` carries the bundle-learned checks; pass `&FactTable::default()`
/// when scanning without a bundle.
pub fn check_all<'a>(
    irs: impl IntoIterator<Item = &'a FunctionIR>,
    facts: &FactTable,
) -> Vec<CheckerFinding> {
    let mut seen: FxHashSet<(String, String, usize)> = FxHashSet::default();
    let mut all = Vec::new();
    let irs: Vec<&FunctionIR> = irs.into_iter().collect();
    // Program-level rules run once over the whole IR set (they correlate
    // guards in one function with definitions in another).
    for f in guard_bypass::check_allowlist_definitions(&irs) {
        let key = (
            f.function.clone(),
            f.rule.clone(),
            f.span.map(|s| s.0).unwrap_or(usize::MAX),
        );
        if seen.insert(key) {
            all.push(f);
        }
    }
    for ir in &irs {
        let findings = weak_hash::check(ir)
            .into_iter()
            .chain(guard_bypass::check(ir))
            .chain(guard_bypass::check_credentials(ir))
            .chain(schema_policy::check(ir))
            .chain(learned::check(ir, facts));
        for f in findings {
            let key = (
                f.function.clone(),
                f.rule.clone(),
                f.span.map(|s| s.0).unwrap_or(usize::MAX),
            );
            if seen.insert(key) {
                all.push(f);
            }
        }
    }
    all.sort_by_key(|f| {
        (
            f.function.clone(),
            f.span.map(|s| s.0).unwrap_or(usize::MAX),
        )
    });
    all
}

/// The learned-check rule: apply corpus-verified checks from the fact
/// table. Mechanism identical to the seed checks (call matching over the
/// IR); the conclusions are the bundle's.
mod learned {
    use super::CheckerFinding;
    use crate::analysis::taint::facts::FactTable;
    use crate::ir::function::{FunctionIR, Instruction, Operand};

    /// Last-segment names of every call in the function (one pass).
    fn call_segments(ir: &FunctionIR) -> Vec<String> {
        let mut segs = Vec::new();
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                match instr {
                    Instruction::CallStatic { func, .. } => {
                        segs.push(func.rsplit('.').next().unwrap_or(func).to_string());
                    }
                    Instruction::CallVirtual { method, .. } => {
                        segs.push(method.rsplit('.').next().unwrap_or(method).to_string());
                    }
                    _ => {}
                }
            }
        }
        segs
    }

    /// True when `var` (or its def chain) is compared against a literal
    /// with one of `ops` in this function, inline range enforcement
    /// (`if (d < 0 || d > MAX) …`). Walks one BinaryOp hop up from `var`
    /// and one down (comparisons may be written in either order).
    fn has_range_check(ir: &FunctionIR, var: crate::ir::function::VarId, ops: &[String]) -> bool {
        // Collect BinaryOps involving `var` on either side.
        let involved = |lhs: &Operand, rhs: &Operand| {
            matches!(lhs, Operand::Var(v) if *v == var)
                || matches!(rhs, Operand::Var(v) if *v == var)
        };
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                if let Instruction::BinaryOp { op, lhs, rhs, .. } = instr
                    && ops.contains(op)
                    && involved(lhs, rhs)
                {
                    // The OTHER side must be a literal (the bound).
                    let other = match (lhs, rhs) {
                        (Operand::Var(v), r) if *v == var => r,
                        (l, Operand::Var(v)) if *v == var => l,
                        _ => continue,
                    };
                    if matches!(
                        other,
                        Operand::StringLiteral(_)
                            | Operand::IntLiteral(_)
                            | Operand::FloatLiteral(_)
                    ) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Apply every learned check to one function.
    pub fn check(ir: &FunctionIR, facts: &FactTable) -> Vec<CheckerFinding> {
        if facts.learned_checks.is_empty() {
            return Vec::new();
        }
        let segs = call_segments(ir);
        let mut findings = Vec::new();
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                let (callee, args) = match instr {
                    Instruction::CallStatic { func, args, .. } => (func, args),
                    Instruction::CallVirtual {
                        method: func, args, ..
                    } => (func, args),
                    _ => continue,
                };
                for fact in facts.learned_checks_for(callee) {
                    // Guard qualification: a rule with `unless_guard` is
                    // silent when the guard call is present in the same
                    // function, the corpus says "trigger without
                    // enforcement" is the violation.
                    if let Some(guard) = &fact.unless_guard {
                        let gseg = guard.rsplit('.').next().unwrap_or(guard);
                        if segs.iter().any(|s| s == gseg) {
                            continue;
                        }
                    }
                    // Range qualification: silent when any trigger argument
                    // var is compared against a literal bound, inline
                    // enforcement without a named helper.
                    if let Some(ops) = &fact.unless_range_check {
                        let guarded = args.iter().any(|a| match a {
                            Operand::Var(v) => has_range_check(ir, *v, ops),
                            _ => false,
                        });
                        if guarded {
                            continue;
                        }
                    }
                    findings.push(CheckerFinding {
                        function: ir.name.clone(),
                        rule: fact.rule.clone(),
                        message: fact.message.clone(),
                        span: instr_span(ir, instr),
                        learned: true,
                    });
                }
            }
        }
        findings
    }

    fn instr_span(ir: &FunctionIR, instr: &Instruction) -> Option<(usize, usize)> {
        let dest = match instr {
            Instruction::CallStatic { dest, .. }
            | Instruction::CallVirtual { dest, .. }
            | Instruction::CallPointer { dest, .. } => (*dest)?,
            _ => return None,
        };
        ir.var_metadata.get(&dest)?.byte_range
    }

    // Silence unused-import warning if Operand becomes unused in future
    // refinements; kept in the destructure for symmetry with seed checks.
    #[allow(dead_code)]
    fn _assert_operand_used(_: &[Operand]) {}
}
