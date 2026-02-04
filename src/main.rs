use std::collections::HashMap;
use std::fs::{read_to_string as read_fs, write as write_fs};
use std::io;
use std::sync::Mutex;

use poise::serenity_prelude as serenity;
use tokio;

mod commands;
mod handler;
mod types;

use types::arc::Campaign;

pub struct Data {
    pub campaigns: Mutex<HashMap<String, Campaign>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

fn input(stdin: io::Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

#[tokio::main]
async fn main() {
    // Set up stdin
    let stdin = io::stdin();

    // Get token
    let token: String;

    if let Ok(t) = read_fs("token.txt") {
        token = t;
    } else {
        print!("No bot token found // You must supply one yourself: ");
        token = input(stdin);
        write_fs("token.txt", &token).unwrap();
    }

    // Create framework
    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::party::partyjoin(),
                commands::party::partynew(),
                commands::random::flip(),
                commands::random::roll(),
                commands::random::shoot(),
                commands::util::ping(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _, fwk| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &fwk.options().commands, serenity::GuildId::new(1241868193014743070)).await?;
                Ok(Data {
                    campaigns: Mutex::new(HashMap::new()),
                })
            })
        })
        .build();

    // Create client
    let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::default() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .framework(framework)
        .event_handler(handler::Handler)
        .await;

    // Start client
    if let Err(reason) = client.unwrap().start().await {
        println!("{reason:?}");
    }
}
