use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn github(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://github.com/nq1-a/hellscaper").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let millis: u128 = ctx.ping().await.as_millis();

    if millis == 0 {
       ctx.say("bot has not yet performed a full heartbeat --- please try again later").await?; 
    } else {
        ctx.say(format!("**PONG!**\nDelay: {} ms", ctx.ping().await.as_millis())).await?;
    }

    Ok(())
}

#[poise::command(slash_command, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Au revoir").await?;
    ctx.framework().shard_manager.shutdown_all().await;
    Ok(())
}
