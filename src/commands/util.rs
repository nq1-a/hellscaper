use std::fs::{
    read_to_string as read_fs,
};

use rand::Rng;

use poise::CreateReply;
use poise::serenity_prelude::{
    model::id::ChannelId,
};

use crate::{Context, Error};
use crate::commands::level::{
    add_points,
    dec_points,
    get_points,
    lvl_points
};
use crate::types::gif::Gif;

#[poise::command(
    slash_command,
    description_localized("en-US", "Returns the bot's GitHub page")
)]
pub async fn github(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(ctx.data().config.get("github").unwrap()).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Show the help page for a command (if one exists)")
)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let command_san: String = command
        .unwrap_or("help".to_string())
        .replace(&['/', '\\'], "");

    if let Ok(t) = read_fs(String::from("help/") + &command_san + &String::from(".md")) {
        ctx.say(t).await?;
    } else {
        ctx.say("NO HELP PAGE FOUND FOR THAT COMMAND").await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Returns how long it took for this command to be processed internally")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let millis: u128 = ctx.ping().await.as_millis();

    if millis == 0 {
       ctx.say("bot has not yet performed a full heartbeat --- please try again later").await?;
    } else {
        ctx.say(format!("**PONG!**\nDelay: {} ms", ctx.ping().await.as_millis())).await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Speak through the bot (raw)")
)]
pub async fn sayraw(ctx: Context<'_>, text: String) -> Result<(), Error> {
    let author_id: u64 = ctx.author().id.get();
    let channel: ChannelId = ctx.channel_id();

    // TODO: Make this configurable
    if lvl_points(get_points(&ctx.data(), author_id)) >= 30 {
        ctx.send(CreateReply::default()
            .content("ON IT")
            .ephemeral(true)
        ).await?;

        channel.say(&ctx.http(), text).await?;
    } else {
        ctx.send(CreateReply::default()
            .content("MUST BE AT LEAST LEVEL 30 TO USE SAYRAW")
            .ephemeral(true)
        ).await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Shuts the bot down"),
    owners_only
)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    let picked: &str;
    let splash: Vec<&str> = vec![
        "AU REVOIR",
        "ARRIVEDERCI",
        "FAREWELL",
        "SAYONARA",
    ];

    {
        let mut rng = rand::thread_rng();
        picked = splash[rng.gen_range(0..splash.len())];
    }

    ctx.say(picked).await?;
    ctx.framework().shard_manager.shutdown_all().await;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Pray to ethnic Peter"),
)]
pub async fn praytoethnicpeter(
  ctx: Context<'_>,
  #[description = "Multiplier on the point gain/loss (default 1, min 0, max 5)"]
  mult: Option<f32>,
) -> Result<(), Error> {
    let author_id: u64 = ctx.author().id.get();
    let splash: Vec<&str> = vec![
        "Hi Im Ethnic Peter",
        "petah",
        "what you gonna make me do? whack off a guy?",
        "and the",
        "allah is the greatest, but im greatester",
        "uhhhhhhhhhhhhhhhhhhhhhhhhhhhhh",
        "i want to eat Ethnic Food:tm:",
        "Quote - Quoteman",
        "witness my power ;)",
        "your mother was so great that i deemed her worthy of my jizz",
        "he knew GERIATRICWIZARDS",
        "try saying the magic word",
    ];

    let ep: u32;
    let picked: &str;
    let picked_i: usize;

    {
        let mut ethnicpeters = ctx.data().ethnicpeters.lock().unwrap();
        *ethnicpeters += 1;
        ep = *ethnicpeters;

        let mut rng = rand::thread_rng();
        picked_i = rng.gen_range(0..splash.len());
        picked = splash[picked_i];
    }

    let pts: i64 = (mult.unwrap_or(1.).max(0.).min(5.) * match picked_i {
        11          => -10.,
        2 | 4 | 9   =>  -5.,
        1 | 5 | 10  =>   5.,
        8           =>  10.,
        _           => 0.
    }).round() as i64;

    let extra: String = if      pts > 0 {add_points(&ctx.data(), author_id,  pts as u64); format!("YOU GAINED {} POINTS",  pts)}
                        else if pts < 0 {dec_points(&ctx.data(), author_id, -pts as u64); format!("YOU LOST {} POINTS",   -pts)}
                        else            {"NOTHING HAPPENED".to_string()};

    ctx.say(format!("**YOU PRAYED TO ETHNIC PETER**\nHE SAYS TO YOU: \"{}\"\n{}\n-# THIS IS PRAYER #{}", picked, extra, ep)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Show one of those godforsaken GIFs"),
)]
pub async fn gif(
    ctx: Context<'_>,
    #[description = "The GIF you want to use"] gif: Gif,
) -> Result<(), Error> {
    ctx.say(gif.link()).await?;
    Ok(())
}
