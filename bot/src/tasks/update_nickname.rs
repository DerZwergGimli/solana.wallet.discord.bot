use std::sync::Arc;
use birdseyeapi::birdseyeapi::fetch_multi_price;
use log::{error, info};
use serenity::model::prelude::*;
use serenity::prelude::*;
use solana_wallet::wallet::*;
use crate::bot::ConfigurationStore;

pub async fn update_nickname(ctx: Arc<Context>, _guilds: Vec<GuildId>) {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected WalletStore in TypeMap");
    let config = arc_config.lock().await.clone();

    let mut wallet: Wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet, config.check_unknown_token_accounts);
    wallet.load_config();

    let mut mint_list = vec![];
    wallet.wallet_tokens.clone().into_iter().for_each(|t| mint_list.push(t.mint));
    let token_prices = fetch_multi_price(mint_list, config.birdseye_token).await;

    let mut balance_value = 0.0;
    wallet.wallet_tokens.clone().into_iter().for_each(|token|
        balance_value += ((token.amount as f64) * 10f64.powf(-(token.decimals as f64))) * token_prices.clone().into_iter().find(|price| price.mint == token.mint).unwrap().value
    );

    let name_text: String = format!("💰 ~{:.2} 💰 ", balance_value);
    for _guild in _guilds.iter() {
        match _guild.edit_nickname(&ctx.http, Some(name_text.as_str())).await {
            Ok(_) => { info!("Changed Bot nickname!") }
            Err(_) => { error!("Unable to change bot nickname!") }
        };
    }
    println!("--- >update_nickname ---");
    ctx.set_activity(Activity::playing(config.domain.clone())).await;
}
