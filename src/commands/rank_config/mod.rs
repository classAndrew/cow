mod rank_config;

use serenity::framework::standard::macros::group;

use rank_config::*;

#[group]
#[prefixes("rankconfig", "rc")]
#[description = "Configuration to manage ranks and levelling on the server."]
#[summary = "Rank configuration"]
#[default_command(list)]
#[commands(list, add, remove)]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
struct RankConfig;