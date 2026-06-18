#!/usr/bin/env python3
"""
Bundle Manifest Updater

Tracks SHA-256 hashes of corpus source files to enable incremental
FRC bundle rebuilds. Only files that changed since last build get
re-fingerprinted.

Usage:
    python3 scripts/update_bundle_manifest.py --corpus corpus/targets
    python3 scripts/update_bundle_manifest.py --corpus corpus/targets --check
"""

import argparse
import hashlib
import json
from pathlib import Path

MANIFEST_FILE = "corpus/targets/.bundle_manifest.json"


def compute_file_hash(path: Path) -> str:
    """Compute SHA-256 hash of a file."""
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()


def load_manifest(manifest_path: Path) -> dict:
    """Load existing manifest."""
    if manifest_path.exists():
        return json.loads(manifest_path.read_text())
    return {"files": {}, "version": 1}


def save_manifest(manifest_path: Path, manifest: dict):
    """Save manifest."""
    manifest_path.write_text(json.dumps(manifest, indent=2))


def scan_corpus(corpus_dir: Path) -> dict:
    """Scan corpus directory and return {filename: hash} for all source files."""
    files = {}
    for f in sorted(corpus_dir.iterdir()):
        if f.is_file() and "_positive." in f.name or "_negative." in f.name:
            if not f.name.startswith("."):
                files[f.name] = compute_file_hash(f)
    return files


def main():
    parser = argparse.ArgumentParser(description="Update bundle manifest")
    parser.add_argument("--corpus", type=str, default="corpus/targets")
    parser.add_argument("--check", action="store_true",
                        help="Only check for changes, don't update manifest")
    args = parser.parse_args()

    corpus_dir = Path(args.corpus)
    manifest_path = corpus_dir / ".bundle_manifest.json"

    manifest = load_manifest(manifest_path)
    current_files = scan_corpus(corpus_dir)

    # Find changes
    added = set(current_files.keys()) - set(manifest.get("files", {}).keys())
    removed = set(manifest.get("files", {}).keys()) - set(current_files.keys())
    modified = {
        name for name in current_files
        if name in manifest.get("files", {})
        and current_files[name] != manifest["files"][name]
    }

    if not added and not removed and not modified:
        print("No changes detected. Bundle is up to date.")
        return 0

    print(f"Changes detected:")
    if added:
        print(f"  Added: {len(added)} files")
        for f in sorted(added):
            print(f"    + {f}")
    if removed:
        print(f"  Removed: {len(removed)} files")
        for f in sorted(removed):
            print(f"    - {f}")
    if modified:
        print(f"  Modified: {len(modified)} files")
        for f in sorted(modified):
            print(f"    ~ {f}")

    if args.check:
        print("\nManifest NOT updated (--check mode)")
        return 1

    # Update manifest
    manifest["files"] = current_files
    manifest["version"] = 1
    save_manifest(manifest_path, manifest)
    print(f"\nManifest updated: {manifest_path}")
    return 0


if __name__ == "__main__":
    exit(main())
