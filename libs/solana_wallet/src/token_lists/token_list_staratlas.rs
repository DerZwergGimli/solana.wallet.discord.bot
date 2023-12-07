use serde::{Serialize, Deserialize};

pub type TokenListStarAtlas = Vec<TokenListStarAtlasElement>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListStarAtlasElement {
    #[serde(rename = "_id")]
    pub id: String,
    pub deactivated: bool,
    pub name: String,
    pub description: String,
    pub image: String,
    pub media: Media,
    pub attributes: Attributes,
    pub symbol: String,
    pub markets: Vec<Market>,
    pub total_supply: Option<i64>,
    pub mint: String,
    pub network: Option<Network>,
    pub trade_settings: TradeSettings,
    pub airdrops: Vec<Airdrop>,
    pub primary_sales: Vec<PrimarySale>,
    pub updated_at: Option<String>,
    pub collection: Option<Collection>,
    pub slots: Option<Slots>,
    #[serde(rename = "id")]
    pub token_list_star_atlas_id: String,
    pub created_at: Option<String>,
    #[serde(rename = "__v")]
    pub v: Option<i64>,
    pub trade_blocked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Airdrop {
    #[serde(rename = "_id")]
    pub id: String,
    pub supply: i64,
    #[serde(rename = "id")]
    pub airdrop_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub item_type: ItemType,
    pub tier: Option<i64>,
    pub class: String,
    pub category: Option<String>,
    pub score: Option<i64>,
    pub rarity: Rarity,
    pub musician: Option<String>,
    pub spec: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub unit_length: Option<f64>,
    pub unit_width: Option<f64>,
    pub unit_height: Option<f64>,
    pub series_name: Option<SeriesName>,
    pub episode: Option<i64>,
    pub edition: Option<Edition>,
    pub asset_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Edition {
    #[serde(rename = "alternate-cover")]
    AlternateCover,
    #[serde(rename = "magic-eden")]
    MagicEden,
    #[serde(rename = "star-atlas")]
    StarAtlas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Access,
    Collectible,
    Currency,
    Memories,
    Resource,
    Ship,
    Story,
    Structure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    Anomaly,
    Common,
    Epic,
    Legendary,
    Rare,
    Uncommon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeriesName {
    Core,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub family: Family,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Family {
    #[serde(rename = "Star Atlas")]
    StarAtlas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "id")]
    pub market_id: String,
    pub quote_pair: QuotePair,
    pub serum_program_id: Option<SerumProgramId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuotePair {
    #[serde(rename = "ATLAS")]
    Atlas,
    #[serde(rename = "SOL")]
    Sol,
    #[serde(rename = "USDC")]
    Usdc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SerumProgramId {
    #[serde(rename = "srmv4uTCPF81hWDaPyEN2mLZ8XbvzuEM6LsAxR8NpjU")]
    Srmv4UTcpf81HWDaPyEn2MLz8XbvzuEm6LsAxR8NpjU,
    #[serde(rename = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")]
    The9XQeWvG816BUx9EPjHmaT23YvVm2ZWbrrpZb9PusVFin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub qr_instagram: Option<String>,
    pub qr_facebook: Option<String>,
    pub sketchfab: Option<String>,
    pub audio: Option<String>,
    pub thumbnail_url: Option<String>,
    pub gallery: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Network {
    Devnet,
    #[serde(rename = "mainnet-beta")]
    MainnetBeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimarySale {
    pub list_timestamp: i64,
    #[serde(rename = "id")]
    pub primary_sale_id: Option<String>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub supply: Option<i64>,
    pub price: Option<f64>,
    pub is_minted: Option<bool>,
    pub is_listed: Option<bool>,
    pub mint_timestamp: Option<i64>,
    pub order_id: Option<serde_json::Value>,
    pub expire_timestamp: Option<i64>,
    pub target_pair: Option<QuotePair>,
    pub quote_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slots {
    pub crew_slots: Option<Vec<ComponentSlotElement>>,
    pub component_slots: Option<Vec<ComponentSlotElement>>,
    pub module_slots: Option<Vec<ComponentSlotElement>>,
    pub interior_slots: Option<Vec<InteriorSlotElement>>,
    pub station_slots: Option<Vec<InteriorSlotElement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSlotElement {
    #[serde(rename = "type")]
    pub slot_type: String,
    pub size: Option<Crew>,
    pub quantity: i64,
    pub crew: Option<Crew>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Crew {
    Capital,
    #[serde(rename = "class 8")]
    Class8,
    Commander,
    Crew,
    #[serde(rename = "Class 8")]
    CrewClass8,
    #[serde(rename = "XX-Small")]
    CrewXxSmall,
    Large,
    Medium,
    Small,
    Titan,
    #[serde(rename = "x-small")]
    XSmall,
    #[serde(rename = "xx-small")]
    XxSmall,
    #[serde(rename = "xxx-small")]
    XxxSmall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteriorSlotElement {
    #[serde(rename = "type")]
    pub slot_type: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Coming Soon")]
    ComingSoon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeSettings {
    pub expire_time: Option<ETime>,
    pub sale_time: Option<ETime>,
    pub vwap: Option<f64>,
    pub msrp: Option<Msrp>,
    pub sale_type: Option<String>,
    pub limited: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ETime {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Msrp {
    pub value: f64,
    pub currency_symbol: QuotePair,
}
