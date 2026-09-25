// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

use super::options::CliOptions;
use crate::reporter::Reporter;
use crate::{Advisory, FrensenseError, Result};
use std::collections::HashSet;

/// Print results in the requested format.
///
/// # Errors
/// Returns an error if serialization fails.
pub fn print_results(
    advisories: &[Advisory],
    format: &str,
    input_path: &std::path::Path,
) -> Result<()> {
    match format {
        "json" => {
            let wrapper = serde_json::json!({
                "clean": advisories.is_empty(),
                "advisory_count": advisories.len(),
                "advisories": advisories,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&wrapper)
                    .map_err(|e| FrensenseError::Config(format!("JSON error: {e}")))?
            );
        }
        "sarif" => {
            let sarif = Reporter::to_sarif(advisories, input_path);
            println!(
                "{}",
                serde_json::to_string_pretty(&sarif)
                    .map_err(|e| FrensenseError::Config(format!("JSON error: {e}")))?
            );
        }
        _ => {
            if advisories.is_empty() {
                println!("Analysis Complete: No findings.");
            } else {
                println!("Analysis: {}", input_path.display());
                println!();
                for v in advisories {
                    let severity_label = match v.severity {
                        crate::Severity::Critical => "[CRITICAL]",
                        crate::Severity::Warning => "[WARNING]",
                        crate::Severity::Info => "[INFO]",
                    };
                    // Header on its own line: location never dangles onto a
                    // multi-line observation (previously the trailing
                    // (file:line:col) landed after the last taint-path step).
                    println!(
                        "{} {}: ({}:{}:{})",
                        severity_label, v.title, v.file_path, v.line, v.column
                    );
                    print_observation(&v.observation, &v.taint_steps);
                    println!("   - Impact: {}", v.impact);
                    println!("   - Suggestion: {}\n", v.improvement);
                }
                println!("Total Findings: {}", advisories.len());
            }
        }
    }
    Ok(())
}

/// Print an advisory observation, annotating taint-path steps with their
/// `file:line` when the step's span resolves in a readable file.
///
/// Cross-function flows are the point: a path that enters `writer`, stores
/// a field, and re-emerges through `reader` must show where each hop lives,
/// including files other than the finding's own. Byte offsets from the
/// engine's spans are converted to line numbers by reading the referenced
/// source (cached per path render; files that cannot be read keep the
/// unannotated step text).
fn print_observation(observation: &str, taint_steps: &[(String, Option<(String, usize)>)]) {
    let mut lines = observation.lines();
    // First line: the flow summary.
    if let Some(summary) = lines.next() {
        println!("   {}", summary);
    }
    let mut in_path = false;
    let mut step_idx = 0usize;
    for line in lines {
        if line.trim_end() == "Taint path:" {
            in_path = true;
            println!("   {line}");
            continue;
        }
        if in_path {
            // Path step: annotate with the step's location when available.
            let location = taint_steps
                .get(step_idx)
                .and_then(|(_, loc)| loc.as_ref())
                .and_then(|(file, byte)| line_for(file, *byte));
            step_idx += 1;
            match location {
                Some(loc) => println!("   {line}  [{loc}]"),
                None => println!("   {line}"),
            }
        } else {
            println!("   {line}");
        }
    }
}

/// Byte offset → 1-based line number for a file, reading it once per call
/// site (advisory counts are small; a global cache is unnecessary).
fn line_for(file: &str, byte_offset: usize) -> Option<String> {
    let source = std::fs::read(file).ok()?;
    let end = byte_offset.min(source.len());
    let line = source[..end].iter().filter(|&&b| b == b'\n').count() + 1;
    let name = std::path::Path::new(file)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| file.to_string());
    Some(format!("{name}:{line}"))
}

/// Save the current findings as a baseline.
///
/// # Errors
/// Returns an error if the baseline file cannot be written.
pub fn save_baseline(advisories: &[Advisory], path: &str) -> Result<()> {
    let content = serde_json::to_string_pretty(advisories)
        .map_err(|e| FrensenseError::Config(format!("JSON error: {e}")))?;
    std::fs::write(path, content)
        .map_err(|e| FrensenseError::Config(format!("Failed to write baseline: {e}")))?;
    println!("[SUCCESS] Captured baseline to {path}");
    Ok(())
}

/// Compare the current findings against a baseline.
///
/// # Errors
/// Returns an error if the baseline file cannot be read or parsed.
pub fn compare_baseline(advisories: &[Advisory], path: &str) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| FrensenseError::Config(format!("Failed to read baseline: {e}")))?;
    let baseline: Vec<Advisory> = serde_json::from_str(&content)
        .map_err(|e| FrensenseError::Config(format!("Failed to parse baseline: {e}")))?;

    let baseline_ids: HashSet<_> = baseline.iter().map(Advisory::identity).collect();
    let new_advisories: Vec<_> = advisories
        .iter()
        .filter(|a| !baseline_ids.contains(&a.identity()))
        .collect();

    let regression_detected = !new_advisories.is_empty();
    if regression_detected {
        println!(
            "\n[REGRESSION] {} new advisories detected!",
            new_advisories.len()
        );
        for adv in &new_advisories {
            println!("  + {}:{} ({})", adv.file_path, adv.line, adv.title);
        }
    } else {
        println!("\n[OK] No new advisories compared to baseline.");
    }
    Ok(regression_detected)
}

/// Apply CLI filters to advisories.
pub fn apply_filters(advisories: &mut [Advisory], options: &CliOptions) {
    let _ = (advisories, options);
}
