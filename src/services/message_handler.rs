use serenity::{
    client::Context,
    model::channel::Message
};
use log::error;
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
        match db.provide_exp(server_id, msg.author.id).await {
            Err(ex) => {
                error!("Failed providing exp to user: {}", ex)
            },
            Ok(new_level) => {
                if new_level < 0 {
                    return;
                }

                if let Err(ex2) =
                    msg.channel_id.say(&ctx.http, format!("You leveled up to level {}", new_level)).await {
                    error!("Error sending level-up message: {}", ex2);
                }
            }
        }
    }
}