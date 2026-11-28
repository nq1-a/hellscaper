use poise::serenity_prelude::{self as serenity, Context};
use rand::Rng;
use regex::Regex;

pub struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: serenity::Message) {
        if msg.content.starts_with(".") {
            let re: Regex = Regex::new(r"\.((\d+)\/)?(\d+)?d(\d+)([-\+]\d+)?([AD])?").unwrap();
            if let Some(caps) = re.captures(&msg.content) {
                let roll_count: usize = match caps.get(3) {
                    Some(c) => c.as_str().parse::<usize>().unwrap_or(1),
                    None => 1,
                }.min(100);

                let roll_max: i32 = caps[4].parse::<i32>().unwrap();

                let mut rolls: Vec<i32> = vec![];
                let mut nat_mins: Vec<bool> = vec![];
                let mut nats: Vec<bool> = vec![];

                rolls.resize_with(roll_count, || {
                    let mut roll: i32 = rand::thread_rng().gen_range(1..=roll_max);

                    // Advantage/disadvantage
                    if let Some(ad) = caps.get(6).map(|m| m.as_str()) {
                        let alt_roll: i32 = rand::thread_rng().gen_range(1..=roll_max);
                        if ad.starts_with("A") {roll = roll.max(alt_roll)}
                        if ad.starts_with("D") {roll = roll.min(alt_roll)}
                    }

                    // NAT & bias
                    let nat_min: bool = roll == 1;
                    let nat: bool = nat_min || roll == roll_max;

                    nat_mins.push(nat_min);
                    nats.push(nat);

                    if !nat && let Some(bias) = caps.get(5).map(|m| m.as_str()) {
                        roll += bias.parse::<i32>().unwrap();
                    }

                    roll
                });

                // Build message
                let mut res: String = String::new();

                for (i, r) in rolls.iter().enumerate() {
                    if nats[i] {res += "NAT ";}
                    res += &r.to_string();
                    if i < rolls.len() - 1 {res += ", ";}
                }

                res += "\n";

                if let Some(dc_raw) = caps.get(2).map(|m| m.as_str()) {
                    let dc: i32 = dc_raw.parse::<i32>().unwrap();
                    res += &format!("**DC {} ", dc);
                    res += if nat_mins[0] || (!nats[0] && dc > rolls[0]) {"FAILURE**"} else {"SUCCESS**"};
                }

                // Output
                let _ = msg.reply(&ctx.http, res).await;
            }
        }
    }
}
