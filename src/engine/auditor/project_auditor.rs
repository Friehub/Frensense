// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseEnvironment, ProjectRule, SourceRegistry};
use crate::semantics::SymbolRegistry;

pub struct ProjectAuditor {
    pub rules: Vec<Box<dyn ProjectRule>>,
}

impl ProjectAuditor {
    pub fn new(rules: Vec<Box<dyn ProjectRule>>) -> Self {
        Self { rules }
    }

    pub fn run(
        &self,
        symbols: &SymbolRegistry,
        sources: &SourceRegistry,
        env: GenSenseEnvironment,
    ) -> Vec<Advisory> {
        self.rules
            .iter()
            .filter(|r| r.is_enabled_in(env))
            .flat_map(|r| r.check_project(symbols, sources))
            .collect()
    }
}
