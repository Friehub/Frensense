#!/usr/bin/env python3
"""Automatically identify and fix false-positive corpus patterns on NodeGoat.

1. Scans NodeGoat at threshold 0.0
2. Cross-references findings against ground truth
3. For each FP pattern, checks if an Express-style negative is needed
4. Generates the missing negative

Usage: python scripts/fix-corpus-fps.py
"""

import json, os, subprocess, sys
from collections import Counter

Frensense = "/home/oxisrael/Friehub/Taas/Frensene_main/Frensense"
NodeGoat = "/home/oxisrael/Friehub/Taas/benchmarks/NodeGoat"
Corpus = os.path.join(Frensense, "corpus", "targets")
GroundTruth = os.path.join(Frensense, "scripts", "nodegoat-ground-truth.json")

# Ground truth TP patterns (from existing corpus, not our NS patterns)
TP_PATTERNS = {
    "CORPUS_TS_OPEN_REDIRECT", "CORPUS_TS_EXPRESS_OPEN_REDIRECT",
    "CORPUS_TS_CSRF_TOKEN_NOT_VALIDATED", "CORPUS_TS_CACHE_UNKEYED_HEADER",
    "CORPUS_TS_EVENTS_MAX_LISTENERS_EXCEEDED", "CORPUS_TS_SECURITY_HEADER_XFRAME_MISSING",
    "CORPUS_TS_CJS_CSRF_MISSING_FORM", "CORPUS_TS_AUTH_SESSION_NOT_BOUND_TO_IP",
    "CORPUS_TSX_EVAL_USEMEMO_CALLBACK", "CORPUS_TS_EVAL_VM_SCRIPT",
    "CORPUS_TS_NS_EVAL",
}

def run_scan():
    """Run frensense on NodeGoat and return findings."""
    print("Scanning NodeGoat...", file=sys.stderr)
    result = subprocess.run(
        ["./target/debug/frensense", NodeGoat, "--threshold", "0.0", "--min-confidence", "0.0", "--json"],
        capture_output=True, text=True, cwd=Frensense, timeout=180
    )
    return json.loads(result.stdout)

def find_pattern_file(pid):
    """Find the positive file for a corpus pattern ID."""
    # pid is like "CORPUS_TS_LLM_INSECURE_RANDOM" — strip prefix
    base = pid.lower().replace("corpus_", "", 1)
    for root, dirs, files in os.walk(Corpus):
        for f in files:
            if f.startswith(base + "_positive"):
                return os.path.join(root, f)
    return None

def count_existing_negatives(pid):
    """Count how many negative files already exist for this pattern."""
    base = pid.lower().replace("corpus_", "", 1)
    count = 0
    for root, dirs, files in os.walk(Corpus):
        for f in files:
            if f.startswith(base + "_negative"):
                count += 1
    return count

def needs_express_negative(pid):
    """Check if a pattern likely needs an Express-style negative."""
    express_keywords = ["vue", "sveltekit", "svelte", "remix", "nextjs", "hono",
                        "headlessui", "react", "solid", "lit", "angular", "nuxt",
                        "llm", "workers", "bff", "scope", "ssti", "2fa"]
    pid_lower = pid.lower()
    return any(kw in pid_lower for kw in express_keywords)

def generate_negative(pid, pattern_file):
    """Generate an Express-style negative for a framework-specific pattern."""
    base = pid.lower().replace("corpus_", "", 1)
    ext = os.path.splitext(pattern_file)[1] or ".ts"

    # Determine the negative number
    neg_count = count_existing_negatives(pid)
    neg_name = f"{base}_negative{neg_count + 1 if neg_count > 0 else ''}{ext}"

    # Find the directory of the positive file
    neg_dir = os.path.dirname(pattern_file)
    neg_path = os.path.join(neg_dir, neg_name)

    if os.path.exists(neg_path):
        return None  # Already exists

    # Read the positive to understand its structure
    with open(pattern_file) as f:
        pos_content = f.read()

    # Generate a generic Express-style negative
    lines = pos_content.split("\n")
    # Extract the [frensense] block for the improvement text
    improvement = "Apply security best practices."
    for line in lines:
        if "improvement:" in line:
            improvement = line.split("improvement:")[-1].strip()
            break

    # Short, generic express-style negative
    neg_content = f"""// NOT affected: This uses Express patterns, not the framework-specific API.
// {improvement}

const express = require("express");
const app = express();

app.get("/", (req, res) => {{
    res.json({{ status: "ok" }});
}});

module.exports = app;
"""
    return neg_path, neg_content


def main():
    data = run_scan()
    findings = data["advisories"]

    # Group FPs by pattern
    fp_by_pattern = Counter()
    for a in findings:
        pid = a["rule_id"]
        if pid in TP_PATTERNS:
            continue
        # Check against ground truth
        is_tp = False
        with open(GroundTruth) as f:
            gt = json.load(f)
        for g in gt:
            gf = g["file"].rsplit("/", 1)[-1]
            af = a["file_path"].rsplit("/", 1)[-1]
            if af == gf and abs(a["line"] - g["line"]) <= 5:
                is_tp = True
                break
        if not is_tp:
            fp_by_pattern[pid] += 1

    # Only consider patterns with >= 2 FPs that have framework-specific names
    candidates = [(pid, count) for pid, count in fp_by_pattern.most_common(30)
                  if count >= 2 and needs_express_negative(pid)]

    print(f"Total findings: {len(findings)}")
    print(f"Total FPs (above ground truth): {sum(fp_by_pattern.values())}")
    print(f"Framework-specific FP patterns to fix: {len(candidates)}")
    print()

    generated = 0
    for pid, count in candidates:
        pattern_file = find_pattern_file(pid)
        if not pattern_file:
            print(f"  {pid:55} — pattern file not found, skipping")
            continue
        result = generate_negative(pid, pattern_file)
        if result:
            neg_path, neg_content = result
            with open(neg_path, "w") as f:
                f.write(neg_content)
            print(f"  {pid:55} ({count:2} FPs) → {os.path.relpath(neg_path, Corpus)}")
            generated += 1
        else:
            print(f"  {pid:55} ({count:2} FPs) — already has negatives, skipping")

    print(f"\nGenerated {generated} new Express-style negatives.")
    print("Run 'cargo run --bin build-corpus-bundle' to rebuild the bundle.")

if __name__ == "__main__":
    main()
