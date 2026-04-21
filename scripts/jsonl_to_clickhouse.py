#!/usr/bin/env python3
"""
sink_jsonl_to_clickhouse.py
----------------------------
Sinks all *.jsonl files in the current directory into a ClickHouse table.

Usage:
    python sink_jsonl_to_clickhouse.py [OPTIONS]

Options:
    --host        ClickHouse host          (default: localhost)
    --port        ClickHouse native port   (default: 9000)
    --user        ClickHouse user          (default: default)
    --password    ClickHouse password      (default: "")
    --database    Target database          (default: default)
    --table       Target table             (default: transactions)
    --batch-size  Rows per insert batch    (default: 10000)
    --create-table  Auto-create table if it doesn't exist (flag)

Requirements:
    pip install clickhouse-driver
"""

import argparse
import glob
import json
import logging
import os
import sys
from pathlib import Path

try:
    from clickhouse_driver import Client
except ImportError:
    print("ERROR: clickhouse-driver is not installed.")
    print("       Run: pip install clickhouse-driver")
    sys.exit(1)

# ---------------------------------------------------------------------------
# Logging
# ---------------------------------------------------------------------------
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s  %(levelname)-8s  %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
)
log = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# DDL — matches the JSONL schema exactly
# ---------------------------------------------------------------------------
CREATE_TABLE_SQL = """
CREATE TABLE IF NOT EXISTS {database}.{table}
(
    tx_id             String,
    contract_type     String,
    total_fee_burn    Int64,
    energy_usage_total Int64,
    energy_from_stake Int64,
    net_from_burn     Int64,
    net_from_stake    Int64,
    block_number      UInt64,
    block_timestamp   Int64,
    contract_address  String,
    signature_count   UInt8,
    perm_threshold    String,
    `from`            String
)
ENGINE = MergeTree()
ORDER BY (block_number, tx_id)
SETTINGS index_granularity = 8192;
"""

# Ordered list of columns — must match the DDL above
COLUMNS = [
    "tx_id",
    "contract_type",
    "total_fee_burn",
    "energy_usage_total",
    "energy_from_stake",
    "net_from_burn",
    "net_from_stake",
    "block_number",
    "block_timestamp",
    "contract_address",
    "signature_count",
    "perm_threshold",
    "from",
]

# Contract_type Black List for the sake of filtering unwanted transactions from jsonl files
EXCLUDED_CONTRACT_TYPES = {"57","58"}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def parse_row(line: str, lineno: int, filepath: str):
    """Parse a single JSONL line into an ordered tuple. Returns None on error."""
    line = line.strip()
    if not line:
        return None
    try:
        obj = json.loads(line)
    except json.JSONDecodeError as exc:
        log.warning("  Skipping malformed JSON at %s:%d — %s", filepath, lineno, exc)
        return None

    if obj.get("contract_type") in EXCLUDED_CONTRACT_TYPES:
        return None

    return tuple(obj.get(col, None) for col in COLUMNS)


def insert_batch(client: Client, database: str, table: str, batch: list):
    """Insert a list of row-tuples into ClickHouse."""
    col_list = ", ".join(f"`{c}`" for c in COLUMNS)
    client.execute(
        f"INSERT INTO {database}.{table} ({col_list}) VALUES",
        batch,
    )


def process_file(client: Client, filepath: str, database: str, table: str, batch_size: int) -> int:
    """Stream a single JSONL file into ClickHouse. Returns total rows inserted."""
    log.info("Processing: %s", filepath)
    batch = []
    total = 0
    errors = 0

    with open(filepath, "r", encoding="utf-8") as fh:
        for lineno, line in enumerate(fh, start=1):
            row = parse_row(line, lineno, filepath)
            if row is None:
                errors += 1
                continue
            batch.append(row)

            if len(batch) >= batch_size:
                insert_batch(client, database, table, batch)
                total += len(batch)
                log.info("  → Inserted %d rows (cumulative: %d)", len(batch), total)
                batch = []

    if batch:
        insert_batch(client, database, table, batch)
        total += len(batch)
        log.info("  → Inserted final %d rows (cumulative: %d)", len(batch), total)

    log.info("  ✓ Done — %d rows inserted, %d lines skipped", total, errors)
    return total


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Sink JSONL files into ClickHouse.")
    parser.add_argument("--host",         default="localhost",    help="ClickHouse host")
    parser.add_argument("--port",         default=9000, type=int, help="ClickHouse native port")
    parser.add_argument("--user",         default="default",      help="ClickHouse user")
    parser.add_argument("--password",     default="",             help="ClickHouse password")
    parser.add_argument("--database",     default="tron",      help="Target database")
    parser.add_argument("--table",        default="transactions",  help="Target table name")
    parser.add_argument("--batch-size",   default=200_000, type=int, help="Rows per insert batch")
    parser.add_argument("--create-table", action="store_true",    help="Auto-create table if missing")
    parser.add_argument("--dir",          default="../jsonl_sink/output",            help="Directory to scan for *.jsonl files")
    args = parser.parse_args()

    # --- Connect -----------------------------------------------------------
    log.info("Connecting to ClickHouse at %s:%d (user=%s, db=%s)", args.host, args.port, args.user, args.database)
    try:
        client = Client(
            host=args.host,
            port=args.port,
            user=args.user,
            password=args.password,
            database=args.database,
            connect_timeout=10,
            compression=True,
        )
        client.execute("SELECT 1")
        log.info("Connection OK.")
    except Exception as exc:
        log.error("Could not connect to ClickHouse: %s", exc)
        sys.exit(1)

    # --- Optionally create table -------------------------------------------
    if args.create_table:
        ddl = CREATE_TABLE_SQL.format(database=args.database, table=args.table)
        log.info("Creating table %s.%s if it doesn't exist...", args.database, args.table)
        client.execute(ddl)
        log.info("Table ready.")

    # --- Discover JSONL files ----------------------------------------------
    pattern = os.path.join(args.dir, "*.jsonl")
    files = sorted(glob.glob(pattern))
    if not files:
        log.warning("No *.jsonl files found in '%s'. Nothing to do.", args.dir)
        sys.exit(0)

    log.info("Found %d JSONL file(s) to process.", len(files))

    # --- Process each file -------------------------------------------------
    grand_total = 0
    for filepath in files:
        try:
            grand_total += process_file(client, filepath, args.database, args.table, args.batch_size)
        except Exception as exc:
            log.error("Failed to process %s: %s", filepath, exc)
            raise   # re-raise so the error is visible; remove to continue on failure

    log.info("=" * 60)
    log.info("All done. Total rows inserted across all files: %d", grand_total)


if __name__ == "__main__":
    main()