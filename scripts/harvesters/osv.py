"""
OSV.dev Dataset Harvester

Queries the OSV.dev API for vulnerabilities and extracts function-level
bug/fix pairs from linked fix commits.

API: https://osv.dev/list
"""

import json
import subprocess
import tempfile
import re
from pathlib import Path
from typing import Optional

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False

# Language to OSV ecosystem mapping
LANG_ECOSYSTEM = {
    "rust": "crates.io",
    "typescript": "npm",
    "javascript": "npm",
    "python": "pypi",
}

ECOSYSTEM_LANG = {v: k for k, v in LANG_ECOSYSTEM.items()}


def harvest_osv(
    output_dir: Path,
    language: Optional[str],
    ecosystem: Optional[str],
    limit: int,
    dry_run: bool,
) -> int:
    """Harvest patterns from OSV.dev."""
    if not HAS_REQUESTS:
        print("  OSV: 'requests' package not installed, skipping")
        print("  Install with: pip install requests")
        return 0

    ecosystems = []
    if ecosystem:
        ecosystems = [ecosystem]
    elif language:
        eco = LANG_ECOSYSTEM.get(language)
        if eco:
            ecosystems = [eco]
        else:
            print(f"  OSV: no ecosystem mapping for language '{language}'")
            return 0
    else:
        ecosystems = ["crates.io", "npm", "pypi"]

    harvested = 0
    seen_fixes = set()

    for eco in ecosystems:
        if harvested >= limit:
            break

        print(f"  OSV: querying {eco}...")
        vulns = _query_osv(eco, limit * 2)  # fetch extra to account for filtering

        for vuln in vulns:
            if harvested >= limit:
                break

            fix_urls = _extract_fix_urls(vuln)
            for fix_url in fix_urls:
                if harvested >= limit:
                    break

                if fix_url in seen_fixes:
                    continue
                seen_fixes.add(fix_url)

                pairs = _extract_from_fix_commit(fix_url, language)
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


def _query_osv(ecosystem: str, limit: int) -> list:
    """Query OSV.dev for vulnerabilities in an ecosystem.

    OSV requires a package name. We query well-known packages per ecosystem
    to get a representative sample of vulnerabilities.
    """
    # Well-known packages with known vulnerabilities per ecosystem
    PACKAGES = {
        "crates.io": ["serde", "tokio", "hyper", "reqwest", "axum", "actix-web",
                      "openssl", "regex", "chrono", "crossbeam"],
        "npm": ["express", "lodash", "minimist", "node-fetch", "axios",
                "webpack-dev-server", "json5", "qs", "tar", "path-parse"],
        "pypi": ["django", "flask", "requests", "urllib3", "cryptography",
                 "pillow", "pyyaml", "jinja2", "aiohttp", "starlette"],
    }

    packages = PACKAGES.get(ecosystem, [])
    if not packages:
        print(f"  OSV: no known packages for {ecosystem}")
        return []

    all_vulns = []
    url = "https://api.osv.dev/v1/query"

    for pkg_name in packages:
        if len(all_vulns) >= limit:
            break
        try:
            r = requests.post(url, json={"package": {"name": pkg_name, "ecosystem": ecosystem}}, timeout=15)
            r.raise_for_status()
            vulns = r.json().get("vulns", [])
            all_vulns.extend(vulns)
        except Exception as e:
            print(f"  OSV: query failed for {ecosystem}/{pkg_name}: {e}")

    return all_vulns[:limit]


def _extract_fix_urls(vuln: dict) -> list:
    """Extract fix commit URLs from an OSV vulnerability."""
    urls = []
    for ref in vuln.get("references", []):
        if ref.get("type") == "FIX":
            url = ref.get("url", "")
            if "github.com" in url and "/commit/" in url:
                urls.append(url)
    return urls


def _extract_from_fix_commit(fix_url: str, language: Optional[str]) -> list:
    """Extract function-level pairs from a GitHub fix commit."""
    pairs = []

    # Parse GitHub URL
    parts = fix_url.split("github.com/")[1].split("/commit/")
    if len(parts) != 2:
        return pairs

    repo, sha = parts[0], parts[1].split("?")[0]

    # Fetch the diff
    diff = _fetch_github_diff(repo, sha)
    if not diff:
        return pairs

    # Parse diff into file changes
    files = _parse_diff_files(diff)

    for filepath, file_diff in files.items():
        ext = Path(filepath).suffix.lstrip(".")
        lang = _ext_to_lang(ext)
        if not lang:
            continue
        if language and lang != language:
            continue

        func_pairs = _extract_functions_from_diff(file_diff, lang, filepath)
        pairs.extend(func_pairs)

    return pairs


def _fetch_github_diff(repo: str, sha: str) -> Optional[str]:
    """Fetch diff for a GitHub commit.

    Set the GITHUB_TOKEN environment variable to avoid unauthenticated rate limiting
    (60 req/hr). Authenticated requests get 5,000 req/hr.
    """
    import os
    url = f"https://api.github.com/repos/{repo}/commits/{sha}"
    headers = {"Accept": "application/vnd.github.diff"}
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    try:
        r = requests.get(url, headers=headers, timeout=15)
        if r.status_code == 200:
            return r.text
        if r.status_code == 403:
            print(f"  OSV: GitHub rate limited — set GITHUB_TOKEN env var to increase limit")
    except Exception:
        pass
    return None


def _parse_diff_files(diff: str) -> dict:
    """Parse unified diff into per-file changes."""
    files = {}
    current_file = None

    for line in diff.split("\n"):
        if line.startswith("diff --git"):
            parts = line.split(" b/")
            if len(parts) == 2:
                current_file = parts[1]
                files[current_file] = []
        elif current_file is not None:
            files[current_file].append(line)

    # Convert to strings
    return {k: "\n".join(v) for k, v in files.items()}


def _ext_to_lang(ext: str) -> Optional[str]:
    """Map file extension to language name."""
    mapping = {
        "rs": "rust",
        "ts": "typescript",
        "tsx": "typescript",
        "js": "javascript",
        "jsx": "javascript",
        "py": "python",
    }
    return mapping.get(ext)


def _extract_functions_from_diff(diff: str, language: str, filepath: str) -> list:
    """Extract function-level before/after pairs from a unified diff."""
    pairs = []
    lang_ext = {"rust": "rs", "typescript": "ts", "javascript": "js", "python": "py"}
    ext = lang_ext.get(language, "txt")

    hunks = _parse_hunks(diff)
    for hunk in hunks:
        before_lines = [l[1:] for l in hunk["removed"] if l.startswith("-")]
        after_lines = [l[1:] for l in hunk["added"] if l.startswith("+")]

        if not before_lines or not after_lines:
            continue

        before_code = "\n".join(before_lines)
        after_code = "\n".join(after_lines)

        if _is_trivial_change(before_code, after_code):
            continue

        func_name = _extract_function_name(before_code, language) or "changed_function"
        slug = re.sub(r"[^a-z0-9]", "_", func_name.lower()).strip("_")[:40]
        cwe = "cve"  # placeholder
        pattern_name = f"{language}_{cwe}_{slug}"

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
    hunks = []
    current = None
    for line in diff.split("\n"):
        if line.startswith("@@"):
            if current:
                hunks.append(current)
            current = {"removed": [], "added": []}
        elif current is not None:
            if line.startswith("-"):
                current["removed"].append(line)
            elif line.startswith("+"):
                current["added"].append(line)
    if current:
        hunks.append(current)
    return hunks


def _extract_function_name(code: str, language: str) -> Optional[str]:
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
    def strip(s):
        lines = []
        for line in s.split("\n"):
            stripped = line.strip()
            if stripped and not stripped.startswith("//") and not stripped.startswith("#"):
                lines.append(stripped)
        return "\n".join(lines)
    return strip(before) == strip(after)


def _write_pair(output_dir: Path, pair: dict):
    ext = pair.get("language", "txt")
    lang_ext = {"rust": "rs", "typescript": "ts", "javascript": "js", "python": "py"}
    ext = lang_ext.get(ext, "txt")
    name = pair["name"]
    (output_dir / f"{name}_positive.{ext}").write_text(pair["positive"])
    (output_dir / f"{name}_negative.{ext}").write_text(pair["negative"])
