// SPDX-License-Identifier: MIT
#![warn(clippy::unwrap_used)]

#[cfg(feature = "remediation")]
use gensense::patcher::PatchManager;
use gensense::{Advisory, Engine, Result, Severity};
use std::env;
use std::path::{Path, PathBuf};

fn print_help() {
    println!("GenSense - Semantic Insight for Modern Codebases");
    println!("Version: {}", gensense::GENSENSE_VERSION);
    println!("Usage: gensense <path> [options]");
    println!("\nOptions:");
    println!("  --version          Display version and features");
    println!("  --list-rules       Display the active rules catalog");
    println!("  --generate-docs    Generate RULES.md documentation");
    println!("  --debug <file>     Anonymized AST debug dump");
    println!("  --severity <level> Filter findings by severity (critical, warning, info)");
    println!("  --min-confidence <0.0-1.0> Filter findings by confidence score (default: 0.0)");
    println!("  --tag <name>       Enable an optional diagnostic tag (e.g., sbom, governance)");
    println!("  --strict           Exit with code 1 if any findings match filter");
    println!("  --json             Output findings as JSON");
    println!("  --sarif            Output findings in SARIF format");
    println!("  --emit-baseline <file> Capture current advisories to a baseline file");
    println!("  --compare-baseline <file> Compare against a baseline and fail on regressions");
    #[cfg(feature = "remediation")]
    println!("  --fix              Apply automated remediation (experimental)");
    #[cfg(feature = "remediation")]
    println!("  --diff             Show unified diff of proposed changes");

    println!("\nFeatures Enabled:");
    #[cfg(feature = "rust")]
    println!("  [x] Rust Analysis");
    #[cfg(feature = "typescript")]
    println!("  [x] TypeScript/JS Analysis");
    #[cfg(feature = "solidity")]
    println!("  [x] Solidity Analysis");
    #[cfg(feature = "fingerprinting")]
    println!("  [x] N-Gram Fingerprinting");
    #[cfg(feature = "remediation")]
    println!("  [x] Auto-Remediation");
}

fn print_version() {
    println!("GenSense version {}", gensense::GENSENSE_VERSION);
    println!("\nFeatures Enabled:");
    #[cfg(feature = "rust")]
    println!("  [x] Rust Analysis");
    #[cfg(feature = "typescript")]
    println!("  [x] TypeScript/JS Analysis");
    #[cfg(feature = "solidity")]
    println!("  [x] Solidity Analysis");
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
                if let Some(expected_line) = expect_line {
                    if finding.line != expected_line {
                        println!(
                            "[FAIL: Line mismatch] Expected finding on line {}, but found on line {}",
                            expected_line, finding.line
                        );
                        std::process::exit(1);
                    }
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
    let engine = Engine::new();
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
    severity_filter: Option<Severity>,
    enabled_tags: Vec<String>,
    extra_rule_dirs: Vec<PathBuf>,
    no_builtin: bool,
    emit_baseline_path: Option<String>,
    compare_baseline_path: Option<String>,
    min_confidence: f32,
}

fn parse_options(args: &[String]) -> CliOptions {
    let mut options = CliOptions {
        format: "text".to_string(),
        is_strict: false,
        do_fix: false,
        show_diff: false,
        severity_filter: None,
        enabled_tags: Vec::new(),
        extra_rule_dirs: Vec::new(),
        no_builtin: false,
        emit_baseline_path: None,
        compare_baseline_path: None,
        min_confidence: 0.0,
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
            "--no-builtin-rules" => options.no_builtin = true,
            "--min-confidence" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(c) = val.parse::<f32>() {
                        options.min_confidence = c;
                    }
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
                println!("GenSense: Analysis Results for {}\n", input_path.display());
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

    let mut filtered_advisories = engine.run(&input_path)?;
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

    if let Some(pos) = args.iter().position(|a| a == "--debug") {
        if let Some(file_path) = args.get(pos + 1) {
            if let Err(e) = handle_debug_ast(file_path) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
            return true;
        }
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
            eprintln!("Usage: gensense <path> [options]");
            eprintln!();
            eprintln!("Run 'gensense --help' for more information");
            std::process::exit(1);
        }
    };
    let input_path_buf = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(&input_path_str);
    if input_path_buf.exists() {
        input_path_buf.canonicalize().unwrap_or(input_path_buf)
    } else {
        input_path_buf
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

    if let Some(filter) = options.severity_filter {
        advisories.retain(|a| a.severity == filter);
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
        if options.show_diff {
            if let Ok(diff) = patcher.generate_diff(adv, Path::new(&adv.file_path)) {
                println!("{diff}");
            }
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
