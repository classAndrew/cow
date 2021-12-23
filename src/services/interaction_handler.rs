use serenity::{
    client::Context,
    model::interactions::{
        Interaction,
        InteractionResponseType
    }
};
use log::{error};
use serenity::http::CacheHttp;

pub async fn interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        let content = match command.data.name.as_str() {
            _ => format!("Received interaction for {}", command.data.name),
        };

        if let Err(ex) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
        {
            error!("Failed to respond to slash command: {}", ex);
        }
    }
}