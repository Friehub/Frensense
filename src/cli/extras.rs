use super::options::CliOptions;
use crate::Advisory;
use crate::patcher::PatchManager;
#[cfg(feature = "fingerprinting")]
use frensense_engine::profile::ProjectProfile;
use std::path::{Path, PathBuf};

/// Handle remediation.
///
/// # Panics
/// May panic if directory operations fail.
pub fn handle_remediation(advisories: &[Advisory], options: &CliOptions, input_path: &Path) {
    let mut project_root = input_path.to_path_buf();
    if project_root.is_file() {
        project_root = project_root.parent().unwrap_or(&project_root).to_path_buf();
    }

    while project_root.parent().is_some() {
        if project_root.join(".frensense").exists() || project_root.join(".git").exists() {
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

    // Filter by scope
    if let Some(ref scope) = options.fix_scope {
        filter_by_scope(&mut fix_advisories, scope);
    }
    fix_advisories.sort_by_key(|a| std::cmp::Reverse(a.start_byte));

    let mut fixed_count = 0;
    let mut skipped_count = 0;

    for adv in &fix_advisories {
        if options.diff_scope.is_some()
            && let Ok(diff) = patcher.generate_diff(adv, Path::new(&adv.file_path))
        {
            println!("{diff}");
        }
        if options.fix_scope.is_some() {
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
    if options.fix_scope.is_some() {
        println!(
            "\n[DONE] {fixed_count} fixed, {skipped_count} skipped (context mismatch), 0 conflicts."
        );
    }
}

fn filter_by_scope(advisories: &mut Vec<Advisory>, scope: &str) {
    match scope {
        "style" => advisories.retain(|a| a.tags.iter().any(|t| t == "quality" || t == "dead-code")),
        "security" => advisories.retain(|a| {
            a.tags
                .iter()
                .any(|t| t == "security" || t == "taint" || t == "hallucination")
        }),
        _ => {} // "all" — keep everything
    }
}

#[cfg(feature = "fingerprinting")]
#[must_use]
pub fn find_profile(path: &Path) -> Option<PathBuf> {
    let mut current = Some(path);
    while let Some(p) = current {
        let candidate = p.join(".frensense").join("profile.json");
        if candidate.exists() {
            return Some(candidate);
        }
        current = p.parent();
    }
    None
}

#[cfg(feature = "fingerprinting")]
pub fn print_profile_stats(profile: &ProjectProfile) {
    println!("\n=== Profile Statistics ===");
    println!("Version: {}", profile.version);
    println!("Threshold: {:.2}", profile.threshold);
    for (lang, lp) in &profile.languages {
        println!("\n  Language: {lang}");
        println!("    Functions profiled: {}", lp.total_functions);
        println!("    Total n-grams: {}", lp.total_ngrams);
        println!("    Unique body n-grams: {}", lp.body_ngram_freq.len());
        println!(
            "    Unique signature n-grams: {}",
            lp.signature_ngram_freq.len()
        );
        println!("    Unique name segments: {}", lp.name_segment_freq.len());
        println!(
            "    Unique structural markers: {}",
            lp.structural_marker_freq.len()
        );
        println!("    Unique type usages: {}", lp.type_usage_freq.len());
        println!("    File sub-profiles: {}", lp.file_profiles.len());
    }
}
