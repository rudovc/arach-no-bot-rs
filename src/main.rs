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
        dotenv::from_filename(".env.staging").ok();
    }

    let database = database::get_database();

    let framework = framework::get_framework(database).await;

    framework.run().await.unwrap();
}
