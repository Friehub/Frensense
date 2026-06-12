// SPDX-License-Identifier: MIT

use super::FrensenseAuditor;
use crate::{FrensenseRule, ProjectRule};
use std::path::Path;

impl FrensenseAuditor {
    #[must_use]
    pub fn default_rules() -> (Vec<Box<dyn FrensenseRule>>, Vec<Box<dyn ProjectRule>>) {
        (Vec::new(), Vec::new())
    }

    #[allow(clippy::unused_self)]
    pub fn build_rule_set(
        _root: &Path,
        _extra_dirs: &[std::path::PathBuf],
        _no_builtin_rules: bool,
    ) -> (Vec<Box<dyn FrensenseRule>>, Vec<Box<dyn ProjectRule>>) {
        (Vec::new(), Vec::new())
    }
}
