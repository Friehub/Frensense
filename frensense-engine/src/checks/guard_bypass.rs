// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Two seed mechanisms for guard/policy bugs taint analysis cannot see:
//!
//! 1. **Substring allowlist guards** (`redirect_challenge` class): a guard
//!    that validates attacker input with `x.includes(item)` / `indexOf(...) >= 0`
//!    over an allowlist is bypassable, `https://evil.com?https://good.com`
//!    contains the allowed substring. The *mechanism* is "validation by
//!    substring containment"; the corpus decides which callees/guards matter.
//!
//! 2. **Password KDF policy** (`weak_password` class): a value flowing into a
//!    credential setter/hasher guarded by nothing but a fast digest wrapper.
//!    Mechanism: "hash-named call in a password-setting function".

use crate::checks::CheckerFinding;
use crate::ir::function::{FunctionIR, Instruction, Operand};

fn last_segment(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

/// Containment-test callees: last-segment match. `includes` is the canonical
/// JS shape; `indexOf`/`search`/`match` count when the result feeds a
/// boolean guard (approximated here by presence in the same function as an
/// allowlist-style loop accumulation).
static CONTAINMENT_CALLEES: &[&str] = &["includes", "indexOf", "contains"];

/// Credential-setting call names (last segment): functions whose argument is
/// a plaintext password by convention.
static CREDENTIAL_SINKS: &[&str] = &["hash", "hashPassword", "hashpw", "setPassword", "set"];

/// Credential parameter names (source_name of the arg var) that mark a
/// value as a plaintext credential.
static CREDENTIAL_PARAM_NAMES: &[&str] = &[
    "password",
    "passwd",
    "pwd",
    "clearTextPassword",
    "clearPassword",
    "newPassword",
];

/// Rule 1: substring-containment guard. Fires on `includes`/`indexOf` calls
/// where the receiver OR the argument is a function parameter named like a
/// URL/redirect target (`url`, `toUrl`, `redirect`, ...), the shape of an
/// allowlist validation over attacker input.
pub fn check(ir: &FunctionIR) -> Vec<CheckerFinding> {
    let mut findings = Vec::new();
    let param_names: Vec<String> = ir
        .parameters
        .iter()
        .filter_map(|p| ir.var_metadata.get(p).and_then(|m| m.source_name.clone()))
        .collect();
    let has_url_param = param_names.iter().any(|n| {
        n.to_ascii_lowercase().contains("url") || n.to_ascii_lowercase().contains("redirect")
    });

    for block in ir.blocks.values() {
        for instr in &block.instructions {
            let (callee, recv, args, dest) = match instr {
                Instruction::CallVirtual {
                    method,
                    receiver,
                    args,
                    dest,
                    ..
                } => (method, Some(receiver), args, dest.as_ref()),
                Instruction::CallStatic {
                    func, args, dest, ..
                } => (func, None, args, dest.as_ref()),
                _ => continue,
            };
            let seg = last_segment(callee);
            if !CONTAINMENT_CALLEES.contains(&seg) {
                continue;
            }
            // URL-ish receiver (`url.includes(x)`) or URL-ish argument in a
            // function that also takes a URL parameter, the allowlist-guard
            // shape. Receiver name comes from var metadata when present.
            let recv_is_url = recv
                .and_then(|r| match r {
                    Operand::Var(v) => ir.var_metadata.get(v).and_then(|m| m.source_name.clone()),
                    _ => None,
                })
                .map(|n| {
                    let l = n.to_ascii_lowercase();
                    l.contains("url") || l.contains("redirect")
                })
                .unwrap_or(false);
            let arg_is_urlish = args.iter().any(|a| match a {
                Operand::Var(v) => ir
                    .var_metadata
                    .get(v)
                    .and_then(|m| m.source_name.clone())
                    .map(|n| {
                        let l = n.to_ascii_lowercase();
                        l.contains("url") || l.contains("redirect") || l.contains("allowed")
                    })
                    .unwrap_or(false),
                _ => false,
            });
            if has_url_param && (recv_is_url || arg_is_urlish) {
                let span = dest
                    .and_then(|d| ir.var_metadata.get(d))
                    .and_then(|m| m.byte_range);
                findings.push(CheckerFinding {
                    learned: false,
                    function: ir.name.clone(),
                    rule: "substring_allowlist_guard".to_string(),
                    message: format!(
                        "Allowlist validation uses substring containment `{seg}`, \
                         bypassable by embedding an allowed URL inside an attacker \
                         host (`https://evil.com?https://allowed`). Use exact-match \
                         or parse-and-compare-origin instead."
                    ),
                    span,
                });
            }
        }
    }
    findings
}

/// Rule 2: credential hashing policy. A call named like a hasher whose
/// argument is (or resolves to) a parameter named like a plaintext password
/// fires the weak-credential-storage rule. Mechanism only: whether the
/// wrapped implementation is acceptable is exactly what corpus facts
/// (wrapper→primitive mappings) refine.
pub fn check_credentials(ir: &FunctionIR) -> Vec<CheckerFinding> {
    let mut findings = Vec::new();
    for block in ir.blocks.values() {
        for instr in &block.instructions {
            let (callee, args, dest) = match instr {
                Instruction::CallVirtual {
                    method, args, dest, ..
                }
                | Instruction::CallStatic {
                    func: method,
                    args,
                    dest,
                    ..
                } => (method, args, dest.as_ref()),
                _ => continue,
            };
            let seg = last_segment(callee);
            if !CREDENTIAL_SINKS.contains(&seg) {
                continue;
            }
            let cred_arg = args.iter().any(|a| match a {
                Operand::Var(v) => ir
                    .var_metadata
                    .get(v)
                    .and_then(|m| m.source_name.clone())
                    .map(|n| {
                        let l = n.to_ascii_lowercase();
                        CREDENTIAL_PARAM_NAMES
                            .iter()
                            .any(|c| l == c.to_ascii_lowercase())
                    })
                    .unwrap_or(false),
                _ => false,
            });
            if cred_arg {
                let span = dest
                    .and_then(|d| ir.var_metadata.get(d))
                    .and_then(|m| m.byte_range);
                findings.push(CheckerFinding {
                    learned: false,
                    function: ir.name.clone(),
                    rule: "credential_kdf_policy".to_string(),
                    message: format!(
                        "Credential `{seg}` call receives a plaintext password, \
                         password storage must use a memory-hard KDF \
                         (bcrypt/argon2/scrypt), not a fast digest wrapper."
                    ),
                    span,
                });
            }
        }
    }
    findings
}

/// Program-level rule: when a substring-containment guard exists somewhere
/// in the program AND a static initializer builds an allowlist collection
/// (`new Set([...])` / `new Map(...)`) of URL-ish strings, the *definition*
/// itself is part of the vulnerability, attacker URLs embed allowed
/// entries. One finding per allowlist definition, spanned at the
/// definition.
pub fn check_allowlist_definitions(irs: &[&FunctionIR]) -> Vec<CheckerFinding> {
    let guard_exists = irs.iter().any(|ir| !check(ir).is_empty());
    if !guard_exists {
        return Vec::new();
    }
    let mut findings = Vec::new();
    for ir in irs {
        for block in ir.blocks.values() {
            for instr in &block.instructions {
                let (callee, args, dest) = match instr {
                    Instruction::CallStatic {
                        func, args, dest, ..
                    } => (func, args, dest),
                    _ => continue,
                };
                let seg = last_segment(callee);
                if !matches!(seg, "Set" | "Map") {
                    continue;
                }
                let mut urlish = false;
                let mut first_url_span = None;
                for a in args {
                    if let Operand::StringLiteral(s) = a {
                        let l = s.to_ascii_lowercase();
                        if l.contains("http") || l.contains("://") {
                            urlish = true;
                            // The entry literal's own span: the concrete
                            // allowlist item an attacker embeds. Prefer it
                            // over the whole-definition span.
                            if first_url_span.is_none() {
                                first_url_span = Some(a);
                            }
                        }
                    }
                }
                if !urlish {
                    continue;
                }
                // Span of the first URL entry's *node*: the StringLiteral
                // operand doesn't carry a span, so fall back through the
                // dest (whole definition). Entry-level precision needs the
                // literal's byte range in Operand metadata, track TODO.
                let _ = first_url_span;
                let span = dest
                    .as_ref()
                    .and_then(|d| ir.var_metadata.get(d))
                    .and_then(|m| m.byte_range);
                findings.push(CheckerFinding {
                    learned: false,
                    function: ir.name.clone(),
                    rule: "allowlist_definition_bypassable".to_string(),
                    message: format!(
                        "Allowlist `{}` is enforced by substring containment elsewhere in                          the program, any URL embedding one of these entries passes the                          guard (`https://evil.com?https://allowed`). Enforce                          origin-exact matching at the guard.",
                        ir.name
                    ),
                    span,
                });
            }
        }
    }
    findings
}
