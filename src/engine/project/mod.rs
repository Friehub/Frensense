// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

mod runner;

use crate::{FileId, Severity};
use std::path::PathBuf;

/// The scan engine.
///
/// Configuration is minimal by design: the analysis knowledge lives in the
/// language specs (`frensense-lang`) and the `.frc` bundle's learned facts,
/// not in knobs on this struct.
pub struct Engine {
    /// Optional `.frc` bundle bytes (static) whose learned facts are merged
    /// over the built-in fact table.
    pub(crate) corpus_bundle: Option<&'static [u8]>,
    /// Optional `.frc` bundle loaded from disk at scan time.
    pub(crate) corpus_bundle_path: Option<PathBuf>,
    /// Optional seed-facts file (corpus-owned, JSON): deployment-specific
    /// knowledge (session-store roots, project conventions) that is not
    /// language semantics. Merged over spec tables, under bundle facts.
    pub(crate) seed_facts_path: Option<PathBuf>,
    pub(crate) severity_filter: Option<Severity>,
    pub(crate) language_filter: Option<Vec<&'static str>>,
    pub(crate) min_confidence: f64,
}

impl Engine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            corpus_bundle: None,
            corpus_bundle_path: None,
            seed_facts_path: None,
            severity_filter: None,
            language_filter: None,
            min_confidence: 0.0,
        }
    }

    /// Provide a seed-facts file (corpus-owned JSON) to merge over the
    /// spec-built fact table.
    pub fn set_seed_facts_path(&mut self, path: PathBuf) {
        self.seed_facts_path = Some(path);
    }

    /// Provide a `.frc` bundle (static bytes) whose learned facts teach the
    /// engine additional sinks/sanitizers/guards.
    pub fn set_corpus_bundle(&mut self, bundle: &'static [u8]) {
        self.corpus_bundle = Some(bundle);
    }

    /// Provide a `.frc` bundle loaded from disk (mut + scan workflow).
    pub fn set_corpus_bundle_path(&mut self, path: PathBuf) {
        self.corpus_bundle_path = Some(path);
    }

    pub fn set_severity_filter(&mut self, severity: Option<Severity>) {
        self.severity_filter = severity;
    }

    pub fn set_language_filter(&mut self, extensions: Vec<&'static str>) {
        self.language_filter = Some(extensions);
    }

    pub fn set_min_confidence(&mut self, min: f64) {
        self.min_confidence = min;
    }

    /// Register a synthetic file id for a scanned path (used by the runner
    /// when converting findings to advisories).
    #[allow(dead_code)]
    pub(crate) fn next_file_id(seq: usize) -> FileId {
        FileId(u32::try_from(seq).unwrap_or(u32::MAX))
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
