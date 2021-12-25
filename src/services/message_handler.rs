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
        if let Err(ex) = db.provide_exp(server_id, msg.author.id).await {
            error!("Failed providing exp to user: {}", ex)
        }
    }
}