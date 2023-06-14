use std::fmt::format;
use std::str;

use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use wallet::wallet::Wallet;

use birdseyeapi::birdseyeapi::fetch_multi_price;
use tx_scanner::tx_scanner::TxScanner;

use crate::bot::ConfigurationStore;

//use crate::bot::{WalletStore};

#[command]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected ConfigStore in TypeMap");
    let config = arc_config.lock().await.clone();

    let wallet = Wallet::new(config.clone());
    let tokens_wallet = wallet.get_token_amounts().await;
    let tokens_prices = fetch_multi_price(tokens_wallet.clone().into_iter().map(|token| token.mint).collect()).await;

    let mut table_string = "".to_string();
    let mut table_row;
    for token in tokens_wallet.into_iter() {
        table_row = format!("{} \t {:.2} \t {:.2}",
                            config.clone().accounts.into_iter().find(|acc| acc.mint == token.mint).unwrap().symbol,
                            token.amount.to_string(),
                            (token.amount * tokens_prices.clone().into_iter().find(|price| price.mint == token.mint).unwrap().value).to_string()
        );
        table_string = format!("{}{}\n", table_string, table_row);
    }

    msg.channel_id.say(&ctx.http, table_string).await?;

    Ok(())
}