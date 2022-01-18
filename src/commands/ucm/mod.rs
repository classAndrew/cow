mod library;
mod libcal_models;

use serenity::framework::standard::macros::group;

use library::*;

#[group]
#[prefixes("ucm", "ucmerced")]
#[description = "Get information about UC Merced's services and facilities."]
#[summary = "UC Merced info"]
#[commands(library)]
struct UCM;