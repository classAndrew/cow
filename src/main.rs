pub mod models;

use std::env;
use models::config::Config;
use std::fs;
use serde_json;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "bruh").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    print!("Reading from {}", env::current_dir()?.display());
    let config_json = fs::read_to_string("config.json").expect("config.json not found");
    let config : Config = serde_json::from_str(&config_json).expect("config.json is malformed");
    let token = config.token;

    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Discord failed to initialize");

    if let Err(ex) = client.start().await {
        println!("Client error: {:?}", ex);
    }

    Ok(())
}