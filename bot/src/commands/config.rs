use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot::ConfigurationStore;

//use crate::bot::Configuration;

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected ConfigStore in TypeMap");
    let config = arc_config.lock().await.clone();


    // for account in config.accounts.into_iter() {
    //     let message = format!("{}, {}, {}", account.symbol, account.account, account.last_signature);
    //     msg.channel_id.say(&ctx.http, message).await?;
    // }


    Ok(())
}