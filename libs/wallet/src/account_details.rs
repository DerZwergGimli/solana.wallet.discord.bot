use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountDetails {
    pub account: String,
    pub mint: String,
    pub amount: f64,

}
