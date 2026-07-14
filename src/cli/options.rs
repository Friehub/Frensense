use crate::{Severity, Suite};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    Yes,
    No,
}
impl Flag {
    #[must_use]
    pub fn is_yes(&self) -> bool {
        matches!(self, Flag::Yes)
    }
}

pub struct CliOptions {
    pub format: String,
    pub is_strict: Flag,
    pub fix_scope: Option<String>,
    pub diff_scope: Option<String>,
    pub diff_only: Flag,
    pub severity_filter: Option<Severity>,
    pub enabled_tags: Vec<String>,
    pub emit_baseline_path: Option<String>,
    pub compare_baseline_path: Option<String>,
    pub min_confidence: f64,
    pub language_filter: Option<String>,
    pub suite: Suite,
    pub jaccard_threshold: Option<f64>,
    pub confidence_boost_rate: Option<f64>,
    pub confidence_boost_max: Option<f64>,
    pub max_source_lines: Option<usize>,
    pub ngram_window_size: Option<usize>,
    pub min_ngram_count: Option<usize>,
    pub taint_confidence_interprocedural: Option<f64>,
    pub taint_confidence_intraprocedural: Option<f64>,
    pub default_taint_max_depth: Option<usize>,
    pub disabled_rules: Vec<String>,
    pub severity_overrides: Vec<(String, Severity)>,
    #[cfg(feature = "fingerprinting")]
    pub learn_profile: Flag,
    #[cfg(feature = "fingerprinting")]
    pub check_profile: Flag,
    #[cfg(feature = "fingerprinting")]
    pub profile_threshold: Option<f64>,
    #[cfg(feature = "fingerprinting")]
    pub profile_stats: Flag,
    pub corpus_dir: Option<PathBuf>,
    pub corpus_threshold: f64,
    pub threshold_overrides: Vec<(String, f64)>,
    pub baseline_path: Option<PathBuf>,
    pub update_baseline: Flag,
    pub extra_taint_rule_dirs: Vec<PathBuf>,
    pub check_deps: Flag,
    pub learn_mode: Flag,
    pub learn_positive: Option<PathBuf>,
    pub learn_negative: Option<PathBuf>,
    pub learn_output: Option<PathBuf>,
    pub build_bundle: Flag,
    pub build_bundle_output: Option<PathBuf>,
    pub scan_mode: String,
    pub ngram_sim_threshold: Option<f64>,
}

#[allow(clippy::too_many_lines)]
/// Parse options.
///
/// # Panics
/// May panic if parsing fails.
pub fn parse_options(args: &[String]) -> CliOptions {
    let mut options = CliOptions {
        format: "text".to_string(),
        is_strict: Flag::No,
        fix_scope: None,
        diff_scope: None,
        diff_only: Flag::No,
        severity_filter: None,
        enabled_tags: Vec::new(),
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
        learn_profile: Flag::No,
        #[cfg(feature = "fingerprinting")]
        check_profile: Flag::No,
        #[cfg(feature = "fingerprinting")]
        profile_threshold: None,
        #[cfg(feature = "fingerprinting")]
        profile_stats: Flag::No,
        corpus_dir: None,
        corpus_threshold: 0.40,
        threshold_overrides: Vec::new(),
        baseline_path: None,
        update_baseline: Flag::No,
        extra_taint_rule_dirs: Vec::new(),
        check_deps: Flag::No,
        learn_mode: Flag::No,
        learn_positive: None,
        learn_negative: None,
        learn_output: None,
        build_bundle: Flag::No,
        build_bundle_output: None,
        scan_mode: "fast".to_string(),
        ngram_sim_threshold: None,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => options.format = "json".to_string(),
            "--sarif" => options.format = "sarif".to_string(),
            "--strict" => options.is_strict = Flag::Yes,
            "--fix" => {
                let scope = args.get(i + 1).map(std::string::String::as_str);
                match scope {
                    Some("style" | "security" | "all") => {
                        options.fix_scope = Some(scope.unwrap().to_string());
                        i += 1;
                    }
                    _ => options.fix_scope = Some("all".to_string()),
                }
            }
            "--diff" => {
                let scope = args.get(i + 1).map(std::string::String::as_str);
                match scope {
                    Some("style" | "security" | "all") => {
                        options.diff_scope = Some(scope.unwrap().to_string());
                        i += 1;
                    }
                    _ => options.diff_scope = Some("all".to_string()),
                }
            }
            "--diff-only" => options.diff_only = Flag::Yes,
            "--mode" => {
                if let Some(val) = args.get(i + 1) {
                    options.scan_mode = match val.to_lowercase().as_str() {
                        "fast" => "fast".to_string(),
                        "taint" => "taint".to_string(),
                        _ => {
                            eprintln!("Error: Unknown mode '{}'. Valid: fast, taint", val);
                            std::process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            "--ngram-sim-threshold" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(c) = val.parse::<f64>() {
                        options.ngram_sim_threshold = Some(c);
                    } else {
                        eprintln!("Error: Invalid --ngram-sim-threshold value '{val}'");
                        std::process::exit(1);
                    }
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
                    options.confidence_boost_rate = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --confidence-boost-rate value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--confidence-boost-max" => {
                if let Some(val) = args.get(i + 1) {
                    options.confidence_boost_max = Some(val.parse::<f64>().unwrap_or_else(|_| {
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
                        Some(val.parse::<f64>().unwrap_or_else(|_| {
                            eprintln!("Error: Invalid --taint-conf-inter value '{val}'");
                            std::process::exit(1);
                        }));
                    i += 1;
                }
            }
            "--taint-conf-intra" => {
                if let Some(val) = args.get(i + 1) {
                    options.taint_confidence_intraprocedural =
                        Some(val.parse::<f64>().unwrap_or_else(|_| {
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
            "--learn-profile" => options.learn_profile = Flag::Yes,
            #[cfg(feature = "fingerprinting")]
            "--check-profile" => options.check_profile = Flag::Yes,
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
            "--profile-stats" => options.profile_stats = Flag::Yes,
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
            "--threshold-sec" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(t) = val.parse::<f64>() {
                        options.threshold_overrides.push(("sec".to_string(), t));
                    }
                    i += 1;
                }
            }
            "--threshold-llm" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(t) = val.parse::<f64>() {
                        options.threshold_overrides.push(("llm".to_string(), t));
                    }
                    i += 1;
                }
            }
            "--threshold-arch" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(t) = val.parse::<f64>() {
                        options.threshold_overrides.push(("arch".to_string(), t));
                    }
                    i += 1;
                }
            }
            "--threshold-async" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(t) = val.parse::<f64>() {
                        options.threshold_overrides.push(("async".to_string(), t));
                    }
                    i += 1;
                }
            }
            "--threshold-csa" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(t) = val.parse::<f64>() {
                        options.threshold_overrides.push(("csa".to_string(), t));
                    }
                    i += 1;
                }
            }
            "--baseline" => {
                if let Some(val) = args.get(i + 1) {
                    options.baseline_path = Some(PathBuf::from(val));
                    i += 1;
                }
            }
            "--update-baseline" => options.update_baseline = Flag::Yes,
            "--extra-taint-rules" => {
                if let Some(val) = args.get(i + 1) {
                    options.extra_taint_rule_dirs.push(PathBuf::from(val));
                    i += 1;
                }
            }
            "--check-deps" => options.check_deps = Flag::Yes,
            "--learn" => {
                options.learn_mode = Flag::Yes;
                // Next two args are positive and negative files
                if let Some(pos) = args.get(i + 1) {
                    options.learn_positive = Some(PathBuf::from(pos));
                    i += 1;
                }
                if let Some(neg) = args.get(i + 1) {
                    options.learn_negative = Some(PathBuf::from(neg));
                    i += 1;
                }
            }
            "--learn-output" => {
                if let Some(val) = args.get(i + 1) {
                    options.learn_output = Some(PathBuf::from(val));
                    i += 1;
                }
            }
            "--build-bundle" => options.build_bundle = Flag::Yes,
            "--build-bundle-output" => {
                if let Some(val) = args.get(i + 1) {
                    options.build_bundle_output = Some(PathBuf::from(val));
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    options
}

#[must_use]
/// Get input path.
///
/// # Panics
/// May panic if current directory cannot be determined.
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
