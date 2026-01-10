use poise::serenity_prelude as serenity;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: serenity::Message) {
        if msg.content[0] == "." {
            println!("balls");
        }
    }
}
