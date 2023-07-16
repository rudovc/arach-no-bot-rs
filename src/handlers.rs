pub mod message;
pub mod reaction;

use crate::database;
use crate::staging;
use crate::types::ReactionInteraction;
use anyhow::{anyhow, Error, Result};
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
                if !staging::is_allowed_channel_in_current_mode(new_message.channel_id) {
                    return Err(anyhow!(
                        "Event fired in disallowed channel for current mode."
                    ));
                }

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
                if !staging::is_allowed_channel_in_current_mode(reaction.channel_id) {
                    return Err(anyhow!(
                        "Event fired in disallowed channel for current mode."
                    ));
                }

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
                if !staging::is_allowed_channel_in_current_mode(reaction.channel_id) {
                    return Err(anyhow!(
                        "Event fired in disallowed channel for current mode."
                    ));
                }

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
