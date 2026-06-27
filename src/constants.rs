pub mod channels;

pub const R_DTG_REGEX_FALLBACK: &str = r".*(?:https)?:(?:\/\/)??(?:([d,D]estiny[t,T]he[g,G]ame\.)|(?:np\.)|(?:www\.)|(?:old\.))?((?:redd\.it)|(?:re(?:ve)?ddit\.com))((?:\/r\/)|(?:\/v\/)?[d,D]estiny[t,T]he[g,G]ame)?.*";
pub const TWITTER_REGEX_FALLBACK: &str = r".*(?:https|http):\/\/?(?:(?:(?:www\.)?(?:(?:fixup|fixv)?x\.com|(:?(?:fx|vx)?twitter\.com)))|t\.co)";

pub const HMM_EMOJI_ID_FALLBACK: u64 = 684857454521352198;
pub const HAHA_EMOJI_IDS_FALLBACK: [u64; 3] =
    [931317049395937320, 599772724029685760, 1092619586706341989];

#[derive(Clone)]
pub struct Environment {
    pub r_dtg_regex: String,
    pub twitter_regex: String,
    pub hmm_emoji_id: u64,
    pub haha_emoji_ids: [u64; 3],
}

impl Environment {
    pub fn load() -> Self {
        let hmm_emoji_id = if let Ok(hmm_emoji_string) = std::env::var("HMM_EMOJI_ID") {
            hmm_emoji_string.parse().unwrap_or(HMM_EMOJI_ID_FALLBACK)
        } else {
            HMM_EMOJI_ID_FALLBACK
        };

        return Environment {
            r_dtg_regex: std::env::var("R_DTG_REGEX").unwrap_or(R_DTG_REGEX_FALLBACK.to_owned()),
            twitter_regex: std::env::var("TWITTER_REGEX")
                .unwrap_or(TWITTER_REGEX_FALLBACK.to_owned()),
            hmm_emoji_id,
            haha_emoji_ids: HAHA_EMOJI_IDS_FALLBACK,
        };
    }
}
