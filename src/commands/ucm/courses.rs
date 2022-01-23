use chrono::Datelike;
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
use crate::commands::ucm::course_models::CourseList;

#[command]
#[description = "Get the course list for a major"]
#[only_in(guilds)]
pub async fn courses(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // setting the session cookies
    let term_url = "https://reg-prod.ec.ucmerced.edu/StudentRegistrationSsb/ssb/term/search?mode=courseSearch&term=202210&studyPath=&studyPathText=&startDatepicker=&endDatepicker=";
    let search_url = "https://reg-prod.ec.ucmerced.edu/StudentRegistrationSsb/ssb/courseSearch/courseSearch";

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    client.get(term_url).send().await?;
    client.get(search_url).send().await?;

    let major = args.single::<String>()
        .unwrap_or("".into())
        .to_uppercase();

    // need to also grab current semester
    let url = format!("https://reg-prod.ec.ucmerced.edu/StudentRegistrationSsb/ssb/courseSearchResults/courseSearchResults?\
        txt_subject={}\
        &txt_term=202210\
        &startDatepicker=\
        &endDatepicker=\
        &pageOffset=0\
        &pageMaxSize=10\
        &sortColumn=subjectDescription\
        &sortDirection=asc", major);

    match client.get(url).send().await {
        Ok(response) => {
            // TODO: add pagination for courses
            match response.json::<CourseList>().await {
                Ok(course_list) => {
                    msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e
                                .title("Course List")
                                .description(format!("For major: {}", major));

                            for course in course_list.data {
                                let title = course.course_title.unwrap_or("No Title".into());
                                e.field(format!("{} {}-{}", major, course.course_number.unwrap_or("000".into()), title), 
                                    course.course_description.unwrap_or("No description".into())+"...", false);
                            }

                            e
                        })
                    }).await?;
                }
                Err(ex) => {
                    msg.channel_id.say(&ctx.http, "The course search gave us weird data, try again later?").await?;
                    error!("Failed to process course search: {}", ex);
                }
            }
        }
        Err(ex) => {
            msg.channel_id.say(&ctx.http, "Failed to connect to the course search API, try again later?").await?;
            error!("Failed to get course search: {}", ex);
        }
    }

    Ok(())
}