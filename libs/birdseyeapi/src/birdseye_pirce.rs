use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BirdseyePrice {
    pub mint: String,
    pub value: f64,
    pub update_unix_time: i64,
    pub updateHumanTime: String,
    pub priceChange24h: f64,
}