use super::options::CliOptions;
use crate::reporter::Reporter;
use crate::{Advisory, Engine, FrensenseError, Result};
use std::collections::HashSet;
use std::path::Path;

pub fn print_results(
    filtered_advisories: &[Advisory],
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
                    .map_err(|e| FrensenseError::Config(format!("JSON error: {e}")))?
            );
        }
        "sarif" => {
            let sarif = Reporter::to_sarif(filtered_advisories, input_path);
            println!(
                "{}",
                serde_json::to_string_pretty(&sarif)
                    .map_err(|e| FrensenseError::Config(format!("JSON error: {e}")))?
            );
        }
        _ => {
            if filtered_advisories.is_empty() {
                println!("Analysis Complete: Looking great! No structural concerns found.");
            } else {
                println!("╔══════════════════════════════════════════════════╗");
                println!(
                    "║  Frensense v{}                              ║",
                    crate::FRENSENSE_VERSION
                );
                println!("║  Semantic Code Analysis Engine                ║");
                println!("╚══════════════════════════════════════════════════╝");
                println!("Analysis: {}", input_path.display());
                println!();
                for v in filtered_advisories {
                    let severity_label = match v.severity {
                        crate::Severity::Critical => "[CRITICAL]",
                        crate::Severity::Warning => "[WARNING]",
                        crate::Severity::Info => "[INFO]",
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

pub fn compare_baseline(filtered_advisories: &[Advisory], path: &str) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| FrensenseError::Config(format!("Failed to read baseline: {e}")))?;
    let baseline: Vec<Advisory> = serde_json::from_str(&content)
        .or_else(|_| serde_yaml::from_str(&content))
        .map_err(|e| FrensenseError::Config(format!("Failed to parse baseline: {e}")))?;

    let baseline_fuzzy: HashSet<_> = baseline.iter().map(Advisory::fuzzy_identity).collect();
    let current_fuzzy: HashSet<_> = filtered_advisories
        .iter()
        .map(Advisory::fuzzy_identity)
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

pub fn save_baseline(advisories: &[Advisory], path: &str) -> Result<()> {
    let content = serde_json::to_string_pretty(advisories)
        .map_err(|e| FrensenseError::Config(format!("JSON error: {e}")))?;
    std::fs::write(path, content)
        .map_err(|e| FrensenseError::Config(format!("Failed to write baseline: {e}")))?;
    println!("[SUCCESS] Captured baseline to {path}");
    Ok(())
}

pub fn apply_filters(advisories: &mut Vec<Advisory>, options: &CliOptions, engine: &Engine) {
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
        advisories.retain(|a| a.severity.meets_threshold(filter));
    }
}
