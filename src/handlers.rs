pub mod message;
pub mod reaction;

use crate::database;
use crate::staging;
use crate::types::ReactionInteraction;
use color_eyre::{
    eyre::{eyre, Error},
    Result,
};
use poise::event::Event;
use poise::serenity_prelude as serenity;
use tracing::{error, info};

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
                    let channel = &new_message.channel(&ctx.http).await?.id();
                    let user = &new_message.author;

                    return Err(eyre!("Event fired in disallowed channel for current mode.\nChannel: {}\nUser: {}", channel.as_u64(), user.name));
                }

                message::handle(new_message.to_owned(), ctx).await?;
            }
            Event::ReactionAdd {
                add_reaction: reaction,
            } => {
                if !staging::is_allowed_channel_in_current_mode(reaction.channel_id) {
                    let giver = &reaction.member;

                    return Err(eyre!("Event fired in disallowed channel for current mode.\nChannel: {}\nGiver: {}", reaction.channel_id, giver.as_ref()
                    .and_then(|m| m.user.as_ref().map(|u| &u.name))
                    .unwrap_or(&"None".to_owned()),));
                }

                let result =
                    reaction::handle(ReactionInteraction::Add, reaction, ctx, framework.user_data)
                        .await;

                if let Ok(result) = result {
                    let giver_name = &reaction.user(&ctx).await.map(|u| u.name.to_string());
                    let message = &reaction.message(&ctx.http).await;
                    let message_timestamp = reaction
                        .message(&ctx.http)
                        .await
                        .map(|m| m.timestamp.to_string());
                    let author_name = reaction.message(&ctx.http).await.map(|m| m.author.name);
                    let message_id = message.as_ref().map(|m| m.id.to_string());
                    let channel_id = message.as_ref().map(|m| m.channel_id.to_string());

                    info!(
                                "Reaction remove event success.\nGiver: {}\nAuthor: {}\nMessage: {}: {} / {}\nDB response: {}",
                                giver_name.as_ref().unwrap_or(&"None".to_owned()),
                                &author_name.unwrap_or("None".to_owned()),
                                &message_timestamp.unwrap_or("None".to_owned()),
                                &channel_id.unwrap_or("None".to_owned()),
                                &message_id.unwrap_or("None".to_owned()),
                                &result.unwrap_or("None".to_owned())
                            );
                } else {
                    error!("{}", result.unwrap_err().to_string());
                }
            }
            Event::ReactionRemove {
                removed_reaction: reaction,
            } => {
                if !staging::is_allowed_channel_in_current_mode(reaction.channel_id) {
                    let giver = &reaction.member;

                    return Err(eyre!("Event fired in disallowed channel for current mode.\nChannel: {}\nGiver: {}", reaction.channel_id, giver.as_ref()
                    .and_then(|m| m.user.as_ref().map(|u| &u.name))
                    .unwrap_or(&"None".to_owned()),));
                }

                let result = reaction::handle(
                    ReactionInteraction::Remove,
                    reaction,
                    ctx,
                    framework.user_data,
                )
                .await;

                if let Ok(result) = result {
                    let giver_name = &reaction.user(&ctx).await.map(|u| u.name.to_string());
                    let message = &reaction.message(&ctx.http).await;
                    let message_timestamp = reaction
                        .message(&ctx.http)
                        .await
                        .map(|m| m.timestamp.to_string());
                    let author_name = reaction.message(&ctx.http).await.map(|m| m.author.name);
                    let message_id = message.as_ref().map(|m| m.id.to_string());
                    let channel_id = message.as_ref().map(|m| m.channel_id.to_string());

                    info!(
                        "Reaction remove event success.\nAuthor: {}\nGiver: {}\nMessage: {}: {} / {}\nDB response: {}",
                        giver_name.as_ref().unwrap_or(&"None".to_owned()),
                        &author_name.unwrap_or("None".to_owned()),
                        &message_timestamp.unwrap_or("None".to_owned()),
                        &channel_id.unwrap_or("None".to_owned()),
                        &message_id.unwrap_or("None".to_owned()),
                        &result.unwrap_or("None".to_owned())
                    );
                } else {
                    error!("{}", result.unwrap_err().to_string());
                }
            }
            _ => {}
        }

        Ok(())
    })
}
