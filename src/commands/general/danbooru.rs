use std::convert::Infallible;
use log::error;
// Fun with stupid APIs!
use serenity::client::Context;
use serenity::framework::standard::{ArgError, Args, CommandResult};
use serenity::model::channel::{Message};
use serenity::framework::standard::macros::{command};
use tokio::fs;
use crate::Config;
use serde::Deserialize;
use regex::Regex;
use serenity::utils::MessageBuilder;

#[derive(Debug, Deserialize)]
struct Post {
    // Bytes.
    pub file_size: Option<u64>,
    // Features of the image
    pub tag_string_general: Option<String>,
    pub tag_string_character: Option<String>,
    pub tag_string_artist: Option<String>,
    pub file_url: Option<String>
}

#[command]
#[bucket = "danbooru"]
async fn reimu(ctx: &Context, msg: &Message) -> CommandResult {
    fetch_by_tag(ctx, msg, "hakurei_reimu").await
}

#[command]
#[bucket = "danbooru"]
async fn momiji(ctx: &Context, msg: &Message) -> CommandResult {
    fetch_by_tag(ctx, msg, "inubashiri_momiji").await
}

#[command]
#[bucket = "danbooru"]
async fn sanae(ctx: &Context, msg: &Message) -> CommandResult {
    fetch_by_tag(ctx, msg, "kochiya_sanae").await
}

#[command]
#[bucket = "danbooru"]
async fn danbooru(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let non_tag = Regex::new(r"[^A-Za-z0-9()_.]").unwrap();
    let tag_option = args
        .iter()
        .map(|o: Result<String, ArgError<Infallible>>| o.unwrap().trim().to_lowercase())
        .map(|o| non_tag.replace_all(&*o, "").to_string())
        .reduce(|a, b| format!("{}_{}", a, b));

    if let Some(tag) = tag_option {
        return fetch_by_tag(ctx, msg, &tag).await;
    } else {
        msg.channel_id.say(&ctx.http, "You need to pass a valid Danbooru tag to search for.").await?;
    }

    Ok(())
}

fn is_nice_post(post: &Post) -> bool {
    if post.tag_string_general.is_none() || post.file_url.is_none() || post.file_size.is_none() || post.tag_string_character.is_none() || post.tag_string_artist.is_none() {
        return false;
    }

    let is_comic = post.tag_string_general.clone().unwrap().split(' ').any(|o| o == "comic");
    let character_count = post.tag_string_character.clone().unwrap().split(' ').count();

    post.file_size.unwrap() <= 8 * 1024 * 1024 &&
        character_count <= 3 &&
        !is_comic
}

async fn fetch_by_tag(ctx: &Context, msg: &Message, tag: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let config_json = fs::read_to_string("config.json").await?;
    let config : Config = serde_json::from_str(&config_json).expect("config.json is malformed");

    let url = if let Some(channel) = msg.channel(&ctx).await {
        if channel.is_nsfw() {
            // Unfortunately, someone else tested this for me. It works.
            format!("https://danbooru.donmai.us/posts/random.json?tags={}", tag)
        } else {
            format!("https://danbooru.donmai.us/posts/random.json?tags=rating:s+{}", tag)
        }
    } else {
        format!("https://danbooru.donmai.us/posts/random.json?tags=rating:s+{}", tag)
    };

    match client
        .get(&url)
        .basic_auth(&config.danbooru_login, Some(&config.danbooru_api_key))
        .send()
        .await {
        Ok(data) => {
            let text = data.text().await.unwrap();
            error!("Response: {}", text);
            match serde_json::from_str::<Post>(&*text) {
                Ok(mut post) => {
                    let mut attempts = 0;
                    while !is_nice_post(&post) && attempts < 3 {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        post = client.get(&url).basic_auth(&config.danbooru_login, Some(&config.danbooru_api_key)).send().await.unwrap().json::<Post>().await.unwrap();
                        attempts += 1;
                    }

                    if attempts >= 3 {
                        msg.channel_id.say(&ctx.http, "Temporary failure; rate limit?").await?;
                        return Ok(());
                    }

                    let _ = msg.channel_id.send_message(&ctx.http, |m|
                        m.embed(|e|
                            e.title(MessageBuilder::new().push("Artist: ").push_safe(post.tag_string_artist.clone().unwrap()).build())
                                .url(post.file_url.clone().unwrap())
                                .image(post.file_url.unwrap())
                        )
                    ).await;
                },
                Err(ex) => {
                    error!("No results found...: {}", ex);
                    msg.channel_id.say(&ctx.http, "No results found...").await?;
                }
            }
        },
        Err(ex) => {
            error!("Failed to send request: {}", ex);
            msg.channel_id.say(&ctx.http, "Failed to access Danbooru... try again later?").await?;
        }
    }

    Ok(())
}