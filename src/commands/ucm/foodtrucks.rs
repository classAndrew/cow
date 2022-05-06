use chrono::{Local, NaiveDate};
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
use scraper::{Html, Selector};

pub struct FoodTruckSchedule {
    pub date: NaiveDate,
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>
}

impl FoodTruckSchedule {
    pub fn new() -> Self {
        FoodTruckSchedule {
            date: Local::now().date().naive_local(),
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None
        }
    }
}

fn reduce_columns(table: &[Vec<String>], column: usize) -> String {
    table
        .iter()
        .skip(1)
        .map(|o| o.get(column))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap().to_string())
        .reduce(|a, b| format!("{}, {}", a, b))
        .unwrap_or_default()
}

fn paragraph_to_date(input: &str) -> NaiveDate {
    let mut month: u32 = 1;
    let mut month_modified = false;
    let mut day: u32 = 1;
    let mut year: i32 = 2022;

    // I could use RegEx, buuut
    let nicer_input = input.to_lowercase().replace('.', " ").replace(',', " ").replace('/', " ").replace('-', " ").replace('>', " ").replace('<', " ");
    for item in nicer_input.split(' ') {
        if !month_modified && item.len() >= 3 {
            match &item[..3] {
                // This is so ugly.
                "jan" => { month = 1; month_modified = true; continue; },
                "feb" => { month = 2; month_modified = true; continue; },
                "mar" => { month = 3; month_modified = true; continue; },
                "apr" => { month = 4; month_modified = true; continue; },
                "may" => { month = 5; month_modified = true; continue; },
                "jun" => { month = 6; month_modified = true; continue; },
                "jul" => { month = 7; month_modified = true; continue; },
                "aug" => { month = 8; month_modified = true; continue; },
                "sep" => { month = 9; month_modified = true; continue; },
                "oct" => { month = 10; month_modified = true; continue; },
                "nov" => { month = 11; month_modified = true; continue; },
                "dec" => { month = 12; month_modified = true; continue; },
                &_ => { }
            }
        }

        if let Ok(num) = item.parse::<u32>() {
            if !month_modified && num > 0 && num < 13 {
                month = num;
                month_modified = true;
                continue;
            } else if month_modified && num > 0 && num < 32 {
                day = num;
            } else if num > 2000 {
                year = num as i32;
            } else {
                error!("What am I reading... \"{}\"", nicer_input);
            }
        }
    }

    NaiveDate::from_ymd(year, month, day)
}

fn process_schedules(data: &str) -> Vec<FoodTruckSchedule> {
    let page = Html::parse_document(data);

    let select_paragraph = Selector::parse("p").unwrap();
    let select_table = Selector::parse("table").unwrap();
    let select_header = Selector::parse("thead").unwrap();
    let select_row = Selector::parse("tr").unwrap();
    let select_column = Selector::parse("th, td").unwrap();

    let tables = page.select(&select_table)
        .filter(|o| o.select(&select_header).count() > 0)
        .map(|o|
            o.select(&select_row).map(|p|
                p.select(&select_column).map(|q|
                    q.text().next().map(|r| r.to_string()).unwrap_or_else(|| "unknown".to_string())
            ).filter(|o| !o.trim().is_empty()).collect::<Vec<_>>()
        ).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dates = page.select(&select_paragraph).map(|o| o.inner_html()).filter(|o| o.contains("week of")).map(|o| paragraph_to_date(&*o)).collect::<Vec<_>>();

    let mut out: Vec<FoodTruckSchedule> = Vec::new();

    for (table, date) in tables.iter().zip(dates) {
        if table.is_empty() {
            error!("Gathered an empty table??");
        } else {
            let title = table.get(0).unwrap();
            let mut schedule = FoodTruckSchedule::new();
            schedule.date = date;
            // Clippy hates range loops...
            for (i, _) in title.iter().enumerate() {
                let day = title[i].to_lowercase();
                match day.trim() {
                    "monday" => schedule.monday = Some(reduce_columns(table, i)),
                    "tuesday" => schedule.tuesday = Some(reduce_columns(table, i)),
                    "wednesday" => schedule.wednesday = Some(reduce_columns(table, i)),
                    "thursday" => schedule.thursday = Some(reduce_columns(table, i)),
                    "friday" => schedule.friday = Some(reduce_columns(table, i)),
                    _ => {}
                }
            }
            out.push(schedule);
        }
    }

    out
}

async fn pick_best_schedule(schedules: &Vec<FoodTruckSchedule>) -> Option<&FoodTruckSchedule> {
    if schedules.is_empty() {
        return None;
    }

    let now = Local::now().naive_local().date();
    for schedule in schedules {
        let date = schedule.date;
        let diff = now.signed_duration_since(date).num_days();
        if diff < 5 && diff > -2 {
            return Some(schedule);
        }
    }

    let first = schedules.first().unwrap();
    if now.signed_duration_since(first.date).num_days() < 0 {
        return Some(first);
    }
    return Some(schedules.last().unwrap())
}

async fn print_schedule(ctx: &Context, msg: &Message, schedule: &FoodTruckSchedule) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
        e
            .title("Food Truck Schedule")
            .description(format!("For the week of {}", schedule.date.format("%B %d, %Y")))
            .field("Monday", schedule.monday.clone().unwrap_or_else(|| "Unknown/Closed".to_string()), false)
            .field("Tuesday", schedule.tuesday.clone().unwrap_or_else(|| "Unknown/Closed".to_string()), false)
            .field("Wednesday", schedule.wednesday.clone().unwrap_or_else(|| "Unknown/Closed".to_string()), false)
            .field("Thursday", schedule.thursday.clone().unwrap_or_else(|| "Unknown/Closed".to_string()), false)
            .field("Friday", schedule.friday.clone().unwrap_or_else(|| "Unknown/Closed".to_string()), false)
    })).await?;

    Ok(())
}

#[command]
#[aliases(foodtruck)]
#[description = "Get the current food truck schedule."]
pub async fn foodtrucks(ctx: &Context, msg: &Message) -> CommandResult {
    const URL: &str = "https://dining.ucmerced.edu/food-trucks";
    match reqwest::get(URL).await {
        Ok(response) => {
            match response.text().await {
                Ok(data) => {
                    let schedules = process_schedules(&*data);
                    let best_schedule = pick_best_schedule(&schedules).await;
                    if let Some(schedule) = best_schedule {
                        print_schedule(ctx, msg, schedule).await?;
                    } else {
                        msg.channel_id.say(&ctx.http, "Could not get any valid schedules... Did the website change layout?").await?;
                    }
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