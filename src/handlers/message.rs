use poise::serenity_prelude as serenity;

pub fn message(message: &serenity::Message) {
    println!("{:?}", message);
}
