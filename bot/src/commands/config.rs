use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

//use crate::bot::Configuration;

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    // let data_read = ctx.data.read().await;
    // let arc_config = data_read.get::<Configuration>().expect("Expected ConfigStore in TypeMap");
    // let config = arc_config.lock().await.clone();
    //
    // let _ = msg.channel_id.send_message(&ctx.http, |m| {
    //     m.embed(|e| {
    //         e.title("Configuration")
    //             .color(Color::ORANGE)
    //             .field("RPC-URL", config.rpc_url, false)
    //             .field("Wallet-Address", config.wallet_address, false)
    //             .field("Update-Timeout (ms)", config.update_timeout, false)
    //     })
    // }).await;

    Ok(())
}