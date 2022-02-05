use serde::Deserialize;
use std::convert::{TryFrom, From};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use chrono::{Datelike, DateTime, Local, NaiveTime, Weekday};

#[derive(FromPrimitive)]
pub enum Day {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

pub enum Meal {
    Breakfast,
    Lunch,
    Dinner,
    Other(String) // Force a search.
}

impl TryFrom<u32> for Day {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(v).ok_or(())
    }
}

impl From<chrono::Weekday> for Day {
    fn from(v: chrono::Weekday) -> Self {
        match v {
            Weekday::Mon => Day::Monday,
            Weekday::Tue => Day::Tuesday,
            Weekday::Wed => Day::Wednesday,
            Weekday::Thu => Day::Thursday,
            Weekday::Fri => Day::Friday,
            Weekday::Sat => Day::Saturday,
            Weekday::Sun => Day::Sunday
        }
    }
}

impl TryFrom<&str> for Day {
    type Error = ();
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v.to_lowercase().as_str() {
            "sunday" => Ok(Day::Sunday),
            "monday" => Ok(Day::Monday),
            "tuesday" => Ok(Day::Tuesday),
            "wednesday" => Ok(Day::Wednesday),
            "thursday" => Ok(Day::Thursday),
            "friday" => Ok(Day::Friday),
            "saturday" => Ok(Day::Saturday),
            &_ => Err(())
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::Sunday => write!(f, "Sunday"),
            Day::Monday => write!(f, "Monday"),
            Day::Tuesday => write!(f, "Tuesday"),
            Day::Wednesday => write!(f, "Wednesday"),
            Day::Thursday => write!(f, "Thursday"),
            Day::Friday => write!(f, "Friday"),
            Day::Saturday => write!(f, "Saturday")
        }
    }
}

impl From<&str> for Meal {
    fn from(v: &str) -> Self {
        match v.to_lowercase().as_str() {
            "breakfast" => Meal::Breakfast,
            "lunch" => Meal::Lunch,
            "dinner" => Meal::Dinner,
            other => Meal::Other(other.to_string())
        }
    }
}

impl Display for Meal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Meal::Breakfast => write!(f, "Breakfast"),
            Meal::Lunch => write!(f, "Lunch"),
            Meal::Dinner => write!(f, "Dinner"),
            Meal::Other(x) => write!(f, "{}", x)
        }
    }
}

// Shrinking some models down since they're pretty large.

pub trait PavData {}

#[derive(Debug, Deserialize)]
pub struct PavResult<T> {
    pub code: u16,
    pub message: String,
    pub data: T
}

// Pavilion Info

#[derive(Debug, Deserialize)]
pub struct Location {
    // WHY DOES THIS HAVE BOTH _id AND id IN THE JSON???
    #[serde(rename = "_id")]
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct Company {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "locationInfo")]
    pub location_info: Location
}

// Pavilion Groups

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub order: Option<i32>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MenuGroups {
    pub menu_groups: Vec<Group>,
    pub menu_categories: Vec<Group>
}

impl MenuGroups {
    fn search(array: &[Group], query: &str) -> Option<String> {
        let query_lower = query.to_lowercase();
        array.iter().find(|x| x.name.to_lowercase().contains(&query_lower)).map(|s| s.id.clone())
    }

    pub fn get_group(&self, day: Day) -> Option<String> {
        MenuGroups::search(&self.menu_groups, &day.to_string())
    }

    pub fn get_category(&self, meal: Meal) -> Option<String> {
        MenuGroups::search(&self.menu_categories, &meal.to_string())
    }
}

// Pavilion Items

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub description: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MenuItems {
    pub menu_items: Vec<Item>
}

// Pavilion Times (hard-coded, IDK if there's an API for them)
pub struct PavilionTime;

impl PavilionTime {

    pub fn schedule(datetime: &DateTime<Local>) -> (Day, Meal) {
        let breakfast_weekday_start: NaiveTime = NaiveTime::from_hms(7, 0, 0);
        let breakfast_end: NaiveTime = NaiveTime::from_hms(10, 30, 0);
        let _breakfast_weekday: Range<NaiveTime> = breakfast_weekday_start..breakfast_end;
        let breakfast_weekend_start: NaiveTime = NaiveTime::from_hms(9, 0, 0);
        let _breakfast_weekend: Range<NaiveTime> = breakfast_weekend_start..breakfast_end;
        let lunch_start: NaiveTime = NaiveTime::from_hms(11, 0, 0);
        let lunch_end: NaiveTime= NaiveTime::from_hms(15, 0, 0);
        let lunch: Range<NaiveTime> = lunch_start..lunch_end;
        let dinner_start: NaiveTime = NaiveTime::from_hms(16, 0, 0);
        let dinner_end: NaiveTime = NaiveTime::from_hms(21, 0, 0);
        let dinner: Range<NaiveTime> = dinner_start..dinner_end;

        let day = Day::from(datetime.weekday());
        let time = datetime.time();

        if time < breakfast_end {
            return (day, Meal::Breakfast);
        } else if lunch.contains(&time) {
            return (day, Meal::Lunch);
        } else if dinner.contains(&time) {
            return (day, Meal::Dinner);
        }

        (Day::from(datetime.weekday().succ()), Meal::Breakfast)
    }
}