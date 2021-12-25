use serenity::model::channel::Embed;
use serenity::model::channel::Message;

fn level_up(m: Message, prev_lvl: u64, next_lvl: u64) -> impl Fn(Message) {
    // serenity wants this called like channel.send_message(ctx, level_up(...))
    // Rust compiler broken so I can't test if this actually works. fixing atm
    move |m| {
        m.embed(|e| e
            .title("Level up")
            .description(format!("Leveled from {} to {}.", prev_lvl, next_lvl))
        )
    } 
}