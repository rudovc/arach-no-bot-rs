use crate::constants;
use crate::staging::is_testing_channel;
use color_eyre::eyre::eyre;
use color_eyre::Result;

use poise::serenity_prelude::{CacheHttp, EmojiId, Guild, Message, ReactionType};

fn check_string_for_twitter(content: &str) -> Result<bool, regex::Error> {
    Ok(regex::Regex::new(constants::TWITTER_REGEX)?.is_match(content))
}

fn check_string_for_r_drg(content: &str) -> Result<bool, regex::Error> {
    Ok(regex::Regex::new(constants::R_DTG_REGEX)?.is_match(content))
}

fn check_message_content_for_bad_words(
    channel_id: u64,
    content: &str,
    testing: bool,
) -> Result<bool> {
    let has_twitter_links = check_string_for_twitter(content)?;

    if has_twitter_links {
        Ok(has_twitter_links)
    } else {
        check_message_content_in_channel_for_r_dtg(channel_id, content, testing)
    }
}

fn check_message_content_in_channel_for_r_dtg(
    channel_id: u64,
    content: &str,
    testing: bool,
) -> Result<bool> {
    match (channel_id, testing) {
        (constants::channels::DENSITY_THE_GAME_ID, false) | (_, true) => {
            let has_match = check_string_for_r_drg(content)?;

            Ok(has_match)
        }
        _ => Err(eyre!(
            "Channel ID {channel_id} is not in monitored channel list."
        )),
    }
}

pub async fn handle(
    message: Message,
    ctx: impl CacheHttp
        + std::convert::AsRef<poise::serenity_prelude::Cache>
        + std::convert::AsRef<poise::serenity_prelude::Http>,
) -> Result<()> {
    let Message { channel_id, .. } = message;

    let is_testing_channel = is_testing_channel(message.channel_id);
    let result = check_message_content_for_bad_words(
        channel_id.into(),
        &message.content,
        is_testing_channel,
    )?;

    if result {
        let guild: Guild = message
            .guild(&ctx)
            .ok_or_else(|| eyre!("Did not find guild for message id: {}.", message.id))?;

        let emoji = if is_testing_channel {
            ReactionType::Unicode("🤖".to_owned())
        } else {
            ReactionType::from(
                guild
                    .emoji(&ctx, EmojiId::from(constants::HMM_EMOJI_ID))
                    .await?,
            )
        };

        message.react(&ctx, emoji).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    enum Expect {
        Result(bool),
        Error(),
    }

    #[test_case(
        constants::channels::DENSITY_THE_GAME_ID,
        Expect::Result(false),
        "Does not include the Bad Word!",
        false ;
        "when there's no mention of r/dtg"
    )]
    #[test_case(
        constants::channels::DENSITY_THE_GAME_ID,
        Expect::Result(true),
        "Includes the Bad Word! https://www.reddit.com/r/DestinyTheGame/",
        false ;
        "when there's mention of r/dtg"
    )]
    #[test_case(
        0,
        Expect::Error(),
        "Includes the Bad Word! https://www.reddit.com/r/DestinyTheGame/",
        false ;
        "when a message is sent outside of the destiny channel"
    )]
    #[test_case(
        constants::channels::DENSITY_THE_GAME_ID,
        Expect::Result(true),
        "Includes the Bad Word! https://www.reddit.com/r/DestinyTheGame/",
        true ;
        "when a matching message is sent in the testing channel"
    )]
    #[test_case(
        constants::channels::DENSITY_THE_GAME_ID,
        Expect::Result(false),
        "Does not include the Bad Word!",
        true ;
        "when a non-matching message is sent in the testing channel"
    )]
    #[tokio::test]
    async fn test_handler_only_read_messages_in_specific_channels(
        channel_id: u64,
        expect: Expect,
        content: &str,
        testing: bool,
    ) {
        match expect {
            Expect::Result(expect) => {
                let result =
                    check_message_content_in_channel_for_r_dtg(channel_id, content, testing);
                assert_eq!(
                    result.expect("Result should not error when provided a valid channel."),
                    expect
                );
            }
            Expect::Error() => {
                let result =
                    check_message_content_in_channel_for_r_dtg(channel_id, content, testing);
                assert!(result
                    .is_err_and(|e| e.to_string().contains("is not in monitored channel list")));
            }
        };
    }

    #[test_case(
        "Does not include the Bad Word",
        false ;
        "when there's no mention of r/dtg"
    )]
    #[test_case(
        "Includes the Bad Word! https://www.reddit.com/r/DestinyTheGame/",
        true ;
        "when there's a regular www r/dtg link"
    )]
    #[test_case(
        "Includes the Bad Word! https://old.reddit.com/r/DestinyTheGame/%E2%9C%93",
        true ;
        "when there's an old reddit r/dtg link"
    )]
    fn test_flag_messages_with_r_dtg(test_content: &str, expect: bool) {
        assert_eq!(check_string_for_r_drg(test_content).unwrap(), expect);
    }

    #[test_case(
        "Does not include the Elon Musk Bad Word",
        false ;
        "when there's no twitter link"
    )]
    #[test_case(
        "Includes the Elon Musk Bad Word! https://twitter.com/SomeRandomTwitterHandler/status/12345678910",
        true ;
        "when there's a regular www twitter link"
    )]
    #[test_case(
        "Includes the Elon Musk Bad Word! https://t.co/abc123def456",
        true ;
        "when there's a shortened twitter link (t.co)"
    )]
    #[test_case(
            "Includes the Elon Musk Bad Word! https://x.com/home",
        true ;
        "when there's an X (yuck!) link"
    )]
    fn test_flag_messages_with_twitter_links(test_content: &str, expect: bool) {
        assert_eq!(check_string_for_twitter(test_content).unwrap(), expect);
    }
}
