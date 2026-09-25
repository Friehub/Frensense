// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing
//! Frensense consumer library: the CLI, MCP, and reporting layer over the
//! `frensense-engine` compiler.
//!
//! This crate owns no analysis logic. It collects files, hands them to
//! [`frensense_engine::scan`], and renders flow-derived findings.

pub mod cli;
pub mod engine;
pub mod mcp;
pub mod parser;
pub mod reporter;

pub use engine::Engine;

use std::path::Path;
use thiserror::Error;

pub use frensense_engine::FileId;

pub const FRENSENSE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

impl Severity {
    #[must_use]
    pub fn meets_threshold(&self, threshold: Severity) -> bool {
        match (self, threshold) {
            (Severity::Critical, _)
            | (Severity::Info, Severity::Info)
            | (Severity::Warning, Severity::Warning | Severity::Info) => true,
            (Severity::Warning, Severity::Critical) | (Severity::Info, _) => false,
        }
    }
}

/// A single finding, rendered compiler-style: where, what, why, how to fix.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[must_use]
pub struct Advisory {
    pub title: String,
    pub file_id: FileId,
    pub file_path: String,
    pub severity: Severity,
    pub confidence: f64,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub line: u32,
    /// Last line of the finding's span (defaults to `line`). Multi-line
    /// findings, an allowlist definition, a large expression, report the
    /// full extent so consumers and scorers can match against any line the
    /// finding actually covers.
    #[serde(default)]
    pub end_line: u32,
    pub column: u32,
    pub start_byte: u32,
    pub end_byte: u32,
    pub original_content: String,
    pub enclosing_symbol: Option<String>,
    pub fingerprint: String,
    pub requires_human: bool,
    pub tags: Vec<String>,
    /// Per-step descriptions of the taint flow (source→sink) plus the byte
    /// offset of each step, when the engine captured a path. Empty for
    /// non-dataflow findings. Rendered into SARIF `codeFlows`.
    #[serde(default)]
    pub taint_steps: Vec<(String, Option<(String, usize)>)>,
}

impl Advisory {
    /// Create an advisory with common defaults pre-filled.
    pub fn bare(
        title: impl Into<String>,
        severity: Severity,
        file_id: FileId,
        file_path: &Path,
        observation: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            file_id,
            file_path: file_path.display().to_string(),
            severity,
            confidence: 0.5,
            observation: observation.into(),
            impact: String::new(),
            improvement: String::new(),
            line: 0,
            end_line: 0,
            column: 0,
            start_byte: 0,
            end_byte: 0,
            original_content: String::new(),
            enclosing_symbol: None,
            fingerprint: String::new(),
            requires_human: true,
            tags: Vec::new(),
            taint_steps: Vec::new(),
        }
    }

    pub fn with_confidence(mut self, v: f64) -> Self {
        self.confidence = v;
        self
    }
    pub fn with_line(mut self, v: u32) -> Self {
        self.line = v;
        self
    }
    /// Set the span's last line; `line` remains the first.
    pub fn with_end_line(mut self, v: u32) -> Self {
        self.end_line = v;
        self
    }
    pub fn with_column(mut self, v: u32) -> Self {
        self.column = v;
        self
    }
    pub fn with_content(mut self, v: impl Into<String>) -> Self {
        self.original_content = v.into();
        self
    }
    pub fn with_impact(mut self, v: impl Into<String>) -> Self {
        self.impact = v.into();
        self
    }
    pub fn with_improvement(mut self, v: impl Into<String>) -> Self {
        self.improvement = v.into();
        self
    }
    pub fn with_enclosing_symbol(mut self, v: impl Into<String>) -> Self {
        self.enclosing_symbol = Some(v.into());
        self
    }
    pub fn with_tags<const N: usize>(mut self, tags: [&str; N]) -> Self {
        self.tags = tags.iter().map(std::string::ToString::to_string).collect();
        self
    }

    /// Stable identity key for baseline comparisons.
    #[must_use]
    pub fn identity(&self) -> (String, String, u32, u32) {
        (
            self.fingerprint.clone(),
            self.file_path.clone(),
            self.line,
            self.column,
        )
    }
}

#[derive(Error, Debug)]
pub enum FrensenseError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Engine error: {0}")]
    Engine(String),
}

pub type Result<T> = std::result::Result<T, FrensenseError>;
