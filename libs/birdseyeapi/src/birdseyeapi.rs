use log::info;
use serde_json::Value;

use crate::birdseye_pirce::BirdseyePrice;

pub async fn fetch_multi_price(mints: Vec<String>) -> Vec<BirdseyePrice> {
    let mut url_to_fetch = "https://public-api.birdeye.so/public/multi_price?list_address=".to_string();
    mints.clone().into_iter().for_each(|mint| url_to_fetch += &(mint + "%2C"));

    let body =
        reqwest::get(&url_to_fetch[0..url_to_fetch.len() - 3])
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    let json: Value = serde_json::from_str(body.as_str()).unwrap();
    let mut mapped_pirces: Vec<BirdseyePrice> = vec![];
    mints.into_iter().for_each(|m| {
        if json["data"][m.clone()] != "null" {
            mapped_pirces.push({
                BirdseyePrice {
                    mint: m.clone(),
                    value: json["data"][m.clone()]["value"].as_f64().unwrap_or_default(),
                    update_unix_time: json["data"][m.clone()]["updateUnixTime"].as_i64().unwrap_or_default(),
                    update_human_time: json["data"][m.clone()]["updateHumanTime"].to_string(),
                    price_change24h: json["data"][m]["priceChange24h"].as_f64().unwrap_or_default(),
                }
            })
        }
    });
    info!("Birdseye prices fetched: {:?}", mapped_pirces);
    mapped_pirces
}