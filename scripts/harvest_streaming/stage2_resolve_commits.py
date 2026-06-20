"""Stage 2: Resolve each candidate to a confirmed Rust/TypeScript commit."""
import os
import re
import time
import requests
from typing import Optional
from checkpoint import get_db

# Load .env file if present
_env_path = os.path.join(os.path.dirname(__file__), "..", "..", ".env")
if os.path.exists(_env_path):
    with open(_env_path) as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, value = line.split("=", 1)
                os.environ.setdefault(key.strip(), value.strip())

GITHUB_API = "https://api.github.com"
TOKEN = os.environ.get("GITHUB_TOKEN", "")
HEADERS = {"Authorization": f"Bearer {TOKEN}", "Accept": "application/vnd.github+json"} if TOKEN else {"Accept": "application/vnd.github+json"}

RUST_EXT = (".rs",)
TS_EXT = (".ts", ".tsx")

RUST_PRIORITY_ORGS = {"tokio-rs", "rust-lang", "actix", "hyperium", "serde-rs", "RustCrypto",
                       "rustsec", "rust-secure-code", "diesel-rs", "tauri-apps"}


def gh_get(url: str, **kwargs) -> requests.Response:
    resp = requests.get(url, headers=HEADERS, timeout=30, **kwargs)
    remaining = int(resp.headers.get("X-RateLimit-Remaining", 1))
    if remaining < 5:
        reset = int(resp.headers.get("X-RateLimit-Reset", time.time() + 60))
        sleep_for = max(reset - time.time(), 1)
        print(f"  rate limit low, sleeping {sleep_for:.0f}s")
        time.sleep(sleep_for)
    return resp


def resolve_sha(repo: str, ref_url: str) -> Optional[str]:
    commit_match = re.search(r"/commit/([0-9a-f]{7,40})", ref_url)
    if commit_match:
        return commit_match.group(1)

    pr_match = re.search(r"/pull/(\d+)", ref_url)
    if pr_match:
        pr_num = pr_match.group(1)
        resp = gh_get(f"{GITHUB_API}/repos/{repo}/pulls/{pr_num}/commits")
        if resp.status_code == 200:
            commits = resp.json()
            if commits:
                return commits[-1]["sha"]

    advisory_match = re.search(r"/security/advisories/(GHSA-[\w-]+)", ref_url)
    if advisory_match:
        ghsa_id = advisory_match.group(1)
        resp = gh_get(f"{GITHUB_API}/advisories/{ghsa_id}")
        if resp.status_code == 200:
            refs = resp.json().get("references", [])
            for r in refs:
                url = r.get("url", "") if isinstance(r, dict) else str(r)
                cm = re.search(r"/commit/([0-9a-f]{7,40})", url)
                if cm:
                    return cm.group(1)
    return None


def confirm_language(repo: str, sha: str) -> Optional[str]:
    resp = gh_get(f"{GITHUB_API}/repos/{repo}/commits/{sha}")
    if resp.status_code != 200:
        return None
    files = resp.json().get("files", [])
    has_rust = any(f["filename"].endswith(RUST_EXT) for f in files)
    has_ts = any(f["filename"].endswith(TS_EXT) for f in files)
    if has_rust:
        return "rust"
    if has_ts:
        return "typescript"
    return None


def resolve_batch(batch_id: int, limit: int):
    db = get_db()
    rows = db.execute(
        "SELECT cve_id, repo, ref_url FROM candidates WHERE status = 'pending' LIMIT ?",
        (limit,),
    ).fetchall()

    rows = sorted(rows, key=lambda r: r[1].split("/")[0] not in RUST_PRIORITY_ORGS)

    resolved = 0
    for cve_id, repo, ref_url in rows:
        sha = resolve_sha(repo, ref_url)
        if not sha:
            db.execute("UPDATE candidates SET status = 'unresolvable' WHERE cve_id=? AND repo=?",
                       (cve_id, repo))
            continue
        lang = confirm_language(repo, sha)
        if not lang:
            db.execute("UPDATE candidates SET status = 'wrong_language' WHERE cve_id=? AND repo=?",
                       (cve_id, repo))
            continue
        db.execute(
            """UPDATE candidates SET resolved_sha=?, lang=?, status='confirmed', batch=?
               WHERE cve_id=? AND repo=?""",
            (sha, lang, batch_id, cve_id, repo),
        )
        resolved += 1
        if resolved % 10 == 0:
            db.commit()
            total_confirmed = db.execute("SELECT COUNT(*) FROM candidates WHERE status='confirmed'").fetchone()[0]
            print(f"  resolved {resolved} commits (total confirmed: {total_confirmed})")
    db.commit()
    print(f"Stage 2 done: {resolved} confirmed Rust/TypeScript commits in batch {batch_id}.")


if __name__ == "__main__":
    import sys
    from typing import Optional
    resolve_batch(batch_id=int(sys.argv[1]), limit=int(sys.argv[2]) if len(sys.argv) > 2 else 5000)
