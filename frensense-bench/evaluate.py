#!/usr/bin/env python3
"""
Frensense benchmark evaluator.
Reads SARIF output and computes TP/FP/FN, precision, recall, F1
against a labels.json ground truth file.

Usage:
  python evaluate.py --labels <labels.json> --sarif <results.sarif> \
                     [--output <metrics.json>] [--threshold <float>]
"""
import argparse
import json
import os
import sys
from pathlib import Path


def load_labels(path: str) -> list[dict]:
    with open(path) as f:
        return json.load(f)


def load_sarif(path: str) -> list[dict]:
    """Extract findings from a SARIF file as (file, line) pairs."""
    findings = []
    with open(path) as f:
        sarif = json.load(f)
    for run in sarif.get("runs", []):
        for result in run.get("results", []):
            for loc in result.get("locations", []):
                pl = loc.get("physicalLocation", {})
                artifact = pl.get("artifactLocation", {}).get("uri", "")
                line = pl.get("region", {}).get("startLine", 0)
                rule = result.get("ruleId", "")
                findings.append({"file": artifact, "line": line, "rule": rule})
    return findings


def load_json_findings(path: str) -> list[dict]:
    """Extract findings from Frensense JSON output."""
    findings = []
    with open(path) as f:
        data = json.load(f)
    # Support both array-of-findings and {findings: [...]} envelopes
    items = data if isinstance(data, list) else data.get("findings", [])
    for item in items:
        findings.append({
            "file": item.get("file_path", item.get("file", "")),
            "line": item.get("line", 0),
            "rule": item.get("rule_id", item.get("rule", "")),
        })
    return findings


def normalise_file(path: str) -> str:
    """Strip leading path components to get a relative-style key."""
    return Path(path).name


def evaluate(labels: list[dict], findings: list[dict], tolerance_lines: int = 5) -> dict:
    """
    Match findings to labels with ±tolerance_lines line tolerance.
    Returns TP, FP, FN counts and the derived metrics.
    """
    matched_labels = set()
    matched_findings = set()

    label_index: dict[str, list[tuple[int, int]]] = {}
    for i, label in enumerate(labels):
        key = normalise_file(label["file"])
        label_index.setdefault(key, []).append((i, label["line"]))

    for j, finding in enumerate(findings):
        key = normalise_file(finding["file"])
        if key not in label_index:
            continue
        for i, label_line in label_index[key]:
            if i in matched_labels:
                continue
            if abs(finding["line"] - label_line) <= tolerance_lines:
                matched_labels.add(i)
                matched_findings.add(j)
                break

    tp = len(matched_labels)
    fp = len(findings) - len(matched_findings)
    fn = len(labels) - len(matched_labels)

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
    recall    = tp / (tp + fn) if (tp + fn) > 0 else 0.0
    f1        = (2 * precision * recall / (precision + recall)
                 if (precision + recall) > 0 else 0.0)

    return {
        "tp": tp, "fp": fp, "fn": fn,
        "precision": round(precision, 4),
        "recall":    round(recall,    4),
        "f1":        round(f1,        4),
        "total_findings": len(findings),
        "total_labels":   len(labels),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Frensense benchmark evaluator")
    parser.add_argument("--labels",    required=True,  help="Ground truth labels.json")
    parser.add_argument("--sarif",     default=None,   help="Frensense SARIF output")
    parser.add_argument("--json-file", default=None,   help="Frensense JSON output")
    parser.add_argument("--output",    default=None,   help="Write metrics to JSON file")
    parser.add_argument("--threshold", default=0.40,   type=float, help="Threshold used (for record only)")
    parser.add_argument("--tolerance", default=5,      type=int,   help="Line tolerance for match (default 5)")
    args = parser.parse_args()

    labels = load_labels(args.labels)
    print(f"Loaded {len(labels)} ground truth labels from {args.labels}")

    if args.sarif:
        findings = load_sarif(args.sarif)
        source = args.sarif
    elif args.json_file:
        findings = load_json_findings(args.json_file)
        source = args.json_file
    else:
        print("ERROR: provide --sarif or --json-file", file=sys.stderr)
        sys.exit(1)

    print(f"Loaded {len(findings)} findings from {source}")

    metrics = evaluate(labels, findings, args.tolerance)
    metrics["threshold"] = args.threshold
    metrics["source"]    = source

    print()
    print(f"  Threshold : {args.threshold}")
    print(f"  TP        : {metrics['tp']}")
    print(f"  FP        : {metrics['fp']}")
    print(f"  FN        : {metrics['fn']}")
    print(f"  Precision : {metrics['precision']:.4f}")
    print(f"  Recall    : {metrics['recall']:.4f}")
    print(f"  F1        : {metrics['f1']:.4f}")

    if args.output:
        with open(args.output, "w") as f:
            json.dump(metrics, f, indent=2)
        print(f"\nMetrics written to {args.output}")


if __name__ == "__main__":
    main()
