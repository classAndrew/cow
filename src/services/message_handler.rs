use serenity::{
    client::Context,
    model::channel::Message
};
use log::error;
use serenity::model::id::RoleId;
use crate::{Database, db};

pub async fn message(_: &Context, _msg: &Message) {
    // This is basically useless for most cases.
}

pub async fn non_command(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    let db = db!(ctx);

    if let Some(server_id) = msg.guild_id {
        match db.channel_disabled(server_id, msg.channel_id).await {
            Err(ex) => {
                error!("Failed checking if the current channel was disabled: {}", ex);
            },
            Ok(result) => {
                if result {
                    return;
                }
            }
        }

        match db.provide_exp(server_id, msg.author.id).await {
            Err(ex) => {
                error!("Failed providing exp to user: {}", ex)
            },
            Ok((new_level, old_rank, new_rank)) => {
                if new_level < 0 {
                    return;
                }

                let mut content = format!("<@{}> leveled up from {} to {}.", msg.author.id.as_u64(), new_level - 1, new_level);
                if let Some(new_rank_id) = new_rank {
                    content += &*format!("\nYou are now a <@&{}>.", new_rank_id);

                    let mut error = false;
                    let guild = msg.guild(&ctx).await.unwrap();
                    let mut member = guild.member(&ctx.http, msg.author.id).await.unwrap();

                    if let Some(old_rank_id) = old_rank {
                        let old_rank = RoleId::from(old_rank_id);
                        if member.roles.contains(&old_rank) {
                            // We know we're in a guild, so an error is probably an API issue.
                            if let Err(ex) = member.remove_role(&ctx.http, old_rank).await {
                                error = true;
                                content += "\n(We failed to update your roles; maybe we don't have permission?)";
                                error!("Failed to remove role from user: {}", ex);
                            }
                        }
                    }

                    if let Err(ex) = member.add_role(&ctx.http, RoleId::from(new_rank_id)).await {
                        if !error {
                            content += "\n(We failed to update your roles; maybe we don't have permission?)";
                        }
                        error!("Failed to add role to user: {}", ex);
                    }
                }

                if let Err(ex2) =
                    msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| e
                        .title("Level Up!")
                        .description(content)
                    )).await {
                        error!("Error sending level-up message: {}", ex2)
                };
            }
        }
    }
}