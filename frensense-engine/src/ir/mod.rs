// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! The IR layer: pure data and the compiler frontend.
//!
//! * [`function`], `FunctionIR`, `Instruction`, `Operand`, `VarMetadata`:
//!   the typed representation every analysis consumes.
//! * [`lowering`], tree-sitter AST → `FunctionIR` (via `frensense-lang`).
//! * [`ssa`], Memory SSA construction.
//!
//! This layer knows nothing about taint, checks, or program graphs, it is
//! the reusable compiler frontend.

pub mod function;
pub mod lowering;
pub mod ssa;

#[cfg(test)]
mod tests;
