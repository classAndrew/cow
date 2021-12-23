use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command
    }
};

#[command]
#[description = "Info about this bot."]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {m.content("I don't think, therefore I do not am.")}).await?;
    Ok(())
}