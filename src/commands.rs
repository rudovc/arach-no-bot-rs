mod hahas;
mod with_staging;
use hahas::*;

use crate::database;

pub fn get_slash_commands() -> Vec<poise::Command<database::Database, anyhow::Error>> {
    with_staging::commands_with_staging(vec![hahas()])
}
