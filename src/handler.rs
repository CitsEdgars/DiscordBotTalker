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

        let special_user_id = 181459240429420544;

        if cfg.who_asked && msg.author.id.0 == special_user_id {    
            // Respond with a GIF
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("")   //message potential
                 .embed(|e| {
                    e.image("https://media1.giphy.com/media/v1.Y2lkPTc5MGI3NjExcDFoYzEzeThzZWd4MG5iY293MnhkeXNwcng0ZXdzeGlsdDhyc3FmMSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/aT7Bec0LYoTowryv0B/giphy.gif")
                  })
            }).await {
                println!("Failed to send gif: {:?}", why);
            }
        }

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
