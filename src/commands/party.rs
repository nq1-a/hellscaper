use crate::{Context, Error};
use crate::types::arc::Campaign;

#[poise::command(slash_command)]
pub async fn partynew(
    ctx: Context<'_>,
    name: String,
) -> Result<(), Error> {
    let author: u64 = ctx.author().id.get();

    ctx.data().campaigns.lock().unwrap().insert(
        name.clone(),
        Campaign::new(author)
    );

    // TODO Make this actually ping campaign events
    ctx.say(format!("<@1459061785909788855>\n# NEW CAMPAIGN\n**HOST:** <@{}>\n\nJOIN BY TYPING `/partyjoin {}`", author, &name)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn partyjoin(
    ctx: Context<'_>,
    name: String,
) -> Result<(), Error> {
    let mut success = false;

    {
        let mut campaigns = ctx.data().campaigns.lock().unwrap();
        
        if let Some(c) = campaigns.get_mut(&name) {
            c.add(ctx.author().id.get());
            success = true;
        }
    }

    ctx.say(if success {"SUCCESS!"} else {"CAMPAIGN NOT FOUND"}).await?;
    Ok(())
}
