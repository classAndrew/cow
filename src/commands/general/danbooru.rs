use std::path::Path;
use log::error;
// Fun with stupid APIs!
use serenity::client::Context;
use serenity::framework::standard::{CommandResult};
use serenity::model::channel::{Message};
use serenity::framework::standard::macros::{command};
use tokio::fs;
use crate::Config;
use serde::Deserialize;
use rand::Rng;
use serenity::http::AttachmentType;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
struct Post {
    // 's', 'q', 'e' (safe, questionable, explicit)
    pub rating: String,
    // Bytes.
    pub file_size: u64,
    // MD5 hash.
    pub md5: String,
    // Features of the image
    pub tag_string_general: String,
    pub tag_string_character: String,
    pub tag_string_copyright: String,
    pub tag_string_artist: String,
    pub tag_string_meta: String,
    pub file_url: String,
    pub large_file_url: String,
    pub preview_file_url: String
}

#[command]
#[bucket = "danbooru"]
async fn reimu(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = fetch_by_tag(ctx, msg, "hakurei_reimu").await;

    Ok(())
}

#[command]
#[bucket = "danbooru"]
async fn momiji(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = fetch_by_tag(ctx, msg, "inubashiri_momiji").await;

    Ok(())
}

async fn fetch_by_tag(ctx: &Context, msg: &Message, tag: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let config_json = fs::read_to_string("config.json").await?;
    let config : Config = serde_json::from_str(&config_json).expect("config.json is malformed");

    let url = if let Some(channel) = msg.channel(&ctx).await {
        if channel.is_nsfw() {
            // I'm not even going to test this.
            format!("https://danbooru.donmai.us/posts.json?tags={}&limit=200", tag)
        } else {
            format!("https://danbooru.donmai.us/posts.json?tags=rating:s+{}&limit=200", tag)
        }
    } else {
        format!("https://danbooru.donmai.us/posts.json?tags=rating:s+{}&limit=200", tag)
    };

    let data = client
        .get(url)
        .basic_auth(config.danbooru_login, Some(config.danbooru_api_key))
        .send()
        .await?;

    match data.json::<Vec<Post>>().await {
        Ok(data) => {
            let mut index = rand::thread_rng().gen_range(0..data.len());
            let mut post = data.get(index).unwrap();
            while post.file_size >= 8 * 1024 * 1024 {
                index = rand::thread_rng().gen_range(0..data.len());
                post = data.get(index).unwrap();
            }
            let last_index = post.file_url.rfind('/').unwrap() + 1;
            let file_name = &post.file_url[last_index..];
            let bytes = client.get(&post.file_url).send().await?.bytes().await?;
            let mut file = fs::File::create(file_name).await?;
            file.write_all(&*bytes).await?;

            let _ = msg.channel_id.send_message(&ctx.http, |m|
                {
                    let execution = m
                        .embed(|e| {
                            e.description(format!("Author: {}", &post.tag_string_artist))
                            .attachment(file_name);

                            e
                        });

                    execution.add_file(AttachmentType::Path(Path::new(file_name)));

                    execution
                }
            ).await;

            fs::remove_file(file_name).await?;
        },
        Err(ex) => {
            error!("Failed to fetch data from Danbooru: {}", ex);
            msg.channel_id.say(&ctx.http, "Failed to send message").await?;
        }
    }

    Ok(())
}