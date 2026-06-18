#!/usr/bin/env python3
"""
GitHub CVE Fix Harvester

Queries GitHub for CVE-fixing commits in specific repos and extracts
function-level diffs as corpus pairs. No 12GB download needed.

Strategy: For each known-vulnerable package, find commits that mention
CVE IDs, extract the diff, and split into function-level pairs.

Usage:
    python3 scripts/harvesters/github_fixes.py --limit 100 --output corpus/targets/
"""

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path
from typing import Optional

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False

# Known vulnerable Rust crates and TS/JS npm packages with CVE-fixing commits
KNOWN_VULNERABLE_REPOS = {
    "rust": [
        ("rust-lang/rust", ["CVE-2024", "CVE-2023"]),
        ("denoland/deno", ["CVE-2024", "CVE-2023"]),
        ("actix/actix-web", ["fix", "security", "vuln"]),
        ("tokio-rs/tokio", ["fix", "security"]),
        ("hyperium/hyper", ["fix", "security", "CVE"]),
        ("seanmonstar/reqwest", ["fix", "CVE"]),
        ("serde-rs/serde", ["fix", "CVE"]),
        ("dtolnay/syn", ["fix", "CVE"]),
    ],
    "typescript": [
        ("expressjs/express", ["CVE", "security", "fix"]),
        ("fastify/fastify", ["CVE", "security", "fix"]),
        ("vercel/next.js", ["CVE", "security"]),
        ("facebook/react", ["CVE", "security"]),
        ("vuejs/vue", ["CVE", "security"]),
        ("angular/angular", ["CVE", "security"]),
        ("koajs/koa", ["CVE", "security"]),
        ("hapijs/hapi", ["CVE", "security"]),
    ],
}

EXT_MAP = {"rust": "rs", "typescript": "ts"}


def harvest_github_fixes(
    output_dir: Path,
    language: Optional[str],
    limit: int,
    dry_run: bool,
) -> int:
    """Harvest CVE fix pairs from GitHub repos."""
    if not HAS_REQUESTS:
        print("  GitHub fixes: 'requests' package required")
        return 0

    # Primary approach: OSV fix commits (more reliable than keyword search)
    print("  Harvesting from OSV fix commits...")
    harvested = harvest_from_osv(output_dir, language, limit, dry_run)

    # If we need more, try GitHub keyword search
    if harvested < limit:
        print(f"  Harvested {harvested} from OSV, trying GitHub search for more...")
        harvested += _harvest_from_github_search(output_dir, language, limit - harvested, dry_run)

    return harvested


def _harvest_from_github_search(
    output_dir: Path,
    language: Optional[str],
    limit: int,
    dry_run: bool,
) -> int:
    """Fallback: search GitHub for CVE-fixing commits."""
    harvested = 0
    repos = []

    if language:
        repos = KNOWN_VULNERABLE_REPOS.get(language, [])
    else:
        for lang_repos in KNOWN_VULNERABLE_REPOS.values():
            repos.extend(lang_repos)

    for repo, keywords in repos:
        if harvested >= limit:
            break

        lang = "rust" if "/rust" in str(keywords) or repo.endswith("/rust") else "typescript"
        if language and lang != language:
            continue

        commits = _search_cve_commits(repo, keywords, per_page=5)

        for commit in commits:
            if harvested >= limit:
                break
            sha = commit.get("sha", "")
            pairs = _extract_pairs_from_commit(repo, sha, lang)
            for pair in pairs:
                if harvested >= limit:
                    break
                if dry_run:
                    print(f"    Would write: {pair['name']}")
                else:
                    _write_pair(output_dir, pair)
                harvested += 1

    return harvested


def _search_cve_commits(repo: str, keywords: list, per_page: int = 10) -> list:
    """Search for commits matching keywords in a repo."""
    query = " ".join(keywords)
    url = f"https://api.github.com/search/commits"
    params = {"q": f"{query} repo:{repo}", "per_page": per_page}
    headers = {"Accept": "application/vnd.github.cloak-preview+json"}

    try:
        r = requests.get(url, params=params, headers=headers, timeout=15)
        if r.status_code == 200:
            return r.json().get("items", [])
    except Exception:
        pass
    return []


def harvest_from_osv(
    output_dir: Path,
    language: Optional[str],
    limit: int,
    dry_run: bool,
) -> int:
    """Harvest fix commits from OSV advisories that have GitHub fix references."""
    if not HAS_REQUESTS:
        return 0

    PACKAGES = {
        "rust": [
            ("actix-web", "crates.io"), ("tokio", "crates.io"),
            ("hyper", "crates.io"), ("reqwest", "crates.io"),
            ("axum", "crates.io"), ("warp", "crates.io"),
            ("rocket", "crates.io"), ("regex", "crates.io"),
            ("openssl", "crates.io"), ("time", "crates.io"),
            ("rustls", "crates.io"), ("h2", "crates.io"),
            ("webpki", "crates.io"), ("ring", "crates.io"),
        ],
        "typescript": [
            ("express", "npm"), ("next", "npm"), ("axios", "npm"),
            ("lodash", "npm"), ("fastify", "npm"), ("koa", "npm"),
            ("socket.io", "npm"), ("vite", "npm"), ("terser", "npm"),
            ("minimist", "npm"), ("qs", "npm"), ("body-parser", "npm"),
            ("multer", "npm"), ("jsonwebtoken", "npm"), ("uuid", "npm"),
            ("node-fetch", "npm"), ("undici", "npm"), ("tar", "npm"),
            ("helmet", "npm"), ("cors", "npm"), ("cookie", "npm"),
            ("bcrypt", "npm"), ("got", "npm"), ("underscore", "npm"),
            ("path-to-regexp", "npm"), ("debug", "npm"), ("formidable", "npm"),
            ("ssri", "npm"), ("y18n", "npm"), ("node-notifier", "npm"),
        ],
    }

    harvested = 0
    seen_commits = set()

    langs = [language] if language else ["rust", "typescript"]
    for lang in langs:
        packages = PACKAGES.get(lang, [])
        for pkg_name, eco in packages:
            if harvested >= limit:
                break

            vulns = _query_osv(pkg_name, eco)
            for vuln in vulns:
                if harvested >= limit:
                    break

                fix_url = _find_fix_commit(vuln)
                if not fix_url or fix_url in seen_commits:
                    continue
                seen_commits.add(fix_url)

                repo, sha = _parse_github_commit_url(fix_url)
                if not repo or not sha:
                    continue

                pairs = _extract_pairs_from_commit(repo, sha, lang)
                for pair in pairs:
                    if harvested >= limit:
                        break
                    if dry_run:
                        print(f"    Would write: {pair['name']}")
                    else:
                        _write_pair(output_dir, pair)
                    harvested += 1

    return harvested


def _query_osv(pkg_name: str, ecosystem: str) -> list:
    try:
        r = requests.post("https://api.osv.dev/v1/query",
            json={"package": {"name": pkg_name, "ecosystem": ecosystem}}, timeout=10)
        return r.json().get("vulns", [])
    except Exception:
        return []


def _find_fix_commit(vuln: dict) -> Optional[str]:
    for ref in vuln.get("references", []):
        url = ref.get("url", "")
        if "github.com" in url and "/commit/" in url:
            return url
    return None


def _parse_github_commit_url(url: str) -> tuple:
    try:
        parts = url.split("github.com/")[1].split("/commit/")
        repo = parts[0]
        sha = parts[1].split("?")[0]
        return repo, sha
    except (IndexError, ValueError):
        return None, None


def _extract_pairs_from_commit(repo: str, sha: str, language: str) -> list:
    """Extract function-level pairs from a commit diff."""
    pairs = []

    diff = _fetch_diff(repo, sha)
    if not diff:
        return pairs

    ext = EXT_MAP.get(language, "txt")
    files = _parse_diff_files(diff)

    for filepath, file_diff in files.items():
        if not filepath.endswith(f".{ext}"):
            continue

        func_pairs = _extract_functions_from_diff(file_diff, language, filepath)
        pairs.extend(func_pairs)

    return pairs


def _fetch_diff(repo: str, sha: str) -> Optional[str]:
    """Fetch diff for a commit."""
    url = f"https://api.github.com/repos/{repo}/commits/{sha}"
    try:
        r = requests.get(url, headers={"Accept": "application/vnd.github.diff"}, timeout=15)
        if r.status_code == 200:
            return r.text
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
    return {k: "\n".join(v) for k, v in files.items()}


def _extract_functions_from_diff(diff: str, language: str, filepath: str) -> list:
    """Extract function-level before/after pairs."""
    pairs = []
    hunks = _parse_hunks(diff)

    for hunk in hunks:
        before_lines = [l[1:] for l in hunk["removed"] if l.startswith("-")]
        after_lines = [l[1:] for l in hunk["added"] if l.startswith("+")]

        if not before_lines or not after_lines:
            continue

        before_code = "\n".join(before_lines)
        after_code = "\n".join(after_lines)

        if _is_trivial(before_code, after_code):
            continue

        func_name = _extract_func_name(before_code, language) or "fixed_function"
        slug = re.sub(r"[^a-z0-9]", "_", func_name.lower()).strip("_")[:40]
        cwe = "cve"
        name = f"{language}_{cwe}_{slug}"

        pairs.append({
            "name": name,
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


def _extract_func_name(code: str, language: str) -> Optional[str]:
    if language == "rust":
        m = re.search(r"fn\s+(\w+)", code)
    elif language in ("typescript", "javascript"):
        m = re.search(r"(?:function|const|let|var)\s+(\w+)", code)
    else:
        return None
    return m.group(1) if m else None


def _is_trivial(before: str, after: str) -> bool:
    def strip(s):
        return "\n".join(l.strip() for l in s.split("\n")
                         if l.strip() and not l.strip().startswith("//") and not l.strip().startswith("#"))
    return strip(before) == strip(after)


def _write_pair(output_dir: Path, pair: dict):
    ext = EXT_MAP.get(pair["language"], "txt")
    name = pair["name"]
    (output_dir / f"{name}_positive.{ext}").write_text(pair["positive"])
    (output_dir / f"{name}_negative.{ext}").write_text(pair["negative"])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=100)
    parser.add_argument("--output", type=str, default="corpus/targets")
    parser.add_argument("--language", choices=["rust", "typescript"])
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    output_dir = Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)

    count = harvest_github_fixes(output_dir, args.language, args.limit, args.dry_run)
    print(f"\nHarvested: {count} patterns to {output_dir}")
    return 0


if __name__ == "__main__":
    exit(main())
