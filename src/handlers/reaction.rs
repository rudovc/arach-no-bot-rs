use crate::constants;
use crate::database::{self as database, workaround::WithPutRequest};
use crate::types::ReactionInteraction;
use crate::types::UserRecord;
use anyhow::{anyhow, Result};
use poise::serenity_prelude as serenity;

pub async fn handle<'a>(
    reaction_interaction: ReactionInteraction,
    ctx: Option<
        impl serenity::CacheHttp
            + std::convert::AsRef<poise::serenity_prelude::Cache>
            + std::convert::AsRef<poise::serenity_prelude::Http>,
    >,
    database: &database::Database,
) -> Result<()> {
    let reaction: &serenity::Reaction = (&reaction_interaction).into();

    if let (Some(ctx), serenity::ReactionType::Custom { id: emoji_id, .. }) =
        (&ctx, &reaction.emoji)
    {
        let message = reaction.message(ctx).await?;

        if !constants::HAHA_EMOJI_IDS.contains(emoji_id.as_u64())
            || reaction
                .user_id
                // We're not going to allow cheating
                .is_some_and(|reaction_giver_id| reaction_giver_id == message.author.id)
        {
            return Ok(());
        }

        let user = database
            .connection
            .at("users")
            .at(&message.author.id.to_string());

        let user_record = UserRecord::new(&message.author.name, None);

        match reaction_interaction {
            ReactionInteraction::Add(_) => {
                user.update(&user_record)
                    .await
                    .map_err(|err| anyhow!(err))?;

                let increment = &database::FirebaseIncrement::increment_by(1);

                user.at("haha_count")
                    .put(increment)
                    .await
                    .map_err(|err| anyhow!(err))?;
            }
            ReactionInteraction::Remove(_) => {
                user.update(&user_record)
                    .await
                    .map_err(|err| anyhow!(err))?;

                let increment = &database::FirebaseIncrement::increment_by(-1);

                user.at("haha_count")
                    .put(increment)
                    .await
                    .map_err(|err| anyhow!(err))?;
            }
        };
    }

    Ok(())
}

// TODO: Tests
