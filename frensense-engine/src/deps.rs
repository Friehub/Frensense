// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

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
    check_deps_enabled: bool,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_check_deps(enabled: bool) -> Self {
        Self {
            check_deps_enabled: enabled,
            ..Default::default()
        }
    }

    pub fn load_project(&mut self, root: &Path) {
        // If root is a file, use its parent directory
        let project_root = if root.is_file() {
            root.parent().unwrap_or(root)
        } else {
            root
        };

        // Find workspace root for Rust projects
        let workspace_root = Self::find_workspace_root(project_root);

        if let Some(ref ws_root) = workspace_root {
            self.load_cargo_lock(ws_root);
            self.load_cargo_toml_deps(ws_root);
        } else {
            self.load_cargo_lock(project_root);
            self.load_cargo_toml_deps(project_root);
        }
        self.load_package_json(project_root);

        // Also load package.json from common subdirectories (monorepo support)
        self.load_package_json_from_dir(&project_root.join("apps"));
        self.load_package_json_from_dir(&project_root.join("packages"));
        self.load_package_json_from_dir(&project_root.join("services"));

        // Try to load workspace patterns from pnpm-workspace.yaml or similar
        self.load_workspace_packages(project_root);

        // Traverse upward to find ALL package.json files in ancestor directories
        // This handles monorepos where deps are spread across workspace roots
        let mut current = project_root.to_path_buf();
        for _ in 0..10 {
            if !current.pop() {
                break;
            }
            if current.join("package.json").exists() {
                self.load_package_json(&current);
                self.load_workspace_packages(&current);
                self.load_package_json_from_dir(&current.join("apps"));
                self.load_package_json_from_dir(&current.join("packages"));
                self.load_package_json_from_dir(&current.join("services"));
            }
            // Also check for Cargo.toml (Rust workspace root)
            if current.join("Cargo.toml").exists() {
                self.load_cargo_lock(&current);
                self.load_cargo_toml_deps(&current);
            }
            // Stop at git root
            if current.join(".git").exists() {
                break;
            }
        }

        if self.check_deps_enabled {
            self.verify_cargo_metadata_available(project_root);
        }
    }

    fn load_workspace_packages(&mut self, root: &Path) {
        // Try pnpm-workspace.yaml
        let pnpm_path = root.join("pnpm-workspace.yaml");
        if let Ok(content) = fs::read_to_string(&pnpm_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("- '") || trimmed.starts_with("- \"") {
                    let pattern = trimmed
                        .trim_start_matches("- ")
                        .trim_matches('\'')
                        .trim_matches('"');
                    // Convert glob pattern to directory path (e.g., "apps/*" -> "apps")
                    if let Some(star_pos) = pattern.find("/*") {
                        let dir_name = &pattern[..star_pos];
                        self.load_package_json_from_dir(&root.join(dir_name));
                    } else if !pattern.contains('*') {
                        self.load_package_json(&root.join(pattern));
                    }
                }
            }
        }

        // Try yarn workspaces in package.json
        let pkg_path = root.join("package.json");
        if let Ok(content) = fs::read_to_string(&pkg_path) {
            if let Some(ws_start) = content.find("\"workspaces\"") {
                let ws_section = &content[ws_start..];
                // Check if workspaces is an object with "packages" key
                if let Some(packages_start) = ws_section.find("\"packages\"") {
                    let packages_section = &ws_section[packages_start..];
                    for line in packages_section.lines() {
                        let trimmed = line.trim().trim_matches(',').trim_matches('"');
                        if trimmed.starts_with("apps/")
                            || trimmed.starts_with("packages/")
                            || trimmed.starts_with("services/")
                        {
                            let dir = trimmed.trim_end_matches("/*");
                            self.load_package_json_from_dir(&root.join(dir));
                        }
                    }
                }
            }
        }
    }

    fn verify_cargo_metadata_available(&self, root: &Path) {
        // Check if this is a Rust project (has Cargo.toml)
        if !root.join("Cargo.toml").exists() {
            return;
        }

        // Check if cargo metadata command works
        let output = std::process::Command::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .arg("--no-deps")
            .current_dir(root)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                // cargo metadata works, we can use it for more accurate dependency checking
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("Warning: cargo metadata failed: {}", stderr.trim());
                eprintln!(
                    "Dependency checking may be incomplete. Install Rust toolchain for full support."
                );
            }
            Err(e) => {
                eprintln!("Warning: cargo not found on PATH: {e}");
                eprintln!("Dependency checking will use Cargo.toml/Cargo.lock only.");
                eprintln!("Install Rust toolchain for full dependency verification.");
            }
        }
    }

    fn load_package_json_from_dir(&mut self, dir: &Path) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.file_name().is_none_or(|n| n != "node_modules") {
                    self.load_package_json(&path);
                    self.load_package_json_from_dir(&path);
                }
            }
        }
    }

    fn _find_package_json_upward(start: &Path) -> Option<PathBuf> {
        let mut current = start.to_path_buf();
        for _ in 0..10 {
            if current.join("package.json").exists() {
                return Some(current);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    fn find_workspace_root(root: &Path) -> Option<PathBuf> {
        let mut current = root.to_path_buf();
        for _ in 0..5 {
            if current.join("Cargo.lock").exists() {
                return Some(current);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    fn load_cargo_lock(&mut self, root: &Path) {
        let lock_dir = Self::find_workspace_root(root).unwrap_or_else(|| root.to_path_buf());
        let lock_path = lock_dir.join("Cargo.lock");
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

    fn load_cargo_toml_deps(&mut self, root: &Path) {
        let toml_path = root.join("Cargo.toml");
        let Ok(content) = fs::read_to_string(&toml_path) else {
            return;
        };

        let mut in_deps = false;
        let mut in_dev = false;
        let mut in_ws_deps = false;
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "[dependencies]" {
                in_deps = true;
                in_dev = false;
                in_ws_deps = false;
                continue;
            }
            if trimmed == "[dev-dependencies]" {
                in_deps = false;
                in_dev = true;
                in_ws_deps = false;
                continue;
            }
            if trimmed == "[workspace.dependencies]" {
                in_deps = false;
                in_dev = false;
                in_ws_deps = true;
                continue;
            }
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                in_deps = false;
                in_dev = false;
                in_ws_deps = false;
                continue;
            }

            if trimmed.starts_with("name = \"") {
                if let Some(s) = trimmed
                    .strip_prefix("name = \"")
                    .and_then(|s| s.strip_suffix('"'))
                {
                    self.cargo_deps.insert(s.to_string());
                }
                continue;
            }

            if in_deps || in_dev || in_ws_deps {
                if let Some(name) = extract_dep_name(trimmed) {
                    self.cargo_deps.insert(name);
                }
            }
        }

        // Note: [workspace.dependencies] is now handled in the same loop above
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
        // Exact match
        if self.npm_deps.contains(package_name) {
            return true;
        }

        // Handle subpath imports: "dotenv/config" → check "dotenv"
        if let Some(slash_pos) = package_name.find('/') {
            let base = &package_name[..slash_pos];
            if self.npm_deps.contains(base) {
                return true;
            }
        }

        // Handle scoped packages: "@fastify/cors" → already in deps as "@fastify/cors"
        // But also check if "@fastify" prefix matches any dep
        if let Some(stripped) = package_name.strip_prefix('@') {
            if let Some(slash_pos) = stripped.find('/') {
                let scope = &package_name[..=slash_pos]; // e.g., "@fastify"
                if self.npm_deps.iter().any(|d| d.starts_with(scope)) {
                    return true;
                }
            }
        }

        false
    }

    pub fn scan_file(&self, source: &str, file_path: &Path) -> Vec<HallucinatedImport> {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

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
                            && !is_nodejs_builtin(&import)
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
        // Handle both standalone `{` and `"key": {` patterns
        if trimmed.contains('{') {
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
            let key = trimmed[..pos].trim().trim_matches('"').trim_matches(',');
            if !key.is_empty() && key != "}" {
                set.insert(key.to_string());
            }
        }
    }
}

fn extract_dep_name(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('[') || trimmed.starts_with('#') {
        return None;
    }
    // Skip package metadata lines
    if trimmed.starts_with("name = ")
        || trimmed.starts_with("version = ")
        || trimmed.starts_with("authors = ")
    {
        return None;
    }
    let key = trimmed.split('=').next()?.trim();
    if key.is_empty() || key.contains(' ') || key.contains('"') || key.contains('{') {
        return None;
    }
    Some(key.to_string())
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
        // Skip type-only imports: "import type { ... } from '...'"
        if rest.trim_start().starts_with("type ") {
            return None;
        }
        let package = rest
            .rsplit("from ")
            .next()?
            .trim_matches(|c| c == '"' || c == '\'' || c == ';');
        Some(package.to_string())
    } else if line.contains("require(") {
        let start = line.find("require(")? + 8;
        let end = line[start..].find(')')?;
        let package = line[start..start + end].trim_matches(|c| c == '"' || c == '\'');
        Some(package.to_string())
    } else {
        None
    }
}

fn is_stdlib_crate(name: &str) -> bool {
    matches!(name, "std" | "core" | "alloc" | "proc_macro" | "test")
}

fn is_local_module(name: &str) -> bool {
    name.starts_with("crate::") || name == "self" || name == "super"
}

fn is_relative_import(name: &str) -> bool {
    name.starts_with('.') || name.starts_with('/') || name.starts_with("~/")
}

fn is_nodejs_builtin(name: &str) -> bool {
    // Node.js built-in modules (v18+)
    matches!(
        name,
        "assert"
            | "async_hooks"
            | "buffer"
            | "child_process"
            | "cluster"
            | "console"
            | "constants"
            | "crypto"
            | "dgram"
            | "dns"
            | "domain"
            | "events"
            | "fs"
            | "http"
            | "http2"
            | "https"
            | "inspector"
            | "module"
            | "net"
            | "os"
            | "path"
            | "perf_hooks"
            | "process"
            | "punycode"
            | "querystring"
            | "readline"
            | "repl"
            | "stream"
            | "string_decoder"
            | "sys"
            | "timers"
            | "tls"
            | "trace_events"
            | "tty"
            | "url"
            | "util"
            | "v8"
            | "vm"
            | "worker_threads"
            | "zlib"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_crate() {
        assert_eq!(
            extract_rust_crate("use serde::Serialize;"),
            Some("serde".to_string())
        );
        assert_eq!(
            extract_rust_crate("use tokio::runtime::Runtime;"),
            Some("tokio".to_string())
        );
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
        assert_eq!(
            extract_ts_package("import './styles.css';"),
            Some("./styles.css".to_string())
        );
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

    #[test]
    fn test_nodejs_builtin_not_flagged() {
        let mut resolver = DependencyResolver::new();
        resolver.npm_deps.insert("express".to_string());

        let source = r#"import crypto from 'crypto';
import express from 'express';
import fs from 'fs';
"#;
        let hits = resolver.scan_file(source, Path::new("test.ts"));

        // Only express should be found (it's in deps), crypto and fs are builtins
        assert_eq!(hits.len(), 0);
    }

    #[test]
    fn test_type_import_not_flagged() {
        let resolver = DependencyResolver::new();

        let source =
            "import type { Request } from 'express';\nimport { Response } from 'express';\n";
        let hits = resolver.scan_file(source, Path::new("test.ts"));

        // type import should not be flagged, but Response import should be (not in deps)
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].import_name, "express");
    }

    #[test]
    fn test_monorepo_upward_traversal() {
        // Create a temporary directory structure simulating a monorepo
        let tmp_dir = tempfile::tempdir().unwrap();
        let root = tmp_dir.path();

        // Create root package.json (multi-line to match real format)
        std::fs::write(
            root.join("package.json"),
            "{\n  \"dependencies\": {\n    \"react\": \"^18.0.0\"\n  }\n}",
        )
        .unwrap();

        // Create .git directory to mark as git root
        std::fs::create_dir(root.join(".git")).unwrap();

        // Create apps/web/package.json
        std::fs::create_dir_all(root.join("apps/web")).unwrap();
        std::fs::write(
            root.join("apps/web/package.json"),
            "{\n  \"dependencies\": {\n    \"next\": \"^14.0.0\"\n  }\n}",
        )
        .unwrap();

        // Create a file in a deep directory
        std::fs::create_dir_all(root.join("apps/web/src")).unwrap();
        std::fs::write(root.join("apps/web/src/index.ts"), "").unwrap();

        let mut resolver = DependencyResolver::new();
        resolver.load_project(&root.join("apps/web/src/index.ts"));

        // Should find both react (from root) and next (from apps/web)
        assert!(
            resolver.check_npm_import("react"),
            "Should find react from root package.json"
        );
        assert!(
            resolver.check_npm_import("next"),
            "Should find next from apps/web/package.json"
        );
    }
}
