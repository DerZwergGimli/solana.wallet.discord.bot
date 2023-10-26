use dotenv::dotenv;

use configuration::helper;
use solana_wallet::Wallet;

use crate::bot::init_bot;

mod bot;
mod commands;

#[tokio::main]
async fn main() {
    println!("--- Starting ---");
    dotenv().ok();
    env_logger::init();


    println!("--- Load config! ---");
    let config = helper::read_config("config.json".to_string());

    // println!("--- Build TokenList! ---");
    // let mut wallet = Wallet::new(
    //     "https://solana-mainnet.g.alchemy.com/v2/AaKsvOkJp4LwaW08RHWRZo43ZWtYPiOD".to_string(),
    //     "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string());
    //


    init_bot(config.clone()).await;
    println!("--- EXIT ---");
}

