// SPDX-License-Identifier: MIT

//! Per-category Platt scaling calibration (M5).
//!
//! Maps raw similarity scores to calibrated probabilities using logistic regression
//! trained on labeled TP/FP data, with separate parameters per vulnerability category.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Per-category calibration parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerCategoryCalibration {
    /// Global calibration parameters (fallback)
    pub global: CalibrationParams,
    /// Per-category calibration parameters
    pub categories: HashMap<String, CalibrationParams>,
}

/// Single category calibration parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationParams {
    /// Coefficient A for logistic function: P(y=1) = 1 / (1 + exp(A*x + B))
    pub a: f64,
    /// Intercept B for logistic function
    pub b: f64,
    /// Number of samples used for training
    pub n_samples: usize,
    /// Accuracy on training set
    pub accuracy: f64,
}

impl Default for CalibrationParams {
    fn default() -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            n_samples: 0,
            accuracy: 0.0,
        }
    }
}

impl PerCategoryCalibration {
    /// Calibrate a raw score using category-specific parameters.
    /// Falls back to global parameters if category not found.
    pub fn calibrate(&self, raw_score: f64, category: &str) -> f64 {
        let params = self.categories.get(category).unwrap_or(&self.global);
        params.calibrate(raw_score)
    }

    /// Calibrate using global parameters only.
    pub fn calibrate_global(&self, raw_score: f64) -> f64 {
        self.global.calibrate(raw_score)
    }

    /// Train per-category calibration parameters from labeled data.
    /// `scores_and_labels` is a map of category -> (scores, labels).
    pub fn train(scores_and_labels: &[(String, Vec<f64>, Vec<bool>)]) -> Self {
        // Train global parameters from all data
        let all_scores: Vec<f64> = scores_and_labels
            .iter()
            .flat_map(|(_, scores, _)| scores.iter().copied())
            .collect();
        let all_labels: Vec<bool> = scores_and_labels
            .iter()
            .flat_map(|(_, _, labels)| labels.iter().copied())
            .collect();
        let global = CalibrationParams::train(&all_scores, &all_labels);

        // Train per-category parameters
        let mut categories = HashMap::new();
        for (category, scores, labels) in scores_and_labels {
            if scores.len() >= 20 {
                // Minimum samples for reliable calibration
                let params = CalibrationParams::train(scores, labels);
                categories.insert(category.clone(), params);
            }
        }

        Self { global, categories }
    }

    /// Get calibration summary for reporting.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Global: a={:.4}, b={:.4}, n={}, acc={:.4}",
            self.global.a, self.global.b, self.global.n_samples, self.global.accuracy
        ));
        for (cat, params) in &self.categories {
            lines.push(format!(
                "  {}: a={:.4}, b={:.4}, n={}, acc={:.4}",
                cat, params.a, params.b, params.n_samples, params.accuracy
            ));
        }
        lines.join("\n")
    }
}

impl CalibrationParams {
    /// Calibrate a raw score to a probability using Platt scaling.
    pub fn calibrate(&self, raw_score: f64) -> f64 {
        let z = self.a * raw_score + self.b;
        1.0 / (1.0 + (-z).exp())
    }

    /// Train calibration parameters from labeled data.
    pub fn train(scores: &[f64], labels: &[bool]) -> Self {
        if scores.is_empty() || labels.is_empty() || scores.len() != labels.len() {
            return Self::default();
        }

        let n = scores.len() as f64;
        let n_pos = labels.iter().filter(|&&l| l).count() as f64;
        let n_neg = n - n_pos;

        if n_pos == 0.0 || n_neg == 0.0 {
            return Self::default();
        }

        // Platt scaling: fit logistic regression using maximum likelihood
        let mut a = 0.0;
        let mut b = ((n_neg + 1.0) / (n_pos + 1.0)).ln();

        // Gradient descent to minimize cross-entropy loss
        let learning_rate = 0.1;
        let iterations = 500;

        for _ in 0..iterations {
            let mut grad_a = 0.0;
            let mut grad_b = 0.0;

            for (x, &y) in scores.iter().zip(labels.iter()) {
                let z = a * x + b;
                let p = 1.0 / (1.0 + (-z).exp());
                let target = if y { 1.0 } else { 0.0 };

                grad_a += (p - target) * x;
                grad_b += p - target;
            }

            a -= learning_rate * grad_a / n;
            b -= learning_rate * grad_b / n;
        }

        // Calculate accuracy
        let correct = scores
            .iter()
            .zip(labels.iter())
            .filter(|(x, y)| {
                let p = 1.0 / (1.0 + (-(a * **x + b)).exp());
                (p >= 0.5) == **y
            })
            .count();
        let accuracy = correct as f64 / n;

        Self {
            a,
            b,
            n_samples: scores.len(),
            accuracy,
        }
    }
}

/// Load per-category calibration from a JSON file.
pub fn load_per_category_calibration(path: &std::path::Path) -> Option<PerCategoryCalibration> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Save per-category calibration to a JSON file.
pub fn save_per_category_calibration(
    cal: &PerCategoryCalibration,
    path: &std::path::Path,
) -> Result<(), String> {
    let json = serde_json::to_string_pretty(cal).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_category_calibration() {
        let mut cal = PerCategoryCalibration::default();
        cal.categories.insert(
            "cmd_injection".to_string(),
            CalibrationParams {
                a: 4.0,
                b: -1.5,
                n_samples: 100,
                accuracy: 0.85,
            },
        );

        // Should use category-specific params
        let result = cal.calibrate(0.5, "cmd_injection");
        assert!(result > 0.0 && result < 1.0);

        // Should fall back to global for unknown category
        let result_global = cal.calibrate(0.5, "unknown");
        assert!(result_global > 0.0 && result_global < 1.0);
    }

    #[test]
    fn test_train_per_category() {
        // Need at least 20 samples per category for reliable calibration
        let scores_injection: Vec<f64> = (0..25).map(|i| 0.3 + i as f64 * 0.02).collect();
        let labels_injection: Vec<bool> = (0..25).map(|i| i >= 10).collect();

        let scores_sql: Vec<f64> = (0..25).map(|i| 0.2 + i as f64 * 0.03).collect();
        let labels_sql: Vec<bool> = (0..25).map(|i| i >= 12).collect();

        let data = vec![
            (
                "cmd_injection".to_string(),
                scores_injection,
                labels_injection,
            ),
            ("sql_injection".to_string(), scores_sql, labels_sql),
        ];

        let cal = PerCategoryCalibration::train(&data);
        assert!(cal.global.n_samples > 0);
        assert!(cal.categories.contains_key("cmd_injection"));
        assert!(cal.categories.contains_key("sql_injection"));
    }
}
