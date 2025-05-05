use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use tokio::time::{sleep, Duration};

use std::path::Path;
use crate::commands::helpers::helper_slots::parse_grid_from_filename;
use crate::commands::helpers::helper_slots::calculate_winnings;

#[command]
pub async fn slots(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<String>().ok();
    let second = args.single::<String>().ok();

    let (lines, bet_per_line): (usize, usize) = match (first, second) {
        (Some(a), Some(b)) => (a.parse().unwrap_or(10), b.parse().unwrap_or(1)),
        (Some(a), None) => (10, a.parse().unwrap_or(1)),
        _ => (10, 1),
    };

    let filename = "slot_20250505T192405_{7,9,9}_{5,9,8}_{9,7,8}_{7,8,9}_{7,6,9}.gif";
    //  Otherwise fetch a random file from the generated gifs folder - for now hardcoded

    if let Some(grid) = parse_grid_from_filename(filename) {
        msg.channel_id.send_files(&ctx.http, vec![Path::new(&filename)], |m| m).await?;

        sleep(Duration::from_secs(7)).await;

        let winnings = calculate_winnings(&grid, lines, bet_per_line);
        let response = if winnings > 0 {
            format!(
                "You bet {} on each of {} lines and won {}!",
                bet_per_line,
                lines,
                winnings
            )
        } else {
            format!(
                "😢 You bet {} on {} lines and won nothing this time. Better luck next spin!",
                bet_per_line * lines, lines
            )
        };
        
        msg.channel_id.say(&ctx.http, response).await?;
    } else {
        // Switch off slots if something goes wrong?
        msg.reply(ctx, "Failed to parse slot grid from image filename.").await?;
    }

    Ok(())
}