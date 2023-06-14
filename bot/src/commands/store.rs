use std::env;
use std::fmt::format;

use prettytable::{row, Table};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;

//use crate::bot::{Configuration, WalletStore};
//use crate::config::config::get_config;

#[command]
async fn store(ctx: &Context, msg: &Message) -> CommandResult {
    // let config = get_config(env::var("CONFIG_PATH").expect("Please set env: CONFIG_PATH"));
    // let message = format!("Store-info: \n\
    // cursor: {:}\n\
    // block: {:}", config.substream_config.cursor_id, config.substream_config.cursor_block);
    //
    //
    // msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}