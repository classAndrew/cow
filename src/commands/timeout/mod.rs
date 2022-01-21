mod timeout_config;

use serenity::framework::standard::macros::group;

use timeout_config::*;

#[group]
#[prefixes("timeout")]
#[description = "Commands for viewing and settinge the cooldown for chat xp."]
#[summary = "Timeouts"]
#[default_command(get)]
#[commands(set, get)]
struct Timeout;

