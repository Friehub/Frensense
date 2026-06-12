// SPDX-License-Identifier: MIT

use crate::semantics::SymbolRegistry;
use crate::{Advisory, FrensenseEnvironment, ProjectRule, SourceRegistry};

pub struct ProjectAuditor {
    pub rules: Vec<Box<dyn ProjectRule>>,
}

impl ProjectAuditor {
    #[must_use]
    pub fn new(rules: Vec<Box<dyn ProjectRule>>) -> Self {
        Self { rules }
    }

    #[must_use]
    pub fn run(
        &self,
        symbols: &SymbolRegistry,
        sources: &SourceRegistry,
        env: FrensenseEnvironment,
    ) -> Vec<Advisory> {
        self.rules
            .iter()
            .filter(|r| r.is_enabled_in(env))
            .flat_map(|r| r.check_project(symbols, sources))
            .collect()
    }
}
