use poise::serenity_prelude::ChannelId;

use crate::constants;

pub fn is_testing_channel(channel_id: ChannelId) -> bool {
    let private_testing_id = std::env::var("PRIVATE_TESTING_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok());

    let testing_ids = [
        Some(constants::channels::BOT_TESTING_ID),
        private_testing_id,
    ];

    testing_ids.contains(&Some(*channel_id.as_u64()))
}

pub fn is_allowed_channel_in_current_mode(channel_id: ChannelId) -> bool {
    #[cfg(feature = "staging")]
    return is_testing_channel(channel_id);

    #[cfg(not(feature = "staging"))]
    return !is_testing_channel(channel_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use poise::serenity_prelude::ChannelId;
    use test_case::test_case;

    #[test_case(0, false ; "when channel_id is 0")]
    #[test_case(constants::channels::DENSITY_THE_GAME_ID, false ; "when channel is destiny the game channel")]
    #[test_case(constants::channels::BOT_TESTING_ID, true ; "when channel is the testing channel")]
    fn test_should_correctly_identify_testing_channels(channel_id: u64, allow: bool) {
        let result = is_testing_channel(ChannelId::from(channel_id));
        assert_eq!(result, allow);
    }

    #[test_case(0, false ; "when channel_id is 0")]
    #[test_case(constants::channels::DENSITY_THE_GAME_ID, false ; "when channel is destiny the game channel")]
    #[test_case(constants::channels::BOT_TESTING_ID, true ; "when channel is the testing channel")]
    fn test_allow_testing_and_forbid_other_channels_depending_on_mode(
        channel_id: u64,
        allow: bool,
    ) {
        #[cfg(feature = "staging")]
        {
            let result = is_allowed_channel_in_current_mode(ChannelId::from(channel_id));
            assert_eq!(result, allow);
        }

        #[cfg(not(feature = "staging"))]
        {
            let result = is_allowed_channel_in_current_mode(ChannelId::from(channel_id));
            assert_eq!(result, !allow);
        }
    }
}
