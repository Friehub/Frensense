// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use std::env;
use std::path::Path;
use taas_auditor::{AstAuditor, Engine, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("TaaS Auditor - Industrial Strength Static Analysis");
        println!("Usage: taas-auditor <path> [options]");
        println!("\nOptions:");
        println!("  --list-rules       Display the active rules catalog");
        println!("  --severity <level> Filter findings by severity (critical, warning, info)");
        println!("  --strict           Exit with code 1 if any findings match filter");
        println!("  --json             Output findings as JSON");
        println!("  --sarif            Output findings in SARIF format");
        println!("  --fix              Apply automated remediation (experimental)");
        println!("  --diff             Show unified diff of proposed changes");
        std::process::exit(0);
    }

    if args.contains(&"--list-rules".to_string()) {
        let engine = Engine::new(AstAuditor::default_auditor());
        let rules = engine.list_rules();
        println!("TaaS Auditor: Active Rules Catalog");
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
    let mut severity_filter: Option<taas_auditor::Severity> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => format = "json".to_string(),
            "--sarif" => format = "sarif".to_string(),
            "--strict" => is_strict = true,
            "--fix" => do_fix = true,
            "--diff" => show_diff = true,
            "--severity" => {
                if let Some(level) = args.get(i + 1) {
                    severity_filter = match level.to_lowercase().as_str() {
                        "critical" => Some(taas_auditor::Severity::Critical),
                        "warning" => Some(taas_auditor::Severity::Warning),
                        "info" => Some(taas_auditor::Severity::Info),
                        _ => {
                            eprintln!("Error: Unknown severity level '{level}'");
                            std::process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let mut engine = Engine::new(AstAuditor::default_auditor());
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
            let sarif =
                taas_auditor::reporter::Reporter::to_sarif(&filtered_advisories, input_path);
            println!("{}", serde_json::to_string_pretty(&sarif).unwrap());
        }
        _ => {
            if filtered_advisories.is_empty() {
                println!("Analysis Complete: Looking great! No structural concerns found.");
            } else {
                println!(
                    "TaaS Auditor: Analysis Results for {}\n",
                    input_path.display()
                );
                for v in &filtered_advisories {
                    let severity_label = match v.severity {
                        taas_auditor::Severity::Critical => "[CRITICAL]",
                        taas_auditor::Severity::Warning => "[WARNING]",
                        taas_auditor::Severity::Info => "[INFO]",
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

    if do_fix || show_diff {
        // ... (existing fix/diff logic if any, currently mostly in Engine::run)
    }

    if is_strict && !filtered_advisories.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
