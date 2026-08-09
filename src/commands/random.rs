use std::collections::HashMap;

use rand::Rng;

use crate::{Context, Error};
use crate::types::{
    traits::Bias,
    chr::{Character, Stats},
    weapon::Weapon,
    wroll::WRoll,
};

async fn wroll(
    ctx: Context<'_>,
    flags: String,
    iden_c: String,
    mut flag_cons: impl FnMut(char, &mut i32, &mut i32) -> i32,
    stat_cons: impl Fn(&Stats) -> i32,
    settings: WRoll<'_>,
) -> Result<(), Error> {
    let author: u64 = ctx.author().id.get();
    let mut ad: i32 = 0;
    let mut n1_bar: i32 = 0;

    // Get bar
    let mut bar: i32 = settings.init_bar;

    for c in flags.chars() {
        bar += flag_cons(c, &mut ad, &mut n1_bar);
    }

    // Update bar using stats
    let valid: bool;

    'sget: {
        let mut characters = ctx.data().characters.lock().unwrap();

        if !characters.contains_key(&author) {
            characters.insert(author, HashMap::new());
        };

        let cl = characters.get(&author).unwrap();
        let chr: Option<&Character> = cl.get(&iden_c);

        valid = chr.is_some();
        if !valid {break 'sget;}
        bar -= stat_cons(&chr.unwrap().stats);
    }

    if !valid {
        ctx.say("NO CHARACTER WITH THAT IDENTIFIER EXISTS").await?;
        return Ok(());
    }

    // Get roll
    let rolls: Vec<i32> = (1..(ad.abs() + 2))
        .map(|_| rand::thread_rng().gen_range(1..21))
        .collect();

    let roll: i32 = if ad >= 0 {*rolls.iter().max().unwrap()} else {*rolls.iter().min().unwrap()};

    // NAT
    let nat_min: bool = roll <= 1 + n1_bar;
    let nat: bool = nat_min || roll == 20;

    // Build message
    let mut res: String = String::new();

    if !nat {
        res += &if roll >= bar {format!("**{}** -- {} ≥ {}", settings.succ_msg.to_uppercase(), roll, bar)}
                else {format!("**{}** -- {} < {}", settings.fail_msg.to_uppercase(), roll, bar)};
    } else {
        res += &if nat_min {format!("**{}** -- ", settings.fumb_msg.to_uppercase())}
                else {format!("**{}** -- ", settings.crit_msg.to_uppercase())};
        res += &format!("NAT {}", roll);
    }

    res += &format!("\n-# flags: {}{}", flags, settings.tail_msg);
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(slash_command, subcommands(
    "flip",
    "roll",
    "shoot",
))]
pub async fn random(_ctx: Context<'_>) -> Result<(), Error> {Ok(())}

#[poise::command(
    slash_command,
    description_localized("en-US", "Flip a coin")
)]
async fn flip(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(
        if rand::thread_rng().gen_range(0..2) == 1 {"HEADS (1)"}
        else {"TAILS (0)"}
    ).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Roll a die")
)]
async fn roll(
    ctx: Context<'_>,
    #[description = "Side count"] sides: u32,
) -> Result<(), Error> {
    ctx.say(format!("{} (d{})", rand::thread_rng().gen_range(1..=sides), sides)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Use a ranged weapon")
)]
pub async fn shoot(
    ctx: Context<'_>,
    #[description = "The unique ID of your character"] character: String,
    #[description = "Your weapon"] weapon: Weapon,
    #[description = "List of modifiers to your attack"] flags: String,
) -> Result<(), Error> {
    return wroll(
        ctx,
        flags
            + if weapon.auto() {"r"} else {""}
            + if weapon.experimental() {"E"} else {""},
        character,
        |c, ad, n1_bar| match c {
            'A' => {*ad += 1; 0},
            'r' => {*n1_bar += 4; *ad += 1; 2},
            'E' => {*n1_bar = 1.max(*n1_bar * 2); 0},
            'F' => {*ad += 1; 4},
            'b' => {*ad -= 1; 0},
            'B' => {*ad -= 1; 0},
            'D' => {*ad -= 1; 0},
            'e' => -4,
            'n' => -4,
            'd' => -3,
            's' => -3,
            'l' => -2,
            'o' =>  0,
            'a' =>  1,
            'L' =>  3,
            'v' =>  3,
            'f' =>  5,
            _   =>  0
        },
        |stats| if weapon.aoe() {1} else {stats.agility / 2} +
                stats.intelligence / 2 +
                if weapon.innate() {stats.strength} else {0},
        WRoll {
            init_bar: 11 - weapon.bias(),
            crit_msg: "CRIT!",
            succ_msg: "HIT",
            fail_msg: "MISS",
            fumb_msg: weapon.jam_msg(),
            tail_msg: &format!(", weapon: {:?}", weapon),
        }
    ).await;
}
