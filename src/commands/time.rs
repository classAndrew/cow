use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command
    }
};

use std::time::SystemTime;

#[command]
#[description = "Time command (to demonstrate how to load two commands)"]
pub async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    let ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time")
        .as_millis();

    let response = format!("{} ms since epoch", ms); 
    msg.reply(ctx, response).await?;

    Ok(())
}