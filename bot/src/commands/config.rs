use configuration::helper;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use solana_wallet::wallet::Wallet;

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    let config = helper::read_config("config.json".to_string());
    let mut wallet: Wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet);
    wallet.load_config();


    for token in wallet.wallet_tokens {
        let message = format!("{} - {}", token.account, token.last_signature);
        msg.channel_id.say(&ctx.http, message).await?;
    }

    Ok(())
}