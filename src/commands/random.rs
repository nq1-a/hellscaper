use std::collections::HashMap;

use rand::Rng;

use crate::{Context, Error};
use crate::types::{
    traits::Bias,
    chr::{Character, Stats},
    weapon::{MeleeWeapon, RangedWeapon},
    wroll::WRoll,
};

async fn wroll(
    ctx: Context<'_>,
    flags: String,
    iden_c: String,
    mut flag_cons: impl FnMut(char, &mut i32, &mut i32, &mut i32) -> i32,
    stat_cons: impl Fn(&Stats) -> i32,
    settings: WRoll<'_>,
) -> Result<(), Error> {
    let author: u64 = ctx.author().id.get();
    let mut ad: i32 = 0;
    let mut aux_bias: i32 = 0;
    let mut n1_bar: i32 = settings.n1_bar_d;

    // Get bar
    let mut bar: i32 = settings.init_bar;

    for c in flags.chars() {
        bar += flag_cons(c, &mut ad, &mut n1_bar, &mut aux_bias);
    }

    // Update bar using stats
    let valid: bool;
    let mut chr_name: String = String::new();

    'sget: {
        let mut characters = ctx.data().characters.lock().unwrap();

        if !characters.contains_key(&author) {
            characters.insert(author, HashMap::new());
        };

        let cl = characters.get(&author).unwrap();
        let chr: Option<&Character> = cl.get(&iden_c.to_lowercase());

        valid = chr.is_some();
        if !valid {break 'sget;}
        let chr_u: &Character = chr.unwrap();
        bar -= stat_cons(&chr_u.stats);
        chr_name = chr_u.name.clone();
    }

    if !valid {
        ctx.say("NO CHARACTER WITH THAT IDENTIFIER EXISTS").await?;
        return Ok(());
    }

    // Get roll
    let rolls: Vec<i32> = (1..(ad.abs() + 2))
        .map(|_| rand::thread_rng().gen_range(1..21))
        .collect();

    let roll: i32 = if ad >= 0 {*rolls.iter().max().unwrap()} else {*rolls.iter().min().unwrap()}
                    + settings.pre_bias
                    + aux_bias;

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

    res += &format!("\n-# character: {}, flags: {}{}", chr_name.to_lowercase(), flags, settings.tail_msg);
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(slash_command, subcommands(
    "blast",
    "clash",
    "flip",
    "learn",
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
    #[description = "Your weapon"] weapon: RangedWeapon,
    #[description = "List of modifiers to your attack"] flags: String,
) -> Result<(), Error> {
    return wroll(
        ctx,
        flags
            + if weapon.auto() {"r"} else {""}
            + if weapon.experimental() {"E"} else {""},
        character,
        |c, ad, n1_bar, _| match c {
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
            pre_bias: 0,
            n1_bar_d: 0,
        }
    ).await;
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Try to learn information")
)]
pub async fn learn(
    ctx: Context<'_>,
    #[description = "The unique ID of your character"] character: String,
    #[description = "List of modifiers to your action"] flags: String,
) -> Result<(), Error> {
    return wroll(
        ctx,
        flags,
        character,
        |c, ad, _n1_bar, _| match c {
            'A' => {*ad += 1; 0},
            'D' => {*ad -= 1; 0},
            'n' => -4,
            'g' => -3,
            'e' => -2,
            's' => -2,
            'p' =>  2,
            'd' =>  3,
            't' =>  3,
            'i' =>  5,
            _   =>  0
        },
        |stats| stats.intelligence,
        WRoll {
            init_bar: 10,
            crit_msg: "EUREKA!",
            succ_msg: "LEARNED",
            fail_msg: "FAILED",
            fumb_msg: "REGRESSED",
            tail_msg: "",
            pre_bias: 0,
            n1_bar_d: 0,
        }
    ).await;
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get caught in a blast")
)]
async fn blast(
    ctx: Context<'_>,
    #[description = "The unique ID of your character"] character: String,
    #[description = "List of modifiers to your action"] flags: String,
) -> Result<(), Error> {
    return wroll(
        ctx,
        flags,
        character,
        |c, ad, n1_bar, _| match c {
            'A' => {*ad += 1; 0},
            'S' => {*ad += 1; 0},
            'D' => {*ad -= 1; 0},
            'C' => {*n1_bar += 2; 0},
            'f' => -5,
            't' =>  3,
            'n' =>  6,
            _   =>  0
        },
        |stats| stats.agility / 3 +
                stats.resilience,
        WRoll {
            init_bar: 9,
            crit_msg: "UNSCATHED",
            succ_msg: "SCATHED",
            fail_msg: "HIT",
            fumb_msg: "EVISCERATED",
            tail_msg: "",
            pre_bias: 0,
            n1_bar_d: 0,
        }
    ).await;
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Clash against someone melee-to-melee")
)]
pub async fn clash(
    ctx: Context<'_>,
    #[description = "The unique ID of your character"] character: String,
    #[description = "Your weapon"] weapon: MeleeWeapon,
    #[description = "List of modifiers to your attack"] flags: String,
) -> Result<(), Error> {
    return wroll(
        ctx,
        flags,
        character,
        |c, ad, n1_bar, aux_bias| match c {
            'A' => {*ad += 1; 0},
            'D' => {*ad -= 1; 0},
            'd' => {*n1_bar += 3; *aux_bias += 1; 0},
            'r' => -2,
            't' =>  3,
            'i' =>  4,
            _   =>  0
        },
        |stats| stats.agility / 2 +
                stats.strength,
        WRoll {
            init_bar: 13,
            crit_msg: "CRIT!",
            succ_msg: "VANTAGE",
            fail_msg: "COUNTER",
            fumb_msg: "FUMBLE",
            tail_msg: &format!(", weapon: {:?}", weapon),
            pre_bias: weapon.bias(),
            n1_bar_d: 2,
        }
    ).await;
}
