use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MappedTX {
    pub signature: String,
    pub block: u64,
    pub timestamp: i64,
    pub signer: String,
    pub source_account: String,
    pub destination_account: String,
    pub mint_send: String,
    pub amount_send_parsed: f64,
    pub message: String,
}