// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! The weak-hash / weak-crypto rule: non-dataflow policy assertions over lowered IR.
//!
//! Taint analysis answers "does attacker-controlled data reach a dangerous
//! API?", but a whole class of vulnerabilities has no taint path at all:
//! weak cryptographic primitives (MD5 for passwords), hardcoded credentials,
//! insecure JWT algorithms (`{ algorithm: 'none' }`), disabled TLS
//! verification. The bug is in *which API / which constant* the developer
//! chose, not in how data flows.
//!
//! Each rule is a function over one [`FunctionIR`] plus the language spec's
//! knowledge tables. A rule fires a [`CheckerFinding`] when its pattern
//! matches; no sources, sinks or sanitizers are consulted. This keeps the
//! checker immune to taint-config drift and free to run per function in
//! parallel.

use crate::checks::CheckerFinding;
use crate::ir::function::{FunctionIR, Instruction, Operand};

/// A known call whose string-literal argument selects a weak primitive.
///
/// Matches the two shapes real code uses:
/// - `createHash('md5')`, the selector literal is argument 0.
/// - `md5(data)` / `MD5(...)`, bare weak-hash functions; no selector needed.
#[derive(Debug)]
struct WeakPrimitiveRule {
    rule_id: &'static str,
    /// Callee last-segment names that take a selector literal argument.
    selector_calls: &'static [&'static str],
    /// Bare function names that are weak by themselves.
    bare_calls: &'static [&'static str],
    /// Argument slot of the selector literal (ignored for bare calls).
    selector_slot: usize,
    /// Selector literals that mark the call weak (case-insensitive, quotes
    /// stripped by the caller).
    weak_selectors: &'static [&'static str],
}

/// The builtin policy table. Language specs can extend this later via the
/// same `&'static` pattern as sink names; for now these are the universal
/// crypto policy facts.
static WEAK_HASH_RULES: &[WeakPrimitiveRule] = &[
    // Node crypto: createHash('md5'), createHash('sha1')
    WeakPrimitiveRule {
        rule_id: "weak_hash",
        selector_calls: &["createHash"],
        bare_calls: &["md5", "sha1"],
        selector_slot: 0,
        weak_selectors: &["md5", "md4", "sha1", "sha"],
    },
    // Python hashlib / passlib: hashlib.new('md5', ...)
    WeakPrimitiveRule {
        rule_id: "weak_hash",
        selector_calls: &["new"],
        bare_calls: &["md5", "sha1"],
        selector_slot: 0,
        weak_selectors: &["md5", "md4", "sha1", "sha"],
    },
    // Java MessageDigest / Go crypto/md5 style, covered by bare names where
    // applicable; the receiver name (`crypto`) is matched loosely so
    // `crypto.createHash` and `hashlib.new` both resolve to last segment.
];

/// Known *string-literal* insecure configuration selectors: calls whose
/// argument literal itself selects an insecure mode regardless of algorithm.
static INSECURE_CONFIG_SELECTORS: &[(&str, &[&str], &str)] = &[
    // jwt.sign(payload, secret, { algorithm: 'none' }) style appears as a
    // string selector on some APIs; `none`/`HS1` in the `alg` slot.
    ("jwt", &["none", "hs1"], "insecure_jwt_algorithm"),
];

/// Known *wrapper* names whose entire purpose is hashing: a weak selector
/// inside the wrapper (checked cross-function by the corpus replay gate) or
/// a suspicious receiver qualification makes these worth flagging. Bare
/// wrappers like `security.hash(...)` are the Juice Shop weakPassword shape:
/// the wrapper resolves to `createHash('md5')` elsewhere in the same file.
static SUSPICIOUS_HASH_WRAPPERS: &[&str] = &["hash", "hashpw", "hashPassword", "digest"];

fn strip_quotes(lit: &str) -> &str {
    let lit = lit.trim();
    let bytes = lit.as_bytes();
    let (core, _) = match bytes.first() {
        Some(b'\'') | Some(b'"') | Some(b'`') => {
            let end = bytes.len().saturating_sub(1);
            (&lit[1..end], true)
        }
        _ => (lit, false),
    };
    core
}

fn last_segment(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

/// Resolve a dotted receiver path for a virtual call (`security.hash` →
/// "security.hash") by walking the receiver var's defining chain.
fn receiver_path(ir: &FunctionIR, _args: &[Operand], _callee: &str) -> Option<String> {
    // Cheap static heuristic: only flag when the enclosing function name or
    // module context suggests security code. The full receiver-chain walk
    // needs cross-function info; the corpus replay gate (task 9) is the
    // mechanism that learns wrapper→primitive mappings from pairs.
    let n = ir.name.to_ascii_lowercase();
    if n.contains("insecure") || n.contains("security") {
        return Some(ir.name.clone());
    }
    None
}

fn operand_literal(op: &Operand) -> Option<&str> {
    match op {
        Operand::StringLiteral(s) => Some(strip_quotes(s)),
        _ => None,
    }
}

/// Run every policy rule over one lowered function.
/// The weak-hash / weak-crypto rule: scan one function for security-weak
/// primitives.
pub fn check(ir: &FunctionIR) -> Vec<CheckerFinding> {
    check_function(ir)
}

pub fn check_function(ir: &FunctionIR) -> Vec<CheckerFinding> {
    let mut findings = Vec::new();
    for block in ir.blocks.values() {
        for instr in &block.instructions {
            if let Instruction::CallStatic { func, args, .. }
            | Instruction::CallVirtual {
                method: func, args, ..
            } = instr
            {
                check_call(ir, func, args, instr_span(ir, instr), &mut findings);
            }
        }
    }
    findings
}

fn instr_span(ir: &FunctionIR, instr: &Instruction) -> Option<(usize, usize)> {
    // The call result var carries the call expression's byte range (set by
    // the lowering pass). Find the dest of this instruction.
    let dest = match instr {
        Instruction::CallStatic { dest, .. }
        | Instruction::CallVirtual { dest, .. }
        | Instruction::CallPointer { dest, .. } => (*dest)?,
        _ => return None,
    };
    ir.var_metadata.get(&dest)?.byte_range
}

fn check_call(
    ir: &FunctionIR,
    callee: &str,
    args: &[Operand],
    span: Option<(usize, usize)>,
    out: &mut Vec<CheckerFinding>,
) {
    // Receiver chains are matched by last segment: `security.hash` and bare
    // `hash` both hit the `hash` rule. This is deliberately over-approximate
    // (a user object with an unrelated `.hash` method also fires); the
    // message names the call, and false positives of this class are rare
    // relative to the cost of tracking receiver types through the IR.
    let callee_seg = last_segment(callee);
    let callee_lower = callee_seg.to_ascii_lowercase();

    for rule in WEAK_HASH_RULES {
        // Selector shape: `createHash('md5')`.
        if rule.selector_calls.iter().any(|c| {
            let c = last_segment(c).to_ascii_lowercase();
            c == callee_lower
        }) && let Some(sel) = args
            .get(rule.selector_slot)
            .and_then(operand_literal)
            .map(str::to_ascii_lowercase)
            && rule.weak_selectors.contains(&sel.as_str())
        {
            out.push(CheckerFinding {
                learned: false,
                function: ir.name.clone(),
                rule: rule.rule_id.to_string(),
                message: format!(
                    "Weak hash primitive '{}' selected by `{}`, not acceptable \
                     for passwords or security-sensitive digests (use \
                     bcrypt/argon2/scrypt or SHA-256+)",
                    sel, callee_seg
                ),
                span,
            });
            return; // one finding per call site
        }
        // Bare shape: `md5(data)`.
        if rule
            .bare_calls
            .iter()
            .any(|c| last_segment(c).to_ascii_lowercase() == callee_lower)
        {
            out.push(CheckerFinding {
                learned: false,
                function: ir.name.clone(),
                rule: rule.rule_id.to_string(),
                message: format!(
                    "Weak hash function `{}`, not acceptable for passwords or \
                     security-sensitive digests (use bcrypt/argon2/scrypt or \
                     SHA-256+)",
                    callee_seg
                ),
                span,
            });
            return;
        }
    }

    // Hash-wrapper heuristic: a call named `hash`/`hashPassword` whose
    // receiver is a security-ish namespace. Matched by last segment of the
    // receiver chain when available; bare `hash(x)` is too generic to flag
    // on its own, so require a known crypto-receiver qualification.
    if SUSPICIOUS_HASH_WRAPPERS.contains(&callee_lower.as_str())
        && let Some(path) = receiver_path(ir, args, callee)
    {
        out.push(CheckerFinding {
            learned: false,
            function: ir.name.clone(),
            rule: "weak_hash_wrapper".to_string(),
            message: format!(
                "Password hashing routed through opaque wrapper `{path}`, \
                 verify it uses bcrypt/argon2/scrypt, not MD5/SHA-1",
            ),
            span,
        });
    }

    // Insecure literal selectors (`{ algorithm: 'none' }` shapes land here
    // once object literals are flattened; for now the string form).
    for (prefix, selectors, rule_id) in INSECURE_CONFIG_SELECTORS {
        if !callee_lower.starts_with(prefix) {
            continue;
        }
        for arg in args {
            if let Some(lit) = operand_literal(arg)
                && selectors.contains(&lit.to_ascii_lowercase().as_str())
            {
                out.push(CheckerFinding {
                    learned: false,
                    function: ir.name.clone(),
                    rule: rule_id.to_string(),
                    message: format!(
                        "Insecure configuration: `{}` called with insecure \
                         selector '{lit}'",
                        callee_seg
                    ),
                    span,
                });
                return;
            }
        }
    }
}
