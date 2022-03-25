use log::error;
use regex::Regex;
use serenity::client::Context;
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command};
use serenity::utils::MessageBuilder;
use crate::Lavalink;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "`help, join, leave, play, now_playing, skip`").await?;

    Ok(())
}

async fn join_interactive(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(&ctx.http, "Join a voice channel first.").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await.unwrap().clone();

    let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

    match handler {
        Ok(connection_info) => {
            let data = ctx.data.read().await;
            let lava_client = data.get::<Lavalink>().unwrap().clone();
            lava_client.create_session(&connection_info).await?;
            msg.channel_id.say(&ctx.http, format!("Joined <#{}>", connect_to)).await?;
        }
        Err(ex) => {
            msg.channel_id.say(&ctx.http, "Failed to join your VC...").await?;
            error!("Error joining the channel: {}", ex)
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    join_interactive(ctx, msg).await
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.unwrap().clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(ex) = manager.remove(guild_id).await {
            error!("Failed to disconnect: {}", ex);
        }

        {
            // Free up the LavaLink client.
            let data = ctx.data.read().await;
            let lava_client = data.get::<Lavalink>().unwrap().clone();
            lava_client.destroy(guild_id).await?;
        }

        msg.channel_id.say(&ctx.http, "Disconnected from VC. Goodbye!").await?;
    } else {
        msg.channel_id.say(&ctx.http, "I'm not in a VC.").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if args.is_empty() {
        msg.channel_id.say(&ctx.http,"Please enter a query or link.").await?;
    }

    let query = args.message().to_string();

    let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            
            msg.channel_id.say(&ctx.http, "Error finding channel info").await?;

            return Ok(());
        }
    };

    let lava_client = {
        let data = ctx.data.read().await;
        data.get::<Lavalink>().unwrap().clone()
    };

    let manager = songbird::get(ctx).await.unwrap().clone();

    if manager.get(guild_id).is_none() {
        if let Err(ex) = join_interactive(ctx, msg).await {
            msg.channel_id.say(&ctx.http, "Failed to connect to voice channel; maybe I don't have permissions?").await?;
            error!("Failed to connect to vc: {}", ex);
            return Ok(());
        }
    }

    if let Some(_handler) = manager.get(guild_id) {
        
        let query_information = lava_client.auto_search_tracks(&query).await?;

        if query_information.tracks.is_empty() {
            msg.channel_id
                .say(&ctx, "Could not find any video of the search query.")
                .await?;
            return Ok(());
        }

        if let Err(why) = &lava_client.play(guild_id, query_information.tracks[0].clone()).queue()
            .await
        {
            error!("Failed to queue: {}", why);
            return Ok(());
        };

        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "Added to queue: {}",
                    query_information.tracks[0].info.as_ref().unwrap().title
                ),
            )
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases(np, nowplaying)]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().clone();
    let guild_id = msg.guild_id.unwrap();

    if let Some(node) = lava_client.nodes().await.get(&guild_id.0) {
        if let Some(track) = &node.now_playing {
            let info = track.track.info.as_ref().unwrap();
            let re = Regex::new(r#"(?:youtube\.com/(?:[^/]+/.+/|(?:v|e(?:mbed)?)/|.*[?&]v=)|youtu\.be/)([^"&?/\s]{11})"#).unwrap();
            let caps = re.captures(&*info.uri).unwrap();
            let id = caps.get(1).map(|m| m.as_str());
            let server_name = guild_id.name(&ctx).await;

            msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
                 e
                    .author(|a| a.name(match server_name {
                        Some(name) => format!("Now Playing in {}", name),
                        None => "Now Playing".to_string()
                    }))
                    .title(&info.title)
                    .url(&info.uri)
                    .field("Artist", &info.author, true)
                    .field("Duration", format!("{}/{}", track.start_time, track.end_time.map_or_else(|| "<unknown>".to_string(), |o| format!("{}", o))), true)
                    .field("Requested By", match track.requester {
                        Some(id) => format!("<@{}>", id),
                        None => "Unknown".to_string()
                    }, true);

                if let Some(yt_id) = id {
                    e.thumbnail(format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", yt_id));
                }

                e
            }
            )).await?;
        } else {
            msg.channel_id.say(&ctx.http, "Nothing is playing at the moment.").await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "Nothing is playing at the moment.").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().clone();

    if let Some(track) = lava_client.skip(msg.guild_id.unwrap()).await {
        msg.channel_id.say(&ctx.http, MessageBuilder::new().push("Skipped: ").push_mono_line_safe(&track.track.info.as_ref().unwrap().title)).await?;
    } else if let Some(node) = lava_client.nodes().await.get(&msg.guild_id.unwrap().0) {
        if node.now_playing.is_some() {
            if let Err(ex) = lava_client.stop(msg.guild_id.unwrap()).await {
                error!("Failed to stop music: {}", ex);
            }
            msg.channel_id.say(&ctx.http, "Stopped the player, since this was the last song.").await?;
        } else {
            msg.channel_id.say(&ctx.http, "There is nothing to skip.").await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "There is nothing to skip.").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().clone();

    if let Some(node) = lava_client.nodes().await.get(&msg.guild_id.unwrap().0) {
        let queue = &node.queue;
        for track in queue {

        }
    } else {
        msg.channel_id.say(&ctx.http, "Nothing is playing at the moment.").await?;
    }

    Ok(())
}