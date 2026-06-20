"""
CVEfixes Dataset Harvester (LEGACY - JSON format)

IMPORTANT: This harvester expects the JSON export format produced by running the
CVEfixes collection scripts against the GitHub API (git_commits/*.json).

The Zenodo distribution (CVEfixes.db, ~12 GB) uses a different schema —
a SQLite database with method_change/file_change/cve tables. For that
format, use scripts/extract_cvefixes_targeted.py instead.

See docs/CVEFIXES_INTEGRATION.md for the full acquisition workflow.

Usage (legacy JSON format only):
    python3 scripts/harvesters/cvefixes.py --dataset-path /path/to/json/dir

Usage (recommended - SQLite format):
    python3 scripts/extract_cvefixes_targeted.py --db /path/to/CVEfixes.db
"""

import subprocess
import tempfile
import re
from pathlib import Path
from typing import Optional

# Language extensions
LANG_EXT = {
    "rust": "rs",
    "typescript": "ts",
    "javascript": "js",
    "python": "py",
}

EXT_LANG = {v: k for k, v in LANG_EXT.items()}


def harvest_cvefixes(
    output_dir: Path,
    language: Optional[str],
    limit: int,
    dataset_path: Optional[str],
    dry_run: bool,
) -> int:
    """Harvest patterns from CVEfixes dataset (LEGACY - JSON format).

    WARNING: This function expects the JSON export format (git_commits/*.json).
    For the Zenodo SQLite database (CVEfixes.db), use:
        python3 scripts/extract_cvefixes_targeted.py --db /path/to/CVEfixes.db
    """
    print("  WARNING: This harvester uses the legacy JSON format.")
    print("  For Zenodo SQLite database, use scripts/extract_cvefixes_targeted.py")
    print("  See docs/CVEFIXES_INTEGRATION.md for details")

    if dataset_path is None:
        print("  CVEfixes: no dataset path provided, cloning...")
        dataset_path = _clone_dataset()

    dataset = Path(dataset_path)
    if not dataset.exists():
        print(f"  CVEfixes: dataset not found at {dataset_path}")
        return 0

    harvested = 0
    commits_dir = dataset / "git_commits"

    if not commits_dir.exists():
        # Try alternate structure
        commits_dir = dataset

    for commit_file in sorted(commits_dir.glob("*.json")):
        if harvested >= limit:
            break

        try:
            import json
            with open(commit_file) as f:
                commit_data = json.load(f)
        except (json.JSONDecodeError, IOError):
            continue

        pairs = _extract_pairs_from_commit(commit_data, language)
        for pair in pairs:
            if harvested >= limit:
                break

            if dry_run:
                print(f"  Would write: {pair['name']}")
                harvested += 1
                continue

            _write_pair(output_dir, pair)
            harvested += 1

    return harvested


def _clone_dataset() -> str:
    """Clone CVEfixes dataset to temp directory."""
    tmpdir = tempfile.mkdtemp(prefix="cvefixes_")
    subprocess.run(
        ["git", "clone", "--depth", "1",
         "https://github.com/secureIT-project/CVEfixes.git",
         tmpdir],
        check=True,
        capture_output=True,
    )
    return tmpdir


def _extract_pairs_from_commit(commit_data: dict, language: Optional[str]) -> list:
    """Extract function-level pairs from a commit's file changes."""
    pairs = []

    files = commit_data.get("files", [])
    for file_info in files:
        filepath = file_info.get("file_path", "")
        ext = Path(filepath).suffix.lstrip(".")

        if ext not in EXT_LANG:
            continue

        lang = EXT_LANG[ext]
        if language and lang != language:
            continue

        diff = file_info.get("diff", "")
        if not diff:
            continue

        func_pairs = _extract_functions_from_diff(diff, lang, filepath)
        pairs.extend(func_pairs)

    return pairs


def _extract_functions_from_diff(diff: str, language: str, filepath: str) -> list:
    """Extract function-level before/after pairs from a unified diff."""
    pairs = []

    # Parse hunks
    hunks = _parse_hunks(diff)
    if not hunks:
        return pairs

    # For each hunk, extract the function context
    for hunk in hunks:
        before_lines = [l[1:] for l in hunk["removed"] if l.startswith("-")]
        after_lines = [l[1:] for l in hunk["added"] if l.startswith("+")]

        if not before_lines or not after_lines:
            continue

        before_code = "\n".join(before_lines)
        after_code = "\n".join(after_lines)

        # Skip trivial changes (comments, whitespace only)
        if _is_trivial_change(before_code, after_code):
            continue

        # Extract function names
        before_func = _extract_function_name(before_code, language)
        after_func = _extract_function_name(after_code, language)

        if not before_func and not after_func:
            continue

        func_name = before_func or after_func or "changed_function"

        # Generate pattern name
        # CWE extraction requires a DB lookup not available in this JSON-based harvester.
        # The SQLite-based extractor (extract_cvefixes_targeted.py) resolves CWE from
        # the cwe_classification table. Use a safe fallback here.
        cwe = "cve"
        pattern_name = f"{lang}_{cwe}_{_slugify(func_name)}"

        pairs.append({
            "name": pattern_name,
            "language": language,
            "positive": before_code,
            "negative": after_code,
            "func_name": func_name,
            "source_file": filepath,
        })

    return pairs


def _parse_hunks(diff: str) -> list:
    """Parse unified diff into hunks."""
    hunks = []
    current_hunk = None

    for line in diff.split("\n"):
        if line.startswith("@@"):
            if current_hunk:
                hunks.append(current_hunk)
            current_hunk = {"removed": [], "added": []}
        elif current_hunk is not None:
            if line.startswith("-"):
                current_hunk["removed"].append(line)
            elif line.startswith("+"):
                current_hunk["added"].append(line)

    if current_hunk:
        hunks.append(current_hunk)

    return hunks


def _extract_function_name(code: str, language: str) -> Optional[str]:
    """Extract the first function name from code snippet."""
    if language == "rust":
        m = re.search(r"fn\s+(\w+)", code)
    elif language in ("typescript", "javascript"):
        m = re.search(r"(?:function|const|let|var)\s+(\w+)", code)
    elif language == "python":
        m = re.search(r"def\s+(\w+)", code)
    else:
        return None
    return m.group(1) if m else None


def _is_trivial_change(before: str, after: str) -> bool:
    """Check if change is trivial (whitespace, comments only)."""
    # Strip comments and whitespace
    def strip(s):
        lines = []
        for line in s.split("\n"):
            stripped = line.strip()
            if stripped and not stripped.startswith("//") and not stripped.startswith("#"):
                lines.append(stripped)
        return "\n".join(lines)

    return strip(before) == strip(after)


def _slugify(name: str) -> str:
    """Convert function name to slug."""
    return re.sub(r"[^a-z0-9]", "_", name.lower()).strip("_")[:40]


def _write_pair(output_dir: Path, pair: dict):
    """Write a positive/negative pair to corpus/targets/."""
    ext = LANG_EXT[pair["language"]]
    name = pair["name"]

    pos_file = output_dir / f"{name}_positive.{ext}"
    neg_file = output_dir / f"{name}_negative.{ext}"

    pos_file.write_text(pair["positive"])
    neg_file.write_text(pair["negative"])
