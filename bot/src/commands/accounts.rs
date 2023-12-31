use serenity::framework::standard::{CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use configuration::helper;
use serenity::utils::Color;
use solana_wallet::wallet::Wallet;


#[command]
async fn accounts(ctx: &Context, msg: &Message) -> CommandResult {
    let config = helper::read_config("config.json".to_string());

    let mut wallet: Wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet, config.check_unknown_token_accounts);
    wallet.load_config();


    let mut page = 0;
    let mut table = vec![];

    let mut got_triggered = false;


    for (wallet_token_index, wallet_token) in wallet.wallet_tokens.clone().into_iter().filter(|token| token.amount != 0).enumerate() {
        let name = match wallet_token.info.name.len() {
            0 => { "???".to_string() }
            _ => { wallet_token.info.name }
        };
        table.push((name, (format!("{:.2}", (wallet_token.amount as f32) * 10f32.powf(-(wallet_token.decimals as f32))).to_string()), true));


        if ((wallet_token_index % 21 == 0) || (wallet_token_index == wallet.wallet_tokens.len())) && wallet_token_index != 0
        {
            send_embed_paged(&ctx, config.domain.clone(), msg, page, &mut table).await;
            table = vec![];
            page += 1;
            got_triggered = true
        }
    }
    if !got_triggered {
        send_embed_paged(&ctx, config.domain.clone(), msg, page, &mut table).await;
    }
    Ok(())
}

async fn send_embed_paged(ctx: &&Context,name: String, msg: &Message, mut page: i32, mut table: &mut Vec<(String, String, bool)>)  {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("{} Accounts [{}]",name, page))
                .color(Color::ORANGE)
                .fields(table.clone())
        })
    }).await.unwrap();
}