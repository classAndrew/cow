use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::{
            UserId,
            RoleId,
            GuildId
        },
        guild::Guild
    },
    framework::standard::{
        CommandResult,
        macros::{
            command
        },
        Args
    },
    utils::{
        MessageBuilder
    }
};
use crate::{Database, db};
use log::{error};

async fn is_admin(ctx: &Context, msg: &Message, server_id: &GuildId) -> bool {
    let member = server_id.member(&ctx.http, msg.author.id).await.unwrap();
    // Note: .permissions(&ctx) as a method is used, for *Interactions* use .permissions as a field
    return if let Ok(permissions) = member.permissions(&ctx).await {
        permissions.administrator()
    } else {
        error!("Failed to get permissions? This should never occur.");
        false
    }
}

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
        if !is_admin(ctx, msg, &server_id).await {
            return Ok(());
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

#[command]
#[description = "Configure how ranks are provided."]
// Parameters: rankconfig [min_level] [rank]
pub async fn rankconfig(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);
    // So much nesting...
    if let Some(guild) = msg.guild(&ctx.cache).await {
        if !is_admin(ctx, msg, &guild.id).await {
            return Ok(());
        }

        if args.is_empty() {
            return config_display(ctx, msg, guild, db).await;
        }

        if let Ok(min_level) = args.single::<i32>() {
            let role_id: RoleId;

            if let Ok(role) = args.parse::<RoleId>() {
                role_id = role;
            } else {
                let role_text = args.rest();
                if let Some(role) = guild.role_by_name(role_text) {
                    role_id = role.id;
                } else {
                    let content = MessageBuilder::new().push("Could not find a role on this server matching \"").push_safe(role_text).push("\"!").build();
                    msg.channel_id.say(&ctx.http, content).await?;
                    return Ok(())
                }
            }

            // Both min_level and role_id are initialized by this point


        } else {
            msg.channel_id.say(&ctx.http, "The first argument should be a positive integer, representing the minimum level for this rank.").await?;
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}

async fn config_display(ctx: &Context, msg: &Message, guild: Guild, db: std::sync::Arc<Database>) -> CommandResult {
    match db.get_roles(guild.id).await {
        Ok(items) => {
            if let Err(ex) = msg.channel_id.send_message(&ctx.http, |m| {m.embed(|e| {
                e.title("Rank to Level Mapping")
                    .description(
                         items.into_iter()
                             .map(|i| {
                                 let (name, role, level) = i;
                                 let mut content = format!("{}: <no role> at level {}", name, level);
                                 if let Some(role_id) = role {
                                     content = format!("{}: <@&{}> at level {}", name, role_id, level);
                                 }
                                 content
                             })
                             .reduce(|a, b| {format!("{}\n{}", a, b)})
                             .unwrap()
            )})}).await {
                error!("Failed to send message to server: {}", ex);
            }
        },
        Err(ex) => error!("Failed to get roles for server: {}", ex)
    }

    return Ok(())
}