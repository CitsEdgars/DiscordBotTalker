use dotenvy::dotenv;
use std::env;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, channel::ReactionType},
    prelude::*,
};

// Define your commands
#[command]
async fn tava(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Mamma!").await?;
    Ok(())
}

// Group the commands (you can add more later)
#[group]
#[commands(tava)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        // Respond when someone says "hello"
        if msg.content.to_lowercase().contains("hello") {
            let _ = msg.reply(&ctx, "Hey there! 👋").await;
        }

        let emojis = [
            ReactionType::Unicode("🫃".to_string()),
            ReactionType::Unicode("🫃🏻".to_string()),
            ReactionType::Unicode("🫃🏼".to_string()),
            ReactionType::Unicode("🫃🏽".to_string()),
            ReactionType::Unicode("🫃🏾".to_string()),
            ReactionType::Unicode("🫃🏿".to_string()),
        ];
        
        for emoji in emojis {
            if let Err(why) = msg.react(&ctx, emoji).await {
                println!("Failed to react: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // This loads the .env file
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // Set command prefix to "!"
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
