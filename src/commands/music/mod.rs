mod music_commands;

use serenity::framework::standard::macros::group;

use music_commands::*;

#[group]
#[prefixes("music")]
#[description = "Commands for playing music."]
#[summary = "Music"]
#[default_command(help)]
#[commands(help, join, leave, play, now_playing, skip, queue)]
struct Music;