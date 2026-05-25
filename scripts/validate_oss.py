#!/usr/bin/env python3
"""Validate GenSense rules against real open-source codebases.

Usage:
    python scripts/validate_oss.py                           # from pre-built binary
    python scripts/validate_oss.py --build                   # rebuild first
    python scripts/validate_oss.py --quick                   # only scan repo subdirs
    python scripts/validate_oss.py --clean                   # remove repos after
    python scripts/validate_oss.py --skip-clone              # skip clone, use existing

Clones repos shallowly, runs gensense on each, then prints a per-rule
summary with sample findings for FP/TP review.
"""

import subprocess, json, os, sys, argparse, shutil, textwrap
from pathlib import Path
from collections import defaultdict

GENSENSE_BIN = Path("./target/release/gensense")
WORKDIR = Path("/tmp/gensense-oss-validation")

REPOS = {
    "express": {
        "url": "https://github.com/expressjs/express.git",
        "scan": "lib",
    },
    "serde": {
        "url": "https://github.com/serde-rs/serde.git",
        "scan": ".",
    },
    "tokio": {
        "url": "https://github.com/tokio-rs/tokio.git",
        "scan": "tokio/src",
    },
}

# Rules we're most interested in validating (FP assessment).
# Leave empty to report all rules.
TARGET_RULES = set()


def build_gensense():
    print("[build] cargo build --release ...")
    r = subprocess.run(["cargo", "build", "--release"], capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stderr)
        sys.exit(1)
    print("[build] done")


def clone_repos(force=False):
    for name, repo in REPOS.items():
        dest = WORKDIR / name
        if dest.exists():
            if force:
                shutil.rmtree(dest)
            else:
                print(f"[clone] {name} — exists, skip")
                continue
        print(f"[clone] {name} — cloning {repo['url']} ...")
        r = subprocess.run(
            ["git", "clone", "--depth", "1", repo["url"], str(dest)],
            capture_output=True, text=True,
        )
        if r.returncode != 0:
            print(f"  WARN: {r.stderr[:200]}")
        else:
            size = sum(f.stat().st_size for f in dest.rglob("*") if f.is_file()) // 1024
            print(f"  OK ({size} KB)")


def run_gensense(path: Path) -> list:
    """Run gensense on path, return list of advisories."""
    try:
        result = subprocess.run(
            [str(GENSENSE_BIN), str(path), "--json"],
            capture_output=True, text=True, timeout=180,
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


def print_summary(all_findings, elapsed):
    by_rule = defaultdict(list)
    for f in all_findings:
        by_rule[f["rule_id"]].append(f)

    total = len(all_findings)

    print(f"\n{'='*72}")
    print(f"  OSS VALIDATION — {total} findings in {len(by_rule)} rules ({elapsed:.0f}s)")
    print(f"{'='*72}")

    if not total:
        print("  No findings. Check binary path or repo structure.")
        return

    # Summary table header
    print(f"  {'Rule':<33} {'Sev':<9} {'Count':<6}  Repos")
    print(f"  {'─'*33} {'─'*9} {'─'*6}  {'─'*20}")

    for rule_id in sorted(by_rule.keys()):
        findings = by_rule[rule_id]
        severity = findings[0].get("severity", "?")
        repos_affected = set()
        file_lines = []
        for f in findings:
            for repo_name in REPOS:
                if repo_name in f["file_path"]:
                    repos_affected.add(repo_name)
            rel_path = f["file_path"]
            for repo_name in REPOS:
                rel_path = rel_path.replace(str(WORKDIR / repo_name) + "/", f"{repo_name}/")
            file_lines.append((rel_path, f["line"], f["observation"]))

        print(f"  {rule_id:<33s} {severity:<9s} {len(findings):<6d}  {', '.join(sorted(repos_affected))}")

        for rpath, line, obs in file_lines[:3]:
            short = obs[:90]
            print(f"    {rpath}:{line}  {short}")
        if len(findings) > 3:
            extra = len(findings) - 3
            print(f"    ... +{extra} more")


def main():
    parser = argparse.ArgumentParser(description="Validate GenSense rules on OSS codebases")
    parser.add_argument("--build", action="store_true", help="Rebuild gensense binary first")
    parser.add_argument("--clean", action="store_true", help="Remove cloned repos after run")
    parser.add_argument("--quick", action="store_true", help="Only scan repo subdirs")
    parser.add_argument("--skip-clone", action="store_true", help="Skip cloning, use existing")
    parser.add_argument("--rule", action="append", help="Filter to specific rule(s)", default=None)
    args = parser.parse_args()

    if not GENSENSE_BIN.exists() or args.build:
        build_gensense()

    if not GENSENSE_BIN.exists():
        print("FATAL: binary not found. Build with --build first.")
        sys.exit(1)

    if not args.skip_clone:
        clone_repos(force=False)

    import time
    t0 = time.time()
    all_findings = []

    for name, repo in REPOS.items():
        scan_path = WORKDIR / name / repo["scan"]
        if not scan_path.exists():
            print(f"[scan] {name} — path missing: {scan_path}")
            continue

        print(f"[scan] {name} — {scan_path} ...")
        findings = run_gensense(scan_path)

        if args.rule:
            findings = [f for f in findings if f["rule_id"] in args.rule]

        print(f"  -> {len(findings)} findings")
        all_findings.extend(findings)

    elapsed = time.time() - t0
    print_summary(all_findings, elapsed)

    if args.clean:
        print("[clean] removing repos ...")
        shutil.rmtree(WORKDIR, ignore_errors=True)


if __name__ == "__main__":
    main()
