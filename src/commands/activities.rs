use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    model::channel::AttachmentType,
    prelude::*,
};
use tokio::time::{sleep, Duration};

use crate::commands::helpers::helper_slots::calculate_winnings;
use crate::commands::helpers::helper_slots::generate_grid;

use reqwest::Client;
use serde::Serialize;

use super::super::LocalBank;

#[derive(Serialize)]
struct GifRequest {
    image_urls: Vec<String>,
    // delay: Option<u32>,
}

pub async fn send_gif_request(
    image_urls: Vec<String>,
) -> Result<bytes::Bytes, reqwest::Error> {
    let payload = GifRequest { image_urls };

    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:3000/generate-gif")
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    let bytes = response.bytes().await?;
    Ok(bytes)
}
#[command]
pub async fn slots(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("Player {} spins", msg.author.id.0);
    let bet_input = args.single::<String>().ok();       // bet
    let line_input = args.single::<String>().ok();       // lines
    let max_bet: u32 = 50000;
    let min_bet: u32 = 10;
    let columns = 5;
    let rows = 3;
    let grid = generate_grid(rows, columns);

    let (lines, total_bet): (u32, u32) = match (bet_input, line_input) {
        (Some(b), Some(l)) => (
            parse_in_range(&b, min_bet, 1, max_bet),  // lines: default 10, min 1, max 20
            parse_in_range(&l, 10, 1, 10),  // bet: default 1, min 1, max 100
        ),
        (Some(b), None) => (
            10,
            parse_in_range(&b, min_bet, 1, max_bet),  // lines: default 10, min 1, max 20
        ),
        _ => (10, 10),
    };

    let user_id = msg.author.id.0.to_string();
    let mut local_bank = LocalBank::load_bank();
    
    if total_bet > local_bank.get_balance(&user_id) {
        msg.channel_id.say(&ctx.http, format!("Not enough funds -> Go do something else!")).await?;
        return Ok(())
    }
    local_bank.remove_tokens(&user_id, total_bet as u32);


    let mut roll_outputs: Vec<String> = vec![];
    for col_idx in 0..columns {
        let col_str: String = (0..rows)
            .map(|row_idx| grid[row_idx][col_idx].to_string())
            .collect::<Vec<_>>()
            .join("");
        roll_outputs.push(format!("{}{}", col_idx, col_str));
    }
    
    match send_gif_request(roll_outputs.clone()).await {
        Ok(gif_bytes) => {
            let attachment = AttachmentType::Bytes {
                data: std::borrow::Cow::Owned(gif_bytes.to_vec()),
                filename: "generated.gif".into(),
            };

            if let Err(e) = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("🎰 Spinning…")
                     .add_file(attachment)
                })
                .await
            {
                eprintln!("Failed to send GIF to Discord: {}", e);
                return Ok(());
            }

            // 2) Wait for the GIF to “play”
            sleep(Duration::from_secs(7)).await;

            //bug on paper - > method accepts bet per line, but due to division it can be wrong
            let winnings = calculate_winnings(&grid, lines, total_bet/lines);
            let response = if winnings > 0 {
                local_bank.add_tokens(&user_id, winnings as u32);
                format!(
                    "You bet {} on {} lines and won {}!", total_bet, lines, winnings
                )
            } else {
                format!(
                    "You bet {} on {} lines and won nothing this time. Better luck next spin!",
                    total_bet,
                    lines
                )
            };

            local_bank.save_balance();

            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, response)
                .await
            {
                eprintln!("Failed to send winnings message: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error fetching GIF: {:#?}", e);
            let _ = msg
                .channel_id
                .say(&ctx.http, "Nebūs, lol.")
                .await;
        }
    }

    Ok(())
}

fn parse_in_range(s: &str, default: u32, min: u32, max: u32) -> u32 {
    println!("{}-{}-{}-{}", s, default, min, max);
    let n = s.trim().parse::<u32>()
        .ok()
        .map(|n| n.clamp(min, max))
        .unwrap_or(default);
    println!("{}", n);
    n
}