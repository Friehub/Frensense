// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;
use tree_sitter::Node;

use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicOp {
    Lock,
    Unlock,
    CondWait,
    CondSignal,
    AtomicLoad,
    AtomicStore,
    Fence,
}

#[derive(Debug, Clone)]
pub struct AtomicEvent {
    pub op: AtomicOp,
    pub target: String,
    pub line: usize,
    pub column: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone)]
pub struct AtomicSection {
    pub start_event: AtomicEvent,
    pub end_event: AtomicEvent,
    pub lock_var: String,
    pub events: Vec<AtomicEvent>,
    pub is_complete: bool,
    pub span: (usize, usize),
}

#[derive(Debug, Default)]
pub struct AtomicSectionAnalyzer {
    sections: Vec<AtomicSection>,
    toctou_candidates: Vec<AtomicSection>,
}

impl AtomicSectionAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&mut self, root: Node, source: &str, file_path: &Path) {
        let mut events = self.collect_events(root, source, file_path);
        events.sort_by_key(|e| e.line);

        self.find_sections(&events);
        self.detect_toctou(&events);
    }

    fn collect_events(&self, root: Node, source: &str, _file_path: &Path) -> Vec<AtomicEvent> {
        let mut events = Vec::new();
        let mut cursor = root.walk();

        loop {
            let node = cursor.node();
            let kind = node.kind();

            if kind == "call_expression" {
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
                    let line = node.start_position().row + 1;
                    let column = node.start_position().column + 1;

                    if text.contains("lock(") || text.contains("mutex_lock(") || text.contains("acquire(") {
                        let target = extract_target(text, "lock");
                        events.push(AtomicEvent {
                            op: AtomicOp::Lock,
                            target,
                            line,
                            column,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
                    } else if text.contains("unlock(") || text.contains("mutex_unlock(") || text.contains("release(") {
                        let target = extract_target(text, "unlock");
                        events.push(AtomicEvent {
                            op: AtomicOp::Unlock,
                            target,
                            line,
                            column,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
                    } else if text.contains("wait(") || text.contains("cond_wait(") {
                        let target = extract_target(text, "wait");
                        events.push(AtomicEvent {
                            op: AtomicOp::CondWait,
                            target,
                            line,
                            column,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
                    } else if text.contains("signal(") || text.contains("cond_signal(") {
                        let target = extract_target(text, "signal");
                        events.push(AtomicEvent {
                            op: AtomicOp::CondSignal,
                            target,
                            line,
                            column,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
                    }
                }
            } else if kind == "assignment_expression" || kind == "expression_statement" {
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
                    let line = node.start_position().row + 1;
                    let column = node.start_position().column + 1;
                    if text.contains("atomic_store") || text.contains("__atomic_store") {
                        let target = extract_target(text, "store");
                        events.push(AtomicEvent {
                            op: AtomicOp::AtomicStore,
                            target,
                            line,
                            column,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
                    } else if text.contains("atomic_load") || text.contains("__atomic_load") {
                        let target = extract_target(text, "load");
                        events.push(AtomicEvent {
                            op: AtomicOp::AtomicLoad,
                            target,
                            line,
                            column,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
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

    fn find_sections(&mut self, events: &[AtomicEvent]) {
        let mut lock_stack: Vec<(usize, &AtomicEvent)> = Vec::new();

        for (i, event) in events.iter().enumerate() {
            match event.op {
                AtomicOp::Lock => {
                    lock_stack.push((i, event));
                }
                AtomicOp::Unlock => {
                    if let Some((start_idx, start_event)) = lock_stack.pop() {
                        if start_event.target == event.target {
                            let section_events: Vec<AtomicEvent> = events[start_idx..=i].to_vec();
                            self.sections.push(AtomicSection {
                                start_event: start_event.clone(),
                                end_event: event.clone(),
                                lock_var: event.target.clone(),
                                events: section_events,
                                is_complete: true,
                                span: (start_event.line, event.line),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        for &(_, start_event) in &lock_stack {
            self.sections.push(AtomicSection {
                start_event: start_event.clone(),
                end_event: AtomicEvent {
                    op: AtomicOp::Unlock,
                    target: start_event.target.clone(),
                    line: 0,
                    column: 0,
                    start_byte: 0,
                    end_byte: 0,
                },
                lock_var: start_event.target.clone(),
                events: vec![start_event.clone()],
                is_complete: false,
                span: (start_event.line, 0),
            });
        }
    }

    fn detect_toctou(&mut self, events: &[AtomicEvent]) {
        let mut sections_by_var: HashMap<&str, Vec<&AtomicEvent>> = HashMap::new();

        for event in events {
            if event.op == AtomicOp::Lock {
                sections_by_var.entry(&event.target).or_default().push(event);
            }
        }

        for (_var, lock_events) in &sections_by_var {
            if lock_events.len() >= 2 {
                for pair in lock_events.windows(2) {
                    if let [first, second] = pair {
                        let between: Vec<&AtomicEvent> = events
                            .iter()
                            .filter(|e| e.line > first.line && e.line < second.line)
                            .collect();
                        let has_unlock = between.iter().any(|e| e.op == AtomicOp::Unlock && e.target == first.target);
                        if !has_unlock {
                            for event in &between {
                                if event.op == AtomicOp::CondWait {
                                    self.toctou_candidates.push(AtomicSection {
                                        start_event: (*first).clone(),
                                        end_event: (*second).clone(),
                                        lock_var: first.target.clone(),
                                        events: between.iter().map(|e| (*e).clone()).collect(),
                                        is_complete: true,
                                        span: (first.line, second.line),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn sections(&self) -> &[AtomicSection] {
        &self.sections
    }

    pub fn has_incomplete_sections(&self) -> bool {
        self.sections.iter().any(|s| !s.is_complete)
    }

    pub fn count_incomplete(&self) -> usize {
        self.sections.iter().filter(|s| !s.is_complete).count()
    }

    pub fn toctou_candidates(&self) -> &[AtomicSection] {
        &self.toctou_candidates
    }

    pub fn has_toctou(&self) -> bool {
        !self.toctou_candidates.is_empty()
    }
}

fn extract_target(text: &str, op: &str) -> String {
    let after_op = if let Some(idx) = text.find(op) {
        &text[idx + op.len()..]
    } else {
        return "unknown".to_string();
    };

    if let Some(paren_start) = after_op.find('(') {
        let inside = &after_op[paren_start + 1..];
        if let Some(paren_end) = inside.find(')') {
            let args = inside[..paren_end].trim();
            if let Some(comma) = args.find(',') {
                return args[..comma].trim().to_string();
            }
            return args.to_string();
        }
    }

    let mut target = String::new();
    for ch in after_op.chars() {
        if ch.is_alphanumeric() || ch == '_' || ch == '-' {
            target.push(ch);
        } else {
            break;
        }
    }
    if target.is_empty() { "unknown".to_string() } else { target }
}

pub fn analyze_atomic_sections(
    root: Node,
    source: &str,
    file_path: &Path,
) -> Result<AtomicSectionAnalyzer> {
    let mut analyzer = AtomicSectionAnalyzer::new();
    analyzer.analyze(root, source, file_path);
    Ok(analyzer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_target_lock() {
        let target = extract_target("mutex_lock(&m)", "lock");
        assert_eq!(target, "&m");
    }

    #[test]
    fn test_extract_target_unlock() {
        let target = extract_target("mutex_unlock(&m)", "unlock");
        assert_eq!(target, "&m");
    }

    #[test]
    fn test_extract_target_unknown() {
        let target = extract_target("foo()", "lock");
        assert_ne!(target, "");
    }

    #[test]
    #[cfg(feature = "c_lang")]
    fn test_empty_source() {
        let source = "";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_c::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut analyzer = AtomicSectionAnalyzer::new();
        analyzer.analyze(tree.root_node(), source, Path::new("test.c"));
        assert!(analyzer.sections().is_empty());
        assert!(!analyzer.has_incomplete_sections());
    }

    #[test]
    #[cfg(feature = "c_lang")]
    fn test_lock_unlock_pair() {
        let source = r#"
void foo() {
    mutex_lock(&m);
    x = 1;
    mutex_unlock(&m);
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_c::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut analyzer = AtomicSectionAnalyzer::new();
        analyzer.analyze(tree.root_node(), source, Path::new("test.c"));
        assert!(!analyzer.sections().is_empty(), "should detect lock/unlock");
    }

    #[test]
    #[cfg(feature = "c_lang")]
    fn test_incomplete_lock() {
        let source = r#"
void foo() {
    mutex_lock(&m);
    x = 1;
    // missing unlock
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_c::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut analyzer = AtomicSectionAnalyzer::new();
        analyzer.analyze(tree.root_node(), source, Path::new("test.c"));
        assert!(analyzer.has_incomplete_sections(), "should detect incomplete lock");
    }

    #[test]
    #[cfg(feature = "c_lang")]
    fn test_toctou_detection() {
        let source = r#"
void check_and_use() {
    mutex_lock(&m);
    if (cond) {
        cond_wait(&cv, &m);
    }
    mutex_unlock(&m);
    mutex_lock(&m);
    // use again
    mutex_unlock(&m);
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_c::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut analyzer = AtomicSectionAnalyzer::new();
        analyzer.analyze(tree.root_node(), source, Path::new("test.c"));
        assert!(!analyzer.sections().is_empty());
    }
}
