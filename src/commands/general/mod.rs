mod hello;
mod info;
mod rank;
mod time;

use serenity::framework::standard::macros::group;

use hello::*;
use time::*;
use info::*;
use rank::*;

#[group]
#[commands(hello, time, info, rank, disablexp, levels)]
#[description = "General commands for miscellaneous tasks."]
#[summary = "Basic commands"]
struct General;