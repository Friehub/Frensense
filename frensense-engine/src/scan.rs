// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! The stable scan API (`8.10`): source text + facts → findings.
//!
//! Everything downstream (CLI, bundler replay gate, benchmarks) calls this;
//! nothing touches engine internals. The fact table is the *teaching*
//! interface: built-ins from `frensense-lang`, merged with `.frc` bundle
//! facts.

use rustc_hash::FxHashMap;

use crate::analysis::forward::ProgramSvfg;
use crate::analysis::taint::config::TaintConfig;
use crate::analysis::taint::engine::{BackwardTaintEngine, BackwardVerdict, SinkFinding};
use crate::analysis::taint::facts::FactTable;
use crate::checks::{self, CheckerFinding};
use crate::harness::lower_source;
use crate::ir::function::FunctionIR;

/// Scan a set of source files under a config and fact table.
///
/// `sources` maps file path → extension (used to pick the language spec).
pub fn scan(
    files: &[(String, String, String)], // (path, source, ext)
    config: &TaintConfig,
    facts: &FactTable,
) -> ScanResult {
    match prepare(files) {
        Ok(p) => scan_prepared(&p, config, facts),
        Err(e) => ScanResult {
            findings: Vec::new(),
            located: Vec::new(),
            located_checker: Vec::new(),
            checker: Vec::new(),
            errors: vec![e],
        },
    }
}

/// A program lowered once, ready for repeated scans under different fact
/// tables. The replay gate lowers each family exactly once and re-scans it
/// per candidate fact, lowering dominates scan cost, so this turns the
/// gate's O(candidates × families) lowerings into O(families).
pub struct PreparedProgram {
    irs: FxHashMap<String, FunctionIR>,
    fn_file: FxHashMap<String, String>,
    file_source: FxHashMap<String, String>,
}

/// Lower every file once, remembering which file each function came from
/// (functions are keyed by name; first file wins on collision).
///
/// # Errors
/// Returns the first lowering error, if any.
pub fn prepare(files: &[(String, String, String)]) -> Result<PreparedProgram, String> {
    let mut irs: FxHashMap<String, FunctionIR> = FxHashMap::default();
    let mut fn_file: FxHashMap<String, String> = FxHashMap::default();
    let mut file_source: FxHashMap<String, String> = FxHashMap::default();
    for (path, source, ext) in files {
        let fns = lower_source(path, source, ext)?;
        for (name, ir) in fns {
            fn_file.entry(name.clone()).or_insert_with(|| path.clone());
            file_source
                .entry(path.clone())
                .or_insert_with(|| source.clone());
            irs.entry(name).or_insert(ir);
        }
    }
    Ok(PreparedProgram {
        irs,
        fn_file,
        file_source,
    })
}

/// Scan a [`PreparedProgram`] under a config and fact table. Same semantics
/// as [`scan`]; the lowering is reused across calls.
pub fn scan_prepared(
    prepared: &PreparedProgram,
    config: &TaintConfig,
    facts: &FactTable,
) -> ScanResult {
    let mut statics: FxHashMap<String, &FunctionIR> = FxHashMap::default();
    // Leak per function: the program graph borrows IRs for its lifetime.
    // Scan results own no reference to them, and scans are long-lived.
    for (name, ir) in &prepared.irs {
        let leaked: &'static FunctionIR = Box::leak(Box::new(ir.clone()));
        statics.insert(name.clone(), leaked);
    }

    let fn_file = &prepared.fn_file;
    let file_source = &prepared.file_source;

    // Non-dataflow policy checks run on the same lowered IR, no taint
    // needed, so weak-crypto/config bugs surface even with zero taint paths.
    // `facts` also carries corpus-learned checks installed by the bundle.
    let checker_findings = checks::check_all(statics.values().copied(), facts);

    let prog = ProgramSvfg::new(&statics, config);
    let mut engine = BackwardTaintEngine::new(&prog, config)
        .with_fact_table(facts)
        .with_fn_file(fn_file);
    engine.run();

    // Resolve each finding to a file + line/column from the sink's byte span.
    let located: Vec<LocatedFinding> = engine
        .findings
        .iter()
        .filter_map(|f| {
            let file = fn_file.get(&f.function)?;
            let source = file_source.get(file)?;
            let (line, col) = f
                .sink_span
                .map(|(start, _)| {
                    let before = &source[..start.min(source.len())];
                    let line = before.matches('\n').count() as u32 + 1;
                    let col = before
                        .rfind('\n')
                        .map_or(before.len(), |i| before.len() - i - 1)
                        as u32
                        + 1;
                    (line, col)
                })
                .unwrap_or((0, 0));
            Some(LocatedFinding {
                finding: f.clone(),
                file: file.clone(),
                line,
                column: col,
            })
        })
        .collect();

    // Locate checker findings the same way: function → file, span → line/col.
    let located_checker: Vec<LocatedChecker> = checker_findings
        .iter()
        .filter_map(|c| {
            let file = fn_file.get(&c.function)?;
            let source = file_source.get(file)?;
            let (line, col, end_line) = c
                .span
                .map(|(start, end)| {
                    let before = &source[..start.min(source.len())];
                    let line = before.matches('\n').count() as u32 + 1;
                    let col = before
                        .rfind('\n')
                        .map_or(before.len(), |i| before.len() - i - 1)
                        as u32
                        + 1;
                    let end_before = &source[..end.min(source.len())];
                    let end_line = end_before.matches('\n').count() as u32 + 1;
                    (line, col, end_line)
                })
                .unwrap_or((0, 0, 0));
            Some(LocatedChecker {
                finding: c.clone(),
                file: file.clone(),
                line,
                column: col,
                end_line: end_line.max(line),
            })
        })
        .collect();

    ScanResult {
        findings: engine.findings,
        located,
        located_checker,
        checker: checker_findings,
        errors: Vec::new(),
    }
}

/// The result of one scan: vulnerable findings + non-fatal errors.
#[derive(Debug, Default)]
pub struct ScanResult {
    /// All sink findings (every verdict); filter on [`ScanResult::alerts`]
    /// semantics.
    pub findings: Vec<SinkFinding>,
    /// The same findings resolved to source locations (file/line/column).
    pub located: Vec<LocatedFinding>,
    /// Non-dataflow policy violations (weak crypto, insecure config),
    /// resolved to file/line/column.
    pub located_checker: Vec<LocatedChecker>,
    /// Non-dataflow policy violations (weak crypto, insecure config).
    /// These have no taint path by definition; they are policy assertions
    /// over the API/constant choices in the lowered IR.
    pub checker: Vec<CheckerFinding>,
    pub errors: Vec<String>,
}

/// A [`CheckerFinding`] pinned to its source location.
#[derive(Debug, Clone)]
pub struct LocatedChecker {
    pub finding: CheckerFinding,
    /// Path of the file containing the violating call.
    pub file: String,
    /// 1-based line of the violating call (0 when no span recorded).
    pub line: u32,
    /// 1-based column (0 when no span recorded).
    pub column: u32,
    /// 1-based last line of the finding's span (>= `line`). Multi-line
    /// spans (allowlist definitions) report their full extent.
    pub end_line: u32,
}

/// A [`SinkFinding`] pinned to its source location.
#[derive(Debug, Clone)]
pub struct LocatedFinding {
    pub finding: SinkFinding,
    /// Path of the file containing the sink call.
    pub file: String,
    /// 1-based line of the sink call (0 when the lowering recorded no span).
    pub line: u32,
    /// 1-based column of the sink call (0 when no span recorded).
    pub column: u32,
}

impl ScanResult {
    /// Findings with verdict [`BackwardVerdict::Vulnerable`], the alerts.
    pub fn vulnerable(&self) -> impl Iterator<Item = &SinkFinding> {
        self.findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable && f.alert.is_some())
    }

    /// True if any sink argument was reached by a source in a dangerous
    /// slot (sink-signature aware).
    pub fn has_alert(&self) -> bool {
        self.vulnerable().next().is_some()
            // Learned/built-in policy checks are alerts too: a corpus family
            // whose positive violates a learned check must separate exactly
            // like a taint family. Without this, Check facts could never be
            // validated by the replay gate.
            || !self.checker.is_empty()
    }
}
