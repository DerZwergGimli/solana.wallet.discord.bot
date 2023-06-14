use std::env;

use dotenv::dotenv;

use birdseyeapi::birdseyeapi;
use configuration::helper;
use tx_parser::tx_scanner::TxScanner;
use wallet::solana::Wallet;

use crate::bot::init_bot;

mod bot;
mod commands;

#[tokio::main]
async fn main() {
    println!("--- Starting ---");
    dotenv().ok();
    env_logger::init();


    let config = helper::read_config("config.json".to_string());

    let wallet = Wallet::new(config.clone());
    let account_details = wallet.get_token_amounts().await;
    println!("{:?}", account_details);

    let mut scanner = TxScanner::new(config.clone());
    let txs = scanner.check().await.expect("TODO: panic message");
    scanner.update_config(txs);


    let prices = birdseyeapi::fetch_multi_price(config.clone().accounts.into_iter().map(|account| account.mint).collect()).await;
    println!("{:?}", prices);
    // let config = config::config::get_config(env::var("CONFIG_PATH").expect("Please set env: CONFIG_PATH"));
    // let wallet = solana::wallet::Wallet::new(config.clone());
    // let mut substream = substreams::substream_service::SubstreamService::new(config.clone());
    // // substream.run().await;
    //
    // init_bot(config.clone(), wallet, substream).await;

    init_bot(config.clone()).await;
    println!("--- EXIT ---");
}

