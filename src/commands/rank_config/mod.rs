mod roles;
mod diagnostics;

use serenity::framework::standard::macros::group;

use roles::*;
use diagnostics::*;

#[group]
#[prefixes("rankconfig", "rc")]
#[description = "Configuration to manage ranks and levelling on the server."]
#[summary = "Rank configuration"]
#[default_command(list)]
#[commands(list, add, remove, scan, fix)]
struct RankConfig;