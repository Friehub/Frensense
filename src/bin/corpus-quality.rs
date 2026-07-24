// SPDX-License-Identifier: MIT
//! Corpus quality scoring tool.
//!
//! Scores each positive/negative pair on 0-100 based on structural quality
//! heuristics. Outputs TSV sorted by score ascending — lowest first.
//!
//! Usage: cargo run --bin corpus-quality -- <corpus_dir>

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let corpus_dir = args.get(1).cloned().unwrap_or_else(|| {
        eprintln!("Usage: corpus-quality <corpus_dir>");
        std::process::exit(1);
    });
    let dir = std::path::Path::new(&corpus_dir);

    let files = collect_files(dir);
    let mut scores: Vec<(u32, String, Vec<String>)> = Vec::new();

    // Group by pattern ID
    use std::collections::HashMap;
    let mut by_pattern: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for path in &files {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let pattern = name
            .strip_suffix("_positive")
            .or_else(|| name.strip_suffix("_negative")
                .or_else(|| {
                    for suffix in &["_negative2", "_negative3", "_negative4"] {
                        if let Some(p) = name.strip_suffix(suffix) {
                            return Some(p);
                        }
                    }
                    None
                }))
            .map(|p| p.to_string())
            .unwrap_or_default();
        if !pattern.is_empty() {
            by_pattern.entry(pattern).or_default().push(path.clone());
        }
    }

    for (pattern, files) in &by_pattern {
        let pos = files.iter().find(|p| {
            let name = p.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            name.contains("_positive")
        });
        let negs: Vec<&PathBuf> = files.iter().filter(|p| {
            let name = p.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            name.contains("_negative")
        }).collect();

        let Some(pos_path) = pos else { continue; };
        let src = match std::fs::read_to_string(pos_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut score: u32 = 0;
        let mut checks: Vec<String> = Vec::new();
        let lower = src.to_lowercase();

        // +20 Has [frensense] block with all 3 fields
        let has_obs = src.contains("observation:");
        let has_imp = src.contains("impact:");
        let has_impv = src.contains("improvement:");
        if has_obs && has_imp && has_impv {
            score += 20;
        } else {
            checks.push("no_full_frensense".to_string());
        }

        // +15 Has at least one import statement
        if lower.contains("import ") || lower.contains("require(") {
            score += 15;
        } else {
            checks.push("no_import".to_string());
        }

        // +15 Has 2+ functions
        let fn_count = src.matches("function ").count()
            + src.matches("=>").count() / 2; // rough estimate
        if fn_count >= 2 {
            score += 15;
        } else {
            checks.push("under_2_functions".to_string());
        }

        // +15 Typed HTTP handler parameter
        if lower.contains("express.request")
            || lower.contains("express.response")
            || lower.contains(": request")
            || lower.contains(": response")
            || lower.contains(": context")
            || lower.contains("req: request")
            || lower.contains("res: response")
        {
            score += 15;
        } else {
            checks.push("untyped_handler".to_string());
        }

        // +10 Taint source is explicit
        if lower.contains("req.body")
            || lower.contains("req.query")
            || lower.contains("req.params")
            || lower.contains("req.headers")
            || lower.contains("req.cookies")
            || lower.contains("c.req")
            || lower.contains("ctx.request")
        {
            score += 10;
        } else {
            checks.push("no_taint_source".to_string());
        }

        // +10 Has CWE in [frensense] block
        if lower.contains("cwe:") {
            score += 10;
        } else {
            checks.push("no_cwe".to_string());
        }

        // +10 Negative uses same sink call safely
        if let Some(neg_path) = negs.first() {
            if let Ok(neg_src) = std::fs::read_to_string(neg_path) {
                // Check if negative imports the same modules (just the fix variant)
                let pos_imports: Vec<&str> = src.lines()
                    .filter(|l| l.trim().starts_with("import ") || l.trim().starts_with("const "))
                    .collect();
                let neg_imports: Vec<&str> = neg_src.lines()
                    .filter(|l| l.trim().starts_with("import ") || l.trim().starts_with("const "))
                    .collect();
                if !pos_imports.is_empty() && !neg_imports.is_empty() {
                    score += 10;
                } else {
                    checks.push("neg_no_same_imports".to_string());
                }
            } else {
                checks.push("neg_unreadable".to_string());
            }
        } else {
            checks.push("no_negative".to_string());
        }

        // -20 File under 10 lines
        let line_count = src.lines().count();
        if line_count < 10 {
            score = score.saturating_sub(20);
            checks.push("under_10_lines".to_string());
        }

        // -20 Placeholder names
        for placeholder in &["foo", "bar", "test", "dostuff", "dothing", "dosth"] {
            if lower.contains(placeholder) {
                score = score.saturating_sub(20);
                checks.push("placeholder_names".to_string());
                break;
            }
        }

        // -10 req typed as any throughout
        let any_count = src.matches(": any").count();
        if any_count >= 3 {
            score = score.saturating_sub(10);
            checks.push("excessive_any".to_string());
        }

        // -10 Only one function
        if fn_count < 2 {
            score = score.saturating_sub(10);
        }

        scores.push((score, pattern.clone(), checks));
    }

    // Sort by score ascending
    scores.sort_by_key(|(s, _, _)| *s);

    // Output TSV
    println!("score\tpattern_id\tfailing_checks");
    for (score, pattern, checks) in &scores {
        println!("{}\t{}\t{}", score, pattern, checks.join(","));
    }

    // Summary
    let total = scores.len();
    let below_50 = scores.iter().filter(|(s, _, _)| *s < 50).count();
    let above_80 = scores.iter().filter(|(s, _, _)| *s >= 80).count();
    eprintln!("\nScored {total} patterns: {below_50} below 50, {above_80} above 80");
}

fn collect_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    collect_recursive(dir, &mut result);
    result
}

fn collect_recursive(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return; };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_recursive(&path, out);
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if matches!(ext, "ts" | "tsx" | "js" | "jsx" | "rs") {
                    out.push(path);
                }
            }
        }
    }
}
