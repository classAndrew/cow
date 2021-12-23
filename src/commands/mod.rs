mod command_framework;
mod hello;
mod time;

use serenity::framework::standard::{
    StandardFramework,
    macros::group
};

use hello::*;
use time::*;

#[group]
#[commands(hello, time)]
struct General;

pub fn get_framework(pref: &str) -> StandardFramework {
    return StandardFramework::new()
        .configure(|c| c.prefix(pref))
        .group(&GENERAL_GROUP)
}