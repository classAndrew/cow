use log::error;
use serenity::{
    framework::standard::{
        macros::command, Args, CommandResult, 
    }, 
    model::channel::Message, client::Context
};

use crate::{Database, db};
use crate::util::{ to_ms, from_ms };

#[command]
#[description = "Sets server-wide cooldown for messaging xp gain."]
#[usage = "<#m#d#s#h> in any order"]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = db!(ctx);
    // nesting part 2
    if let Some(server_id) = msg.guild_id {
        if let Ok(timeout) = args.single::<String>() {
            if let Some(timeout) = to_ms(timeout) {
                match db.set_timeout(server_id, timeout).await {
                    Ok(_) => { msg.reply(&ctx.http, format!("Set timeout to {}.", from_ms(timeout as u64))).await?; }
                    Err(err) => {
                        msg.reply(&ctx.http, "Could not set timeout").await?;
                        error!("Could not set timeout: {}", err);
                    }
                }
            } else {
                msg.reply(&ctx.http, "The timeout must be in the form #s#m#h#d").await?;
            }
        } else {
            msg.reply(&ctx.http, "The timeout must be in the form #s#m#h#d").await?;
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}

#[command]
#[description = "Gets the server-wide cooldown for messaging xp gain."]
#[only_in(guilds)]
pub async fn get(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    if let Some(server_id) = msg.guild_id {
        match db.get_timeout(server_id).await {
            Ok(timeout) => { msg.reply(&ctx.http, format!("The timeout is {}.", from_ms(timeout as u64))).await?; }
            Err(err) => {
                msg.reply(&ctx.http, "Could not set timeout").await?;
                error!("Could not get timeout: {}", err);
            }
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}