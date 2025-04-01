pub mod channels;

pub const R_DTG_REGEX: &str = r".*(?:https)?:(?:\/\/)??(?:([d,D]estiny[t,T]he[g,G]ame\.)|(?:np\.)|(?:www\.)|(?:old\.))?((?:redd\.it)|(?:re(?:ve)?ddit\.com))((?:\/r\/)|(?:\/v\/)?[d,D]estiny[t,T]he[g,G]ame)?.*";
pub const TWITTER_REGEX: &str =
    r".*(?:https|http):\/\/(?:(twitter\.com)|(?:www\.)|(?:t\.co)|(?:x\.com))";

pub const HMM_EMOJI_ID: u64 = 684857454521352198;
pub const HAHA_EMOJI_IDS: [u64; 3] = [931317049395937320, 599772724029685760, 1092619586706341989];
