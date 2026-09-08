use std::collections::HashMap;

use poise::CreateReply;
use poise::serenity_prelude::User;

use crate::{Context, Error};
use crate::types::chr::{Character, Stats};

#[poise::command(slash_command, subcommands(
    "new",
    "list",
    "view",
))]
pub async fn character(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Create a character")
)]
async fn new(
    ctx: Context<'_>,
    #[description = "A unique identifier (3 lowercase letters/special symbols)"]
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
    #[description = "Allows stats to add to greater than 2"]
    max_override: Option<bool>,
    #[description = "Can override an existing character's stats if true"]
    allow_edit: Option<bool>,
) -> Result<(), Error> {
    // Make sure iden is valid
    if iden.len() != 3 {
        ctx.send(CreateReply::default()
            .content("LENGTH OF IDENTIFIER MUST BE EXACTLY 3")
            .ephemeral(true)
        ).await?;
    }

    // Get the author's user ID
    let author: u64 = ctx.author().id.get();

    // Create stats
    let l_stats: Result<Stats, String> = Stats::new(
        agility,
        charisma,
        intelligence,
        resilience,
        strength,
        max_override.unwrap_or_default(),
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
        let iden_c = iden.to_lowercase();

        // Existence checks
        if !characters.contains_key(&author) {
            characters.insert(author, HashMap::new());
        };

        let cl = characters.get_mut(&author).unwrap();

        valid = cl.contains_key(&iden_c);
        let editing: bool = allow_edit.unwrap_or_default();

        // Edit a character if it already exists
        if valid && editing {
            if let Some(c) = cl.get_mut(iden_c.as_str()) {
                c.stats.agility = agility;
                c.stats.charisma = charisma;
                c.stats.intelligence = intelligence;
                c.stats.resilience = resilience;
                c.stats.strength = strength;
            }
        }

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

#[poise::command(
    slash_command,
    description_localized("en-US", "Show an existing character")
)]
async fn view(
    ctx: Context<'_>,
    #[description = "The character's identifier"]
    iden: String,
    #[description = "Whose character it is"]
    user: Option<User>,
) -> Result<(), Error> {
    let target: u64 = user.as_ref().unwrap_or_else(|| ctx.author()).id.get();
    let view: String;

    {
        let mut characters = ctx.data().characters.lock().unwrap();

        if !characters.contains_key(&target) {
            characters.insert(target, HashMap::new());
        }

        let cl = characters.get_mut(&target).unwrap();

        if cl.len() == 0 {
            view = "TARGET HAS NO CHARACTERS".to_string();
        } else if let Some(c) = cl.get(iden.as_str()) {
            view = format!("**{}** ({}){}\n{}\n\n",
                c.name,
                iden,
                if c.stats.sum() > 2 {"\nOVERRIDE"} else {""},
                c.stats,
            );
        } else {
            view = "CHARACTER NOT FOUND".to_string();
        }
    }

    ctx.send(CreateReply::default()
        .content(view)
        .ephemeral(true)
    ).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "List existing characters")
)]
async fn list(
    ctx: Context<'_>,
    user: Option<User>,
    #[description = "Which page you're on starting from 1 (each has 8 characters)"]
    page: Option<u16>,
) -> Result<(), Error> {
    let target: u64 = user.as_ref().unwrap_or_else(|| ctx.author()).id.get();
    let mut list: String;

    let page_c: u16 = page.unwrap_or(1).max(1);
    let page_s: u16 = page_c - 1 << 3;

    {
        let mut characters = ctx.data().characters.lock().unwrap();

        if !characters.contains_key(&target) {
            characters.insert(target, HashMap::new());
        }

        let cl = characters.get_mut(&target).unwrap();

        if cl.len() == 0 {
            list = "TARGET HAS NO CHARACTERS".to_string();
        } else if cl.len() as u16 <= page_s {
            list = "NO CHARACTERS FOUND ON THIS PAGE".to_string();
        } else {
            list = format!("# PAGE {}/{}\n", page_c, (cl.len() - 1) / 8 + 1);
            let mut i: u16 = 0;

            let mut keys: Vec<_> = cl.keys()
                .clone()
                .collect::<Vec<_>>();
            keys.sort();

            for k in keys {
                if i >= page_s + 8 {break;}
                let v = cl.get(k.as_str()).unwrap();

                if i >= page_s {
                    list = format!("{}**{}** ({}){}\n{}\n\n",
                        list,
                        v.name,
                        k,
                        if v.stats.sum() > 2 {"\nOVERRIDE"} else {""},
                        v.stats,
                    );
                }

                i += 1;
            }
        }
    }

    ctx.send(CreateReply::default()
        .content(list)
        .ephemeral(true)
    ).await?;

    Ok(())
}
