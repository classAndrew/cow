mod general;
mod rank_config;
mod timeout;

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
                hook,
                help
            }, HelpOptions, CommandGroup
        }
    },
    client::Context
};

use crate::commands::general::GENERAL_GROUP;
use crate::commands::rank_config::RANKCONFIG_GROUP;
use crate::commands::timeout::TIMEOUT_GROUP;

#[help]
#[individual_command_tip = "Cow help command\n\n\
Add the command you want to learn more about to the help command\n"]
#[command_not_found_text = "Could not find command: `{}`."]
#[max_levenshtein_distance(2)]
#[lacking_permissions = "Nothing"]
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
        .group(&TIMEOUT_GROUP)
    ));
}