use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BirdseyePrice {
    pub mint: String,
    pub value: f64,
    pub update_unix_time: i64,
    pub update_human_time: String,
    pub price_change24h: f64,
}