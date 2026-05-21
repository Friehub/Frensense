// SPDX-License-Identifier: MIT

use crate::engine::source::SourceRegistry;
use crate::semantics::SymbolRegistry;
use crate::{Advisory, FileId, ProjectRule, RuleMetadata, Severity};
use once_cell::sync::Lazy;
use std::borrow::Cow;

pub struct GlobalAllocatorCheck;

static META: Lazy<RuleMetadata> = Lazy::new(|| RuleMetadata {
    id: Cow::Borrowed("RUST_NO_GLOBAL_ALLOCATOR"),
    name: Cow::Borrowed("Rust No Global Allocator"),
    severity: Severity::Info,
    observation: Cow::Borrowed("No #[global_allocator] is declared in this project."),
    impact: Cow::Borrowed(
        "The default system allocator is used. For allocation-heavy workloads (e.g. parallel AST scanning), mimalloc or jemalloc typically improve throughput by 20-40%.",
    ),
    improvement: Cow::Borrowed(
        "Add mimalloc = \"2\" to Cargo.toml and declare #[global_allocator] static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc; in main.rs.",
    ),
    tags: vec![],
    category: Cow::Borrowed("Quality"),
    confidence: 0.95,
});

impl ProjectRule for GlobalAllocatorCheck {
    fn metadata(&self) -> &RuleMetadata {
        &META
    }

    fn check_project(&self, _symbols: &SymbolRegistry, sources: &SourceRegistry) -> Vec<Advisory> {
        let has_allocator = sources
            .all_sources()
            .any(|(_, src)| src.contains("#[global_allocator]"));

        if !has_allocator {
            // Find a suitable file to report the advisory on (e.g. main.rs, lib.rs, or first rust file).
            // Default to FileId(0) if none found.
            let rust_file = sources
                .all_files()
                .find(|f| f.path.extension().and_then(|e| e.to_str()) == Some("rs"));

            let (file_id, file_path) = match rust_file {
                Some(f) => (f.id, f.path.to_string_lossy().to_string()),
                None => (FileId(0), String::new()),
            };

            vec![Advisory {
                rule_id: "RUST_NO_GLOBAL_ALLOCATOR".into(),
                file_id,
                file_path,
                severity: Severity::Info,
                confidence: 0.95,
                observation: "No #[global_allocator] is declared in this project.".into(),
                impact: "The default system allocator is used. For allocation-heavy workloads (e.g. parallel AST scanning), mimalloc or jemalloc typically improve throughput by 20-40%.".into(),
                improvement: "Add mimalloc = \"2\" to Cargo.toml and declare #[global_allocator] static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc; in main.rs.".into(),
                line: 1,
                column: 1,
                start_byte: 0,
                end_byte: 0,
                original_content: sources
                    .get(file_id)
                    .and_then(|src| {
                        src.content
                            .lines()
                            .next()
                            .map(str::trim)
                            .map(std::string::String::from)
                    })
                    .unwrap_or_default(),
                proposed_replacement: None,
                proposed_import: None,
                enclosing_symbol: None,
                fingerprint: String::new(),
                auto_fixable: false,
                requires_human: true,
                tags: vec![],
            }]
        } else {
            vec![]
        }
    }
}
