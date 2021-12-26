use serenity::builder::CreateMessage;

pub fn level_up<'a, 'b>(next_level: i32) -> impl FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>
where 'a: 'b
{
    move |m| {
        m.embed(|e| e
            .title("Level up")
            .description(format!("You leveled up to **level {}**.", next_level))
        )
    }
}