use poise::CreateReply;
use poise::serenity_prelude::{
    CreateAllowedMentions,
    Role,
    User,
};

use crate::{Context, Error};
use crate::types::data::Data;

pub fn add_points(data: &Data, user: u64, pts: u64) {
    let mut points = data.points.lock().unwrap();
    if let Some(p) = points.get_mut(&user) {*p += pts;}
    else {points.insert(user, pts);}
}

pub fn get_points(data: &Data, user: u64) -> u64 {
    let points = data.points.lock().unwrap();
    return *points.get(&user).unwrap_or(&0);
}

pub fn lvl_points(pts: u64) -> u64 {
    let fpts: f64 = pts as f64;
    (((fpts + 25.) / 4.5).log(2.2) + fpts / (1000. + fpts / 100.) - 1.)
    .max(1.) as u64
} 

#[poise::command(slash_command, subcommands(
    "view",
    "leaderboard",
    "vanitynew",
    "vanityequip",
    "vanityunequip",
    "vanitylist",
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
    let points_t: u64 = get_points(&ctx.data(), target.id.get());

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

        return Ok(());
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

#[poise::command(
    slash_command,
    description_localized("en-US", "Create a new vanity role"),
    required_permissions = "ADMINISTRATOR"
)]
async fn vanitynew(
    ctx: Context<'_>,
    role: Role,
    lvl: u64,
) -> Result<(), Error> {
    let role_id: u64 = role.id.get();
    let valid: bool;

    'get: {
        let mut vanities = ctx.data().vanities.lock().unwrap();
        valid = vanities.contains_key(&role_id);
        if valid {break 'get;}

        vanities.insert(
            role_id,
            lvl
        );
    }

    if !valid {
        ctx.send(CreateReply::default()
            .content("VANITY ROLE SET")
            .ephemeral(true)
        ).await?;
    } else {
        ctx.send(CreateReply::default()
            .content("THAT ROLE IS ALREADY A VANITY ROLE")
            .ephemeral(true)
        ).await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Equip a vanity role")
)]
async fn vanityequip(
    ctx: Context<'_>,
    role: Role,
) -> Result<(), Error> {
    let author_id: u64 = ctx.author().id.get();
    let role_id: u64 = role.id.get();
    let mut successes: u8 = 0;

    if let Some(member) = ctx.author_member().await {
        {
            let vanities = ctx.data().vanities.lock().unwrap();

            if let Some(l) = vanities.get(&role_id) {
                successes += 1u8;
                if lvl_points(get_points(&ctx.data(), author_id)) >= *l {
                    successes += 1u8;
                }
            }
        }

        if successes == 2u8 {
            member.add_role(&ctx.http(), role.id).await?;
            successes = 3u8;
        }
    }

    let messages: Vec<&str> = vec![
        "ROLE IS NOT A VANITY ROLE",
        "YOUR LEVEL IS TOO LOW",
        "COULD NOT EQUIP ROLE",
        "EQUIP SUCCESS",
    ];

    ctx.say(messages[successes as usize]).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Unequip a vanity role")
)]
async fn vanityunequip(
    ctx: Context<'_>,
    role: Role,
) -> Result<(), Error> {
    let role_id: u64 = role.id.get();
    let mut successes: u8 = 0;

    // FIXME: This code is absolute garbage lmao
    if let Some(member) = ctx.author_member().await {
        {
            let vanities = ctx.data().vanities.lock().unwrap();

            if let Some(_) = vanities.get(&role_id) {
                successes += 1u8;
            }
        }

        if successes == 1u8 {
            member.remove_role(&ctx.http(), role.id).await?;
            successes = 2u8;
        }
    }

    let messages: Vec<&str> = vec![
        "ROLE IS NOT A VANITY ROLE",
        "COULD NOT UNEQUIP ROLE (DO YOU EVEN HAVE IT ON?)",
        "UNEQUIP SUCCESS",
    ];

    ctx.say(messages[successes as usize]).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "List vanity roles")
)]
async fn vanitylist(ctx: Context<'_>) -> Result<(), Error> {
    // Get board data
    let mut board: Vec<(u64, u64)>;

    {
        let vanities = ctx.data().vanities.lock().unwrap();
        board = vanities
            .iter()
            .map(|v| (*v.0, *v.1))
            .collect::<Vec<_>>()
            .clone();
    }
    
    // Check to see if we actually need to do anything
    if board.len() == 0 {
        ctx.send(CreateReply::default()
            .content("NO VANITY ROLES CREATED")
            .ephemeral(true)
        ).await?;

        return Ok(());
    }

    // Sort board
    board.sort_by(|a, b| a.1.cmp(&b.1));

    // Format & send
    let entries: Vec<String> = board
        .iter()
        .map(|s| format!("<@&{}> (LEVEL {})", s.0, s.1))
        .collect();

    ctx.send(CreateReply::default()
        .content(entries.join("\n"))
        .allowed_mentions(CreateAllowedMentions::new().empty_users())
    ).await?;

    Ok(())
}

