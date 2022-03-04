use log::error;
use serenity::client::Context;
use serenity::model::channel::{Reaction, ReactionConversionError, ReactionType};
use serenity::model::id::{GuildId, MessageId};
use serenity::utils::parse_emoji;
use crate::{Database, db};

pub async fn add_reaction(ctx: &Context, reactions: &Reaction) {
    if reactions.guild_id.is_none() {
        return;
    }
    let guild_id = reactions.guild_id.unwrap();
    let db = db!(ctx);
    match db.get_cowboard_config(guild_id).await {
        Ok(config) => {
            match reactions.message(&ctx.http).await {
                Ok(message) => {
                    match ReactionType::try_from(config.emote) {
                        Ok(config_emote) => {
                            let matched_reaction = message.reactions.into_iter().find(|o| o.reaction_type == config_emote);
                            if let Some(reaction) = matched_reaction {
                                // Pray that the database's constraints work.
                                if reaction.count >= config.add_threshold as u64 {
                                    // Moo that thing!

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

pub fn reaction_remove_all(p0: &Context, p1: Option<GuildId>, p2: MessageId) {
    todo!()
}