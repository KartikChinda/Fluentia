use serde::{Deserialize, Serialize};
use uuid::Uuid;

// just making simple models, basic shit.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password_hash: String,
    pub language: String,
    pub level: String,
}

// this is essentially a constructor for creating a new user.
impl User {
    pub fn new(name: &str, password_hash: &str, language: &str, level: &str) -> Self {
        User {
            user_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            password_hash: password_hash.to_string(),
            language: language.to_string(),
            level: level.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    pub conversation_id: String,
    pub user_id: String,
    pub messages: Vec<String>,
    pub topic: String,
}

impl Conversation {
    pub fn new(user_id: &str, messages: Vec<String>, topic: &str) -> Self {
        Conversation {
            conversation_id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            messages,
            topic: topic.to_string(),
        }
    }
}
