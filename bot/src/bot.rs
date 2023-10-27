use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use birdseyeapi::birdseyeapi::fetch_multi_price;
use chrono::{DateTime, NaiveDateTime, Utc};


use log::{error, info};
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::gateway::{Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::utils::Color;

use configuration::configuration::{Configuration};
use configuration::helper;
use serenity::model::prelude::Activity;

use solana_wallet::wallet::*;

use crate::commands::address::*;
use crate::commands::balance::*;
use crate::commands::config::*;
use crate::commands::help::*;
use crate::commands::ping::*;

pub struct ConfigurationStore;

impl TypeMapKey for ConfigurationStore {
    type Value = Arc<Mutex<Configuration>>;
}


pub struct Handler {
    pub(crate) is_loop_running: AtomicBool,
}

#[group]
#[commands(ping, help, balance, config, address)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache built successfully!");
        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {

            //Update-Wallet Balance (aka: bot-name)
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    let data_read = ctx1.data.read().await;
                    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected ConfigStore in TypeMap");
                    let config = arc_config.lock().await.clone();

                    update_nickname(Arc::clone(&ctx1), _guilds.clone()).await;
                    tokio::time::sleep(Duration::from_millis(config.update_tx_sleep)).await;
                }
            });

            //Check TX Queue Task
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    let data_read = ctx2.data.read().await;
                    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected ConfigStore in TypeMap");
                    let config = arc_config.lock().await.clone();


                    check_tx_queue(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_secs(config.update_tx_sleep)).await;
                }
            });

            let default_panic = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                default_panic(info);
                std::process::exit(1);
            }));

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


async fn check_tx_queue(ctx: Arc<Context>) {
    let config = helper::read_config("config.json".to_string());
    let typing = ChannelId(config.discord_channel_id_default).start_typing(&ctx.http).unwrap();

    let mut wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet, config.check_unknown_token_accounts);
    wallet.load_config();
    let transactions: Vec<WalletTransaction> = wallet.get_and_update_signatures().await;

    for transaction in transactions {
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

        let title_message = format!(":information_source: \t {:}", transaction.instruction);

        let _ = ChannelId(channel_id).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(title_message)
                    .color(Color::ORANGE)
                    .field(info_message, "", false)
                    .field("Timestamp", format!("{}", DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(transaction.timestamp, 1000000).unwrap(), Utc)), false)
                    .field("Signature", transaction.signature.clone(), false)
                    .field("Link", format!("https://solscan.io/tx/{}", transaction.signature.clone()), false)
                    .thumbnail(transaction.info.image_url.clone())
            })
        }).await;
    }

    wallet.save_config();
    typing.stop();
}


async fn update_nickname(ctx: Arc<Context>, _guilds: Vec<GuildId>) {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected WalletStore in TypeMap");
    let config = arc_config.lock().await.clone();

    let mut wallet: Wallet = Wallet::new(config.clone().rpc_url, config.clone().wallet, config.check_unknown_token_accounts);
    wallet.load_config();

    let mut mint_list = vec![];
    wallet.wallet_tokens.clone().into_iter().for_each(|t| mint_list.push(t.mint));
    let token_prices = fetch_multi_price(mint_list, config.birdseye_token).await;

    let mut balance_value = 0.0;
    wallet.wallet_tokens.clone().into_iter().for_each(|token|
        balance_value += ((token.amount as f64) * 10f64.powf(-(token.decimals as f64))) * token_prices.clone().into_iter().find(|price| price.mint == token.mint).unwrap().value
    );

    let name_text: String = format!("💰 ~{:.2} 💰 ", balance_value);
    for _guild in _guilds.iter() {
        match _guild.edit_nickname(&ctx.http, Some(name_text.as_str())).await {
            Ok(_) => { info!("Changed Bot nickname!") }
            Err(_) => { error!("Unable to change bot nickname!") }
        };
    }
    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc2822();

    ctx.set_activity(Activity::playing(&formatted_time)).await;
}


pub async fn init_bot(config: Configuration) {
    let http = Http::new(&config.clone().discord_token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix(config.clone().discord_prefix)).group(&GENERAL_GROUP);


    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(config.clone().discord_token, intents)
        .framework(framework)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ConfigurationStore>(Arc::new(Mutex::new(config)));
    }


    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
