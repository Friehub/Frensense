// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet, VecDeque};
use crate::data_flow::TaintOrigin;
use crate::graph::{EdgeKind, SemanticGraph};
use crate::symbols::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrossFileTaint {
    pub source_file: String,
    pub sink_file: String,
    pub source_symbol: String,
    pub sink_symbol: String,
    pub origin: TaintOrigin,
    pub path_length: usize,
}

#[derive(Debug, Clone, Default)]
pub struct CrossFileTaintResolver {
    call_graph: HashMap<String, Vec<String>>,
    reverse_call_graph: HashMap<String, Vec<String>>,
    module_map: HashMap<String, Vec<String>>,
    exposed_taint: HashMap<(String, String), TaintOrigin>,
}

impl CrossFileTaintResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_from_symbols(&mut self, all_symbols: &[&Symbol], graph: &SemanticGraph) {
        for sym in all_symbols {
            let sym_key = format!("{}:{}", sym.file_path, sym.name);
            let callees = graph
                .find_nodes(&sym.name)
                .into_iter()
                .flat_map(|node_id| graph.neighbors_of(node_id, EdgeKind::Calls))
                .filter_map(|callee_id| graph.get_symbol(callee_id))
                .map(|callee| format!("{}:{}", callee.file_path, callee.name));

            let callee_list: Vec<String> = callees.collect();
            if !callee_list.is_empty() {
                self.call_graph.entry(sym_key.clone()).or_default().extend(callee_list.clone());
                for callee in &callee_list {
                    self.reverse_call_graph.entry(callee.clone()).or_default().push(sym_key.clone());
                }
            }
        }
    }

    pub fn register_exposed_taint(&mut self, symbol_key: &str, file_path: &str, origin: TaintOrigin) {
        self.exposed_taint.insert((symbol_key.to_string(), file_path.to_string()), origin);
    }

    pub fn resolve_taint(
        &self,
        sink_symbol: &str,
        sink_file: &str,
        max_depth: usize,
    ) -> Vec<CrossFileTaint> {
        let sink_key = format!("{sink_file}:{sink_symbol}");
        let mut results = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((sink_key.clone(), 0));
        visited.insert(sink_key.clone());

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            if let Some((origin, source_key)) = self.find_taint_source(&current) {
                let mut source_file = source_key.clone();
                let mut source_symbol = source_key.clone();
                if let Some(idx) = source_key.find(':') {
                    source_file = source_key[..idx].to_string();
                    source_symbol = source_key[idx + 1..].to_string();
                }
                results.push(CrossFileTaint {
                    source_file,
                    sink_file: sink_file.to_string(),
                    source_symbol,
                    sink_symbol: sink_symbol.to_string(),
                    origin,
                    path_length: depth + 1,
                });
            }

            if let Some(callers) = self.reverse_call_graph.get(&current) {
                for caller in callers {
                    if visited.insert(caller.clone()) {
                        queue.push_back((caller.clone(), depth + 1));
                    }
                }
            }

            if let Some(imports) = self.module_map.get(&current) {
                for imp in imports {
                    if visited.insert(imp.clone()) {
                        queue.push_back((imp.clone(), depth + 1));
                    }
                }
            }
        }

        results
    }

    fn find_taint_source(&self, key: &str) -> Option<(TaintOrigin, String)> {
        for ((sym_key, file_path), origin) in &self.exposed_taint {
            let full_key = format!("{file_path}:{sym_key}");
            if full_key == key {
                return Some((origin.clone(), key.to_string()));
            }
        }
        None
    }

    pub fn all_taint_paths(&self, max_depth: usize) -> Vec<CrossFileTaint> {
        let mut results = Vec::new();
        for sink_key in self.call_graph.keys() {
            if let Some(idx) = sink_key.find(':') {
                let sink_file = &sink_key[..idx];
                let sink_symbol = &sink_key[idx + 1..];
                let paths = self.resolve_taint(sink_symbol, sink_file, max_depth);
                results.extend(paths);
            }
        }
        results
    }

    pub fn callers_of(&self, symbol_key: &str) -> Vec<&str> {
        self.reverse_call_graph
            .get(symbol_key)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn callees_of(&self, symbol_key: &str) -> Vec<&str> {
        self.call_graph
            .get(symbol_key)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}

pub fn build_resolver(project_symbols: &[(&Symbol, &SemanticGraph)]) -> CrossFileTaintResolver {
    let mut resolver = CrossFileTaintResolver::new();
    let all_symbol_refs: Vec<&Symbol> = project_symbols.iter().map(|(s, _)| *s).collect();
    for (_sym, graph) in project_symbols {
        resolver.build_from_symbols(&all_symbol_refs, graph);
    }
    resolver
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::SemanticGraph;
    use crate::symbols::{Symbol, SymbolKind};
    use crate::FileId;

    fn make_symbol(name: &str, file: &str) -> Symbol {
        Symbol {
            name: name.to_string(),
            kind: SymbolKind::Function,
            start_byte: 0,
            end_byte: 0,
            file_path: file.to_string(),
            file_id: FileId(0),
            line: 1,
            column: 1,
            end_line: 1,
        }
    }

    #[test]
    fn test_resolve_taint_no_sources() {
        let resolver = CrossFileTaintResolver::new();
        let results = resolver.resolve_taint("sink", "test.rs", 3);
        assert!(results.is_empty());
    }

    #[test]
    fn test_exposed_taint_direct() {
        let mut resolver = CrossFileTaintResolver::new();
        resolver.register_exposed_taint("source", "a.rs", TaintOrigin::UserInput);
        let mut graph = SemanticGraph::new();
        let sym = make_symbol("source", "a.rs");
        graph.add_symbol(sym);
        resolver.build_from_symbols(&[&make_symbol("source", "a.rs")], &graph);
    }
}
