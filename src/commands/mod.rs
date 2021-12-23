mod hello;
mod time;
mod info;

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
use serenity::prelude::TypeMapKey;

use hello::*;
use time::*;
use info::*;

pub struct FrameworkContainer;

impl TypeMapKey for FrameworkContainer {
    type Value = Arc<Box<dyn Framework + Sync + std::marker::Send>>;
}

#[group]
#[commands(hello, time, info)]
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