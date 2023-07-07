mod commands;
mod handlers;
mod types;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age(), commands::hahas()],
            event_handler: |_ctx, event, _, _| {
                Box::pin(async move {
                    if let poise::Event::Message { new_message } = event {
                        handlers::message(new_message);
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged()
                .union(serenity::GatewayIntents::privileged()),
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(types::Data {})
            })
        });

    framework.run().await.unwrap();
}
