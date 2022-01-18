use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hours {
    pub from: String,
    pub to: String
}

#[derive(Debug, Deserialize)]
pub struct Times {
    pub note: Option<String>,
    pub status: String,
    pub hours: Option<Vec<Hours>>,
    pub currently_open: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct Day {
    pub date: String,
    pub times: Times,
    pub rendered: String
}

#[derive(Debug, Deserialize)]
pub struct Week {
    #[serde(rename = "Sunday")]
    pub sunday: Day,
    #[serde(rename = "Monday")]
    pub monday: Day,
    #[serde(rename = "Tuesday")]
    pub tuesday: Day,
    #[serde(rename = "Wednesday")]
    pub wednesday: Day,
    #[serde(rename = "Thursday")]
    pub thursday: Day,
    #[serde(rename = "Friday")]
    pub friday: Day,
    #[serde(rename = "Saturday")]
    pub saturday: Day
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub lid: u16,
    pub name: String,
    pub category: String,
    pub url: String,
    pub contact: String,
    pub lat: String,
    pub long: String,
    pub color: String,
    #[serde(rename = "fn")] // I have no idea what the field is for.
    pub f: Option<String>,
    pub parent_lid: Option<u16>,
    pub weeks: Vec<Week>
}

#[derive(Debug, Deserialize)]
pub struct Calendar {
    pub locations: Vec<Location>
}