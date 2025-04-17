mod config;
mod handler;
mod commands;

use config::FeatureConfig;
use handler::Handler;
use commands::GENERAL_GROUP;

use serenity::{
    prelude::*,
    framework::standard::StandardFramework,
    Client
};
use dotenvy::dotenv;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

use config::FeatureKey;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");

    let config = FeatureConfig::load("features.json").unwrap_or_default();
    let shared_config = Arc::new(Mutex::new(config));

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let handler = Handler::new(Arc::clone(&shared_config));

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<FeatureKey>(shared_config);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}