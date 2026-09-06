// SPDX-License-Identifier: MIT

use crate::{Advisory, FrensenseError, Result};
use diff;
use pathdiff;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

pub struct PatchManager {
    root_dir: PathBuf,
}

impl PatchManager {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    /// Resolves a logical import path (potentially containing {{root}})
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// to a relative path from the current file.
    fn resolve_import_path(&self, from_file: &Path, import_stmt: &str) -> String {
        if !import_stmt.contains("{{root}}") {
            return import_stmt.to_string();
        }

        let mut final_stmt = import_stmt.to_string();

        // Extract the path between quotes
        if let Some(start_quote) = import_stmt.find('\'').or_else(|| import_stmt.find('\"'))
            && let Some(end_quote) = import_stmt.rfind('\'').or_else(|| import_stmt.rfind('\"'))
            && start_quote < end_quote
        {
            let logical_path = &import_stmt[start_quote + 1..end_quote];
            if logical_path.contains("{{root}}") {
                let resolved_root =
                    logical_path.replace("{{root}}", self.root_dir.to_str().unwrap_or(""));
                let target = PathBuf::from(&resolved_root);

                let absolute_from = if from_file.is_absolute() {
                    from_file.to_path_buf()
                } else {
                    self.root_dir.join(from_file)
                };

                let from_dir = absolute_from.parent().unwrap_or(&absolute_from);

                if let Some(rel_path) = pathdiff::diff_paths(&target, from_dir) {
                    let mut s = rel_path.to_string_lossy().to_string();
                    if !s.starts_with('.') {
                        s = format!("./{s}");
                    }
                    final_stmt = format!(
                        "{}{}{}",
                        &import_stmt[..=start_quote],
                        s,
                        &import_stmt[end_quote..]
                    );
                }
            }
        }
        final_stmt
    }

    /// Generates a unified diff for an advisory's proposed replacement.
    /// Generates a unified diff for an advisory's proposed replacement.
    ///
    /// # Errors
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Returns an error if the diff generation fails.
    pub fn generate_diff(&self, advisory: &Advisory, file_path: &Path) -> Result<String> {
        let Some(proposed) = &advisory.proposed_replacement else {
            return Ok(String::new());
        };

        let mut diff_output = String::new();
        let _ = writeln!(diff_output, "--- a/{}", file_path.display());
        let _ = writeln!(diff_output, "+++ b/{}", file_path.display());
        let _ = writeln!(
            diff_output,
            "@@ -{},{} +{},{} @@",
            advisory.line, 1, advisory.line, 1
        );

        let result = diff::lines(&advisory.original_content, proposed);
        for line in result {
            match line {
                diff::Result::Left(l) => {
                    let _ = writeln!(diff_output, "-{l}");
                }
                diff::Result::Both(l, _) => {
                    let _ = writeln!(diff_output, " {l}");
                }
                diff::Result::Right(r) => {
                    let _ = writeln!(diff_output, "+{r}");
                }
            }
        }

        Ok(diff_output)
    }

    /// Applies all advisories to a file atomically using Shadow Writing.
    ///
    /// # Errors
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Returns an error if file reading, patching, or renaming fails.
    pub fn apply_fixes(&self, advisories: &[&Advisory], file_path: &Path) -> Result<()> {
        if advisories.is_empty() {
            return Ok(());
        }

        let absolute_path = self.root_dir.join(file_path);
        let content = fs::read_to_string(&absolute_path).map_err(|e| {
            FrensenseError::Config(format!("Failed to read file for patching: {e}"))
        })?;

        // Sort advisories back-to-front by start_byte to ensure offset stability
        let mut sorted_advisories = advisories.to_vec();
        sorted_advisories.sort_by_key(|b| std::cmp::Reverse(b.start_byte));

        let mut updated_content = content.clone();
        let import_re = regex::Regex::new(r"(?m)^import\s+.*").ok();

        for advisory in sorted_advisories {
            let Some(proposed) = &advisory.proposed_replacement else {
                continue;
            };

            let start = advisory.start_byte as usize;
            let end = advisory.end_byte as usize;

            if start > updated_content.len()
                || end > updated_content.len()
                || updated_content[start..end] != advisory.original_content
            {
                return Err(FrensenseError::Config(format!(
                    "Patch failed for {}: Context mismatch at byte {}. Expected '{}', found '{}'",
                    file_path.display(),
                    start,
                    advisory.original_content,
                    if start < updated_content.len() {
                        &updated_content[start..std::cmp::min(end, updated_content.len())]
                    } else {
                        "EOF"
                    }
                )));
            }

            // 1. Create updated content in memory using precise range replacement.
            let mut new_content = String::with_capacity(updated_content.len() + proposed.len());
            new_content.push_str(&updated_content[..start]);
            new_content.push_str(proposed);
            new_content.push_str(&updated_content[end..]);

            updated_content = new_content;

            // 2. Import Injection
            if let Some(import_template) = &advisory.proposed_import {
                let import_stmt = self.resolve_import_path(file_path, import_template);
                if !updated_content.contains(&import_stmt) {
                    let mut insertion_offset = 0;
                    if let Some(re) = &import_re
                        && let Some(last_match) = re.find_iter(&updated_content).last()
                        && let Some(line_end) = updated_content[last_match.end()..].find('\n')
                    {
                        insertion_offset = last_match.end() + line_end + 1;
                    }

                    let mut final_content =
                        String::with_capacity(updated_content.len() + import_stmt.len() + 2);
                    final_content.push_str(&updated_content[..insertion_offset]);
                    final_content.push_str(&import_stmt);
                    final_content.push('\n');

                    if insertion_offset == 0
                        && !updated_content.is_empty()
                        && !updated_content.starts_with('\n')
                    {
                        final_content.push('\n');
                    }

                    final_content.push_str(&updated_content[insertion_offset..]);
                    updated_content = final_content;
                }
            }
        }

        let tmp_path = absolute_path.with_extension("patch_tmp");
        fs::write(&tmp_path, updated_content).map_err(|e| {
            FrensenseError::Config(format!("Failed to write temporary patch file: {e}"))
        })?;

        // 3. Atomic rename (on Unix, this is atomic).
        fs::rename(&tmp_path, &absolute_path).map_err(|e| {
            FrensenseError::Config(format!("Failed to apply patch atomically: {e}"))
        })?;

        Ok(())
    }

    /// Applies a single advisory to a file atomically using Shadow Writing.
    ///
    /// # Errors
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Returns an error if file reading, patching, or renaming fails.
    pub fn apply_fix(&self, advisory: &Advisory, file_path: &Path) -> Result<()> {
        self.apply_fixes(&[advisory], file_path)
    }
}
