#!/bin/bash
# GenSense Memory Stability Benchmark
# Runs the engine 20 times and logs RSS to detect leaks.

set -e

BINARY="./target/release/frensense"
TARGET="src"
LOG="bench_samples/memory_stability.log"

mkdir -p bench_samples
echo "Iteration, RSS (KB)" > $LOG

echo "Starting Memory Stability Benchmark (20 iterations)..."

for i in {1..20}; do
    RSS=$(/usr/bin/time -f "%M" $BINARY $TARGET 2>&1 > /dev/null)
    echo "$i, $RSS" >> $LOG
    echo "Iteration $i: $RSS KB"
done

echo "Benchmark complete. Results saved to $LOG"
