use rand::Rng;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Side count"] sides: u32,
) -> Result<(), Error> {
    ctx.say(format!("{} (d{})", rand::thread_rng().gen_range(1..=sides), sides)).await?;
    Ok(())
}
