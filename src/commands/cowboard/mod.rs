mod cowboard_config;
mod cowboard_db;
mod cowboard_db_models;
pub mod cowboard_handler;

use serenity::framework::standard::macros::group;

use cowboard_config::*;

#[group]
#[prefixes("cowboard")]
#[description = "Commands for modifying how the cowboard (starboard) functions."]
#[summary = "Cowboard"]
#[default_command(info)]
#[commands(info, emote, addthreshold, removethreshold, channel, webhook)]
struct Cowboard;

