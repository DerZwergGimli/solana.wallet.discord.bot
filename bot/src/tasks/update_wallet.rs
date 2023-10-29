use std::sync::Arc;
use chrono::{DateTime, NaiveDateTime, Utc};
use configuration::helper;
use serenity::model::prelude::*;
use serenity::utils::*;
use serenity::prelude::*;
use solana_wallet::wallet::*;

pub async fn update_wallet(ctx: Arc<Context>) {
    let config = helper::read_config("config.json".to_string());
    let typing = ChannelId(config.discord_channel_id_default).start_typing(&ctx.http).unwrap();

    let mut wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet, config.check_unknown_token_accounts);
    wallet.load_config();


    let transactions: Vec<WalletTransaction> = wallet.get_and_update_signatures().await;

    for transaction in transactions.clone() {
        let direction_emote = if transaction.is_incoming { ":inbox_tray:" } else { ":outbox_tray:" };


        let info_message = format!("{:} {:.2} {:}",
                                   direction_emote,
                                   transaction.amount as f64 * 10.0f64.powf(-(transaction.decimals as f64)),
                                   transaction.info.name
        );
        let channel_id =
            match config.accounts.clone().into_iter().find(|account| {
                account.mint == transaction.mint
            }) {
                None => { config.discord_channel_id_default }
                Some(data) => { data.discord_channel_id }
            };

        let title_message = format!(":information_source: {:}", transaction.instruction);


        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(NaiveDateTime::from_timestamp_opt(transaction.timestamp, 1000000).unwrap(), Utc);

        let _ = ChannelId(channel_id).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(title_message)
                    .color(Color::ORANGE)
                    .field(info_message, "", false)
                    .field("Timestamp", format!("{}", timestamp), false)
                    .field("Signature", transaction.signature.clone(), false)
                    .field("Solscan", format!("[link](https://solscan.io/tx/{:})", transaction.signature.clone()), true)
                    .field("SolanaFM", format!("[link](https://solana.fm/tx/{:})", transaction.signature.clone()), true)
                    .field("Confirmation", "React with <:accepted:887091162018615317> to this message!", false)
                    .thumbnail(transaction.info.image_url.clone())
            })
        }).await;
    }

    if !transactions.is_empty() {
        wallet.update_accounts().await;
        wallet.update_accounts_balances().await;
        wallet.update_token_names().await;
        wallet.save_config();
    }
    println!("--- >update_wallet ---");
    typing.stop();
}
