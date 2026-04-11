CREATE TABLE IF NOT EXISTS transactions (
    id VARCHAR PRIMARY KEY,
    tx_id VARCHAR,
    timestamp BIGINT,
    result BOOLEAN,
    contract_type VARCHAR,
    fee BIGINT,
    energy_used BIGINT,
    energy_penalty BIGINT,
    block_number BIGINT,
    block_timestamp BIGINT
);