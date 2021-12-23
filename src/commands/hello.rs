use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command
    }
};

#[command]
#[description = "Hello world test command."]
pub async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "world").await?;

    Ok(())
}