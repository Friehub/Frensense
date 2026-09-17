// SPDX-License-Identifier: MIT

use super::{FindingContext, FindingModule};
use crate::engine::project::FileSnapshot;
use crate::{Advisory, Severity};

/// Detects vulnerable dependencies by checking package.json/Cargo.toml
pub struct VulnerableDeps;

impl FindingModule for VulnerableDeps {
    fn run(&self, snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let filename = snap.path.to_string_lossy();
        let vulns = frensense_providers::deps_provider::check_all_deps(&filename, &snap.content);

        for vuln in vulns {
            let mut advisory = Advisory::bare(
                format!("VULN_DEP_{}", vuln.package_name.to_uppercase()),
                Severity::Warning,
                snap.id,
                &snap.path,
                format!(
                    "Vulnerable dependency detected: {}@{}",
                    vuln.package_name, vuln.version
                ),
            );
            advisory.confidence = 0.9;
            advisory.impact = vuln.description.to_string();
            advisory.improvement = format!("Upgrade {} to a secure version", vuln.package_name);
            advisory.cwe = Some(vuln.cwe.to_string());
            advisory.cvss = Some(7.5);
            advisory.owasp = Some("A06:2021".to_string());
            advisory.tags = vec!["vulnerable-dependency".to_string()];
            advisories.push(advisory);
        }

        advisories
    }
}
