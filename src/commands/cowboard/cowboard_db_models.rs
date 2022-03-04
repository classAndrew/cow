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
            emote: ":cow:".to_string(),
            webhook_id: None,
            webhook_token: None
        }
    }
    
    pub fn update(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        crate::commands::cowboard::cowboard_db::update_cowboard(self)
    }
}

pub struct CowboardMessage {
    pub message_id: u64,
    pub message_channel_id: u64,
    pub post_id: u64,
    pub post_channel_id: u64,
    pub guild_id: u64
}

impl CowboardMessage {
    pub fn new(id: u64) -> Self {
        CowboardMessage {
            message_id: 0,
            message_channel_id: 0,
            post_id: 0,
            post_channel_id: 0,
            guild_id: 0
        }
    }
}