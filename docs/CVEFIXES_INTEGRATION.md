# CVEfixes Integration Guide: Seeding the Frensense Corpus

> Covers acquisition, extraction, and ingestion of the CVEfixes 1.0.8 dataset into
> `corpus/targets/` for both Rust and TypeScript. The dataset is distributed as a 12 GB
> SQLite dump on Zenodo — not a Git repo you can clone. This guide addresses that directly.

---

## 1. The Dataset Size Problem

The `CVEfixes-1.0.8.zip` already present at the repo root contains **only the collection
tooling** — Python scripts, documentation, and an ER diagram. It does not contain the
actual vulnerability data. The real dataset is a compressed SQLite dump hosted on Zenodo:

| Resource | Location |
|---|---|
| Zenodo record | https://doi.org/10.5281/zenodo.4476563 |
| Compressed SQL dump | `CVEfixes.sql.gz` (approx. 2–4 GB compressed, ~12 GB expanded) |
| SQLite database | `CVEfixes.db` after decompression |

You do not need to decompress the full database to extract Rust and TypeScript patterns.
The sections below cover three acquisition strategies ordered by disk cost.

---

## 2. Acquisition Strategies

### Strategy A: Targeted SQL Extraction (Recommended — ~2 GB disk)

Download the compressed dump from Zenodo, stream it directly through SQLite without
writing the full uncompressed database to disk, and extract only the rows you need.

**Step 1: Download the dump**

```bash
# Zenodo direct download — check the record for the current file URL
wget -c "https://zenodo.org/record/4476563/files/CVEfixes.sql.gz" \
  -O /tmp/CVEfixes.sql.gz
```

If `wget` is not available:
```bash
curl -L -C - -o /tmp/CVEfixes.sql.gz \
  "https://zenodo.org/record/4476563/files/CVEfixes.sql.gz"
```

The `-c` / `-C -` flag resumes interrupted downloads. The file is large — expect
30–90 minutes on a home connection.

**Step 2: Stream into SQLite**

```bash
# Do NOT fully decompress first. Stream directly.
gzip -dc /tmp/CVEfixes.sql.gz | sqlite3 /tmp/CVEfixes.db
```

This writes the full 12 GB database once. If disk space is constrained, see
Strategy B (streaming extraction without writing the db file).

**Step 3: Run the targeted extractor**

```bash
cd /home/oxisrael/Friehub/Taas/frensense
python3 scripts/extract_cvefixes_targeted.py \
  --db /tmp/CVEfixes.db \
  --language rust \
  --output corpus/targets \
  --limit 500

python3 scripts/extract_cvefixes_targeted.py \
  --db /tmp/CVEfixes.db \
  --language typescript \
  --output corpus/targets \
  --limit 500
```

See Section 4 for the `extract_cvefixes_targeted.py` script specification.

---

### Strategy B: Streaming Extraction Without Writing Full DB (~500 MB disk)

If you cannot spare 12 GB, the SQL dump can be piped through a filter that writes only
the rows for Rust and TypeScript directly to a smaller output database.

```bash
# Create a filtered database with only Rust/TypeScript method_change rows
gzip -dc /tmp/CVEfixes.sql.gz | grep -E \
  "(CREATE TABLE|INSERT INTO (cve|fixes|commits|file_change|method_change))" \
  | sqlite3 /tmp/CVEfixes_filtered.db
```

This is fragile because the INSERT statements may span lines. The safer alternative is
to load the full database and immediately filter:

```bash
# Load full db, then immediately export a filtered subset
gzip -dc /tmp/CVEfixes.sql.gz | sqlite3 /tmp/CVEfixes.db

sqlite3 /tmp/CVEfixes.db <<'EOF'
ATTACH DATABASE '/tmp/CVEfixes_small.db' AS out;

CREATE TABLE out.method_change AS
  SELECT mc.*, f.programming_language, f.filename, cv.cve_id, cc.cwe_id
  FROM method_change mc
  JOIN file_change f ON mc.file_change_id = f.file_change_id
  JOIN commits c ON f.hash = c.hash
  JOIN fixes fx ON c.hash = fx.hash
  JOIN cve cv ON fx.cve_id = cv.cve_id
  LEFT JOIN cwe_classification cc ON cv.cve_id = cc.cve_id
  WHERE f.programming_language IN ('Rust', 'TypeScript', 'JavaScript');

DETACH DATABASE out;
EOF

rm /tmp/CVEfixes.db   # reclaim the 12 GB
# CVEfixes_small.db is now ~50-200 MB
```

Then point the extractor at the smaller database.

---

### Strategy C: Rebuild from Scratch with Sample Limit (~0 GB download, ~12 min runtime)

The CVEfixes 1.0.8 zip already contains the collection scripts. You can run them
directly against the NVD + GitHub API to collect a bounded sample without downloading
the full Zenodo dump. This is slower in elapsed time but uses no disk for a pre-built
dump.

**Setup:**

```bash
cd /tmp
unzip /home/oxisrael/Friehub/Taas/frensense/CVEfixes-1.0.8.zip
cd CVEfixes-1.0.8

# Install dependencies
pip install -r requirements.txt

# Create config
cat > .CVEfixes.ini <<'EOF'
[CVEfixes]
database_path = /tmp/cvefixes_sample
sample_limit = 50

[GitHub]
user = YOUR_GITHUB_USERNAME
token = YOUR_GITHUB_PAT
EOF

mkdir -p /tmp/cvefixes_sample
```

**Run collection:**

```bash
sh Code/create_CVEfixes_from_scratch.sh
# Runtime: ~12 minutes for sample_limit=50
# Output: /tmp/cvefixes_sample/CVEfixes.db
```

**Then extract:**

```bash
cd /home/oxisrael/Friehub/Taas/frensense
python3 scripts/extract_cvefixes_targeted.py \
  --db /tmp/cvefixes_sample/CVEfixes.db \
  --language rust typescript \
  --output corpus/targets \
  --limit 100
```

> **Note:** `sample_limit = 50` yields approximately 30–80 method-level pairs after
> language filtering. Increase to `200` for ~200–400 pairs (runtime ~45 minutes).
> GitHub token is required — unauthenticated rate limit is 60 req/hr which will cause
> failures after ~25 repositories.

---

## 3. CVEfixes Database Schema (Relevant Tables)

The `method_change` table is the entry point for function-level extraction.

```sql
-- The table that maps directly to Frensense corpus pairs
SELECT
  mc.method_change_id,
  mc.name,               -- function name
  mc.code,               -- full function source (before or after, depending on before_change)
  mc.before_change,      -- 'True' = vulnerable version, 'False' = fixed version
  mc.code_before,        -- vulnerable version of the function (if stored)
  mc.code_after,         -- fixed version of the function (if stored)
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
WHERE f.programming_language IN ('Rust', 'TypeScript', 'JavaScript')
  AND mc.code IS NOT NULL
  AND mc.code != ''
ORDER BY cv.cve_id, mc.before_change DESC;
```

**Key semantics:**

| Column | Meaning for Frensense |
|---|---|
| `before_change = 'True'` | Vulnerable code — maps to `_positive` (what Frensense should flag) |
| `before_change = 'False'` | Fixed code — maps to `_negative` (what Frensense should pass) |
| `code` | The function body at this version |
| `code_before` / `code_after` | Some rows store both in the same record |

The `before_change` naming is counterintuitive. In Frensense's convention:
- **positive** = the example the engine should _flag_ = the **vulnerable** version (`before_change = True`)
- **negative** = the example the engine should _pass_ = the **fixed** version (`before_change = False`)

---

## 4. Targeted Extractor Script Specification

The existing `scripts/harvesters/cvefixes.py` is written for the JSON export format
from the CVEfixes Git repo (which is separate from the Zenodo SQLite dump). It will not
work directly against the SQLite database.

Create `scripts/extract_cvefixes_targeted.py` to query the SQLite database directly:

```python
#!/usr/bin/env python3
"""
CVEfixes SQLite Targeted Extractor
Reads CVEfixes.db directly and emits corpus/targets/ pairs for Frensense.

Usage:
    python3 scripts/extract_cvefixes_targeted.py \
        --db /tmp/CVEfixes.db \
        --language rust typescript \
        --output corpus/targets \
        --limit 500 \
        --min-lines 5

Schema note:
    before_change = 'True'  -> vulnerable -> _positive
    before_change = 'False' -> fixed      -> _negative
"""

import argparse
import re
import sqlite3
import sys
from pathlib import Path
from collections import defaultdict

LANG_MAP = {
    "Rust": "rust",
    "TypeScript": "typescript",
    "JavaScript": "typescript",  # treat JS as ts for corpus naming
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


def extract(db_path: str, languages: list, output_dir: Path, limit: int, min_lines: int):
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

    # Group by (cve_id, function_name) to pair before/after
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
        fn_slug  = slugify(func_name)
        cve_slug = slugify(cve_id)

        pattern_name = f"{lang}_cvefixes_{cwe_slug}_{cve_slug}_{fn_slug}"[:100]

        (output_dir / f"{pattern_name}_positive.{ext}").write_text(pos_code)
        (output_dir / f"{pattern_name}_negative.{ext}").write_text(neg_code)
        written += 1
        print(f"  [{written:4d}] {pattern_name}")

    print(f"\nWrote {written} pairs to {output_dir}")
    return written


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--db",       required=True, help="Path to CVEfixes.db")
    p.add_argument("--language", nargs="+",
                   choices=["rust", "typescript", "javascript"],
                   default=["rust", "typescript"])
    p.add_argument("--output",   default="corpus/targets")
    p.add_argument("--limit",    type=int, default=500)
    p.add_argument("--min-lines",type=int, default=5,
                   help="Minimum lines in function body (filters trivial hunks)")
    args = p.parse_args()

    # Map user-facing language names to DB programming_language values
    db_langs = []
    for l in args.language:
        if l in ("rust",):
            db_langs.append("Rust")
        elif l in ("typescript", "javascript"):
            db_langs.extend(["TypeScript", "JavaScript"])

    count = extract(
        db_path   = args.db,
        languages = db_langs,
        output_dir= Path(args.output),
        limit     = args.limit,
        min_lines = args.min_lines,
    )
    return 0 if count > 0 else 1


if __name__ == "__main__":
    sys.exit(main())
```

---

## 5. Post-Extraction: Deduplication and Bundle Rebuild

After extraction, run deduplication to remove near-duplicate pairs that differ only in
whitespace or comments (this is already implemented in `scripts/deduplicate_corpus.py`):

```bash
cd /home/oxisrael/Friehub/Taas/frensense

# 1. Deduplicate
python3 scripts/deduplicate_corpus.py \
  --corpus corpus/targets \
  --dry-run          # preview what would be removed

python3 scripts/deduplicate_corpus.py \
  --corpus corpus/targets  # apply

# 2. Rebuild the FRC bundle
cargo run --bin build-corpus-bundle -- \
  --corpus corpus/targets \
  --output frensense-corpus.frc

# 3. Run corpus tests
cargo test -p frensense-engine -- corpus

# 4. Verify recall on ground truth
python3 scripts/validate_recall.py \
  --corpus corpus/targets \
  --ground-truth corpus/ground_truth/axum_labels.json
```

---

## 6. Expected Output Volume

Based on the CVEfixes 1.0.8 dataset composition:

| Language | Records in DB | After filtering (no blanks, paired) | Expected pairs |
|---|---|---|---|
| Rust | ~400 method changes | ~30–60% have both before/after | ~120–240 |
| TypeScript / JavaScript | ~900 method changes | ~30–60% have both before/after | ~270–540 |

The Rust count is low because CVEfixes was built primarily against C/C++ and Python
repositories. The TypeScript count includes npm packages (express, lodash, qs, etc.)
which are well represented.

For higher Rust coverage, supplement with OSV.dev (see `scripts/harvesters/osv.py`
targeting `crates.io`) or with the Semgrep rule fixtures approach described in
`CORPUS_STRATEGY.md` Section 3 (Tier 3).

---

## 7. Naming Convention Reference

All files in `corpus/targets/` follow this convention:

```
{lang}_{source}_{cwe_slug}_{cve_id_slug}_{fn_slug}_{positive|negative}.{ext}
```

Examples:

```
rust_cvefixes_sql_injection_cve_2021_1234_process_query_positive.rs
rust_cvefixes_sql_injection_cve_2021_1234_process_query_negative.rs
typescript_cvefixes_cmd_injection_cve_2020_5678_exec_command_positive.ts
typescript_cvefixes_cmd_injection_cve_2020_5678_exec_command_negative.ts
```

The `_positive` suffix marks the **vulnerable** version (what Frensense should flag).
The `_negative` suffix marks the **fixed** version (what Frensense should not flag).

This is consistent with all existing files in `corpus/targets/`.
