// SPDX-License-Identifier: MIT

use crate::semantics::graph::SemanticGraph;
use crate::Advisory;
use petgraph::graph::NodeIndex;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Struct,
    Class,
    Interface,
    Enum,
    Constant,
    Module,
    Variable,
    Parameter,
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    // NEW: end_line enables range-based scope lookup.
    // When end_line is 0, it means "not set" and callers fall back to
    // exact line matching. This keeps full backwards compatibility with
    // any code that constructs Symbol without knowing the end line.
    pub end_line: usize,
}

#[derive(Default)]
pub struct SymbolRegistry {
    pub graph: SemanticGraph,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, symbol: Symbol) -> NodeIndex {
        self.graph.add_symbol(symbol)
    }

    pub fn find(&self, name: &str) -> Vec<&Symbol> {
        self.graph
            .find_nodes(name)
            .into_iter()
            .filter_map(|idx| self.graph.get_symbol(idx))
            .collect()
    }

    pub fn find_at(&self, name: &str, file: &str, line: usize) -> Option<&Symbol> {
        self.find(name)
            .into_iter()
            .find(|s| s.file_path == file && s.line == line)
    }

    pub fn find_node_at(&self, name: &str, file: &str, line: usize) -> Option<NodeIndex> {
        self.graph.find_nodes(name).into_iter().find(|&idx| {
            if let Some(sym) = self.graph.get_symbol(idx) {
                sym.file_path == file && sym.line == line
            } else {
                false
            }
        })
    }

    /// Find the NodeIndex of the function whose body contains `line`.
    ///
    /// Resolution order:
    /// 1. Range match  — if end_line > 0, the function owns every line in
    ///    [start_line, end_line]. This is the preferred path once discovery
    ///    sets end_line correctly.
    /// 2. Exact match  — if end_line == 0 (symbol built without range info),
    ///    fall back to exact start-line comparison. This preserves compatibility
    ///    with older code paths and tests that construct Symbol manually.
    ///
    /// When multiple functions match (e.g. nested functions both contain the
    /// line), the innermost one wins — it has the smallest line span.
    pub fn find_function_at(&self, file: &str, line: usize) -> Option<NodeIndex> {
        let mut best: Option<(NodeIndex, usize)> = None; // (idx, span_size)

        for idx in self.graph.all_nodes() {
            if let Some(sym) = self.graph.get_symbol(idx) {
                if sym.kind != SymbolKind::Function || sym.file_path != file {
                    continue;
                }

                let matched = if sym.end_line > 0 {
                    // Range match: line must fall inside the function body
                    line >= sym.line && line <= sym.end_line
                } else {
                    // Exact match: backwards-compatible fallback
                    sym.line == line
                };

                if matched {
                    let span = sym.end_line.saturating_sub(sym.line);
                    match best {
                        None => best = Some((idx, span)),
                        Some((_, prev_span)) if span < prev_span => {
                            best = Some((idx, span));
                        }
                        _ => {}
                    }
                }
            }
        }

        best.map(|(idx, _)| idx)
    }

    pub fn add_call_edge(&mut self, file_path: &Path, caller: &str, callee: &str) {
        let path_str = file_path.to_string_lossy();
        let callers = self.graph.find_nodes(caller);
        let callees = self.graph.find_nodes(callee);

        for &caller_idx in &callers {
            if let Some(sym) = self.graph.get_symbol(caller_idx) {
                if sym.file_path == path_str {
                    for &callee_idx in &callees {
                        self.graph.add_edge(
                            caller_idx,
                            callee_idx,
                            crate::semantics::EdgeKind::Calls,
                        );
                    }
                }
            }
        }
    }

    pub fn check_graph_deadlock(&self) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        for idx in self.graph.all_nodes() {
            if let Some(crate::semantics::graph::SemanticNode::Declaration(sym)) =
                self.graph.get_node(idx)
            {
                if sym.kind == crate::semantics::SymbolKind::Function {
                    let events = self.graph.ordered_events_in_scope(idx);

                    let mut lock_held = false;
                    for ev in events {
                        match ev.event_type {
                            crate::semantics::graph::EventType::Acquire => lock_held = true,
                            crate::semantics::graph::EventType::Release => lock_held = false,
                            crate::semantics::graph::EventType::Await if lock_held => {
                                advisories.push(Advisory {
                                    rule_id: "GRAPH_DEADLOCK".to_string(),
                                    severity: crate::Severity::Critical,
                                    observation: format!(
                                        "Potential Deadlock: Lock '{}' held across await point at {}:{}.",
                                        ev.label, ev.line, ev.column
                                    ),
                                    impact: "May cause permanent thread/task starvation.".to_string(),
                                    improvement: "Release the lock before awaiting or use an async-aware mutex.".to_string(),
                                    line: ev.line,
                                    column: ev.column,
                                    file_path: ev.file_path.clone(),
                                    original_content: "".to_string(),
                                    proposed_replacement: None,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        advisories
    }
}
