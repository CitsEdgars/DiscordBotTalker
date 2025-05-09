use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

use super::super::LocalBank;

#[command]
async fn wallet(ctx: &Context, msg: &Message) -> CommandResult {
    // !wallet
    let user_id = msg.author.id.0.to_string();
    let local_bank = LocalBank::load_bank();
    local_bank.get_balance(&user_id);

    msg.channel_id.say(&ctx.http, format!("Your account balance: {}", local_bank.get_balance(&user_id))).await?;
    Ok(())
}

#[command]
async fn gain(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // !gain {amount}
    let amount = args.single::<String>().ok().unwrap();
    let user_id = msg.author.id.0.to_string();
    let mut local_bank = LocalBank::load_bank();
    let amount_gained = amount.parse::<u32>().unwrap();

    local_bank.add_tokens(&user_id, amount_gained);
    local_bank.save_balance();

    msg.channel_id.say(&ctx.http, format!("Money (${}) has been added to your balance. Your account balance: {}", amount_gained, local_bank.get_balance(&user_id))).await?;
    Ok(())
}

#[command]
async fn transfer(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // !transfer {amount} {source_player} {target_player}
    let amount = args.single::<String>().ok().unwrap();
    let src_player = args.single::<String>().ok().unwrap();
    let trgt_player: String = args.single::<String>().ok().unwrap();
    let mut local_bank = LocalBank::load_bank();

    let amount = amount.parse::<u32>().unwrap();
    let src_id = extract_id_from_mention(src_player);
    let trgt_id = extract_id_from_mention(trgt_player);

    if src_id != msg.author.id.0.to_string() {
        msg.channel_id.say(&ctx.http, format!("Trying to be cheeky and tranfer other funds? Funny dude. Get lost.")).await?;
        return Ok(())
    }

    if local_bank.get_balance(&src_id) <= amount {
        msg.channel_id.say(&ctx.http, format!("You are too poor to transfer ${} to anyone. Check your balance.", amount)).await?;
        return Ok(())
    }
    
    local_bank.remove_tokens(&src_id, amount);
    local_bank.add_tokens(&trgt_id, amount);
    local_bank.save_balance();
    msg.channel_id.say(&ctx.http, format!("Money (${}) has been transfered.", amount)).await?;
    Ok(())
}

fn extract_id_from_mention(player_mention: String) -> String{
    player_mention.trim_start_matches("<@").trim_end_matches('>').to_string()
}