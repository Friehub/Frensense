// SPDX-License-Identifier: MIT

use std::path::Path;
use tree_sitter::Node;

use crate::graph::{EdgeKind, EventType, SemanticGraph, SemanticNodeId, TemporalEvent};

#[derive(Debug, Clone)]
pub struct TemporalRuleToml {
    pub id: String,
    pub sequence: Vec<String>,
    pub behavior: String,
    pub severity: String,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TemporalEventLabel {
    pub call_pattern: String,
    pub event_name: String,
}

#[derive(Debug, Clone)]
pub struct TemporalConstraint {
    pub before: String,
    pub after: String,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemporalRule {
    pub name: String,
    pub constraints: Vec<TemporalConstraint>,
}

impl TemporalRule {
    pub fn validate(&self, events: &[TemporalEvent], file_path: &str) -> Vec<TemporalEvent> {
        let mut violations = Vec::new();

        for constraint in &self.constraints {
            let before_events: Vec<&TemporalEvent> = events
                .iter()
                .filter(|e| e.label == constraint.before)
                .collect();
            let after_events: Vec<&TemporalEvent> = events
                .iter()
                .filter(|e| e.label == constraint.after)
                .collect();

            for be in &before_events {
                let has_following_after = after_events.iter().any(|ae| ae.line > be.line);
                if !has_following_after {
                    violations.push(TemporalEvent {
                        event_type: EventType::Call,
                        label: format!(
                            "Violation: {} should be followed by {}",
                            constraint.before, constraint.after
                        ),
                        file_path: file_path.to_string(),
                        line: be.line,
                        column: be.column,
                    });
                }
            }
        }

        violations
    }
}

#[derive(Debug, Default)]
pub struct TemporalAnalyzer {
    graph: SemanticGraph,
    rules: Vec<TemporalRule>,
    violations: Vec<TemporalEvent>,
    last_event_id: Option<SemanticNodeId>,
    labels: Vec<TemporalEventLabel>,
}

impl TemporalAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: TemporalRule) {
        self.rules.push(rule);
    }

    pub fn add_default_rules(&mut self) {
        self.rules.push(TemporalRule {
            name: "lock_unlock".to_string(),
            constraints: vec![TemporalConstraint {
                before: "lock".to_string(),
                after: "unlock".to_string(),
                description: "Every lock() must be followed by unlock()".to_string(),
                severity: Severity::Error,
            }],
        });

        self.rules.push(TemporalRule {
            name: "acquire_release".to_string(),
            constraints: vec![TemporalConstraint {
                before: "acquire".to_string(),
                after: "release".to_string(),
                description: "Every acquire() must be followed by release()".to_string(),
                severity: Severity::Error,
            }],
        });

        self.rules.push(TemporalRule {
            name: "open_close".to_string(),
            constraints: vec![TemporalConstraint {
                before: "open".to_string(),
                after: "close".to_string(),
                description: "Every open() must be followed by close()".to_string(),
                severity: Severity::Warning,
            }],
        });

        self.rules.push(TemporalRule {
            name: "connect_disconnect".to_string(),
            constraints: vec![TemporalConstraint {
                before: "connect".to_string(),
                after: "disconnect".to_string(),
                description: "Every connect() must be followed by disconnect()".to_string(),
                severity: Severity::Warning,
            }],
        });

        self.rules.push(TemporalRule {
            name: "RUST_LOCK_SLEEP".to_string(),
            constraints: vec![TemporalConstraint {
                before: "lock".to_string(),
                after: "sleep".to_string(),
                description: "Holding a lock while sleeping may cause deadlock".to_string(),
                severity: Severity::Error,
            }],
        });
    }

    /// Add rules from TOML configuration.
    pub fn add_rules_from_toml(&mut self, rules: &[TemporalRuleToml]) {
        for toml_rule in rules {
            for seq_item in &toml_rule.sequence {
                let call_pattern = seq_item.clone();
                let event_name = seq_item.clone();
                if !self.labels.iter().any(|l| l.event_name == event_name) {
                    self.labels.push(TemporalEventLabel {
                        call_pattern,
                        event_name,
                    });
                }
            }

            let constraints = toml_rule
                .sequence
                .windows(2)
                .map(|pair| TemporalConstraint {
                    before: pair[0].clone(),
                    after: pair[1].clone(),
                    description: toml_rule.observation.clone(),
                    severity: match toml_rule.severity.to_lowercase().as_str() {
                        "error" | "critical" => Severity::Error,
                        "warning" => Severity::Warning,
                        _ => Severity::Info,
                    },
                })
                .collect();

            self.rules.push(TemporalRule {
                name: toml_rule.id.clone(),
                constraints,
            });
        }
    }

    pub fn labels(&self) -> &[TemporalEventLabel] {
        &self.labels
    }

    pub fn analyze(&mut self, events: &[TemporalEvent], file_path: &str) -> Vec<TemporalEvent> {
        self.violations.clear();
        self.last_event_id = None;

        for event in events {
            let node_id = self.graph.add_event(event.clone());
            if let Some(prev) = self.last_event_id {
                self.graph
                    .add_edge(prev, node_id, EdgeKind::SequentiallyFollows);
            }
            self.last_event_id = Some(node_id);
        }

        for rule in &self.rules {
            let rule_violations = rule.validate(events, file_path);
            self.violations.extend(rule_violations);
        }

        self.violations.clone()
    }

    pub fn analyze_with_events(
        &mut self,
        root: Node,
        source: &str,
        file_path: &Path,
        temporal_labels: Option<&[TemporalEventLabel]>,
    ) -> Vec<TemporalEvent> {
        let events = crate::graph::extract_temporal_events(root, source, file_path, temporal_labels);
        let file_str = file_path.to_string_lossy().to_string();
        self.analyze(&events, &file_str)
    }

    pub fn analyze_event_list(&self, events: &[TemporalEvent]) -> Vec<TemporalEvent> {
        let mut all_violations = Vec::new();

        for rule in &self.rules {
            for constraint in &rule.constraints {
                let before_events: Vec<&TemporalEvent> = events
                    .iter()
                    .filter(|e| e.label == constraint.before)
                    .collect();
                let after_events: Vec<&TemporalEvent> = events
                    .iter()
                    .filter(|e| e.label == constraint.after)
                    .collect();

                for be in &before_events {
                    let has_following_after = after_events.iter().any(|ae| ae.line > be.line);
                    if !has_following_after {
                        all_violations.push(TemporalEvent {
                            event_type: EventType::Call,
                            label: format!(
                                "Violation: {} should be followed by {} ({})",
                                constraint.before, constraint.after, rule.name
                            ),
                            file_path: be.file_path.clone(),
                            line: be.line,
                            column: be.column,
                        });
                    }
                }
            }
        }

        all_violations
    }

    pub fn check_must_follow<'a>(
        &self,
        events: &'a [TemporalEvent],
        before: &str,
        after: &str,
    ) -> Vec<&'a TemporalEvent> {
        events
            .iter()
            .filter(|e| e.label == before)
            .filter(|be| {
                !events
                    .iter()
                    .any(|ae| ae.label == after && ae.line > be.line)
            })
            .collect()
    }

    pub fn violations(&self) -> &[TemporalEvent] {
        &self.violations
    }

    pub fn graph(&self) -> &SemanticGraph {
        &self.graph
    }
}

pub fn extract_ordered_events<'a>(
    root: Node<'a>,
    source: &'a str,
    file_path: &Path,
    temporal_labels: Option<&[TemporalEventLabel]>,
) -> Vec<TemporalEvent> {
    let mut events = Vec::new();
    let mut cursor = root.walk();
    let file_str = file_path.to_string_lossy().to_string();

    loop {
        let node = cursor.node();
        let kind = node.kind();

        if kind == "call_expression" {
            if let Ok(call_text) = node.utf8_text(source.as_bytes()) {
                let line = node.start_position().row + 1;
                let column = node.start_position().column + 1;

                let event_type = if call_text.contains(".lock()")
                    || call_text.contains("mutex_lock(")
                {
                    // In Rust, `let guard = mutex.lock()` binds a MutexGuard that is
                    // automatically dropped at scope end (RAII). Only flag if NOT
                    // assigned to a let binding.
                    let is_let_binding = {
                        let mut p = node.parent();
                        while let Some(parent) = p {
                            if parent.kind() == "let_declaration"
                                || parent.kind() == "variable_declaration"
                            {
                                break;
                            }
                            if parent.kind() == "function_item"
                                || parent.kind() == "function_definition"
                                || parent.kind() == "arrow_function"
                                || parent.kind() == "method_definition"
                            {
                                break;
                            }
                            p = parent.parent();
                        }
                        p.is_some_and(|pt| {
                            pt.kind() == "let_declaration" || pt.kind() == "variable_declaration"
                        })
                    };
                    if is_let_binding {
                        None
                    } else {
                        Some((EventType::Acquire, "lock"))
                    }
                } else if call_text.contains(".unlock()") || call_text.contains("mutex_unlock(") {
                    Some((EventType::Release, "unlock"))
                } else if call_text.contains(".await") {
                    Some((EventType::Await, "await"))
                } else if call_text.contains(".close()") {
                    Some((EventType::Release, "close"))
                } else if call_text.contains(".open(") {
                    Some((EventType::Acquire, "open"))
                } else if call_text.contains(".connect(") {
                    Some((EventType::Acquire, "connect"))
                } else if call_text.contains(".disconnect(") {
                    Some((EventType::Release, "disconnect"))
                } else if call_text.contains(".acquire(") {
                    Some((EventType::Acquire, "acquire"))
                } else if call_text.contains(".release(") {
                    Some((EventType::Release, "release"))
                } else if call_text.contains("sleep(") || call_text.contains("thread::sleep(") {
                    Some((EventType::Call, "sleep"))
                } else {
                    None
                };

                if let Some((event_type, label)) = event_type {
                    events.push(TemporalEvent {
                        event_type,
                        label: label.to_string(),
                        file_path: file_str.clone(),
                        line,
                        column,
                    });
                }

                if let Some(temporal_labels) = temporal_labels {
                    for label in temporal_labels {
                        if call_text.contains(label.call_pattern.as_str()) {
                            events.push(TemporalEvent {
                                event_type: EventType::Call,
                                label: label.event_name.clone(),
                                file_path: file_str.clone(),
                                line,
                                column,
                            });
                        }
                    }
                }
            }
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return events;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_rule_lock_unlock_violation() {
        let events = vec![TemporalEvent {
            event_type: EventType::Acquire,
            label: "lock".to_string(),
            file_path: "test.rs".to_string(),
            line: 1,
            column: 1,
        }];

        let rule = TemporalRule {
            name: "lock_unlock".to_string(),
            constraints: vec![TemporalConstraint {
                before: "lock".to_string(),
                after: "unlock".to_string(),
                description: "test".to_string(),
                severity: Severity::Error,
            }],
        };

        let violations = rule.validate(&events, "test.rs");
        assert!(!violations.is_empty(), "should detect missing unlock");
    }

    #[test]
    fn test_temporal_rule_no_violation() {
        let events = vec![
            TemporalEvent {
                event_type: EventType::Acquire,
                label: "lock".to_string(),
                file_path: "test.rs".to_string(),
                line: 1,
                column: 1,
            },
            TemporalEvent {
                event_type: EventType::Release,
                label: "unlock".to_string(),
                file_path: "test.rs".to_string(),
                line: 10,
                column: 1,
            },
        ];

        let rule = TemporalRule {
            name: "lock_unlock".to_string(),
            constraints: vec![TemporalConstraint {
                before: "lock".to_string(),
                after: "unlock".to_string(),
                description: "test".to_string(),
                severity: Severity::Error,
            }],
        };

        let violations = rule.validate(&events, "test.rs");
        assert!(violations.is_empty(), "lock then unlock should be valid");
    }

    #[test]
    fn test_analyzer_with_default_rules() {
        let events = vec![TemporalEvent {
            event_type: EventType::Acquire,
            label: "lock".to_string(),
            file_path: "test.rs".to_string(),
            line: 1,
            column: 1,
        }];

        let mut analyzer = TemporalAnalyzer::new();
        analyzer.add_default_rules();
        let violations = analyzer.analyze(&events, "test.rs");
        assert!(
            violations.iter().any(|v| v.label.contains("lock")),
            "should flag lock without unlock"
        );
    }

    #[test]
    fn test_must_follow() {
        let events = vec![
            TemporalEvent {
                event_type: EventType::Acquire,
                label: "lock".to_string(),
                file_path: "test.rs".to_string(),
                line: 1,
                column: 1,
            },
            TemporalEvent {
                event_type: EventType::Release,
                label: "unlock".to_string(),
                file_path: "test.rs".to_string(),
                line: 5,
                column: 1,
            },
        ];

        let analyzer = TemporalAnalyzer::new();
        let violations = analyzer.check_must_follow(&events, "lock", "unlock");
        assert!(violations.is_empty(), "lock is followed by unlock");

        let events2 = vec![TemporalEvent {
            event_type: EventType::Acquire,
            label: "lock".to_string(),
            file_path: "test.rs".to_string(),
            line: 1,
            column: 1,
        }];
        let violations2 = analyzer.check_must_follow(&events2, "lock", "unlock");
        assert!(!violations2.is_empty(), "should detect lock without unlock");
    }
}
