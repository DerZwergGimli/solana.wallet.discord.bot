use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime};
use chrono::offset::Utc;
use log::{error, info, warn};
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::utils::Color;
use wallet::wallet::Wallet;

use birdseyeapi::birdseyeapi::fetch_multi_price;
use configuration::configuration::Configuration;
use tx_scanner::tx_scanner::TxScanner;

use crate::commands::address::*;
use crate::commands::config::*;
use crate::commands::help::*;
use crate::commands::ping::*;
use crate::commands::store::*;
use crate::commands::wallet::*;

//
// pub struct WalletStore;
//
// impl TypeMapKey for WalletStore {
//     type Value = Arc<Mutex<Wallet>>;
// }

pub struct ConfigurationStore;

impl TypeMapKey for ConfigurationStore {
    type Value = Arc<Mutex<Configuration>>;
}


pub struct WalletStore;

impl TypeMapKey for WalletStore {
    type Value = Arc<Mutex<Wallet>>;
}

pub struct ScannerStore;

impl TypeMapKey for ScannerStore {
    type Value = Arc<Mutex<TxScanner>>;
}


pub struct Handler {
    pub(crate) is_loop_running: AtomicBool,
}

#[group]
#[commands(ping, help, wallet, config, store, address)]
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




            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// async fn update_wallet(ctx: Arc<Context>) {
//     let data_read = ctx.data.read().await;
//     let wallet = data_read.get::<WalletStore>().unwrap();
//     wallet.lock().await.fetch_solana_balance();
//     wallet.lock().await.fetch_token_accounts_balances();
//     wallet.lock().await.fetch_token_account_prices().await;
//     wallet.lock().await.fetch_transactions();
//     println!("wallet-updated!");
// }

async fn check_tx_queue(ctx: Arc<Context>) {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected WalletStore in TypeMap");
    let config = arc_config.lock().await.clone();



}


async fn update_nickname(ctx: Arc<Context>, _guilds: Vec<GuildId>) {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigurationStore>().expect("Expected WalletStore in TypeMap");
    let config = arc_config.lock().await.clone();


    let wallet = Wallet::new(config);
    let tokens_wallet = wallet.get_token_amounts().await;
    let tokens_prices = fetch_multi_price(tokens_wallet.clone().into_iter().map(|token| token.mint).collect()).await;

    let mut balance_value = 0.0;
    tokens_wallet.into_iter().for_each(|token|
        balance_value += (token.amount * tokens_prices.clone().into_iter().find(|price| price.mint == token.mint).unwrap().value)
    );

    let name_text: String = format!("💰 {:.2} 💰 ", f64::trunc(balance_value * 100.0) / 100.0);
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
