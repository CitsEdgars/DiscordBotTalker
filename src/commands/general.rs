use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::fs;
use crate::config::FeatureKey;
use rand::SeedableRng;
use rand::rngs::StdRng;

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

// #[command]
// async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
//     use rand::Rng;

//     let args: Vec<&str> = msg.content.split_whitespace().collect();
//     if args.len() < 2 {
//         msg.reply(ctx, "Usage: !roll <max_number>").await?;
//         return Ok(());
//     }

//     let max: u32 = match args[1].parse() {
//         Ok(n) => n,
//         Err(_) => {
//             msg.reply(ctx, "Please provide a valid number.").await?;
//             return Ok(());
//         }
//     };

//     let mut rng = StdRng::from_entropy();
//     let roll1 = rng.gen_range(0..=max);
//     let roll2 = rng.gen_range(0..=max);
//     let roll3 = rng.gen_range(0..=max);

//     let result = format!("| {:01} | {:01} | {:01} |", roll1, roll2, roll3);

//     let win = roll1 == roll2 || roll1 == roll3 || roll2 == roll3;
//     let reply = if win {
//         format!("{}\n🎉 You've got GREAT TITS!", result)
//     } else {
//         format!("{}\nSucks to suck, eh? 😢", result)
//     };

//     msg.reply(ctx, reply).await?;

//     Ok(())
// }

#[command]
async fn slots(ctx: &Context, msg: &Message) -> CommandResult {
    use rand::seq::SliceRandom;

    let emoji_choices = ["🫃", "🫃🏻", "🫃🏼", "🫃🏽", "🫃🏾", "🫃🏿", "⭐"];
    let mut rng = StdRng::from_entropy();

    // Pick 3 random emojis
    let slots: Vec<&&str> = (0..3).map(|_| emoji_choices.choose(&mut rng).unwrap()).collect();

    let slot_display = format!("| {} | {} | {} |", slots[0], slots[1], slots[2]);

    // Check if any two match
    let win = slots[0] == slots[1] || slots[0] == slots[2] || slots[1] == slots[2];

    let result_msg = if win {
        format!("{}\n🎉 You've got GREAT TITS!", slot_display)
    } else {
        format!("{}\nSucks to suck, eh? 😢", slot_display)
    };

    msg.reply(ctx, result_msg).await?;

    Ok(())
}