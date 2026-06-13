use std::fs::{
    read_to_string as read_fs,
};

use rand::Rng;

use poise::CreateReply;
use poise::serenity_prelude::{
    model::id::ChannelId,
};

use crate::{Context, Error};
use crate::commands::level::lvl_points;
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
pub async fn help(ctx: Context<'_>, command: String) -> Result<(), Error> {
    let command_san: String = command.replace(&['/', '\\'], "");

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
    let lvl: u64;

    {
        let points = ctx.data().points.lock().unwrap();
        lvl = lvl_points(*points.get(&author_id).unwrap_or(&0));
    }

    if lvl >= 35 {
        ctx.send(CreateReply::default()
            .content("ON IT")
            .ephemeral(true)
        ).await?;

        channel.say(&ctx.http(), text).await?;
    } else {
        ctx.send(CreateReply::default()
            .content("MUST BE AT LEAST LEVEL 35")
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
    description_localized("en-US", "Ask for ethnic Peter"),
)]
pub async fn askforethnicpeter(ctx: Context<'_>) -> Result<(), Error> {
    let ep: u32;

    {
        let mut ethnicpeters = ctx.data().ethnicpeters.lock().unwrap();
        *ethnicpeters += 1;
        ep = *ethnicpeters;
    }

    ctx.say(format!("**YOU ASKED FOR ETHNIC PETER**\nTHIS IS ASK #{}", ep)).await?;
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
