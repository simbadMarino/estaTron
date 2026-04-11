#[allow(unused)]
mod pb;

use crate::pb::sf::substreams::tron::v1::Transactions;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
fn map_my_data(transactions: Transactions) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

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

        let tx_id = hex::encode(&tx.txid);

        tables.create_row("transactions", &tx_id)
            .set("tx_id", tx_id.clone())
            .set("timestamp", tx.timestamp)
            .set("result", tx.result)
            .set("contract_type", contract_type)
            .set("fee", fee)
            .set("energy_used", tx.energy_used)
            .set("energy_penalty", tx.energy_penalty)
            .set("block_number", block_number)
            .set("block_timestamp", block_timestamp);
    }

    Ok(tables.to_database_changes())
}

//Full line for future use,
/*
tables.create_row("transactions", &tx_id)
    .set("tx_id", tx_id.clone())
    .set("ref_block_bytes", hex::encode(&tx.ref_block_bytes))
    .set("ref_block_hash", hex::encode(&tx.ref_block_hash))
    .set("expiration", tx.expiration)
    .set("timestamp", tx.timestamp)
    .set("energy_used", tx.energy_used)
    .set("energy_penalty", tx.energy_penalty)
    .set("result", tx.result)
    .set("code", tx.code)
    .set("message", hex::encode(&tx.message))
    .set("contract_type", contract_type)
    .set("fee", fee)
    .set("block_number", block_number)
    .set("block_timestamp", block_timestamp);
*/