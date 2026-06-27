use crate::commands;
use crate::constants::Environment;
use crate::database;
use crate::handlers;
use color_eyre::eyre::Error;
use firebase_rs as firebase;

pub struct Metadata {
    pub database: database::Database,
    pub environment: Environment,
}

pub async fn get_framework(
    database: firebase::Firebase,
) -> poise::FrameworkBuilder<Metadata, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_slash_commands(),
            event_handler: handlers::event,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Metadata {
                    environment: Environment::load(),
                    database: database::Database::new(database),
                })
            })
        })
}
