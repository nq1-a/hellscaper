use poise::serenity_prelude as serenity;

pub struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: serenity::Message) {
        if &msg.content[0..1] == "." {
            println!("starts with period");
        }
    }
}
