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
use crate::commands::ucm::courses_db_models::*;
use crate::{Database, db};

fn fix_time(time: &str) -> String {
    let hour_str = &time[..2];
    let minute_str = &time[2..];
    let hour = hour_str.parse::<u8>().unwrap();

    if hour == 0 {
        return format!("12:{} AM", minute_str);
    }
    if hour == 12 {
        return format!("12:{} PM", minute_str);
    }
    if hour < 12 {
        return format!("{}:{} AM", hour, minute_str);
    }
    format!("{}:{} PM", hour - 12, minute_str)
}

fn is_semester(input: &str) -> i32 {
    match input.to_lowercase().as_str() {
        "fall" => 10,
        "spring" => 20,
        "summer" => 30,
        _ => -1
    }
}

async fn course_embed(ctx: &Context, msg: &Message, class: &Class) -> CommandResult {
    let db = db!(ctx);
    let professors = db.get_professors_for_class(class.id).await;
    let meetings = db.get_meetings_for_class(class.id).await;

    msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
        e.title(format!("{}: {}", &class.course_number, class.course_title.clone().unwrap_or_else(|| "<unknown class name>".to_string())));
        e.description("Enrollment and Waitlist are in terms of seats available/seats taken/max seats.");
        e.field("CRN", class.course_reference_number, true);
        e.field("Credit Hours", class.credit_hours, true);
        e.field("Enrollment", format!("{}/{}/{}", class.seats_available, class.enrollment, class.maximum_enrollment), true);
        e.field("Waitlist", format!("{}/{}/{}", class.wait_available, class.wait_capacity - class.wait_available, class.wait_capacity), true);

        if let Ok(professors) = professors {
            e.field("Professor(s)",
                    professors.iter()
                        .map(|o| format!("- {} {}", o.first_name.clone(), o.last_name.clone()))
                        .reduce(|a, b| format!("{}\n{}", a, b))
                        .unwrap_or_else(|| "No professors are assigned to this course.".to_string()),
                    false);
        }

        if let Ok(meetings) = meetings {
            e.field("Meeting(s)",
                    meetings.iter()
                        .map(|o| {
                            let output = format!("- {}: {} {}",
                                                 o.meeting_type, o.building_description.clone().unwrap_or_else(|| "<no building>".to_string()), o.room.clone().unwrap_or_else(|| "<no room>".to_string()));
                            if o.begin_time.is_some() && o.end_time.is_some() {
                                let begin_time = o.begin_time.clone().unwrap();
                                let end_time = o.end_time.clone().unwrap();
                                return format!("{} from {} to {}", output, fix_time(&begin_time), fix_time(&end_time));
                            }

                            output
                        })
                        .reduce(|a, b| format!("{}\n{}", a, b))
                        .unwrap_or_else(|| "No professors are assigned to this course.".to_string()),
                    false);
        }

        e
    })).await?;

    Ok(())
}

#[command]
#[description = "Search for courses in a term."]
#[usage = "<CRN, Course Number, or Name> [Semester] [Year]"]
pub async fn courses(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Ok(course_registration_number) = args.single::<i32>() {
        // Make sure it's not a year lol
        if course_registration_number >= 10000 {
            let db = db!(ctx);
            match db.get_class(course_registration_number).await {
                Ok(option_class) => {
                    if let Some(class) = option_class {
                        course_embed(ctx, msg, &class).await?;
                    } else {
                        msg.channel_id.say(&ctx.http, format!("Could not find a class with the CRN `{}`.", course_registration_number)).await?;
                    }
                }
                Err(ex) => {
                    error!("Failed to get class: {}", ex);
                    msg.channel_id.say(&ctx.http, "Failed to query our database... try again later?").await?;
                }
            }
            return Ok(())
        }
    }

    Ok(())
}