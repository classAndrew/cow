use serenity::{
    client::Context,
    model::channel::Message
};
use log::{info};

pub async fn message(_: &Context, _msg: &Message) {
    // This is basically useless for most cases.
}

pub async fn non_command(_: &Context, _msg: &Message) {
    // We can use this for ranking... execute a stored proc to give exp?
}