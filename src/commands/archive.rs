use std::fs::{
    read_dir,
    read_to_string as read_fs,
    write as write_fs,
};
use std::path::Path;

use poise::CreateReply;
use poise::serenity_prelude::{
    CreateAttachment,
    builder::GetMessages,
    model::id::{ChannelId, MessageId},
};
use tokio::fs::read as read_async;

use crate::{Context, Error};

#[poise::command(slash_command, subcommands(
    "save",
    "list",
    "get",
))]
pub async fn archive(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Write all text data in a channel to an archive entry")
)]
async fn save(
    ctx: Context<'_>,
    name: String,
    desc: String,
) -> Result<(), Error> {
    let anchor = ctx.say("SAVING TO FILE...").await?;

    // Get messages
    let mut current_msg: MessageId = anchor.clone()
        .into_message().await
        .unwrap().id;

    let channel: ChannelId = ctx.channel_id();
    let mut log: String = String::new();
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
        format!("{}\n{}\n{}\n\n{}", channel_id, name, desc, log)
    ).unwrap();

    anchor.edit(ctx, CreateReply::default()
        .content(format!("SAVED TO ENTRY {}\n-# description: {}", channel_id, desc))
    ).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Lists all archive entries")
)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let anchor = ctx.say("LOADING...").await?;

    // Create list of files
    let mut list: String = String::new();

    for path in read_dir("archive")? {
        if let Ok(p) = path {
            let lines: Vec<String> = read_fs(p.path())
                .unwrap()
                .lines()
                .map(String::from)
                .collect();

            list = format!("**{}** ({})\n*\"{}\"*\n\n{}",
                lines[1],
                lines[0],
                lines[2],
                list
            );
        }
    }

    // Display list
    anchor.edit(ctx, CreateReply::default().content(list)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get archive entry by ID")
)]
async fn get(
    ctx: Context<'_>,
    id: String,
) -> Result<(), Error> {
    let anchor = ctx.say("LOADING...").await?;

    // Check that ID is in a valid format
    if let Err(_) = id.parse::<u64>() {
        anchor.edit(ctx, CreateReply::default().content("INVALID ENTRY")).await?;
        return Ok(());
    }

    // Load file data into attachment
    let path = format!("archive/{}.md", id);
    let attachment = CreateAttachment::bytes(
        read_async(Path::new(&path)).await.unwrap(),
        path
    );

    // Send attachment
    ctx.send(CreateReply::default()
        .content("ENTRY LOADED")
        .attachment(attachment)
    ).await?;

    Ok(())
}
