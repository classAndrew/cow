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

#[command]
#[description = "Configure how ranks are provided."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
// Parameters: rankconfig [min_level] [rank]
pub async fn rankconfig(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);
    // So much nesting...
    if let Some(guild) = msg.guild(&ctx.cache).await {
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

#[command]
#[description = "Configure how ranks are provided."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn config_display(ctx: &Context, msg: &Message) -> CommandResult {
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