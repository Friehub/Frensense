// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Frensense engine: the dataflow compiler.
//!
//! The pipeline: parse (via `frensense-lang` specs) → lower → SSA → SVFG →
//! callgraph → demand-driven taint. Knowledge about sources, sinks, and
//! sanitizers lives in `frensense-lang` specs and `.frc`-bundle fact
//! tables, merged into a [`analysis::taint::config::TaintConfig`] +
//! [`analysis::taint::facts::FactTable`] at scan time.
//!
//! Crate layout (strict layering, each layer only depends on the ones
//! above it):
//! * [`ir`], the compiler frontend: `FunctionIR`, lowering, Memory SSA.
//!   Pure data; knows nothing about taint or checks.
//! * [`graph`], program-graph construction: SVFG, callgraph, heap
//!   points-to, serialization.
//! * [`analysis`], the engines: demand-driven backward taint (with its
//!   fact table and context sensitivity), the forward engine, summaries.
//! * [`checks`], non-dataflow policy rules (weak crypto, insecure JWT
//!   algorithms), one module per rule.
//! * [`harness`], the ONE implementation of "source text → analyzed
//!   program graph", shared by the scanner, e2e drivers, and the bundler.
//! * [`scan`], the stable query API: files + config + facts → findings.

pub mod analysis;
pub mod checks;
pub mod graph;
pub mod harness;
pub mod ir;
pub mod scan;

/// Opaque identifier for a source file within a single analysis session.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u64);

#[derive(Debug, thiserror::Error)]
pub enum FrensenseError {
    #[error("Parse failure: {0}")]
    ParseFailure(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parser error: {0}")]
    ParserError(String),
    #[error("Pattern error: {0}")]
    Pattern(String),
    #[error("Engine error: {0}")]
    Engine(String),
}

impl From<std::io::Error> for FrensenseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<tree_sitter::LanguageError> for FrensenseError {
    fn from(e: tree_sitter::LanguageError) -> Self {
        Self::ParserError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, FrensenseError>;
