#!/usr/bin/env python3
"""
Learn detection patterns from positive/negative file pairs.

Usage:
    python3 scripts/learn_from_pairs.py \
        corpus/targets/ts_eval_injection_positive.ts \
        corpus/targets/ts_eval_injection_negative.ts

    python3 scripts/learn_from_pairs.py --all corpus/targets/
"""

import argparse
import re
import sys
from pathlib import Path


def extract_taint_flows(source):
    """Extract potential taint sources from code."""
    sources = []
    patterns = [
        r"req\.body", r"req\.query", r"req\.params", r"req\.headers",
        r"request\.body", r"request\.args", r"request\.form",
        r"\binput\b", r"args\[", r"argv\[",
        r"process\.env", r"std::env",
        r"\buser\b", r"\bquery\b", r"\bparam\b",
    ]
    for p in patterns:
        if re.search(p, source):
            sources.append(p)
    return sources


def extract_sinks(source):
    """Extract sink functions from code."""
    sinks = []
    patterns = [
        r"\beval\b", r"\bexec\b", r"\bsystem\b", r"\bspawn\b",
        r"\bFunction\(", r"subprocess", r"Command::new",
        r"\.execute\b", r"\.query\b", r"raw_query",
        r"\bread_to_string\b", r"\bwrite\b", r"\bopen\b",
        r"\bfetch\b", r"\bhttp\b",
        r"innerHTML", r"outerHTML", r"document\.write",
    ]
    for p in patterns:
        if re.search(p, source):
            sinks.append(p)
    return sinks


def extract_sanitizers(positive, negative):
    """Extract sanitizers by comparing positive and negative."""
    sanitizers = []
    patterns = [
        r"\bsanitize\b", r"\bvalidate\b", r"\bescape\b", r"\bencode\b",
        r"\bparse\b", r"\bcheck\b", r"\bverify\b", r"\bclean\b",
        r"\bfilter\b", r"\bwhitelist\b", r"\ballowlist\b",
        r"\bbind\b", r"\bprepare\b",
    ]
    for p in patterns:
        if re.search(p, negative) and not re.search(p, positive):
            sanitizers.append(p)
    return sanitizers


def extract_secrets(source):
    """Extract hardcoded secret patterns."""
    secrets = []
    patterns = [
        r"sk_live_[a-zA-Z0-9]+", r"pk_live_[a-zA-Z0-9]+",
        r"AKIA[A-Z0-9]+", r"ghp_[a-zA-Z0-9]+",
        r"password\s*=\s*['\"][^'\"]+['\"]",
        r"secret\s*=\s*['\"][^'\"]+['\"]",
        r"api[_-]?key\s*=\s*['\"][^'\"]+['\"]",
    ]
    for p in patterns:
        if re.search(p, source):
            secrets.append(p)
    return secrets


def learn_from_pair(positive_path, negative_path):
    """Learn patterns from a positive/negative pair."""
    positive = positive_path.read_text()
    negative = negative_path.read_text()

    results = {
        "pair": f"{positive_path.name} / {negative_path.name}",
        "taint": None,
        "temporal": None,
        "secrets": None,
    }

    # Taint flows
    sources = extract_taint_flows(positive)
    sinks = extract_sinks(positive)
    sanitizers = extract_sanitizers(positive, negative)

    if sources and sinks:
        results["taint"] = {
            "sources": sources,
            "sinks": sinks,
            "sanitizers": sanitizers,
        }

    # Secrets
    secrets = extract_secrets(positive)
    if secrets:
        results["secrets"] = secrets

    return results


def main():
    parser = argparse.ArgumentParser(description="Learn patterns from corpus pairs")
    parser.add_argument("positive", nargs="?", help="Positive (buggy) file")
    parser.add_argument("negative", nargs="?", help="Negative (fixed) file")
    parser.add_argument("--all", action="store_true", help="Learn from all pairs in directory")
    parser.add_argument("--corpus-dir", default="corpus/targets", help="Corpus directory")
    parser.add_argument("--output", help="Output directory for learned rules")
    args = parser.parse_args()

    if args.all:
        corpus_dir = Path(args.corpus_dir)
        pairs = []
        for pos in sorted(corpus_dir.glob("*_positive.*")):
            neg = pos.with_name(pos.name.replace("_positive", "_negative"))
            if neg.exists():
                pairs.append((pos, neg))
    elif args.positive and args.negative:
        pairs = [(Path(args.positive), Path(args.negative))]
    else:
        parser.error("Specify files or use --all")

    output_dir = Path(args.output) if args.output else Path("learned_rules")
    output_dir.mkdir(parents=True, exist_ok=True)

    all_taint = []
    all_secrets = []

    for pos, neg in pairs:
        print(f"Analyzing: {pos.name}")
        result = learn_from_pair(pos, neg)

        if result["taint"]:
            all_taint.append(result)
            print(f"  Taint: {len(result['taint']['sources'])} sources → {len(result['taint']['sinks'])} sinks")

        if result["secrets"]:
            all_secrets.append(result)
            print(f"  Secrets: {len(result['secrets'])} patterns")

    # Generate taint rules
    if all_taint:
        taint_toml = "# Auto-generated taint rules\n\n"
        for i, t in enumerate(all_taint):
            sources = "|".join(t["taint"]["sources"])
            sinks = "|".join(t["taint"]["sinks"])
            taint_toml += f"""[[rules]]
id = "LEARNED_{i+1}"
source = "{sources}"
sink = "{sinks}"
severity = "warning"
observation = "Learned from {t['pair']}"
improvement = "Apply sanitization"

"""

        taint_path = output_dir / "learned_taint.toml"
        taint_path.write_text(taint_toml)
        print(f"\nWrote {len(all_taint)} taint rules to {taint_path}")

    print(f"\nDone. Review learned_rules/ and merge into taint_rules.toml")


if __name__ == "__main__":
    main()
