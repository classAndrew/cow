mod library;
mod libcal_models;
mod courses;
mod course_models;

use serenity::framework::standard::macros::group;

use library::*;
use courses::*;

#[group]
#[prefixes("ucm", "ucmerced")]
#[description = "Get information about UC Merced's services and facilities."]
#[summary = "UC Merced info"]
#[commands(library, courses)]
struct UCM;