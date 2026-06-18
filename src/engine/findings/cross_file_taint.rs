use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId};
use regex::Regex;

pub fn find(
    symbols: &SymbolRegistry,
    snap: &crate::engine::project::FileSnapshot,
    source_re: &Regex,
    sink_re: &Regex,
) -> Vec<Advisory> {
    let graph = symbols.graph();
    let mut advisories = Vec::new();

    for sym in graph.all_symbols() {
        if sym.file_path != snap.path.to_string_lossy() || !source_re.is_match(&sym.name) {
            continue;
        }
        let caller_file = &sym.file_path;
        if let Some(caller_id) = graph.find_node(&sym.name, caller_file, sym.line) {
            for callee_id in graph.neighbors_of(caller_id, crate::semantics::graph::EdgeKind::Calls)
            {
                if let Some(callee) = graph.get_symbol(callee_id) {
                    if caller_file != &callee.file_path && sink_re.is_match(&callee.name) {
                        advisories.push(
                            Advisory::bare("CROSS_FILE_TAINT", crate::Severity::Warning, FileId(0), std::path::Path::new(&callee.file_path),
                                format!("Taint may flow from '{}' in {} through call to '{}' in {}.",
                                    sym.name, caller_file, callee.name, callee.file_path))
                                .with_confidence(0.65)
                                .with_content(format!("{} -> {}", sym.name, callee.name))
                                .with_impact("Taint crossing file boundaries may bypass single-file sanitization.")
                                .with_improvement("Validate or sanitize taint at the file boundary.")
                                .with_tags(["taint", "cross-file", "security"])
                        );
                    }
                }
            }
        }
    }
    advisories
}
