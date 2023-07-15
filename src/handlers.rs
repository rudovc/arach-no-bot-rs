pub mod message;
pub mod reaction;

use crate::database;
use crate::types::ReactionInteraction;
use anyhow::{Error, Result};
use poise::event::Event;
use poise::serenity_prelude as serenity;

pub fn event<'a>(
    ctx: &'a serenity::Context,
    event: &'a poise::Event<'a>,
    framework: poise::FrameworkContext<'a, database::Database, Error>,
    _: &'a database::Database,
) -> poise::BoxFuture<'a, Result<()>> {
    Box::pin(async move {
        match event {
            Event::Message { new_message } => {
                let result = message::handle(new_message, Some(ctx)).await;

                println!(
                    "{:?}",
                    if result.is_ok() {
                        "Message event success.".to_owned()
                    } else {
                        result.unwrap_err().to_string()
                    }
                )
            }
            Event::ReactionAdd {
                add_reaction: reaction,
            } => {
                let result = reaction::handle(
                    ReactionInteraction::Add(reaction.to_owned()),
                    Some(ctx),
                    framework.user_data,
                )
                .await;

                println!(
                    "{:?}",
                    if result.is_ok() {
                        "Reaction add event success.".to_owned()
                    } else {
                        result.unwrap_err().to_string()
                    }
                )
            }
            Event::ReactionRemove {
                removed_reaction: reaction,
            } => {
                let result = reaction::handle(
                    ReactionInteraction::Remove(reaction.to_owned()),
                    Some(ctx),
                    framework.user_data,
                )
                .await;

                println!(
                    "{:?}",
                    if result.is_ok() {
                        "Reaction remove event success.".to_owned()
                    } else {
                        result.unwrap_err().to_string()
                    }
                )
            }
            _ => {}
        }

        Ok(())
    })
}
