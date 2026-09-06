#!/usr/bin/env python3
"""
Train confidence calibration parameters from labeled findings.

Usage:
    python3 scripts/train_calibration.py \
        --labels corpus/ground_truth/axum_labels.json \
        --output calibration.json
"""

import argparse
import json
import sys
from pathlib import Path


def sigmoid(z):
    """Sigmoid function."""
    return 1.0 / (1.0 + (-z).__abs__())


def train_platt_scaling(scores, labels, learning_rate=0.01, iterations=1000):
    """
    Train Platt scaling parameters using gradient descent.
    
    Args:
        scores: List of raw confidence scores
        labels: List of boolean labels (True = TP, False = FP)
        learning_rate: Learning rate for gradient descent
        iterations: Number of training iterations
    
    Returns:
        dict with 'a' and 'b' parameters
    """
    n = len(scores)
    if n == 0:
        return {"a": 0.0, "b": 0.0, "n_samples": 0, "accuracy": 0.0}
    
    n_pos = sum(labels)
    n_neg = n - n_pos
    
    if n_pos == 0 or n_neg == 0:
        return {"a": 0.0, "b": 0.0, "n_samples": n, "accuracy": 0.0}
    
    # Initialize with prior probabilities
    a = 0.0
    b = (n_neg + 1) / (n_pos + 1)  # ln(prior)
    
    # Gradient descent
    for _ in range(iterations):
        grad_a = 0.0
        grad_b = 0.0
        
        for x, y in zip(scores, labels):
            z = a * x + b
            p = 1.0 / (1.0 + (-z).__abs__())
            target = 1.0 if y else 0.0
            
            grad_a += (p - target) * x
            grad_b += p - target
        
        a -= learning_rate * grad_a / n
        b -= learning_rate * grad_b / n
    
    # Calculate accuracy
    correct = 0
    for x, y in zip(scores, labels):
        z = a * x + b
        p = 1.0 / (1.0 + (-z).__abs__())
        predicted = p >= 0.5
        if predicted == y:
            correct += 1
    
    accuracy = correct / n if n > 0 else 0.0
    
    return {
        "a": a,
        "b": b,
        "n_samples": n,
        "accuracy": accuracy,
    }


def main():
    parser = argparse.ArgumentParser(description="Train confidence calibration")
    parser.add_argument("--labels", required=True, help="Path to labels JSON file")
    parser.add_argument("--output", required=True, help="Output calibration JSON file")
    args = parser.parse_args()
    
    # Load labels
    with open(args.labels) as f:
        data = json.load(f)
    
    findings = data.get("findings", [])
    
    scores = []
    labels = []
    
    for finding in findings:
        confidence = finding.get("confidence", 0.0)
        label = finding.get("label", "").lower()
        
        if label in ("tp", "true", "true_positive"):
            scores.append(confidence)
            labels.append(True)
        elif label in ("fp", "false", "false_positive"):
            scores.append(confidence)
            labels.append(False)
    
    print(f"Loaded {len(findings)} findings, {len(scores)} labeled")
    print(f"  TP: {sum(labels)}")
    print(f"  FP: {len(labels) - sum(labels)}")
    
    # Train
    params = train_platt_scaling(scores, labels)
    
    print(f"\nCalibration parameters:")
    print(f"  a: {params['a']:.4f}")
    print(f"  b: {params['b']:.4f}")
    print(f"  n_samples: {params['n_samples']}")
    print(f"  accuracy: {params['accuracy']:.2%}")
    
    # Save
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, "w") as f:
        json.dump(params, f, indent=2)
    
    print(f"\nSaved to {output_path}")


if __name__ == "__main__":
    main()
