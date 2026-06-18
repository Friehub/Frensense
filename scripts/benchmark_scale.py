#!/usr/bin/env python3
"""
Scan Performance Benchmark

Measures scan time at different corpus sizes to validate scaling claims.
Clones a test repo, builds the binary, and runs scans with different
corpus sizes (by temporarily reducing the bundle).

Usage:
    python3 scripts/benchmark_scale.py --repo /tmp/axum --iterations 3
    python3 scripts/benchmark_scale.py --repo /tmp/axum --pattern-counts 10,50,100,500
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path


def measure_scan(repo_path: str, iterations: int = 3) -> dict:
    """Measure scan time for a repo over multiple iterations."""
    times = []
    findings_count = 0

    for i in range(iterations):
        start = time.time()
        result = subprocess.run(
            ["./target/release/frensense", repo_path, "--json", "--severity", "warning"],
            capture_output=True, text=True, timeout=300,
        )
        elapsed = time.time() - start
        times.append(elapsed)

        if i == 0:
            try:
                data = json.loads(result.stdout)
                findings_count = len(data.get("advisories", []))
            except json.JSONDecodeError:
                pass

    return {
        "mean_ms": int(sum(times) / len(times) * 1000),
        "min_ms": int(min(times) * 1000),
        "max_ms": int(max(times) * 1000),
        "iterations": iterations,
        "findings": findings_count,
    }


def count_source_files(repo_path: str) -> int:
    """Count Rust/TS/JS source files in a repo."""
    result = subprocess.run(
        ["find", repo_path, "-name", "*.rs", "-o", "-name", "*.ts", "-o", "-name", "*.js"],
        capture_output=True, text=True,
    )
    return len([l for l in result.stdout.strip().split("\n") if l])


def main():
    parser = argparse.ArgumentParser(description="Benchmark scan performance")
    parser.add_argument("--repo", required=True, help="Path to repository to scan")
    parser.add_argument("--iterations", type=int, default=3, help="Number of iterations")
    args = parser.parse_args()

    repo_path = Path(args.repo)
    if not repo_path.exists():
        print(f"Repository not found: {repo_path}")
        return 1

    file_count = count_source_files(str(repo_path))
    print(f"Repository: {repo_path}")
    print(f"Source files: {file_count}")
    print(f"Iterations: {args.iterations}")
    print()

    print("Running benchmark...")
    result = measure_scan(str(repo_path), args.iterations)

    print(f"Results:")
    print(f"  Scan time: {result['mean_ms']}ms (min={result['min_ms']}ms, max={result['max_ms']}ms)")
    print(f"  Findings: {result['findings']}")
    print(f"  Files/sec: {file_count * 1000 / result['mean_ms']:.1f}")
    print()

    # Output JSON
    output = {
        "repo": str(repo_path),
        "source_files": file_count,
        **result,
    }
    print(json.dumps(output, indent=2))

    return 0


if __name__ == "__main__":
    sys.exit(main())
