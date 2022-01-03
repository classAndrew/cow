use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::{
            command
        }
    }
};
use crate::{Database, db};

#[command]
#[description = "Info about this bot."]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    let version = db.get_db_version().await.unwrap();
    let content = format!("I don't think, therefore I do not am.\nVersion: {}", version);
    msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    Ok(())
}