use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::sync::Arc;
use serenity::{
    model::id::{
        UserId,
        GuildId,
        ChannelId, RoleId
    },
    prelude::TypeMapKey
};
use tiberius::{AuthMethod, Config};
use rust_decimal::{
    Decimal,
    prelude::FromPrimitive
};
use rust_decimal::prelude::ToPrimitive;

use crate::Database;
use crate::commands::cowboard::cowboard_db_models::*;

// Separating the database into different modules so it doesn't become a 2000 line file.
impl Database {
    pub async fn get_cowboard_config(&self, server_id: GuildId) -> Result<Cowboard, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let res = conn.query(
            "SELECT channel, add_threshold, remove_threshold, emote, webhook_id, webhook_token FROM [Cowboard].[Server] WHERE id = @P1",
            &[&server])
            .await?
            .into_row()
            .await?;

        let mut out = Cowboard::new(server_id.0);

        if let Some(item) = res {
            let channel_id: Option<rust_decimal::Decimal> = item.get(0);
            let emote_str: &str = item.get(3).unwrap();
            let webhook_id: Option<rust_decimal::Decimal> = item.get(4);
            let webhook_token: Option<&str> = item.get(5);
            out = Cowboard {
                id: server_id.0,
                channel: channel_id.and_then(|o| o.to_u64()),
                add_threshold: item.get(1).unwrap(),
                remove_threshold: item.get(2).unwrap(),
                emote: emote_str.to_string(),
                webhook_id: webhook_id.and_then(|o| o.to_u64()),
                webhook_token: webhook_token.map(|o| o.to_string())
            };
        }

        Ok(out)
    }

    pub async fn update_cowboard(&self, config: &Cowboard) -> Result<Cowboard, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(config.id).unwrap();
        let res = conn.query(
            "UPDATE [Cowboard].[Server] WHERE id = @P1",
            &[&server])
            .await?
            .into_row()
            .await?;

        let mut out = Cowboard::new(server_id.0);

        if let Some(item) = res {
            let channel_id: Option<rust_decimal::Decimal> = item.get(0);
            let emote_str: &str = item.get(3).unwrap();
            let webhook_id: Option<rust_decimal::Decimal> = item.get(4);
            let webhook_token: Option<&str> = item.get(5);
            out = Cowboard {
                id: server_id.0,
                channel: channel_id.and_then(|o| o.to_u64()),
                add_threshold: item.get(1).unwrap(),
                remove_threshold: item.get(2).unwrap(),
                emote: emote_str.to_string(),
                webhook_id: webhook_id.and_then(|o| o.to_u64()),
                webhook_token: webhook_token.map(|o| o.to_string())
            };
        }

        Ok(out)
    }
}