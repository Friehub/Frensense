// SPDX-License-Identifier: MIT

//! Consistency checking between analysis paths.
//!
//! Compares advisory sets from two analysis implementations to catch
//! divergences where one path loses information or produces false positives.
//! Used as a development-time diagnostic and CI regression gate.

use crate::Advisory;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Identifies a unique finding for comparison purposes.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingKey {
    pub rule_id: String,
    pub file_path: String,
    pub line: u32,
}

impl FindingKey {
    pub fn from_advisory(adv: &Advisory) -> Self {
        Self {
            rule_id: adv.rule_id.clone(),
            file_path: adv.file_path.clone(),
            line: adv.line,
        }
    }
}

/// Divergence between two analysis paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divergence {
    /// Findings in Path B (AST-direct) but missing from Path A (graph-based).
    pub missing_in_graph: Vec<FindingKey>,
    /// Findings in Path A (graph-based) but not in Path B (AST-direct).
    pub extra_in_graph: Vec<FindingKey>,
}

/// Aggregate metrics for divergence tracking across multiple files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DivergenceMetrics {
    /// Total findings from Path A (graph-based).
    pub total_graph: usize,
    /// Total findings from Path B (AST-direct).
    pub total_ast: usize,
    /// Total missing in graph (regressions).
    pub total_missing: usize,
    /// Total extra in graph (graph-specific findings).
    pub total_extra: usize,
    /// Per-rule divergence counts.
    pub per_rule: HashMap<String, RuleDivergence>,
    /// Per-file divergence counts.
    pub per_file: HashMap<String, FileDivergence>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleDivergence {
    pub missing: usize,
    pub extra: usize,
    pub total_graph: usize,
    pub total_ast: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileDivergence {
    pub missing: usize,
    pub extra: usize,
}

/// Consistency check between two analysis paths.
#[derive(Debug, Clone)]
pub struct ConsistencyCheck {
    pub path_ast: Vec<Advisory>,
    pub path_graph: Vec<Advisory>,
}

impl ConsistencyCheck {
    #[must_use]
    pub fn new(path_ast: Vec<Advisory>, path_graph: Vec<Advisory>) -> Self {
        Self {
            path_ast,
            path_graph,
        }
    }

    /// Check if both paths produce identical advisory sets.
    #[must_use]
    pub fn verify(&self) -> bool {
        let keys_a: HashSet<_> = self
            .path_ast
            .iter()
            .map(FindingKey::from_advisory)
            .collect();
        let keys_b: HashSet<_> = self
            .path_graph
            .iter()
            .map(FindingKey::from_advisory)
            .collect();
        keys_a == keys_b
    }

    /// Detect specific divergences between paths.
    #[must_use]
    pub fn detect_divergence(&self) -> Divergence {
        let keys_ast: HashSet<_> = self
            .path_ast
            .iter()
            .map(FindingKey::from_advisory)
            .collect();
        let keys_graph: HashSet<_> = self
            .path_graph
            .iter()
            .map(FindingKey::from_advisory)
            .collect();

        let missing_in_graph = keys_ast
            .iter()
            .filter(|k| !keys_graph.contains(k))
            .cloned()
            .collect();

        let extra_in_graph = keys_graph
            .iter()
            .filter(|k| !keys_ast.contains(k))
            .cloned()
            .collect();

        Divergence {
            missing_in_graph,
            extra_in_graph,
        }
    }

    /// Compute aggregate divergence metrics.
    #[must_use]
    pub fn metrics(&self) -> DivergenceMetrics {
        let div = self.detect_divergence();
        let mut metrics = DivergenceMetrics::default();

        metrics.total_graph = self.path_graph.len();
        metrics.total_ast = self.path_ast.len();
        metrics.total_missing = div.missing_in_graph.len();
        metrics.total_extra = div.extra_in_graph.len();

        // Per-rule metrics from graph path
        for adv in &self.path_graph {
            let entry = metrics.per_rule.entry(adv.rule_id.clone()).or_default();
            entry.total_graph += 1;
        }

        // Per-rule metrics from AST path
        for adv in &self.path_ast {
            let entry = metrics.per_rule.entry(adv.rule_id.clone()).or_default();
            entry.total_ast += 1;
        }

        // Count missing per rule
        for key in &div.missing_in_graph {
            if let Some(entry) = metrics.per_rule.get_mut(&key.rule_id) {
                entry.missing += 1;
            }
        }

        // Count extra per rule
        for key in &div.extra_in_graph {
            if let Some(entry) = metrics.per_rule.get_mut(&key.rule_id) {
                entry.extra += 1;
            }
        }

        // Per-file metrics
        for key in &div.missing_in_graph {
            let entry = metrics.per_file.entry(key.file_path.clone()).or_default();
            entry.missing += 1;
        }

        for key in &div.extra_in_graph {
            let entry = metrics.per_file.entry(key.file_path.clone()).or_default();
            entry.extra += 1;
        }

        metrics
    }
}

/// Save divergence metrics to a JSON baseline file.
pub fn save_baseline(metrics: &DivergenceMetrics, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(metrics).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

/// Load divergence metrics from a JSON baseline file.
pub fn load_baseline(path: &Path) -> Option<DivergenceMetrics> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Compare current metrics against a baseline and report regressions.
#[must_use]
pub fn check_regression(current: &DivergenceMetrics, baseline: &DivergenceMetrics) -> Vec<String> {
    let mut regressions = Vec::new();

    if current.total_missing > baseline.total_missing {
        regressions.push(format!(
            "Missing findings increased: {} → {}",
            baseline.total_missing, current.total_missing
        ));
    }

    if current.total_extra > baseline.total_extra {
        regressions.push(format!(
            "Extra findings increased: {} → {}",
            baseline.total_extra, current.total_extra
        ));
    }

    // Check per-rule regressions
    for (rule, current_div) in &current.per_rule {
        if let Some(baseline_div) = baseline.per_rule.get(rule)
            && current_div.missing > baseline_div.missing
        {
            regressions.push(format!(
                "Rule '{}': missing findings increased: {} → {}",
                rule, baseline_div.missing, current_div.missing
            ));
        }
    }

    regressions
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_advisory(rule_id: &str, line: u32) -> Advisory {
        Advisory::bare(
            rule_id,
            crate::Severity::Warning,
            crate::FileId(0),
            std::path::Path::new("test.rs"),
            "test finding",
        )
        .with_line(line)
    }

    #[test]
    fn test_consistency_identical() {
        let advisories = vec![
            make_advisory("TAINT_INPUT_TO_EXEC", 10),
            make_advisory("TAINT_CREDENTIAL_TO_DB", 20),
        ];
        let check = ConsistencyCheck::new(advisories.clone(), advisories);
        assert!(check.verify());
    }

    #[test]
    fn test_consistency_divergence() {
        let path_ast = vec![
            make_advisory("TAINT_INPUT_TO_EXEC", 10),
            make_advisory("TAINT_CREDENTIAL_TO_DB", 20),
        ];
        let path_graph = vec![
            make_advisory("TAINT_INPUT_TO_EXEC", 10),
            make_advisory("CORPUS_XSS_PATTERN", 30),
        ];
        let check = ConsistencyCheck::new(path_ast, path_graph);
        assert!(!check.verify());

        let div = check.detect_divergence();
        assert_eq!(div.missing_in_graph.len(), 1);
        assert_eq!(div.extra_in_graph.len(), 1);
        assert_eq!(div.missing_in_graph[0].rule_id, "TAINT_CREDENTIAL_TO_DB");
        assert_eq!(div.extra_in_graph[0].rule_id, "CORPUS_XSS_PATTERN");
    }

    #[test]
    fn test_metrics() {
        let path_ast = vec![
            make_advisory("TAINT_INPUT_TO_EXEC", 10),
            make_advisory("TAINT_INPUT_TO_EXEC", 20),
        ];
        let path_graph = vec![
            make_advisory("TAINT_INPUT_TO_EXEC", 10),
            make_advisory("CORPUS_XSS_PATTERN", 30),
        ];
        let check = ConsistencyCheck::new(path_ast, path_graph);
        let metrics = check.metrics();

        assert_eq!(metrics.total_graph, 2);
        assert_eq!(metrics.total_ast, 2);
        assert_eq!(metrics.total_missing, 1);
        assert_eq!(metrics.total_extra, 1);
    }

    #[test]
    fn test_check_regression() {
        let baseline = DivergenceMetrics {
            total_missing: 2,
            total_extra: 1,
            ..Default::default()
        };
        let current = DivergenceMetrics {
            total_missing: 5,
            total_extra: 1,
            ..Default::default()
        };

        let regressions = check_regression(&current, &baseline);
        assert_eq!(regressions.len(), 1);
        assert!(regressions[0].contains("Missing findings increased"));
    }
}
