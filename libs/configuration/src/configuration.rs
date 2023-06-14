use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub update_config: bool,
    pub discord_token: String,
    pub discord_prefix: String,
    pub accounts: Vec<ConfigAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigAccount {
    pub mint: String,
    pub account: String,
    pub last_signature: String,
    pub discord_channel_id: String,
}
