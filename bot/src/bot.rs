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
use crate::commands::accounts::*;
use crate::commands::config::*;
use crate::commands::help::*;
use crate::commands::ping::*;
use crate::tasks::update_nickname::update_nickname;
use crate::tasks::update_wallet::update_wallet;

pub struct ConfigurationStore;

impl TypeMapKey for ConfigurationStore {
    type Value = Arc<Mutex<Configuration>>;
}


pub struct Handler {
    pub(crate) is_loop_running: AtomicBool,
}

#[group]
#[commands(ping, help, accounts, config, address)]
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


                    update_wallet(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_millis(config.update_tx_sleep)).await;
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
