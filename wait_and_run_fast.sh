#!/bin/bash
echo "Waiting for frensense-corpus.frc to be updated..."
OLD_STAT=$(stat -c %Y frensense-corpus.frc)
while true; do
    NEW_STAT=$(stat -c %Y frensense-corpus.frc)
    if [ "$NEW_STAT" != "$OLD_STAT" ]; then
        echo "Updated!"
        break
    fi
    sleep 5
done

echo "Running benchmark..."
JUICE_SHOP_DIR=/home/oxisrael/Friehub/Taas/juice-shop python3 scripts/benchmark_juice_shop.py --frensense-bin ./target/release/frensense > benchmark_results_fast.txt
cat benchmark_results_fast.txt
