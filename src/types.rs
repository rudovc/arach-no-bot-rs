use crate::database;
use color_eyre::eyre::Error;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserRecord {
    pub name: String,
    #[serde(skip_serializing)]
    pub haha_count: u32,
}

impl UserRecord {
    pub fn new(name: &str, haha_count: Option<u32>) -> Self {
        UserRecord {
            name: name.to_owned(),
            haha_count: haha_count.unwrap_or(0),
        }
    }
}

pub type PoiseContext<'a> = poise::Context<'a, database::Database, Error>;

pub enum ReactionInteraction {
    Add,
    Remove,
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(Some(0), "Test User", 0 ; "When user has 0 hahas")]
    #[test_case(Some(14), "Test User", 14 ; "When user has some hahas")]
    #[test_case(None, "Test User", 0 ; "When user has None hahas")]
    fn test_user_record_constructor(count: Option<u32>, name: &str, expected_count: u32) {
        let user_record = UserRecord::new(name, count);
        assert_eq!(user_record.name, name);
        assert_eq!(user_record.haha_count, expected_count);
    }
}
