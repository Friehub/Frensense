// SPDX-License-Identifier: MIT

use crate::{FrensenseRule, ProjectRule};
use std::path::{Path, PathBuf};

#[must_use]
pub fn load_user_rules(
    _project_root: &Path,
    _extra_dirs: &[PathBuf],
) -> (Vec<Box<dyn FrensenseRule>>, Vec<Box<dyn ProjectRule>>) {
    (Vec::new(), Vec::new())
}
