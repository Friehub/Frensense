#!/usr/bin/env python3
"""Classify Frensense findings as TP/FP on real open-source codebases.

Usage:
    python scripts/classify_findings.py                    # Interactive classification
    python scripts/classify_findings.py --scan-only        # Just scan, don't classify
    python scripts/classify_findings.py --repo axum        # Scan specific repo only
    python scripts/classify_findings.py --from-scan FILE   # Classify from existing scan file

Ground truth stored in: corpus/ground_truth/{repo}_labels.json
"""

import subprocess, json, os, sys, argparse, time, hashlib
from pathlib import Path
from collections import defaultdict

FRENSENSE_BIN = Path("./target/release/frensense")
WORKDIR = Path("/tmp/frensense-oss-validation")
GROUND_TRUTH_DIR = Path("corpus/ground_truth")

REPOS = {
    "axum": {
        "url": "https://github.com/tokio-rs/axum.git",
        "scan": "axum/src",
    },
    "actix-web": {
        "url": "https://github.com/actix/actix-web.git",
        "scan": "actix-web/src",
    },
    "hyper": {
        "url": "https://github.com/hyperium/hyper.git",
        "scan": "src",
    },
    "express": {
        "url": "https://github.com/expressjs/express.git",
        "scan": "lib",
    },
    "fastify": {
        "url": "https://github.com/fastify/fastify.git",
        "scan": "lib",
    },
}


def finding_id(finding: dict, index: int = 0) -> str:
    """Generate a stable ID for a finding based on its content."""
    key = f"{finding['rule_id']}:{finding['file_path']}:{finding['line']}:{finding.get('observation', '')}:{index}"
    return hashlib.sha256(key.encode()).hexdigest()[:12]


def run_frensense(path: Path) -> list:
    """Run frensense on path, return list of advisories."""
    try:
        result = subprocess.run(
            [str(FRENSENSE_BIN), str(path), "--json"],
            capture_output=True, text=True, timeout=600,
        )
    except subprocess.TimeoutExpired:
        print(f"  [TIMEOUT] skipped")
        return []

    if result.returncode not in (0, 1):
        print(f"  [ERR {result.returncode}] stderr: {result.stderr[:200]}")
        return []

    stdout = result.stdout.strip()
    if not stdout:
        return []

    try:
        data = json.loads(stdout)
        return data.get("advisories", [])
    except json.JSONDecodeError:
        print(f"  [PARSE ERR] first 200 chars: {stdout[:200]}")
        return []


def scan_repo(name: str, repo: dict) -> list:
    """Scan a repo and return findings with IDs."""
    scan_path = WORKDIR / name / repo["scan"]
    if not scan_path.exists():
        print(f"[scan] {name} — path missing: {scan_path}")
        return []

    print(f"[scan] {name} — {scan_path} ...")
    findings = run_frensense(scan_path)
    print(f"  -> {len(findings)} findings")

    for i, f in enumerate(findings):
        f["_id"] = finding_id(f, i)
        f["_repo"] = name

    return findings


def load_existing_labels(repo_name: str) -> dict:
    """Load existing labels for a repo."""
    label_file = GROUND_TRUTH_DIR / f"{repo_name}_labels.json"
    if not label_file.exists():
        return {}
    with open(label_file) as fh:
        data = json.load(fh)
    return {item["_id"]: item for item in data.get("findings", [])}


def save_labels(repo_name: str, findings: list, labels: dict):
    """Save labels for a repo."""
    GROUND_TRUTH_DIR.mkdir(parents=True, exist_ok=True)
    label_file = GROUND_TRUTH_DIR / f"{repo_name}_labels.json"

    labeled_findings = []
    for f in findings:
        fid = f["_id"]
        if fid in labels:
            labeled = {**f, **labels[fid]}
        else:
            labeled = {**f, "label": "unlabeled"}
        labeled_findings.append(labeled)

    data = {
        "repo": repo_name,
        "scan_date": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "total_findings": len(labeled_findings),
        "labeled_count": sum(1 for f in labeled_findings if f["label"] != "unlabeled"),
        "findings": labeled_findings,
    }

    with open(label_file, "w") as fh:
        json.dump(data, fh, indent=2)
    print(f"[save] {label_file} ({data['labeled_count']}/{data['total_findings']} labeled)")


def interactive_classify(findings: list, existing_labels: dict) -> dict:
    """Interactive classification of findings."""
    labels = dict(existing_labels)
    unlabeled = [f for f in findings if f["_id"] not in labels]

    if not unlabeled:
        print("All findings already labeled!")
        return labels

    print(f"\n{len(unlabeled)} unlabeled findings. Classification commands:")
    print("  t = TP (true positive)")
    print("  f = FP (false positive)")
    print("  s = skip")
    print("  q = quit and save")
    print("  a = auto-label all remaining as FP (for quick baseline)\n")

    for i, finding in enumerate(unlabeled):
        rel_path = finding["file_path"]
        for repo_name in REPOS:
            rel_path = rel_path.replace(str(WORKDIR / repo_name) + "/", "")

        print(f"[{i+1}/{len(unlabeled)}] {finding['rule_id']} @ {rel_path}:{finding['line']}")
        print(f"  Observation: {finding.get('observation', 'N/A')[:120]}")
        if finding.get("improvement"):
            print(f"  Suggestion:  {finding['improvement'][:120]}")

        while True:
            cmd = input("  Label [t/f/s/q/a]: ").strip().lower()
            if cmd == "t":
                labels[finding["_id"]] = {"label": "tp"}
                break
            elif cmd == "f":
                labels[finding["_id"]] = {"label": "fp"}
                break
            elif cmd == "s":
                break
            elif cmd == "q":
                return labels
            elif cmd == "a":
                for f in unlabeled[i:]:
                    if f["_id"] not in labels:
                        labels[f["_id"]] = {"label": "fp"}
                print(f"  Auto-labeled {len(unlabeled) - i} remaining as FP")
                return labels
            else:
                print("  Unknown command. Use t/f/s/q/a")

    return labels


def compute_metrics(labels: dict) -> dict:
    """Compute precision/recall metrics from labels."""
    tp = sum(1 for v in labels.values() if v["label"] == "tp")
    fp = sum(1 for v in labels.values() if v["label"] == "fp")
    total_labeled = tp + fp
    unlabeled = sum(1 for v in labels.values() if v["label"] == "unlabeled")

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
    false_positive_rate = fp / (tp + fp) if (tp + fp) > 0 else 0.0

    return {
        "true_positives": tp,
        "false_positives": fp,
        "total_labeled": total_labeled,
        "unlabeled": unlabeled,
        "precision": precision,
        "false_positive_rate": false_positive_rate,
    }


def print_metrics(metrics: dict, by_rule: dict = None):
    """Print metrics summary."""
    print(f"\n{'='*72}")
    print(f"  TP/FP CLASSIFICATION SUMMARY")
    print(f"{'='*72}")
    print(f"  True Positives:   {metrics['true_positives']}")
    print(f"  False Positives:  {metrics['false_positives']}")
    print(f"  Precision:        {metrics['precision']:.1%}")
    print(f"  FP Rate:          {metrics['false_positive_rate']:.1%}")
    print(f"  Unlabeled:        {metrics['unlabeled']}")

    if by_rule:
        print(f"\n  {'Rule':<33} {'TP':>4} {'FP':>4} {'Precision':>10}")
        print(f"  {'─'*33} {'─'*4} {'─'*4} {'─'*10}")
        for rule_id in sorted(by_rule.keys()):
            r = by_rule[rule_id]
            prec = r["tp"] / (r["tp"] + r["fp"]) if (r["tp"] + r["fp"]) > 0 else 0.0
            print(f"  {rule_id:<33} {r['tp']:>4} {r['fp']:>4} {prec:>9.1%}")

    print(f"{'='*72}\n")


def main():
    parser = argparse.ArgumentParser(description="Classify Frensense findings as TP/FP")
    parser.add_argument("--scan-only", action="store_true", help="Just scan, don't classify interactively")
    parser.add_argument("--repo", help="Scan specific repo only")
    parser.add_argument("--from-scan", help="Classify from existing scan JSON file")
    parser.add_argument("--auto-fp", action="store_true", help="Auto-label all as FP (baseline)")
    args = parser.parse_args()

    GROUND_TRUTH_DIR.mkdir(parents=True, exist_ok=True)

    if args.from_scan:
        with open(args.from_scan) as fh:
            all_findings = json.load(fh)
        for f in all_findings:
            f["_id"] = finding_id(f)
            if "_repo" not in f:
                f["_repo"] = "unknown"
    else:
        repos_to_scan = {args.repo: REPOS[args.repo]} if args.repo else REPOS
        all_findings = []
        for name, repo in repos_to_scan.items():
            findings = scan_repo(name, repo)
            all_findings.extend(findings)

    # Group by repo
    by_repo = defaultdict(list)
    for f in all_findings:
        by_repo[f["_repo"]].append(f)

    if args.scan_only:
        scan_file = GROUND_TRUTH_DIR / "latest_scan.json"
        with open(scan_file, "w") as fh:
            json.dump(all_findings, fh, indent=2)
        print(f"\n[save] Scan results: {scan_file} ({len(all_findings)} findings)")
        return

    # Classify per repo
    all_labels = {}
    for repo_name, findings in by_repo.items():
        existing = load_existing_labels(repo_name)
        print(f"\n--- {repo_name}: {len(findings)} findings ---")

        if args.auto_fp:
            labels = {f["_id"]: {"label": "fp"} for f in findings}
        else:
            labels = interactive_classify(findings, existing)

        all_labels.update(labels)
        save_labels(repo_name, findings, labels)

    # Compute and print metrics
    metrics = compute_metrics(all_labels)

    by_rule = defaultdict(lambda: {"tp": 0, "fp": 0})
    for fid, label_info in all_labels.items():
        finding = next((f for f in all_findings if f["_id"] == fid), None)
        if finding:
            rule = finding["rule_id"]
            if label_info["label"] == "tp":
                by_rule[rule]["tp"] += 1
            elif label_info["label"] == "fp":
                by_rule[rule]["fp"] += 1

    print_metrics(metrics, by_rule)


if __name__ == "__main__":
    main()
