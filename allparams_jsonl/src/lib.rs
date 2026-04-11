#[allow(unused)]
mod pb;

use crate::pb::sf::substreams::sink::files::v1::Lines;
use crate::pb::sf::substreams::tron::v1::Transactions;


fn extract_owner_address(parameter: &Option<prost_types::Any>) -> String {
    parameter.as_ref()
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

        let contract_address = tx.info
            .as_ref()
            .map(|i| hex::encode(&i.contract_address))
            .unwrap_or_default();

        let signature_count = tx.signature.len() as i64;

        let energy_usage_total = tx.info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.energy_usage_total)
            .unwrap_or(0);

        let from_address = tx.contracts
            .first()
            .map(|c| extract_owner_address(&c.parameter))
            .unwrap_or_default();

        let energy_fee = tx.info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.energy_fee)
            .unwrap_or(0);
        
        let net_fee = tx.info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.net_fee)
            .unwrap_or(0);
        
        let net_usage = tx.info
            .as_ref()
            .and_then(|i| i.receipt.as_ref())
            .map(|r| r.net_usage)
            .unwrap_or(0);

            

            let line = format!(
                //r#"{{"tx_id":"{}","contract_type":"{}","fee":{},"energy_usage_total":{},"block_number":{},"block_timestamp":{},"contract_address":"{}","signature_count":{}}}"#,
                r#"{{"tx_id":"{}","contract_type":"{}","fee":{},"energy_used":{},"energy_usage_total":{},"energy_fee":{},"net_fee":{},"net_usage":{},"block_number":{},"block_timestamp":{},"contract_address":"{}","signature_count":{},"from":"{}"}}"#,
                hex::encode(&tx.txid),
                contract_type,
                fee,
                tx.energy_used,
                energy_usage_total,
                energy_fee,
                net_fee,
                net_usage,
                block_number,
                block_timestamp,
                contract_address,
                signature_count,
                from_address
            );

        lines.push(line);
    }

    Ok(Lines { lines })
}

//Full line for future use,
/*
let line = format!(
    r#"{{"tx_id":"{}","ref_block_bytes":"{}","ref_block_hash":"{}","expiration":{},"timestamp":{},"energy_used":{},"energy_penalty":{},"result":{},"code":{},"message":"{}","contract_type":"{}","fee":{},"block_number":{},"block_timestamp":{},"contract_address":"{}","method_id":"{}"}}"#,
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
    contract_address,
    method_id,
);*/