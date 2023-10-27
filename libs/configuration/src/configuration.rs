use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub name: String,
    pub wallet: String,
    pub rpc_url: String,
    pub check_unknown_token_accounts: bool,
    pub discord_token: String,
    pub birdseye_token: String,
    pub discord_prefix: String,
    pub update_name_sleep: u64,
    pub update_tx_sleep: u64,
    pub accounts: Vec<ConfigAccount>,
    pub discord_channel_id_default: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigAccount {
    pub mint: String,
    pub discord_channel_id: u64,
}
