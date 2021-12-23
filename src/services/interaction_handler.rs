use serenity::{
    client::Context,
    model::interactions::{
        Interaction,
        InteractionResponseType
    }
};
use log::error;
use serenity::framework::Framework;
use crate::FrameworkContainer;
use chrono::{Utc};
use serenity::utils::CustomMessage;

pub async fn interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        let app_id = command.application_id.as_u64();
        let cmd_name = command.data.name.as_str();
        let content = format!("<@!{}> {}", app_id, cmd_name);

        let data_read = ctx.data.read().await;
        let fw = data_read.get::<FrameworkContainer>().expect("Couldn't find framework");

        let mut dummy_message = CustomMessage::new();

        dummy_message.channel_id(command.channel_id)
            .content(content)
            .author(command.user.clone())
            .timestamp(Utc::now());

        if let Some(guild_id) = command.guild_id {
            dummy_message.guild_id(guild_id);
        }

        (*fw).dispatch(ctx.clone(), dummy_message.build()).await;

        if let Err(ex) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::UpdateMessage)
            })
            .await
        {
            error!("Failed to respond to slash command: {}", ex);
        }
    }
}