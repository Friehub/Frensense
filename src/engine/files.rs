// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! File collection: walk the target, keep supported source files.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Collects all files reachable from `root` that match supported extensions
/// and the optional language filter.
#[must_use]
pub fn collect_files(root: &Path, language_filter: Option<&Vec<&'static str>>) -> Vec<PathBuf> {
    if root.is_file() {
        return vec![root.to_path_buf()];
    }
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            if e.file_type().is_dir() && e.path() != root {
                return name != "target"
                    && name != "node_modules"
                    && name != "dist"
                    && name != "build"
                    && name != "vendor"
                    && name != "out"
                    && name != ".next"
                    && name != ".nuxt"
                    && name != ".cache"
                    && name != "coverage"
                    && name != "cypress"
                    && name != "playwright"
                    && name != "storybook-static"
                    && !name.starts_with('.');
            }
            if e.file_type().is_file() {
                if let Ok(meta) = e.metadata()
                    && meta.len() > 500_000
                {
                    return false;
                }
                if name.ends_with(".min.js")
                    || name.ends_with(".bundle.js")
                    || name.ends_with(".chunk.js")
                    || name.ends_with(".debug.js")
                {
                    return false;
                }
            }
            true
        })
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| crate::parser::is_supported(p))
        .filter(|p| !is_test_file(p))
        .filter(|p| {
            if let Some(allowed) = language_filter {
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                allowed.contains(&ext)
            } else {
                true
            }
        })
        .collect()
}

#[must_use]
fn is_test_file(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let stem = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");

    if name.ends_with(".test.ts")
        || name.ends_with(".test.tsx")
        || name.ends_with(".test.js")
        || name.ends_with(".test.jsx")
        || name.ends_with(".spec.ts")
        || name.ends_with(".spec.tsx")
        || name.ends_with(".spec.js")
        || name.ends_with(".spec.jsx")
        || name.ends_with("_test.rs")
        || name.ends_with(".test.rs")
    {
        return true;
    }

    let path_str = path.to_string_lossy();
    if path_str.contains("/tests/")
        || path_str.contains("/test/")
        || path_str.contains("__tests__/")
        || path_str.contains("/__mocks__/")
        || path_str.contains("/mocks/")
    {
        return true;
    }

    stem.starts_with("mock") || stem.to_lowercase().ends_with(".mock")
}
