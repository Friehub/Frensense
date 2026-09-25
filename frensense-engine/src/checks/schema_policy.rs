// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Schema-policy seed mechanisms (R1): policy bugs that live in *tool
//! schemas*, the LLM-era analogue of config bugs.
//!
//! 1. **Unbounded number schema** (`unbounded_number_schema`): a schema
//!    builder describes a numeric bound in prose ("maximum 10") but the
//!    builder chain never applies `.max()`/`.min()`, the declared policy is
//!    decorative, enforced nowhere. The LLM's own decision is the "source",
//!    so no taint path can exist; this is a policy assertion over the IR.
//!
//! Mechanism only: which schema builders (`z.number`, `number`, ...) and
//! which bounds matter is corpus-refinable via learned checks.

use crate::checks::CheckerFinding;
use crate::ir::function::{FunctionIR, Instruction, Operand};

/// Schema number-builder methods (last segment) that produce an unbounded
/// numeric field when their builder chain lacks a `.max()`/`.min()` call.
static NUMBER_BUILDERS: &[&str] = &["number", "int", "float", "bigint"];

/// Bound keywords whose appearance in a description implies a declared
/// (but unenforced) numeric policy.
static BOUND_KEYWORDS: &[&str] = &["maximum", "max", "minimum", "min", "limit", "up to"];

/// Methods that would actually enforce a bound on the builder chain.
static ENFORCERS: &[&str] = &[
    "max",
    "min",
    "minimum",
    "maximum",
    "int",
    "multipleOf",
    "step",
];

pub fn check(ir: &FunctionIR) -> Vec<CheckerFinding> {
    let mut findings = Vec::new();
    // Collect describe/description string args keyed by their receiver chain,
    // and builder+enforcer method calls with their spans.
    let mut described: Vec<(String, Option<(usize, usize)>)> = Vec::new();
    let mut builders: Vec<(String, Option<(usize, usize)>)> = Vec::new();
    let mut enforcers: Vec<&str> = Vec::new();
    for block in ir.blocks.values() {
        for instr in &block.instructions {
            match instr {
                Instruction::CallVirtual {
                    method,
                    receiver,
                    args,
                    dest,
                    ..
                } => {
                    let seg = method.rsplit('.').next().unwrap_or(method);
                    if seg == "describe" || seg == "description" {
                        let text = args.first().and_then(|a| match a {
                            Operand::StringLiteral(s) => Some(s.clone()),
                            _ => None,
                        });
                        if let Some(text) = text {
                            let span = dest
                                .as_ref()
                                .and_then(|d| ir.var_metadata.get(d))
                                .and_then(|m| m.byte_range)
                                .or_else(|| {
                                    if let Operand::Var(r) = receiver {
                                        ir.var_metadata.get(r).and_then(|m| m.byte_range)
                                    } else {
                                        None
                                    }
                                });
                            described.push((text, span));
                        }
                    } else if ENFORCERS.contains(&seg) {
                        enforcers.push(seg);
                    } else if NUMBER_BUILDERS.contains(&seg) {
                        let span = dest
                            .as_ref()
                            .and_then(|d| ir.var_metadata.get(d))
                            .and_then(|m| m.byte_range);
                        builders.push((seg.to_string(), span));
                    }
                }
                _ => {}
            }
        }
    }
    if enforcers.is_empty() && !described.is_empty() && !builders.is_empty() {
        for (_builder, span) in &builders {
            for (text, _dspan) in &described {
                let lower = text.to_ascii_lowercase();
                if BOUND_KEYWORDS.iter().any(|k| lower.contains(k))
                    && lower.chars().any(|c| c.is_ascii_digit())
                {
                    findings.push(CheckerFinding {
                        learned: false,
                        function: ir.name.clone(),
                        rule: "unbounded_number_schema".to_string(),
                        message: format!(
                            "Schema declares a numeric policy in prose (`{text}`) but the \
                             builder chain applies no `.max()`/`.min()` enforcement, the \
                             declared bound is decorative. Add the constraint to the schema \
                             and validate in the handler."
                        ),
                        span: *span,
                    });
                    break;
                }
            }
            if !findings.is_empty() {
                break;
            }
        }
    }
    findings
}
