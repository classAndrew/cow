mod hello;
mod time;

use std::collections::HashSet;
use std::sync::Arc;
use serenity:: {
    model::id::UserId,
    framework:: {
        Framework,
        standard::{
            StandardFramework,
            macros::group
        }
    }
};

use hello::*;
use time::*;

#[group]
#[commands(hello, time)]
struct General;

pub fn get_framework(pref: &str, app_id: UserId, owners: HashSet<UserId>) -> Arc<Box<dyn Framework + Sync + std::marker::Send>> {
    return Arc::new(Box::new(StandardFramework::new()
        .configure(|c| c
            .prefix(pref)
            .on_mention(Some(app_id))
            .owners(owners)
        )
        .group(&GENERAL_GROUP))) ;
}