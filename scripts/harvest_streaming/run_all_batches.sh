#!/usr/bin/env bash
# Run all 9 batches to reach 45,000 patterns
set -euo pipefail
cd "$(dirname "$0")"

TARGET_BATCHES=9
BATCH_SIZE=5000

for i in $(seq 1 $TARGET_BATCHES); do
  echo "=== Batch $i / $TARGET_BATCHES ==="

  echo "[Stage 1] Sweeping NVD for candidates..."
  python3 stage1_nvd_sweep.py "$BATCH_SIZE"

  echo "[Stage 2] Resolving commits..."
  python3 stage2_resolve_commits.py "$i" "$BATCH_SIZE"

  echo "[Stage 3] Extracting before/after content..."
  python3 stage3_extract_diffs.py "$i"

  echo "[Stage 4] Validating and writing corpus pairs..."
  python3 stage4_validate_and_write.py "$i"

  echo "[Stage 5] Deduplicating and rebuilding bundle..."
  cd ../..
  python3 scripts/deduplicate_corpus.py --new corpus/targets/ --since-batch "$i" --jaccard-threshold 0.85
  cargo run --bin build-corpus-bundle
  cd scripts/harvest_streaming

  rm -rf "raw/batch_${i}"
  echo "Batch $i complete. Corpus size: $(ls ../../corpus/targets/*_positive.* | wc -l) patterns."
done

echo "All batches complete."
