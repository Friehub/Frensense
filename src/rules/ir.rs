use crate::{Advisory, GenSenseContext, GenSenseRule, Severity};
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

/// Metadata for diagnostic output generation.
#[derive(Debug, Clone)]
pub struct DiagnosticTemplate {
    pub observation: String,
    pub impact: String,
    pub improvement: String,
}

/// The Intermediate Representation (IR) of a GenSense Rule.
/// This is the compiled format used by the Execution Engine.
#[derive(Debug, Clone)]
pub struct CoreRuleIr {
    pub id: String,
    pub match_queries: Vec<AstQuery>,
    pub flow_constraints: Vec<FlowConstraint>,
    pub if_matches: Option<regex::Regex>,
    pub must_contain: Option<regex::Regex>,
    pub must_not_contain: Option<regex::Regex>,
    pub max_lines: Option<usize>,
    pub max_depth: Option<usize>,
    pub severity: Severity,
    pub template: DiagnosticTemplate,
    pub target_ext: String,
    pub tags: Vec<String>,
    pub category: String,
}

impl GenSenseRule for CoreRuleIr {
    fn id(&self) -> &str {
        &self.id
    }
    fn description(&self) -> &str {
        &self.template.observation
    }
    fn category(&self) -> &str {
        &self.category
    }
    fn tags(&self) -> Vec<&str> {
        self.tags.iter().map(|s| s.as_str()).collect()
    }
    fn severity(&self) -> Severity {
        self.severity
    }
    fn impact(&self) -> &str {
        &self.template.impact
    }
    fn improvement(&self) -> &str {
        &self.template.improvement
    }

    fn query(&self) -> Option<&str> {
        // v2 engine will handle multi-query, but for v1 compatibility:
        self.match_queries.get(0).map(|q| q.selector.as_str())
    }

    fn applies_to(&self, ext: &str) -> bool {
        if self.target_ext == "*" {
            return true;
        }
        self.target_ext == ext
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];

        // 1. Regex Content Matching
        if let Some(re) = &self.if_matches {
            if re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    self.template.observation.clone(),
                    self.template.impact.clone(),
                    self.template.improvement.clone(),
                ));
            } else {
                return Vec::new(); // In IR, if primary pattern fails, we skip everything else
            }
        }

        // 2. Content Constraints
        if let Some(re) = &self.must_contain {
            if !re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    format!("Pattern '{}' was expected but not found.", re.as_str()),
                    self.template.impact.clone(),
                    self.template.improvement.clone(),
                ));
            }
        }

        if let Some(re) = &self.must_not_contain {
            if re.is_match(code) {
                advisories.push(self.new_advisory(
                    &node,
                    format!("Prohibited pattern '{}' was found.", re.as_str()),
                    self.template.impact.clone(),
                    self.template.improvement.clone(),
                ));
            }
        }

        // 3. Metric Checks
        let start_pos = node.start_position();
        let end_pos = node.end_position();
        let node_lines = end_pos.row - start_pos.row + 1;

        if let Some(max) = self.max_lines {
            if node_lines > max {
                advisories.push(self.new_advisory(
                    &node,
                    format!("Function size ({node_lines} lines) exceeds threshold of {max}."),
                    self.template.impact.clone(),
                    self.template.improvement.clone(),
                ));
            }
        }

        // 2. Flow Constraints
        for constraint in &self.flow_constraints {
            match constraint {
                FlowConstraint::TaintReached { source, sink } => {
                    // Re-use existing DataFlowAnalyzer logic
                    let src_re = regex::Regex::new(source).unwrap();
                    let sink_re = regex::Regex::new(sink).unwrap();

                    let mut top = node;
                    while let Some(parent) = top.parent() {
                        top = parent;
                    }

                    let analyzer = crate::semantics::data_flow::DataFlowAnalyzer::new(context, top);
                    advisories.extend(analyzer.analyze_block(
                        node,
                        &src_re,
                        &sink_re,
                        self, // This works because we implement GenSenseRule
                        crate::semantics::data_flow::TaintRegistry::default(),
                    ));
                }
                FlowConstraint::ScopeConstraint { pattern: _ } => {
                    // Logic from core.rs
                }
                FlowConstraint::Temporal { sequence, behavior } => {
                    let analyzer = crate::semantics::temporal::TemporalAnalyzer::new(context);
                    advisories.extend(analyzer.check_temporal(node, sequence, behavior, self));
                }
            }
        }

        advisories
    }
}

impl CoreRuleIr {
    pub fn new_advisory(
        &self,
        node: &Node,
        observation: String,
        impact: String,
        improvement: String,
    ) -> Advisory {
        Advisory {
            rule_id: self.id.clone(),
            severity: self.severity,
            observation,
            impact,
            improvement,
            line: node.start_position().row + 1,
            column: node.start_position().column + 1,
            file_path: String::new(),
            original_content: String::new(),
            proposed_replacement: None,
        }
    }
}
