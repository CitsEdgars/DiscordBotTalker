use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::fs;

use crate::config::FeatureKey;

#[command]
async fn features(ctx: &Context, msg: &Message) -> CommandResult {
    let config_text = fs::read_to_string("features.json").unwrap_or_else(|_| "{}".to_string());
    let features: serde_json::Value = serde_json::from_str(&config_text)?;

    let mut response = String::from("🔧 **Feature Flags:**\n");
    for (key, val) in features.as_object().unwrap() {
        let status = if val.as_bool() == Some(true) { "✅ ON" } else { "❌ OFF" };
        response.push_str(&format!("• **{}**: {}\n", key, status));
    }

    msg.reply(ctx, response).await?;
    Ok(())
}

#[command]
pub async fn toggle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let feature = args.single::<String>().unwrap_or_default();
    let data = ctx.data.read().await;
    let config_lock = data.get::<FeatureKey>().unwrap().clone();
    let mut config = config_lock.lock().await;

    let response: String = match feature.as_str() {
        "preg" => {
            config.preg = !config.preg;
            format!("preg_man is now {}", config.preg)
        }
        "zdr" => {
            config.zdr = !config.zdr;
            format!("zdr is now {}", config.zdr)
        }
        "who_asked" => {
            config.who_asked = !config.who_asked;
            format!("who_asked is now {}", config.who_asked)
        }
        _ => "Unknown feature.".to_string(),
    };

    config.save("features.json")?;

    msg.reply(ctx, response).await?;
    Ok(())
}

#[command]
async fn test(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Replace with your actual emoji name and ID
    // msg.channel_id.say(&ctx.http, format!("Test emoji: {}", custom_emoji)).await?;
    let irir = args.single::<String>().ok().unwrap();

    println!("{}", irir);

    Ok(())
}