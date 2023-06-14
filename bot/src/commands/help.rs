use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use crate::bot::ConfigurationStore;

//use crate::bot::Configuration;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let config = data_read.get::<ConfigurationStore>().expect("Expected ConfigStore in TypeMap");
    let prefix = config.lock().await.clone().discord_prefix;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Wallet Tracker Help")
                .description("This are all commands available for the wallet-tracker-bot.")
                .color(Color::ORANGE)
                .field(prefix.clone() + "help", "Shows this message", false)
                .field(prefix.clone() + "store", "Prints stored-last signatures", false)
                .field(prefix.clone() + "address", "Prints wallet-address helper", false)
                .field(prefix.clone() + "wallet", "Prints wallet info", false)
                .timestamp(Timestamp::now())
        })
    }).await?;

    Ok(())
}