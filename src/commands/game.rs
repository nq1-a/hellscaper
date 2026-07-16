use tokio::time::{Duration, sleep};

use poise::CreateReply;
use poise::serenity_prelude::CreateMessage;

use crate::{Context, Error};

#[poise::command(slash_command, subcommands(
    "quicktime"
))]
pub async fn game(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Create a new quicktime event")
)]
async fn quicktime(
    ctx: Context<'_>,
    prompt: String,
    #[description = "What the player must say (case-insensitive)"]
    answer: String,
    #[description = "How long the player gets to answer (in seconds)"]
    time: u8,
) -> Result<(), Error> {
    ctx.send(CreateReply::default()
        .content("SUCCESS")
        .ephemeral(true)
    ).await?;
    
    let anchor = ctx.channel_id().send_message(
        &ctx.http(),
        CreateMessage::new().content(&prompt)
    ).await?;

    let mid: u64 = anchor.id.get();
    let uid: String = format!("{}:{}", mid, answer.to_lowercase());

    {
        let mut qt = ctx.data().quicktime.lock().unwrap();
        qt.insert(uid.clone(), vec![]);
    }

    sleep(Duration::from_secs(time as u64)).await;
    let mut ulist: String = "CORRECT GUESSERS:".to_string();

    'collect: {
        let qt = ctx.data().quicktime.lock().unwrap();
        
        if let Some(idv) = qt.get(&uid) {
            if idv.len() == 0 {
                ulist = "NO CORRECT GUESSES".to_string();
                break 'collect;
            }

            for id in idv {
                ulist = format!("{}\n<@{}>", ulist, id);
            }
        }
    }

    anchor.reply(ctx, format!("{}\nCORRECT ANSWER: {}", ulist, answer.to_uppercase())).await?;

    Ok(())
}
