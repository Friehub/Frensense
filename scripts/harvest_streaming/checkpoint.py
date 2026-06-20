"""Tiny SQLite checkpoint store — metadata only, never source code."""
import sqlite3
from pathlib import Path

SCHEMA = """
CREATE TABLE IF NOT EXISTS candidates (
    cve_id TEXT, cwe_id TEXT, repo TEXT, ref_url TEXT,
    resolved_sha TEXT, lang TEXT, status TEXT DEFAULT 'pending',
    batch INTEGER, PRIMARY KEY (cve_id, repo)
);
CREATE TABLE IF NOT EXISTS written_pairs (
    pattern_name TEXT PRIMARY KEY, cve_id TEXT, batch INTEGER, fingerprint_hash TEXT
);
CREATE TABLE IF NOT EXISTS nvd_progress (
    window_start TEXT PRIMARY KEY, window_end TEXT, completed INTEGER DEFAULT 0
);
"""


def get_db(path: str = "harvest_checkpoint.db") -> sqlite3.Connection:
    conn = sqlite3.connect(path)
    conn.executescript(SCHEMA)
    return conn
