# 🚀 ClickHouse + TRON Data Pipeline — Summary & Best Practices

## 🧠 Overview

The following covers building a **high-scale ClickHouse pipeline** for TRON transaction data (~1B rows), including:

- Efficient ingestion from JSONL
- Handling duplicates at scale
- Schema design for compression & performance
- Query optimization
- Debugging memory and timestamp issues

---

# 🏗️ Final Recommended Architecture

## ✅ Table Schema (Production-Ready)

```sql
CREATE TABLE tron.transactions
(
    tx_id String,
    contract_type String,
    total_fee_burn Int64,
    energy_usage_total Int64,
    energy_from_stake Int64,
    net_from_burn Int64,
    net_from_stake Int64,
    block_number UInt64,
    block_timestamp Int64,
    contract_address String,
    signature_count UInt8,
    perm_threshold String,
    `from` String
)
ENGINE = ReplacingMergeTree(block_timestamp)
ORDER BY (block_number, tx_id);
```

🧠 Key Design Decisions
Feature	Why
ReplacingMergeTree	Handles deduplication
block_timestamp version	Keeps latest record
ORDER BY (block_number, tx_id)	Preserves compression & locality
👉 Loses control over ordering and compression

✅ Correct Workflow
-- Step 1: Create table
CREATE TABLE ...

-- Step 2: Insert with ordering
INSERT INTO transactions_dedup
SELECT *
FROM transactions
ORDER BY block_number, tx_id;

-- Step 3: Deduplicate
OPTIMIZE TABLE transactions_dedup FINAL;

🔁 Deduplication Strategy
❗ Important
Deduplication is NOT immediate
Happens during merges
🔍 Check duplicates (fast)
SELECT
    count() AS total_rows,
    uniq(tx_id) AS unique_tx,
    total_rows - unique_tx AS duplicate_rows
FROM transactions;

⚠️ Note
uniq() is approximate → can produce small negative values
Use exact check if needed:
SELECT
    count(),
    uniqExact(tx_id)
FROM transactions;


📊 Monitor size
SELECT
    formatReadableSize(sum(data_compressed_bytes)) AS compressed,
    formatReadableSize(sum(data_uncompressed_bytes)) AS uncompressed,
    round(sum(data_uncompressed_bytes) / sum(data_compressed_bytes), 2) AS ratio
FROM system.parts
WHERE database = 'your_database'
  AND table = 'transactions';



🧠 Key Learnings
1. ORDER BY is critical

Compression depends more on ORDER BY than codecs

2. Insert order matters
Table ORDER BY = structure
Insert ORDER BY = data quality
3. Dedup is asynchronous
Happens during merges
Not during insert
4. Avoid full GROUP BY on high cardinality
Use uniq() instead
Or engine-level dedup
5. ClickHouse philosophy

🔥 Next Steps (Optional)

Build Grafana dashboards
Create materialized views
Track TRON metrics (TPS, fees, contract usage)