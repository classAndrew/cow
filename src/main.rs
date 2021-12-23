mod models;
mod commands;
mod services;

use commands::get_framework;

use std::env;
use models::config::Config;
use services::*;
use std::fs;
use env_logger::Env;
use serde_json;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{channel::Message, gateway::Ready, interactions::Interaction
    }
};
use log::{error, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        message_handler::message(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        bot_init::ready(ctx, ready).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_handler::interaction(ctx, interaction).await;
    }
}

async fn init_logger() -> std::io::Result<()> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    info!("Initializing cow v{}", VERSION.unwrap_or("<unknown>"));
    info!("Reading from {}", env::current_dir()?.display());

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if let Err(ex) = init_logger().await {
        error!("Failed to initialize logger: {}", ex);
    }

    let config_json = fs::read_to_string("config.json").expect("config.json not found");
    let config : Config = serde_json::from_str(&config_json).expect("config.json is malformed");

    let framework = get_framework(&config.cmd_prefix);
    let token = config.token;
    let application_id = config.application_id;

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_id)
        // TODO: Replace with framework_arc, so we can keep a copy of the framework ref to use in interaction_handler
        .framework(framework)
        .await
        .expect("Discord failed to initialize");

    if let Err(ex) = client.start().await {
        error!("Discord bot client error: {:?}", ex);
    }

    Ok(())
}