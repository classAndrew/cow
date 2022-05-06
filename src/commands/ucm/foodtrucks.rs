use chrono::Datelike;
use kuchiki::traits::{NodeIterator, TendrilSink};
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
        }
    }
};

fn process_schedules(data: &str) -> Vec<Vec<(String, String)>> {
    let page = kuchiki::parse_html().one(data);
    // Get all tables on the webpage
    let tables = page.select("table").unwrap();

    let schedules: Vec<Vec<(String, String)>> = tables.iter
        // Only pick the ones with a header (usually the days of the week)
        .filter(|o| o.as_node().children().any(|p| p.select_first("thead").is_ok()))
        .map(|o| {
            // Combine the header values and truck listings.
            let header = o.as_node().select_first("thead").unwrap().as_node().select_first("tr").unwrap().as_node().children();
            let items = o.as_node().select_first("tbody").unwrap().as_node().select("tr").unwrap();

            let schedule: Vec<(String, String)> = header.zip(items).map(|p| {
                let (day, trucks) = p;
                let day_text = day.children().text_nodes().next().map(|r| r.take()).unwrap_or_else(|| "bruh".to_string());
                let truck_text = trucks.as_node().children().map(|q| q.children().text_nodes().next().map(|r| r.take()).unwrap_or_else(|| "moment".to_string())).reduce(|a, b| format!("{}, {}", a, b)).unwrap();

                (day_text, truck_text)
            }).collect();

            schedule
        })
        .collect();

    schedules
}

#[command]
#[aliases(foodtruck)]
#[description = "Get the current food truck schedule."]
pub async fn foodtrucks(ctx: &Context, msg: &Message) -> CommandResult {
    let date = chrono::offset::Local::now();
    const URL: &str = "https://dining.ucmerced.edu/food-trucks";
    match reqwest::get(URL).await {
        Ok(response) => {
            match response.text().await {
                Ok(data) => {
                    let schedules = process_schedules(&*data);
                    msg.channel_id.say(&ctx.http, format!("Got {} schedules", schedules.len())).await?;
                    error!("{}", schedules.iter().map(|o| o.iter().map(|(p, q)| format!("({}, {})", p, q)).reduce(|a, b| format!("{}, {}", a, b)).unwrap()).reduce(|a, b| format!("[{}], [{}]", a, b)).unwrap());
                }
                Err(ex) => {
                    msg.channel_id.say(&ctx.http, "UC Merced gave us weird data, try again later?").await?;
                    error!("Failed to process calendar: {}", ex);
                }
            }
        }
        Err(ex) => {
            msg.channel_id.say(&ctx.http, "Failed to connect to the UC Merced website, try again later?").await?;
            error!("Failed to get food truck schedule: {}", ex);
        }
    }

    Ok(())
}