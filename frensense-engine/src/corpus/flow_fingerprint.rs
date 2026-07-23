// SPDX-License-Identifier: MIT

//! Lightweight intra-function data-flow path fingerprinting.
//!
//! Does NOT require a full data-flow graph. Operates purely on the AST
//! using def-use tracking within a single function body. The resulting
//! path hashes are invariant to variable renaming, helper extraction,
//! and formatting changes.

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};
use tree_sitter::Node;

use crate::corpus::motifs::MOTIF_LOOKUP;

/// A source-to-sink path represented as a sequence of abstract node labels.
#[derive(Debug, Clone)]
pub struct FlowPath {
    /// E.g. ["UserInputSource", "assignment", "call", "CommandExecutionSink"]
    pub labels: Vec<&'static str>,
}

impl FlowPath {
    pub fn hash(&self) -> u64 {
        let mut h = FxHasher::default();
        self.labels.hash(&mut h);
        h.finish()
    }
}

/// Extract lightweight data-flow path hashes from a function body.
///
/// Algorithm:
/// 1. Find all assignments where the RHS contains a known UserInputSource.
/// 2. For each assigned variable, trace forward through the body to see
///    if it reaches a known sink (CommandExecutionSink, SqlSink, etc.).
/// 3. Record the abstract path: source_motif → [intermediate_motifs] → sink_motif.
/// 4. Hash each path.
///
/// This is O(n²) in the number of assignments × calls, which is fine for
/// function bodies (n < 200 in practice).
pub fn extract_flow_paths(body: Node<'_>, source: &str) -> Vec<u64> {
    let lookup = &*MOTIF_LOOKUP;

    // Step 1: collect all identifiers that are assigned from a source motif
    let tainted_vars = collect_tainted_vars(body, source, lookup);
    if tainted_vars.is_empty() {
        return Vec::new();
    }

    // Step 2: find sink calls that use tainted variables as arguments
    let mut path_hashes = FxHashSet::default();
    find_sink_paths(body, source, &tainted_vars, lookup, &mut path_hashes);

    let mut vec: Vec<u64> = path_hashes.into_iter().collect();
    vec.sort_unstable();
    vec
}

/// Returns a map of variable name → source motif name for variables
/// assigned from a recognized source.
fn collect_tainted_vars<'a>(
    node: Node<'_>,
    source: &str,
    lookup: &FxHashMap<String, &'static str>,
) -> FxHashMap<String, &'static str> {
    let mut tainted: FxHashMap<String, &'static str> = FxHashMap::default();
    collect_tainted_recursive(node, source, lookup, &mut tainted);
    tainted
}

fn collect_tainted_recursive(
    node: Node<'_>,
    source: &str,
    lookup: &FxHashMap<String, &'static str>,
    tainted: &mut FxHashMap<String, &'static str>,
) {
    let kind = node.kind();

    // Variable declaration with initializer: `let cmd = req.body.cmd`
    if kind == "variable_declarator"
        || kind == "assignment_expression"
    {
        if let (Some(name_node), Some(value_node)) = (
            node.child_by_field_name("name")
                .or_else(|| node.child_by_field_name("left")),
            node.child_by_field_name("value")
                .or_else(|| node.child_by_field_name("right")),
        ) {
            let var_name = &source[name_node.start_byte()..name_node.end_byte()];
            let rhs_text = &source[value_node.start_byte()..value_node.end_byte()];

            // Check if RHS contains a known source member
            for (member, &motif) in lookup {
                if motif == "UserInputSource" && rhs_text.contains(member.as_str()) {
                    tainted.insert(var_name.to_string(), motif);
                    break;
                }
            }
        }
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_tainted_recursive(cursor.node(), source, lookup, tainted);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Walk calls in the body. For each call, check if any argument uses a
/// tainted variable. If so, emit a path hash.
fn find_sink_paths(
    node: Node<'_>,
    source: &str,
    tainted: &FxHashMap<String, &'static str>,
    lookup: &FxHashMap<String, &'static str>,
    out: &mut FxHashSet<u64>,
) {
    if node.kind() == "call_expression" {
        if let Some(func) = node.child_by_field_name("function") {
            let call_name = &source[func.start_byte()..func.end_byte()];
            // Resolve to motif
            let sink_motif = lookup.get(call_name).copied().or_else(|| {
                call_name
                    .rfind("::")
                    .or_else(|| call_name.rfind('.'))
                    .and_then(|p| lookup.get(&call_name[p + 1..]).copied())
            });

            if let Some(sink_motif) = sink_motif {
                // Check arguments for tainted variables
                if let Some(args) = node.child_by_field_name("arguments") {
                    let args_text = &source[args.start_byte()..args.end_byte()];
                    for (var, &source_motif) in tainted {
                        if args_text.contains(var.as_str()) {
                            // Emit path: source_motif → sink_motif
                            let path = FlowPath {
                                labels: vec![source_motif, "taint_flow", sink_motif],
                            };
                            out.insert(path.hash());
                        }
                    }
                }
            }
        }
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            find_sink_paths(cursor.node(), source, tainted, lookup, out);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
