mod hello;
mod time;
mod info;
mod rank;
mod rank_config;

use std::{collections::HashSet};
use std::sync::Arc;
use serenity::framework::standard::{CommandResult, help_commands, Args};
use serenity:: {
    model::{
        id::UserId,
        channel::Message
    },
    framework:: {
        Framework,
        standard::{
            StandardFramework,
            macros::{
                group,
                hook,
                help
            }, HelpOptions, CommandGroup
        }
    },
    client::Context
};

use hello::*;
use time::*;
use info::*;
use rank::*;

#[group]
#[commands(hello, time, info, rank, disablexp, levels)]
struct General;

#[help]
// there's no way to grab the bot prefix
#[individual_command_tip = "Cow help command\n\n\
Add the command you want to learn more about to the help command\n"]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = "Strikethrough commands require elevated permissions."]
async fn cow_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

use rank_config::*;

#[group]
#[prefixes("rankconfig", "rc")]
#[default_command(list)]
#[commands(list, add)]
struct RankConfig;

#[hook]
async fn non_command(ctx: &Context, msg: &Message) {
    crate::message_handler::non_command(ctx, msg).await;
}

pub fn get_framework(pref: &str, app_id: UserId, owners: HashSet<UserId>) -> Arc<Box<dyn Framework + Sync + std::marker::Send>> {
    return Arc::new(Box::new(StandardFramework::new()
        .configure(|c| c
            .prefix(pref)
            .on_mention(Some(app_id))
            .owners(owners)
        )
        .normal_message(non_command)
        .help(&COW_HELP)
        .group(&GENERAL_GROUP)
        .group(&RANKCONFIG_GROUP)
    ));
}