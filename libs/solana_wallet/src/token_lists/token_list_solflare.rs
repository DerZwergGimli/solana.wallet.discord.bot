use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListSolflare {
    pub name: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub keywords: Vec<String>,
    pub tags: Tags,
    pub timestamp: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Tags {
    pub lp_token: LpToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LpToken {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub chain_id: i64,
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: i64,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub tags: Vec<String>,
    pub verified: bool,
    pub holders: Option<i64>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub coingecko_id: String,
}
