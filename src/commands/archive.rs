use std::fs::write as write_fs;

use poise::serenity_prelude::{
    builder::GetMessages,
    model::id::{ChannelId, MessageId},
};

use crate::{Context, Error};

#[poise::command(slash_command, subcommands(
    "save",
))]
pub async fn archive(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Create a new party")
)]
async fn save(
    ctx: Context<'_>,
    desc: String,
) -> Result<(), Error> {
    let anchor = ctx.say("SAVING TO FILE...").await;

    // Get messages
    let mut current_msg: MessageId = anchor
        .unwrap()
        .into_message().await
        .unwrap().id;

    let channel: ChannelId = ctx.channel_id();
    let mut log: String = "".to_string();
    let mut batch: u32 = 1;

    loop {
        let msg_list = channel.messages(
            &ctx.http(),
            GetMessages::new().before(current_msg).limit(100)
        ).await.unwrap_or(vec![]);

        if msg_list.len() == 0 {break;}

        for msg in &msg_list {
            log = format!("**{}:** {}\n{}",
                msg.author.name.clone(),
                msg.content.clone(),
                log
            );
        }

        current_msg = msg_list.last().unwrap().id;
        println!("batch {} length {}", batch, msg_list.len());
        batch += 1;
    }

    // Write to file
    let channel_id: u64 = channel.get();

    let _ = write_fs(
        format!("archive/{}.md", channel_id),
        format!("#description {}\n\n{}", desc, log)
    ).unwrap();

    ctx.say(format!("SAVED TO ENTRY {}\n-# description: {}", channel_id, desc)).await?;

    Ok(())
}
