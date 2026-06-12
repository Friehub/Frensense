// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct HallucinatedImport {
    pub import_name: String,
    pub line: usize,
    pub column: usize,
    pub file_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct DependencyResolver {
    cargo_deps: HashSet<String>,
    npm_deps: HashSet<String>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_project(&mut self, root: &Path) {
        self.load_cargo_lock(root);
        self.load_package_json(root);
    }

    fn load_cargo_lock(&mut self, root: &Path) {
        let lock_path = root.join("Cargo.lock");
        let Ok(content) = fs::read_to_string(&lock_path) else {
            return;
        };
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name = \"") {
                let name = trimmed
                    .strip_prefix("name = \"")
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or("");
                self.cargo_deps.insert(name.to_string());
            }
        }
    }

    fn load_package_json(&mut self, root: &Path) {
        let pkg_path = root.join("package.json");
        let Ok(content) = fs::read_to_string(&pkg_path) else {
            return;
        };
        let Some(deps_start) = content.find("\"dependencies\"") else {
            return;
        };
        parse_json_keys(&content[deps_start..], &mut self.npm_deps);
        if let Some(dev_start) = content.find("\"devDependencies\"") {
            parse_json_keys(&content[dev_start..], &mut self.npm_deps);
        }
    }

    pub fn check_cargo_import(&self, crate_name: &str) -> bool {
        self.cargo_deps.contains(crate_name)
    }

    pub fn check_npm_import(&self, package_name: &str) -> bool {
        self.npm_deps.contains(package_name)
    }

    pub fn scan_file(
        &self,
        source: &str,
        file_path: &Path,
    ) -> Vec<HallucinatedImport> {
        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let mut hits = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();

            match ext {
                "rs" => {
                    if let Some(import) = extract_rust_crate(trimmed) {
                        if !self.check_cargo_import(&import)
                            && !is_stdlib_crate(&import)
                            && !is_local_module(&import)
                        {
                            hits.push(HallucinatedImport {
                                import_name: import,
                                line: line_num + 1,
                                column: line.find("::").unwrap_or(0) + 1,
                                file_path: file_path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
                "ts" | "tsx" | "js" | "jsx" => {
                    if let Some(import) = extract_ts_package(trimmed) {
                        if !self.check_npm_import(&import)
                            && !is_relative_import(&import)
                        {
                            hits.push(HallucinatedImport {
                                import_name: import,
                                line: line_num + 1,
                                column: 1,
                                file_path: file_path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        hits
    }
}

fn parse_json_keys(text: &str, set: &mut HashSet<String>) {
    let mut in_object = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('{') {
            in_object = true;
            continue;
        }
        if !in_object {
            continue;
        }
        if trimmed.starts_with('}') {
            break;
        }
        let colon_pos = trimmed.find(':');
        if let Some(pos) = colon_pos {
            let key = trimmed[..pos]
                .trim()
                .trim_matches('"')
                .trim_matches(',');
            if !key.is_empty() && key != "}" {
                set.insert(key.to_string());
            }
        }
    }
}

fn extract_rust_crate(line: &str) -> Option<String> {
    if line.starts_with("use ") || line.starts_with("extern crate ") {
        let after = line
            .strip_prefix("use ")
            .or_else(|| line.strip_prefix("extern crate "))?;
        let crate_name = after.split("::").next()?;
        if crate_name == "crate" || crate_name == "self" || crate_name == "super" {
            return None;
        }
        Some(crate_name.to_string())
    } else {
        None
    }
}

fn extract_ts_package(line: &str) -> Option<String> {
    if let Some(rest) = line
        .strip_prefix("import ")
        .or_else(|| line.strip_prefix("from "))
    {
        let package = rest
            .rsplit("from ")
            .next()?
            .trim_matches(|c| c == '"' || c == '\'' || c == ';');
        Some(package.to_string())
    } else if line.contains("require(") {
        let start = line.find("require(")? + 8;
        let end = line[start..].find(')')?;
        let package = line[start..start + end]
            .trim_matches(|c| c == '"' || c == '\'');
        Some(package.to_string())
    } else {
        None
    }
}

fn is_stdlib_crate(name: &str) -> bool {
    matches!(
        name,
        "std"
            | "core"
            | "alloc"
            | "proc_macro"
            | "test"
    )
}

fn is_local_module(name: &str) -> bool {
    name.starts_with("crate::") || name == "self" || name == "super"
}

fn is_relative_import(name: &str) -> bool {
    name.starts_with('.') || name.starts_with('/') || name.starts_with("~/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_crate() {
        assert_eq!(extract_rust_crate("use serde::Serialize;"), Some("serde".to_string()));
        assert_eq!(extract_rust_crate("use tokio::runtime::Runtime;"), Some("tokio".to_string()));
        assert_eq!(extract_rust_crate("use crate::foo;"), None);
        assert_eq!(extract_rust_crate("use self::bar;"), None);
    }

    #[test]
    fn test_extract_ts_package() {
        assert_eq!(
            extract_ts_package("import { foo } from 'lodash';"),
            Some("lodash".to_string())
        );
        assert_eq!(
            extract_ts_package("import React from 'react';"),
            Some("react".to_string())
        );
        assert_eq!(extract_ts_package("import './styles.css';"), Some("./styles.css".to_string()));
    }

    #[test]
    fn test_stdlib_filter() {
        assert!(is_stdlib_crate("std"));
        assert!(is_stdlib_crate("core"));
        assert!(is_stdlib_crate("alloc"));
        assert!(!is_stdlib_crate("serde"));
    }

    #[test]
    fn test_hallucinated_import_detection() {
        let mut resolver = DependencyResolver::new();
        resolver.cargo_deps.insert("serde".to_string());
        resolver.cargo_deps.insert("tokio".to_string());

        let source = "use serde::Serialize;\nuse fake_crate::Foo;\nuse tokio::runtime;\n";
        let hits = resolver.scan_file(source, Path::new("test.rs"));

        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].import_name, "fake_crate");
    }
}
