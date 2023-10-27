use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use crate::bot::ConfigurationStore;

//use crate::bot::Configuration;

#[command]
async fn address(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected ConfigStore in TypeMap");
    let config = arc_config.lock().await.clone();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Wallet-Address")
                .color(Color::ORANGE)
                .field("Domain", config.domain.clone(), false)
                .field("Address", config.wallet.clone(), false)
                .field("Solscan", format!("[link](https://solscan.io/account/{:})", config.wallet.clone()), true)
                .field("SolanaFM", format!("[link](https://solana.fm/address/{:})", config.wallet.clone()), true)
                .field("STEP.Finance", format!("[link](https://app.step.finance/en/dashboard?watching={:})", config.wallet.clone()), true)
                // .field("SolanaBeach", format!("https://solanabeach.io/address/{:}", config.wallet.clone()), false)
                // .field("SolanaFM", format!("https://solana.fm/address/{:}", config.wallet.clone()), false)
                // .field("STEP.Finance", format!("https://app.step.finance/en/dashboard?watching={:}", config.wallet), false)
        })
    }).await;

    Ok(())
}