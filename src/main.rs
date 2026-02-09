use std::fs::{
    create_dir_all as create_path,
    read_to_string as read_fs,
    write as write_fs
};
use std::io;
use std::sync::Mutex;

use poise::serenity_prelude as serenity;
use tokio::time::{Duration, sleep};

mod commands;
mod handler;
mod types;

use types::data::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Function to handle stdin
fn input(stdin: io::Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

// Create directory if it does not exist
fn make_dir(path: &str) {
    if let Err(e) = create_path(path) {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {},
            _ => panic!("{:?}", e)
        }
    }
}

// Save loop
pub async fn save_loop(data: &Data) {
    loop {
        sleep(Duration::from_secs(300)).await;
        
        if let Ok(ser) = serde_json::to_string(data) {
            println!("SAVING...");
            write_fs("state.json", ser).unwrap();
        }
    }
}

// Main loop
#[tokio::main]
async fn main() {
    // Set up stdin
    let stdin = io::stdin();

    // Create directories if they do not exist
    make_dir("archive");

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
                commands::archive::archive(),
                commands::party::party(),
                commands::random::flip(),
                commands::random::roll(),
                commands::random::shoot(),
                commands::util::github(),
                commands::util::ping(),
                commands::util::shutdown(),
            ],
            event_handler: |_, ev, _, data| {
                Box::pin(async move {
                    match ev {
                        serenity::FullEvent::Ready {data_about_bot: _} => 'ready: {
                            {
                                let mut ready = data.ready.lock().unwrap();
                                if *ready {break 'ready ();}
                                *ready = true;
                            }

                            println!("ACTIVATED");
                            save_loop(&data).await;
                        },
                        _ => {}
                    }

                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _, fwk| {
            Box::pin(async move {
                
                let mut data: Data = serde_json::from_str::<Data>(
                    &read_fs("state.json").unwrap_or_default()
                ).unwrap_or_default();

                data.ready = Mutex::new(false);
                data.load_cfg("config.toml");
                
                for id in serde_json::from_str::<Vec<u64>>(
                    data.config.get("guild_ids").unwrap()
                ).unwrap_or_default() {
                    println!("{} ONLINE", id);
                    poise::builtins::register_in_guild(
                        ctx,
                        &fwk.options().commands,
                        serenity::GuildId::new(id)
                    ).await?;
                }

                Ok(data)
            })
        })
        .build();

    // Create client
    let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::default()
            | serenity::GatewayIntents::GUILD_MESSAGE_POLLS
            | serenity::GatewayIntents::MESSAGE_CONTENT)
        .framework(framework)
        .event_handler(handler::Handler)
        .await;

    // Start client
    if let Err(reason) = client.unwrap().start().await {
        println!("{reason:?}");
    }
}
