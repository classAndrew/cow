use std::time::Duration;
use log::error;
use serenity::client::Context;
use tokio::time;
use crate::{db, Database};

pub async fn check_reminders(ctx: &Context) {
    let mut interval_day = time::interval(Duration::from_secs(60));
    loop {
        interval_day.tick().await;
        let db = db!(ctx);
        match db.trigger_reminders().await {
            Ok(triggers) => {
                for trigger in triggers {
                    if let Ok(user) = ctx.http.get_user(trigger.user_id).await {
                        let _ = user.direct_message(&ctx.http, |m| { m.content("Bruh.") });
                    }
                }
            },
            Err(ex) => {
                error!("Failed to query reminders: {}", ex);
            }
        }
    }
}