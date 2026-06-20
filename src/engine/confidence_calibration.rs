// SPDX-License-Identifier: MIT

//! Confidence calibration using Platt scaling.
//!
//! Maps raw similarity scores to calibrated probabilities using logistic regression
//! trained on labeled TP/FP data.

use serde::{Deserialize, Serialize};

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
        // Initialize with prior probabilities
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

/// Load calibration parameters from a JSON file.
pub fn load_calibration(path: &std::path::Path) -> Option<CalibrationParams> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Save calibration parameters to a JSON file.
pub fn save_calibration(params: &CalibrationParams, path: &std::path::Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(params).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_params_default() {
        let params = CalibrationParams::default();
        // Default params with a=0, b=0 should output 0.5 for any input
        let result = params.calibrate(0.5);
        assert!((result - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_train_calibration() {
        // Perfect separation: scores > 0.5 are positive, <= 0.5 are negative
        let scores = vec![0.2, 0.3, 0.4, 0.6, 0.7, 0.8];
        let labels = vec![false, false, false, true, true, true];

        let params = CalibrationParams::train(&scores, &labels);
        assert!(params.accuracy > 0.5);
        assert!(params.calibrate(0.8) > params.calibrate(0.2));
    }
}
