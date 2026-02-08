use crate::{Context, Error};
use rand::Rng;

#[poise::command(
    slash_command,
    description_localized("en-US", "Returns the bot's GitHub page")
)]
pub async fn github(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://github.com/nq1-a/hellscaper").await?;
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
    description_localized("en-US", "Shuts the bot down")
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
