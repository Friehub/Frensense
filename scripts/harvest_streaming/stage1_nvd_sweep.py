"""Stage 1: NVD Sweep — Query NVD for CVEs with GitHub fix references."""
import os
import re
import time
import requests
from datetime import datetime, timedelta
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

NVD_BASE = "https://services.nvd.nist.gov/rest/json/cves/2.0"
API_KEY = os.environ.get("NVD_API_KEY")
RATE_DELAY = 0.6 if API_KEY else 6.0

PRIORITY_CWES = {
    "CWE-89", "CWE-78", "CWE-79", "CWE-22", "CWE-918", "CWE-502",
    "CWE-416", "CWE-476", "CWE-362", "CWE-190", "CWE-787", "CWE-617",
    "CWE-369", "CWE-400", "CWE-770", "CWE-94", "CWE-352", "CWE-601",
}

GITHUB_REF_RE = re.compile(r"github\.com/([\w.-]+)/([\w.-]+)/(?:commit|pull|security/advisories)/")


def fetch_window(start: str, end: str, start_index: int = 0) -> dict:
    params = {
        "pubStartDate": f"{start}T00:00:00.000",
        "pubEndDate": f"{end}T00:00:00.000",
        "resultsPerPage": 2000,
        "startIndex": start_index,
    }
    headers = {"apiKey": API_KEY} if API_KEY else {}
    for attempt in range(3):
        try:
            resp = requests.get(NVD_BASE, params=params, headers=headers, timeout=60)
            resp.raise_for_status()
            return resp.json()
        except requests.exceptions.HTTPError as e:
            if resp.status_code == 503:
                time.sleep(30 * (attempt + 1))
                continue
            raise
        except requests.exceptions.Timeout:
            time.sleep(10 * (attempt + 1))
            continue
    return {"totalResults": 0, "vulnerabilities": []}


def extract_candidates(cve_record: dict) -> list:
    out = []
    cve = cve_record["cve"]
    cve_id = cve["id"]

    cwe_ids = {
        d["value"]
        for w in cve.get("weaknesses", [])
        for d in w.get("description", [])
        if d["value"].startswith("CWE-")
    }
    relevant_cwes = cwe_ids & PRIORITY_CWES
    if not relevant_cwes:
        return out

    for ref in cve.get("references", []):
        url = ref.get("url", "")
        m = GITHUB_REF_RE.search(url)
        if not m:
            continue
        owner, repo = m.group(1), m.group(2)
        out.append({
            "cve_id": cve_id,
            "cwe_id": sorted(relevant_cwes)[0],
            "repo": f"{owner}/{repo}",
            "ref_url": url,
        })
    return out


def sweep(batch_size: int, target_lang_hint: bool = True):
    db = get_db()
    written_this_batch = 0
    cur_end = datetime.utcnow()

    row = db.execute(
        "SELECT MIN(window_start) FROM nvd_progress WHERE completed = 1"
    ).fetchone()
    if row and row[0]:
        cur_end = datetime.fromisoformat(row[0])

    while written_this_batch < batch_size:
        cur_start = cur_end - timedelta(days=119)
        s, e = cur_start.strftime("%Y-%m-%d"), cur_end.strftime("%Y-%m-%d")

        already_done = db.execute(
            "SELECT completed FROM nvd_progress WHERE window_start = ?", (s,)
        ).fetchone()
        if already_done and already_done[0]:
            cur_end = cur_start
            continue

        start_index = 0
        while True:
            data = fetch_window(s, e, start_index)
            total = data.get("totalResults", 0)
            for vuln in data.get("vulnerabilities", []):
                for cand in extract_candidates(vuln):
                    db.execute(
                        """INSERT OR IGNORE INTO candidates
                           (cve_id, cwe_id, repo, ref_url, status, batch)
                           VALUES (?, ?, ?, ?, 'pending', NULL)""",
                        (cand["cve_id"], cand["cwe_id"], cand["repo"], cand["ref_url"]),
                    )
                    written_this_batch += 1
            db.commit()
            start_index += 2000
            time.sleep(RATE_DELAY)
            if start_index >= total:
                break

        db.execute(
            "INSERT OR REPLACE INTO nvd_progress (window_start, window_end, completed) VALUES (?, ?, 1)",
            (s, e),
        )
        db.commit()
        total_candidates = db.execute("SELECT COUNT(*) FROM candidates").fetchone()[0]
        print(f"  swept {s}..{e}: {written_this_batch} new candidates (total: {total_candidates})")
        cur_end = cur_start

    print(f"Stage 1 done: {written_this_batch} new candidates queued.")


if __name__ == "__main__":
    import sys
    sweep(batch_size=int(sys.argv[1]) if len(sys.argv) > 1 else 5000)
