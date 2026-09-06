#!/usr/bin/env python3
"""Check corpus pattern completeness and generate missing sidecar .toml files.

Usage:
    python3 scripts/corpus_check.py corpus/targets/           # report only
    python3 scripts/corpus_check.py corpus/targets/ --generate # generate missing toml
"""
import sys
import os
from pathlib import Path

REQUIRED_FIELDS = ["id", "severity", "observation", "impact", "improvement"]


def discover_patterns(corpus_dir: Path) -> dict[str, dict]:
    """Discover all patterns from positive/negative file pairs."""
    patterns: dict[str, dict] = {}
    for f in corpus_dir.iterdir():
        name = f.name
        if "_positive." in name:
            stem = name.split("_positive.")[0]
            patterns.setdefault(stem, {"positive": f, "negative": None, "toml": None})
            patterns[stem]["positive"] = f
        elif "_negative." in name:
            stem = name.split("_negative.")[0]
            patterns.setdefault(stem, {"positive": None, "negative": f, "toml": None})
            patterns[stem]["negative"] = f

    # Check for toml sidecars
    for stem in list(patterns.keys()):
        toml_path = corpus_dir / f"{stem}.toml"
        patterns[stem]["toml"] = toml_path if toml_path.exists() else None

    return patterns


def check_pattern(stem: str, info: dict) -> list[str]:
    """Check a single pattern for issues. Returns list of warning strings."""
    issues = []
    if not info["positive"]:
        issues.append("missing positive example")
    if not info["negative"]:
        issues.append("missing negative example")
    if not info["toml"]:
        issues.append("missing sidecar .toml")
    return issues


def generate_toml(stem: str, toml_path: Path):
    """Generate a stub sidecar .toml for a pattern."""
    # Derive severity from pattern name
    severity = "Warning"
    if any(k in stem for k in ["secret", "key", "token", "password", "credential"]):
        severity = "Critical"
    elif any(k in stem for k in ["sql_injection", "cmd_injection", "rce", "eval", "exec"]):
        severity = "Critical"
    elif any(k in stem for k in ["xss", "csrf", "ssrf", "path_traversal"]):
        severity = "Warning"

    # Derive a human-readable name
    readable = stem.replace("_", " ").title()

    content = f'''id = "{stem.upper()}"
severity = "{severity}"
observation = "Corpus pattern: {readable}."
impact = "Function shape matches a known violation pattern."
improvement = "Review against corpus example."
'''
    toml_path.write_text(content)


def main():
    if len(sys.argv) < 2:
        print("Usage: corpus_check.py <corpus_dir> [--generate]")
        sys.exit(1)

    corpus_dir = Path(sys.argv[1])
    generate = "--generate" in sys.argv

    if not corpus_dir.is_dir():
        print(f"Error: {corpus_dir} is not a directory")
        sys.exit(1)

    patterns = discover_patterns(corpus_dir)

    total = len(patterns)
    has_toml = sum(1 for p in patterns.values() if p["toml"])
    missing_toml = total - has_toml
    has_both = sum(1 for p in patterns.values() if p["positive"] and p["negative"])
    incomplete = total - has_both

    print(f"Corpus patterns: {total}")
    print(f"  Complete (pos+neg+toml): {has_both - (has_both - sum(1 for p in patterns.values() if p['positive'] and p['negative'] and p['toml']))}")
    print(f"  Missing sidecar .toml:   {missing_toml}")
    print(f"  Missing positive:        {sum(1 for p in patterns.values() if not p['positive'])}")
    print(f"  Missing negative:        {sum(1 for p in patterns.values() if not p['negative'])}")
    print()

    # Report issues
    has_issues = False
    for stem in sorted(patterns):
        issues = check_pattern(stem, patterns[stem])
        if issues:
            has_issues = True
            print(f"  {stem}: {', '.join(issues)}")

    if not has_issues:
        print("  All patterns complete!")

    # Generate missing toml files
    if generate:
        generated = 0
        for stem in sorted(patterns):
            if not patterns[stem]["toml"]:
                toml_path = corpus_dir / f"{stem}.toml"
                generate_toml(stem, toml_path)
                generated += 1
        print(f"\nGenerated {generated} sidecar .toml files")

    sys.exit(1 if has_issues and not generate else 0)


if __name__ == "__main__":
    main()
