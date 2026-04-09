#!/bin/bash

BLOCKS_PER_DAY=28800
BLOCKS_PER_QUARTER=$((BLOCKS_PER_DAY * 90))

QUARTERS=(
  "2019-Q1 7200000"
  "2019-Q2 9792000"
  "2019-Q3 12384000"
  "2019-Q4 14976000"
  "2020-Q1 17568000"
  "2020-Q2 20160000"
  "2020-Q3 22752000"
  "2020-Q4 25344000"
  "2021-Q1 27936000"
  "2021-Q2 30528000"
  "2021-Q3 33120000"
  "2021-Q4 35712000"
  "2022-Q1 38304000"
  "2022-Q2 40896000"
  "2022-Q3 43488000"
  "2022-Q4 46080000"
  "2023-Q1 48672000"
  "2023-Q2 51264000"
  "2023-Q3 53856000"
  "2023-Q4 56448000"
  "2024-Q1 59040000"
  "2024-Q2 61632000"
  "2024-Q3 64224000"
  "2024-Q4 66816000"
  "2025-Q1 69408000"
  "2025-Q2 72000000"
  "2025-Q3 74592000"
  "2025-Q4 77184000"
)

for quarter in "${QUARTERS[@]}"; do
  LABEL=$(echo $quarter | awk '{print $1}')
  START=$(echo $quarter | awk '{print $2}')
  END=$((START + BLOCKS_PER_QUARTER))

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Processing $LABEL | blocks $START → $END"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  substreams-sink-files run \
    bin/tron-foundational-v0.1.2.spkg \
    filtered_transactions \
    "./output/$LABEL" \
    --encoder="protojson:.transactions[]" \
    --endpoint mainnet.tron.streamingfast.io:443 \
    --start-block $START \
    --stop-block $END \
    --params filtered_transactions=contract_type:AccountPermissionUpdateContract \
    --state-store "./output/$LABEL/state.yaml"

  echo "✓ Done $LABEL → saved to ./output/$LABEL"
done

echo "All quarters complete."