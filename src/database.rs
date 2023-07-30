pub mod workaround;

use firebase_rs as firebase;

pub type IncrementMap = std::collections::HashMap<String, FirebaseIncrement>;

pub struct Database {
    pub connection: firebase::Firebase,
}

impl Database {
    pub fn new(connection: firebase::Firebase) -> Self {
        Database { connection }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct FirebaseIncrement {
    increment: i32,
}

impl FirebaseIncrement {
    pub fn new(amount: i32) -> FirebaseIncrement {
        FirebaseIncrement { increment: amount }
    }

    pub fn increment_by(amount: i32) -> std::collections::HashMap<String, FirebaseIncrement> {
        let mut map = std::collections::HashMap::new();

        map.insert(".sv".to_string(), FirebaseIncrement { increment: amount });

        map
    }
}

pub fn get_database() -> firebase::Firebase {
    firebase::Firebase::auth(
        &std::env::var("FIREBASE_URL")
            .expect("FIREBASE_URL environment variable should be present."),
        &std::env::var("FIREBASE_TOKEN")
            .expect("FIREBASE_TOKEN environment variable should be present."),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_firebase_increment() {
        let increment = FirebaseIncrement::increment_by(1);

        assert_eq!(increment.get(".sv").unwrap().increment, 1);

        let increment = FirebaseIncrement::increment_by(-1);

        assert_eq!(increment.get(".sv").unwrap().increment, -1);
    }
}
