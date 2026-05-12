// SPDX-License-Identifier: MIT

use crate::semantics::data_flow::{DataFlowAnalyzer, TaintRegistry};
use crate::semantics::SymbolRegistry;
use crate::{
    Advisory, FileId, GenSenseContext, GenSenseRule, ProjectRule, RuleMetadata, SourceRegistry,
};
use std::path::Path;
use tree_sitter::Node;

/// AST Query component of the IR.
#[derive(Debug, Clone)]
pub struct AstQuery {
    pub selector: String,     // e.g., "(function_item) @node"
    pub capture_name: String, // e.g., "node"
}

/// Data-flow constraint component of the IR.
#[derive(Debug, Clone)]
pub enum FlowConstraint {
    TaintReached {
        source: regex::Regex,
        sink: regex::Regex,
    },
    ScopeConstraint {
        pattern: String, // e.g., "async_fn"
    },
    Temporal {
        sequence: Vec<regex::Regex>, // List of call patterns
        behavior: TemporalBehavior,
    },
}

#[derive(Debug, Clone)]
pub enum TemporalBehavior {
    MustFollow,
    MustNotFollow,
    ForbiddenBetween(regex::Regex, regex::Regex), // No X between Y and Z
}

/// The Intermediate Representation (IR) of a GenSense Rule.
#[derive(Debug, Clone)]
pub struct CoreRuleIr {
    pub metadata: RuleMetadata,
    pub match_queries: Vec<AstQuery>,
    pub flow_constraints: Vec<FlowConstraint>,
    pub if_matches: Option<regex::Regex>,
    pub must_contain: Option<regex::Regex>,
    pub must_not_contain: Option<regex::Regex>,
    pub max_lines: Option<usize>,
    pub max_depth: Option<usize>,
    pub target_ext: String,
    pub use_query: bool,
}

impl GenSenseRule for CoreRuleIr {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn applies_to(&self, ext: &str) -> bool {
        if self.target_ext == "*" {
            return true;
        }
        self.target_ext == ext
    }

    fn query(&self) -> Option<&str> {
        self.match_queries.first().map(|q| q.selector.as_str())
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut top = node;
        while let Some(parent) = top.parent() {
            top = parent;
        }

        // Taint Cache Check
        let file_path = context.file_path.to_string_lossy().to_string();
        let function_line = context
            .symbols
            .find_function_at(&file_path, node.start_position().row + 1)
            .and_then(|idx| context.symbols.graph.get_symbol(idx))
            .map(|s| s.line)
            .unwrap_or(0); // Fallback to 0 if not in a function (e.g. global)

        // Use node's own start position as part of the key for non-function scope
        // to avoid cache collisions for global-scope nodes
        let cache_key = (
            self.id().to_string(),
            file_path.clone(),
            if function_line == 0 {
                node.start_position().row
            } else {
                function_line
            },
        );

        {
            let cache = context.taint_cache.borrow();
            if cache.contains_key(&cache_key) {
                return Vec::new();
            }
        }

        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // 1. Regex Content Matching (Gate)
        if let Some(re) = &self.if_matches {
            if !re.is_match(code) {
                return Vec::new();
            }
            // If it's a simple pattern rule (no other constraints), it fires here.
            if self.flow_constraints.is_empty()
                && self.must_contain.is_none()
                && self.must_not_contain.is_none()
                && self.max_lines.is_none()
                && self.max_depth.is_none()
            {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    self.metadata.observation.to_string(),
                ));
            }
        }

        // 2. Content Constraints
        if let Some(re) = &self.must_contain {
            if !re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Pattern '{}' was expected but not found.", re.as_str()),
                ));
            }
        }

        if let Some(re) = &self.must_not_contain {
            if re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Prohibited pattern '{}' was found.", re.as_str()),
                ));
            }
        }

        // 3. Metric Checks
        let node_lines = node.end_position().row - node.start_position().row + 1;
        if let Some(max) = self.max_lines {
            if node_lines > max {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Function size ({node_lines} lines) exceeds threshold of {max}."),
                ));
            }
        }

        if let Some(max) = self.max_depth {
            let mut depth = 0;
            let mut curr = node;
            while let Some(parent) = curr.parent() {
                depth += 1;
                curr = parent;
            }
            if depth > max {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    format!("Nesting depth ({depth}) exceeds threshold of {max}."),
                ));
            }
        }

        // 4. Flow Constraints
        for constraint in &self.flow_constraints {
            match constraint {
                FlowConstraint::TaintReached { source, sink } => {
                    let analyzer = DataFlowAnalyzer::new(context, top);
                    let mut registry = TaintRegistry::default();
                    analyzer.discover_symbols(&mut registry);

                    let target_node = node.child_by_field_name("body").unwrap_or(node);
                    let findings =
                        analyzer.analyze_block(target_node, source, sink, self, registry);

                    if !findings.is_empty() {
                        advisories.extend(findings);
                    }

                    // Cache results for the current function
                    let mut cache = context.taint_cache.borrow_mut();
                    cache.insert(cache_key.clone(), advisories.clone());
                }
                FlowConstraint::ScopeConstraint { .. } => {}
                FlowConstraint::Temporal { sequence, behavior } => {
                    let analyzer = crate::semantics::temporal::TemporalAnalyzer::new(context);
                    advisories.extend(analyzer.check_temporal(node, sequence, behavior, self));
                }
            }
        }

        advisories
    }
}

/// Project-wide flow constraint.
#[derive(Debug, Clone)]
pub enum ProjectFlowConstraint {
    /// Asserts that every symbol matching `source_pattern`
    /// has a reachable symbol matching `guard_pattern`.
    MustHaveGuard {
        source_re: regex::Regex,
        guard_re: regex::Regex,
        source_glob: glob::Pattern,
        guard_glob: glob::Pattern,
    },
    /// Asserts no symbol matching `pattern` is reachable from outside its own file.
    MustBeInternal {
        re: regex::Regex,
        glob: glob::Pattern,
    },
    /// Asserts that taint from `source_pattern` cannot reach `sink_pattern` across any call chain.
    CrossFileTaintFree {
        source_re: regex::Regex,
        sink_re: regex::Regex,
    },
}

#[derive(Debug, Clone)]
pub struct ProjectRuleIr {
    pub metadata: RuleMetadata,
    pub constraints: Vec<ProjectFlowConstraint>,
}

impl ProjectRule for ProjectRuleIr {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn check_project(&self, symbols: &SymbolRegistry, sources: &SourceRegistry) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        for constraint in &self.constraints {
            match constraint {
                ProjectFlowConstraint::MustHaveGuard {
                    source_re,
                    guard_re,
                    source_glob,
                    guard_glob,
                } => {
                    let source_symbols = symbols.find_by_regex(source_re);

                    for sym in source_symbols {
                        if !source_glob.matches(&sym.file_path) {
                            continue;
                        }

                        // BFS to find if any reachable symbol matches guard_pattern
                        let mut visited = std::collections::HashSet::new();
                        let mut queue = std::collections::VecDeque::new();
                        queue.push_back(sym);
                        visited.insert(sym.name.clone()); // Simplification: name as ID for now

                        let mut has_guard = false;
                        while let Some(current) = queue.pop_front() {
                            if guard_re.is_match(&current.name)
                                && guard_glob.matches(&current.file_path)
                            {
                                has_guard = true;
                                break;
                            }

                            for callee in symbols.get_callees(current) {
                                if visited.insert(callee.name.clone()) {
                                    queue.push_back(callee);
                                }
                            }
                        }

                        if !has_guard {
                            advisories.push(self.new_advisory_for_symbol(
                                sym,
                                sources,
                                format!(
                                    "Critical Path Violation: '{}' is missing a reachable security guard matching '{}' in files matching '{}'.",
                                    sym.name, guard_re.as_str(), guard_glob.as_str()
                                ),
                            ));
                        }
                    }
                }
                ProjectFlowConstraint::MustBeInternal { re, glob } => {
                    let target_symbols = symbols.find_by_regex(re);

                    for sym in target_symbols {
                        if !glob.matches(&sym.file_path) {
                            continue;
                        }

                        for caller in symbols.get_callers(sym) {
                            if caller.file_path != sym.file_path {
                                advisories.push(self.new_advisory_for_symbol(
                                    sym,
                                    sources,
                                    format!(
                                        "Encapsulation Leak: Internal symbol '{}' is called from outside its file (by '{}' in '{}').",
                                        sym.name, caller.name, caller.file_path
                                    ),
                                ));
                            }
                        }
                    }
                }
                ProjectFlowConstraint::CrossFileTaintFree { source_re, sink_re } => {
                    let source_symbols = symbols.find_by_regex(source_re);
                    let sink_symbols = symbols.find_by_regex(sink_re);

                    if sink_symbols.is_empty() {
                        continue;
                    }

                    for sym in source_symbols {
                        let mut visited = std::collections::HashSet::new();
                        let mut queue = std::collections::VecDeque::new();
                        queue.push_back(sym);
                        visited.insert(sym.name.clone());

                        while let Some(current) = queue.pop_front() {
                            if sink_re.is_match(&current.name) {
                                advisories.push(self.new_advisory_for_symbol(
                                    sym,
                                    sources,
                                    format!(
                                        "Cross-File Taint Leak: User-controlled input from '{}' can reach sensitive sink '{}' via call chain.",
                                        sym.name, current.name
                                    ),
                                ));
                                break;
                            }

                            for callee in symbols.get_callees(current) {
                                if visited.insert(callee.name.clone()) {
                                    queue.push_back(callee);
                                }
                            }
                        }
                    }
                }
            }
        }

        advisories
    }
}

impl ProjectRuleIr {
    fn new_advisory_for_symbol(
        &self,
        sym: &crate::semantics::Symbol,
        sources: &SourceRegistry,
        observation: String,
    ) -> Advisory {
        let file_id = sources
            .get_by_path(Path::new(&sym.file_path))
            .map(|f| f.id)
            .unwrap_or(FileId(0));

        Advisory {
            rule_id: self.metadata.id.to_string(),
            file_id,
            file_path: sym.file_path.clone(),
            severity: self.metadata.severity,
            observation,
            impact: self.metadata.impact.to_string(),
            improvement: self.metadata.improvement.to_string(),
            line: sym.line as u32,
            column: sym.column as u32,
            start_byte: sym.start_byte as u32,
            end_byte: sym.end_byte as u32,
            original_content: String::new(), // Symbols don't store full content yet
            proposed_replacement: None,
        }
    }
}
