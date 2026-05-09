// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, AuditorError, Result};
use diff;
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
            .map_err(|e| AuditorError::Config(format!("Failed to read file for patching: {e}")))?;

        // Context-Aware Verification: Ensure the original content still exists exactly as expected.
        if !content.contains(&advisory.original_content) {
            return Err(AuditorError::Config(format!(
                "Patch failed for {}: Context mismatch. Code has changed since scan.",
                file_path.display()
            )));
        }

        // Shadow Writing Pattern:
        // 1. Create updated content in memory.
        let updated_content = content.replace(&advisory.original_content, proposed);

        // 2. Write to a temporary file.
        let tmp_path = absolute_path.with_extension("patch_tmp");
        fs::write(&tmp_path, updated_content).map_err(|e| {
            AuditorError::Config(format!("Failed to write temporary patch file: {e}"))
        })?;

        // 3. Atomic rename (on Unix, this is atomic).
        fs::rename(&tmp_path, &absolute_path)
            .map_err(|e| AuditorError::Config(format!("Failed to apply patch atomically: {e}")))?;

        Ok(())
    }
}
