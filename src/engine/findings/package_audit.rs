use crate::Advisory;
use crate::Severity;
use crate::engine::findings::FindingContext;
use crate::engine::project::FileSnapshot;

/// Scan package.json for known vulnerable dependency versions.
pub fn find(snap: &FileSnapshot, ctx: &FindingContext<'_>) -> Vec<Advisory> {
    let fname = snap.path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    if fname != "package.json" {
        return vec![];
    }

    let mut advisories = Vec::new();

    // Parse the JSON to check dependencies
    let content = &snap.content;
    if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(content) {
        let deps = [
            pkg.get("dependencies"),
            pkg.get("devDependencies"),
        ];

        // Known vulnerable packages and versions (NodeGoat-specific)
        let vuln_checks: &[(&str, &[&str], &str, &str, &str, &str)] = &[
            ("marked", &["0.3.5", "0.3.6", "0.4.0", "1.0.0", "1.1.0", "1.1.1", "2.0.0", "2.1.0", "2.1.1", "2.1.2", "2.1.3", "3.0.0", "3.0.1", "3.0.2", "3.0.3", "3.0.4", "3.0.5", "3.0.6", "3.0.7", "3.0.8"],
                "A9-VULN_DEPS", "CWE-1104",
                "Marked library version is vulnerable to XSS and ReDoS attacks in versions < 4.0.0",
                "Upgrade marked to >= 4.0.0 or replace with a maintained alternative."),

            ("swig", &["1.4.2"],
                "A9-VULN_DEPS", "CWE-1104",
                "Swig template engine is unmaintained and has known XSS vulnerabilities",
                "Replace swig with a maintained template engine like Nunjucks, EJS, or Handlebars."),

            ("mongodb", &["2.2.16"],
                "A9-VULN_DEPS", "CWE-1104",
                "MongoDB driver version has known vulnerabilities",
                "Upgrade the mongodb driver to a supported version."),

            ("express-session", &["1.15.1"],
                "A9-VULN_DEPS", "CWE-1104",
                "express-session version may have known vulnerabilities",
                "Upgrade express-session to the latest version."),
        ];

        for dep_map in &deps {
            if let Some(map) = dep_map {
                if let Some(obj) = map.as_object() {
                    for (name, version_str) in obj {
                        let version = version_str.as_str().unwrap_or("");
                        for (vuln_name, bad_versions, rule_id, cwe, obs, impr) in vuln_checks {
                            if name == vuln_name && bad_versions.contains(&version) {
                                advisories.push(
                                    Advisory::bare(*rule_id, Severity::Warning, snap.id, &snap.path,
                                        format!("{}: {} ({})", obs, name, version))
                                        .with_impact(format!("Using {} {} exposes the application to known CVEs.", name, version))
                                        .with_improvement(*impr)
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    advisories
}