mod timeout;

use serenity::framework::standard::macros::group;

use timeout::*;

#[group]
#[prefixes("timeout")]
#[description = "Commands for viewing and settinge the cooldown for chat xp."]
#[summary = "Timeouts"]
#[default_command(get)]
#[commands(set, get)]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
struct Timeout;

