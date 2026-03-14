use rand::Rng;

use poise::CreateReply;
use poise::serenity_prelude::{
    model::id::ChannelId,
};

use crate::{Context, Error};

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
    description_localized("en-US", "Speak through the bot (raw)"),
    owners_only
)]
pub async fn sayraw(ctx: Context<'_>, text: String) -> Result<(), Error> {
    let channel: ChannelId = ctx.channel_id();

    ctx.send(CreateReply::default()
        .content("ON IT")
        .ephemeral(true)
    ).await?;

    channel.say(&ctx.http(), text).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Shuts the bot down"),
    required_permissions = "ADMINISTRATOR"
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
