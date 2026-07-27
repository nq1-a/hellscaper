use std::collections::HashMap;

use poise::CreateReply;

use crate::{Context, Error};
use crate::types::chr::{Character, Stats};

#[poise::command(slash_command, subcommands(
    "new"
))]
pub async fn character(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Create a new character")
)]
async fn new(
    ctx: Context<'_>,
    #[description = "A unique three-letter identifier"]
    iden: String,
    #[description = "The character's name"]
    name: String,
    #[description = "Movement control & reaction time"]
    agility: i32,
    #[description = "Favorability by others"]
    charisma: i32,
    #[description = "Problem-solving, reasoning, & knowledge"]
    intelligence: i32,
    #[description = "Natural resistance to various types of harm"]
    resilience: i32,
    #[description = "Greater muscle power"]
    strength: i32,
) -> Result<(), Error> {
    let author: u64 = ctx.author().id.get();

    // Create stats
    let l_stats: Result<Stats, String> = Stats::new(
        agility,
        charisma,
        intelligence,
        resilience,
        strength,
    );

    if l_stats.is_err() {
        ctx.send(CreateReply::default()
            .content(l_stats.unwrap_err())
            .ephemeral(true)
        ).await?;

        return Ok(());
    };

    let stats: Stats = l_stats.unwrap();

    // Create character
    let valid: bool;

    'get: {
        let mut characters = ctx.data().characters.lock().unwrap();
        let iden_c = iden.clone();

        // Existence checks
        let cl = if characters.contains_key(&author) {
            characters.get_mut(&author).unwrap()
        } else {
            &mut characters.insert(author, HashMap::new()).unwrap()
        };

        valid = cl.contains_key(&iden_c);
        if valid {break 'get;}

        // New character insertion
        cl.insert(
            iden_c,
            Character {
                name,
                stats
            }
        );
    }

    // Send confirmation message
    ctx.send(CreateReply::default()
        .content(if valid {"A CHARACTER WITH THAT IDENTIFIER ALREADY EXISTS"}
                 else     {"CHARACTER CREATED"})
        .ephemeral(true)
    ).await?;

    // Wrap up
    Ok(())
}
