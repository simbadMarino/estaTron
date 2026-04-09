#[allow(unused)]
mod pb;

use crate::pb::sf::substreams::sink::files::v1::Lines;
use crate::pb::sf::substreams::tron::v1::Transactions;

#[substreams::handlers::map]
fn map_my_data(transactions: Transactions) -> Result<Lines, substreams::errors::Error> {
    let mut lines = vec![];

    for tx in transactions.transactions {
        let contract_type = tx.contracts
            .first()
            .map(|c| c.r#type.to_string())
            .unwrap_or_default();

        let fee = tx.info
            .as_ref()
            .map(|i| i.fee)
            .unwrap_or(0);

        let block_number = tx.info
            .as_ref()
            .map(|i| i.block_number)
            .unwrap_or(0);

        let block_timestamp = tx.info
            .as_ref()
            .map(|i| i.block_time_stamp)
            .unwrap_or(0);

        let line = format!(
            r#"{{"tx_id":"{}","timestamp":{},"result":{},"contract_type":"{}","fee":{},"energy_used":{},"energy_penalty":{},"block_number":{},"block_timestamp":{}}}"#,
            hex::encode(&tx.txid),
            tx.timestamp,
            tx.result,
            contract_type,
            fee,
            tx.energy_used,
            tx.energy_penalty,
            block_number,
            block_timestamp,
        );

        lines.push(line);
    }

    Ok(Lines { lines })
}

//Full line for future use, 
/*
let line = format!(
    r#"{{"tx_id":"{}","ref_block_bytes":"{}","ref_block_hash":"{}","expiration":{},"timestamp":{},"energy_used":{},"energy_penalty":{},"result":{},"code":{},"message":"{}","contract_type":"{}","fee":{},"block_number":{},"block_timestamp":{}}}"#,
    hex::encode(&tx.txid),
    hex::encode(&tx.ref_block_bytes),
    hex::encode(&tx.ref_block_hash),
    tx.expiration,
    tx.timestamp,
    tx.energy_used,
    tx.energy_penalty,
    tx.result,
    tx.code,
    hex::encode(&tx.message),
    contract_type,
    fee,
    block_number,
    block_timestamp,
);*/