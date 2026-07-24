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
        // 0-9
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
        // 10-19
        "he knew GERIATRICWIZARDS",
        "try saying the magic word",
        &format!("<@{}> haha get pinged", author_id),
        "ac6d3a0502ed084c26488848927155daaca487de6ac36f65694f87f8c0d0a8eb1564ccb284c0fc5b3f20f708b6818213be2f479be84507499ef186ac0f0771db098ae096c15585d4dce2f14d68a50a8337e03d218f8523d88a1513f0a7465498ffe23ba85a698a9e773aa85e1ab039b7ea45e6064e5cda14c4489b454e3c0d7839773cfe645d53f3316cbcef1608255af79e0a94f42c8090d9d0ab1545f6eec2556e9fb5e444555200106d1f035ef28b22b5ac9a1a2cbaa56358670c85053af924205ed3b8ca3064fd60332d1a89df102b4419c7171ea0088d31e5541ccd29d0702b68bf0484c456a37df1f2900ae1cc6ab10261678eb339a3ffdc1652822727e24f5900f4b5fd0a8493f1d85c2abd96ca1ac217fba11073c235c5cd7f80acb0bc3f7aa19b6ac57c24ee68ed4a6952f951b0aaddfc0e8cdca7f471a1b5973ea19d35fa2e0a42feb5b2615491a203fed94d1ea836e139532948db4566b81602693fdd051abbd19d3d4a1c3826776989fa85969fcc8cfd77b0d0e215b4207a56280f8ac8d1f9504e1d53eb48371b4f78e80ff790397729d6c3d27d7ab783c023ce63b867fd320c9020f9ea3206f571b9d86ce8ec4e997490337b1848c63c903273e96f8eddb4f28443cb1cd330dec83773fbf7b857e1eac58cea7e5d07ae333a66cdd5b647ef1ccf12cdd94b0872fdc402dfb514ec06d1648264160412881953c84b64aff8b0d4bb4a306183a1b3876307fadfb925458dd8166286fd64c8dbd26f1a925c2d55cc91e6a380eac5c0faac6dc92a35abf75bfee86119ada86191b420c26a0c4d0b27df50f3a33e637e2739a9379ca83d7889b171e4bf9dfa870abfbd",
        "have some free points",
        "i am indifferent to genocide",
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
        11                      => -10.,
        2 | 4 | 9  | 12 | 15    =>  -5.,
        1 | 5 | 10              =>   5.,
        8 | 14                  =>  10.,
        _                       =>   0.
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
