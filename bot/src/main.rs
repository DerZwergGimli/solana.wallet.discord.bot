use dotenv::dotenv;
use configuration::helper;
use solana_wallet::wallet::*;
use crate::bot::init_bot;

mod bot;
mod commands;

#[tokio::main]
async fn main() {
    println!("--- Booted ---");
    dotenv().ok();
    env_logger::init();


    println!("--- Load config! ---");
    let config = helper::read_config("config.json".to_string());

    println!("--- Load/Build TokenList! ---");
    let mut wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet, config.check_unknown_token_accounts);
    wallet.load_config();
    if wallet.wallet_tokens.is_empty() {
        // Update
        wallet.update_accounts().await;
        wallet.update_accounts_balances().await;
        wallet.update_token_names().await;
        wallet.save_config();
    }


    println!("--- Init BOT! ---");
    init_bot(config.clone()).await;
    println!("--- EXIT ---");
}

