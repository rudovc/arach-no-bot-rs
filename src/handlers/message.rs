use crate::constants;
use ::anyhow::anyhow;

use poise::serenity_prelude as serenity;

#[derive(Debug)]
pub struct Message<'a> {
    channel_id: &'a u64,
    content: &'a String,
    message: Option<&'a serenity::Message>,
}

impl<'a> From<&'a Message<'_>> for Message<'a> {
    fn from(message: &'a Message) -> Message<'a> {
        Message {
            channel_id: message.channel_id,
            content: &message.content,
            message: message.message,
        }
    }
}

impl<'a> From<&'a serenity::Message> for Message<'a> {
    fn from(message: &'a serenity::Message) -> Message<'a> {
        Message {
            channel_id: message.channel_id.as_u64(),
            content: &message.content,
            message: Some(message),
        }
    }
}

fn scan_for_r_dtg(content: &str) -> Result<bool, regex::Error> {
    Ok(regex::Regex::new(constants::R_DTG_REGEX)?.is_match(content))
}

pub async fn message<'a>(
    message: impl Into<Message<'a>>,
    ctx: Option<
        impl serenity::CacheHttp
            + std::convert::AsRef<poise::serenity_prelude::Cache>
            + std::convert::AsRef<poise::serenity_prelude::Http>,
    >,
) -> anyhow::Result<bool> {
    let Message {
        channel_id,
        content,
        message,
    } = message.into();

    let is_private_testing_channel = *{
        &std::env::var("PRIVATE_TESTING_ID")
            .is_ok_and(|id_string| id_string.parse::<u64>().is_ok_and(|id| &id == channel_id))
    };

    match (*channel_id, is_private_testing_channel) {
        (constants::channels::BOT_TESTING_ID, _)
        | (constants::channels::DENSITY_THE_GAME_ID, _)
        | (_, true) => {
            let has_match = scan_for_r_dtg(content)?;

            if let (true, Some(message), Some(ctx)) = (has_match, message, ctx) {
                let guild: serenity::Guild = message
                    .guild(&ctx)
                    .ok_or_else(|| anyhow!("Did not find guild for message id: {}.", message.id))?;

                let emoji = guild
                    .emoji(
                        &ctx,
                        serenity::EmojiId::from(if is_private_testing_channel {
                            constants::TESTING_EMOJI_ID
                        } else {
                            constants::HMM_EMOJI_ID
                        }),
                    )
                    .await?;
                {
                    message.react(&ctx, emoji).await?;
                }
            }

            Ok(has_match)
        }
        _ => Err(anyhow!(
            "Channel ID {channel_id} is not in monitored channel list."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_only_read_messages_in_specific_channels() {
        let mut test_message = Message {
            channel_id: &0,
            content: &String::from("Irrelevant content"),
            message: None,
        };

        let result = message(&test_message, None::<serenity::Context>).await;

        assert!(result.is_err_and(|e| e.to_string().contains("is not in monitored channel list")));

        test_message.channel_id = &constants::channels::DENSITY_THE_GAME_ID;

        let result = message(&test_message, None::<serenity::Context>).await;

        assert!(!result.unwrap());

        test_message.channel_id = &constants::channels::BOT_TESTING_ID;

        let result = message(&test_message, None::<serenity::Context>).await;

        assert!(!result.unwrap());
    }

    #[test]
    fn regex_should_only_flag_messages_with_r_dtg() {
        let test_content = "Does not include the Bad Word";

        assert!(!scan_for_r_dtg(test_content).unwrap());

        let test_content = "Includes the Bad Word! https://www.reddit.com/r/DestinyTheGame/";

        assert!(scan_for_r_dtg(test_content).unwrap())
    }
}
