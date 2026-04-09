#!/bin/bash

BLOCKS_PER_DAY=10000  #Max amount of blocks is 
BLOCKS_PER_MONTH=$((BLOCKS_PER_DAY * 1))

# Format: "START_BLOCK MONTHS"
RANGES=(
  "7871320 1"      # 10k blocks from this block   7871320 (2019-03-28) AccountUpdatePersmission close to implementation date
  "10000000 1"     # 10k blocks from this block
  "15000000 1"     # 10k blocks from this block
  "20000000 1"     # 10k blocks from this block
  "25000000 1"     # 10k blocks from this block
  "30000000 1"    # 10k blocks from this block
  "35000000 1"    # 10k blocks from this block
  "40000000 1"    # 10k blocks from this block
  "45000000 1"    # 10k blocks from this block
  "50000000 1"    # 10k blocks from this block
  "55000000 1"    # 10k blocks from this block
  "60000000 1"    # 10k blocks from this block
  "65000000 1"    # 10k blocks from this block
  "70000000 1"    # 10k blocks from this block
  "75000000 1"    # 10k blocks from this block
  "80000000 1"    # 10k blocks from this block
  "81024914 1"    # 10k blocks from this block   Closer to 2026
)

for range in "${RANGES[@]}"; do
  START=$(echo $range | awk '{print $1}')
  MONTHS=$(echo $range | awk '{print $2}')
  BLOCK_COUNT=$((MONTHS * BLOCKS_PER_MONTH))

  echo "Running $MONTHS month(s) from block $START (+$BLOCK_COUNT blocks)..."

  substreams run bin/tron-foundational-v0.1.2.spkg filtered_transactions \
    -e mainnet.tron.streamingfast.io:443 \
    -s $START -t +$BLOCK_COUNT \
    -p filtered_transactions=contract_type:AccountPermissionUpdateContract \
    --output jsonl >> AccountPermOutput2.jsonl

  echo "Done: $START → +$BLOCK_COUNT"
done
