use poise::serenity_prelude as serenity;

use crate::constants;

pub fn is_testing_channel(channel_id: serenity::ChannelId) -> bool {
    let private_testing_id = std::env::var("PRIVATE_TESTING_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok());

    let testing_ids = [
        Some(constants::channels::BOT_TESTING_ID),
        private_testing_id,
    ];

    testing_ids.contains(&Some(*channel_id.as_u64()))
}

pub fn is_allowed_channel_in_current_mode(channel_id: serenity::ChannelId) -> bool {
    #[cfg(feature = "staging")]
    return is_testing_channel(channel_id);

    #[cfg(not(feature = "staging"))]
    return !is_testing_channel(channel_id);
}
