use serenity::framework::standard::CommandResult;
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
    for (wallet_token_index, wallet_token) in wallet.wallet_tokens.clone().into_iter().enumerate() {
        let name = match wallet_token.info.name.len() {
            0 => { "unknown".to_string() }
            _ => { wallet_token.info.name }
        };
        table.push((name, (format!("{:.2}", (wallet_token.amount as f32) * 10f32.powf(-(wallet_token.decimals as f32))).to_string()), true));


        if ((wallet_token_index % 21 == 0) || (wallet_token_index == wallet.wallet_tokens.len())) && wallet_token_index != 0
        {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("Wallet-Balances [{}]", page))
                        .color(Color::ORANGE)
                        .fields(table.clone())
                })
            }).await?;
            table = vec![];
            page += 1;
        }
    }


    Ok(())
}