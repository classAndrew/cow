use serenity::{
    client::Context,
    model::channel::Message
};
use log::{debug};

pub async fn message(_: Context, msg: Message) {
    // TODO this can be used to provide if a user levels up
    debug!("<message received by {}>", msg.author.name);
}