#!/bin/bash
set -e

# Frensense benchmark runner for Juice Shop
#
# Usage:
#   ./run_juiceshop.sh               # default: v0.5.0 results dir
#   VERSION=v0.6.0 ./run_juiceshop.sh

BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd .. && pwd)"
REPO_ROOT="$(cd "${BASE_DIR}/.." && pwd)"

FRENSENSE_BIN="${REPO_ROOT}/target/release/frensense"

# Benchmark dataset
DATASET_DIR="${BASE_DIR}/datasets/juiceshop"

# Ground-truth labels for the evaluation script
LABELS_JSON="${REPO_ROOT}/scripts/juiceshop-ground-truth.json"

VERSION="${VERSION:-v0.5.0}"
RESULTS_DIR="${BASE_DIR}/results/${VERSION}-juiceshop"
EVAL_SCRIPT="${BASE_DIR}/evaluate.py"

# ---------------------------------------------------------------------------
# Sanity checks
# ---------------------------------------------------------------------------
if [[ ! -f "$FRENSENSE_BIN" ]]; then
    echo "ERROR: frensense binary not found at ${FRENSENSE_BIN}" >&2
    echo "       Run: cargo build --release --bin frensense" >&2
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
echo "Frensense Benchmark — Juice Shop — ${VERSION}"
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
    json_file="${RESULTS_DIR}/juiceshop_threshold_${threshold}.json"
    timing_file="${RESULTS_DIR}/timing_threshold_${threshold}.txt"

    /usr/bin/time -v \
        "$FRENSENSE_BIN" "$DATASET_DIR" \
            --threshold "$threshold" \
            --json \
            --use-compiler \
            > "$json_file" \
            2> "$timing_file" || true

    WALL=$(grep "Elapsed (wall clock)" "$timing_file" | awk -F': ' '{print $2}' || echo "n/a")
    RSS=$(grep "Maximum resident"     "$timing_file" | awk -F': ' '{print $2}' || echo "n/a")
    FINDINGS=$(python3 -c "import json,sys; d=json.load(open('${json_file}')); print(len(d) if isinstance(d,list) else len(d.get('findings',d.get('advisories',[]))))" 2>/dev/null || echo "0")

    echo "  Wall: ${WALL}  |  Peak RSS: ${RSS}  |  Findings: ${FINDINGS}"

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
