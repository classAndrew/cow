mod hello;
mod info;
mod rank;
mod time;
mod ban;
mod danbooru;

use serenity::framework::standard::macros::group;

use hello::*;
use time::*;
use info::*;
use rank::*;
use ban::*;
use danbooru::*;

#[group]
#[commands(hello, time, info, rank, disablexp, levels, bangenshinplayers, banleagueplayers, banvalorantplayers, reimu)]
#[description = "General commands for miscellaneous tasks."]
#[summary = "Basic commands"]
struct General;
