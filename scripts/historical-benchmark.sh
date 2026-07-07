#!/bin/bash
# Historical Self-Scan Benchmark
#
# Scans a target repo at every tagged version using the current frensense binary
# and reports how advisory counts evolved over time.
#
# Usage:
#   ./scripts/historical-benchmark.sh <target-repo-path> [scan-subpath] [--sample N] [--last N]
#
#   scan-subpath   Subdirectory to scan within the target repo (default: src)
#   --sample N     Only scan every Nth tag (e.g. --sample 10) for large repos.
#   --last N       Only scan the N most recent tags.
#
# Examples:
#   git clone git@github.com:tokio-rs/tokio.git /tmp/tokio
#   ./scripts/historical-benchmark.sh /tmp/tokio tokio/src --last 10

set -euo pipefail

TARGET_REPO=""
SCAN_PATH="src"
SAMPLE="1"
LAST_N=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --sample)
      SAMPLE="$2"
      shift 2
      ;;
    --last)
      LAST_N="$2"
      shift 2
      ;;
    *)
      if [ -z "$TARGET_REPO" ]; then
        TARGET_REPO="$1"
      elif [ "$SCAN_PATH" == "src" ] && [ "$1" != "$TARGET_REPO" ]; then
        SCAN_PATH="$1"
      fi
      shift
      ;;
  esac
done

if [ -z "$TARGET_REPO" ]; then
  echo "Usage: $0 <target-repo-path> [scan-subpath] [--sample N] [--last N]"
  exit 1
fi

if [ ! -d "$TARGET_REPO/.git" ]; then
  echo "Error: $TARGET_REPO is not a git repository"
  exit 1
fi

FRENSENSE="$(dirname "$0")/../target/release/frensense"
if [ ! -x "$FRENSENSE" ]; then
  echo "Building frensense release binary..."
  cargo build --release --manifest-path "$(dirname "$0")/../Cargo.toml"
fi

echo "building tag list..."
TAGS=($(cd "$TARGET_REPO" && git tag --sort=version:refname))

if [ -n "$LAST_N" ]; then
  # Slice the array to keep only the last N elements
  START_IDX=$(( ${#TAGS[@]} - LAST_N ))
  if [ $START_IDX -lt 0 ]; then
    START_IDX=0
  fi
  TAGS=("${TAGS[@]:$START_IDX}")
fi

echo "found ${#TAGS[@]} tags to scan"

CSV="historical-scan-$(basename "$TARGET_REPO")-$(date +%Y%m%d).csv"
echo "tag,advisories,critical,warning,info" > "$CSV"

COUNT=0
for TAG in "${TAGS[@]}"; do
  # Skip if sampling
  if [ "$SAMPLE" != "1" ] && [ $((COUNT % SAMPLE)) -ne 0 ]; then
    COUNT=$((COUNT + 1))
    continue
  fi
  COUNT=$((COUNT + 1))

  echo -n "[$COUNT/${#TAGS[@]}] checking out $TAG ... "
  (cd "$TARGET_REPO" && git checkout --quiet "$TAG" 2>/dev/null)

  # Run frensense with --json, suppress frensense's own stdout/stderr
  JSON=$("$FRENSENSE" "$TARGET_REPO/$SCAN_PATH" --json 2>/dev/null || echo '{"advisories":[],"advisory_count":0}')

  TOTAL=$(echo "$JSON" | python3 -c "import json,sys; d=json.load(sys.stdin); print(d.get('advisory_count',0))")
  CRIT=$(echo "$JSON" | python3 -c "import json,sys; d=json.load(sys.stdin); print(sum(1 for a in d.get('advisories',[]) if a.get('severity')=='Critical'))")
  WARN=$(echo "$JSON" | python3 -c "import json,sys; d=json.load(sys.stdin); print(sum(1 for a in d.get('advisories',[]) if a.get('severity')=='Warning'))")
  INFO=$(echo "$JSON" | python3 -c "import json,sys; d=json.load(sys.stdin); print(sum(1 for a in d.get('advisories',[]) if a.get('severity')=='Info'))")

  echo "$TAG,$TOTAL,$CRIT,$WARN,$INFO" >> "$CSV"
  echo "ok (total=$TOTAL crit=$CRIT warn=$WARN info=$INFO)"
done

# Restore default branch
(cd "$TARGET_REPO" && git checkout --quiet main 2>/dev/null || git checkout --quiet master 2>/dev/null || true)

echo ""
echo "Done. Results written to $CSV"
echo ""
echo "Quick summary:"
python3 -c "
import csv
with open('$CSV') as f:
    rows = list(csv.DictReader(f))
    totals = [int(r['advisories']) for r in rows]
    print(f'  Tags scanned:  {len(rows)}')
    print(f'  Min findings:  {min(totals)}')
    print(f'  Max findings:  {max(totals)}')
    print(f'  Mean findings: {sum(totals)/len(totals):.1f}')
"
