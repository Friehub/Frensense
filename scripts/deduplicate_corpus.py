#!/usr/bin/env python3
"""
Corpus Deduplication via MinHash

Finds near-duplicate corpus patterns by fingerprinting each positive example
and clustering with MinHash + LSH. Keeps the most structurally diverse
pattern from each cluster.

Usage:
    python3 scripts/deduplicate_corpus.py --corpus corpus/targets --threshold 0.85
    python3 scripts/deduplicate_corpus.py --corpus corpus/targets --dry-run
"""

import argparse
import hashlib
import re
from pathlib import Path
from collections import defaultdict
from typing import List, Tuple, Set


def tokenize(source: str) -> List[str]:
    """Simple tokenizer: split on whitespace and punctuation, lowercase."""
    tokens = re.findall(r"[a-z_]+|\d+|[^\s]", source.lower())
    return tokens


def ngrams(tokens: List[str], n: int = 5) -> Set[str]:
    """Generate n-grams from token list."""
    if len(tokens) < n:
        return {tuple(tokens)} if tokens else set()
    return {tuple(tokens[i:i+n]) for i in range(len(tokens) - n + 1)}


def minhash_signature(ngrams: Set[str], num_hashes: int = 128) -> List[int]:
    """Compute MinHash signature for a set of n-grams."""
    sig = []
    for i in range(num_hashes):
        min_val = float("inf")
        for ng in ngrams:
            h = hash((i, ng)) % (2**32)
            min_val = min(min_val, h)
        sig.append(min_val)
    return sig


def jaccard_similarity(a: Set, b: Set) -> float:
    """Compute Jaccard similarity between two sets."""
    if not a and not b:
        return 1.0
    if not a or not b:
        return 0.0
    return len(a & b) / len(a | b)


def lsh_buckets(signatures: List[List[int]], num_bands: int, rows_per_band: int) -> dict:
    """Bucket items using LSH. Returns {bucket_key: [item_indices]}."""
    buckets = defaultdict(list)
    for idx, sig in enumerate(signatures):
        for band in range(num_bands):
            start = band * rows_per_band
            end = start + rows_per_band
            if end > len(sig):
                break
            band_hash = hash(tuple(sig[start:end]))
            buckets[(band, band_hash)].append(idx)
    return buckets


def find_duplicates(
    corpus_dir: Path,
    threshold: float = 0.85,
) -> List[Tuple[str, str, float]]:
    """Find duplicate patterns in corpus. Returns [(name_a, name_b, similarity)]."""
    # Load all positive files
    positives = {}
    for f in sorted(corpus_dir.glob("*_positive.*")):
        name = f.name.rsplit("_positive.", 1)[0]
        try:
            content = f.read_text()
            positives[name] = content
        except Exception:
            continue

    if len(positives) < 2:
        return []

    # Tokenize and compute n-grams
    names = list(positives.keys())
    ngrams_list = [ngrams(tokenize(positives[n])) for n in names]

    # Compute MinHash signatures
    sigs = [minhash_signature(ng) for ng in ngrams_list]

    # LSH bucketing
    num_bands = 16
    rows_per_band = 128 // num_bands
    buckets = lsh_buckets(sigs, num_bands, rows_per_band)

    # Find candidate pairs from shared buckets
    candidates = set()
    for bucket_indices in buckets.values():
        for i in range(len(bucket_indices)):
            for j in range(i + 1, len(bucket_indices)):
                pair = (min(bucket_indices[i], bucket_indices[j]),
                        max(bucket_indices[i], bucket_indices[j]))
                candidates.add(pair)

    # Compute exact Jaccard for candidates
    duplicates = []
    for i, j in candidates:
        sim = jaccard_similarity(ngrams_list[i], ngrams_list[j])
        if sim >= threshold:
            duplicates.append((names[i], names[j], sim))

    duplicates.sort(key=lambda x: -x[2])
    return duplicates


def cluster_patterns(
    corpus_dir: Path,
    threshold: float = 0.85,
) -> List[List[str]]:
    """Cluster patterns and return list of clusters (each cluster is a list of names)."""
    duplicates = find_duplicates(corpus_dir, threshold)

    # Build a union-find structure
    # Load names
    names = []
    for f in sorted(corpus_dir.glob("*_positive.*")):
        name = f.name.rsplit("_positive.", 1)[0]
        names.append(name)

    name_to_idx = {n: i for i, n in enumerate(names)}
    parent = list(range(len(names)))
    rank = [0] * len(names)

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(x, y):
        rx, ry = find(x), find(y)
        if rx == ry:
            return
        if rank[rx] < rank[ry]:
            rx, ry = ry, rx
        parent[ry] = rx
        if rank[rx] == rank[ry]:
            rank[rx] += 1

    # Cluster by transitive closure of duplicates
    for name_a, name_b, _ in duplicates:
        i = name_to_idx[name_a]
        j = name_to_idx[name_b]
        union(i, j)

    # Group by root
    clusters_dict = defaultdict(list)
    for idx, name in enumerate(names):
        root = find(idx)
        clusters_dict[root].append(name)

    return list(clusters_dict.values())


def main():
    parser = argparse.ArgumentParser(description="Deduplicate corpus patterns")
    parser.add_argument("--corpus", type=str, default="corpus/targets")
    parser.add_argument("--threshold", type=float, default=0.85)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--remove-duplicates", action="store_true",
                        help="Remove duplicate files (keep first in each cluster)")
    args = parser.parse_args()

    corpus_dir = Path(args.corpus)
    if not corpus_dir.exists():
        print(f"Corpus directory not found: {corpus_dir}")
        return 1

    print(f"Scanning {corpus_dir}...")

    # Find duplicates
    duplicates = find_duplicates(corpus_dir, args.threshold)
    print(f"Found {len(duplicates)} duplicate pairs (threshold={args.threshold})")

    for name_a, name_b, sim in duplicates[:20]:
        print(f"  {sim:.2f}: {name_a} <-> {name_b}")

    # Cluster
    clusters = cluster_patterns(corpus_dir, args.threshold)
    multi = [c for c in clusters if len(c) > 1]
    print(f"\n{len(clusters)} total clusters, {len(multi)} with duplicates")

    for cluster in multi:
        print(f"  Cluster ({len(cluster)}): {cluster[0]}")
        for name in cluster[1:]:
            print(f"    -> {name}")

    if args.remove_duplicates and not args.dry_run:
        removed = 0
        for cluster in multi:
            # Keep first, remove rest
            for name in cluster[1:]:
                for ext in ["rs", "ts", "tsx", "js", "jsx", "py"]:
                    pos = corpus_dir / f"{name}_positive.{ext}"
                    neg = corpus_dir / f"{name}_negative.{ext}"
                    if pos.exists():
                        pos.unlink()
                        removed += 1
                    if neg.exists():
                        neg.unlink()
                        removed += 1
        print(f"\nRemoved {removed} duplicate files")

    return 0


if __name__ == "__main__":
    exit(main())
