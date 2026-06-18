#!/usr/bin/env python3
"""
Recall Validation

Scans known-vulnerable codebases and measures what percentage of known CVEs
FrenSense detects via corpus pattern matching.

Usage:
    python3 scripts/validate_recall.py --repo /tmp/axum --cve-list cves.json
    python3 scripts/validate_recall.py --repo /tmp/axum --auto-detect
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path


def run_frensense(repo_path: str, json_output: bool = True) -> dict:
    """Run frensense on a repo and return findings."""
    cmd = ["./target/release/frensense", repo_path, "--json", "--severity", "info"]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
    if json_output:
        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError:
            return {"advisories": [], "error": "Failed to parse JSON"}
    return {"stdout": result.stdout, "stderr": result.stderr}


def main():
    parser = argparse.ArgumentParser(description="Validate recall on known-vulnerable repos")
    parser.add_argument("--repo", required=True, help="Path to repository to scan")
    parser.add_argument("--cve-list", help="JSON file with list of CVEs to check")
    parser.add_argument("--auto-detect", action="store_true",
                        help="Auto-detect CVEs from git log")
    args = parser.parse_args()

    repo_path = Path(args.repo)
    if not repo_path.exists():
        print(f"Repository not found: {repo_path}")
        return 1

    print(f"Scanning {repo_path}...")
    result = run_frensense(str(repo_path))

    advisories = result.get("advisories", [])
    print(f"Found {len(advisories)} findings")

    # Count by rule
    from collections import Counter
    rules = Counter(a.get("rule_id", "unknown") for a in advisories)
    print("\nFindings by rule:")
    for rule, count in rules.most_common():
        print(f"  {rule}: {count}")

    # If CVE list provided, measure recall
    if args.cve_list:
        with open(args.cve_list) as f:
            cves = json.load(f)
        print(f"\nCVE recall check ({len(cves)} CVEs):")
        found = 0
        for cve in cves:
            cve_id = cve.get("id", "")
            # Check if any finding references this CVE
            matched = any(cve_id in str(a) for a in advisories)
            if matched:
                found += 1
                print(f"  DETECTED: {cve_id}")
            else:
                print(f"  MISSED:   {cve_id}")
        print(f"\nRecall: {found}/{len(cves)} ({100*found/len(cves):.1f}%)")

    # Output JSON for further analysis
    output = {
        "repo": str(repo_path),
        "total_findings": len(advisories),
        "by_rule": dict(rules),
    }
    print(f"\n{json.dumps(output, indent=2)}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
