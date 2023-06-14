use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub wallet: String,
    pub rpc_url: String,
    pub update_config: bool,
    pub discord_token: String,
    pub discord_prefix: String,
    pub update_name_sleep: u64,
    pub update_tx_sleep: u64,
    pub accounts: Vec<ConfigAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigAccount {
    pub mint: String,
    pub account: String,
    pub last_signature: String,
    pub discord_channel_id: u64,
    pub symbol: String,
}
