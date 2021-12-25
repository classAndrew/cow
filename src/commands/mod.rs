mod hello;
mod time;
mod info;
mod rank;

use std::collections::HashSet;
use std::sync::Arc;
use serenity:: {
    model::{
        id::UserId,
        channel::Message
    },
    framework:: {
        Framework,
        standard::{
            StandardFramework,
            macros::{
                group,
                hook
            }
        }
    },
    client::Context
};

use hello::*;
use time::*;
use info::*;
use rank::*;

#[group]
#[commands(hello, time, info, rank, disablexp)]
struct General;

#[hook]
async fn non_command(ctx: &Context, msg: &Message) {
    crate::message_handler::non_command(ctx, msg).await;
}

pub fn get_framework(pref: &str, app_id: UserId, owners: HashSet<UserId>) -> Arc<Box<dyn Framework + Sync + std::marker::Send>> {
    return Arc::new(Box::new(StandardFramework::new()
        .configure(|c| c
            .prefix(pref)
            .on_mention(Some(app_id))
            .owners(owners)
        )
        .normal_message(non_command)
        .group(&GENERAL_GROUP)
    ));
}