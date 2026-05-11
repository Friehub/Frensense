// SPDX-License-Identifier: MIT

use crate::semantics::data_flow::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata};
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
        source: String, // Regex pattern
        sink: String,   // Regex pattern
    },
    ScopeConstraint {
        pattern: String, // e.g., "async_fn"
    },
    Temporal {
        sequence: Vec<String>, // List of call patterns
        behavior: TemporalBehavior,
    },
}

#[derive(Debug, Clone)]
pub enum TemporalBehavior {
    MustFollow,
    MustNotFollow,
    ForbiddenBetween(String, String), // No X between Y and Z
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
        {
            let cache = context.taint_cache.borrow();
            if cache.contains_key(&(self.id().to_string(), top.id())) {
                return Vec::new();
            }
        }

        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // 1. Regex Content Matching
        if let Some(re) = &self.if_matches {
            if re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    context,
                    self.metadata.impact.to_string(),
                ));
            } else {
                return Vec::new();
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

        // 4. Flow Constraints
        for constraint in &self.flow_constraints {
            match constraint {
                FlowConstraint::TaintReached { source, sink } => {
                    let src_re = regex::Regex::new(source).unwrap();
                    let sink_re = regex::Regex::new(sink).unwrap();
                    let analyzer = DataFlowAnalyzer::new(context, top);
                    let mut registry = TaintRegistry::default();
                    analyzer.discover_symbols(&mut registry);

                    let target_node = node.child_by_field_name("body").unwrap_or(node);
                    advisories.extend(analyzer.analyze_block(
                        target_node,
                        &src_re,
                        &sink_re,
                        self,
                        registry,
                    ));
                }
                FlowConstraint::ScopeConstraint { .. } => {}
                FlowConstraint::Temporal { sequence, behavior } => {
                    let analyzer = crate::semantics::temporal::TemporalAnalyzer::new(context);
                    advisories.extend(analyzer.check_temporal(node, sequence, behavior, self));
                }
            }
        }

        // Populate Taint Cache for the ROOT
        {
            let mut cache = context.taint_cache.borrow_mut();
            cache.insert((self.id().to_string(), top.id()), Vec::new());
        }

        advisories
    }
}
