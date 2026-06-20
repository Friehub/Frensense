"""
CVEfixes SQLite Harvester

Wrapper around extract_cvefixes_targeted.py for use with harvest_corpus.py.
Queries the CVEfixes SQLite database directly.
"""

import sys
from pathlib import Path
from typing import Optional

# Add scripts directory to path for import
sys.path.insert(0, str(Path(__file__).parent.parent))


def harvest_cvefixes_sqlite(
    output_dir: Path,
    language: Optional[str],
    limit: int,
    db_path: Optional[str],
    dry_run: bool,
) -> int:
    """
    Harvest patterns from CVEfixes SQLite database.

    Args:
        output_dir: Directory to write corpus pairs
        language: Filter by language (rust, typescript, etc.)
        limit: Maximum patterns to harvest
        db_path: Path to CVEfixes.db SQLite database
        dry_run: If True, don't write files

    Returns:
        Number of patterns harvested
    """
    if db_path is None:
        print("  CVEfixes: No database path provided (--dataset-path)")
        print("  Download CVEfixes.db from: https://doi.org/10.5281/zenodo.13138703")
        print("  Then run: python3 scripts/extract_cvefixes_targeted.py --db /path/to/CVEfixes.db")
        return 0

    db_file = Path(db_path)
    if not db_file.exists():
        print(f"  CVEfixes: Database not found at {db_path}")
        return 0

    if dry_run:
        print(f"  CVEfixes: Would harvest from {db_path} (dry run)")
        return 0

    # Import and run the targeted extractor
    from extract_cvefixes_targeted import extract

    # Map language filter to database language names
    db_langs = []
    if language:
        lang_map = {
            "rust": ["Rust"],
            "typescript": ["TypeScript", "JavaScript"],
            "javascript": ["TypeScript", "JavaScript"],
        }
        db_langs = lang_map.get(language, [])
    else:
        db_langs = ["Rust", "TypeScript", "JavaScript"]

    count = extract(
        db_path=str(db_file),
        languages=db_langs,
        output_dir=output_dir,
        limit=limit,
        min_lines=5,
    )

    return count
