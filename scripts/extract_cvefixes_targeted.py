#!/usr/bin/env python3
"""
CVEfixes SQLite Targeted Extractor

Reads CVEfixes.db directly and emits corpus/targets/ pairs for Frensense.
Can also rebuild from scratch using the CVEfixes collection scripts (Strategy C).

Usage (from existing DB):
    python3 scripts/extract_cvefixes_targeted.py \
        --db /tmp/CVEfixes.db \
        --language rust typescript \
        --output corpus/targets \
        --limit 500

Usage (rebuild from scratch — no 12GB download):
    python3 scripts/extract_cvefixes_targeted.py \
        --rebuild --sample-limit 50 \
        --language rust typescript \
        --output corpus/targets

Schema note:
    before_change = 'True'  -> vulnerable -> _positive
    before_change = 'False' -> fixed      -> _negative
"""

import argparse
import os
import re
import sqlite3
import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path

LANG_MAP = {
    "Rust": "rust",
    "TypeScript": "typescript",
    "JavaScript": "typescript",
}

LANG_EXT = {
    "rust": "rs",
    "typescript": "ts",
}

CWE_SLUG = {
    "CWE-89":  "sql_injection",
    "CWE-78":  "cmd_injection",
    "CWE-22":  "path_traversal",
    "CWE-79":  "xss",
    "CWE-918": "ssrf",
    "CWE-416": "use_after_free",
    "CWE-190": "integer_overflow",
    "CWE-362": "race_condition",
    "CWE-476": "null_deref",
    "CWE-502": "deserialization",
}


def slugify(name: str) -> str:
    return re.sub(r"[^a-z0-9]", "_", name.lower()).strip("_")[:40]


def rebuild_database(sample_limit: int) -> str:
    """Run CVEfixes collection scripts to build a fresh database."""
    cvefixes_dir = Path("/tmp/CVEfixes-1.0.8")
    if not cvefixes_dir.exists():
        # Try extracting from zip
        zip_path = Path("CVEfixes-1.0.8.zip")
        if zip_path.exists():
            subprocess.run(["unzip", "-o", str(zip_path), "-d", "/tmp"], check=True)
        else:
            print("Error: CVEfixes-1.0.8.zip not found in current directory")
            print("Download from: https://doi.org/10.5281/zenodo.13138703")
            sys.exit(1)

    # Create config
    config_path = cvefixes_dir / ".CVEfixes.ini"
    db_dir = Path(tempfile.mkdtemp(prefix="cvefixes_"))
    config_path.write_text(f"""[CVEfixes]
database_path = {db_dir}
sample_limit = {sample_limit}

[GitHub]
user = {os.environ.get('GITHUB_USER', '')}
token = {os.environ.get('GITHUB_TOKEN', '')}
""")

    print(f"Rebuilding CVEfixes database (sample_limit={sample_limit})...")
    print(f"Output: {db_dir}/CVEfixes.db")

    result = subprocess.run(
        ["sh", "Code/create_CVEfixes_from_scratch.sh"],
        cwd=str(cvefixes_dir),
        capture_output=True, text=True,
    )

    if result.returncode != 0:
        print(f"Collection failed:\n{result.stderr}")
        sys.exit(1)

    db_path = db_dir / "CVEfixes.db"
    if not db_path.exists():
        print(f"Database not created at {db_path}")
        sys.exit(1)

    print(f"Database created: {db_path}")
    return str(db_path)


def extract(db_path: str, languages: list, output_dir: Path, limit: int, min_lines: int):
    """Extract function-level pairs from CVEfixes database."""
    lang_filter = ", ".join(f"'{l}'" for l in languages)
    query = f"""
        SELECT
            mc.name,
            mc.code,
            mc.before_change,
            f.programming_language,
            f.filename,
            cv.cve_id,
            cc.cwe_id
        FROM method_change mc
        JOIN file_change f    ON mc.file_change_id = f.file_change_id
        JOIN commits c        ON f.hash = c.hash
        JOIN fixes fx         ON c.hash = fx.hash
        JOIN cve cv           ON fx.cve_id = cv.cve_id
        LEFT JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
        WHERE f.programming_language IN ({lang_filter})
          AND mc.code IS NOT NULL
          AND length(mc.code) > 0
        ORDER BY cv.cve_id, mc.before_change DESC
    """

    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
    cur = conn.cursor()
    cur.execute(query)

    groups = defaultdict(dict)
    for row in cur:
        key = (row["cve_id"], row["name"] or "unknown")
        version = "positive" if row["before_change"] == "True" else "negative"
        groups[key][version] = {
            "code": row["code"],
            "lang": LANG_MAP.get(row["programming_language"], "typescript"),
            "cwe": row["cwe_id"] or "cve",
        }
    conn.close()

    output_dir.mkdir(parents=True, exist_ok=True)
    written = 0

    for (cve_id, func_name), pair in groups.items():
        if written >= limit:
            break
        if "positive" not in pair or "negative" not in pair:
            continue

        lang = pair["positive"]["lang"]
        ext = LANG_EXT.get(lang)
        if ext is None:
            continue

        pos_code = pair["positive"]["code"]
        neg_code = pair["negative"]["code"]

        if len(pos_code.splitlines()) < min_lines:
            continue
        if pos_code.strip() == neg_code.strip():
            continue

        cwe_raw = pair["positive"]["cwe"] or "cve"
        cwe_slug = CWE_SLUG.get(cwe_raw, slugify(cwe_raw))
        fn_slug = slugify(func_name)
        cve_slug = slugify(cve_id)

        pattern_name = f"{lang}_cvefixes_{cwe_slug}_{cve_slug}_{fn_slug}"[:100]

        (output_dir / f"{pattern_name}_positive.{ext}").write_text(pos_code)
        (output_dir / f"{pattern_name}_negative.{ext}").write_text(neg_code)
        written += 1
        print(f"  [{written:4d}] {pattern_name}")

    print(f"\nWrote {written} pairs to {output_dir}")
    return written


def main():
    p = argparse.ArgumentParser(description="Extract CVEfixes pairs for FrenSense corpus")
    p.add_argument("--db", help="Path to existing CVEfixes.db")
    p.add_argument("--rebuild", action="store_true",
                   help="Rebuild database from scratch using CVEfixes scripts")
    p.add_argument("--sample-limit", type=int, default=50,
                   help="Number of repos to collect when rebuilding (default: 50)")
    p.add_argument("--language", nargs="+",
                   choices=["rust", "typescript", "javascript"],
                   default=["rust", "typescript"])
    p.add_argument("--output", default="corpus/targets")
    p.add_argument("--limit", type=int, default=500)
    p.add_argument("--min-lines", type=int, default=5,
                   help="Minimum lines in function body")
    args = p.parse_args()

    if args.rebuild:
        db_path = rebuild_database(args.sample_limit)
    elif args.db:
        db_path = args.db
    else:
        print("Error: specify --db <path> or --rebuild")
        sys.exit(1)

    db_langs = []
    for l in args.language:
        if l in ("rust",):
            db_langs.append("Rust")
        elif l in ("typescript", "javascript"):
            db_langs.extend(["TypeScript", "JavaScript"])

    count = extract(
        db_path=db_path,
        languages=db_langs,
        output_dir=Path(args.output),
        limit=args.limit,
        min_lines=args.min_lines,
    )
    return 0 if count > 0 else 1


if __name__ == "__main__":
    sys.exit(main())
