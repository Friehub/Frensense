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
    pub mine_negatives: bool,
    pub mine_negatives_dir: String,
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
    pub use_compiler: bool,
    pub ngram_sim_threshold: Option<f64>,
    pub emit_hypotheses: bool,
    // Scorer configuration
    pub scorer_cross_lingual_penalty: Option<f32>,
    pub scorer_semantic_zero_penalty: Option<f64>,
    pub scorer_semantic_match_boost: Option<f64>,
    pub scorer_noise_gate_moderate: Option<f64>,
    pub scorer_noise_gate_strong: Option<f64>,
    pub scorer_neg_penalty_floor: Option<f64>,
    pub scorer_neg_penalty_weight: Option<f64>,
    pub scorer_context_mismatch_penalty: Option<f64>,
    // Taint/verification config
    pub taint_verified_boost: Option<f64>,
    pub cross_file_taint_boost: Option<f64>,
    pub taint_boost_cap: Option<f64>,
    pub score_suppression_floor: Option<f64>,
    // LSH config
    pub lsh_num_hashes: Option<usize>,
    pub lsh_bands: Option<usize>,
    pub lsh_rows_per_band: Option<usize>,
    // Fingerprinting config
    pub ngram_windows: Option<String>,
    pub cf_max_depth: Option<usize>,
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
        mine_negatives: false,
        mine_negatives_dir: String::from("mined_negatives"),
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
        use_compiler: false,
        ngram_sim_threshold: None,
        emit_hypotheses: false,
        scorer_cross_lingual_penalty: None,
        scorer_semantic_zero_penalty: None,
        scorer_semantic_match_boost: None,
        scorer_noise_gate_moderate: None,
        scorer_noise_gate_strong: None,
        scorer_neg_penalty_floor: None,
        scorer_neg_penalty_weight: None,
        scorer_context_mismatch_penalty: None,
        taint_verified_boost: None,
        cross_file_taint_boost: None,
        taint_boost_cap: None,
        score_suppression_floor: None,
        lsh_num_hashes: None,
        lsh_bands: None,
        lsh_rows_per_band: None,
        ngram_windows: None,
        cf_max_depth: None,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => options.format = "json".to_string(),
            "--sarif" => options.format = "sarif".to_string(),
            "--emit-hypotheses" => options.emit_hypotheses = true,
            "--use-compiler" => options.use_compiler = true,
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
                        "taint-only" => "taint-only".to_string(),
                        _ => {
                            eprintln!("Error: Unknown mode '{}'. Valid: fast, taint, taint-only", val);
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
            "--mine-negatives" => {
                options.mine_negatives = true;
                if let Some(dir) = args.get(i + 1) {
                    if !dir.starts_with('-') {
                        options.mine_negatives_dir = dir.clone();
                        i += 1;
                    }
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
            // Scorer configuration flags
            "--scorer-cross-lingual-penalty" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_cross_lingual_penalty = Some(val.parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-cross-lingual-penalty value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-semantic-zero-penalty" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_semantic_zero_penalty = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-semantic-zero-penalty value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-semantic-match-boost" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_semantic_match_boost = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-semantic-match-boost value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-noise-gate-moderate" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_noise_gate_moderate = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-noise-gate-moderate value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-noise-gate-strong" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_noise_gate_strong = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-noise-gate-strong value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-neg-penalty-floor" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_neg_penalty_floor = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-neg-penalty-floor value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-neg-penalty-weight" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_neg_penalty_weight = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-neg-penalty-weight value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--scorer-context-mismatch-penalty" => {
                if let Some(val) = args.get(i + 1) {
                    options.scorer_context_mismatch_penalty = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --scorer-context-mismatch-penalty value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--taint-verified-boost" => {
                if let Some(val) = args.get(i + 1) {
                    options.taint_verified_boost = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --taint-verified-boost value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--cross-file-taint-boost" => {
                if let Some(val) = args.get(i + 1) {
                    options.cross_file_taint_boost = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --cross-file-taint-boost value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--taint-boost-cap" => {
                if let Some(val) = args.get(i + 1) {
                    options.taint_boost_cap = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --taint-boost-cap value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--score-suppression-floor" => {
                if let Some(val) = args.get(i + 1) {
                    options.score_suppression_floor = Some(val.parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --score-suppression-floor value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--lsh-num-hashes" => {
                if let Some(val) = args.get(i + 1) {
                    options.lsh_num_hashes = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --lsh-num-hashes value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--lsh-bands" => {
                if let Some(val) = args.get(i + 1) {
                    options.lsh_bands = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --lsh-bands value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--lsh-rows-per-band" => {
                if let Some(val) = args.get(i + 1) {
                    options.lsh_rows_per_band = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --lsh-rows-per-band value '{val}'");
                        std::process::exit(1);
                    }));
                    i += 1;
                }
            }
            "--ngram-windows" => {
                if let Some(val) = args.get(i + 1) {
                    options.ngram_windows = Some(val.clone());
                    i += 1;
                }
            }
            "--cf-max-depth" => {
                if let Some(val) = args.get(i + 1) {
                    options.cf_max_depth = Some(val.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid --cf-max-depth value '{val}'");
                        std::process::exit(1);
                    }));
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
