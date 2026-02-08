use std::time::Duration;

use poise::CreateReply;
use poise::serenity_prelude::{
    CreateAllowedMentions,
    CreateMessage,
    CreatePoll,
    CreatePollAnswer,
};

use crate::{Context, Error};
use crate::types::arc::Campaign;

#[poise::command(slash_command, subcommands(
    "new",
    "join",
    "list",
    "poll",
))]
pub async fn party(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Create a new party")
)]
async fn new(
    ctx: Context<'_>,
    name: String,
) -> Result<(), Error> {
    let author: u64 = ctx.author().id.get();

    ctx.data().campaigns.lock().unwrap().insert(
        name.clone(),
        Campaign::new(author)
    );

    ctx.say(format!("<@&{}>\n# NEW CAMPAIGN\n**HOST:** <@{}>\n\nJOIN BY TYPING `/party join {}`",
        ctx.data().config.get("campaign_role").unwrap(),
        author,
        &name)
    ).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Join an existing party")
)]
async fn join(
    ctx: Context<'_>,
    name: String,
) -> Result<(), Error> {
    let mut success: bool = false;

    {
        let mut campaigns = ctx.data().campaigns.lock().unwrap();
        
        if let Some(c) = campaigns.get_mut(&name) {
            c.add(ctx.author().id.get());
            success = true;
        }
    }

    ctx.say(if success {"SUCCESS"} else {"CAMPAIGN NOT FOUND"}).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "List existing parties")
)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let mut list: String = String::new();

    {
        let campaigns = ctx.data().campaigns.lock().unwrap();

        if campaigns.len() == 0 {
            list = "NO CAMPAIGNS FOUND".to_string();
        } else {
            for (k, v) in campaigns.iter() {
                list = format!("{}## {}\n-# Members: {}\n\n",
                    list,
                    k,
                    v.ping_list(", ")
                );
            } 
        }
    }

    ctx.send(CreateReply::default()
        .content(list)
        .allowed_mentions(CreateAllowedMentions::new().empty_users())
    ).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Pings & polls an entire party for readiness")
)]
async fn poll(
    ctx: Context<'_>,
    name: String,
) -> Result<(), Error> {
    // Get campaign data
    let mentions: String;
    let owner: u64;

    {
        let campaigns = ctx.data().campaigns.lock().unwrap();
        let c = campaigns.get(&name).unwrap();

        mentions = c.ping_all();
        owner = c.owner();
    }

    if owner != ctx.author().id.get() {
        ctx.send(CreateReply::default()
            .content("ERROR: YOU ARE NOT THE HOST")
            .ephemeral(true)
        ).await?;

        return Ok(());
    }

    // Create poll
    let poll = CreatePoll::new()
        .question("ready?")
        .answers(vec![
            CreatePollAnswer::new().text("ready").emoji("✅".to_string()),
            CreatePollAnswer::new().text("in a bit"),
            CreatePollAnswer::new().text("in 1-2 hours"),
            CreatePollAnswer::new().text("in 3-6 hours"),
            CreatePollAnswer::new().text("not this session").emoji("❌".to_string()),
        ])
        .duration(Duration::from_secs(3600));

    // Send
    ctx.channel_id().send_message(
        &ctx.http(),
        CreateMessage::new()
            .content(mentions)
            .poll(poll)
    ).await?;

    // Reply & return
    ctx.send(CreateReply::default()
        .content("SUCCESS")
        .ephemeral(true)
    ).await?;

    Ok(())
}
