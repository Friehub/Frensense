// SPDX-License-Identifier: MIT
#![warn(clippy::unwrap_used)]

use gensense::cli::{
    apply_filters, compare_baseline, find_profile, get_input_path, handle_early_args,
    handle_remediation, parse_options, print_profile_stats, print_results, save_baseline,
};
use gensense::parser::ParserRegistry;
use gensense::{Engine, Result};
use std::env;
use std::path::PathBuf;

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

    #[cfg(feature = "fingerprinting")]
    if options.learn_profile {
        let mut all_fingerprints = Vec::new();
        let files = Engine::collect_files(&input_path, None);
        let wsize = options.ngram_window_size.unwrap_or(5);
        for p in &files {
            if let Ok(content) = std::fs::read_to_string(p) {
                let mut fps = Vec::new();
                if let Ok((_language, tree)) = engine.auditor().parse_source(p, &content) {
                    gensense::engine::fingerprint::extract_fingerprints(
                        tree.root_node(),
                        &content,
                        p,
                        &mut fps,
                        wsize,
                    );
                    all_fingerprints.extend(fps);
                }
            }
        }
        let profile = gensense::engine::profile::ProjectProfile::learn(&all_fingerprints);
        let profile_dir = if input_path.is_file() {
            input_path.parent().unwrap_or(&input_path)
        } else {
            &input_path
        };
        let profile_path = profile_dir.join(".gensense").join("profile.json");
        profile.save(&profile_path)?;
        println!(
            "[OK] Profile learned and saved to {}",
            profile_path.display()
        );
        if options.profile_stats {
            print_profile_stats(&profile);
        }
        return Ok(());
    }

    #[cfg(feature = "fingerprinting")]
    if options.check_profile {
        let profile_path = find_profile(&input_path)
            .unwrap_or_else(|| input_path.join(".gensense").join("profile.json"));
        if !profile_path.exists() {
            eprintln!(
                "Error: No profile found at {}. Run `gensense --learn-profile` first.",
                profile_path.display()
            );
            std::process::exit(1);
        }
        let profile = gensense::engine::profile::ProjectProfile::load(&profile_path)
            .map_err(|e| gensense::GenSenseError::Config(format!("Failed to load profile: {e}")))?;
        if let Some(threshold) = options.profile_threshold {
            engine.set_profile_threshold(threshold);
        }
        engine = engine.with_profile(profile);
        if options.profile_stats {
            if let Some(profile) = engine.profile() {
                print_profile_stats(profile);
            }
        }
    }

    #[cfg(feature = "fingerprinting")]
    if options.profile_stats && !options.learn_profile && !options.check_profile {
        let profile_path = find_profile(&input_path)
            .unwrap_or_else(|| input_path.join(".gensense").join("profile.json"));
        if profile_path.exists() {
            let profile =
                gensense::engine::profile::ProjectProfile::load(&profile_path).map_err(|e| {
                    gensense::GenSenseError::Config(format!("Failed to load profile: {e}"))
                })?;
            print_profile_stats(&profile);
        } else {
            eprintln!("No profile found at {}.", profile_path.display());
        }
        return Ok(());
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
