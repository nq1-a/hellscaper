use poise::CreateReply;
use poise::serenity_prelude::{
    CreateAllowedMentions,
    User,
};

use crate::{Context, Error};

fn lvl_points(pts: u64) -> u64 {
    let fpts: f64 = pts as f64;

    (((fpts + 25.) / 4.5).log(2.2) + fpts / (1000. + fpts / 100.) - 1.)
    .max(1.) as u64
}

#[poise::command(slash_command, subcommands(
    "view",
    "leaderboard",
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

#[poise::command(
    slash_command,
    description_localized("en-US", "See everyone's ranked scores")
)]
async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    // Get leaderboard data
    let mut board: Vec<(u64, u64)>;

    {
        let points = ctx.data().points.lock().unwrap();
        board = points
            .iter()
            .map(|s| (*s.0, *s.1))
            .collect::<Vec<_>>()
            .clone();
    }
    
    // Check to see if we actually need to do anything
    if board.len() == 0 {
        ctx.send(CreateReply::default()
            .content("NO DATA!")
            .ephemeral(true)
        ).await?;

        Ok(())
    }

    // Sort board
    board.sort_by(|a, b| b.1.cmp(&a.1));

    // Format & send
    let entries: Vec<String> = board[0..10.min(board.len())]
        .iter()
        .map(|s| format!("<@{}>: LEVEL {} ({} POINTS)", s.0, lvl_points(s.1), s.1))
        .collect();

    ctx.send(CreateReply::default()
        .content(entries.join("\n"))
        .allowed_mentions(CreateAllowedMentions::new().empty_users())
    ).await?;

    Ok(())
}
