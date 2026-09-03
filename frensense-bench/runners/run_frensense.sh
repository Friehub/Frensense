#!/bin/bash
set -e

# Frensense benchmark runner — builds a Precision/Recall curve over a threshold sweep.
#
# Usage:
#   ./run_frensense.sh               # default: nodegoat fixtures, v0.5.0 results dir
#   VERSION=v0.6.0 ./run_frensense.sh
#
# Corpus loading priority:
#   1. Uses the FRC bundle embedded in the binary at compile time (fast, canonical).
#   2. Pass --corpus <dir> as an override only when testing experimental patterns.
#      Run `cargo run --bin build-corpus-bundle` first to regenerate the bundle, then
#      `touch src/bin/frensense.rs && cargo build --bin frensense` to embed it.

BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd .. && pwd)"
REPO_ROOT="$(cd "${BASE_DIR}/.." && pwd)"

FRENSENSE_BIN="${REPO_ROOT}/target/debug/frensense"

# Benchmark dataset — synthetic NodeGoat-derived fixtures (committed in the repo)
DATASET_DIR="${BASE_DIR}/datasets/nodegoat"

# Ground-truth labels for the evaluation script
LABELS_JSON="${REPO_ROOT}/scripts/nodegoat-ground-truth.json"

VERSION="${VERSION:-v0.5.0}"
RESULTS_DIR="${BASE_DIR}/results/${VERSION}"
EVAL_SCRIPT="${BASE_DIR}/evaluate.py"

# ---------------------------------------------------------------------------
# Sanity checks
# ---------------------------------------------------------------------------
if [[ ! -f "$FRENSENSE_BIN" ]]; then
    echo "ERROR: frensense binary not found at ${FRENSENSE_BIN}" >&2
    echo "       Run: cargo build --bin frensense" >&2
    exit 1
fi

if [[ ! -d "$DATASET_DIR" ]]; then
    echo "ERROR: dataset directory not found: ${DATASET_DIR}" >&2
    exit 1
fi

if [[ ! -f "$LABELS_JSON" ]]; then
    echo "WARN: ground-truth labels not found at ${LABELS_JSON} — evaluation step will be skipped"
fi

mkdir -p "$RESULTS_DIR"

FILE_COUNT=$(find "$DATASET_DIR" -type f \( -name "*.ts" -o -name "*.js" \) | wc -l)
LOC=$(find "$DATASET_DIR" -type f \( -name "*.ts" -o -name "*.js" \) -exec wc -l {} + | tail -1 | awk '{print $1}')

echo "=========================================="
echo "Frensense Benchmark  —  ${VERSION}"
echo "  Binary : ${FRENSENSE_BIN}"
echo "  Dataset: ${DATASET_DIR} (${FILE_COUNT} files, ~${LOC} LOC)"
echo "  Results: ${RESULTS_DIR}"
echo "=========================================="

# ---------------------------------------------------------------------------
# Threshold sweep
# ---------------------------------------------------------------------------
for threshold in 0.20 0.30 0.40 0.50 0.60 0.70; do
    echo ""
    echo "--- threshold=${threshold} ---"
    json_file="${RESULTS_DIR}/nodegoat_threshold_${threshold}.json"
    timing_file="${RESULTS_DIR}/timing_threshold_${threshold}.txt"

    # Measure wall-time and peak RSS alongside the scan.
    # The binary uses its embedded FRC bundle (no --corpus flag needed).
    /usr/bin/time -v \
        "$FRENSENSE_BIN" "$DATASET_DIR" \
            --threshold "$threshold" \
            --json \
            > "$json_file" \
            2> "$timing_file" || true   # never abort the sweep on non-zero exit

    WALL=$(grep "Elapsed (wall clock)" "$timing_file" | awk -F': ' '{print $2}' || echo "n/a")
    RSS=$(grep "Maximum resident"     "$timing_file" | awk -F': ' '{print $2}' || echo "n/a")
    FINDINGS=$(python3 -c "import json,sys; d=json.load(open('${json_file}')); print(len(d) if isinstance(d,list) else len(d.get('findings',d.get('advisories',[]))))" 2>/dev/null || echo "0")

    echo "  Wall: ${WALL}  |  Peak RSS: ${RSS}  |  Findings: ${FINDINGS}"

    # Evaluate against ground truth if labels are present
    if [[ -f "$LABELS_JSON" ]]; then
        echo "  Evaluating..."
        python3 "$EVAL_SCRIPT" \
            --labels "$LABELS_JSON" \
            --json-file "$json_file" \
            --threshold "$threshold" \
            --output "${RESULTS_DIR}/metrics_threshold_${threshold}.json" \
            2>/dev/null || echo "  (evaluate.py failed — check script)"
    fi
done

echo ""
echo "=========================================="
echo "Benchmark complete. Results in ${RESULTS_DIR}"
echo "=========================================="
