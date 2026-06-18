#!/usr/bin/env python3
"""
Semgrep Rule Fixture Harvester

Walks the semgrep-rules repository and extracts test fixtures as Frensense
corpus pairs. Each Semgrep rule directory contains a `tests/` subdirectory
with `_bad.*` / `_ok.*` or `_bad_*.*/`_ok_*.* test files.

    _bad.ts  -> _positive.ts  (the vulnerable/flagged version)
    _ok.ts   -> _negative.ts  (the clean version)

Source:
    git clone --depth 1 https://github.com/semgrep/semgrep-rules /tmp/semgrep-rules

Usage:
    python3 scripts/harvesters/semgrep.py \\
        --repo /tmp/semgrep-rules \\
        --languages typescript javascript rust \\
        --output corpus/targets \\
        --limit 400 \\
        --dry-run
"""

import argparse
import re
import sys
from pathlib import Path
from typing import Optional


# Semgrep rule directory language name -> Frensense language name + extension
LANG_DIRS = {
    "typescript": ("typescript", "ts"),
    "javascript": ("typescript", "ts"),   # ts corpus accepts js patterns
    "rust":       ("rust",       "rs"),
    "python":     ("python",     "py"),   # only if loader supports py
}

# Pattern to detect test fixture files within a semgrep rule directory
# Tests typically live in tests/<rulename>_bad.ext or tests/<rulename>_ok.ext
BAD_SUFFIXES  = ("_bad", "_bad_", "-bad", ".bad")
OK_SUFFIXES   = ("_ok",  "_ok_",  "-ok",  ".ok")


def slugify(name: str) -> str:
    return re.sub(r"[^a-z0-9]", "_", name.lower()).strip("_")[:60]


def find_test_pairs(rule_dir: Path, ext: str) -> list[tuple[Path, Path]]:
    """Find (_bad, _ok) file pairs in a semgrep rule's tests directory."""
    tests_dir = rule_dir / "tests"
    if not tests_dir.exists():
        # Some rules co-locate tests in the rule dir itself
        tests_dir = rule_dir

    pairs = []
    seen_bad: dict[str, Path] = {}
    seen_ok:  dict[str, Path] = {}

    for f in tests_dir.glob(f"*.{ext}"):
        stem = f.stem
        for sfx in BAD_SUFFIXES:
            if sfx in stem:
                key = stem.replace(sfx, "").strip("_-")
                seen_bad[key] = f
                break
        for sfx in OK_SUFFIXES:
            if sfx in stem:
                key = stem.replace(sfx, "").strip("_-")
                seen_ok[key] = f
                break

    for key in seen_bad:
        if key in seen_ok:
            pairs.append((seen_bad[key], seen_ok[key]))

    return pairs


def extract_rule_id(rule_dir: Path) -> Optional[str]:
    """Extract rule ID from the YAML file in the rule directory."""
    for yaml_file in rule_dir.glob("*.yml"):
        try:
            text = yaml_file.read_text(encoding="utf-8", errors="replace")
            m = re.search(r"^\s*-?\s*id:\s*(.+)$", text, re.MULTILINE)
            if m:
                return m.group(1).strip().strip("\"'")
        except Exception:
            pass
    return None


def harvest_semgrep(
    repo_path: str,
    languages: list,
    output_dir: Path,
    limit: int,
    dry_run: bool,
    min_bytes: int,
) -> int:
    repo = Path(repo_path)
    if not repo.exists():
        print(f"  Semgrep: repo not found at {repo_path}")
        print(f"  Clone with: git clone --depth 1 https://github.com/semgrep/semgrep-rules {repo_path}")
        return 0

    output_dir.mkdir(parents=True, exist_ok=True)
    written = 0

    for lang_dir_name in languages:
        if written >= limit:
            break

        if lang_dir_name not in LANG_DIRS:
            print(f"  Semgrep: unsupported language '{lang_dir_name}', skipping")
            continue

        frensense_lang, ext = LANG_DIRS[lang_dir_name]
        lang_path = repo / lang_dir_name

        if not lang_path.exists():
            print(f"  Semgrep: no directory for language '{lang_dir_name}' in repo")
            continue

        print(f"  Semgrep: scanning {lang_path} ...")

        # Walk all rule directories under the language directory
        for rule_dir in sorted(lang_path.rglob("*")):
            if written >= limit:
                break
            if not rule_dir.is_dir():
                continue

            pairs = find_test_pairs(rule_dir, ext)
            if not pairs:
                continue

            rule_id = extract_rule_id(rule_dir) or slugify(rule_dir.name)

            for bad_file, ok_file in pairs:
                if written >= limit:
                    break

                try:
                    pos_content = bad_file.read_text(encoding="utf-8", errors="replace")
                    neg_content = ok_file.read_text(encoding="utf-8", errors="replace")
                except Exception as e:
                    print(f"  Semgrep: read error {bad_file}: {e}")
                    continue

                if len(pos_content.encode()) < min_bytes:
                    continue
                if pos_content.strip() == neg_content.strip():
                    continue

                # Name: {lang}_semgrep_{rule_slug}_{n}
                rule_slug = slugify(rule_id)
                pattern_name = f"{frensense_lang}_semgrep_{rule_slug}"

                pos_path = output_dir / f"{pattern_name}_positive.{ext}"
                neg_path = output_dir / f"{pattern_name}_negative.{ext}"

                # Avoid collisions if rule appears more than once
                collision_idx = 1
                while pos_path.exists() and not dry_run:
                    pattern_name = f"{frensense_lang}_semgrep_{rule_slug}_{collision_idx}"
                    pos_path = output_dir / f"{pattern_name}_positive.{ext}"
                    neg_path = output_dir / f"{pattern_name}_negative.{ext}"
                    collision_idx += 1

                if dry_run:
                    print(f"  [DRY] {pattern_name}")
                else:
                    pos_path.write_text(pos_content, encoding="utf-8", errors="replace")
                    neg_path.write_text(neg_content, encoding="utf-8", errors="replace")

                written += 1
                if written % 50 == 0:
                    print(f"  ... {written} pairs written")

    print(f"\nSemgrep: wrote {written} pairs to {output_dir}")
    return written


def main() -> int:
    p = argparse.ArgumentParser(
        description="Harvest Frensense corpus pairs from semgrep-rules test fixtures"
    )
    p.add_argument(
        "--repo", required=True,
        help="Path to cloned semgrep-rules repository"
    )
    p.add_argument(
        "--languages", nargs="+",
        choices=list(LANG_DIRS.keys()),
        default=["typescript", "javascript", "rust"],
        help="Languages to extract (default: typescript javascript rust)"
    )
    p.add_argument(
        "--output", default="corpus/targets",
        help="Output directory (default: corpus/targets)"
    )
    p.add_argument(
        "--limit", type=int, default=400,
        help="Maximum pairs to write (default: 400)"
    )
    p.add_argument(
        "--min-bytes", type=int, default=100,
        help="Minimum file size in bytes — filters trivial stubs (default: 100)"
    )
    p.add_argument(
        "--dry-run", action="store_true",
        help="Print what would be written without writing files"
    )
    args = p.parse_args()

    count = harvest_semgrep(
        repo_path  = args.repo,
        languages  = args.languages,
        output_dir = Path(args.output),
        limit      = args.limit,
        dry_run    = args.dry_run,
        min_bytes  = args.min_bytes,
    )
    return 0 if count > 0 else 1


if __name__ == "__main__":
    sys.exit(main())
