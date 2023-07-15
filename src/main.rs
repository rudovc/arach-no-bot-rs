mod commands;
mod constants;
mod database;
mod framework;
mod handlers;
pub mod types;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = database::get_database();

    let framework = framework::get_framework(database).await;

    framework.run().await.unwrap();
}
