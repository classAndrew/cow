use std::error::Error;
use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::{
            RoleId
        }
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

// Parameters: rankconfig add [min_level] [rank]

#[command]
#[description = "Add a rank to the configuration."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);
    // So much nesting...
    if let Some(guild) = msg.guild(&ctx.cache).await {
        if let Ok(min_level) = args.single::<i32>() {
            let role_id: RoleId;
            let mut role_text: String;

            if let Ok(role) = args.parse::<RoleId>() {
                role_id = role;
                if let Some(role) = guild.roles.get(&role) {
                    role_text = role.name.clone();
                } else {
                    msg.channel_id.say(&ctx.http, format!("Could not find a role on this server matching <@&{}>!", role_id.as_u64())).await?;
                    return Ok(())
                }
            } else {
                role_text = args.rest().to_string();
                if let Some(role) = guild.role_by_name(&*role_text) {
                    role_id = role.id;
                    role_text = role.name.clone(); // Just to make it exact.
                } else {
                    let content = MessageBuilder::new().push("Could not find a role on this server matching \"").push_safe(role_text).push("\"!").build();
                    msg.channel_id.say(&ctx.http, content).await?;
                    return Ok(())
                }
            }

            // Both min_level and role_id are initialized by this point
            match db.add_role(msg.guild_id.unwrap(), &role_text, role_id, min_level).await {
                Ok(success) => {
                    if success {
                        msg.channel_id.say(&ctx.http, format!("Successfully added <@&{}> with minimum level {}.", role_id.as_u64(), min_level)).await?;
                    } else {
                        msg.channel_id.say(&ctx.http, format!("There is a duplicate role with minimum level {}.", min_level)).await?;
                    }
                }
                Err(ex) => {
                    error!("Failed to add role for server: {}", ex);
                    msg.channel_id.say(&ctx.http, "Failed to add role to the server.").await?;
                }
            }
        } else {
            msg.channel_id.say(&ctx.http, "The first argument should be a positive integer, representing the minimum level for this rank.").await?;
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}

#[command]
#[description = "List the current ranks on this server."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    if let Some(guild_id) = msg.guild_id {
        match db.get_roles(guild_id).await {
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
                                .unwrap_or(String::from("No roles are registered on this server."))
                        )})}).await {
                    error!("Failed to send message to server: {}", ex);
                }
            },
            Err(ex) => error!("Failed to get roles for server: {}", ex)
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}