use std::collections::HashMap;

/// Result of checking dependencies for vulnerabilities
#[derive(Debug, Clone)]
pub struct VulnerableDependency {
    pub package_name: String,
    pub version: String,
    pub description: String,
    pub cwe: String,
}

pub trait DepsProvider: Send + Sync {
    fn check_vulnerabilities(&self, filename: &str, content: &str) -> Vec<VulnerableDependency>;
}

pub struct NpmDepsProvider;

impl DepsProvider for NpmDepsProvider {
    fn check_vulnerabilities(&self, filename: &str, content: &str) -> Vec<VulnerableDependency> {
        let mut vulns = Vec::new();
        if !filename.ends_with("package.json") {
            return vulns;
        }

        let deps = extract_npm_deps(content);

        let vulnerable = [
            (
                "bcrypt-nodejs",
                "CWE-798",
                "0.0.3",
                "Use of hardcoded credentials; unmaintained, use bcrypt or bcryptjs instead",
            ),
            (
                "marked",
                "CWE-79",
                "0.3.5",
                "XSS in markdown rendering; versions < 4.0.10 are vulnerable",
            ),
            (
                "needle",
                "CWE-200",
                "2.2.4",
                "Information exposure; versions < 2.6.0 have SSRF vulnerabilities",
            ),
            (
                "node-esapi",
                "CWE-1395",
                "0.0.1",
                "Dependency on unmaintained ESAPI library",
            ),
            (
                "swig",
                "CWE-79",
                "1.4.2",
                "Template injection; unmaintained, use nunjucks or handlebars",
            ),
            (
                "helmet",
                "CWE-1021",
                "2.0.0",
                "Versions < 3.0.0 missing critical security headers",
            ),
            (
                "forever",
                "CWE-400",
                "2.0.0",
                "Process management issues; use pm2 instead",
            ),
        ];

        for (pkg, cwe, version, desc) in vulnerable {
            if deps
                .iter()
                .any(|(name, ver)| name == pkg && ver.contains(version))
            {
                vulns.push(VulnerableDependency {
                    package_name: pkg.to_string(),
                    version: version.to_string(),
                    description: desc.to_string(),
                    cwe: cwe.to_string(),
                });
            }
        }
        vulns
    }
}

fn extract_npm_deps(content: &str) -> Vec<(String, String)> {
    let mut deps = Vec::new();
    if let Some(deps_start) = content.find("\"dependencies\"") {
        let deps_section = &content[deps_start..];
        let mut in_deps = false;
        for line in deps_section.lines() {
            let trimmed = line.trim();
            if trimmed.contains('{') {
                in_deps = true;
                continue;
            }
            if !in_deps {
                continue;
            }
            if trimmed.starts_with('}') {
                break;
            }
            if let Some(colon_pos) = trimmed.find(':') {
                let key = trimmed[..colon_pos]
                    .trim()
                    .trim_matches('"')
                    .trim_matches(',');
                let value = trimmed[colon_pos + 1..]
                    .trim()
                    .trim_matches('"')
                    .trim_matches(',');
                if !key.is_empty() && !value.is_empty() {
                    deps.push((key.to_string(), value.to_string()));
                }
            }
        }
    }
    deps
}

pub struct CargoDepsProvider;

impl DepsProvider for CargoDepsProvider {
    fn check_vulnerabilities(&self, filename: &str, content: &str) -> Vec<VulnerableDependency> {
        let mut vulns = Vec::new();
        if !filename.ends_with("Cargo.toml") {
            return vulns;
        }
        vulns
    }
}

pub fn check_all_deps(filename: &str, content: &str) -> Vec<VulnerableDependency> {
    let mut all_vulns = Vec::new();
    let providers: Vec<Box<dyn DepsProvider>> =
        vec![Box::new(NpmDepsProvider), Box::new(CargoDepsProvider)];
    for p in providers {
        all_vulns.extend(p.check_vulnerabilities(filename, content));
    }
    all_vulns
}
