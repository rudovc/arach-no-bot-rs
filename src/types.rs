use crate::database;
use poise::serenity_prelude as serenity;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserRecord {
    pub name: String,
    #[serde(skip_serializing)]
    pub hahas_count: u32,
}

impl UserRecord {
    pub fn new(name: &str, hahas_count: Option<u32>) -> Self {
        UserRecord {
            name: name.to_owned(),
            hahas_count: hahas_count.unwrap_or(0),
        }
    }
}

pub type PoiseContext<'a> = poise::Context<'a, database::Database, anyhow::Error>;

pub enum ReactionInteraction {
    Add(serenity::Reaction),
    Remove(serenity::Reaction),
}

impl<'a> From<&'a ReactionInteraction> for &'a serenity::Reaction {
    fn from(val: &'a ReactionInteraction) -> Self {
        match val {
            ReactionInteraction::Add(reaction) => reaction,
            ReactionInteraction::Remove(reaction) => reaction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_record_constructor() {
        let user_record = UserRecord::new("Test User", Some(0));

        assert_eq!(user_record.name, "Test User");
        assert_eq!(user_record.hahas_count, 0);

        let user_record = UserRecord::new("Other User", None);

        assert_eq!(user_record.name, "Other User");
        assert_eq!(user_record.hahas_count, 0);
    }
}
