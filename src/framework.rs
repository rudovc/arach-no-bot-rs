use crate::commands;
use crate::database;
use crate::handlers;
use anyhow::Error;
use firebase_rs as firebase;
use poise::serenity_prelude as serenity;

pub async fn get_framework(
    database: firebase::Firebase,
) -> poise::FrameworkBuilder<database::Database, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age(), commands::hahas()],
            event_handler: handlers::event,
            ..Default::default()
        })
        .token(
            std::env::var("DISCORD_TOKEN")
                .expect("DISCORD_TOKEN environment variable should be present."),
        )
        .intents(
            serenity::GatewayIntents::non_privileged()
                .union(serenity::GatewayIntents::privileged()),
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(database::Database::new(database))
            })
        })
}
