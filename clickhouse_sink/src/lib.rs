#[allow(unused)]
mod pb;

use crate::pb::protocol::AccountPermissionUpdateContract;
use crate::pb::sf::substreams::tron::v1::Transactions;
use prost::Message;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

fn decode_permission_threshold(parameter: &Option<prost_types::Any>) -> String {
    parameter
        .as_ref()
        .and_then(|p| AccountPermissionUpdateContract::decode(p.value.as_slice()).ok())
        .and_then(|c| c.owner)
        .map(|p| p.threshold.to_string())
        .unwrap_or_default()
}

fn extract_owner_address(parameter: &Option<prost_types::Any>) -> String {
    parameter
        .as_ref()
        .and_then(|p| {
            let value = &p.value;
            if value.len() > 2 && value[0] == 0x0a {
                let len = value[1] as usize;
                if value.len() >= 2 + len {
                    Some(hex::encode(&value[2..2 + len]))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_default()
}

#[substreams::handlers::map]
fn db_out(transactions: Transactions) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for tx in transactions.transactions {
        let contract_type = tx
            .contracts
            .first()
            .map(|c| c.r#type.to_string())
            .unwrap_or_default();

        let total_fee_burn = tx
            .info
            .as_ref()
            .map(|i| i.fee)
            .unwrap_or(0);

        let block_number = tx
            .info
            .as_ref()
            .map(|i| i.block_number)
            .unwrap_or(0);

        let block_timestamp = tx
            .info
            .as_ref()
            .map(|i| i.block_time_stamp)
            .unwrap_or(0);

        let contract_address = tx
            .info
            .as_ref()
            .map(|i| hex::encode(&i.contract_address))
            .unwrap_or_default();

        let signature_count = tx.signature.len() as u8;

        let energy_usage_total = tx
            .info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.energy_usage_total)
            .unwrap_or(0);

        let from_address = tx
            .contracts
            .first()
            .map(|c| extract_owner_address(&c.parameter))
            .unwrap_or_default();

        let energy_from_stake = tx
            .info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.energy_fee)
            .unwrap_or(0);

        let net_from_burn = tx
            .info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.net_fee)
            .unwrap_or(0);

        let net_from_stake = tx
            .info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.net_usage)
            .unwrap_or(0);

        let permission_threshold = if contract_type == "46" {
            tx.contracts
                .first()
                .map(|c| decode_permission_threshold(&c.parameter))
                .unwrap_or_default()
        } else {
            String::new()
        };

        let tx_id = hex::encode(&tx.txid);

        tables
            .create_row("transactions", &tx_id)
            .set("tx_id", tx_id.clone())
            .set("contract_type", contract_type)
            .set("total_fee_burn", total_fee_burn)
            .set("energy_usage_total", energy_usage_total)
            .set("energy_from_stake", energy_from_stake)
            .set("net_from_burn", net_from_burn)
            .set("net_from_stake", net_from_stake)
            .set("block_number", block_number)
            .set("block_timestamp", block_timestamp)
            .set("contract_address", contract_address)
            .set("signature_count", signature_count)
            .set("perm_threshold", permission_threshold)
            .set("from", from_address);
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