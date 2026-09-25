// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Language support queries, backed by `frensense-lang` specs.

use std::path::Path;

/// True if the file's extension is handled by a registered language spec.
#[must_use]
pub fn is_supported(path: &Path) -> bool {
    frensense_lang::spec_for_path(path).is_some()
}

/// Extensions registered for a language name (e.g. "typescript" → ts, tsx...).
#[must_use]
pub fn extensions_for(name: &str) -> Option<&'static [&'static str]> {
    frensense_lang::all_specs()
        .find(|s| s.name().eq_ignore_ascii_case(name))
        .map(|s| s.extensions())
}
