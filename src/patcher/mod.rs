// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseError, Result};
#[cfg(feature = "remediation")]
use diff;
use pathdiff;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "remediation")]
pub struct PatchManager {
    root_dir: PathBuf,
}

#[cfg(feature = "remediation")]
impl PatchManager {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    /// Resolves a logical import path (potentially containing {{root}})
    /// to a relative path from the current file.
    fn resolve_import_path(&self, from_file: &Path, import_stmt: &str) -> String {
        if !import_stmt.contains("{{root}}") {
            return import_stmt.to_string();
        }

        let mut final_stmt = import_stmt.to_string();

        // Extract the path between quotes
        if let Some(start_quote) = import_stmt.find('\'').or(import_stmt.find('\"')) {
            if let Some(end_quote) = import_stmt.rfind('\'').or(import_stmt.rfind('\"')) {
                if start_quote < end_quote {
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
                                &import_stmt[..start_quote + 1],
                                s,
                                &import_stmt[end_quote..]
                            );
                        }
                    }
                }
            }
        }
        final_stmt
    }

    /// Generates a unified diff for an advisory's proposed replacement.
    pub fn generate_diff(&self, advisory: &Advisory, file_path: &Path) -> Result<String> {
        let proposed = match &advisory.proposed_replacement {
            Some(p) => p,
            None => return Ok(String::new()),
        };

        let mut diff_output = String::new();
        diff_output.push_str(&format!("--- a/{}\n", file_path.display()));
        diff_output.push_str(&format!("+++ b/{}\n", file_path.display()));
        diff_output.push_str(&format!(
            "@@ -{},{} +{},{} @@\n",
            advisory.line, 1, advisory.line, 1
        ));

        let result = diff::lines(&advisory.original_content, proposed);
        for line in result {
            match line {
                diff::Result::Left(l) => {
                    diff_output.push_str(&format!("-{l}\n"));
                }
                diff::Result::Both(l, _) => {
                    diff_output.push_str(&format!(" {l}\n"));
                }
                diff::Result::Right(r) => {
                    diff_output.push_str(&format!("+{r}\n"));
                }
            }
        }

        Ok(diff_output)
    }

    /// Applies the fix atomically using Shadow Writing.
    pub fn apply_fix(&self, advisory: &Advisory, file_path: &Path) -> Result<()> {
        let proposed = match &advisory.proposed_replacement {
            Some(p) => p,
            None => return Ok(()),
        };

        let absolute_path = self.root_dir.join(file_path);
        let content = fs::read_to_string(&absolute_path)
            .map_err(|e| GenSenseError::Config(format!("Failed to read file for patching: {e}")))?;

        // Range-Based Verification: Ensure the content at the specific offset matches.
        let start = advisory.start_byte as usize;
        let end = advisory.end_byte as usize;

        if start > content.len()
            || end > content.len()
            || content[start..end] != advisory.original_content
        {
            return Err(GenSenseError::Config(format!(
                "Patch failed for {}: Context mismatch at byte {}. Expected '{}', found '{}'",
                file_path.display(),
                start,
                advisory.original_content,
                if start < content.len() {
                    &content[start..std::cmp::min(end, content.len())]
                } else {
                    "EOF"
                }
            )));
        }

        // 1. Create updated content in memory using precise range replacement.
        let mut updated_content = String::with_capacity(content.len() + proposed.len());
        updated_content.push_str(&content[..start]);
        updated_content.push_str(proposed);
        updated_content.push_str(&content[end..]);

        // 2. Import Injection
        if let Some(import_template) = &advisory.proposed_import {
            let import_stmt = self.resolve_import_path(file_path, import_template);
            if !updated_content.contains(&import_stmt) {
                // Find insertion point: after the last import statement or at the top
                let mut insertion_offset = 0;

                // Simple heuristic to find the end of the import block
                let mut last_pos = 0;
                while let Some(pos) = updated_content[last_pos..].find("import ") {
                    let absolute_pos = last_pos + pos;
                    if let Some(line_end) = updated_content[absolute_pos..].find('\n') {
                        insertion_offset = absolute_pos + line_end + 1;
                        last_pos = insertion_offset;
                    } else {
                        insertion_offset = updated_content.len();
                        break;
                    }
                }

                let mut final_content =
                    String::with_capacity(updated_content.len() + import_stmt.len() + 2);
                final_content.push_str(&updated_content[..insertion_offset]);
                final_content.push_str(&import_stmt);
                final_content.push('\n');

                // Add a blank line if we are prepending at the very top and it's not currently blank
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

        // 3. Write to a temporary file.
        let tmp_path = absolute_path.with_extension("patch_tmp");
        fs::write(&tmp_path, updated_content).map_err(|e| {
            GenSenseError::Config(format!("Failed to write temporary patch file: {e}"))
        })?;

        // 3. Atomic rename (on Unix, this is atomic).
        fs::rename(&tmp_path, &absolute_path)
            .map_err(|e| GenSenseError::Config(format!("Failed to apply patch atomically: {e}")))?;

        Ok(())
    }
}
