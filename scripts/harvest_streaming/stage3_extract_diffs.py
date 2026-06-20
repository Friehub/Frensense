"""Stage 3: Extract before/after file content for each confirmed commit."""
import base64
import time
from pathlib import Path
from typing import Optional
from checkpoint import get_db
from stage2_resolve_commits import gh_get, GITHUB_API


def get_file_at_ref(repo: str, path: str, ref: str) -> Optional[str]:
    resp = gh_get(f"{GITHUB_API}/repos/{repo}/contents/{path}", params={"ref": ref})
    if resp.status_code != 200:
        return None
    data = resp.json()
    if data.get("encoding") != "base64":
        return None
    return base64.b64decode(data["content"]).decode("utf-8", errors="replace")


def get_changed_files(repo: str, sha: str, lang: str) -> list:
    resp = gh_get(f"{GITHUB_API}/repos/{repo}/commits/{sha}")
    if resp.status_code != 200:
        return []
    commit = resp.json()
    parent_sha = commit["parents"][0]["sha"] if commit.get("parents") else None
    if not parent_sha:
        return []
    ext = (".rs",) if lang == "rust" else (".ts", ".tsx")
    return [
        {"path": f["filename"], "parent_sha": parent_sha}
        for f in commit.get("files", [])
        if f["filename"].endswith(ext) and f["status"] == "modified"
    ]


def extract_batch(batch_id: int, raw_output_dir: Path):
    db = get_db()
    rows = db.execute(
        "SELECT cve_id, cwe_id, repo, resolved_sha, lang FROM candidates WHERE batch=? AND status='confirmed'",
        (batch_id,),
    ).fetchall()

    raw_output_dir.mkdir(parents=True, exist_ok=True)
    extracted = 0

    for cve_id, cwe_id, repo, sha, lang in rows:
        changed = get_changed_files(repo, sha, lang)
        for cf in changed:
            after_code = get_file_at_ref(repo, cf["path"], sha)
            before_code = get_file_at_ref(repo, cf["path"], cf["parent_sha"])
            if not after_code or not before_code or before_code == after_code:
                continue

            safe_name = f"{cve_id}__{repo.replace('/', '_')}__{Path(cf['path']).stem}"
            ext = ".rs" if lang == "rust" else ".ts"
            (raw_output_dir / f"{safe_name}{ext}.before").write_text(before_code)
            (raw_output_dir / f"{safe_name}{ext}.after").write_text(after_code)
            (raw_output_dir / f"{safe_name}.meta").write_text(f"{cve_id}\t{cwe_id}\t{lang}\t{repo}\t{sha}")
            extracted += 1
            time.sleep(0.1)

        if extracted % 100 == 0 and extracted > 0:
            print(f"  extracted {extracted} raw file pairs so far...")

    print(f"Stage 3 done: {extracted} raw before/after file pairs written to {raw_output_dir}")


if __name__ == "__main__":
    import sys
    from typing import Optional
    extract_batch(batch_id=int(sys.argv[1]), raw_output_dir=Path(f"raw/batch_{sys.argv[1]}"))
