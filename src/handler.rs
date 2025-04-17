use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, channel::ReactionType},
    prelude::*,
};

use crate::config::SharedConfig;

pub struct Handler {
    pub config: SharedConfig,
}

impl Handler {
    pub fn new(config: SharedConfig) -> Self {
        Handler { config }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let cfg = self.config.lock().await;

        let lowered = msg.content.to_lowercase();
        if cfg.zdr && ["hello", "zdr", "priv"].contains(&lowered.as_str()) {
            let _ = msg.reply(&ctx, "Zdarova Tovarisch! Clava Urainy!").await;
        }

        if cfg.preg {
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
}
