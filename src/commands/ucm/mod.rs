mod library;
mod libcal_models;
mod courses;
mod course_models;
mod pavilion;
mod pav_models;
pub mod course_reminders;
mod courses_db;
mod courses_db_models;

use serenity::framework::standard::macros::group;

use library::*;
use courses::*;
use pavilion::*;

#[group]
#[prefixes("ucm", "ucmerced")]
#[description = "Get information about UC Merced's services and facilities."]
#[summary = "UC Merced info"]
#[commands(library, courses, pavilion)]
struct UCM;