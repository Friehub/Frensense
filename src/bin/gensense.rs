// SPDX-License-Identifier: MIT

use gensense::{Engine, GenSenseAuditor, Result};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
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
            doc.push_str(&format!(
                "| `{}` | {:?} | {} | {} |\n",
                rule.id(),
                rule.severity(),
                rule.category(),
                rule.description()
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

    let input_path_str = args.get(1).unwrap();
    let input_path = Path::new(input_path_str);

    let mut format = "text".to_string();
    let mut is_strict = false;
    let mut do_fix = false;
    let mut show_diff = false;
    let mut severity_filter: Option<gensense::Severity> = None;
    let mut enabled_tags = Vec::new();

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
            _ => {}
        }
        i += 1;
    }

    let mut engine = Engine::new(GenSenseAuditor::default_auditor());
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
                serde_json::to_string_pretty(&filtered_advisories).unwrap()
            );
        }
        "sarif" => {
            let sarif = gensense::reporter::Reporter::to_sarif(&filtered_advisories, input_path);
            println!("{}", serde_json::to_string_pretty(&sarif).unwrap());
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

    #[cfg(feature = "remediation")]
    if (do_fix || show_diff) && !filtered_advisories.is_empty() {
        use gensense::patcher::PatchManager;
        let patcher = PatchManager::new(input_path);
        for adv in &filtered_advisories {
            if adv.proposed_replacement.is_some() {
                if show_diff {
                    if let Ok(diff) = patcher.generate_diff(adv, Path::new(&adv.file_path)) {
                        println!("{diff}");
                    }
                }
                if do_fix && patcher.apply_fix(adv, Path::new(&adv.file_path)).is_ok() {
                    println!("[FIXED] {}", adv.file_path);
                }
            }
        }
    }

    if is_strict && !filtered_advisories.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
