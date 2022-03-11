use serenity::{
    framework::standard::{
        macros::command, Args, CommandResult,
    },
    model::channel::Message, client::Context
};
use serenity::model::channel::ReactionType;
use serenity::utils::MessageBuilder;
use crate::{Database, db};

#[command]
#[description = "Get the current settings for the cowboard."]
#[only_in(guilds)]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);

    if let Some(guild_id) = msg.guild_id {
        if let Ok(config) = db.get_cowboard_config(guild_id).await {
            msg.channel_id.send_message(&ctx.http, |m| m.embed(|e|
                e
                    .title("Cowboard Settings")
                    .description("If the emote doesn't display below, you probably want to use a different one!")
                    .field("Emote", &config.emote, true)
                    .field("Raw Emote", MessageBuilder::new().push_mono(&config.emote).build(), true)
                    .field("Channel", config.channel.map(|o| format!("<#{}>", o)).unwrap_or_else(|| "No Cowboard Channel".to_string()), true)
                    .field("Webhook", if config.webhook_id.is_some() && config.webhook_token.is_some() { "Enabled" } else { "Disabled" }, true)
            )).await?;
        } else {
            msg.channel_id.say(&ctx.http, "Failed to fetch Cowboard settings for this server...").await?;
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}

#[command]
#[description = "Set"]
#[usage = "An emote, preferably one on the server or a default Discord emoji."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn emote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);

    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "You need to pass an emote to this command, like :cow:.").await?;
        return Ok(());
    }

    if let Ok(emoji) = args.single::<ReactionType>() {
        if let Some(guild_id) = msg.guild_id {
            match db.get_cowboard_config(guild_id).await {
                Ok(config) => {}
                Err(ex) => {
                    msg.reply(&ctx.http, "Failed to get").await?;
                }
            }
        } else {
            msg.reply(&ctx.http, "This command can only be run in a server.").await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "Failed to process an emote from the given message...").await?;
        return Ok(());
    }

    Ok(())
}

#[command]
#[description = "Sets server-wide cooldown for messaging xp gain."]
#[usage = "<#m#d#s#h> in any order"]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn min(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);

    Ok(())
}

#[command]
#[description = "Sets server-wide cooldown for messaging xp gain."]
#[usage = "<#m#d#s#h> in any order"]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);



    Ok(())
}

#[command]
#[description = "Toggle webhook usage for the cowboard, versus the bot sending the messages."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn webhook(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);

    if let Some(guild) = msg.guild(ctx).await {
        let current_user = ctx.cache.current_user().await;
        if let Some(channel) = msg.channel(ctx).await {
            if let Some(guild_channel) = channel.guild() {
                if let Ok(perm) = guild_channel.permissions_for_user(ctx, current_user.id).await {
                    if !perm.manage_webhooks() {
                        msg.channel_id.say(&ctx.http, "Can't toggle webhooks because I don't have permissions to add webhooks to ").await?;
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}