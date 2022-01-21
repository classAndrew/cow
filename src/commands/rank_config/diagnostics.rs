use std::collections::HashSet;
use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::{
            RoleId
        }
    },
    framework::standard::{
        CommandResult,
        macros::{
            command
        }
    }
};
use crate::{Database, db};

#[command]
#[description = "Scan for discrepancies between server member roles and the stored info."]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn scan(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);
    if let Some(guild_id) = msg.guild_id {
        let roles = db.get_roles(guild_id).await?;
        let role_set = roles.into_iter().filter(|r| r.role_id.is_some()).map(|r| r.role_id.unwrap()).collect::<HashSet<_>>();
        let users = db.get_users(guild_id).await?;
        for u in users {
            if let Ok(member) = guild_id.member(&ctx.http, u.user).await {
                let member_role_set: HashSet<RoleId> = HashSet::from_iter(member.roles.iter().cloned());
            }
        }
    } else {
        msg.reply(&ctx.http, "This command can only be run in a server.").await?;
    }

    Ok(())
}