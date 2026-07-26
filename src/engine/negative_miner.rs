// SPDX-License-Identifier: MIT

//! Structural negative mining from scan output.
//!
//! When `--mine-negatives` is enabled, findings with confidence in the
//! "grey zone" (below 0.45 but above the reporting threshold) are treated
//! as probable false positives. Their function source is extracted and
//! written to a `mined_negatives/` directory as candidate negative examples.
//!
//! A human reviews these files and promotes promising candidates to
//! `corpus/targets/{pattern}_negative{N}.ts` for the next bundle build.
//! This closes the feedback loop between real-world scans and corpus quality.

use std::fs;
use std::path::{Path, PathBuf};

use crate::Advisory;

/// Extract the function source from the original file around the finding's line.
/// Returns a best-effort snippet — may be truncated or empty if the file is
/// unreadable or the line is out of range.
fn extract_function_snippet(file_path: &Path, line: u32) -> String {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    let line = line as usize;
    if line == 0 || line > content.lines().count() {
        return String::new();
    }

    // Extract a window around the finding: 5 lines before to 5 lines after
    let start = line.saturating_sub(5);
    let end = (line + 5).min(content.lines().count());

    let mut snippet = String::new();
    for (i, line_text) in content.lines().enumerate() {
        let line_num = i + 1;
        if line_num >= start && line_num <= end {
            if line_num == line {
                snippet.push_str(&format!("// >>> FINDING at line {}\n", line));
            }
            snippet.push_str(line_text);
            snippet.push('\n');
        }
    }
    snippet
}

/// Write a mined negative candidate to disk.
///
/// Creates `mined_negatives/{pattern_id}/{timestamp}_{line}.{ext}` containing
/// the original source snippet around the finding. The file extension matches
/// the scanned source file.
fn write_mined_negative(advisory: &Advisory, output_dir: &Path) -> Result<PathBuf, String> {
    let pattern_dir = output_dir.join(sanitize_filename(&advisory.rule_id));
    fs::create_dir_all(&pattern_dir).map_err(|e| e.to_string())?;

    let source_path = Path::new(&advisory.file_path);
    let ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let filename = format!(
        "{}_{}_{}.{}",
        timestamp,
        advisory.line,
        sanitize_filename(&advisory.rule_id),
        ext
    );
    let out_path = pattern_dir.join(&filename);

    let snippet = extract_function_snippet(source_path, advisory.line);

    let mut content = String::new();
    content.push_str(&format!(
        "// Mined negative candidate for {}\n",
        advisory.rule_id
    ));
    content.push_str(&format!(
        "// Source: {}:{}\n",
        advisory.file_path, advisory.line
    ));
    content.push_str(&format!("// Confidence: {:.3}\n", advisory.confidence));
    content.push_str("// Review and promote to corpus/targets/ as _negative{N}.{ext}\n");
    content.push_str("// === Source snippet ===\n");
    content.push_str(&snippet);

    fs::write(&out_path, &content).map_err(|e| e.to_string())?;

    Ok(out_path)
}

/// Sanitize a string for use as a filename.
fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Mine negative candidates from a set of advisories.
///
/// Any finding with confidence between `min_confidence` and 0.45 is treated
/// as a probable false positive and its function source is extracted and
/// written to the output directory as a candidate negative example.
pub fn mine_negatives(
    advisories: &[Advisory],
    output_dir: &Path,
    min_confidence: f64,
) -> Result<usize, String> {
    let mut count = 0;

    for adv in advisories {
        // Grey zone: below 0.45 but above the minimum reporting threshold
        if adv.confidence >= 0.45 || adv.confidence < min_confidence {
            continue;
        }

        // Skip findings without a valid file path
        if adv.file_path.is_empty() || adv.file_path == "." {
            continue;
        }

        match write_mined_negative(adv, output_dir) {
            Ok(path) => {
                eprintln!(
                    "Mined negative candidate: {} -> {}",
                    path.display(),
                    adv.rule_id
                );
                count += 1;
            }
            Err(e) => {
                eprintln!("Error mining negative for {}: {}", adv.rule_id, e);
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("hello"), "hello");
        assert_eq!(sanitize_filename("hello/world:test"), "hello_world_test");
        assert_eq!(sanitize_filename("a/b/c"), "a_b_c");
    }

    #[test]
    fn test_extract_function_snippet_nonexistent_file() {
        let snippet = extract_function_snippet(Path::new("/nonexistent/file.rs"), 1);
        assert!(snippet.is_empty());
    }
}
