use poise::serenity_prelude;
use std::io::ErrorKind;
use tokio::fs::create_dir;

use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
mod commands;
mod constants;
mod database;
mod framework;
mod handlers;
mod staging;
pub mod types;

#[tokio::main]
async fn main() {
    #[cfg(feature = "staging")]
    {
        dotenvy::from_filename(".env.staging").ok();
    }

    create_dir("./log")
        .await
        .or_else(|e| match e.kind() == ErrorKind::AlreadyExists {
            true => Ok(()),
            false => Err(e),
        })
        .expect("Expected to create log directory.");

    let appender = tracing_appender::rolling::hourly("./log", "rolling.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN environment variable should be present.");

    let intents = serenity_prelude::GatewayIntents::non_privileged()
        .union(serenity_prelude::GatewayIntents::privileged());

    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .finish()
            .with(fmt::Layer::default().with_writer(non_blocking_appender)),
    )
    .expect("Unable to set global tracing subscriber");

    let database = database::get_database();

    let framework = framework::get_framework(database).await.build();

    let client = serenity_prelude::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client
        .expect("Client could not be creaeted")
        .start()
        .await
        .expect("Client could not start");
}
