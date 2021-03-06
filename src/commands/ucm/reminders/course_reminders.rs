use log::error;
use serenity::{
    client::Context,
    model::{
        channel::Message
    },
    framework::standard::{
        CommandResult,
        macros::{
            command
        }, Args
    }
};

use crate::{db, Database};
use crate::commands::ucm::courses_db_models::Reminder;

#[command]
#[description = "List the reminders set."]
pub async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db!(ctx);

    match db.get_user_reminders(msg.author.id).await {
        Ok(reminders) => {
            msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
                e.title("Your Course Reminders");

                if reminders.is_empty() {
                    e.description("You do not have any reminders set. Add some using `reminders add`.");
                } else {
                    for reminder in reminders {
                        e.field(format!("CRN {}", reminder.course_reference_number),
                                format!("Minimum Trigger: `{}`\nFor Waitlist: `{}`\nTriggered: `{}`", reminder.min_trigger, reminder.for_waitlist, reminder.triggered),
                                false);
                    }
                }

                e
            })).await?;
        }
        Err(ex) => {
            error!("Failed to get reminders for user: {}", ex);
            msg.channel_id.say(&ctx.http, "Failed to get your reminders... try again later?").await?;
        }
    }

    Ok(())
}

#[command]
#[description = "Control reminders for class seats."]
#[usage = "[CRN] <minimum seats> <for waitlist>"]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "You need to pass in a valid CRN.\n\
        You can also pass in the minimum amount of seats to trigger the reminder, as well.\n\
        If you want, you can also make it trigger on waitlist seats instead (true/false), however you must have done the previous part beforehand.\n\
        Ex. `reminders add 31415 1 true`").await?;
        return Ok(());
    }

    let mut min_trigger = 1;
    let mut for_waitlist = false;

    let course_reference_number = match args.single::<i32>() {
        Ok(value) => { value }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "You need to pass in a valid CRN for the first value.").await?;
            return Ok(());
        }
    };

    if !args.is_empty() {
        match args.single::<i32>() {
            Ok(value) => {
                if value < 1 {
                    msg.channel_id.say(&ctx.http, "Your minimum trigger must be greater than or equal to 1 seat.").await?;
                    return Ok(());
                }
                min_trigger = value;
            }
            Err(_) => {
                msg.channel_id.say(&ctx.http, "You need to pass in a positive integer for minimum trigger.").await?;
                return Ok(());
            }
        }
    }

    if !args.is_empty() {
        match args.single::<bool>() {
            Ok(value) => {
                for_waitlist = value;
            }
            Err(_) => {
                msg.channel_id.say(&ctx.http, "Put \"true\" if you want to trigger on waitlist slots, otherwise omit this field (or put \"false\").").await?;
                return Ok(());
            }
        }
    }

    let reminder = Reminder {
        user_id: msg.author.id.0,
        course_reference_number,
        min_trigger,
        for_waitlist,
        triggered: false
    };

    let db = db!(ctx);

    if let Ok(Some(class)) = db.get_class(course_reference_number).await {
        if let Err(ex) = db.add_reminder(&reminder).await {
            error!("Failed to add reminder: {}", ex);
            msg.channel_id.say(&ctx.http, "Error adding your reminder. Maybe you have a duplicate?").await?;
        } else {
            msg.channel_id.say(&ctx.http, format!("Successfully added your reminder for {}: {}!",
                                                  class.course_number,
                                                  class.course_title.unwrap_or_else(|| "<unknown class name>".to_string())
            )).await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "Could not find this CRN... did you type it right?").await?;
    }

    Ok(())
}

#[command]
#[description = "Control reminders for class seats."]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "You need to pass in a valid CRN for a reminder you set up.").await?;
        return Ok(());
    }

    if let Ok(course_reference_number) = args.single::<i32>() {
        let db = db!(ctx);
        match db.remove_reminder(msg.author.id, course_reference_number).await {
            Ok(success) => {
                if success {
                    msg.channel_id.say(&ctx.http, "Successfully removed your reminder.").await?;
                } else {
                    msg.channel_id.say(&ctx.http, "You did not have a reminder with this CRN.").await?;
                }
            }
            Err(ex) => {
                error!("Failed to remove reminder: {}", ex);
                msg.channel_id.say(&ctx.http, "Failed to remove your reminder... try again later?").await?;
            }
        }
    } else {
        msg.channel_id.say(&ctx.http, "That is not a valid CRN.").await?;
    }

    Ok(())
}