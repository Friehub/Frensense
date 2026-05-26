// SPDX-License-Identifier: MIT
#![warn(clippy::unwrap_used)]

use gensense::parser::ParserRegistry;
#[cfg(feature = "remediation")]
use gensense::patcher::PatchManager;
use gensense::{Advisory, Engine, Result, Severity};
use std::env;
use std::path::{Path, PathBuf};

fn print_help() {
    println!("GenSense - Semantic Code Analysis Engine");
    println!("Version: {}", gensense::GENSENSE_VERSION);
    println!("Analyzes Rust, TypeScript, JavaScript, and YAML codebases for bugs,");
    println!("anti-patterns, security risks, and SQL drift — with AST-level precision.");
    println!();
    println!("Usage: gensense [path] [options]");
    println!();
    println!("Arguments:");
    println!("  path                File or directory to scan (default: current directory)");
    println!();
    println!("Analysis Options:");
    println!("  --language <lang>   Language filter: rust, typescript, javascript, yaml");
    println!("  --diff-only         Only scan files changed since the last git commit");
    println!("  --severity <level>  Minimum severity: critical, warning, info");
    println!("  --tag <name>        Enable optional diagnostic tag (e.g., sbom, governance)");
    println!("  --suite <name>      Rule precision tier: default, extended, all");
    println!("  --no-builtin-rules  Disable built-in rules (use only custom --rules-dir)");
    println!("  --rules-dir <dir>   Load custom rules from a directory");
    println!("  --disable-rule <id> Disable a specific rule by ID (repeatable)");
    println!("  --override-severity <RULE_ID>:<level>  Override severity for a rule");
    println!("                      Level: critical, warning, info (repeatable)");
    println!();
    println!("Confidence & Tuning:");
    println!("  --confidence <tier>      Preset: high (≥0.85), medium (≥0.60), low (≥0.30), any");
    println!("  --min-confidence <0-1>   Raw confidence threshold (default: 0.0)");
    println!("  --jaccard-threshold <0-1>  Similarity threshold for duplicate detection");
    println!("  --confidence-boost-rate <0-1>  Boost rate for overlapping findings");
    println!("  --confidence-boost-max <0-1>   Maximum confidence boost");
    println!("  --max-source-lines <N>   Limit source lines for analysis");
    println!("  --ngram-window <N>       Fingerprint n-gram window size");
    println!("  --min-ngram-count <N>    Minimum n-gram count threshold");
    println!("  --taint-conf-inter <0-1> Interprocedural taint confidence threshold");
    println!("  --taint-conf-intra <0-1> Intraprocedural taint confidence threshold");
    println!("  --taint-max-depth <N>    Maximum taint propagation depth");
    println!();
    println!("Output Options:");
    println!("  --json              Output findings as JSON");
    println!("  --sarif             Output findings in SARIF format");
    println!("  --strict            Exit with code 1 if any findings match filter");
    println!("  --emit-baseline <file>   Save current findings as a baseline");
    println!("  --compare-baseline <file>  Compare findings against a baseline");
    #[cfg(feature = "remediation")]
    println!("  --fix               Apply automated remediation (experimental)");
    #[cfg(feature = "remediation")]
    println!("  --diff              Show unified diff of proposed changes");
    println!();
    println!("Development:");
    println!("  --version           Display version and enabled features");
    println!("  --list-rules        List all active rules and their severities");
    println!("  --generate-docs     Generate RULES.md documentation file");
    println!("  --debug <file>      Dump anonymized AST for a source file");
    println!("  test-rule <rule.yml> --fixture <file> --expect-finding <id>");
    println!("                      Test a custom rule against a fixture file");
    println!("                      Optional: --expect-line <N>");
    println!();
    println!("Examples:");
    println!("  gensense                            Scan current directory");
    println!("  gensense src/                       Scan a specific directory");
    println!("  gensense main.rs                    Scan a single file");
    println!("  gensense --language rust .           Scan Rust files only");
    println!("  gensense --diff-only --strict        Check changed files, fail on any finding");
    println!("  gensense --json --suite extended     Export extended scan as JSON");
    println!("  gensense --disable-rule RUST_STD_OUTPUT .    Disable a specific rule");
    println!("  gensense --override-severity FILE_TOO_LONG:info .  Change rule severity");
    println!("  gensense --emit-baseline baseline.json   Save baseline");
    println!("  gensense --compare-baseline baseline.json  Check for regressions");
    println!();
    println!("Features Enabled:");
    #[cfg(feature = "rust")]
    println!("  [x] Rust Analysis");
    #[cfg(feature = "typescript")]
    println!("  [x] TypeScript/JS Analysis");
    #[cfg(feature = "fingerprinting")]
    println!("  [x] N-Gram Fingerprinting");
    #[cfg(feature = "remediation")]
    println!("  [x] Auto-Remediation");
}

fn print_version() {
    println!(
        "GenSense v{} - Semantic Code Analysis Engine",
        gensense::GENSENSE_VERSION
    );
    println!("Ship with confidence. Audit with insight.");
    println!("\nFeatures Enabled:");
    #[cfg(feature = "rust")]
    println!("  [x] Rust Analysis");
    #[cfg(feature = "typescript")]
    println!("  [x] TypeScript/JS Analysis");
    #[cfg(feature = "fingerprinting")]
    println!("  [x] N-Gram Fingerprinting");
    #[cfg(feature = "remediation")]
    println!("  [x] Auto-Remediation");
}

#[derive(serde::Deserialize)]
struct RulesWrapper {
    rules: Vec<gensense::rules::core::CoreRule>,
}

fn handle_test_rule(args: &[String]) {
    if args.len() < 7 {
        eprintln!(
            "Usage: gensense test-rule <rule.yml> --fixture <file> --expect-finding <id> [--expect-line <N>]"
        );
        std::process::exit(1);
    }
    let rule_file_path = args[2].clone();

    let mut fixture = None;
    let mut expect_id = None;
    let mut expect_line = None;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--fixture" => {
                if let Some(f) = args.get(i + 1) {
                    fixture = Some(f.clone());
                    i += 1;
                }
            }
            "--expect-finding" => {
                if let Some(id) = args.get(i + 1) {
                    expect_id = Some(id.clone());
                    i += 1;
                }
            }
            "--expect-line" => {
                if let Some(line) = args.get(i + 1) {
                    expect_line = line.parse::<u32>().ok();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let fixture_path = fixture.expect("Missing --fixture argument");
    let expected_id = expect_id.expect("Missing --expect-finding argument");

    let rule_content = std::fs::read_to_string(&rule_file_path).expect("Failed to read rule file");

    let wrapper: RulesWrapper =
        serde_yaml::from_str(&rule_content).expect("Failed to parse YAML rules");
    let mut rules: Vec<Box<dyn gensense::GenSenseRule>> = Vec::new();
    for rule in wrapper.rules {
        match gensense::rules::compiler::RuleCompiler::compile(rule) {
            Ok(compiled) => rules.push(Box::new(compiled)),
            Err(e) => {
                eprintln!("Error compiling rule: {e}");
                std::process::exit(1);
            }
        }
    }

    let mut engine = Engine::new();
    engine.set_rules(rules);
    engine.set_isolate_rules(true);

    let advisories = engine
        .run(Path::new(&fixture_path))
        .expect("Analysis failed");

    advisories
        .iter()
        .find(|a| a.rule_id == expected_id)
        .map_or_else(
            || {
                println!("[FAIL: Rule not triggered] Expected to find rule {expected_id}");
                std::process::exit(1);
            },
            |finding| {
                if let Some(expected_line) = expect_line
                    && finding.line != expected_line
                {
                    println!(
                        "[FAIL: Line mismatch] Expected finding on line {}, but found on line {}",
                        expected_line, finding.line
                    );
                    std::process::exit(1);
                }
                println!("[PASS]");
                std::process::exit(0);
            },
        );
}

fn handle_generate_docs() -> Result<()> {
    use std::fmt::Write;
    let engine = Engine::new();
    let _ = engine.list_rules();
    let mut doc = String::new();
    doc.push_str("# GenSense Rule Catalog\n\n");
    doc.push_str(
        "This catalog lists all semantic rules currently active in the GenSense engine.\n\n",
    );
    doc.push_str("| Rule ID | Severity | Category | Description |\n");
    doc.push_str("| :--- | :--- | :--- | :--- |\n");
    for rule in engine.auditor().rules() {
        let meta = rule.metadata();
        let _ = writeln!(
            doc,
            "| `{}` | {:?} | {} | {} |",
            rule.id(),
            meta.severity,
            meta.category,
            meta.impact
        );
    }
    std::fs::write("RULES.md", doc).expect("Failed to write RULES.md");
    println!("[SUCCESS] Generated RULES.md");
    std::process::exit(0);
}

fn handle_debug_ast(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    let mut parser = tree_sitter::Parser::new();
    let language =
        gensense::parser::ParserRegistry::get_language(path).expect("Unsupported language");
    parser
        .set_language(&language)
        .expect("Failed to set language");
    let tree = parser.parse(&content, None).expect("Parse failure");
    println!("Anonymized AST for {file_path}:\n");
    println!("{}", tree.root_node().to_sexp());
    std::process::exit(0);
}

fn handle_list_rules() -> Result<()> {
    let (rules, _project_rules) = gensense::engine::auditor::GenSenseAuditor::default_rules();
    let mut engine = Engine::new();
    engine.set_rules(rules);
    let catalog = engine.list_rules();
    println!("GenSense: Active Rules Catalog");
    println!("{:->100}", "");
    println!("{:<30} | {:<10} | Description", "Rule ID", "Severity");
    println!("{:->100}", "");
    for (id, name, sev) in catalog {
        println!("{id:<30} | {sev:<10} | {name}");
    }
    std::process::exit(0);
}

#[allow(clippy::struct_excessive_bools)]
struct CliOptions {
    format: String,
    is_strict: bool,
    do_fix: bool,
    show_diff: bool,
    diff_only: bool,
    severity_filter: Option<Severity>,
    enabled_tags: Vec<String>,
    extra_rule_dirs: Vec<PathBuf>,
    no_builtin: bool,
    emit_baseline_path: Option<String>,
    compare_baseline_path: Option<String>,
    min_confidence: f32,
    language_filter: Option<String>,
    suite: gensense::Suite,
    jaccard_threshold: Option<f64>,
    confidence_boost_rate: Option<f32>,
    confidence_boost_max: Option<f32>,
    max_source_lines: Option<usize>,
    ngram_window_size: Option<usize>,
    min_ngram_count: Option<usize>,
    taint_confidence_interprocedural: Option<f32>,
    taint_confidence_intraprocedural: Option<f32>,
    default_taint_max_depth: Option<usize>,
    disabled_rules: Vec<String>,
    severity_overrides: Vec<(String, Severity)>,
}

#[allow(clippy::too_many_lines)]
fn parse_options(args: &[String]) -> CliOptions {
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
        suite: gensense::Suite::All,
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
    };

    let mut i = 2;
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
                        "critical" => Some(gensense::Severity::Critical),
                        "warning" => Some(gensense::Severity::Warning),
                        "info" => Some(gensense::Severity::Info),
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
                        "default" => gensense::Suite::Default,
                        "extended" => gensense::Suite::Extended,
                        "all" => gensense::Suite::All,
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
                            "critical" => gensense::Severity::Critical,
                            "warning" => gensense::Severity::Warning,
                            "info" => gensense::Severity::Info,
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
            _ => {}
        }
        i += 1;
    }
    options
}

fn print_results(
    filtered_advisories: &[gensense::Advisory],
    format: &str,
    input_path: &Path,
) -> Result<()> {
    match format {
        "json" => {
            let clean = filtered_advisories.is_empty();
            let advisory_count = filtered_advisories.len();
            let requires_human_count = filtered_advisories
                .iter()
                .filter(|a| a.requires_human)
                .count();
            let auto_fixable_count = filtered_advisories
                .iter()
                .filter(|a| a.auto_fixable)
                .count();

            let wrapper = serde_json::json!({
                "clean": clean,
                "advisory_count": advisory_count,
                "requires_human_count": requires_human_count,
                "auto_fixable_count": auto_fixable_count,
                "advisories": filtered_advisories,
            });

            println!(
                "{}",
                serde_json::to_string_pretty(&wrapper)
                    .map_err(|e| gensense::GenSenseError::Config(format!("JSON error: {e}")))?
            );
        }
        "sarif" => {
            let sarif = gensense::reporter::Reporter::to_sarif(filtered_advisories, input_path);
            println!(
                "{}",
                serde_json::to_string_pretty(&sarif)
                    .map_err(|e| gensense::GenSenseError::Config(format!("JSON error: {e}")))?
            );
        }
        _ => {
            if filtered_advisories.is_empty() {
                println!("Analysis Complete: Looking great! No structural concerns found.");
            } else {
                println!("╔══════════════════════════════════════════════════╗");
                println!(
                    "║  GenSense v{}                              ║",
                    gensense::GENSENSE_VERSION
                );
                println!("║  Semantic Code Analysis Engine                ║");
                println!("╚══════════════════════════════════════════════════╝");
                println!("Analysis: {}", input_path.display());
                println!();
                for v in filtered_advisories {
                    let severity_label = match v.severity {
                        gensense::Severity::Critical => "[CRITICAL]",
                        gensense::Severity::Warning => "[WARNING]",
                        gensense::Severity::Info => "[INFO]",
                    };
                    println!(
                        "{} {}: {} ({}:{}:{})",
                        severity_label, v.rule_id, v.observation, v.file_path, v.line, v.column
                    );
                    println!("   - Impact: {}", v.impact);
                    println!("   - Suggestion: {}\n", v.improvement);
                }
                println!("Total Review Suggestions: {}", filtered_advisories.len());
            }
        }
    }
    Ok(())
}

fn compare_baseline(filtered_advisories: &[gensense::Advisory], path: &str) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| gensense::GenSenseError::Config(format!("Failed to read baseline: {e}")))?;
    let baseline: Vec<gensense::Advisory> = serde_json::from_str(&content)
        .or_else(|_| serde_yaml::from_str(&content))
        .map_err(|e| gensense::GenSenseError::Config(format!("Failed to parse baseline: {e}")))?;

    let baseline_fuzzy: std::collections::HashSet<_> = baseline
        .iter()
        .map(gensense::Advisory::fuzzy_identity)
        .collect();
    let current_fuzzy: std::collections::HashSet<_> = filtered_advisories
        .iter()
        .map(gensense::Advisory::fuzzy_identity)
        .collect();

    let new_advisories: Vec<_> = filtered_advisories
        .iter()
        .filter(|a| !baseline_fuzzy.contains(&a.fuzzy_identity()))
        .collect();
    let resolved_advisories: Vec<_> = baseline
        .iter()
        .filter(|a| !current_fuzzy.contains(&a.fuzzy_identity()))
        .collect();

    let mut regression_detected = false;
    if new_advisories.is_empty() {
        println!("\n[OK] No new advisories compared to baseline.");
    } else {
        println!(
            "\n[REGRESSION] {} new advisories detected!",
            new_advisories.len()
        );
        for adv in &new_advisories {
            println!("  + {}:{} ({})", adv.file_path, adv.line, adv.rule_id);
        }
        regression_detected = true;
    }

    if !resolved_advisories.is_empty() {
        println!(
            "[OK] {} advisories resolved since baseline.",
            resolved_advisories.len()
        );
    }

    let new_len: i128 = new_advisories.len() as i128;
    let resolved_len: i128 = resolved_advisories.len() as i128;
    let net = i64::try_from(new_len).unwrap_or(0) - i64::try_from(resolved_len).unwrap_or(0);
    println!(
        "[NET] {} ({} total findings)\n",
        if net > 0 {
            format!("+{net}")
        } else {
            net.to_string()
        },
        filtered_advisories.len()
    );
    Ok(regression_detected)
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    // nosemgrep: rust.lang.security.args.args
    let args: Vec<String> = env::args().collect();
    if handle_early_args(&args) {
        return Ok(());
    }

    let input_path = get_input_path(&args);
    let options = parse_options(&args);

    let mut engine = Engine::new();
    for dir in &options.extra_rule_dirs {
        engine.add_rule_dir(dir.clone());
    }
    engine.set_no_builtin_rules(options.no_builtin);
    engine.set_suite(options.suite);
    engine.set_severity_filter(options.severity_filter);

    if let Some(val) = options.jaccard_threshold {
        engine.set_jaccard_threshold(val);
    }
    if let Some(val) = options.confidence_boost_rate {
        engine.set_confidence_boost_rate(val);
    }
    if let Some(val) = options.confidence_boost_max {
        engine.set_confidence_boost_max(val);
    }
    if let Some(val) = options.max_source_lines {
        engine.set_max_source_lines(val);
    }
    if let Some(val) = options.ngram_window_size {
        engine.set_ngram_window_size(val);
    }
    if let Some(val) = options.min_ngram_count {
        engine.set_min_ngram_count(val);
    }
    if let Some(val) = options.taint_confidence_interprocedural {
        engine.set_taint_confidence_interprocedural(val);
    }
    if let Some(val) = options.taint_confidence_intraprocedural {
        engine.set_taint_confidence_intraprocedural(val);
    }
    if let Some(val) = options.default_taint_max_depth {
        engine.set_default_taint_max_depth(val);
    }

    for rule_id in &options.disabled_rules {
        engine.add_disabled_rule(rule_id);
    }
    for (rule_id, severity) in &options.severity_overrides {
        engine.add_severity_override(rule_id, *severity);
    }

    if let Some(lang_arg) = &options.language_filter {
        if let Some(exts) = gensense::parser::ParserRegistry::extensions_for(lang_arg) {
            engine.set_language_filter(exts);
        } else {
            eprintln!(
                "Error: Unknown language '{lang_arg}'. Supported values: rust, typescript/ts, javascript/js, yaml"
            );
            std::process::exit(1);
        }
    }

    let mut filtered_advisories = if options.diff_only {
        let repo_dir = if input_path.is_dir() {
            input_path.clone()
        } else {
            input_path.parent().unwrap_or(&input_path).to_path_buf()
        };
        let output = std::process::Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("HEAD")
            .current_dir(&repo_dir)
            .output()
            .map_err(|e| {
                gensense::GenSenseError::Config(format!(
                    "Failed to run git diff: {e} — is this a git repository?"
                ))
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(gensense::GenSenseError::Config(format!(
                "git diff failed: {stderr}"
            )));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let diff_files: Vec<PathBuf> = stdout
            .lines()
            .map(|l| repo_dir.join(l))
            .filter(|p| ParserRegistry::is_supported(p))
            .filter(|p| {
                if let Some(ref lang) = options.language_filter {
                    let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                    gensense::parser::ParserRegistry::extensions_for(lang)
                        .is_some_and(|exts| exts.contains(&ext))
                } else {
                    true
                }
            })
            .collect();

        if diff_files.is_empty() {
            eprintln!("No changed files to scan.");
            Vec::new()
        } else {
            eprintln!(
                "Diff-only: scanning {} changed file(s)...",
                diff_files.len()
            );
            engine.run_files(&input_path, &diff_files)?
        }
    } else {
        engine.run(&input_path)?
    };
    apply_filters(&mut filtered_advisories, &options, &engine);

    print_results(&filtered_advisories, &options.format, &input_path)?;

    if let Some(path) = &options.emit_baseline_path {
        save_baseline(&filtered_advisories, path)?;
    }

    let mut regression_detected = false;
    if let Some(path) = &options.compare_baseline_path {
        regression_detected = compare_baseline(&filtered_advisories, path)?;
    }

    #[cfg(feature = "remediation")]
    if (options.do_fix || options.show_diff) && !filtered_advisories.is_empty() {
        handle_remediation(&filtered_advisories, &options, &input_path);
    }

    if regression_detected || (options.is_strict && !filtered_advisories.is_empty()) {
        std::process::exit(1);
    }

    Ok(())
}

fn handle_early_args(args: &[String]) -> bool {
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        std::process::exit(0);
    }

    if args.contains(&"--version".to_string()) {
        print_version();
        std::process::exit(0);
    }

    if args.len() > 1 && args[1] == "test-rule" {
        handle_test_rule(args);
        return true;
    }

    if args.contains(&"--generate-docs".to_string()) {
        if let Err(e) = handle_generate_docs() {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return true;
    }

    if let Some(pos) = args.iter().position(|a| a == "--debug")
        && let Some(file_path) = args.get(pos + 1)
    {
        if let Err(e) = handle_debug_ast(file_path) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return true;
    }

    if args.contains(&"--list-rules".to_string()) {
        if let Err(e) = handle_list_rules() {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return true;
    }

    false
}

fn get_input_path(args: &[String]) -> PathBuf {
    let input_path_str = match args.get(1) {
        Some(path) if !path.starts_with("--") => path.clone(),
        _ => {
            // Default to current directory when no path is given
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
        eprintln!("Run 'gensense --help' for usage information");
        std::process::exit(1);
    }
}

fn apply_filters(advisories: &mut Vec<Advisory>, options: &CliOptions, engine: &Engine) {
    advisories.retain(|a| a.confidence >= options.min_confidence);

    if !options.enabled_tags.is_empty() {
        advisories.retain(|a| {
            engine.auditor().rules().iter().any(|r| {
                r.id() == a.rule_id
                    && options
                        .enabled_tags
                        .iter()
                        .any(|t| r.metadata().tags.iter().any(|rt| rt == t))
            })
        });
    }

    // Post-filter: severity threshold (redundant if pre-filter is active, but catches
    // any rules that bypass the pre-filter, e.g. direct API users)
    if let Some(filter) = options.severity_filter {
        advisories.retain(|a| a.severity.meets_threshold(filter));
    }
}

fn save_baseline(advisories: &[Advisory], path: &str) -> Result<()> {
    let content = serde_json::to_string_pretty(advisories)
        .map_err(|e| gensense::GenSenseError::Config(format!("JSON error: {e}")))?;
    std::fs::write(path, content)
        .map_err(|e| gensense::GenSenseError::Config(format!("Failed to write baseline: {e}")))?;
    println!("[SUCCESS] Captured baseline to {path}");
    Ok(())
}

#[cfg(feature = "remediation")]
fn handle_remediation(advisories: &[Advisory], options: &CliOptions, input_path: &Path) {
    // Find project root (where .gensense or .git exists)
    let mut project_root = input_path.to_path_buf();
    if project_root.is_file() {
        project_root = project_root.parent().unwrap_or(&project_root).to_path_buf();
    }

    while project_root.parent().is_some() {
        if project_root.join(".gensense").exists() || project_root.join(".git").exists() {
            break;
        }
        project_root = project_root
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();
    }

    let patcher = PatchManager::new(&project_root);

    let mut fix_advisories = advisories.to_vec();
    fix_advisories.retain(|a| a.proposed_replacement.is_some());
    // Sort DESC by start_byte to avoid offset drift within a file
    fix_advisories.sort_by_key(|a| std::cmp::Reverse(a.start_byte));

    let mut fixed_count = 0;
    let mut skipped_count = 0;

    for adv in &fix_advisories {
        if options.show_diff
            && let Ok(diff) = patcher.generate_diff(adv, Path::new(&adv.file_path))
        {
            println!("{diff}");
        }
        if options.do_fix {
            match patcher.apply_fix(adv, Path::new(&adv.file_path)) {
                Ok(()) => {
                    println!("[FIXED] {}:{} ({})", adv.file_path, adv.line, adv.rule_id);
                    fixed_count += 1;
                }
                Err(e) => {
                    eprintln!("[SKIP] {}: {}", adv.file_path, e);
                    skipped_count += 1;
                }
            }
        }
    }
    if options.do_fix {
        println!(
            "\n[DONE] {fixed_count} fixed, {skipped_count} skipped (context mismatch), 0 conflicts."
        );
    }
}
