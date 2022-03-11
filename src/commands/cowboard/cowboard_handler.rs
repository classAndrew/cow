use chrono::Utc;
use log::error;
use serenity::client::Context;
use serenity::model::channel::{Embed, Message, Reaction, ReactionType};
use serenity::model::id::{ChannelId, GuildId, MessageId};
use crate::{Database, db};
use crate::commands::cowboard::cowboard_db_models::{Cowboard};

pub async fn add_reaction(ctx: &Context, added_reaction: &Reaction) {
    if added_reaction.guild_id.is_none() {
        return;
    }
    let guild_id = added_reaction.guild_id.unwrap();
    let db = db!(ctx);
    match db.get_cowboard_config(guild_id).await {
        Ok(mut config) => {
            if config.channel.is_none() {
                // No cowboard, why even check?
                return;
            }

            match added_reaction.message(&ctx.http).await {
                Ok(message) => {
                    match ReactionType::try_from(config.emote.as_str()) {
                        Ok(config_emote) => {
                            let matched_reaction = message.reactions.iter().find(|o| o.reaction_type == config_emote);
                            if let Some(reaction) = matched_reaction {
                                // Pray that the database's constraints work.
                                if reaction.count >= config.add_threshold as u64 {
                                    // Moo that thing!
                                    add_moo(ctx, guild_id, added_reaction, &message, &mut config).await;
                                }
                            }
                        }
                        Err(ex) => {
                            error!("Failed to parse emoji from database: {}", ex);
                        }
                    }
                }
                Err(ex) => {
                    error!("Failed to get reacted message: {}", ex);
                }
            }
        }
        Err(ex) => {
            error!("Failed to get cowboard config: {}", ex);
        }
    }
}

async fn add_moo(ctx: &Context, guild_id: GuildId, reaction: &Reaction, message: &Message, config: &mut Cowboard) {
    let db = db!(ctx);

    let message_result = if config.webhook_id.is_some() && config.webhook_token.is_some() {
        send_webhook_message(ctx, message, config).await
    } else {
        send_bot_message(ctx, message, config).await
    };

    if let Err(ex) = message_result {
        error!("Failed to send cowboard message: {}", ex);
        return;
    }

    let post_message = message_result.unwrap();

    // Check again since it might have changed

    if config.webhook_id.is_some() && config.webhook_token.is_some() {
        update_webhook_message(ctx, message, &post_message, reaction, config).await;
    } else {
        update_bot_message(ctx, message, &post_message, reaction, config).await;
    }

    if let Err(ex) = db.moo_message(message.id, reaction.channel_id, post_message.id, post_message.channel_id, guild_id).await {
        error!("Failed to moo a message in the database: {}", ex);
    }
}

async fn send_bot_message(ctx: &Context, message: &Message, config: &Cowboard) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
    let channel = ChannelId::from(config.channel.unwrap());
    Ok(channel.say(&ctx.http, "Loading, please wait warmly...").await?)
}

async fn update_bot_message(ctx: &Context, message: &Message, post_message: &Message, reaction: &Reaction, config: &mut Cowboard) {

}

async fn send_webhook_message(ctx: &Context, message: &Message, config: &mut Cowboard) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
    let token = config.webhook_token.clone().unwrap();
    if let Ok(webhook) = ctx.http.get_webhook_with_token(config.webhook_id.unwrap(), &*token).await {
        let output_username = format_username(ctx, message).await;
        if let Ok(Some(webhook_message)) = webhook.execute(&ctx.http, true, |m|
            m
                .content("Loading, please wait warmly...")
                .avatar_url(message.author.face())
                .username(output_username)
        ).await {
            return Ok(webhook_message);
        }
    }

    disable_webhook(ctx, config).await;
    send_bot_message(ctx, message, config).await
}

async fn format_username(ctx: &Context, message: &Message) -> String {
    let username = format!("{}#{}", message.author.name, message.author.discriminator);
    let nickname = message.author_nick(&ctx.http).await;

    return if let Some(nick) = nickname {
        format!("{} ({})", nick, username)
    } else {
        username
    };
}

async fn update_webhook_message(ctx: &Context, message: &Message, post_message: &Message, reaction: &Reaction, config: &mut Cowboard) {
    let token = config.webhook_token.clone().unwrap();
    if let Ok(webhook) = ctx.http.get_webhook_with_token(config.webhook_id.unwrap(), &*token).await {
        let output_username = format_username(ctx, message).await;
        let safe_content = message.content_safe(ctx).await;

        let mut embeds = vec![
            Embed::fake(|e|
                {
                    let temp = e
                        .author(|a|
                            a.name(&output_username).icon_url(message.author.face()))
                        .description(&safe_content)
                        .timestamp(Utc::now())
                        .footer(|f| f.text(format!("Message ID: {} / User ID: {}", message.id, message.author.id)));

                    if !message.attachments.is_empty() {
                        temp.image(message.attachments.get(0).unwrap().url.clone());
                    }

                    temp
                }
            )
        ];

        for i in 1..message.attachments.len() {
            let attachment = message.attachments.get(i).unwrap();

            // If it has a width (or height), it's probably an image. Or a video but not my problem.
            if attachment.width.is_some() {
                embeds.push(Embed::fake(|e|
                    e
                        .author(|a|
                            a.name(&output_username).icon_url(message.author.face()))
                        .timestamp(Utc::now())
                        .footer(|f| f.text(format!("Message ID: {} / User ID: {} / Attachment ID: {}", message.id, message.author.id, attachment.id)))
                        .image(attachment.url.clone())
                ))
            }
        }

        if let Err(ex) = webhook.edit_message(&ctx.http, message.id,
         |m| m.embeds(embeds)).await {
            error!("Failed to edit post message??? {}", ex);
        }
    } else {
        disable_webhook(ctx, config).await;
        update_bot_message(ctx, message, post_message, reaction, config).await;
    }
}

async fn disable_webhook(ctx: &Context, config: &mut Cowboard) {
    let db = db!(ctx);

    config.webhook_id = None;
    config.webhook_token = None;
    if let Err(ex) = db.update_cowboard(config).await {
        error!("Failed to update cowboard settings: {}", ex);
    }
}

pub async fn remove_reaction(ctx: &Context, removed_reaction: &Reaction) {
    if removed_reaction.guild_id.is_none() {
        return;
    }

    let guild_id = removed_reaction.guild_id.unwrap();
    let db = db!(ctx);
    match db.get_cowboard_config(guild_id).await {
        Ok(config) => {
            match removed_reaction.message(&ctx.http).await {
                Ok(message) => {
                    match ReactionType::try_from(config.emote) {
                        Ok(config_emote) => {
                            let matched_reaction = message.reactions.into_iter().find(|o| o.reaction_type == config_emote);
                            if let Some(reaction) = matched_reaction {
                                // Pray that the database's constraints work.
                                if reaction.count < config.remove_threshold as u64 {
                                    // Unmoo that thing!
                                    remove_moo(ctx, guild_id, removed_reaction.channel_id, removed_reaction.message_id).await;
                                }
                            }
                        }
                        Err(ex) => {
                            error!("Failed to parse emoji from database: {}", ex);
                        }
                    }
                }
                Err(ex) => {
                    error!("Failed to get reacted message: {}", ex);
                }
            }
        }
        Err(ex) => {
            error!("Failed to get cowboard config: {}", ex);
        }
    }
}

pub async fn reaction_remove_all(ctx: &Context, channel_id: ChannelId, message: MessageId) {
    let guild_id = channel_id.message(&ctx.http, message).await.ok().and_then(|o| o.guild_id);
    if let Some(guild) = guild_id {
        remove_moo(ctx, guild, channel_id, message).await;
    }
}

async fn remove_moo(ctx: &Context, guild_id: GuildId, channel_id: ChannelId, message: MessageId) {
    let db = db!(ctx);

    match db.get_cowboard_message(message, channel_id, guild_id).await {
        Ok(message_info) => {
            if let Some(cowboard_message) = message_info {
                let cowboard_channel = ChannelId::from(cowboard_message.post_channel_id);
                match cowboard_channel.message(&ctx.http, cowboard_message.post_id).await {
                    Ok(discord_cowboard_message) => {
                        if let Err(ex) = discord_cowboard_message.delete(&ctx.http).await {
                            error!("Failed to delete message: {}", ex);
                        }
                    }
                    Err(ex) => {
                        error!("Failed to get message in Discord: {}", ex);
                    }
                }
            }
        }
        Err(ex) => {
            error!("Failed to query cowboard message: {}", ex);
        }
    }

    if let Err(ex) = db.unmoo_message(message, channel_id, guild_id).await {
        error!("Failed to unmoo a message in the database: {}", ex);
    }
}