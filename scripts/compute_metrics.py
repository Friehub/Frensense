#!/usr/bin/env python3
"""Compute precision/recall metrics from classified TP/FP labels.

Usage:
    python scripts/compute_metrics.py                    # All repos
    python scripts/compute_metrics.py --repo axum        # Specific repo
    python scripts/compute_metrics.py --json             # JSON output
    python scripts/compute_metrics.py --by-rule          # Per-rule breakdown
"""

import json, os, sys, argparse
from pathlib import Path
from collections import defaultdict

GROUND_TRUTH_DIR = Path("corpus/ground_truth")


def load_all_labels() -> dict:
    """Load labels from all repos. Returns {repo_name: {finding_id: label_info}}."""
    all_labels = {}
    if not GROUND_TRUTH_DIR.exists():
        return all_labels

    for f in GROUND_TRUTH_DIR.glob("*_labels.json"):
        repo_name = f.stem.replace("_labels", "")
        with open(f) as fh:
            data = json.load(fh)
        all_labels[repo_name] = {
            item["_id"]: item for item in data.get("findings", [])
        }
    return all_labels


def compute_metrics(labels: dict) -> dict:
    """Compute aggregate metrics."""
    tp = sum(1 for v in labels.values() if v.get("label") == "tp")
    fp = sum(1 for v in labels.values() if v.get("label") == "fp")
    unlabeled = sum(1 for v in labels.values() if v.get("label") not in ("tp", "fp"))
    total = len(labels)

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0

    return {
        "true_positives": tp,
        "false_positives": fp,
        "unlabeled": unlabeled,
        "total": total,
        "precision": precision,
        "fp_rate": 1.0 - precision if (tp + fp) > 0 else 0.0,
    }


def compute_by_rule(labels: dict) -> dict:
    """Compute per-rule metrics."""
    by_rule = defaultdict(lambda: {"tp": 0, "fp": 0, "unlabeled": 0})
    for item in labels.values():
        rule = item.get("rule_id", "unknown")
        label = item.get("label", "unlabeled")
        if label == "tp":
            by_rule[rule]["tp"] += 1
        elif label == "fp":
            by_rule[rule]["fp"] += 1
        else:
            by_rule[rule]["unlabeled"] += 1

    for rule, counts in by_rule.items():
        total = counts["tp"] + counts["fp"]
        counts["precision"] = counts["tp"] / total if total > 0 else 0.0

    return dict(by_rule)


def print_report(all_labels: dict, flat_labels: dict, by_rule: bool = False, json_output: bool = False):
    """Print metrics report."""
    aggregate = compute_metrics(flat_labels)

    if json_output:
        output = {"aggregate": aggregate}
        if by_rule:
            output["by_rule"] = compute_by_rule(flat_labels)
        print(json.dumps(output, indent=2))
        return

    print(f"\n{'='*72}")
    print(f"  FRENSENSE TP/FP METRICS REPORT")
    print(f"{'='*72}")
    print(f"  Total Findings:  {aggregate['total']}")
    print(f"  True Positives:  {aggregate['true_positives']}")
    print(f"  False Positives: {aggregate['false_positives']}")
    print(f"  Unlabeled:       {aggregate['unlabeled']}")
    print(f"  Precision:       {aggregate['precision']:.1%}")
    print(f"  FP Rate:         {aggregate['fp_rate']:.1%}")

    if by_rule:
        rules = compute_by_rule(flat_labels)
        print(f"\n  {'Rule':<35} {'TP':>4} {'FP':>4} {'Prec':>8}")
        print(f"  {'─'*35} {'─'*4} {'─'*4} {'─'*8}")
        for rule_id in sorted(rules.keys()):
            r = rules[rule_id]
            print(f"  {rule_id:<35} {r['tp']:>4} {r['fp']:>4} {r['precision']:>7.1%}")

    # Per-repo breakdown
    print(f"\n  {'Repo':<20} {'TP':>4} {'FP':>4} {'Prec':>8}")
    print(f"  {'─'*20} {'─'*4} {'─'*4} {'─'*8}")
    for repo_name, labels in all_labels.items():
        rm = compute_metrics(labels)
        print(f"  {repo_name:<20} {rm['true_positives']:>4} {rm['false_positives']:>4} {rm['precision']:>7.1%}")

    print(f"{'='*72}\n")


def main():
    parser = argparse.ArgumentParser(description="Compute TP/FP metrics")
    parser.add_argument("--repo", help="Filter to specific repo")
    parser.add_argument("--json", action="store_true", help="JSON output")
    parser.add_argument("--by-rule", action="store_true", help="Per-rule breakdown")
    args = parser.parse_args()

    all_labels = load_all_labels()

    if args.repo:
        if args.repo not in all_labels:
            print(f"No labels found for repo: {args.repo}")
            print(f"Available: {', '.join(all_labels.keys())}")
            sys.exit(1)
        all_labels = {args.repo: all_labels[args.repo]}

    if not all_labels:
        print("No classified labels found. Run classify_findings.py first.")
        sys.exit(1)

    # Flatten all findings for aggregate metrics
    flat_labels = {}
    for repo_name, labels in all_labels.items():
        flat_labels.update(labels)

    print_report(all_labels, flat_labels, by_rule=args.by_rule, json_output=args.json)


if __name__ == "__main__":
    main()
