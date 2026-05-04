use poise::serenity_prelude::User;

use crate::{Context, Error};

fn lvl_points(pts: u64) -> u64 {
    let fpts: f64 = pts as f64;

    (((fpts + 25.) / 4.5).log(2.2) + fpts / (1000. + fpts / 100.) - 1.)
    .max(1.) as u64
}

#[poise::command(slash_command, subcommands(
    "view",
))]
pub async fn level(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "View any person's level")
)]
async fn view(
    ctx: Context<'_>,
    user: Option<User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let points_t: u64;

    {
        let points = ctx.data().points.lock().unwrap();
        points_t = *points.get(&target.id.get()).unwrap_or(&0);
    }

    ctx.say(format!("**{}**\nLEVEL: {}\nPOINTS: {}",
        target.name,
        lvl_points(points_t),
        points_t,
    )).await?;

    Ok(())
}
