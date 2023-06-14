use std::env;

use dotenv::dotenv;
use wallet::wallet::Wallet;

use birdseyeapi::birdseyeapi;
use configuration::helper;
use tx_scanner::tx_scanner::TxScanner;

use crate::bot::init_bot;

mod bot;
mod commands;

#[tokio::main]
async fn main() {
    println!("--- Starting ---");
    dotenv().ok();
    env_logger::init();

    let config = helper::read_config("config.json".to_string());


    init_bot(config.clone()).await;
    println!("--- EXIT ---");
}

