use poise::CreateReply;

use crate::{Context, Error};

pub async fn say_eph(
    ctx: &Context<'_>,
    msg: &str,
) -> Result<(), Error> {
    ctx.send(CreateReply::default()
        .content(msg)
        .ephemeral(true)
    ).await?;

    Ok(())
}
