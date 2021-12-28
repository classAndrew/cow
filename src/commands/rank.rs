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
use log::{error};
use serenity::framework::standard::CommandError;

#[command]
#[description = "Get your current rank."]
pub async fn rank(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);
    let other = args.single::<UserId>();
    if let Some(server_id) = msg.guild_id {
        let content: String;

        if let Ok(other_id) = other {
            let (xp, level) = db.get_xp(server_id, other_id).await.unwrap();
            if let Ok(other_user) = other_id.to_user(&ctx.http).await {
                content = format!("{} has {} xp, now at level {}", other_user.name, xp, level);
            } else {
                content = format!("Could not find user...");
            }
        } else {
            let (xp, level) = db.get_xp(server_id, msg.author.id).await.unwrap();
            content = format!("You have {} xp, now at level {}", xp, level);
        }
        msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}

#[command]
#[description = "Disable/enable experience from being collected in the current channel."]
pub async fn disablexp(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    if let Some(server_id) = msg.guild_id {
        let member = server_id.member(&ctx.http, msg.author.id).await.unwrap();
        // Note: .permissions(&ctx) as a method is used, for *Interactions* use .permissions as a field
        if let Ok(permissions) = member.permissions(&ctx).await {
            if !permissions.manage_channels() {
                // No permissions, ignore.
                return Ok(());
            }
        } else {
            error!("Failed to get permissions? This should never occur.");
            return Err(CommandError::from("Failed to get permissions"));
        }

        let mut content: String;
        match db.toggle_channel_xp(server_id, msg.channel_id).await {
            Ok(toggle) => {
                if toggle {
                    content = format!("Disabled");
                } else {
                    content = format!("Enabled");
                }
                content += &*format!(" collecting experience in <#{}>.", msg.channel_id.as_u64());
            },
            Err(ex) => {
                content = format!("Failed to toggle channel xp status.");
                error!("Failed to toggle channel xp status: {}", ex);
            }
        }

        msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}

#[command]
#[description = "Get the current rankings in the server."]
pub async fn levels(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    if let Some(server_id) = msg.guild_id {
        let content: String;
        match db.top_members(server_id).await {
            Ok(users) => {
               content = users.into_iter()
                   // Too lazy to check docs if map on vectors is in order, or too lazy.
                   // Honestly it's probably not guaranteed, feel free to change
                   .map(|u| {
                       let (id, level, xp) = u;
                       format!("<@{}> - Level {}, {} xp", id, level, xp)
                   })
                   .reduce(|a, b| {format!("{}\n{}", a, b)})
                   .unwrap();
            },
            Err(ex) => {
                content = format!("Failed to toggle channel xp status.");
                error!("Failed to toggle channel xp status: {}", ex);
            }
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e|
                e.title("Top Users")
                    .description(content)
            )}).await?;
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}
