// SPDX-License-Identifier: MIT

use super::{Engine, FileSnapshot};
use crate::engine::auditor::{AuditOptions, ScanResult};
use crate::parser::ParserRegistry;
use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Collects all files reachable from `root` that match supported extensions
/// and the optional language filter.
pub fn collect_files(root: &Path, language_filter: Option<&Vec<&'static str>>) -> Vec<PathBuf> {
    if root.is_file() {
        return vec![root.to_path_buf()];
    }
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                if e.path() != root {
                    return name != "target" && name != "node_modules" && !name.starts_with('.');
                }
            }
            true
        })
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| ParserRegistry::is_supported(p))
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

pub(crate) fn collect_files_impl(engine: &mut Engine, root: &Path) -> Result<Vec<FileSnapshot>> {
    let files = collect_files(root, engine.language_filter.as_ref());
    let mut snapshots = Vec::new();
    for p in files {
        let content = match std::fs::read_to_string(&p) {
            Ok(c) => c,
            Err(e) => {
                engine.file_cache.remove(&p);
                tracing::warn!("skipping unreadable file {}: {e}", p.display());
                continue;
            }
        };
        let id = engine.source_registry.register(&p, content.clone());
        let auditor = &engine.auditor;
        if engine.file_cache.is_unchanged(&p, &content) {
            if let Ok((_language, tree)) = auditor.parse_source(&p, &content) {
                snapshots.push(FileSnapshot {
                    id,
                    path: p,
                    content,
                    tree,
                    symbols: Vec::new(),
                    edges: Vec::new(),
                    semantic_ops: Vec::new(),
                });
            }
            continue;
        }
        match auditor.parse_source(&p, &content) {
            Ok((language, tree)) => {
                let symbols = auditor.discover_symbols(&p, id, &content, &language, &tree);
                let edges = auditor.scan_for_edges(&p, &content, &language, &tree);
                let semantic_ops = auditor.extract_semantic_ops(&p, &content, &tree);
                if let (Ok(symbols), Ok(edges)) = (symbols, edges) {
                    engine.file_cache.update(&p, &content);
                    snapshots.push(FileSnapshot {
                        id,
                        path: p,
                        content,
                        tree,
                        symbols,
                        edges,
                        semantic_ops,
                    });
                } else {
                    engine.file_cache.remove(&p);
                    tracing::warn!("symbol or edge discovery failed for {}", p.display());
                }
            }
            Err(e) => {
                engine.file_cache.remove(&p);
                tracing::warn!("skipping unparseable file {}: {e}", p.display());
            }
        }
    }
    Ok(snapshots)
}

pub(crate) fn parallel_audit_impl(
    engine: &Engine,
    file_ids: &[(FileId, PathBuf)],
    snapshot_map: &HashMap<FileId, &FileSnapshot>,
    symbols: &mut SymbolRegistry,
    file_trees: &HashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
) -> Result<Vec<Advisory>> {
    let results: Result<Vec<ScanResult>> = file_ids
        .iter()
        .map(|(id, p)| {
            let snap = snapshot_map.get(id).ok_or_else(|| {
                crate::GenSenseError::Engine(format!(
                    "Missing snapshot for file ID {} at path {}",
                    id.0,
                    p.display()
                ))
            })?;
            let opts = AuditOptions {
                file_id: *id,
                path: p,
                content: &snap.content,
                tree: &snap.tree,
                semantic_ops: &snap.semantic_ops,
                symbols,
                graph: symbols.graph(),
                file_trees,
                category_filter: &engine.enabled_categories,
                tag_filter: &engine.enabled_tags,
                suite: engine.suite,
                env: engine.environment,
                severity_filter: engine.severity_filter,
                ngram_window_size: engine.ngram_window_size,
                taint_confidence_interprocedural: engine.taint_confidence_interprocedural,
                taint_confidence_intraprocedural: engine.taint_confidence_intraprocedural,
                default_taint_max_depth: engine.default_taint_max_depth,
            };
            let result = engine.auditor.audit(&opts)?;

            for adv in &result.advisories {
                if let Some(ref sym) = adv.enclosing_symbol {
                    let graph = symbols.graph_mut();
                    graph.record_taint_flow(crate::semantics::graph::TaintFlowRecord {
                        function_name: sym.clone(),
                        file_path: adv.file_path.clone(),
                        source_pattern: String::new(),
                        sink_pattern: String::new(),
                        rule_id: adv.rule_id.clone(),
                    });
                }
            }

            Ok(ScanResult {
                advisories: result.advisories,
                #[cfg(feature = "fingerprinting")]
                fingerprints: result.fingerprints,
            })
        })
        .collect();

    let mut all_advisories = Vec::new();
    for result in results? {
        for a in result.advisories {
            if a.confidence >= engine.min_confidence {
                all_advisories.push(a);
            }
        }
    }
    Ok(all_advisories)
}
