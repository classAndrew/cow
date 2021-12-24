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
#[description = "Get your current rank."]
pub async fn rank(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    if let Some(server_id) = msg.guild_id {
        let xp = db.get_exp(server_id, msg.author.id).await.unwrap();
        let content = format!("You have {} xp", xp);
        msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    }
    Ok(())
}