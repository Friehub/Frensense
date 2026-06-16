// SPDX-License-Identifier: MIT

use super::FrensenseAuditor;
use crate::FrensenseRule;

impl FrensenseAuditor {
    #[must_use]
    pub fn default_rules() -> Vec<Box<dyn FrensenseRule>> {
        Vec::new()
    }
}
