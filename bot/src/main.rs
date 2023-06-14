use dotenv::dotenv;

use configuration::helper;

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

