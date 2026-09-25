// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

use crate::Severity;
use std::path::PathBuf;

/// CLI options, compiler mode only. Every scoring/fingerprint/corpus-threshold
/// knob is gone; the analysis is deterministic.
#[derive(Debug, Clone)]
pub struct CliOptions {
    pub format: String,
    pub is_strict: bool,
    pub severity_filter: Option<Severity>,
    pub language_filter: Option<String>,
    pub min_confidence: f64,
    pub corpus_bundle_path: Option<PathBuf>,
    pub emit_baseline_path: Option<String>,
    pub compare_baseline_path: Option<String>,
    pub diff_only: bool,
}

/// Parse CLI options.
#[must_use]
pub fn parse_options(args: &[String]) -> CliOptions {
    let mut options = CliOptions {
        format: "text".to_string(),
        is_strict: false,
        severity_filter: None,
        language_filter: None,
        min_confidence: 0.0,
        corpus_bundle_path: None,
        emit_baseline_path: None,
        compare_baseline_path: None,
        diff_only: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => options.format = "json".to_string(),
            "--sarif" => options.format = "sarif".to_string(),
            "--strict" => options.is_strict = true,
            "--diff-only" => options.diff_only = true,
            "--severity" => {
                if let Some(level) = args.get(i + 1) {
                    options.severity_filter = match level.to_lowercase().as_str() {
                        "critical" => Some(Severity::Critical),
                        "warning" => Some(Severity::Warning),
                        "info" => Some(Severity::Info),
                        _ => {
                            eprintln!("Error: Unknown severity level '{level}'");
                            std::process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            "--language" => {
                if let Some(val) = args.get(i + 1) {
                    options.language_filter = Some(val.clone());
                    i += 1;
                }
            }
            "--min-confidence" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(c) = val.parse::<f64>() {
                        options.min_confidence = c;
                    }
                    i += 1;
                }
            }
            "--corpus-bundle" | "--bundle" => {
                if let Some(val) = args.get(i + 1) {
                    options.corpus_bundle_path = Some(PathBuf::from(val));
                    i += 1;
                }
            }
            "--emit-baseline" => {
                if let Some(path) = args.get(i + 1) {
                    options.emit_baseline_path = Some(path.clone());
                    i += 1;
                }
            }
            "--compare-baseline" => {
                if let Some(path) = args.get(i + 1) {
                    options.compare_baseline_path = Some(path.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    options
}

/// Get the input path from args (defaults to the current directory).
#[must_use]
/// All positional input paths: every arg that is not an option/option-value.
/// The CLI accepts one or more files/directories; scanning merges findings.
pub fn get_input_paths(args: &[String]) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut skip_next = false;
    for a in args.iter().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if a.starts_with("--") {
            // Options with a separate value consume the next arg.
            skip_next = matches!(
                a.as_str(),
                "--format"
                    | "--lang"
                    | "--bundle"
                    | "--config"
                    | "--baseline"
                    | "--emit-baseline"
                    | "--min-confidence"
                    | "--severity"
            );
            continue;
        }
        let p = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(a);
        if p.exists() {
            paths.push(p.canonicalize().unwrap_or(p));
        } else {
            eprintln!("Error: path '{a}' does not exist - specify a valid file or directory");
            eprintln!();
            eprintln!("Run 'frensense --help' for usage information");
            std::process::exit(1);
        }
    }
    if paths.is_empty() {
        paths.push(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    }
    paths
}

/// Back-compat single-path accessor (first positional input).
pub fn get_input_path(args: &[String]) -> PathBuf {
    get_input_paths(args)
        .into_iter()
        .next()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}
