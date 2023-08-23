use log::error;
use serde::{Serialize, Deserialize};

use super::Auth;
use super::Avatar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: u32,
    pub name: String,
    pub hash: u32,
    pub contacts: Option<Vec<u32>>,
    pub chats: Option<Vec<u32>>,
    pub auth: Option<Auth>,
    pub avatar: Option<Avatar>,
}

impl User {
    pub fn from_string(json: &str) -> Option<Self> {
        match serde_json::from_str(json) {
            Ok(user) => Some(user),
            Err(err) => {
                error!("Failed to parse User, {err:?}");
                None
            }
        }   
    }

    pub fn to_string(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(err) => {
                error!("Failed to stringify User, {err:?}");
                "".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let json = "{\"name\":\"hello\",\"hash\":123456}";
        let user = User::from_string(json).unwrap();
        assert_eq!(user.id, 0);
        assert_eq!(user.name, "hello");
        assert_eq!(user.hash, 123456);
        assert!(user.avatar.is_none());
        assert!(user.auth.is_none());
        assert!(user.chats.is_none());
        assert!(user.contacts.is_none());
    }
}