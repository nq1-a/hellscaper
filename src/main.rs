use std::collections::HashSet;
use std::fs::{
    create_dir_all as create_path,
    read_to_string as read_fs,
    write as write_fs
};
use std::io;
use std::process::exit;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use poise::serenity_prelude as serenity;
use regex::Regex;
use serenity::{
    ChannelId,
    GuildChannel
};
use tokio::time::{Duration, sleep};

mod commands;
mod handler;
mod types;

use commands::level::add_points;
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

// Message cleaning
fn clean_msg(msg: &str) -> String {
    let re = Regex::new(r"<(@|#|t:)\d+(:.)?>").unwrap();
    let res = re.replace_all(msg, "@@@@");

    // FIXME: God help us all
    let re = Regex::new(r"[^\x{0020}-\x{02AF}\x{0370}-\x{070D}\x{0710}-\x{08E1}\x{08E3}-\x{115E}\x{1161}-\x{17FF}\x{1C80}-\x{1FFF}\x{2070}-\x{2DDE}\x{2E00}-\x{D7FB}\x{F900}-\x{FDFD}\x{FE30}-\x{FF9F}\x{FFA1}-\x{FFEE}]|\x{00AD}|\s|\p{M}").unwrap();
    re.replace_all(&res, "").to_string()
}

// Get guild channel
pub async fn get_gch(ctx: &serenity::Context, id: ChannelId) -> Option<GuildChannel> {
    if let Ok(ch) = id.to_channel(ctx).await {
        return ch.guild();
    }

    None
}

// Save loop
pub async fn save_loop(data: &Data) {
    loop {
        sleep(Duration::from_secs(30)).await;
        
        if let Ok(ser) = serde_json::to_string(data) {
            // Get time
            let ctime = SystemTime::now();
            let timestamp = ctime.duration_since(UNIX_EPOCH).unwrap();

            // Show display & save data
            println!("SAVING... ({:?})", timestamp);
            write_fs("state.json", &ser).unwrap();
            write_fs("state~.json", &ser).unwrap();
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
        print!("SUPPLY BOT TOKEN: ");
        token = input(stdin);
        write_fs("token.txt", &token).unwrap();
    }

    // Make sure config.toml exists
    if let Err(_) = read_fs("config.toml") {
        println!("CONFIG FILE NOT FOUND!");
        println!("CREATING NEW CONFIG FILE...");

        write_fs("config.toml", read_fs("config-default.toml").unwrap()).unwrap();

        println!("FILE CREATED // FILL OUT CONFIG FILE AND RERUN PROGRAM");
        exit(1);
    }

    // Create owner set
    let mut owners = HashSet::new();
    owners.insert(serenity::UserId::new(688129525166505999));

    // Create framework
    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::archive::archive(),
                commands::game::game(),
                commands::level::level(),
                commands::party::party(),
                commands::random::random(),
                commands::random::shoot(),
                commands::util::gif(),
                commands::util::github(),
                commands::util::help(),
                commands::util::ping(),
                commands::util::praytoethnicpeter(),
                commands::util::sayraw(),
                commands::util::shutdown(),
            ],
            event_handler: |ctx, ev, _, data| {
                Box::pin(async move {
                    match ev {
                        serenity::FullEvent::Ready {data_about_bot: _} => 'ready: {
                            {
                                let mut ready = data.ready.lock().unwrap();
                                if *ready {break 'ready;}
                                *ready = true;
                            }

                            println!("ACTIVATED");
                            save_loop(&data).await;
                        },
                        serenity::FullEvent::Message {new_message} => 'msg: {
                            if new_message.author.bot || new_message.author.system {break 'msg;}
                            let author: u64 = new_message.author.id.get();

                            // Block to contain mutexes
                            {
                                'pts: {
                                    // Check channel
                                    let channel_desc: String;

                                    if let Some(gch) = get_gch(&ctx, new_message.channel_id).await {
                                        if let Some(pid) = gch.parent_id && let Some(pgch) = get_gch(&ctx, pid).await {
                                            if pgch.topic.unwrap_or_default().contains("<nopts>") {break 'pts;}
                                        }

                                        channel_desc = gch.topic.unwrap_or_default();
                                    } else {break 'pts;}

                                    if channel_desc.contains("<nopts>") {break 'pts;}

                                    // Calculate points
                                    let msg_len: u64 = clean_msg(&new_message.content).len() as u64;
                                    let mut new_pts: u64 = (msg_len / 5u64).max(1).min(20);

                                    let re = Regex::new(r"<(\d+)ptx>").unwrap();
                                    if let Some(caps) = re.captures(&channel_desc) {
                                        new_pts *= caps[1].parse::<u64>().unwrap();
                                    }

                                    // Add points
                                    add_points(&data, author, new_pts);
                                }

                                // Update quicktime messages
                                let mut qt = data.quicktime.lock().unwrap();

                                for (s, v) in qt.iter_mut() {
                                    if new_message.content.to_lowercase() == s.split(":").last().unwrap() {
                                        v.push(author);
                                    }
                                }
                            }
                        },
                        _ => {}
                    }

                    Ok(())
                })
            },
            owners: owners,
            ..Default::default()
        })
        .setup(|ctx, _, fwk| {
            Box::pin(async move {
                
                let mut data: Data = serde_json::from_str::<Data>(
                    &read_fs("state.json").unwrap_or_default()
                ).unwrap_or_default();

                data.ready = Mutex::new(false);
                data.quicktime = Default::default();
                data.load_cfg("config.toml");
                
                for id in serde_json::from_str::<Vec<u64>>(
                    data.config.get(
                        if data.config.get("testing").unwrap() == "true" {"guild_ids_testing"}
                        else {"guild_ids"}
                    ).unwrap()
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
