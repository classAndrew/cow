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

async fn course_embed(ctx: &Context, msg: &Message, class: &Class) {
    
}

#[command]
#[description = "Search for courses in a term."]
#[usage = "<CRN, Course Number, or Name> [Semester] [Year]"]
pub async fn courses(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Ok(course_registration_number) = args.single::<i32>() {
        // Make sure it's not a year lol
        if course_registration_number >= 10000 {

        }
    }

    Ok(())
}