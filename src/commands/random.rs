use rand::Rng;

use crate::{Context, Error};
use crate::types::traits::Bias;
use crate::types::weapon::Weapon;

#[poise::command(
    slash_command,
    description_localized("en-US", "Flip a coin")
)]
pub async fn flip(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn roll(
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
    #[description = "Your weapon"] weapon: Weapon,
    #[description = "List of modifiers to your attack"] flags: String,
) -> Result<(), Error> {
    let mut ad: i32 = 0;

    // Get bar
    let mut bar: i32 = 11;

    for c in flags.chars() {
        bar += match c {
            'A' => {ad += 1; 0},
            'B' => {ad -= 1; 0},
            'D' => {ad -= 1; 0},
            'e' => -4,
            'n' => -4,
            'd' => -3,
            'l' => -3,
            's' => -1,
            'S' => -1,
            'o' =>  0,
            'a' =>  2,
            'b' =>  2,
            'L' =>  3,
            'v' =>  4,
            'f' =>  5,
            _   =>  0
        };
    }

    bar -= weapon.bias();

    // Get roll
    let rolls: Vec<i32> = (1..(ad.abs() + 2))
        .map(|_| rand::thread_rng().gen_range(1..21))
        .collect();

    let roll: i32 = if ad >= 0 {*rolls.iter().max().unwrap()} else {*rolls.iter().min().unwrap()};

    // NAT
    let nat_min: bool = roll == 1 || (weapon == Weapon::railgun && roll == 2);
    let nat: bool = nat_min || roll == 20;

    // Build message
    let mut res: String = String::new();
    
    if !nat {
        res += &if roll >= bar {format!("**HIT** -- {} ≥ {}", roll, bar)}
                else {format!("**MISS** -- {} < {}", roll, bar)};
    } else {
        res += if nat_min {
                    if weapon.jammable() {"**JAMMED** -- "}
                    else {"**MISS** -- "}}
               else {"**CRIT!** -- "};
        res += &format!("NAT {}", roll);
    }

    res += &format!("\n-# weapon: {:?}, flags: {}", weapon, flags);

    ctx.say(res).await?;
    Ok(())
}
