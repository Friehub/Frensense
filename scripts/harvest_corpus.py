#!/usr/bin/env python3
"""
Frensense Corpus Harvest Pipeline

Extracts function-level bug/fix pairs from CVE datasets and writes them
to corpus/targets/ in FrenSense's positive/negative format.

Usage:
    python3 scripts/harvest_corpus.py --source cvefixes --language rust --limit 100
    python3 scripts/harvest_corpus.py --source osv --ecosystem npm --limit 500
    python3 scripts/harvest_corpus.py --source semgrep --semgrep-repo /tmp/semgrep-rules --limit 400
    python3 scripts/harvest_corpus.py --source all --limit 1000
"""

import argparse
import sys
import os
from pathlib import Path

CORPUS_DIR = Path(__file__).parent.parent / "corpus" / "targets"


def main():
    parser = argparse.ArgumentParser(description="Harvest corpus pairs from CVE datasets")
    parser.add_argument(
        "--source",
        choices=["cvefixes", "osv", "semgrep", "all"],
        default="all",
        help="Data source to harvest from",
    )
    parser.add_argument(
        "--language",
        choices=["rust", "typescript", "javascript", "python"],
        default=None,
        help="Filter by language (default: all)",
    )
    parser.add_argument(
        "--ecosystem",
        default=None,
        help="OSV ecosystem filter (npm, crates.io, pypi)",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=100,
        help="Max patterns to harvest (default: 100)",
    )
    parser.add_argument(
        "--output",
        type=str,
        default=str(CORPUS_DIR),
        help=f"Output directory (default: {CORPUS_DIR})",
    )
    parser.add_argument(
        "--dataset-path",
        type=str,
        default=None,
        help="Path to cloned CVEfixes dataset (for cvefixes source)",
    )
    parser.add_argument(
        "--semgrep-repo",
        type=str,
        default=None,
        help="Path to cloned semgrep-rules repository (for semgrep source)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print what would be harvested without writing files",
    )
    args = parser.parse_args()

    output_dir = Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)

    total_harvested = 0

    if args.source in ("cvefixes", "all"):
        from harvesters.cvefixes import harvest_cvefixes
        count = harvest_cvefixes(
            output_dir=output_dir,
            language=args.language,
            limit=args.limit - total_harvested,
            dataset_path=args.dataset_path,
            dry_run=args.dry_run,
        )
        total_harvested += count
        print(f"  CVEfixes: harvested {count} patterns")

    if args.source in ("osv", "all") and total_harvested < args.limit:
        from harvesters.osv import harvest_osv
        count = harvest_osv(
            output_dir=output_dir,
            language=args.language,
            ecosystem=args.ecosystem,
            limit=args.limit - total_harvested,
            dry_run=args.dry_run,
        )
        total_harvested += count
        print(f"  OSV: harvested {count} patterns")

    if args.source in ("semgrep", "all") and total_harvested < args.limit:
        from harvesters.semgrep import harvest_semgrep
        semgrep_repo = args.semgrep_repo
        if semgrep_repo is None:
            print("  Semgrep: --semgrep-repo not provided, skipping")
            print("  Clone with: git clone --depth 1 https://github.com/semgrep/semgrep-rules /tmp/semgrep-rules")
        else:
            languages = [args.language] if args.language else ["typescript", "javascript", "rust"]
            count = harvest_semgrep(
                repo_path=semgrep_repo,
                languages=languages,
                output_dir=output_dir,
                limit=args.limit - total_harvested,
                dry_run=args.dry_run,
                min_bytes=100,
            )
            total_harvested += count
            print(f"  Semgrep: harvested {count} patterns")

    print(f"\nTotal: {total_harvested} patterns written to {output_dir}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
