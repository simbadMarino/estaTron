CREATE TABLE IF NOT EXISTS tron.transactions
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
    method_id String,
    `from` String
)
ENGINE = ReplacingMergeTree(block_timestamp)
ORDER BY (tx_id);