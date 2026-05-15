// SPDX-License-Identifier: MIT

use gensense::{Engine, GenSenseAuditor, Result};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    // nosemgrep: rust.lang.security.args.args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("GenSense - Semantic Insight for Modern Codebases");
        println!("Version: {}", gensense::GENSENSE_VERSION);
        println!("Usage: gensense <path> [options]");
        println!("\nOptions:");
        println!("  --version          Display version and features");
        println!("  --list-rules       Display the active rules catalog");
        println!("  --generate-docs    Generate RULES.md documentation");
        println!("  --debug <file>     Anonymized AST debug dump");
        println!("  --severity <level> Filter findings by severity (critical, warning, info)");
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

        std::process::exit(0);
    }

    if args.contains(&"--version".to_string()) {
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
        std::process::exit(0);
    }

    if args.len() > 1 && args[1] == "test-rule" {
        if args.len() < 7 {
            eprintln!("Usage: gensense test-rule <rule.yml> --fixture <file> --expect-finding <id> [--expect-line <N>]");
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

        let rule_content =
            std::fs::read_to_string(&rule_file_path).expect("Failed to read rule file");

        #[derive(serde::Deserialize)]
        struct RulesWrapper {
            rules: Vec<gensense::rules::core::CoreRule>,
        }

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

        let mut auditor = GenSenseAuditor::default_auditor();
        auditor.rules = rules;
        let mut engine = Engine::new(auditor);
        engine.isolate_rules = true;

        let advisories = engine
            .run(Path::new(&fixture_path))
            .expect("Analysis failed");

        if let Some(finding) = advisories.iter().find(|a| a.rule_id == expected_id) {
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
        } else {
            println!("[FAIL: Rule not triggered] Expected to find rule {expected_id}");
            std::process::exit(1);
        }
    }

    if args.contains(&"--generate-docs".to_string()) {
        let engine = Engine::new(GenSenseAuditor::default_auditor());
        let _ = engine.list_rules();
        let mut doc = String::new();
        doc.push_str("# GenSense Rule Catalog\n\n");
        doc.push_str(
            "This catalog lists all semantic rules currently active in the GenSense engine.\n\n",
        );
        doc.push_str("| Rule ID | Severity | Category | Description |\n");
        doc.push_str("| :--- | :--- | :--- | :--- |\n");
        for rule in engine.auditor.rules() {
            let meta = rule.metadata();
            doc.push_str(&format!(
                "| `{}` | {:?} | {} | {} |\n",
                rule.id(),
                meta.severity,
                meta.category,
                meta.impact
            ));
        }
        std::fs::write("RULES.md", doc).expect("Failed to write RULES.md");
        println!("[SUCCESS] Generated RULES.md");
        std::process::exit(0);
    }

    if let Some(pos) = args.iter().position(|a| a == "--debug") {
        if let Some(file_path) = args.get(pos + 1) {
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
    }

    if args.contains(&"--list-rules".to_string()) {
        let engine = Engine::new(GenSenseAuditor::default_auditor());
        let rules = engine.list_rules();
        println!("GenSense: Active Rules Catalog");
        println!("{:-<100}", "");
        println!("{:<30} | {:<10} | Description", "Rule ID", "Severity");
        println!("{:-<100}", "");
        for (id, desc, sev) in rules {
            println!("{id:<30} | {sev:<10?} | {desc}");
        }
        std::process::exit(0);
    }

    // Validate that we have a path argument before proceeding
    let input_path_str = match args.get(1) {
        Some(path) if !path.starts_with("--") => path.clone(),
        _ => {
            eprintln!("Usage: gensense <path> [options]");
            eprintln!();
            eprintln!("Run 'gensense --help' for more information");
            std::process::exit(1);
        }
    };
    let input_path = std::env::current_dir().unwrap().join(&input_path_str);
    let input_path = if input_path.exists() {
        input_path.canonicalize().unwrap_or(input_path)
    } else {
        input_path
    };
    let input_path = &input_path;

    let mut format = "text".to_string();
    let mut is_strict = false;
    let mut do_fix = false;
    let mut show_diff = false;
    let mut severity_filter: Option<gensense::Severity> = None;
    let mut enabled_tags = Vec::new();
    let mut extra_rule_dirs = Vec::new();
    let mut no_builtin = false;
    let mut emit_baseline_path: Option<String> = None;
    let mut compare_baseline_path: Option<String> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => format = "json".to_string(),
            "--sarif" => format = "sarif".to_string(),
            "--strict" => is_strict = true,
            #[cfg(feature = "remediation")]
            "--fix" => do_fix = true,
            #[cfg(feature = "remediation")]
            "--diff" => show_diff = true,
            "--no-builtin-rules" => no_builtin = true,
            "--rules-dir" => {
                if let Some(dir) = args.get(i + 1) {
                    extra_rule_dirs.push(std::path::PathBuf::from(dir));
                    i += 1;
                }
            }
            "--severity" => {
                if let Some(level) = args.get(i + 1) {
                    severity_filter = match level.to_lowercase().as_str() {
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
                    enabled_tags.push(tag.clone());
                    i += 1;
                }
            }
            "--emit-baseline" => {
                if let Some(path) = args.get(i + 1) {
                    emit_baseline_path = Some(path.clone());
                    i += 1;
                }
            }
            "--compare-baseline" => {
                if let Some(path) = args.get(i + 1) {
                    compare_baseline_path = Some(path.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
    engine.extra_rule_dirs = extra_rule_dirs;
    if no_builtin {
        engine.no_builtin_rules = true;
    }

    for tag in enabled_tags {
        engine.enable_tag(&tag);
    }
    let all_advisories = engine.run(input_path)?;

    // Filter by severity
    let mut filtered_advisories = all_advisories;
    if let Some(filter) = severity_filter {
        filtered_advisories.retain(|a| a.severity == filter);
    }

    match format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&filtered_advisories)
                    .map_err(|e| gensense::GenSenseError::Config(format!("JSON error: {e}")))?
            );
        }
        "sarif" => {
            let sarif = gensense::reporter::Reporter::to_sarif(&filtered_advisories, input_path);
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
                for v in &filtered_advisories {
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

    if let Some(path) = emit_baseline_path {
        let content = serde_json::to_string_pretty(&filtered_advisories)
            .map_err(|e| gensense::GenSenseError::Config(format!("JSON error: {e}")))?;
        std::fs::write(&path, content).map_err(|e| {
            gensense::GenSenseError::Config(format!("Failed to write baseline: {e}"))
        })?;
        println!("[SUCCESS] Captured baseline to {path}");
    }

    let mut regression_detected = false;
    if let Some(path) = &compare_baseline_path {
        let content = std::fs::read_to_string(path).map_err(|e| {
            gensense::GenSenseError::Config(format!("Failed to read baseline: {e}"))
        })?;
        let baseline: Vec<gensense::Advisory> = serde_json::from_str(&content)
            .or_else(|_| serde_yaml::from_str(&content))
            .map_err(|e| {
                gensense::GenSenseError::Config(format!("Failed to parse baseline: {e}"))
            })?;

        let baseline_fuzzy: std::collections::HashSet<_> =
            baseline.iter().map(|a| a.fuzzy_identity()).collect();
        let current_fuzzy: std::collections::HashSet<_> = filtered_advisories
            .iter()
            .map(|a| a.fuzzy_identity())
            .collect();

        let new_advisories: Vec<_> = filtered_advisories
            .iter()
            .filter(|a| !baseline_fuzzy.contains(&a.fuzzy_identity()))
            .collect();
        let resolved_advisories: Vec<_> = baseline
            .iter()
            .filter(|a| !current_fuzzy.contains(&a.fuzzy_identity()))
            .collect();

        if !new_advisories.is_empty() {
            println!(
                "\n[REGRESSION] {} new advisories detected!",
                new_advisories.len()
            );
            for adv in &new_advisories {
                println!("  + {}:{} ({})", adv.file_path, adv.line, adv.rule_id);
            }
            regression_detected = true;
        } else {
            println!("\n[OK] No new advisories compared to baseline.");
        }

        if !resolved_advisories.is_empty() {
            println!(
                "[OK] {} advisories resolved since baseline.",
                resolved_advisories.len()
            );
        }

        let net = (new_advisories.len() as i32) - (resolved_advisories.len() as i32);
        println!(
            "[NET] {} ({} total findings)\n",
            if net > 0 {
                format!("+{net}")
            } else {
                net.to_string()
            },
            filtered_advisories.len()
        );
    }

    #[cfg(feature = "remediation")]
    if (do_fix || show_diff) && !filtered_advisories.is_empty() {
        use gensense::patcher::PatchManager;

        // Find project root (where .gensense or .git exists)
        let mut project_root = input_path.to_path_buf();
        if project_root.is_file() {
            project_root = project_root.parent().unwrap_or(&project_root).to_path_buf();
        }

        while project_root.parent().is_some() {
            if project_root.join(".gensense").exists() || project_root.join(".git").exists() {
                break;
            }
            project_root = project_root.parent().unwrap().to_path_buf();
        }

        let patcher = PatchManager::new(&project_root);

        let mut fix_advisories = filtered_advisories.clone();
        fix_advisories.retain(|a| a.proposed_replacement.is_some());
        // Sort DESC by start_byte to avoid offset drift within a file
        fix_advisories.sort_by_key(|a| std::cmp::Reverse(a.start_byte));

        let mut fixed_count = 0;
        let mut skipped_count = 0;

        for adv in &fix_advisories {
            if show_diff {
                if let Ok(diff) = patcher.generate_diff(adv, Path::new(&adv.file_path)) {
                    println!("{diff}");
                }
            }
            if do_fix {
                match patcher.apply_fix(adv, Path::new(&adv.file_path)) {
                    Ok(_) => {
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
        if do_fix {
            println!(
                "\n[DONE] {fixed_count} fixed, {skipped_count} skipped (context mismatch), 0 conflicts."
            );
        }
    }

    if regression_detected || (is_strict && !filtered_advisories.is_empty()) {
        std::process::exit(1);
    }

    Ok(())
}
