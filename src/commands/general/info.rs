use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::{
            command
        }
    }
};

#[command]
#[description = "Info about this bot."]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    let content = format!("Cow v{} - A Discord bot written by HelloAndrew and DoggySazHi", VERSION.unwrap_or("<unknown>"));
    msg.channel_id.send_message(&ctx.http, |m| {m.content(content)}).await?;
    Ok(())
}