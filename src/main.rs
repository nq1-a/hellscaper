use std::fs::{read_to_string as read_fs, write as write_fs};

use serenity::async_trait;
use serenity::prelude::*;
use tokio;

#[tokio::main]
async fn main() {
    if let Ok(data) = read_fs("token.txt") {
        // TODO: make bot work
    } else {
        write_fs("token.txt", "").unwrap();
    }
}
