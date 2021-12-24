use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command
    }
};
use crate::Database;

#[command]
#[description = "Info about this bot."]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let db = {
        let ctx_global = ctx.data.read().await;
        let out = ctx_global.get::<Database>().expect("Couldn't find database").clone();

        out
    };
    let version = db.get_db_version().await.unwrap();
    let content = format!("I don't think, therefore I do not am.\nVersion: {}", version);
    msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    Ok(())
}