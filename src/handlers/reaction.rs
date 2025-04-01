use crate::constants;
use crate::database::{workaround::WithPutRequest, Database, FirebaseIncrement, IncrementMap};
use crate::types::{ReactionInteraction, UserRecord};
use color_eyre::{eyre::eyre, Result};
use poise::serenity_prelude::{Cache, CacheHttp, Http, Reaction, ReactionType, UserId};

fn match_haha_emoji(emoji_id: &u64) -> Result<()> {
    if !constants::HAHA_EMOJI_IDS.contains(emoji_id) {
        Err(eyre!("Emoji does not match haha emoji."))
    } else {
        Ok(())
    }
}

fn check_for_self_reaction(
    reaction_giver_id: Option<UserId>,
    message_author_id: UserId,
) -> Result<()> {
    match reaction_giver_id {
        Some(id) if id == message_author_id => {
            Err(eyre!("User tried to haha react their own message."))
        }
        Some(_) => Ok(()),
        None => Err(eyre!("Expected to find user ID, found None")),
    }
}

fn get_firebase_increment_for_reaction(reaction_interaction: ReactionInteraction) -> IncrementMap {
    match reaction_interaction {
        ReactionInteraction::Add => FirebaseIncrement::increment_by(1),
        ReactionInteraction::Remove => FirebaseIncrement::increment_by(-1),
    }
}

pub async fn handle(
    reaction_interaction: ReactionInteraction,
    reaction: &Reaction,
    ctx: impl CacheHttp + std::convert::AsRef<Cache> + std::convert::AsRef<Http>,
    database: &Database,
) -> Result<Option<String>> {
    if let ReactionType::Custom { id: emoji_id, .. } = &reaction.emoji {
        let message = reaction.message(ctx).await?;

        match_haha_emoji(emoji_id.as_u64())?;
        check_for_self_reaction(reaction.user_id, message.author.id)?;

        let user = database
            .connection
            .at("users")
            .at(&message.author.id.to_string());

        let user_record = UserRecord::new(&message.author.name, None);

        user.update(&user_record).await.map_err(|err| eyre!(err))?;

        let increment = get_firebase_increment_for_reaction(reaction_interaction);

        let response = user
            .at("haha_count")
            .put(&increment)
            .await
            .map_err(|err| eyre!(err))?;

        Ok(Some(response.data))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReactionInteraction;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(0, false ; "when the emoji does not match a known haha")]
    #[test_case(
        931317049395937320, true ;
        "when the emoji matches the local known haha"
    )]
    #[test_case(
        599772724029685760, true ;
        "when the emoji matches an external haha"
    )]
    #[test_case(
        1092619586706341989, true ;
        "when the emoji matches another external haha"
    )]
    fn test_matching_a_haha_emoji(emoji_id: u64, expect: bool) {
        let result = match_haha_emoji(&emoji_id);

        assert_eq!(result.is_ok(), expect)
    }

    #[test_case(
        Some(UserId::from(0)), UserId::from(1), true ; "when the user IDs don't match"
    )]
    #[test_case(Some(UserId::from(4799853325)), UserId::from(6381533619), true)]
    #[test_case(Some(UserId::from(9100277759)), UserId::from(9316850468), true)]
    #[test_case(Some(UserId::from(7480516711)), UserId::from(6780785561), true)]
    #[test_case(
        Some(UserId::from(1)), UserId::from(1), false ; "when the user IDs do match"
    )]
    #[test_case(Some(UserId::from(7132173634)), UserId::from(7132173634), false)]
    #[test_case(Some(UserId::from(2061429558)), UserId::from(2061429558), false)]
    #[test_case(Some(UserId::from(2100720041)), UserId::from(2100720041), false)]
    #[test_case(Some(UserId::from(1)), UserId::from(1), false)]
    #[test_case(
        None, UserId::from(1), false ; "when the reaction giver user IDs doesn't exist"
    )]
    fn test_self_reaction_filter(
        reaction_giver_id: Option<UserId>,
        message_author_id: UserId,
        expect: bool,
    ) {
        let result = check_for_self_reaction(reaction_giver_id, message_author_id);

        assert_eq!(result.is_ok(), expect);

        match (reaction_giver_id, &result) {
            (Some(_), Err(err)) => {
                assert!(err
                    .to_string()
                    .contains("User tried to haha react their own message."));
            }
            (None, Err(err)) => {
                assert!(err
                    .to_string()
                    .contains("Expected to find user ID, found None"));
            }
            _ => (),
        }
    }

    #[test_case(ReactionInteraction::Add, HashMap::from([(".sv".to_owned(), FirebaseIncrement::new(1))]) ; "when a reaction is added" )]
    #[test_case(ReactionInteraction::Remove, HashMap::from([(".sv".to_owned(), FirebaseIncrement::new(-1))]) ; "when a reaction is removed" )]
    fn test_create_firebase_increment_object_for_interaction(
        reaction_interaction: ReactionInteraction,
        expect: IncrementMap,
    ) {
        assert_eq!(
            get_firebase_increment_for_reaction(reaction_interaction),
            expect
        )
    }
}
