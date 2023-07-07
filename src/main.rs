mod commands;
mod types;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("I'm running!");
    println!("{:?}", std::env::var("DISCORD_TOKEN"));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age(), commands::hahas()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .intents(serenity::GatewayIntents::privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(types::Data {})
            })
        });

    framework.run().await.unwrap();
}
