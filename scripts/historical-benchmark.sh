#!/bin/bash
# Historical Self-Scan Benchmark
#
# Scans a target repo at every tagged version using the current gensense binary
# and reports how advisory counts evolved over time.
#
# Usage:
#   ./scripts/historical-benchmark.sh <target-repo-path> [--sample]
#
#   --sample   Only scan every Nth tag (e.g. --sample 10) for large repos.
#
# Example:
#   git clone git@github.com:tokio-rs/tokio.git /tmp/tokio
#   ./scripts/historical-benchmark.sh /tmp/tokio --sample 5

set -euo pipefail

TARGET_REPO="$1"
SAMPLE="${2:-1}"  # default: every tag

if [ ! -d "$TARGET_REPO/.git" ]; then
  echo "Error: $TARGET_REPO is not a git repository"
  exit 1
fi

GENSENSE="$(dirname "$0")/../target/release/gensense"
if [ ! -x "$GENSENSE" ]; then
  echo "Building gensense release binary..."
  cargo build --release --manifest-path "$(dirname "$0")/../Cargo.toml"
fi

echo "building tag list..."
TAGS=($(cd "$TARGET_REPO" && git tag --sort=version:refname))
echo "found ${#TAGS[@]} tags"

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

  # Run gensense with --json, suppress gensense's own stdout/stderr
  JSON=$("$GENSENSE" "$TARGET_REPO/src" --json 2>/dev/null || echo '{"advisories":[],"advisory_count":0}')

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
