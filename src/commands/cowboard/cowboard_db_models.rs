pub struct Cowboard {
    pub id: u64,
    pub channel: Option<u64>,
    pub add_threshold: i32,
    pub remove_threshold: i32,
    pub emote: String,
    pub webhook_id: Option<u64>,
    pub webhook_token: Option<String>
}

impl Cowboard {
    pub fn new(id: u64) -> Self {
        Cowboard {
            id,
            channel: None,
            add_threshold: 5,
            remove_threshold: 4,
            emote: "🐮".to_string(),
            webhook_id: None,
            webhook_token: None
        }
    }
}

pub struct CowboardMessage {
    pub message_id: u64,
    pub message_channel_id: u64,
    pub post_id: u64,
    pub post_channel_id: u64,
    pub guild_id: u64
}