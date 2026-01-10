use poise::serenity_prelude as serenity;
use rand::Rng;
use regex::Regex;

pub struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: serenity::Message) {
        if &msg.content[0..1] != "." { return; }

        let re: Regex = Regex::new(r"\.((\d+)\/)?(\d+)?d(\d+)([-\+]\d+)?([AD])?").unwrap();
        if let Some(caps) = re.captures(&msg.content) {
            let roll_max: i32 = caps[4].parse::<i32>().unwrap();
            let mut roll: i32 = rand::thread_rng().gen_range(1..=roll_max);
           
            let nat_max: bool = roll == roll_max;
            let nat_min: bool = roll == 1;
            let nat: bool = nat_min || nat_max;

            if !nat && let Some(bias) = caps.get(5).map(|m| m.as_str()) {
                roll += bias.parse::<i32>().unwrap();
            }

            // Build message
            let mut res: String = String::new();
            if nat {res += "NAT ";}
            res += &format!("{}\n", roll);

            if let Some(dc_raw) = caps.get(2).map(|m| m.as_str()) {
                let dc: i32 = dc_raw.parse::<i32>().unwrap();
                res += &format!("**DC {} ", dc);
                res += if nat_min || (!nat_max && dc > roll) {"FAILURE**"} else {"SUCCESS**"};
            }

            // Output
            let _ = msg.reply(&ctx.http, res).await;
        }
    }
}
