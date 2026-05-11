#!/bin/bash
# GenSense Benchmark Script
# Part of the Stabilization Phase (DISCIPLINE.md Priority 3)

set -e

# Ensure binary is built
cargo build --release --features cli

BINARY="./target/release/gensense"
SAMPLES=(
    "src"
    "tests/samples"
)

echo "### GenSense Performance Benchmark ###"
echo "Date: $(date)"
echo "OS: $(uname -a)"
echo ""

for sample in "${SAMPLES[@]}"; do
    if [ -d "$sample" ]; then
        echo "Benchmarking path: $sample"
        # Run 3 times and take average (simplified)
        for i in {1..3}; do
            /usr/bin/time -f "Run $i: %e seconds (RSS: %M KB)" $BINARY "$sample" 2>&1 > /dev/null
        done
        echo "-----------------------------------"
    fi
done
