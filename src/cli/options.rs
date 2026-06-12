#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]
use crate::{Severity, Suite};
use std::path::PathBuf;

#[allow(clippy::struct_excessive_bools)]
pub struct CliOptions {
    pub format: String,
    pub is_strict: bool,
    pub do_fix: bool,
    pub show_diff: bool,
    pub diff_only: bool,
    pub severity_filter: Option<Severity>,
    pub enabled_tags: Vec<String>,
    pub extra_rule_dirs: Vec<PathBuf>,
    pub no_builtin: bool,
    pub emit_baseline_path: Option<String>,
    pub compare_baseline_path: Option<String>,
    pub min_confidence: f32,
    pub language_filter: Option<String>,
    pub suite: Suite,
    pub jaccard_threshold: Option<f64>,
    pub confidence_boost_rate: Option<f32>,
    pub confidence_boost_max: Option<f32>,
    pub max_source_lines: Option<usize>,
    pub ngram_window_size: Option<usize>,
    pub min_ngram_count: Option<usize>,
    pub taint_confidence_interprocedural: Option<f32>,
    pub taint_confidence_intraprocedural: Option<f32>,
    pub default_taint_max_depth: Option<usize>,
    pub disabled_rules: Vec<String>,
    pub severity_overrides: Vec<(String, Severity)>,
    #[cfg(feature = "fingerprinting")]
    pub learn_profile: bool,
    #[cfg(feature = "fingerprinting")]
    pub check_profile: bool,
    #[cfg(feature = "fingerprinting")]
    pub profile_threshold: Option<f64>,
    #[cfg(feature = "fingerprinting")]
    pub profile_stats: bool,
    pub corpus_dir: Option<PathBuf>,
    pub corpus_threshold: f64,
}

#[allow(clippy::too_many_lines)]
pub fn parse_options(args: &[String]) -> CliOptions {
    let mut options = CliOptions {
        format: "text".to_string(),
        is_strict: false,
        do_fix: false,
        show_diff: false,
        diff_only: false,
        severity_filter: None,
        enabled_tags: Vec::new(),
        extra_rule_dirs: Vec::new(),
        no_builtin: false,
        emit_baseline_path: None,
        compare_baseline_path: None,
        min_confidence: 0.0,
        language_filter: None,
        suite: Suite::All,
        jaccard_threshold: None,
        confidence_boost_rate: None,
        confidence_boost_max: None,
        max_source_lines: None,
        ngram_window_size: None,
        min_ngram_count: None,
        taint_confidence_interprocedural: None,
        taint_confidence_intraprocedural: None,
        default_taint_max_depth: None,
        disabled_rules: Vec::new(),
        severity_overrides: Vec::new(),
        #[cfg(feature = "fingerprinting")]
        learn_profile: false,
        #[cfg(feature = "fingerprinting")]
        check_profile: false,
        #[cfg(feature = "fingerprinting")]
        profile_threshold: None,
        #[cfg(feature = "fingerprinting")]
        profile_stats: false,
        corpus_dir: None,
        corpus_threshold: 0.65,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => options.format = "json".to_string(),
            "--sarif" => options.format = "sarif".to_string(),
            "--strict" => options.is_strict = true,
            #[cfg(feature = "remediation")]
            "--fix" => options.do_fix = true,
            #[cfg(feature = "remediation")]
            "--diff" => options.show_diff = true,
            "--diff-only" => options.diff_only = true,
            "--no-builtin-rules" => options.no_builtin = true,
            "--min-confidence" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(c) = val.parse::<f32>() {
                        options.min_confidence = c;
                    }
                    i += 1;
                }
            }
            "--confidence" => {
                if let Some(val) = args.get(i + 1) {
                    options.min_confidence = match val.to_lowercase().as_str() {
                        "high" => 0.85,
                        "medium" => 0.60,
                        "low" => 0.30,
                        "any" => 0.0,
                        _ => {
                            eprintln!(
                                "Error: Unknown confidence tier '{val}'. Valid: high, medium, low, any"
                            );
                            std::process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            "--rules-dir" => {
                if let Some(dir) = args.get(i + 1) {
                    options.extra_rule_dirs.push(PathBuf::from(dir));
                    i += 1;
                }
            }
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
            "--tag" => {
                if let Some(tag) = args.get(i + 1) {
                    options.enabled_tags.push(tag.clone());
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
            "--language" => {
                if let Some(val) = args.get(i + 1) {
                    options.language_filter = Some(val.clone());
                    i += 1;
                }
            }
            "--suite" => {
                if let Some(val) = args.get(i + 1) {
                    options.suite = match val.to_lowercase().as_str() {
                        "default" => Suite::Default,
                        "extended" => Suite::Extended,
                        "all" => Suite::All,
                        _ => {
                            eprintln!(
                                "Error: Unknown suite '{val}'. Valid values: default, extended, all"
                            );
                            std::process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            "--jaccard-threshold" => {
                if let Some(val) = args.get(i + 1) {
                    options.jaccard_threshold = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --jaccard-threshold value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--confidence-boost-rate" => {
                if let Some(val) = args.get(i + 1) {
                    options.confidence_boost_rate = Some(val.parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --confidence-boost-rate value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--confidence-boost-max" => {
                if let Some(val) = args.get(i + 1) {
                    options.confidence_boost_max = Some(val.parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --confidence-boost-max value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--max-source-lines" => {
                if let Some(val) = args.get(i + 1) {
                    options.max_source_lines = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --max-source-lines value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--ngram-window" => {
                if let Some(val) = args.get(i + 1) {
                    options.ngram_window_size = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --ngram-window value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--min-ngram-count" => {
                if let Some(val) = args.get(i + 1) {
                    options.min_ngram_count = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --min-ngram-count value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--taint-conf-inter" => {
                if let Some(val) = args.get(i + 1) {
                    options.taint_confidence_interprocedural =
                        Some(val.parse::<f32>().unwrap_or_else(|_| {
                            eprintln!("Error: Invalid --taint-conf-inter value '{val}'");
                            std::process::exit(1);
                        }));
                    i += 1;
                }
            }
            "--taint-conf-intra" => {
                if let Some(val) = args.get(i + 1) {
                    options.taint_confidence_intraprocedural =
                        Some(val.parse::<f32>().unwrap_or_else(|_| {
                            eprintln!("Error: Invalid --taint-conf-intra value '{val}'");
                            std::process::exit(1);
                        }));
                    i += 1;
                }
            }
            "--taint-max-depth" => {
                if let Some(val) = args.get(i + 1) {
                    options.default_taint_max_depth =
                        Some(val.parse::<usize>().unwrap_or_else(|_| {
                            eprintln!("Error: Invalid --taint-max-depth value '{val}'");
                            std::process::exit(1);
                        }));
                    i += 1;
                }
            }
            "--disable-rule" => {
                if let Some(val) = args.get(i + 1) {
                    options.disabled_rules.push(val.clone());
                    i += 1;
                }
            }
            "--override-severity" => {
                if let Some(val) = args.get(i + 1) {
                    if let Some((rule_id, level)) = val.split_once(':') {
                        let severity = match level.to_lowercase().as_str() {
                            "critical" => Severity::Critical,
                            "warning" => Severity::Warning,
                            "info" => Severity::Info,
                            _ => {
                                eprintln!(
                                    "Error: Unknown severity level '{level}' in --override-severity. Valid: critical, warning, info"
                                );
                                std::process::exit(1);
                            }
                        };
                        options
                            .severity_overrides
                            .push((rule_id.to_string(), severity));
                    } else {
                        eprintln!("Error: --override-severity requires format <RULE_ID>:<level>");
                        std::process::exit(1);
                    }
                    i += 1;
                }
            }
            #[cfg(feature = "fingerprinting")]
            "--learn-profile" => options.learn_profile = true,
            #[cfg(feature = "fingerprinting")]
            "--check-profile" => options.check_profile = true,
            #[cfg(feature = "fingerprinting")]
            "--profile-threshold" => {
                if let Some(val) = args.get(i + 1) {
                    options.profile_threshold = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --profile-threshold value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            #[cfg(feature = "fingerprinting")]
            "--profile-stats" => options.profile_stats = true,
            "--corpus" => {
                if let Some(val) = args.get(i + 1) {
                    options.corpus_dir = Some(PathBuf::from(val));
                    i += 1;
                }
            }
            "--threshold" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(t) = val.parse::<f64>() {
                        options.corpus_threshold = t;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    options
}

pub fn get_input_path(args: &[String]) -> PathBuf {
    let input_path_str = match args.get(1) {
        Some(path) if !path.starts_with("--") => path.clone(),
        _ => {
            return std::env::current_dir().expect("Failed to get current directory");
        }
    };
    let input_path_buf = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(&input_path_str);
    if input_path_buf.exists() {
        input_path_buf.canonicalize().unwrap_or(input_path_buf)
    } else {
        eprintln!(
            "Error: path '{input_path_str}' does not exist — specify a valid file or directory"
        );
        eprintln!();
        eprintln!("Run 'frensense --help' for usage information");
        std::process::exit(1);
    }
}
