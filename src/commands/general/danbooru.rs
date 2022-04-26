use std::convert::Infallible;
use std::path::Path;
use log::error;
// Fun with stupid APIs!
use serenity::client::Context;
use serenity::framework::standard::{ArgError, Args, CommandResult};
use serenity::model::channel::{Message};
use serenity::framework::standard::macros::{command};
use tokio::fs;
use crate::Config;
use serde::Deserialize;
use rand::Rng;
use regex::Regex;
use serenity::http::AttachmentType;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
struct Post {
    // 's', 'q', 'e' (safe, questionable, explicit)
    pub rating: Option<String>,
    // Bytes.
    pub file_size: Option<u64>,
    // MD5 hash.
    pub md5: Option<String>,
    // Features of the image
    pub tag_string_general: Option<String>,
    pub tag_string_character: Option<String>,
    pub tag_string_copyright: Option<String>,
    pub tag_string_artist: Option<String>,
    pub tag_string_meta: Option<String>,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
    pub preview_file_url: Option<String>
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
        .map(|o: Result<String, ArgError<Infallible>>| o) // Identity map to set type. It keeps complaining.
        .map(|o| o.unwrap().trim().to_lowercase())
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
            if data.is_empty() {
                msg.channel_id.say(&ctx.http, "No results found...").await?;
                return Ok(());
            }

            let mut index = rand::thread_rng().gen_range(0..data.len());
            let mut post = data.get(index).unwrap();
            while !is_nice_post(post) {
                index = rand::thread_rng().gen_range(0..data.len());
                post = data.get(index).unwrap();
            }
            let file_url = post.file_url.clone().unwrap();
            let last_index = &file_url.rfind('/').unwrap() + 1;
            let file_name = &file_url[last_index..];
            let bytes = client.get(&file_url).send().await?.bytes().await?;
            let mut file = fs::File::create(file_name).await?;
            file.write_all(&*bytes).await?;

            let _ = msg.channel_id.send_message(&ctx.http, |m|
                {
                    let execution = m
                        .embed(|e| {
                            e.title(format!("Artist: {}", post.tag_string_artist.clone().unwrap()))
                            .url(post.file_url.clone().unwrap())
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