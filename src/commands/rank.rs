use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::UserId
    },
    framework::standard::{
        CommandResult,
        macros::{
            command
        },
        Args
    }
};
use crate::{Database, db};

#[command]
#[description = "Get your current rank."]
pub async fn rank(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);
    let other = args.single::<UserId>();
    if let Some(server_id) = msg.guild_id {
        let content: String;

        if let Ok(other_id) = other {
            let (xp, level) = db.get_exp(server_id, other_id).await.unwrap();
            if let Ok(other_user) = other_id.to_user(&ctx.http).await {
                content = format!("{} has {} xp, now at level {}", other_user.name, xp, level);
            } else {
                content = format!("Could not find user...");
            }
        } else {
            let (xp, level) = db.get_exp(server_id, msg.author.id).await.unwrap();
            content = format!("You have {} xp, now at level {}", xp, level);
        }
        msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}